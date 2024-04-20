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
                moves.push(Move::new(position.x, position.y, x, y, None));
            }
        }
    }
}
pub fn is_legal_move(r#move: Move, color: Color, board: &Chessboard) -> bool {
    if r#move.from.x.abs_diff(r#move.to.x) > 1 { return false; }
    if r#move.from.y.abs_diff(r#move.to.y) > 1 { return false; }
    if r#move.special_move.is_some() { return false; }
    match board.get(r#move.to.x, r#move.to.y) {
        Ok(Some(piece)) => piece.color != color,
        Ok(None) => true,
        Err(_) => false
    }
}
