use crate::{draw::Draw, redraw_system::redraw::Redraw, ScrRect};





pub trait SimpleRedraw: Draw + Clone {
    fn bounding_box(&self) -> ScrRect;
}

impl<T: SimpleRedraw> Redraw<T> for T {
    fn update_data_and_redraw_positions(&mut self, update_data: T) -> Vec<crate::ScrPos> {
        let old = self.clone();
        let old_positions = old.bounding_box().into_iter();
        let new_positions = update_data.bounding_box().into_iter();

        *self = update_data;

        old_positions.chain(new_positions).collect()
    }
} 



