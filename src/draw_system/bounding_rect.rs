
use crate::ScrPos;



#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BoundingRect {
    pub top_left_corner: ScrPos,
    pub size: ScrPos,
}

impl BoundingRect {
    pub fn contains(&self, point: ScrPos) -> bool {
           point.x                          >= self.top_left_corner.x 
        && point.x + self.top_left_corner.x <  self.size.           x
        && point.y                          >= self.top_left_corner.y 
        && point.y + self.top_left_corner.y <  self.size.           y
    }
}