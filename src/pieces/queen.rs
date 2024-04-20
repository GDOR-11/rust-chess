use crate::chessboard::{Coordinate, Chessboard, Move};

use super::{Color, add_line_to_moves, is_legal_move_in_linear_movement, PieceType};

pub fn append_legal_moves(position: Coordinate, color: Color, board: &Chessboard, moves: &mut Vec<Move>) {
    for x in -1..=1 {
        for y in -1..=1 {
            if x == 0 && y == 0 { continue; }
            add_line_to_moves(position, (x, y), color, board, moves);
        }
    }
}

pub fn is_legal_move(r#move: Move, color: Color, board: &Chessboard) -> bool {
    is_legal_move_in_linear_movement(r#move, color, board, PieceType::Queen)
}
