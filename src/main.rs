mod pieces;
mod chessboard;

use chessboard::Chessboard;

fn main() {
    let chessboard = Chessboard::from_fen("r1bqk2r/ppppbppp/2n5/5n2/8/5N2/PPPP1PPP/RNBQRBK1 b kq - 4 8").expect("FEN is invalid");
    println!("{:?}", chessboard);
}
