use crate::ScrPos;

use crate::draw_system::{bounding_rect::*, draw_cell::*};











pub trait Draw {
    type Cell: DrawCell;

    // Required methods
    fn get_cell(&self, pos: ScrPos) -> Self::Cell;

    fn bounding_rect(&self) -> BoundingRect;
}




