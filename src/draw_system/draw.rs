use crate::ScrPos;


use super::styled_char::StyledChar;











pub trait Draw {

    // Required methods
    fn get_cell(&self, pos: ScrPos) -> Option<StyledChar>;

}



