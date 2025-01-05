use bounding_rect::BoundingRect;
use canvas::Canvas;
use draw::Draw;
use draw_with::DrawWith;
use rust_tuil::*;
use style::{Color, Style};
use styled_char::{Stylable, StyledChar};



struct Squares {
    pub col1: Color,
    pub col2: Color,
}

impl DrawWith<BoundingRect> for Squares {
    fn bounding_rect(&self, params: &BoundingRect) -> BoundingRect {
        *params
    }

    fn get_cell_with(&self, pos: ScrPos, params: &BoundingRect) -> styled_char::StyledChar {
        let col = if (pos.x + pos.y) % 2 == 0 { self.col1 } else { self.col2 };

        ' '.style(Style::from_bg_color(col))
    }
}

struct Gradient {
    pub col1: Color,
    pub col2: Color,
}

impl DrawWith<BoundingRect> for Gradient {
    fn bounding_rect(&self, params: &BoundingRect) -> BoundingRect {
        *params
    }

    fn get_cell_with(&self, pos: ScrPos, params: &BoundingRect) -> StyledChar {
        let u = pos.x as f64 / params.size.x as f64;
        let v = pos.y as f64 / params.size.y as f64;
        
        let x = (u + v) / 2.0;
        
        if let Color::Rgb { red, green, blue } = self.col1 {
            let (r1, g1, b1) = (red, green, blue);
            if let Color::Rgb { red, green, blue } = self.col2 {
                let (r2, g2, b2) = (red, green, blue);
                let r = x * r1 as f64 + (1.0 - x) * r2 as f64;
                let g = x * g1 as f64 + (1.0 - x) * g2 as f64;
                let b = x * g1 as f64 + (1.0 - x) * b2 as f64;
                'X'.style(Style::from_text_color(Color::Rgb { red: r as u8, green: g as u8, blue: b as u8 }))             
            } else { panic!("AAAAAAAAA") }
        } else { panic!("AAAAAAA") }
    }
}


fn main() {
    let mut canvas = Canvas::new(Vec2::new(50, 20));

    canvas.add_content({
        Box::new((Squares {col1: Color::new(100, 100, 100), col2: Color::new(0, 0, 0)}, BoundingRect::new(ScrPos::new(0, 0), ScrPos::new(25, 10)))) as Box<dyn Draw>
    });

    canvas.add_content({
        Box::new((Gradient {col1: Color::new(200, 0, 0), col2: Color::new(0, 50, 255)}, BoundingRect::new(ScrPos::new(5, 1), ScrPos::new(20, 15)))) as Box<dyn Draw>
    });

    canvas.draw_all();
}