use std::collections::HashSet;

use crate::{DrawInstructionBuffer, ScrPos};




pub type RedrawBuffer = HashSet<ScrPos>;


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


pub trait DrawCell {
    fn draw_instructions(&self, buf: &mut DrawInstructionBuffer);
}


pub trait Draw {
    type Cell: DrawCell;

    // Required methods
    fn get_cell(&self, pos: ScrPos) -> Self::Cell;

    fn bounding_rect(&self) -> BoundingRect;
}


pub trait DrawWith<T> {
    type Cell: DrawCell;

    // Required methods
    fn get_cell_with(&self, pos: ScrPos, params: &T) -> Self::Cell;

    fn bounding_rect(&self, params: &T) -> BoundingRect;
}

impl<T, D: DrawWith<T>> Draw for (D, T) {
    type Cell = D::Cell;

    fn get_cell(&self, pos: ScrPos) -> Self::Cell {
        self.0.get_cell_with(pos, &self.1)
    }

    fn bounding_rect(&self) -> BoundingRect {
        self.0.bounding_rect(&self.1)
    }
}


