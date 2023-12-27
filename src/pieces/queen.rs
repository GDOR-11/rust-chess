use crate::chessboard::{Coordinate, Chessboard, Move};

use super::{Color, add_line_to_moves};

pub fn append_legal_moves(position: Coordinate, color: Color, board: &Chessboard, moves: &mut Vec<Move>) {
    for x in -1..=1 {
        for y in -1..=1 {
            if x == 0 && y == 0 { continue; }
            add_line_to_moves(position, (x, y), color, board, moves);
        }
    }
}
