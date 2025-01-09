
use std::ops::Add;

use crate::Vec2;



#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rect<Elem> {
    pub top_left_corner: Vec2<Elem>,
    pub size: Vec2<Elem>,
}

impl<Elem> Rect<Elem> {
    pub fn new(left_x: Elem, top_y: Elem, right_x: Elem, bottom_y: Elem) -> Self {
        Self { top_left_corner: Vec2::new(left_x, top_y), size: Vec2::new(right_x, bottom_y) }
    }
}

impl<Elem: Add<Output = Elem> + Ord + Copy> Rect<Elem> {
    pub fn contains(&self, point: Vec2<Elem>) -> bool {
        point.x >= self.top_left_corner.x 
     && point.x < self.size.x + self.top_left_corner.x
     && point.y >= self.top_left_corner.y 
     && point.y < self.size.y + self.top_left_corner.y
 }
}