//! Determines the look of the table.
//! 
//! You use one of the already-implemented themes, or even 
//! create your own to customize the look of your table even 
//! further.

/// How themes will be represented.
pub struct Theme {
    // top row of table
    pub top_left_corner: char,
    pub top_center: char,
    pub top_right_corner: char,
    // rows excluding top and bottom
    pub middle_left: char,
    pub middle_center: char,
    pub middle_right: char,
    // bottom row of table
    pub bottom_left_corner: char,
    pub bottom_center: char,
    pub bottom_right_corner: char,
    // other border stuff
    pub horizontal_border: char,
    pub vertical_border: char,
    pub internal_horizontal: char,
    pub internal_vertical: char,
}

/// This is where/how the pre-implemented `Theme`s are defined.
/// They are accessed by calling the corresponding function, which
/// returns the desired `Theme` to be used.
impl Theme {
    pub fn heavy() -> Theme {
        Theme {
            top_left_corner: '┏',
            top_center: '┳',
            top_right_corner: '┓',

            middle_left: '┣',
            middle_center: '╋',
            middle_right: '┫',

            bottom_left_corner: '┗',
            bottom_center: '┻',
            bottom_right_corner: '┛',

            horizontal_border: '━',
            vertical_border: '┃',
            internal_horizontal: '━',
            internal_vertical: '┃',
        }
    }

    pub fn borderless(&self) -> Theme {
        Theme {
            top_left_corner: '\0',
            top_center: '\0',
            top_right_corner: '\0',

            middle_left: '\0',
            middle_right: '\0',

            bottom_left_corner: '\0',
            bottom_center: '\0',
            bottom_right_corner: '\0',

            horizontal_border: '\0',
            vertical_border: '\0',
            ..*self
        }
    }
}