use std::{io::Write, thread::sleep, time::Duration};

use rust_tuil::*;
use ansi::*;
use draw::*;



struct ColChar {
    pub col: u8,
    pub char: char,
}

impl ColChar {
    fn none() -> Self {
        ColChar { col: 1, char: ' ' }
    }
}

impl DrawUnit for ColChar {
    fn draw(&self) -> String {
        set_style!(format!("38;5;{c}", c=self.col)) + self.char.to_string().as_str()
    }
}

struct Lines {
    pub size: ScrPos,
    pub pos: ScrPos,
    pub col: u8,
}

impl Lines {
    pub fn new(x: u8, y: u8) -> Self {
        Self { size: Vec2::new(x, y), pos: Vec2::new(3, 5), col: 0 }
    }

    pub fn next_frame(&mut self) {
        self.pos.x = (self.pos.x + 1) % self.size.x;
        self.pos.y = (self.pos.y + 1) % self.size.y;
        self.col = (self.col + 1) % u8::MAX;
    }
}

impl Draw for Lines {
    type Unit = ColChar;

    fn draw_unit_at(&self, pos: &ScrPos) -> Self::Unit {
        if pos.x == self.pos.x || pos.y == self.pos.y {
            ColChar { char: 'x', col: self.col }
        } else {
            ColChar::none()
        }
    }
}

impl DrawSized for Lines {
    fn size(&self) -> ScrPos {
        self.size
    }
}



fn main() {
    let mut thigny = Lines::new(30, 10);

    print!("{}", ERASE_SCREEN);
    std::io::stdout().flush().unwrap();

    loop {
        sleep(Duration::from_millis(100 ));
        thigny.next_frame();

        print!("{}{}", ERASE_SCREEN, RESET_CURSOR);
        print!("{}", thigny.draw());
        std::io::stdout().flush().unwrap();
    }
}