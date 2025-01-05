use super::style::Style;



pub struct StyledChar {
    pub char: Option<char>,
    pub style: Style,
}

impl From<char> for StyledChar {
    fn from(value: char) -> Self {
        StyledChar { char: Some(value), style: Style::default() }
    }
}


pub trait Stylable {
    fn style(self, style: Style) -> StyledChar;
}

impl Stylable for char {
    fn style(self, style: Style) -> StyledChar {
        StyledChar { char: Some(self), style: style }
    }
}

