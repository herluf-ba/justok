use std::fmt::Display;

use crate::{square_from_algebraic, square_to_algebraic, to_board_square, Move, Piece, Square};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Board {
    pieces: [Option<Piece>; 64],
    white_to_move: bool,
    en_pessant_square: Option<Square>,
    can_white_castle_king_side: bool,
    can_white_castle_queen_side: bool,
    can_black_castle_king_side: bool,
    can_black_castle_queen_side: bool,
    half_move_clock: u32,
    full_move_counter: u32,
}

impl Board {
    /// Create a new chess board with no pieces placed.
    pub fn blank() -> Self {
        Self {
            pieces: [None; 64],
            white_to_move: true,
            en_pessant_square: None,
            can_white_castle_king_side: true,
            can_white_castle_queen_side: true,
            can_black_castle_king_side: true,
            can_black_castle_queen_side: true,
            half_move_clock: 0,
            full_move_counter: 0,
        }
    }

    /// Create a board from a Forsyth-Edwards-Notation (FEN) string.
    pub fn from_fen(fen: &str) -> Self {
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

        // Read castling rights.
        let castling = fields[2];
        board.can_white_castle_king_side = castling.contains("K");
        board.can_white_castle_queen_side = castling.contains("Q");
        board.can_black_castle_king_side = castling.contains("k");
        board.can_black_castle_queen_side = castling.contains("q");

        // Read en pessant square.
        match fields[3] {
            "-" => {
                board.en_pessant_square = None;
            }
            square => board.en_pessant_square = Some(square_from_algebraic(square)),
        }

        // Read half and full clock counts.
        board.half_move_clock = fields[4].parse::<u32>().unwrap();
        board.full_move_counter = fields[5].parse::<u32>().unwrap();

        board
    }

    /// Create a Forsyth-Edwards-Notation (FEN) string from the current board.
    pub fn to_fen(&self) -> String {
        // FEN contains 6 fields separated by space.
        // They are:
        // 1. Piece placement.
        // 2. Side to move (w/b)
        // 3. Castling ability
        // 4. En pessant target square
        // 5. Halfmove clock
        // 6. Fullmove counter
        // Fields 5. and 6. may be left out.
        let mut fen = String::with_capacity(65 + 2 + 5 + 3 + 2 + 2);

        // Generate the piece placement
        for r in (0..8).rev() {
            let mut consequitive_empty = 0;
            for f in 0..8 {
                match self.at(r * 8 + f) {
                    None => {
                        consequitive_empty += 1;
                    }
                    Some(piece) => {
                        if consequitive_empty > 0 {
                            fen.push(char::from_digit(consequitive_empty, 10).unwrap());
                            consequitive_empty = 0;
                        }
                        fen.push(piece.to_char())
                    }
                }
            }
            if consequitive_empty > 0 {
                fen.push(char::from_digit(consequitive_empty, 10).unwrap());
            }
            fen.push('/');
        }

        fen.pop(); // Drop trailing '/'.

        // Write whose turn it is.
        match self.white_to_move {
            true => fen.push_str(" w"),
            false => fen.push_str(" b"),
        }

        // Write castling rights.
        fen.push(' ');
        if self.can_white_castle_king_side {
            fen.push('K');
        }
        if self.can_white_castle_queen_side {
            fen.push('Q');
        }
        if self.can_black_castle_king_side {
            fen.push('k');
        }
        if self.can_black_castle_queen_side {
            fen.push('q');
        }
        let cant_castle = !self.can_white_castle_king_side
            && !self.can_white_castle_queen_side
            && !self.can_black_castle_king_side
            && !self.can_black_castle_queen_side;
        if cant_castle {
            fen.push('-');
        }

        // Write en pessant square
        fen.push(' ');
        match self.en_pessant_square {
            None => {
                fen.push('-');
            }
            Some(square) => fen.push_str(&square_to_algebraic(square)),
        }
        // Write half and full move counts.
        fen.push(' ');
        fen.push_str(self.half_move_clock.to_string().as_str());
        fen.push(' ');
        fen.push_str(self.full_move_counter.to_string().as_str());

        fen
    }

    /// Place a [Piece] within the board without updating any other state.
    fn place(&mut self, piece: Piece, at: Square) {
        self.pieces[at as usize] = Some(piece);
    }

