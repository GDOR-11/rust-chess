use crate::chessboard::{Coordinate, Chessboard, Move};

use super::{Piece, Color, PieceType, add_line_to_moves};

pub fn append_legal_moves(position: Coordinate, color: Color, board: &Chessboard, moves: &mut Vec<Move>) {
    add_line_to_moves(position, (-1, 0), color, board, moves);
    add_line_to_moves(position, (1, 0), color, board, moves);
    add_line_to_moves(position, (0, 1), color, board, moves);
    add_line_to_moves(position, (0, -1), color, board, moves);
}

#[test]
pub fn rook_legal_moves() {

    let position = Coordinate::new(4, 4);
    macro_rules! move_to {
        ($x: expr, $y: expr) => {
            Move::new(position, Coordinate::new($x, $y), None)
        };
    }
    let chessboard = Chessboard::from_fen("r1bqk2r/ppppbppp/2n5/4Rn2/8/5N2/PPPPBPPP/RNBQ2K1 w kq - 6 8").expect("FEN is invalid");
    assert_eq!(Piece::new(Color::White, PieceType::Rook).get_legal_moves(Coordinate::new(4, 4), &chessboard), vec![
        move_to!(3, 4),
        move_to!(2, 4),
        move_to!(1, 4),
        move_to!(0, 4),
        move_to!(5, 4),
        move_to!(4, 5),
        move_to!(4, 6),
        move_to!(4, 3),
        move_to!(4, 2),
    ]);
}
