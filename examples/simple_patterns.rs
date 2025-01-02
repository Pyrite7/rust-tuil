use draw::{Draw, DrawUnit};
use rust_tuil::*;



enum ChessTile {
    Black,
    White,
}


impl DrawUnit for ChessTile {
    fn draw(&self) -> String {
        match self {
            Self::Black => "\x1B[48;5;16m\x1B[38;5;15mB",
            Self::White => "\x1B[48;5;15m\x1B[38;5;16mW",
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
    
    println!("{}", draw_instructions);
}


