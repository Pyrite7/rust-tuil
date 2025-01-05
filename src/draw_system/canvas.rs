use crate::{utility::non_transparent::NonTransparent, ScrPos};

use super::{draw::Draw, draw_instruction_buffer::DrawInstructionBuffer, styled_char::StyledChar};






pub struct Canvas {
    pub size: ScrPos,
    pub contents: Vec<Box<dyn Draw>>,
    pub background: Box<dyn NonTransparent>,
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
        Self { size, contents: Vec::new(), background }
    }

    pub fn add_content(&mut self, content: Box<dyn Draw>) -> &mut Self {    
        self.contents.push(content);
        self
    } 

    pub fn get_cell(&self, pos: ScrPos) -> StyledChar {
        self.contents
            .iter()
            .map(|draw| draw.get_cell(pos))
            .find(|opt| opt.is_some())
            .unwrap_or_else(|| Some(self.background.get_cell(pos)))
            .unwrap()
    }

    pub fn draw_all(&self) {
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
}

#[macro_export]
macro_rules! add_canvas_content {
    ($canvas:expr, $content:expr) => {
        $canvas.add_content({
            boxdyn!($content)
        })
    };
}
