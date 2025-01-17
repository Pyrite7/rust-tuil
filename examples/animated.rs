

use std::{thread::sleep, time::Duration};

use rust_tuil::{add_to_canvas, canvas::Canvas, draw::*, redraw_system::redraw::Redraw, style::{Color, Style}, styled_char::Stylable, ScrPos, Vec2};




struct Lines {
    pub cross_pos: ScrPos,
    pub size: ScrPos,
}

impl Lines {
    pub fn new(size: ScrPos) -> Self {
        Self { cross_pos: ScrPos::new(5, 3), size }
    }
}

impl Draw for Lines {
    fn get_cell(&self, pos: ScrPos) -> Option<rust_tuil::styled_char::StyledChar> {
        if pos.x == self.cross_pos.x || pos.y == self.cross_pos.y {
            let col = Color::new(pos.x * 2, pos.y * 8, 125);
            Some('#'.style(Style { text_color: Some(col), invisible: Some(false), ..Default::default() }))
        } else {
            None
        }
    }
}

impl Redraw<()> for Lines {
    fn update_data_and_redraw_positions(&mut self, _update_data: ()) -> Vec<ScrPos> {
        let old_pos = self.cross_pos;
        self.cross_pos += ScrPos::new(1, 1);
        if self.cross_pos.x >= self.size.x {
            self.cross_pos.x = 0;
        }
        if self.cross_pos.y >= self.size.y {
            self.cross_pos.y = 0;
        }
        
        let mut buffer: Vec<ScrPos> = Vec::new();
        for x in 0..self.size.x {
            buffer.push(Vec2::new(x, old_pos.y));
            buffer.push(Vec2::new(x, self.cross_pos.y));
        }
        for y in 0..self.size.y {
            buffer.push(Vec2::new(old_pos.x, y));
            buffer.push(Vec2::new(self.cross_pos.x, y));
        }
        buffer
    }
}




fn main() {
    let size = ScrPos::new(70, 20);

    let mut canvas = Canvas::new(size);
    
    let lines = Lines::new(size);

    let li = add_to_canvas!(canvas, lines);

    canvas.reset();

    loop {
        li.borrow_mut().update(&mut canvas.redraw_buffer, ());
        canvas.flush_redraw_buffer();
        sleep(Duration::from_millis(17));
    }
}




