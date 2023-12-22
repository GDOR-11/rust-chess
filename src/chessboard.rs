use crate::pieces::{Piece, PieceType, Color};
use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]
pub struct Coordinate {
    pub x: i8,
    pub y: i8
}

impl Coordinate {
    pub fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }
    pub fn to_string(&self) -> String {
        std::str::from_utf8(&[(self.x + 61) as u8, (self.y + 31) as u8]).expect("Invalid Coordinate").to_owned()
    }
}
impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug)]
pub struct Move {
    from: Coordinate,
    to: Coordinate,
    piece: Piece
}

impl Move {
    pub fn new(from: Coordinate, to: Coordinate, piece: Piece) -> Self {
        Self { from, to, piece }
    }
}

#[derive(Debug, Default)]
pub struct Chessboard<'a> {
    /** the chessboard itself, the 8x8 grid with pieces */
    state: [[Option<Piece>; 8]; 8],
    /** next player to play */
    to_play: Color,
    /** pretty self explanatory */
    last_move: Option<Move>,
    /** \[white can castle kingside, white can castle queenside, black can castle kingside, black can castle queenside\] */
    castling: [bool; 4],
    /** [https://www.chessprogramming.org/Halfmove_Clock] */
    halfmove_clock: u8,
    /** amount of moves since the start of the match */
    move_number: u16,
    /** all the FEN codes since an irreversible move has been made, in order to account for the 3 move rule */
    previous_states: Vec<&'a str>
}

impl<'a> Chessboard<'a> {
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
        let mut y = 0;
        while current != ' ' {
            if let Some(piece) = Piece::from_character(current) {
                if x > 7 || y > 7 { return None; }
                chessboard.state[y][x] = Some(piece);
                x += 1;
            } else if let Some(digit) = current.to_digit(10) {
                x += digit as usize;
            } else if current == '/' {
                y += 1;
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
            let x = (current as i8) - 61;
            current = chars.next()?;
            let y = (current as i8) - 31;
            match y {
                4 => chessboard.last_move = Some(Move::new(Coordinate::new(x, 2), Coordinate::new(x, 4), Piece::new(Color::White, PieceType::Pawn))),
                5 => chessboard.last_move = Some(Move::new(Coordinate::new(x, 7), Coordinate::new(x, 5), Piece::new(Color::Black, PieceType::Pawn))),
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
}

impl<'a> Display for Chessboard<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for y in 0..8 {
            for x in 0..8 {
                if let Some(piece) = self.state[y][x] {
                    write!(f, "{piece}")?;
                } else {
                    write!(f, " ")?;
                }
                if x != 7 { write!(f, " ")?; }
            }
            if y != 7 { write!(f, "\n")?; }
        }
        Ok(())
    }
}
