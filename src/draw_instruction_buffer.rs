use crate::style::*;






pub struct DrawInstructionBuffer {
    buffer: String,
    current_style: Style,
}


impl DrawInstructionBuffer {
    pub fn push_char(&mut self, char: char) {
        self.buffer.push(char);
    }

    pub fn reset_style(&mut self) {
        self.buffer.push_str("\x1B[0m");
        self.current_style = Style::default();
    }

    pub fn set_style(&mut self, style: Style) {
        let string: String = self.current_style.changes_to(&style).into();
        self.buffer.push_str(&string);

        self.current_style.update(&style);
    }
}


