use crate::{draw::Draw, styled_char::StyledChar, ScrPos};



/// Like [Draw], but guarantees non-transparency (i.e. it always returns a StyledChar).
pub trait NonTransparent {
    fn get_cell(&self, pos: ScrPos) -> StyledChar;
}

impl<B: NonTransparent> Draw for B {
    fn get_cell(&self, pos: ScrPos) -> Option<StyledChar> {
        Some(self.get_cell(pos))
    }
}



