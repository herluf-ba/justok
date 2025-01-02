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
            let rank: u8 = 7 - (rank_idx as u8);
            let mut file: u8 = 0;
            for piece in rank_str.chars() {
                match piece {
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                        // Skip this amount of squares
                        file += piece.to_string().parse::<u8>().unwrap();
                    }
                    p => {
                        if let Some(valid) = Piece::from_char(&p) {
                            board.place(valid, rank * 8 + file);
                            file += 1
                        } else {
                            panic!("Not a valid FEN string: '{fen}'")
                        }
                    }
                }
            }
        }

        // Read whose turn it is.
        match fields[1] {
            "w" => board.white_to_move = true,
            "b" => board.white_to_move = false,
            _ => panic!("Not a valid FEN string: '{fen}'"),
        }

        // TODO: Read the rest of the FEN string.

        board
    }

    fn clear(&mut self, square: Square) {
        let mask = !(1u64 << square);
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

        let mask = 1u64 << at;
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
        let mask = 1u64 << square;
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
                let square = r * 8 + f;
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

/// Generates the possible moves given a particular board state.
pub fn generate_moves(board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();

    for r in (0..8).rev() {
        for f in 0..8 {
            let square = r * 8 + f;
            match board.at(square) {
                // Square had a piece with the color whose turn it is
                Some(piece) if board.white_to_move == piece.is_white() => {
                    let valid_target_squares = generate_piece_moves(board, piece, square);
                    let mut piece_moves: Vec<Move> = valid_target_squares
                        .iter()
                        .map(|target| Move {
                            from: square,
                            to: *target,
                        })
                        .collect();
                    moves.append(&mut piece_moves);
                }

                // Square was empty, or had the wrong color piece.
                _ => continue,
            }
        }
    }

    moves
}

/// Converts a i8 that may point to a square in a board to a valid Square if possible.
/// Hint: If you have an iterator over maybe_squares, use filter_map(to_board_square) on it.
fn to_board_square(maybe_square: i8) -> Option<Square> {
    (maybe_square > 0 && maybe_square < 64)
        .then(|| u8::try_from(maybe_square).ok())
        .flatten()
}

/// Generate the valid moves for a particular piece on a certain square within a board.
fn generate_piece_moves(board: &Board, piece: Piece, at: Square) -> Vec<Square> {
    match piece {
        Piece::PawnWhite | Piece::PawnBlack => {
            // TODO: en pessant!
            let rank = at as i8 / 8;
            let file = at as i8 % 8;
            let direction: i8 = match piece.is_white() {
                true => 1,
                false => -1,
            };

            // A pawn may move one square towards the opposing player.
            let mut candidates = vec![(rank + direction) * 8 + file];
            // If it is in it's starting rank, it may leap two squares.
            match (piece.is_white(), rank) {
                (true, 1) | (false, 6) => candidates.push((rank + 2 * direction) * 8 + file),
                _ => {}
            };
            // Filter off squares where another piece of any color resides (pawns capture diagonally).
            let moves = candidates
                .into_iter()
                .filter_map(to_board_square)
                .filter(|&square| board.at(square).is_none());

            // A pawn may capture diagonally.
            let captures = [
                (rank + direction) * 8 + file + 1,
                (rank + direction) * 8 + file - 1,
            ]
            .into_iter()
            .filter_map(to_board_square)
            .filter(|&square| match board.at(square) {
                Some(other) if other.is_white() != piece.is_white() => true,
                _ => false,
            });

            moves.chain(captures).collect()
        }
        Piece::KnightWhite => Vec::new(),
        Piece::BishopWhite => Vec::new(),
        Piece::RookWhite => Vec::new(),
        Piece::QueenWhite => Vec::new(),
        Piece::KingWhite => Vec::new(),
        Piece::KnightBlack => Vec::new(),
        Piece::BishopBlack => Vec::new(),
        Piece::RookBlack => Vec::new(),
        Piece::QueenBlack => Vec::new(),
        Piece::KingBlack => Vec::new(),
    }
}
