use crate::chessboard::{Coordinate, Chessboard, Move};

use super::Color;

pub fn append_legal_moves(position: Coordinate, color: Color, board: &Chessboard, moves: &mut Vec<Move>) {
    println!("IMPLEMENT CASTLING IMPLEMENT CASTLING (crate::pieces::king::append_legal_moves)");

    for x in -1..=1 {
        for y in -1..=1 {
            if x == 0 && y == 0 { continue; }
            let x = (position.x as i8 + x) as u8;
            let y = (position.y as i8 + y) as u8;
            if board.get(x, y).is_ok_and(|piece| piece.is_none() || piece.unwrap().color != color) {
                moves.push(Move::new(position, Coordinate::new(x, y), None));
            }
        }
    }
}
