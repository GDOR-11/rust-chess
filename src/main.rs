mod pieces;
mod chessboard;

use chessboard::{Chessboard, Move, Coordinate};

fn main() {
    let mut chessboard = Chessboard::from_fen("r1bqk2r/ppppbppp/2n5/5n2/8/5N2/PPPP1PPP/RNBQRBK1 b kq - 4 8").expect("FEN is invalid");
    println!("{chessboard}\n");
    chessboard.make_move(Move::new(Coordinate::new(3, 6), Coordinate::new(3, 4), None)).unwrap();
    println!("{chessboard}\n");
    for r#move in chessboard.get_legal_moves() {
        print!("move: {},\n", r#move);
    }
}
