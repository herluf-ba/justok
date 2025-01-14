use std::fmt::Display;

pub mod board;

/// Every type of piece in chess.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Piece {
    PawnWhite = 1,
    KnightWhite = 2,
    BishopWhite = 3,
    RookWhite = 4,
    QueenWhite = 5,
    KingWhite = 6,
    PawnBlack = 8,
    KnightBlack = 9,
    BishopBlack = 10,
    RookBlack = 11,
    QueenBlack = 12,
    KingBlack = 13,
}

impl Piece {
    /// Converts a FEN notation char to a Piece.
    /// Example: 'p' -> PawnBlack
    /// Example: 'K' -> KingWhite
    pub fn from_char(c: &char) -> Option<Self> {
        match c {
            'p' => Some(Self::PawnBlack),
            'P' => Some(Self::PawnWhite),
            'r' => Some(Self::RookBlack),
            'R' => Some(Self::RookWhite),
            'n' => Some(Self::KnightBlack),
            'N' => Some(Self::KnightWhite),
            'b' => Some(Self::BishopBlack),
            'B' => Some(Self::BishopWhite),
            'k' => Some(Self::KingBlack),
            'K' => Some(Self::KingWhite),
            'q' => Some(Self::QueenBlack),
            'Q' => Some(Self::QueenWhite),
            _ => None,
        }
    }

    /// Converts this piece to a FEN notation char.
    /// Example: PawnBlack -> 'a'
    /// Example: KingWhite -> 'K'
    pub fn to_char(&self) -> char {
        match self {
            Self::PawnBlack => 'p',
            Self::PawnWhite => 'P',
            Self::RookBlack => 'r',
            Self::RookWhite => 'R',
            Self::KnightBlack => 'n',
            Self::KnightWhite => 'N',
            Self::BishopBlack => 'b',
            Self::BishopWhite => 'B',
            Self::KingBlack => 'k',
            Self::KingWhite => 'K',
            Self::QueenBlack => 'q',
            Self::QueenWhite => 'Q',
        }
    }

    /// Tells wether this piece is white.
    pub fn is_white(&self) -> bool {
        *self as u8 != 0 && *self as u8 >> 3 < 1
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Piece::PawnWhite => "♙",
                Piece::PawnBlack => "♟︎",
                Piece::KnightWhite => "♘",
                Piece::KnightBlack => "♞",
                Piece::BishopWhite => "♗",
                Piece::BishopBlack => "♝",
                Piece::RookWhite => "♖",
                Piece::RookBlack => "♜",
                Piece::QueenWhite => "♕",
                Piece::QueenBlack => "♛",
                Piece::KingWhite => "♔",
                Piece::KingBlack => "♚",
            }
        )
    }
}

/// Letters of the eight files on a chess board.
pub const FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

/// An integer representing a square on a chess board.
/// There are 64 unique positions on the board, so we actually only need 6 bits.
/// A u8 suffices for now.
pub type Square = u8;

/// Converts a i8 that may point to a square in a board to a valid Square if possible.
/// Hint: If you have an iterator over maybe_squares, use filter_map(to_board_square) on it.
pub fn to_board_square(maybe_square: i8) -> Option<Square> {
    (maybe_square >= 0 && maybe_square < 64)
        .then(|| u8::try_from(maybe_square).ok())
        .flatten()
}

pub fn square_to_algebraic(square: Square) -> String {
    let rank = (square / 8) + 1;
    let file = FILES[(square % 8) as usize];
    format!("{file}{rank}").to_owned()
}

pub fn square_from_algebraic(long_algebraic: &str) -> Square {
    let mut chars = long_algebraic.chars();
    let file = match chars.next() {
        Some('a') => 0,
        Some('b') => 1,
        Some('c') => 2,
        Some('d') => 3,
        Some('e') => 4,
        Some('f') => 5,
        Some('g') => 6,
        Some('h') => 7,
        _ => panic!("Invalid square {}", long_algebraic),
    };
    let rank = chars.next().and_then(|c| c.to_digit(10)).unwrap() - 1;
    u8::try_from(rank * 8 + file).unwrap()
}

/// A move between two squares.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promote_to: Option<Piece>,
}

impl Move {
    pub fn new(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
            promote_to: None,
        }
    }

    /// Read a move from long algebraic notation.
    /// Example "b1a3" -> Move { from: , to: }
    /// Will panic if the move is not valid (has rank or file outside normal chess board).
    pub fn from_str(long_algebraic: &str) -> Move {
        assert!(long_algebraic.len() >= 4);
        let from = square_from_algebraic(&long_algebraic[0..2]);
        let to = square_from_algebraic(&long_algebraic[2..4]);
        let promote_to = long_algebraic
            .chars()
            .nth(4)
            .and_then(|c| Piece::from_char(&c));

        Move {
            from,
            to,
            promote_to,
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            square_to_algebraic(self.from),
            square_to_algebraic(self.to),
            self.promote_to
                .map_or("".to_owned(), |p| p.to_char().to_string())
        )
    }
}