    /// Applies a move to the board. The move is assummed to be legal.
    pub fn apply(&mut self, r#move: Move) {
        // TODO: Detect en-pessant captures.
        let is_capture = self.at(r#move.to).is_some();
        if let Some(p) = self.at(r#move.from) {
            self.pieces[r#move.from as usize] = None;
            self.pieces[r#move.to as usize] = Some(p);

            // Set the half clock.
            let is_pawn_move = p == Piece::PawnWhite || p == Piece::PawnBlack;
            if is_pawn_move || is_capture {
                self.half_move_clock = 0;
            } else {
                self.half_move_clock += 1;
            }

            // Set en pessant square.
            if is_pawn_move {
                self.en_pessant_square = match r#move.to as i8 - r#move.from as i8 {
                    16 => Some(r#move.from + 8),
                    -16 => Some(r#move.from - 8),
                    _ => None,
                }
            } else {
                self.en_pessant_square = None;
            }

            // Update castling rights
            self.can_white_castle_queen_side &= !(p == Piece::KingWhite || r#move.from == 0);
            self.can_white_castle_king_side &= !(p == Piece::KingWhite || r#move.from == 7);
            self.can_black_castle_queen_side &= !(p == Piece::KingBlack || r#move.from == 56);
            self.can_black_castle_king_side &= !(p == Piece::KingBlack || r#move.from == 63);

            // Update whose turn it is, and increment the move counter if needed.
            self.white_to_move = !self.white_to_move;
            if self.white_to_move {
                self.full_move_counter += 1;
            }
        }
    }

    /// Lookup what piece is at a particular square in the board.
    pub fn at(&self, square: Square) -> Option<Piece> {
        *self.pieces.get(square as usize)?
    }

    /// Generate all legal [Move]s possible within the current [Board].
    pub fn generate_moves(&self) -> Vec<Move> {
        let pseudo_legal = self.generate_pseudo_moves();
        pseudo_legal
            .into_iter()
            .filter(|&m| {
                // Check to see if the move is legal,
                // by applying it to the board and check if
                // the opponent can capture the king on the next move.
                // If so, the move would leave the king in check.
                // Note: This is highly inefficient but it is correct.
                let mut b = (*self).clone();
                b.apply(m);
                let king_pos = b
                    .pieces
                    .iter()
                    .position(|p| match p {
                        Some(Piece::KingWhite) if self.white_to_move => true,
                        Some(Piece::KingBlack) if !self.white_to_move => true,
                        _ => false,
                    })
                    .unwrap();
                b.generate_pseudo_moves()
                    .into_iter()
                    .find(|&m| m.to as usize == king_pos)
                    .is_none()
            })
            .collect()
    }

    /// Generate all pseudo-legal [Move]s possible within the current [Board].
    /// A pseudo legal move may leave the player in check.
    pub fn generate_pseudo_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = rank * 8 + file;
                match self.at(square) {
                    // Square had a piece with the color whose turn it is
                    Some(piece) if self.white_to_move == piece.is_white() => {
                        let valid_target_squares = generate_piece_moves(self, piece, square);
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
}

/// Generate the valid moves for a particular piece on a certain square within a board.
fn generate_piece_moves(board: &Board, piece: Piece, at: Square) -> Vec<Square> {
    match piece {
        Piece::PawnWhite | Piece::PawnBlack => {
            let rank = at / 8;
            let file = at % 8;

            let mut moves: Vec<Square> = vec![];

            // A pawn may move one square towards the opposing player.
            let advance_square = match piece.is_white() {
                true if rank < 8 => Some((rank + 1) * 8 + file),
                false if rank > 0 => Some((rank - 1) * 8 + file),
                _ => None,
            };

            // If it is in it's starting rank, it may leap two squares.
            let leap_square = match piece.is_white() {
                true if rank == 1 => Some((rank + 2) * 8 + file),
                false if rank == 6 => Some((rank - 2) * 8 + file),
                _ => None,
            };

            if advance_square.is_some_and(|s| board.at(s).is_none()) {
                moves.push(advance_square.unwrap());
                if leap_square.is_some_and(|s| board.at(s).is_none()) {
                    moves.push(leap_square.unwrap());
                }
            }

            // A pawn may capture diagonally.
            let capture_l = match piece.is_white() {
                true if rank < 8 && file > 1 => Some((rank + 1) * 8 + file - 1),
                false if rank > 0 && file > 1 => Some((rank - 1) * 8 + file - 1),
                _ => None,
            };

            let can_capture_l = capture_l.map(|s| {
                let sees_enemy_piece = matches!(
                    board.at(s),
                    Some(other) if other.is_white() != piece.is_white()
                );
                let en_pessant = board.en_pessant_square.is_some_and(|es| es == s);
                sees_enemy_piece || en_pessant
            }) == Some(true);
            if can_capture_l {
                moves.push(capture_l.unwrap());
            }
            let capture_r = match piece.is_white() {
                true if rank < 8 && file < 7 => Some((rank + 1) * 8 + file + 1),
                false if rank > 0 && file < 7 => Some((rank - 1) * 8 + file + 1),
                _ => None,
            };
            let can_capture_r = capture_r.map(|s| {
                let sees_enemy_piece = matches!(
                    board.at(s),
                    Some(other) if other.is_white() != piece.is_white()
                );
                let en_pessant = board.en_pessant_square.is_some_and(|es| es == s);
                sees_enemy_piece || en_pessant
            }) == Some(true);

            if can_capture_r {
                moves.push(capture_r.unwrap());
            }

            moves
        }
        Piece::KnightWhite | Piece::KnightBlack => {
            // Knights may move two squares orthogonally and then one square along the other orthogonal axis.
            // That comes out to 8 unique squares to land on.
            // . x . x .
            // x . . . x
            // . . n . .
            // x . . . x
            // . x . x .
            let rank = at as i8 / 8;
            let file = at as i8 % 8;
            [
                (rank + 2, file + 1),
                (rank + 2, file - 1),
                (rank - 2, file + 1),
                (rank - 2, file - 1),
                (rank + 1, file + 2),
                (rank + 1, file - 2),
                (rank - 1, file + 2),
                (rank - 1, file - 2),
            ]
            .into_iter()
            .filter(|(r, f)| !(*r > 7 || *f > 7 || *r < 0 || *f < 0))
            .map(|(r, f)| to_board_square(r * 8 + f).unwrap())
            // A knight may land on a square with a opposite colored piece or no piece.
            .filter(|&square| match board.at(square) {
                None => true,
                Some(other) if other.is_white() != piece.is_white() => true,
                _ => false,
            })
            .collect()
        }
        Piece::KingWhite | Piece::KingBlack => {
            // The king may move to any surrounding square.
            let rank = at as i8 / 8;
            let file = at as i8 % 8;
            [
                (rank + 1, file + 1),
                (rank + 1, file),
                (rank + 1, file - 1),
                (rank - 1, file + 1),
                (rank - 1, file),
                (rank - 1, file - 1),
                (rank, file + 1),
                (rank, file - 1),
            ]
            .into_iter()
            .filter(|(r, f)| !(*r > 7 || *f > 7 || *r < 0 || *f < 0))
            .map(|(r, f)| to_board_square(r * 8 + f).unwrap())
            // The king may land on a square with a opposite colored piece or no piece.
            .filter(|&square| match board.at(square) {
                Some(other) if other.is_white() != piece.is_white() => true,
                None => true,
                _ => false,
            })
            .collect()
        }
        Piece::RookWhite | Piece::RookBlack => {
            let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            sliding_moves(&directions, board, piece, at)
        }
        Piece::BishopWhite | Piece::BishopBlack => {
            let directions = [(1, 1), (-1, -1), (-1, 1), (1, -1)];
            sliding_moves(&directions, board, piece, at)
        }
        Piece::QueenWhite | Piece::QueenBlack => {
            let directions = [
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
                (1, 1),
                (-1, -1),
                (-1, 1),
                (1, -1),
            ];
            sliding_moves(&directions, board, piece, at)
        }
    }
}

/// Helper function for computing sliding moves in both orthogonal and diagonal directions.
fn sliding_moves(directions: &[(i8, i8)], board: &Board, piece: Piece, at: Square) -> Vec<Square> {
    let mut moves: Vec<Square> = Vec::new();
    let rank = at as i8 / 8;
    let file = at as i8 % 8;
    for (ro, fo) in directions {
        let mut r = rank;
        let mut f = file;
        loop {
            r += ro;
            f += fo;
            if r > 7 || f > 7 || r < 0 || f < 0 {
                break;
            }
            let square = to_board_square(r * 8 + f).unwrap();
            match board.at(square) {
                None => moves.push(square),
                Some(other) => {
                    if other.is_white() != piece.is_white() {
                        moves.push(square)
                    }
                    break;
                }
            }
        }
    }

    moves
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
