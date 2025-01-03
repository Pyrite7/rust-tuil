
pub mod vec2;
pub use vec2::*;

pub mod draw;
pub use draw::*;

pub mod draw2;

pub mod draw_instruction_buffer;
pub use draw_instruction_buffer::*;

pub mod style;

pub mod ansi;


/// A type alias for the screen position to be used in this library for now.
pub type ScrPos = Vec2<u8>;


impl DrawUnit for char {
    fn draw(&self) -> String {
        self.to_string()
    }
}

