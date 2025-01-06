use crate::{draw::Draw, ScrPos};

use super::redraw_buffer::RedrawBuffer;







pub trait Redraw<Data>: Draw {
    // Required methods
    fn update_data_and_redraw_positions(&mut self, update_data: Data) -> Vec<ScrPos>;

    // Provided methods
    fn update(&mut self, buffer: &mut RedrawBuffer, update_data: Data) {
        let redraw_positions = self.update_data_and_redraw_positions(update_data);
        buffer.add_redraw_positions(redraw_positions);
    }
}




