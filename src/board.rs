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
    to_move_white: bool,
}

impl Board {
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

            to_move_white: true,
        }
    }

    /// Lookup what piece is at a particular square in the board.
    /// `rank` and `file` are 0-indexed.
    pub fn at(&self, rank: u8, file: u8) -> Option<Piece> {
        assert!(rank < 8);
        assert!(file < 8);

        let mask = 1u64 << rank * 8 + file;
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
        for rank in 0..8u8 {
            out.push_str(format!("{} | ", 8 - rank).as_str());

            for file in 0..8u8 {
                match self.at(rank, file) {
                    None => out.push_str("  "),
                    Some(piece) => out.push_str(format!("{} ", piece).as_str()),
                }
            }

            out.push_str("|\n")
        }
        out.push_str("   -----------------\n    a b c d e f g h");
        write!(f, "{}", out)
    }
}
