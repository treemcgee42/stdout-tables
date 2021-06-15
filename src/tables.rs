use crate::wrap;

#[derive(Debug)]
pub struct Table {
    headers: Vec<wrap::WrappedCell>,
    data: Vec<Vec<wrap::WrappedCell>>,
}

impl Table {
    pub fn draw(&self) {
        let ws = &self.headers.iter().map(|wcell| wcell.width).collect::<Vec<_>>();

        println!("{}",Table::table_top_row(ws));

        Table::draw_helper(&self.headers);
        println!("{}", Table::row_sep(ws));

        for (i,d) in self.data.iter().enumerate() {
            Table::draw_helper(d);
            if i != self.data.len()-1 {
                println!("{}",Table::row_sep(ws));
            }
        }

        println!("{}", Table::table_bottom_row(ws));
    }

    fn draw_helper(v: &Vec<wrap::WrappedCell>) {
        let split_headers = v.iter()
            .map(|s| s.content.split('\n').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        for j in 0..split_headers[0].len() {
            let mut to_draw = String::new();
            for i in &split_headers {
                to_draw += "┃";
                to_draw += i[j];
            }
            to_draw += "┃";
            println!("{}",to_draw);
        }
    }

    fn table_top_row(col_widths: &Vec<usize>) -> String {
        let mut to_print = String::new();

        for (i,cw) in col_widths.iter().enumerate() {
            if i==0 {
                to_print += "┏";
                to_print += &(0..*cw).map(|_| "━").collect::<String>();
            } else if i==col_widths.len()-1 {
                to_print += "┳";
                to_print += &(0..*cw).map(|_| "━").collect::<String>();
                to_print += "┓";
            } else {
                to_print += "┳";
                to_print += &(0..*cw).map(|_| "━").collect::<String>();
            }
        }

        to_print
    }

    fn table_bottom_row(col_widths: &Vec<usize>) -> String {
        let mut to_print = String::new();

        for (i,cw) in col_widths.iter().enumerate() {
            if i==0 {
                to_print += "┗";
                to_print += &(0..*cw).map(|_| "━").collect::<String>();
            } else if i==col_widths.len()-1 {
                to_print += "┻";
                to_print += &(0..*cw).map(|_| "━").collect::<String>();
                to_print += "┛";
            } else {
                to_print += "┻";
                to_print += &(0..*cw).map(|_| "━").collect::<String>();
            }
        }

        to_print
    }

    fn row_sep(col_widths: &Vec<usize>) -> String {
        let mut to_print = String::new();

        for (i,cw) in col_widths.iter().enumerate() {
            if i==0 {
                to_print += "┣";
                to_print += &(0..*cw).map(|_| "━").collect::<String>();
            } else if i==col_widths.len()-1 {
                to_print += "╋";
                to_print += &(0..*cw).map(|_| "━").collect::<String>();
                to_print += "┫";
            } else {
                to_print += "╋";
                to_print += &(0..*cw).map(|_| "━").collect::<String>();
            }
        }

        to_print
    }

    pub fn make(hs: Vec<(usize,&str)>,data: Vec<Vec<&str>>) -> Table {
        let mut pre_headers = Vec::new();

        for h in &hs {
            pre_headers.push(wrap::WrappedCell::wrap_str(h.0,h.1).unwrap());
        }

        let the_headers = wrap::WrappedCell::pad_row(pre_headers);

        let mut the_data = Vec::new();

        for d in &data {
            let mut row_of_data = Vec::new();
            for (i,dd) in d.iter().enumerate() {
                row_of_data.push(wrap::WrappedCell::wrap_str(the_headers[i].width,dd).unwrap());
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
    // let unformatted = vec![
    //     Wrap::WrappedCell::wrap(5,"my dear friend rooney"),
    //     Wrap::WrappedCell::wrap(5,"i really like that girl"),
    //     Wrap::WrappedCell::wrap(5,"to be")
    // ];
    // let t = Table {
    //     headers: Wrap::WrappedCell::format_headers(unformatted),
    //     data: Vec::new(),
    // };

    // println!("{:?}",t);
    // Table::draw(&t);
    // assert_eq!(2+2,5);

    let t = Table::make(
        vec![(5,"header 1"), (7,"header 2"), (10,"very long header very very long")],
        vec![
            vec!["some content here", "c", "more row 1 content"],
            vec!["this is a second row of data", "yeah", "very short"]
        ]
    );
    t.draw();
}