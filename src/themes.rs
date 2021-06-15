
pub struct Theme {
    // top row of table
    pub TOP_LEFT_CORNER: char,
    pub TOP_CENTER: char,
    pub TOP_RIGHT_CORNER: char,
    // rows excluding top and bottom
    pub MIDDLE_LEFT: char,
    pub MIDDLE_CENTER: char,
    pub MIDDLE_RIGHT: char,
    // bottom row of table
    pub BOTTOM_LEFT_CORNER: char,
    pub BOTTOM_CENTER: char,
    pub BOTTOM_RIGHT_CORNER: char,
    // other border stuff
    pub HORIZONTAL_BORDER: char,
    pub VERTICAL_BORDER: char,
}

impl Theme {
    pub fn heavy() -> Theme {
        Theme {
            TOP_LEFT_CORNER: '┏',
            TOP_CENTER: '┳',
            TOP_RIGHT_CORNER: '┓',

            MIDDLE_LEFT: '┣',
            MIDDLE_CENTER: '╋',
            MIDDLE_RIGHT: '┫',

            BOTTOM_LEFT_CORNER: '┗',
            BOTTOM_CENTER: '┻',
            BOTTOM_RIGHT_CORNER: '┛',

            HORIZONTAL_BORDER: '━',
            VERTICAL_BORDER: '┃',
        }
    }
}