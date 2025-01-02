use draw::{Draw, DrawUnit};
use rust_tuil::*;



enum ChessTile {
    Black,
    White,
}


impl DrawUnit for ChessTile {
    fn draw(&self) -> String {
        match self {
            Self::Black => "B",
            Self::White => "W",
        }.to_string()
    }
}

struct ChessBoard;

impl Draw for ChessBoard {
    type Unit = ChessTile;

    fn draw_unit_at(&self, pos: &ScrPos) -> ChessTile {
        if (pos.x + pos.y) % 2 == 0 {
            ChessTile::Black
        } else {
            ChessTile::White
        }
    }
}


fn main() {
    let corner = Vec2::new(7, 5);

    let draw_instructions = ChessBoard.draw_to_area(&corner);
    
    let correct = "BWBWBWB\nWBWBWBW\nBWBWBWB\nWBWBWBW\nBWBWBWB\n".to_string();

    assert_eq!(draw_instructions, correct);
}





