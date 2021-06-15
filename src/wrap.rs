
#[derive(Debug)]
pub struct WrappedCell {
    pub width: usize,
    height: usize,
    pub content: String,
}

impl WrappedCell {
    // Break a string up into multiple lines and pad it appropriately
    // with spaces
    // Parameters:
    // w: how long you want the WrappedCell to be (where to split)
    // s: what you want to put in a WrappedCell
    // Returns: a WrappedCell with content s and width w
    pub fn wrap_str(w: usize, s: &str) -> Result<WrappedCell,&'static str> {
        // edge case
        if w <= 0 {
            return Err(
                format!("The width you gave ({}) was invalid. \
                Choose one >=0.",w)
            )
        }

        let mut len_line_so_far = 0;
        let mut wrapped_str = s.chars()
            .enumerate()
            .fold(
                String::new(),
                |acc, (i,c)| {
                    if i != 0 && (i % w == 0) {
                        len_line_so_far = 1;
                        format!("{}\n{}", acc, c)
                    } else {
                        len_line_so_far += 1;
                        format!("{}{}", acc, c)
                    }
                }
            );
        
        wrapped_str += &(0..(w-len_line_so_far)).map(|_| " ").collect::<String>();

        let mut height = s.len() / w;
        if s.len() % w != 0 {
            height += 1;
        }

        Ok(WrappedCell {
            width: w,
            height,
            content: wrapped_str,
        })
    }

    // Given a row (vector) of WrappedCell(s), pad each appropriately
    // so that they are printed evenly even if they may have different
    // lengths. I.e., pad each cell in the row according to the size of
    // the largest cell 
    pub fn pad_row(hs: Vec<WrappedCell>) -> Vec<WrappedCell> {
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

///////////
// TESTS //
///////////

#[test]
fn wrap_str_test() {
    // perfect width alignment (no padding necessary)
    assert_eq!(
        WrappedCell::wrap_str(9, "happy boy").unwrap().content,
        String::from("happy boy")
    );

    // needs some padding at the end
    assert_eq!(
        WrappedCell::wrap_str(10, "Mary had a little lamb!").unwrap().content,
        String::from(
            "Mary had a\
            \n little la\
            \nmb!       "
        )
    );
}