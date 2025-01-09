use std::{cell::RefCell, rc::Rc};

use crate::{redraw_system::redraw_buffer::RedrawBuffer, utility::non_transparent::NonTransparent, ScrPos};

use super::{draw::Draw, draw_instruction_buffer::DrawInstructionBuffer, styled_char::StyledChar};






pub struct Canvas {
    pub size: ScrPos,
    pub contents: Vec<Rc<RefCell<dyn Draw>>>,
    pub background: Box<dyn NonTransparent>,
    pub redraw_buffer: RedrawBuffer,
}


struct DefaultBackground;

impl NonTransparent for DefaultBackground {

    fn get_cell(&self, _pos: ScrPos) -> StyledChar {
        StyledChar::from(' ')
    }
}

impl Canvas {
    pub fn new(size: ScrPos) -> Self {
        let background = Box::new(DefaultBackground) as Box<dyn NonTransparent>;
        Self { size, contents: Vec::new(), background, redraw_buffer: RedrawBuffer::new() }
    }

    pub fn add_content(&mut self, content:Rc<RefCell<dyn Draw>> ) -> &mut Self {
        self.contents.push(content);
        self
    } 

    pub fn get_cell(&self, pos: ScrPos) -> StyledChar {
        self.contents
            .iter()
            .map(|draw| draw.borrow().get_cell(pos))
            .find(|opt| opt.is_some())
            .unwrap_or_else(|| Some(self.background.get_cell(pos)))
            .unwrap()
    }

    // TODO: how should this work with the redraw buffer?
    pub fn reset(&self) {
        let mut buf = DrawInstructionBuffer::new();
        buf.clear_screen();
        buf.reset_cursor_position();
        buf.reset_style();
        print!("{}", buf.get_instructions());
    }

    pub fn draw_all(&self) {
        self.reset();

        let mut buffer = DrawInstructionBuffer::new();

        self.size.row_aware_iter()
            .for_each(|(pos, end_of_row)| {
                buffer.push_styled_char(self.get_cell(pos));
                if end_of_row {
                    buffer.push_char('\n');
                }
            });
        
        print!("{}", buffer.get_instructions());
    }

    pub fn flush_redraw_buffer(&mut self) {
        let mut instruction_buffer = DrawInstructionBuffer::new();
        
        self.redraw_buffer
            .iter()
            .for_each(|pos| {
                instruction_buffer.move_cursor_to(*pos);
                instruction_buffer.push_styled_char(self.get_cell(*pos));
            });
        
        self.redraw_buffer.reset();
        
        print!("{}", instruction_buffer.get_instructions());
        
        
    }
}

#[macro_export]
macro_rules! add_content_directly {
    ($canvas:expr, $content:expr) => {
        $canvas.add_content({
            ::std::rc::Rc::new(::std::cell::RefCell::new($content)) as ::std::rc::Rc<::std::cell::RefCell<dyn Draw>>
        })
    };
}

#[macro_export]
macro_rules! add_to_canvas {
    ($canvas:expr, $content:expr) => {{
        let rc = ::std::rc::Rc::new(::std::cell::RefCell::new($content));
        $canvas.add_content({
            ::std::rc::Rc::clone(&rc) as ::std::rc::Rc<::std::cell::RefCell<dyn Draw>>
        });
        rc
    }};
}