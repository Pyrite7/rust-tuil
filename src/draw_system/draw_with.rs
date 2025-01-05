
use crate::{ScrPos, draw::*};

use super::styled_char::StyledChar;


pub trait DrawWith<T> {

    // Required methods
    fn get_cell_with(&self, pos: ScrPos, params: &T) -> StyledChar;
}

impl<T, D: DrawWith<T>> Draw for (D, T) {

    fn get_cell(&self, pos: ScrPos) -> Option<StyledChar> {
        Some(self.0.get_cell_with(pos, &self.1))
    }
}
