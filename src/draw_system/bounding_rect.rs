
use crate::ScrPos;



#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BoundingRect {
    pub top_left_corner: ScrPos,
    pub size: ScrPos,
}

impl BoundingRect {
    pub fn new(left_x: u8, top_y: u8, right_x: u8, bottom_y: u8) -> Self {
        Self { top_left_corner: ScrPos::new(left_x, top_y), size: ScrPos::new(right_x, bottom_y) }
    }

    pub fn contains(&self, point: ScrPos) -> bool {
           point.x >= self.top_left_corner.x 
        && point.x < self.size.x + self.top_left_corner.x
        && point.y >= self.top_left_corner.y 
        && point.y < self.size.y + self.top_left_corner.y
    }
}