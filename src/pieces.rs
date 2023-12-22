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
}
impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}

#[derive(Debug, Copy, Clone)]
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
}

#[derive(Debug, Copy, Clone)]
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
    pub fn as_character(&self) -> char {
        let lowercase = self.piece_type.to_char();
        if self.color == Color::White { lowercase.to_ascii_uppercase() } else { lowercase }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.as_character())
    }
}
