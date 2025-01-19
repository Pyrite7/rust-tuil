use crate::{style::*, ScrPos, Vec2};

use super::styled_char::StyledChar;






pub struct DrawInstructionBuffer {
    buffer: String,
    current_style: Style,
    current_pos: ScrPos,
    pub mocks: Vec<Mock>,
}

#[derive(Debug)]
pub enum Mock {
    MoveTo(ScrPos),
}


impl DrawInstructionBuffer {

    pub fn new() -> Self {
        Self { buffer: String::new(), current_style: Style::default(), current_pos: ScrPos::default(), mocks: Vec::new() }
    }

    pub fn get_instructions(&self) -> &String {
        &self.buffer
    }

    pub fn hide_cursor(&mut self) {
        self.buffer.push_str("\x1B[?25l");
    }

    pub fn show_cursor(&mut self) {
        self.buffer.push_str("\x1B[?25h");
    }

    pub fn push_styled_char(&mut self, ch: StyledChar) {
        self.set_style(ch.style);

        if let Some(ch) = ch.char {
            self.push_char(ch);
        } else {
            self.move_cursor_by(Vec2::new(1, 0));
        }
    }

    pub fn clear_screen(&mut self) {
        self.buffer.push_str("\x1B[2J");
    }

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

    pub fn reset_cursor_position(&mut self) {
        self.buffer.push_str("\x1B[H");
    }

    pub fn move_cursor_by(&mut self, by: Vec2<i16>) {
        if by != Vec2::new(0, 0) {
            let string = {
                let x_abs = by.x.abs();
                let y_abs = by.y.abs();
                
                let x_part = match by.x.signum() {
                    -1 => format!("\x1B[{}D", x_abs),
                    1 => format!("\x1B[{}C", x_abs),
                    _ => "".to_string(),
                };
    
                let y_part = match by.y.signum() {
                    -1 => format!("\x1B[{}A", y_abs),
                    1 => format!("\x1B[{}B", y_abs),
                    _ => "".to_string(),
                };
    
                x_part + y_part.as_str()
            };
    
            self.buffer.push_str(&string);
        }
    }

    pub fn move_cursor_to(&mut self, to: ScrPos) {
        if self.current_pos != to {
            // As I found out after hours of debugging, apparently the terminal coordinates do not begin at (0,0) but (1,1) instead
            // This caused some very strange behavior :|
            let to_screen = to + ScrPos::new(1, 1);
            self.buffer.push_str(format!("\x1B[{line};{column}H", line=to_screen.y, column=to_screen.x).as_str());
            self.mocks.push(Mock::MoveTo(to_screen));
        }
    }
}


