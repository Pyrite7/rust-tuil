use crate::{draw::Draw, redraw_system::redraw::Redraw, ScrRect};





pub trait SimpleRedraw: Draw + Clone {
    fn bounding_box(&self) -> ScrRect;
}

impl<T: SimpleRedraw> Redraw<T> for T {
    fn update(&mut self, buffer: &mut crate::redraw_system::redraw_buffer::RedrawBuffer, update_data: T) -> () {
        let old = self.clone();

        *self = update_data;

        buffer.add_filtered_redraw_rect(old.bounding_box(), |pos| old.is_opaque_at(*pos));
        buffer.add_filtered_redraw_rect(self.bounding_box(), |pos| self.is_opaque_at(*pos));
    }
} 



