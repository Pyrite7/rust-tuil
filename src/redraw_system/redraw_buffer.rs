use std::collections::HashSet;

use crate::{geometry::iterators::RectIter, ScrPos, ScrRect};





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
    
    pub fn add_redraw_rect(&mut self, rect: ScrRect) {
        self.add_redraw_positions(rect.into_iter().collect());
    }

    pub fn add_filtered_redraw_rect(&mut self, rect: ScrRect, filter: impl Fn(&ScrPos) -> bool) {
        self.add_redraw_positions(rect.into_iter().filter(filter).collect());
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


