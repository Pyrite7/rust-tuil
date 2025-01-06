
pub mod vec2;
use bounding_rect::BoundingRect;
pub use vec2::*;

pub mod draw_system;
pub use draw_system::*;

pub mod utility;

pub mod redraw_system;


/// A type alias for the screen position to be used in this library for now.
pub type ScrPos = Vec2<u8>;

pub type ScrVec = Vec2<i16>;

impl ScrPos {
    pub fn is_inside(&self, rect: BoundingRect) -> bool {
        rect.contains(*self)
    }
}




