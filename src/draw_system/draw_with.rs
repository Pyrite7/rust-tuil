
use crate::{ScrPos, bounding_rect::*, draw::*};

use super::styled_char::StyledChar;


pub trait DrawWith<T> {

    // Required methods
    fn get_cell_with(&self, pos: ScrPos, params: &T) -> StyledChar;

    fn bounding_rect(&self, params: &T) -> BoundingRect;
}

impl<T, D: DrawWith<T>> Draw for (D, T) {

    fn get_cell(&self, pos: ScrPos) -> StyledChar {
        self.0.get_cell_with(pos, &self.1)
    }

    fn bounding_rect(&self) -> BoundingRect {
        self.0.bounding_rect(&self.1)
    }
}
