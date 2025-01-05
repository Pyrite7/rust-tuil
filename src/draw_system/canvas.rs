use std::collections::HashMap;

use crate::ScrPos;

use super::{bounding_rect::BoundingRect, draw::Draw, draw_instruction_buffer::DrawInstructionBuffer, style::Style, styled_char::{Stylable, StyledChar}};

use crate::vec2::*;





pub struct Canvas {
    pub size: ScrPos,
    pub contents: Vec<Box<dyn Draw>>,
    pub background: Box<dyn Draw>,
}


struct DefaultBackground;

impl Draw for DefaultBackground {
    fn bounding_rect(&self) -> super::bounding_rect::BoundingRect {
        BoundingRect::new(ScrPos::default(), ScrPos::default())
    }

    fn get_cell(&self, pos: ScrPos) -> StyledChar {
        StyledChar::from(' ')
    }
}

impl Canvas {
    pub fn new(size: ScrPos) -> Self {
        let background = Box::new(DefaultBackground) as Box<dyn Draw>;
        Self { size, contents: Vec::new(), background }
    }

    pub fn add_content(&mut self, content: Box<dyn Draw>) -> &mut Self {    
        self.contents.push(content);
        self
    } 

    pub fn select_layer(&self, pos: ScrPos) -> &Box<dyn Draw> {
        self.contents
            .iter()
            .find(|draw| draw.bounding_rect().contains(pos))
            .unwrap_or(&self.background)
    }

    pub fn draw_all(&self) {
        let mut buffer = DrawInstructionBuffer::new();

        self.size.row_aware_iter()
            .for_each(|(pos, end_of_row)| {
                let draw = self.select_layer(pos);
                buffer.push_styled_char(draw.get_cell(pos - draw.bounding_rect().top_left_corner));
                if end_of_row {
                    buffer.push_char('\n');
                }
            });
        
        print!("{}", buffer.get_instructions());
    }
}


