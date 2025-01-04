
use crate::draw_instruction_buffer::*;


pub trait DrawCell {
    fn draw_instructions(&self, buf: &mut DrawInstructionBuffer);
}