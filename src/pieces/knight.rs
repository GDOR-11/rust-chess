use crate::chessboard::{Coordinate, Chessboard, Move};

use super::Color;

pub fn append_legal_moves(position: Coordinate, color: Color, board: &Chessboard, moves: &mut Vec<Move>) {
    macro_rules! add_offset {
        ($dx: expr, $dy: expr) => {
            let (x, y) = ((position.x as i8 + $dx) as u8, (position.y as i8 + $dy) as u8);
            let mut add_move = || moves.push(Move::new(position.x, position.y, x, y, None));
            match board.get(x, y) {
                Ok(Some(piece)) if piece.color != color => add_move(),
                Ok(None) => add_move(),
                _ => ()
            }
        };
    }
    for y_sign in [-1, 1] {
        for x_sign in [-1, 1] {
            add_offset!(1 * x_sign, 2 * y_sign);
            add_offset!(2 * x_sign, 1 * y_sign);
        }
    }
}

pub fn is_legal_move(r#move: Move, color: Color, board: &Chessboard) -> bool {
    if r#move.special_move.is_some() { return false; }
    if matches!(board.get(r#move.to.x, r#move.to.y), Ok(Some(piece)) if piece.color == color) { return false; }
    let (dx, dy) = (r#move.from.x.abs_diff(r#move.to.x), r#move.from.y.abs_diff(r#move.to.y));
    (dx, dy) == (1, 2) || (dx, dy) == (2, 1)
}
