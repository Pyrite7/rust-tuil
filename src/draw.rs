use std::marker::PhantomData;

use crate::vec2::*;


/// A type alias for the screen position to be used in this library for now.
pub type ScrPos = Vec2<u8>;


// Think of this like Draw but bundled with size information
pub struct DrawRect<Unit: DrawUnit, Cnt: Draw<Unit>> {
    pub size: ScrPos,
    pub content: Cnt,
    _unit: PhantomData<Unit>,
}


// The thing that defines and decides what to draw to a given location.
pub trait Draw<Unit: DrawUnit> {
    // Required methods
    fn draw_unit_at(&self, pos: &ScrPos) -> Unit;

    // Provided methods
    fn draw_at(&self, pos: &ScrPos) -> String {
        self.draw_unit_at(pos).draw()
    }

    fn draw_to_area(&self, area_size: &ScrPos) -> String {
        area_size
            .row_aware_iter()
            .map(|(pos, end_of_row)| {
                let mut unit = self.draw_at(&pos);
                if end_of_row {
                    unit.push_str("\x1B[0m\n"); //TODO: remove this hardcoded ANSI-code \x1B[0m . (it resets style and color options)
                }
                unit
            })
            .collect()
    }
}


// Information how to draw a single character.
// This must ALWAYS move the cursor exactly 1 space to the right (print 1 character)
pub trait DrawUnit {
    fn draw(&self) -> String;
}


