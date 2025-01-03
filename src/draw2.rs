use std::collections::HashSet;

use crate::ScrPos;



pub type DrawInstBuffer = String;

pub type RedrawBuffer = HashSet<ScrPos>;


pub struct BoundingRect {
    top_left_corner: ScrPos,
    size: ScrPos,
}


pub trait DrawCell {
    fn draw_instructions(&self, buf: &mut DrawInstBuffer);
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


