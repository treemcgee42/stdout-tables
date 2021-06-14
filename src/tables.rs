
#[derive(Debug)]
pub struct Table {
    headers: Vec<Wrap::WrappedCell>,
    data: Vec<Vec<Wrap::WrappedCell>>,
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

    fn draw_helper(v: &Vec<Wrap::WrappedCell>) {
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
            pre_headers.push(Wrap::WrappedCell::wrap(h.0,h.1));
        }

        let the_headers = Wrap::WrappedCell::format_headers(pre_headers);

        let mut the_data = Vec::new();

        for d in &data {
            let mut row_of_data = Vec::new();
            for (i,dd) in d.iter().enumerate() {
                row_of_data.push(Wrap::WrappedCell::wrap(the_headers[i].width,dd));
            }
            the_data.push(Wrap::WrappedCell::format_headers(row_of_data));
        }

        Table {
            headers: the_headers,
            data: the_data,
        }
    }
}

pub mod Wrap {
    use super::*;

    #[derive(Debug)]
    pub struct WrappedCell {
        pub width: usize,
        height: usize,
        pub content: String,
    }

    impl WrappedCell {
        pub fn wrap(w: usize, s: &str) -> WrappedCell {
            let mut l_line_so_far = 0;

            let mut wrapped_str = s.chars()
                .enumerate()
                .fold(
                    String::new(),
                    |acc, (i,c)| {
                        if i != 0 && (i % w == 0) {
                            l_line_so_far = 1;
                            format!("{}\n{}", acc, c)
                        } else {
                            l_line_so_far += 1;
                            format!("{}{}", acc, c)
                        }
                    }
                );
            
            wrapped_str += &(0..(w-l_line_so_far)).map(|_| " ").collect::<String>();

            let mut height = s.len() / w;
            if s.len() % w != 0 {
                height += 1;
            }

            WrappedCell {
                width: w,
                height,
                content: wrapped_str,
            }
        }

        pub fn format_headers(hs: Vec<WrappedCell>) -> Vec<WrappedCell> {
            let mut max_height = 0;
            for h in &hs {
                if h.height > max_height {
                    max_height = h.height;
                }
            }

            let mut formatted = Vec::new();
            for h in &hs {
                let width = h.width;
                let mut content = h.content.clone();
                if h.height < max_height {
                    for i in 0..(max_height-h.height) {
                        content += &format!(
                            "\n{}",
                            (0..h.width).map(|_| " ").collect::<String>()
                        );
                    }
                }
                formatted.push(
                    WrappedCell { height: max_height, width, content}
                );
            }

            formatted
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
