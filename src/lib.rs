use std::fmt::Display;

pub mod bit_board;
pub mod board;

/// Every type of piece in chess.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Piece {
    PawnWhite,
    KnightWhite,
    BishopWhite,
    RookWhite,
    QueenWhite,
    KingWhite,
    PawnBlack,
    KnightBlack,
    BishopBlack,
    RookBlack,
    QueenBlack,
    KingBlack,
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

    /// Tells wether this piece is white.
    pub fn is_white(&self) -> bool {
        match self {
            Piece::PawnWhite
            | Piece::KnightWhite
            | Piece::BishopWhite
            | Piece::RookWhite
            | Piece::QueenWhite
            | Piece::KingWhite => true,
            _ => false,
        }
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

/// A move between two squares.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Move {
    pub from: Square,
    pub to: Square,
}
