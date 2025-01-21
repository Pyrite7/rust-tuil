use std::collections::HashSet;

use crate::{geometry::iterators::RectIter, ScrPos};





pub struct RedrawBuffer {
    pub redraw_positions: HashSet<ScrPos>,
}

impl RedrawBuffer {
    pub fn new() -> Self {
        Self { redraw_positions: HashSet::new() }
    }

    pub fn reset(&mut self) {
        self.redraw_positions.clear();
    }

    pub fn add_redraw_positions(&mut self, positions: Vec<ScrPos>) {
        positions
            .into_iter()
            .for_each(|pos| { self.redraw_positions.insert(pos); });
    }

    pub fn iter(&self, size: ScrPos) -> RedrawPosIter {
        RedrawPosIter { redraw_buffer: self, iter: size.rect_iter() }
    }
}




pub struct RedrawPosIter<'a> {
    redraw_buffer: &'a RedrawBuffer,
    iter: RectIter<u8>,
}

impl<'a> Iterator for RedrawPosIter<'a> {
    type Item = ScrPos;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find(|elem| self.redraw_buffer.redraw_positions.contains(elem))
    }
}


