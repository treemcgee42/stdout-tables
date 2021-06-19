//! Table-related structures and operations

use crate::wrap;
use crate::themes;

use std::error::Error;
use std::fmt;

/// The representation of a table
#[derive(Debug)]
pub struct Table {
    headers: Vec<wrap::WrappedCell>,
    data: Vec<Vec<wrap::WrappedCell>>,
}

impl Table {
    /// Display the `Table` as a table by printing to `stdout`.
    /// 
    /// # Arguments
    /// 
    /// * `theme` - the `Theme` to use when drawing the table
    pub fn draw(&self, theme: themes::Theme) {
        let column_widths = &self.headers.iter()
                                .map(|wcell| wcell.width)
                                .collect::<Vec<_>>();

        // draw top border
        println!("{}",Table::table_top_border(column_widths,&theme));

        // draw the column headers
        Table::draw_row(&self.headers, theme.vertical_border, theme.internal_vertical);
        println!("{}", Table::table_row_sep(column_widths,&theme));

        // draw the rows of data
        for (i,d) in self.data.iter().enumerate() {
            Table::draw_row(d, theme.vertical_border, theme.internal_vertical);
            if i != self.data.len()-1 {
                println!("{}",Table::table_row_sep(column_widths,&theme));
            }
        }

        // draw bottom border
        println!("{}", Table::table_bottom_border(column_widths,&theme));
    }

