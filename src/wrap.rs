
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