use crate::{draw::Draw, BoundingRect};



pub struct Rect<D: Draw> {
    pub contents: D,
    pub rect: BoundingRect,
}

impl<D: Draw> Rect<D> {
    pub fn new(contents: D, rect: BoundingRect) -> Self {
        Self { contents, rect }
    }
}

impl<D: Draw> Draw for Rect<D> {
    fn get_cell(&self, pos: crate::ScrPos) -> Option<crate::styled_char::StyledChar> {
        if self.rect.contains(pos) {
            self.contents.get_cell(pos)
        } else {
            None
        }
    }
}