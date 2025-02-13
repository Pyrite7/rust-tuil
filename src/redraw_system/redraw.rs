use crate::{draw::Draw, ScrPos};

use super::redraw_buffer::RedrawBuffer;







pub trait Redraw<Data>: Draw {
    // Required methods
    fn update(&mut self, buffer: &mut RedrawBuffer, update_data: Data) -> ();
}




