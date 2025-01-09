pub mod draw_system;
pub use draw_system::*;

pub mod utility;

pub mod redraw_system;

pub mod geometry;
use geometry::rect::Rect;
pub use geometry::vec2::*;


/// A type alias for the screen position to be used in this library for now.
pub type ScrPos = Vec2<u8>;

pub type ScrVec = Vec2<i16>;

impl ScrPos {
    pub fn is_inside(&self, rect: ScrRect) -> bool {
        rect.contains(*self)
    }
}

pub type ScrRect = Rect<u8>;


