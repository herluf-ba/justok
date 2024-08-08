use std::fmt::Display;

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

/// Every square in chess.
#[rustfmt::skip]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    pub fn rank(&self) -> u8 {
        use Square::*;
        match self {
            A1 | B1 | C1 | D1 | E1 | F1 | G1 | H1 => 0,
            A2 | B2 | C2 | D2 | E2 | F2 | G2 | H2 => 1,
            A3 | B3 | C3 | D3 | E3 | F3 | G3 | H3 => 2,
            A4 | B4 | C4 | D4 | E4 | F4 | G4 | H4 => 3,
            A5 | B5 | C5 | D5 | E5 | F5 | G5 | H5 => 4,
            A6 | B6 | C6 | D6 | E6 | F6 | G6 | H6 => 5,
            A7 | B7 | C7 | D7 | E7 | F7 | G7 | H7 => 6,
            A8 | B8 | C8 | D8 | E8 | F8 | G8 | H8 => 7,
        }
    }

    pub fn file(&self) -> u8 {
        use Square::*;
        match self {
            A1 | A2 | A3 | A4 | A5 | A6 | A7 | A8 => 0,
            B1 | B2 | B3 | B4 | B5 | B6 | B7 | B8 => 1,
            C1 | C2 | C3 | C4 | C5 | C6 | C7 | C8 => 2,
            D1 | D2 | D3 | D4 | D5 | D6 | D7 | D8 => 3,
            E1 | E2 | E3 | E4 | E5 | E6 | E7 | E8 => 4,
            F1 | F2 | F3 | F4 | F5 | F6 | F7 | F8 => 5,
            G1 | G2 | G3 | G4 | G5 | G6 | G7 | G8 => 6,
            H1 | H2 | H3 | H4 | H5 | H6 | H7 | H8 => 7,
        }
    }

    pub fn bit_mask(&self) -> u64 {
        1u64 << self.rank() * 8 + self.file()
    }

    pub fn iterator() -> impl Iterator<Item = Square> {
        use Square::*;
        [
            A1, A2, A3, A4, A5, A6, A7, A8, B1, B2, B3, B4, B5, B6, B7, B8, C1, C2, C3, C4, C5, C6,
            C7, C8, D1, D2, D3, D4, D5, D6, D7, D8, E1, E2, E3, E4, E5, E6, E7, E8, F1, F2, F3, F4,
            F5, F6, F7, F8, G1, G2, G3, G4, G5, G6, G7, G8, H1, H2, H3, H4, H5, H6, H7, H8,
        ]
        .iter()
        .copied()
    }
}

/// Every square, for convenient iteration!
pub const SQUARES: [Square; 64] = {
    use Square::*;
    [
        A1, B1, C1, D1, E1, F1, G1, H1, A2, B2, C2, D2, E2, F2, G2, H2, A3, B3, C3, D3, E3, F3, G3,
        H3, A4, B4, C4, D4, E4, F4, G4, H4, A5, B5, C5, D5, E5, F5, G5, H5, A6, B6, C6, D6, E6, F6,
        G6, H6, A7, B7, C7, D7, E7, F7, G7, H7, A8, B8, C8, D8, E8, F8, G8, H8,
    ]
};

// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
// pub struct Move {
//     from: Square,
//     to: Square,
//     piece: Piece,
// }

/// A bit board representation of a game of chess.
/// A chess board has 64 squares, so it can be represented as a 64-bit unsigned integer.
/// a1 is the 1st bit a2 the 2nd bit. The a2 is the 9th bit. h8 is the very last 64th bit.
/// The position of each piece type is saved as 1-bits within these u64's.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Board {
    pawns_white: u64,
    bishops_white: u64,
    knights_white: u64,
    rooks_white: u64,
    king_white: u64,
    queen_white: u64,

    pawns_black: u64,
    bishops_black: u64,
    knights_black: u64,
    rooks_black: u64,
    king_black: u64,
    queen_black: u64,

    // TODO: Add castling rights and en pessant representations
    white_to_move: bool,
}

impl Board {
    /// Create a new chess board with the pieces arranged in the standard fashion.
    pub fn new() -> Self {
        Self {
            pawns_black: 0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000,
            bishops_black:
                0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100,
            knights_black:
                0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010,
            rooks_black: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001,
            king_black: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000,
            queen_black: 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000,

            pawns_white: 0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000,
            bishops_white:
                0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            knights_white:
                0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            rooks_white: 0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            king_white: 0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,
            queen_white: 0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000,

            white_to_move: true,
        }
    }

    /// Lookup what piece is at a particular square in the board.
    pub fn at(&self, square: Square) -> Option<Piece> {
        let mask = square.bit_mask();
        if self.pawns_white & mask >= 1 {
            Some(Piece::PawnWhite)
        } else if self.pawns_black & mask >= 1 {
            Some(Piece::PawnBlack)
        } else if self.knights_white & mask >= 1 {
            Some(Piece::KnightWhite)
        } else if self.knights_black & mask >= 1 {
            Some(Piece::KnightBlack)
        } else if self.bishops_white & mask >= 1 {
            Some(Piece::BishopWhite)
        } else if self.bishops_black & mask >= 1 {
            Some(Piece::BishopBlack)
        } else if self.rooks_white & mask >= 1 {
            Some(Piece::RookWhite)
        } else if self.rooks_black & mask >= 1 {
            Some(Piece::RookBlack)
        } else if self.queen_white & mask >= 1 {
            Some(Piece::QueenWhite)
        } else if self.queen_black & mask >= 1 {
            Some(Piece::QueenBlack)
        } else if self.king_white & mask >= 1 {
            Some(Piece::KingWhite)
        } else if self.king_black & mask >= 1 {
            Some(Piece::KingBlack)
        } else {
            None
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::with_capacity(1028); // make sure the string has capacity for the board string.
        out.push_str("   -----------------\n");

        for square in SQUARES {
            if square.file() == 0 {
                out.push_str(format!("{} | ", 8 - square.rank()).as_str());
            }

            match self.at(square) {
                None => out.push_str("  "),
                Some(piece) => out.push_str(format!("{} ", piece).as_str()),
            }

            if square.file() == 7 {
                out.push_str("|\n");
            }
        }

        out.push_str("   -----------------\n    a b c d e f g h");
        write!(f, "{}", out)
    }
}
