mod pieces;
mod chessboard;

use chessboard::{Chessboard, Move};

fn main() {
    let mut chessboard = Chessboard::from_fen("8/8/8/8/2n5/n7/3N4/1N6 b - - 0 1").expect("FEN is invalid");
    println!("{chessboard}");

    let moves = chessboard.get_legal_moves();

    for r#move in &moves { println!("{move}"); }

    // check all 4096 moves
    for start_y in 0..8 {
        for start_x in 0..8 {
            for end_y in 0..8 {
                for end_x in 0..8 {
                    let r#move = Move::new(start_x, start_y, end_x, end_y, None);
                    assert_eq!(chessboard.is_legal_move(r#move), Ok(moves.contains(&r#move)));
                }
            }
        }
    }
}
