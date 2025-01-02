use crate::*;





// The thing that defines and decides what to draw to a given location.
pub trait Draw: Sized {
    type Unit: DrawUnit;

    // Required methods
    fn draw_unit_at(&self, pos: &ScrPos) -> Self::Unit;

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
                    unit.push_str("\n");
                }
                unit
            })
            .collect()
    }

    fn in_rect(self, size: ScrPos) -> Rect<Self> {
        Rect { contents: self, size }
    }
}

// A wrapper for any Draw with size information.
// Can be used to add size information to any Draw type.
pub struct Rect<Contents: Draw> {
    pub contents: Contents,
    pub size: ScrPos,
}

// A subtrait of Draw which additionally has a size and can be drawn without specifying it.
pub trait DrawSized: Draw {
    // Required method
    fn size(&self) -> ScrPos;

    // Provided methods
    fn draw(&self) -> String {
        self.draw_to_area(&self.size())
    } 
}

impl<Contents: Draw> Draw for Rect<Contents> {
    type Unit = Contents::Unit;
    
    fn draw_unit_at(&self, pos: &ScrPos) -> Self::Unit {
        self.contents.draw_unit_at(pos)
    }
}

impl<Contents: Draw> DrawSized for Rect<Contents> {
    fn size(&self) -> ScrPos {
        self.size
    }
}



// Information how to draw a single character.
// This must ALWAYS move the cursor exactly 1 space to the right (print 1 character)
pub trait DrawUnit {
    fn draw(&self) -> String;
}


