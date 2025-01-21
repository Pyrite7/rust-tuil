use draw::Draw;
use redraw_system::redraw::Redraw;
use rust_tuil::*;
use styled_char::StyledChar;




type PhysVec = Vec2<f64>;




#[derive(Clone, Copy)]
struct Circle {
    center: PhysVec,
    radius: f64,
}

impl Circle {
    fn contains(&self, pos: PhysVec) -> bool {
        let from_center = pos - self.center;
        let radius = (from_center.x.powi(2) + from_center.y.powi(2)).sqrt();
        radius <= self.radius
    }
}

impl Draw for Circle {
    fn get_cell(&self, pos: ScrPos) -> Option<styled_char::StyledChar> {
        if self.contains(pos.try_into().unwrap()) {
            Some('*'.into())
        } else { None }
    }
}

struct CircleUpdateData {
    new_center: Option<PhysVec>,
    new_radius: Option<f64>,
}

impl Redraw<CircleUpdateData> for Circle {
    fn update_data_and_redraw_positions(&mut self, update_data: CircleUpdateData) -> Vec<ScrPos> {
        let old_center = self.center;
        let old_radius = self.radius;

        if let Some(new_center) = update_data.new_center {
            self.center = new_center;
        }
        if let Some(new_radius) = update_data.new_radius {
            self.radius = new_radius;
        }

        todo!()
    }
}





fn main() {

}



