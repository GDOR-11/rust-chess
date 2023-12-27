use crate::chessboard::{Coordinate, Chessboard, Move};

use super::{Color, add_line_to_moves};

pub fn append_legal_moves(position: Coordinate, color: Color, board: &Chessboard, moves: &mut Vec<Move>) {
    add_line_to_moves(position, (1, 1), color, board, moves);
    add_line_to_moves(position, (1, -1), color, board, moves);
    add_line_to_moves(position, (-1, 1), color, board, moves);
    add_line_to_moves(position, (-1, -1), color, board, moves);
}
