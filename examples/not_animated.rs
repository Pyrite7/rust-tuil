use canvas::Canvas;
use draw::Draw;
use rust_tuil::*;
use style::{Color, Style};
use styled_char::{Stylable, StyledChar};



struct Transparent;

impl Draw for Transparent {
    fn get_cell(&self, pos: ScrPos) -> Option<StyledChar> {
        let black = Some(' '.style(Style::from_bg_color(Color::new(0, 0, 0))));
        if (pos.x + pos.y) % 2 == 0 {
            black
        } else { None }
    }
}


struct Gradient {
    pub col1: Color,
    pub col2: Color,
    pub rect: ScrRect,
}

impl Draw for Gradient {

    fn get_cell(&self, pos: ScrPos) -> Option<StyledChar> {
        let u = pos.x as f64 / self.rect.size.x as f64;
        let v = pos.y as f64 / self.rect.size.y as f64;

        if u < 0.0 || u > 1.0 || v < 0.0 || v > 1.0 {
            return None
        }
        
        let x = (u + v) / 2.0;
        
        if let Color::Rgb { red, green, blue } = self.col1 {
            let (r1, g1, b1) = (red, green, blue);
            if let Color::Rgb { red, green, blue } = self.col2 {
                let (r2, g2, b2) = (red, green, blue);
                let r = x * r1 as f64 + (1.0 - x) * r2 as f64;
                let g = x * g1 as f64 + (1.0 - x) * g2 as f64;
                let b = x * b1 as f64 + (1.0 - x) * b2 as f64;
                Some('X'.style(Style::from_text_color(Color::Rgb { red: r as u8, green: g as u8, blue: b as u8 }))    )         
            } else { panic!("AAAAAAAAA") }
        } else { panic!("AAAAAAA") }
    }
}


fn main() {
    let mut canvas = Canvas::new(Vec2::new(50, 20));

    add_content_directly!(canvas, Transparent);
    add_content_directly!(canvas, Gradient {  col1: Color::new(200, 0, 0),
                                            col2: Color::new(0, 50, 255),
                                            rect: ScrRect::new(0, 0, 50, 20) });


    canvas.draw_all();
}