    fn draw_row(v: &Vec<wrap::WrappedCell>, vert_border: char, internal_vert: char) {
        let split_headers = v.iter()
            .map(|s| s.content.split('\n').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        for j in 0..split_headers[0].len() {
            let mut to_draw = String::new();
            for (ind,i) in split_headers.iter().enumerate() {
                if ind==0 { 
                    to_draw += &vert_border.to_string(); 
                    to_draw += i[j];
                } else {
                    to_draw += &internal_vert.to_string();
                    to_draw += i[j];
                }
            }
            to_draw += &vert_border.to_string();
            println!("{}",to_draw);
        }
    }

    fn table_row_border(
        col_widths: &Vec<usize>, 
        left: char, center: char, right: char,
        horiz_border: char
    ) -> String {
        let mut to_print = String::new();

        for (i,cw) in col_widths.iter().enumerate() {
            if i==0 {
                to_print += &left.to_string();
                to_print += &(0..*cw).map(|_| &horiz_border).collect::<String>();
            } else if i==col_widths.len()-1 {
                to_print += &center.to_string();
                to_print += &(0..*cw).map(|_| &horiz_border).collect::<String>();
                to_print += &right.to_string();
            } else {
                to_print += &center.to_string();
                to_print += &(0..*cw).map(|_| &horiz_border).collect::<String>();
            }
        }

        to_print
    }

    fn table_top_border(col_widths: &Vec<usize>, theme: &themes::Theme) -> String {
        Table::table_row_border(
            col_widths, 
            theme.top_left_corner, theme.top_center, theme.top_right_corner, 
            theme.horizontal_border
        )
    }

    fn table_bottom_border(col_widths: &Vec<usize>, theme: &themes::Theme) -> String {
        Table::table_row_border(
            col_widths, 
            theme.bottom_left_corner, theme.bottom_center, theme.bottom_right_corner, 
            theme.horizontal_border
        )
    }

    fn table_row_sep(col_widths: &Vec<usize>, theme: &themes::Theme) -> String {
        Table::table_row_border(
            col_widths, 
            theme.middle_left, theme.middle_center, theme.middle_right, 
            theme.internal_horizontal
        )
    }

    /// Numbers the entries of the table.
    /// 
    /// Creates a new column and inserts an entry into each row of data with their 
    /// index in the table.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use stdout_tables::tables::Table;
    /// 
    /// let mut t = Table::make(Vec::new(), Vec::new());
    /// t.number();
    /// ```
    pub fn number(&mut self) {
        self.headers.insert(0, wrap::WrappedCell::wrap_str(3, String::from("#")).unwrap());

        for i in 0..self.data.len() {
            self.data.get_mut(i).unwrap().insert(0, wrap::WrappedCell::wrap_str(3, i.to_string()).unwrap());
        }
    }

    /// A way to create a table from `String`s
    /// 
    /// # Arguments
    /// 
    /// * `headers` - the labels for the columns of the desired `Table`. The 
    /// first item in the tuple is the width the column should be, the second 
    /// item is the column label. If the width provided was `None`, then the 
    /// width of the column will be the length of the length of the column 
    /// label.
    /// * `data` - each sub-`Vec` is a row, ordered by which column they should 
    /// appear under.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use stdout_tables::tables::Table;
    /// 
    /// let t: Table = Table::make(
    ///     vec![
    ///         (None, String::from("first column")), 
    ///         (Some(7), String::from("second column")),
    ///         (Some(10), String::from("this is a third column"))
    ///     ],
    ///     vec![
    ///         vec![
    ///             String::from("first entry"), String::from("second entry"), 
    ///             String::from("third entry")
    ///         ],
    ///         vec![
    ///             String::from("first-first entry"), String::from("second-second entry"), 
    ///             String::from("third-third entry")
    ///         ]
    ///     ]
    /// );
    /// ```
    pub fn make(headers: Vec<(Option<usize>,String)>,data: Vec<Vec<String>>) -> Table {
        let mut pre_headers = Vec::new();

        for h in &headers {
            let w;
            match h.0 {
                None => w = h.1.len(),
                Some(n) => w = n,
            }

            pre_headers.push(wrap::WrappedCell::wrap_str(w,h.1.clone()).unwrap());
        }

        let the_headers = wrap::WrappedCell::pad_row(pre_headers);

        let mut the_data = Vec::new();

        for d in &data {
            let mut row_of_data = Vec::new();
            for (i,dd) in d.iter().enumerate() {
                row_of_data.push(wrap::WrappedCell::wrap_str(the_headers[i].width,String::from(dd)).unwrap());
            }
            the_data.push(wrap::WrappedCell::pad_row(row_of_data));
        }

        Table {
            headers: the_headers,
            data: the_data,
        }
    }

    pub fn from_string_vec(
        input_vector: Vec<String>, number_of_columns: usize, column_widths: Option<Vec<usize>>
    ) -> Result<Table,DimensionError> {
        if input_vector.len() % number_of_columns != 0 {
            return Err(DimensionError)
        }

        let pre_headers = input_vector[0..number_of_columns].iter().enumerate()
                            .map(|(i,s)| {
                                wrap::WrappedCell::wrap_str(match &column_widths {
                                    None => 10,
                                    Some(v) => v[i],
                                }, s.clone()).unwrap()
                            })
                            .collect::<Vec<_>>();
        let headers = wrap::WrappedCell::pad_row(pre_headers);

        let mut data = Vec::new();
        let mut row_of_data = Vec::new();
        for (i,d) in input_vector[number_of_columns..].iter().enumerate() {
            row_of_data.push(wrap::WrappedCell::wrap_str(headers[i%number_of_columns].width, d.clone()).unwrap());
            if (i+1) % number_of_columns == 0 {
                data.push(wrap::WrappedCell::pad_row(row_of_data));
                row_of_data = Vec::new();
            }
        }

        Ok(Table { headers, data })
    }
}

#[derive(Debug)]
pub struct DimensionError;
impl fmt::Display for DimensionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dimension mismatch!")
    }
}
impl Error for DimensionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[test]
fn test_format_headers() {
    let t = Table::make(
        vec![(None,String::from("header 1")), (None,String::from("header 2")), 
        (None,String::from("very long header very very long"))],
        vec![
            vec![String::from("some content here"), String::from("c"), 
            String::from("more row 1 content")],
            vec![String::from("this is a second row of data"), String::from("yeah"), 
            String::from("very short")]
        ]
    );
    t.draw(themes::Theme::heavy());
}

#[test]
fn test_from_string_vec() {
    let mut t = Table::from_string_vec(
        vec![
            String::from("col 1"), String::from("col 2"), String::from("col 3"),
            String::from("r11"), String::from("r12"), String::from("r13"),
            String::from("r21"), String::from("r22"), String::from("r23")
        ], 3, None).unwrap();
    t.number();
    t.draw(themes::Theme::heavy().borderless());
}