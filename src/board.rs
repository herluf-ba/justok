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

impl Piece {
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

    pub fn from_rank_and_file(rank: usize, file: usize) -> Option<Self> {
        SQUARES.get(rank * 8 + file).copied()
    }

    /// Gets a u64 with a 1 bit at the position corresponding to this square.
    pub fn bit_mask(&self) -> u64 {
        1u64 << self.rank() * 8 + self.file()
    }
}

/// Every square, for convenient iteration!
/// Note that the order matters here. Piece::from_rank_and_file relies on it.
pub const SQUARES: [Square; 64] = {
    use Square::*;
    [
        A1, B1, C1, D1, E1, F1, G1, H1, A2, B2, C2, D2, E2, F2, G2, H2, A3, B3, C3, D3, E3, F3, G3,
        H3, A4, B4, C4, D4, E4, F4, G4, H4, A5, B5, C5, D5, E5, F5, G5, H5, A6, B6, C6, D6, E6, F6,
        G6, H6, A7, B7, C7, D7, E7, F7, G7, H7, A8, B8, C8, D8, E8, F8, G8, H8,
    ]
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Move {
    pub from: Square,
    pub to: Square,
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
    white_to_move: bool,
}

impl Board {
    /// Create a new chess board with no pieces placed.
    pub fn blank() -> Self {
        Self {
            pawns_white: 0,
            bishops_white: 0,
            knights_white: 0,
            rooks_white: 0,
            king_white: 0,
            queen_white: 0,
            pawns_black: 0,
            bishops_black: 0,
            knights_black: 0,
            rooks_black: 0,
            king_black: 0,
            queen_black: 0,
            white_to_move: true,
        }
    }

    /// Create a bit-board from a Forsyth-Edwards-Notation (FEN) string.
    pub fn from_fen_string(fen: &str) -> Self {
        // TODO: Make this return Result and don't panic

        // FEN contains 6 fields separated by space.
        // They are:
        // 1. Piece placement.
        // 2. Side to move (w/b)
        // 3. Castling ability
        // 4. En pessant target square
        // 5. Halfmove clock
        // 6. Fullmove counter
        // Fields 5. and 6. may be left out.
        let fields: Vec<_> = fen.split_whitespace().collect();
        if fields.len() > 6 || fields.len() < 4 {
            panic!("Not a valid FEN string: '{fen}'");
        }

        // Read piece placement and place onto blank board.
        // Placement is presented from rank 8 to 1, each rank separated by '/'.
        // Each rank lists the pieces (pnbrqk) going from file 1 to 8. White is uppercase.
        // N consequtive blank squares are listed as the number N.
        // For example here is the standard setup:
        // rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR
        let mut board = Self::blank();
        let placement = fields[0];
        for (rank_idx, rank_str) in placement.split('/').enumerate() {
            let rank = 7 - rank_idx;
            let mut file = 0;
            for piece in rank_str.chars() {
                match piece {
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                        // Skip this amount of squares
                        file += piece.to_string().parse::<usize>().unwrap();
                    }
                    p => {
                        if let Some(valid) = Piece::from_char(&p) {
                            board.place(valid, Square::from_rank_and_file(rank, file).unwrap());
                            file += 1
                        } else {
                            panic!("Not a valid FEN string: '{fen}'")
                        }
                    }
                }
            }
        }

        board
    }

    fn clear(&mut self, square: Square) {
        let mask = !square.bit_mask();
        self.pawns_black &= mask;
        self.bishops_black &= mask;
        self.knights_black &= mask;
        self.rooks_black &= mask;
        self.king_black &= mask;
        self.queen_black &= mask;
        self.pawns_white &= mask;
        self.bishops_white &= mask;
        self.knights_white &= mask;
        self.rooks_white &= mask;
        self.king_white &= mask;
        self.queen_white &= mask;
    }

    fn place(&mut self, piece: Piece, at: Square) {
        self.clear(at); // Clear any pieces off the target square.

        let mask = at.bit_mask();
        match piece {
            Piece::PawnWhite => self.pawns_white |= mask,
            Piece::KnightWhite => self.knights_white |= mask,
            Piece::BishopWhite => self.bishops_white |= mask,
            Piece::RookWhite => self.rooks_white |= mask,
            Piece::QueenWhite => self.queen_white |= mask,
            Piece::KingWhite => self.king_white |= mask,
            Piece::PawnBlack => self.pawns_black |= mask,
            Piece::KnightBlack => self.knights_black |= mask,
            Piece::BishopBlack => self.bishops_black |= mask,
            Piece::RookBlack => self.rooks_black |= mask,
            Piece::QueenBlack => self.queen_black |= mask,
            Piece::KingBlack => self.king_black |= mask,
        }
    }

    /// Applies a move to the board with _no_ validation.
    pub fn apply(&mut self, r#move: Move) {
        // Look up what piece is moving.
        if let Some(from_piece) = self.at(r#move.from) {
            // Clear square that piece is moving from and to.
            self.clear(r#move.from);
            self.place(from_piece, r#move.to);
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

        for r in (0..8).rev() {
            out.push_str(format!("{} | ", r + 1).as_str());
            for f in 0..8 {
                let square = Square::from_rank_and_file(r, f).unwrap();
                match self.at(square) {
                    None => out.push_str("  "),
                    Some(piece) => out.push_str(format!("{} ", piece).as_str()),
                }
            }
            out.push_str("|\n");
        }
        out.push_str("   -----------------\n    a b c d e f g h");
        write!(f, "{}", out)
    }
}
