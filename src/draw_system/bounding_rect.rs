
use crate::ScrPos;



#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BoundingRect {
    pub top_left_corner: ScrPos,
    pub size: ScrPos,
}

impl BoundingRect {
    pub fn new(top_left_corner: ScrPos, size: ScrPos) -> Self {
        Self { top_left_corner, size }
    }

    pub fn contains(&self, point: ScrPos) -> bool {
           point.x >= self.top_left_corner.x 
        && point.x < self.size.x + self.top_left_corner.x
        && point.y >= self.top_left_corner.y 
        && point.y < self.size.y + self.top_left_corner.y
    }
}