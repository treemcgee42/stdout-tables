//! Table-related structures and operations

use crate::wrap;
use crate::themes;

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
        let ws = &self.headers.iter().map(|wcell| wcell.width).collect::<Vec<_>>();

        println!("{}",Table::table_top_border(ws,&theme));

        // draw the column headers
        Table::draw_row(&self.headers, theme.VERTICAL_BORDER);
        println!("{}", Table::table_row_sep(ws,&theme));

        for (i,d) in self.data.iter().enumerate() {
            // draw each row of data
            Table::draw_row(d, theme.VERTICAL_BORDER);
            if i != self.data.len()-1 {
                println!("{}",Table::table_row_sep(ws,&theme));
            }
        }

        println!("{}", Table::table_bottom_border(ws,&theme));
    }

    fn draw_row(v: &Vec<wrap::WrappedCell>, vert_border: char) {
        let split_headers = v.iter()
            .map(|s| s.content.split('\n').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        for j in 0..split_headers[0].len() {
            let mut to_draw = String::new();
            for i in &split_headers {
                to_draw += &vert_border.to_string();
                to_draw += i[j];
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
            theme.TOP_LEFT_CORNER, theme.TOP_CENTER, theme.TOP_RIGHT_CORNER, 
            theme.HORIZONTAL_BORDER
        )
    }

    fn table_bottom_border(col_widths: &Vec<usize>, theme: &themes::Theme) -> String {
        Table::table_row_border(
            col_widths, 
            theme.BOTTOM_LEFT_CORNER, theme.BOTTOM_CENTER, theme.BOTTOM_RIGHT_CORNER, 
            theme.HORIZONTAL_BORDER
        )
    }

    fn table_row_sep(col_widths: &Vec<usize>, theme: &themes::Theme) -> String {
        Table::table_row_border(
            col_widths, 
            theme.MIDDLE_LEFT, theme.MIDDLE_CENTER, theme.MIDDLE_RIGHT, 
            theme.HORIZONTAL_BORDER
        )
    }

    /// A way to create a table from `&str`s
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
    /// use stdout-tables::tables;
    /// 
    /// let t: Table = Table::make(
    ///     vec![
    ///         (None, "first column"), 
    ///         (Some(7), "second column"),
    ///         (Some(10), "this is a third column")
    ///     ],
    ///     vec![
    ///         "first entry", "second entry", "third entry",
    ///         "first-first entry", "second-second entry", "third-third entry"
    ///     ]
    /// );
    /// ```
    pub fn make(headers: Vec<(Option<usize>,String)>,data: Vec<Vec<String>>) -> Table {
        let mut pre_headers = Vec::new();

        for h in &headers {
            let mut w = 0;
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