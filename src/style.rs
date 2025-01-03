#![allow(unused)]


#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Style {
    pub text_color: Option<Color>,
    pub bg_color: Option<Color>,
    pub thickness: Option<TextThickness>,
    pub italic: Option<bool>,
    pub underline: Option<Underline>,
    pub blinking: Option<bool>,
    /// NOTE: I'm not completely sure what this does, I havent tested it. It's apparently "inverse/reverse" but I'm not sure which is the more apt name.
    pub reverse: Option<bool>,
    pub invisible: Option<bool>,
    pub strikethrough: Option<bool>,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
/// NOTE: this may not actually be text thickness, I might have misunderstood something.
pub enum TextThickness {
    #[default]
    Default,
    Bold,
    Dim,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Underline {
    #[default]
    None,
    Single,
    /// NOTE: Double underline is only supported in some terminals.
    Double,
}

#[derive(Clone, Copy, PartialEq, Eq)]
/// For now, only the Truecolor (rgb) format for colors is supported.
pub enum Color {
    Rgb {
        red: u8,
        green: u8,
        blue: u8,
    },
    Reset,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb { red: r, green: g, blue: b }
    }
}

impl Style {
    pub fn ignore_all() -> Self {
        Style { text_color: None,
                bg_color: None,
                thickness: None,
                italic: None,
                underline: None,
                blinking: None,
                reverse: None,
                invisible: None,
                strikethrough: None }
    }

    fn compare<T: Eq + Clone>(from: Option<T>, to: Option<T>) -> Option<T> {
        if from == to { None } else { to }
    }

    pub fn changes_to(&self, another: &Style) -> Style {
        Style { text_color: Self::compare::<Color>(self.text_color, another.text_color),
                bg_color: Self::compare::<Color>(self.bg_color, another.bg_color),
                thickness: Self::compare::<TextThickness>(self.thickness, another.thickness),
                italic: Self::compare::<bool>(self.italic, another.italic),
                underline: Self::compare::<Underline>(self.underline, another.underline),
                blinking: Self::compare::<bool>(self.blinking, another.blinking),
                reverse: Self::compare::<bool>(self.reverse, another.reverse),
                invisible: Self::compare::<bool>(self.invisible, another.invisible),
                strikethrough: Self::compare::<bool>(self.strikethrough, another.strikethrough) }
    }

    pub fn update(&mut self, with: &Style) {
        self.text_color = with.text_color.or(self.text_color);
        self.bg_color = with.bg_color.or(self.bg_color);
        self.thickness = with.thickness.or(self.thickness);
        self.italic = with.italic.or(self.italic);
        self.underline = with.underline.or(self.underline);
        self.blinking = with.blinking.or(self.blinking);
        self.invisible = with.invisible.or(self.invisible);
        self.strikethrough = with.strikethrough.or(self.strikethrough);
    }
}

impl Default for Style {
    fn default() -> Self {
        Style { text_color: Some(Color::Reset),
                bg_color: Some(Color::Reset),
                thickness: None, // todo(maybe): set this to Some (after below vvv is finished)
                italic: Some(false),
                underline: None, // todo(maybe): set this to Some (after below vvv is finished)
                blinking: Some(false),
                reverse: Some(false),
                invisible: Some(false),
                strikethrough: Some(false) }
    }
}

impl Into<String> for Style {
    fn into(self) -> String {
        let mut args = Vec::<String>::new();

        if let Some(color) = self.text_color {
            args.push(
                match color {
                    Color::Reset => "39".to_string(),
                    Color::Rgb { red, green, blue } => format!("38;2;{red};{green};{blue}"),
                }
            );
        }

        if let Some(color) = self.bg_color {
            args.push(
                match color {
                    Color::Reset => "49".to_string(),
                    Color::Rgb { red, green, blue } => format!("48;2;{red};{green};{blue}"),
                }
            );
        }

        if let Some(_) = self.thickness { unimplemented!("finish this later maybe") }
        
        if let Some(italic) = self.italic {
            args.push(
                if italic { "3".to_string() } else { "23".to_string() }
            );
        }

        if let Some(_) = self.underline { unimplemented!("finish this later maybe") }

        if let Some(blinking) = self.blinking {
            args.push(
                if blinking { "5".to_string() } else { "25".to_string() }
            );
        }

        if let Some(reverse) = self.reverse {
            args.push(
                if reverse { "7".to_string() } else { "27".to_string() }
            );
        }

        if let Some(invisible) = self.invisible {
            args.push(
                if invisible { "8".to_string() } else { "28".to_string() }
            );
        }

        if let Some(strikethrough) = self.strikethrough {
            args.push(
                if strikethrough { "9".to_string() } else { "29".to_string() }
            );
        }

        format!("\x1B[{}m", args.join(";"))
    }
}

