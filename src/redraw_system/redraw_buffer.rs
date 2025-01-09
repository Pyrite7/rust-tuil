use std::collections::HashSet;

use crate::ScrPos;




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

    pub fn iter(&self) -> std::collections::hash_set::Iter<'_, ScrPos> {
        self.redraw_positions.iter()
    }
}



