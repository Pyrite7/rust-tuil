use crate::ScrPos;

use crate::draw_system::{bounding_rect::*};

use super::styled_char::StyledChar;











pub trait Draw {

    // Required methods
    fn get_cell(&self, pos: ScrPos) -> StyledChar;

    fn bounding_rect(&self) -> BoundingRect;

    
    // Provided methods
}


