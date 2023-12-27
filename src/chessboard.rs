use crate::pieces::{Piece, Color};
use std::fmt::{Display, Formatter, Error};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Coordinate {
    pub x: u8,
    pub y: u8
}

impl Coordinate {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
    pub fn from_string(string: &str) -> Option<Self> {
        let mut chars = string.chars();
        Some(Self {
            x: chars.next()? as u8 - 97,
            y: chars.next()? as u8 - 49
        })
    }
    pub fn to_string(&self) -> String {
        std::str::from_utf8(&[(self.x + 97) as u8, (self.y + 49) as u8]).expect("Invalid Coordinate").to_owned()
    }
}
impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.to_string())
    }
}

#[test]
fn coordinate_to_string() {
    assert_eq!(Coordinate::new(3, 5).to_string(), "d6");
    assert_eq!(Coordinate::new(0, 0).to_string(), "a1");
    assert_eq!(Coordinate::new(7, 7).to_string(), "h8");
}
#[test]
fn coordinate_from_string() {
    assert_eq!(Coordinate::from_string("d6"), Some(Coordinate::new(3, 5)));
    assert_eq!(Coordinate::from_string("a1"), Some(Coordinate::new(0, 0)));
    assert_eq!(Coordinate::from_string("h8"), Some(Coordinate::new(7, 7)));
}

/** moves that are too different from the others, they need special treatment */
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SpecialMove {
    EnPassant,
    Castling
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Move {
    pub from: Coordinate,
    pub to: Coordinate,
    pub special_move: Option<SpecialMove>
}

impl Move {
    pub fn new(from: Coordinate, to: Coordinate, special_move: Option<SpecialMove>) -> Self {
        Self { from, to, special_move }
    }
}
impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}{}", self.from, self.to)
    }
}

#[derive(Debug)]
pub struct Chessboard {
    /** \[P, R, N, B, Q, K, p, r, n, b, q, k\] */
    piece_bitboards: [u64; 12],
    /** array of rows - each tile represents 4 bits of the u32. */
    state: [u32; 8],
    /** next player to play */
    pub to_play: Color,
    /** pretty self explanatory */
    pub last_move: Option<Move>,
    /** \[white can castle kingside, white can castle queenside, black can castle kingside, black can castle queenside\] */
    pub castling: [bool; 4],
    /** [https://www.chessprogramming.org/Halfmove_Clock] */
    halfmove_clock: u8,
    /** amount of moves since the start of the match */
    pub move_number: u16,
    /** all the states since an irreversible move has been made, in order to account for the 3 move rule */
    previous_states: Vec<[u32; 8]>
}
impl Default for Chessboard {
    fn default() -> Self {
        Self {
            piece_bitboards: [0; 12],
            state: [0xCCCCCCCC; 8],
            to_play: Color::default(),
            last_move: None,
            castling: [false, false, false, false],
            halfmove_clock: 0,
            move_number: 0,
            previous_states: vec![]
        }
    }
}

#[derive(Debug)]
pub struct OutsideOfChessboard;

impl Chessboard {
    // read https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation#Definition if you want
    // to know better how this works.
    /**
        creates a Chessboard from a FEN code. If the FEN code is invalid this function will probably
        return None (sometimes it can generate a chess board out of invalid FEN)
    */
    pub fn from_fen(fen: &str) -> Option<Self> {
        let mut chessboard = Chessboard::default();

        let mut chars = fen.chars();
        let mut current = chars.next()?;

        // get the state of the board
        let mut x = 0;
        let mut y = 7;
        while current != ' ' {
            if let Some(piece) = Piece::from_character(current) {
                if chessboard.set(x, y, Some(piece)).is_err() { return None; }
                x += 1;
            } else if let Some(digit) = current.to_digit(10) {
                x += digit as u8;
            } else if current == '/' {
                y -= 1;
                x = 0;
            } else {
                return None;
            }

            current = chars.next()?;
        }

        // get the next player to play
        if chars.next()? == 'b' { chessboard.to_play = Color::Black; }
        chars.next();
        current = chars.next()?;
        
        // castling rights
        while current != ' ' {
            match current {
                'K' => chessboard.castling[0] = true,
                'Q' => chessboard.castling[1] = true,
                'k' => chessboard.castling[2] = true,
                'q' => chessboard.castling[3] = true,
                _ => ()
            }
            current = chars.next()?;
        }
        current = chars.next()?;

        // if en passant is avaible, put it in last move
        if current != '-' {
            let mut coordinate_string = current.to_string();
            coordinate_string.push(chars.next()?);
            let coordinate = Coordinate::from_string(&coordinate_string)?;
            let (x, y) = (coordinate.x, coordinate.y);
            match y {
                4 => chessboard.last_move = Some(Move::new(Coordinate::new(x, 2), coordinate, Some(SpecialMove::EnPassant))),
                5 => chessboard.last_move = Some(Move::new(Coordinate::new(x, 7), coordinate, Some(SpecialMove::EnPassant))),
                _ => return None
            }
        }

        chars.next();
        current = chars.next()?;

        // half move clock
        while current != ' ' {
            if !current.is_digit(10) { return None; }
            let digit = current.to_digit(10).unwrap();
            chessboard.halfmove_clock *= 10;
            chessboard.halfmove_clock += digit as u8;
            current = chars.next()?;
        }

        // move number
        while let Some(current) = chars.next() {
            if !current.is_digit(10) { return None; }
            let digit = current.to_digit(10).unwrap();
            chessboard.move_number *= 10;
            chessboard.move_number += digit as u16;
        }

        Some(chessboard)
    }

