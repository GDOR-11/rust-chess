use crate::chessboard::{Coordinate, Chessboard, Move};

use super::{Color, add_line_to_moves, is_legal_move_in_linear_movement, PieceType};

pub fn append_legal_moves(position: Coordinate, color: Color, board: &Chessboard, moves: &mut Vec<Move>) {
    add_line_to_moves(position, (1, 1), color, board, moves);
    add_line_to_moves(position, (1, -1), color, board, moves);
    add_line_to_moves(position, (-1, 1), color, board, moves);
    add_line_to_moves(position, (-1, -1), color, board, moves);
}

pub fn is_legal_move(r#move: Move, color: Color, board: &Chessboard) -> bool {
    is_legal_move_in_linear_movement(r#move, color, board, PieceType::Bishop)
}
