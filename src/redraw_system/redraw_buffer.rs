use std::collections::HashSet;

use crate::ScrPos;




pub struct RedrawBuffer {
    redraw_positions: HashSet<ScrPos>,
}

impl RedrawBuffer {
    pub fn add_redraw_positions(&mut self, mut positions: Vec<ScrPos>) {
        positions
            .into_iter()
            .for_each(|pos| { self.redraw_positions.insert(pos); });
    }
}



