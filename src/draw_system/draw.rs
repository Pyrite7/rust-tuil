use crate::ScrPos;


use super::styled_char::StyledChar;











pub trait Draw {

    // Required methods
    fn get_cell(&self, pos: ScrPos) -> Option<StyledChar>;

    
    // Provided methods
    fn is_opaque_at(&self, pos: ScrPos) -> bool {
        self.get_cell(pos).is_some()
    }

    fn is_transparent_at(&self, pos: ScrPos) -> bool {
        !self.is_opaque_at(pos)
    }
}



