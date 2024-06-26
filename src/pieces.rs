use std::cmp::Ordering;

use crate::chessboard::{Coordinate, Chessboard, Move};

mod pawn;
mod rook;
mod knight;
mod bishop;
mod queen;
mod king;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Color {
    White,
    Black
}
impl Color {
    pub fn from_char(character: char) -> Self {
        if character.is_uppercase() { Color::White } else { Color::Black }
    }
    pub fn from_code(code: u8) -> Self {
        if code < 6 { Color::White } else { Color::Black }
    }
    pub fn opposite(self) -> Self {
        if self == Color::White { Color::Black } else { Color::White }
    }
}
impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}
impl PieceType {
    pub fn from_char(character: char) -> Option<Self> {
        match character.to_ascii_lowercase() {
            'p' => Some(PieceType::Pawn),
            'r' => Some(PieceType::Rook),
            'n' => Some(PieceType::Knight),
            'b' => Some(PieceType::Bishop),
            'q' => Some(PieceType::Queen),
            'k' => Some(PieceType::King),
            _ => None
        }
    }
    pub fn from_code(code: u8) -> Self {
        match code % 6 {
            0 => PieceType::Pawn,
            1 => PieceType::Rook,
            2 => PieceType::Knight,
            3 => PieceType::Bishop,
            4 => PieceType::Queen,
            5 => PieceType::King,
            _ => panic!("cosmic rays go brrr")
        }
    }
    pub fn to_char(&self) -> char {
        match self {
            PieceType::Pawn => 'p',
            PieceType::Rook => 'r',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Queen => 'q',
            PieceType::King => 'k'
        }
    }
    pub fn to_code(&self) -> u8 {
        match self {
            PieceType::Pawn => 0,
            PieceType::Rook => 1,
            PieceType::Knight => 2,
            PieceType::Bishop => 3,
            PieceType::Queen => 4,
            PieceType::King => 5
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Self { color, piece_type}
    }
    pub fn from_character(character: char) -> Option<Self> {
        let color = Color::from_char(character);
        let piece_type = PieceType::from_char(character)?;
        Some(Self { color, piece_type})
    }
    pub fn to_character(&self) -> char {
        let lowercase = self.piece_type.to_char();
        if self.color == Color::White { lowercase.to_ascii_uppercase() } else { lowercase }
    }
    pub fn from_code(code: u8) -> Option<Self> {
        if code > 11 {
            None
        } else {
            Some(Self { piece_type: PieceType::from_code(code), color: Color::from_code(code) })
        }
    }
    pub fn to_code(&self) -> u8 {
        self.piece_type.to_code() + if self.color == Color::White { 0 } else { 6 }
    }
    pub fn append_legal_moves(&self, position: Coordinate, board: &Chessboard, moves: &mut Vec<Move>) {
        match self.piece_type {
            PieceType::Rook => rook::append_legal_moves(position, self.color, board, moves),
            PieceType::Bishop => bishop::append_legal_moves(position, self.color, board, moves),
            PieceType::Queen => queen::append_legal_moves(position, self.color, board, moves),
            PieceType::King => king::append_legal_moves(position, self.color, board, moves),
            PieceType::Knight => knight::append_legal_moves(position, self.color, board, moves),
            _ => ()
        }
    }
    pub fn get_legal_moves(&self, position: Coordinate, board: &Chessboard) -> Vec<Move> {
        let mut moves = vec![];
        self.append_legal_moves(position, board, &mut moves);
        moves
    }
    pub fn is_legal_move(&self, r#move: Move, board: &Chessboard) -> bool {
        match self.piece_type {
            PieceType::Rook => rook::is_legal_move(r#move, self.color, board),
            PieceType::Bishop => bishop::is_legal_move(r#move, self.color, board),
            PieceType::Queen => queen::is_legal_move(r#move, self.color, board),
            PieceType::King => king::is_legal_move(r#move, self.color, board),
            PieceType::Knight => knight::is_legal_move(r#move, self.color, board),
            _ => false
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.to_character())
    }
}




// functionality for the pieces
fn add_line_to_moves(position: Coordinate, step: (i8, i8), color: Color, board: &Chessboard, moves: &mut Vec<Move>) {
    debug_assert_ne!(step, (0, 0));
    macro_rules! add_move {
        ($x:expr,$y:expr) => {
            moves.push(Move::new(position.x, position.y, $x, $y, None));
        };
    }
    let (mut x, mut y) = (position.x as i8, position.y as i8);
    x += step.0;
    y += step.1;
    while let Ok(piece) = board.get(x as u8, y as u8) {
        if let Some(piece) = piece {
            if piece.color != color { add_move!(x as u8, y as u8); }
            break;
        }
        add_move!(x as u8, y as u8);
        x += step.0;
        y += step.1;
    }
}


/** checks if the move is legal, but only works for rooks, bishops and queens */
fn is_legal_move_in_linear_movement(r#move: Move, color: Color, board: &Chessboard, piece_type: PieceType) -> bool {

    if r#move.from == r#move.to { return false; }
    if r#move.special_move.is_some() { return false; }
    
    let (delta_x, delta_y) = (r#move.from.x.abs_diff(r#move.to.x), r#move.from.y.abs_diff(r#move.to.y));
    match piece_type {
        PieceType::Rook => if delta_x != 0 && delta_y != 0 { return false; }
        PieceType::Bishop => if delta_x != delta_y { return false; }
        PieceType::Queen => if delta_x != 0 && delta_y != 0 && delta_x != delta_y { return false; }
        _ => panic!("piece::is_legal_move_in_linear_movement only works for bishops, queens and rooks!")
    }

    if matches!(board.get(r#move.to.x, r#move.to.y), Ok(Some(piece)) if piece.color == color) {
        return false;
    }

    let x_step = match r#move.from.x.cmp(&r#move.to.x) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1
    };
    let y_step = match r#move.from.y.cmp(&r#move.to.y) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1
    };

    let (mut x, mut y) = (r#move.from.x, r#move.from.y);
    x = (x as i8 + x_step) as u8;
    y = (y as i8 + y_step) as u8;
    while (x, y) != (r#move.to.x, r#move.to.y) {
        if let Ok(Some(_piece)) = board.get(x, y) { return false; }
        x = (x as i8 + x_step) as u8;
        y = (y as i8 + y_step) as u8;
    }
    true
}
