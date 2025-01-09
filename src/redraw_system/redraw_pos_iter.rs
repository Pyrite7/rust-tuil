use crate::{ScrPos, geometry::iterators::Vec2Iter};

use super::redraw_buffer::RedrawBuffer;




pub struct RedrawPosIter<'a> {
    redraw_buffer: &'a RedrawBuffer,
    iter: Vec2Iter<u8>,
}

impl<'a> Iterator for RedrawPosIter<'a> {
    type Item = ScrPos;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find(|elem| self.redraw_buffer.redraw_positions.contains(elem))
    }
}









