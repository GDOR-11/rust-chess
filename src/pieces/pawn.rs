use crate::chessboard::{Coordinate, Chessboard, Move};

use super::Color;

pub fn append_legal_moves(position: Coordinate, color: Color, board: &Chessboard, moves: &mut Vec<Move>) {
    let direction = match color { Color::White => 1, Color::Black => -1 };
    let next_y = (position.y as i8 + direction) as u8;

    if let Ok(Some(piece)) = board.get(position.x, next_y) { } else {
        moves.push(Move::new(position.x, position.y, position.x, next_y, None));
    }
    for dx in [-1, 1] {
        let next_x = (position.x as i8 + dx) as u8;
        if matches!(board.get(next_x, next_y), Ok(Some(piece)) if piece.color != color) {
            moves.push(Move::new(position.x, position.y, (position.x as i8 + dx) as u8, next_y, None));
        }
    }
}

pub fn is_legal_move(r#move: Move, color: Color, board: &Chessboard) -> bool {
}
