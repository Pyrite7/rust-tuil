use crate::ScrPos;


use super::styled_char::StyledChar;











pub trait Draw {

    // Required methods
    fn get_cell(&self, pos: ScrPos) -> Option<StyledChar>;

}

#[macro_export]
macro_rules! boxdyn {
    ($draw:expr) => {
        Box::new($draw) as Box<dyn Draw>
    };
}