    pub fn get(&self, x: u8, y: u8) -> Result<Option<Piece>, OutsideOfChessboard> {
        if x > 7 || y > 7 { Err(OutsideOfChessboard) } else { Ok(Piece::from_code(self.get_code(x, y))) }
    }
    /**
        very fast, but will panic if x or y are out of bounds. If you don't want that behaviour,
        use Chessboard::get
    */
    pub fn get_code(&self, x: u8, y: u8) -> u8 {
        ((self.state[y as usize] >> (4 * x)) % 16) as u8
    }
    pub fn set(&mut self, x: u8, y: u8, piece: Option<Piece>) -> Result<(), OutsideOfChessboard> {
        if x > 7 || y > 7 { Err(OutsideOfChessboard) } else { Ok( self.set_code(x, y, piece.map(|p| p.to_code()).unwrap_or(12))) }
    }
    /**
        very fast, but will panic if x or y are out of bounds. If you don't want that behaviour,
        use Chessboard::get
    */
    pub fn set_code(&mut self, x: u8, y: u8, code: u8) {
        let previous_piece_code = self.get_code(x, y);
        if let Some(bitboard) = self.piece_bitboards.get_mut(previous_piece_code as usize) {
            *bitboard ^= 1 << (x + 8 * y);
        }
        if let Some(bitboard) = self.piece_bitboards.get_mut(code as usize) {
            *bitboard |= 1 << (x + 8 * y);
        }
        self.state[y as usize] &= !(0b1111 << (4 * x));
        self.state[y as usize] |= (code as u32) << (4 * x);
    }

    /**
        make the move without checking if it's valid. To check for validity, use Chessboard::get_legal_moves. Returns the captured piece if there was any.
    */
    pub fn make_move(&mut self, r#move: Move) -> Result<Option<Piece>, OutsideOfChessboard> {
        println!("(castling too) IMPLEMENT EN PASSANT IMPLEMENT EN PASSANT IMPLEMENT EN PASSANT IMPLEMENT EN PASSANT (Chessboard::make_move)");

        let piece = self.get(r#move.from.x, r#move.from.y)?;
        let captured = self.get(r#move.to.x, r#move.to.y)?;

        self.set(r#move.from.x, r#move.from.y, None).unwrap();
        self.set(r#move.to.x, r#move.to.y, piece).unwrap();
        self.to_play = self.to_play.opposite();

        return Ok(captured);
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        let mut moves = vec![];

        for y in 0..8 {
            for x in 0..8 {
                if let Some(piece) = self.get(x, y).expect("DONT MESS WITH THE CODE").filter(|piece| piece.color == self.to_play ) {
                    piece.append_legal_moves(Coordinate::new(x, y), self, &mut moves);
                }
            }
        }

        moves
    }
}

impl Display for Chessboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for y in (0..8).rev() {
            for x in 0..8 {
                if let Some(piece) = self.get(x, y).expect("DONT MESS WITH THE CODE") {
                    write!(f, "{piece}")?;
                } else {
                    write!(f, " ")?;
                }
                if x != 7 { write!(f, " ")?; }
            }
            if y != 0 { write!(f, "\n")?; }
        }
        Ok(())
    }
}
