//! Handles the wrapping needed for the tables.
//! 
//! The strings in the table need to be wrapped in order to fit 
//! nicely into columns. This involves inserting line breaks and 
//! padding with spaces, for instance.

/// Can be thought of as the cells of the table, though in this context 
/// it is slightly more general.
/// 
/// Practically speaking, this should be constructed from a method, such 
/// as `wrap_str()`.
#[derive(Debug)]
pub struct WrappedCell {
    /// How many characters should be displayed before inserting a 
    /// line break
    pub width: usize,
    height: usize,
    /// The string that is being wrapped, stored here in already-wrapped form, 
    /// i.e. with line breaks, space padding, etc.
    pub content: String,
}

impl WrappedCell {
    /// Break a string up into multiple lines and pad it appropriately
    /// with spaces. Can return an appropriate `WrappedCell`.
    /// 
    /// # Arguments
    /// 
    /// * `w` - how long you want the WrappedCell to be (where to split)
    /// * `s` - what you want to put in a WrappedCell
    /// Returns: a WrappedCell with content s and width w
    /// 
    /// # Examples
    /// 
    /// ```
    /// use tables::wrap;
    /// 
    /// // The resulting content  field will be:
    /// // "Mary \nhad a\n litt\nle la\nb!   "
    /// let wc1: WrappedCell = wrap_str(5, String::from("Mary had a little lamb!"))
    ///     .unwrap();
    /// ```
    pub fn wrap_str(w: usize, s: String) -> Result<WrappedCell,&'static str> {
        // edge case
        if w <= 0 {
            return Err("The width you gave was invalid.");
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

    /// Given a row (vector) of WrappedCell(s), pad each appropriately
    /// so that they are printed evenly even if they may have different
    /// lengths. I.e., pad each cell in the row according to the size of
    /// the largest cell.
    /// 
    /// # Arguments
    /// 
    /// * `row` - represented by a `Vec` of `WrappedCell`s
    pub fn pad_row(row: Vec<WrappedCell>) -> Vec<WrappedCell> {
        let mut max_height = 0;
        for h in &row {
            if h.height > max_height {
                max_height = h.height;
            }
        }

        let mut formatted = Vec::new();
        for h in &row {
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
        WrappedCell::wrap_str(9, String::from("happy boy")).unwrap().content,
        String::from("happy boy")
    );

    // needs some padding at the end
    assert_eq!(
        WrappedCell::wrap_str(10, String::from("Mary had a little lamb!")).unwrap().content,
        String::from(
            "Mary had a\
            \n little la\
            \nmb!       "
        )
    );
}