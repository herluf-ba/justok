use std::fmt::Display;

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

    /// Check if another piece is of the same kind.
    /// For instance, are both rooks?
    pub fn is_same_kind(&self, other: &Piece) -> bool {
        use Piece::*;
        match (self, other) {
            (PawnBlack, PawnWhite) => true,
            (PawnWhite, PawnBlack) => true,
            (KnightBlack, KnightWhite) => true,
            (KnightWhite, KnightBlack) => true,
            (BishopBlack, BishopWhite) => true,
            (BishopWhite, BishopBlack) => true,
            (RookBlack, RookWhite) => true,
            (RookWhite, RookBlack) => true,
            (QueenBlack, QueenWhite) => true,
            (QueenWhite, QueenBlack) => true,
            (KingBlack, KingWhite) => true,
            (KingWhite, KingBlack) => true,
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
