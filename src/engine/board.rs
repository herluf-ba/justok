use super::r#move::Move;
use super::piece::Piece;
use super::{Square, square_from_algebraic, square_to_algebraic, to_board_square};
use std::fmt::Display;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Board {
    pieces: [Option<Piece>; 64],
    pub white_to_move: bool,
    en_pessant_square: Option<Square>,
    can_white_castle_king_side: bool,
    can_white_castle_queen_side: bool,
    can_black_castle_king_side: bool,
    can_black_castle_queen_side: bool,
    half_move_clock: u32,
    full_move_counter: u32,
}

/// Additional information about a move.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct MoveInfo {
    /// Pairs of (from, to) square indicies, indicating that the piece on `from` was moved to `to`.
    /// Castling will result in both the rook and king move being added to this list.
    pub moved_pieces: Vec<(Square, Square)>,
    /// List of squares from which a piece was removed.
    /// Promotions will add a removed pawn to this list and the promoted piece to `added_pieces`.
    pub removed_pieces: Vec<Square>,
    /// Pairs of (square, piece) indicating that a piece of kind `piece` was added to `square`.
    /// Promotions will add the promoted piece to this list.
    pub added_pieces: Vec<(Square, Piece)>,
}

#[allow(dead_code)]
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
    pub fn apply(&mut self, r#move: Move) -> MoveInfo {
        let (from, to) = (r#move.from, r#move.to);
        let mut moved_pieces = Vec::new();
        let mut removed_pieces = Vec::new();
        let mut added_pieces = Vec::new();

        // Record captured piece.
        let is_capture = self.at(to).is_some();
        if is_capture {
            removed_pieces.push(to);
        }

        // Update the state of the moved piece.
        let piece = self.at(from).expect("move to target a piece");
        self.pieces[from as usize] = None;
        if let Some(promote_to) = r#move.promote_to {
            self.pieces[to as usize] = Some(promote_to);
            removed_pieces.push(from);
            added_pieces.push((to, promote_to));
        } else {
            self.pieces[to as usize] = Some(piece);
            moved_pieces.push((from, to));
        }

        // Set the half clock.
        let is_pawn_move = piece == Piece::PawnWhite || piece == Piece::PawnBlack;
        if is_pawn_move || is_capture {
            self.half_move_clock = 0;
        } else {
            self.half_move_clock += 1;
        }

        // Capture en-pessant
        let changed_file = (from % 8) != (to % 8);
        if is_pawn_move && changed_file && !is_capture {
            // A pawn changed file without doing a capture. This only happens by en pessant.
            let captured_pawn_square = match self.white_to_move {
                true => to - 8,
                false => to + 8,
            };
            self.pieces[captured_pawn_square as usize] = None;
            removed_pieces.push(captured_pawn_square);
        }

        // Set en pessant square.
        if is_pawn_move {
            self.en_pessant_square = match to as i8 - from as i8 {
                16 => Some(from + 8),
                -16 => Some(from - 8),
                _ => None,
            }
        } else {
            self.en_pessant_square = None;
        }

        // Move the rook when a player castles.
        match (piece, r#move) {
            (Piece::KingWhite, Move { from: 4, to: 2, .. }) => {
                self.pieces[0] = None;
                self.pieces[3] = Some(Piece::RookWhite);
                moved_pieces.push((0, 3));
            }
            (Piece::KingWhite, Move { from: 4, to: 6, .. }) => {
                self.pieces[7] = None;
                self.pieces[5] = Some(Piece::RookWhite);
                moved_pieces.push((7, 5));
            }
            (
                Piece::KingBlack,
                Move {
                    from: 60, to: 62, ..
                },
            ) => {
                self.pieces[63] = None;
                self.pieces[61] = Some(Piece::RookBlack);
                moved_pieces.push((63, 61));
            }
            (
                Piece::KingBlack,
                Move {
                    from: 60, to: 58, ..
                },
            ) => {
                self.pieces[56] = None;
                self.pieces[59] = Some(Piece::RookBlack);
                moved_pieces.push((56, 59));
            }
            _ => {}
        }

        // Update castling rights
        self.can_white_castle_queen_side &= !(piece == Piece::KingWhite || from == 0 || to == 0);
        self.can_white_castle_king_side &= !(piece == Piece::KingWhite || from == 7 || to == 7);
        self.can_black_castle_queen_side &= !(piece == Piece::KingBlack || from == 56 || to == 56);
        self.can_black_castle_king_side &= !(piece == Piece::KingBlack || from == 63 || to == 63);

        // Update whose turn it is, and increment the move counter if needed.
        self.white_to_move = !self.white_to_move;
        if self.white_to_move {
            self.full_move_counter += 1;
        }

        return MoveInfo {
            moved_pieces,
            removed_pieces,
            added_pieces,
        };
    }

    /// Lookup what piece is at a particular square in the board.
    pub fn at(&self, square: Square) -> Option<Piece> {
        *self.pieces.get(square as usize)?
    }

    fn is_in_check(&self) -> bool {
        self.is_side_in_check(self.white_to_move)
    }

    fn is_opponent_in_check(&self) -> bool {
        self.is_side_in_check(!self.white_to_move)
    }

    /// Does this move leave the current player in check?
    /// The check is performed by applying the move,
    /// then checking if the opponent is in check.
    fn would_leave_in_check(&self, r#move: Move) -> bool {
        let mut b = (*self).clone();
        b.apply(r#move);
        b.is_opponent_in_check()
    }

    /// Is black or white in check?
    fn is_side_in_check(&self, check_white: bool) -> bool {
        // Get position of players king
        let king_pos = self.pieces.iter().position(|p| match p {
            Some(Piece::KingWhite) if check_white => true,
            Some(Piece::KingBlack) if !check_white => true,
            _ => false,
        });

        // Pretend that the players king is any other piece of the same color.
        // If the piece is able to capture a piece of the same kind,
        // then the king is in check.
        // So for example if we pretend the white king is a white knight,
        // and that knight can capture a black knight,
        // that means the black knight is checking the white king.
        if let Some(king_pos) = king_pos {
            use Piece::*;
            let pieces = if check_white {
                [
                    QueenWhite,
                    BishopWhite,
                    RookWhite,
                    KnightWhite,
                    PawnWhite,
                    KingWhite,
                ]
            } else {
                [
                    QueenBlack,
                    BishopBlack,
                    RookBlack,
                    KnightBlack,
                    PawnBlack,
                    KingBlack,
                ]
            };
            for piece in pieces {
                let moves = generate_piece_moves(self, piece, king_pos as u8, true);
                let has_capture = moves
                    .iter()
                    .find(|m| {
                        self.at(m.to)
                            .is_some_and(|other| other.is_same_kind(&piece))
                    })
                    .is_some();

                if has_capture {
                    return true;
                }
            }
        }

        false
    }

    /// Generate all legal [Move]s possible within the current [Board].
    pub fn generate_moves(&self) -> Vec<Move> {
        self.generate_pseudo_moves()
            .into_iter()
            .filter(|&m| !self.would_leave_in_check(m))
            .collect()
    }

    /// Generate all legal [Move]s possible within the current [Board].
    pub fn generate_square_moves(&self, from: Square) -> Vec<Move> {
        if let Some(piece) = self.at(from) {
            return generate_piece_moves(self, piece, from, false)
                .into_iter()
                .filter(|&m| !self.would_leave_in_check(m))
                .collect();
        } else {
            return Vec::new();
        }
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
                        let mut piece_moves = generate_piece_moves(self, piece, square, false);
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
fn generate_piece_moves(board: &Board, piece: Piece, at: Square, skip_castling: bool) -> Vec<Move> {
    match piece {
        Piece::PawnWhite | Piece::PawnBlack => {
            let rank = at / 8;
            let file = at % 8;

            let mut moves: Vec<Move> = vec![];

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
                moves.push(Move::new(at, advance_square.unwrap()));
                if leap_square.is_some_and(|s| board.at(s).is_none()) {
                    moves.push(Move::new(at, leap_square.unwrap()));
                }
            }

            // A pawn may capture diagonally.
            let capture_l = match piece.is_white() {
                true if rank < 8 && file > 0 => Some((rank + 1) * 8 + file - 1),
                false if rank > 0 && file > 0 => Some((rank - 1) * 8 + file - 1),
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
                moves.push(Move::new(at, capture_l.unwrap()));
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
                moves.push(Move::new(at, capture_r.unwrap()));
            }

            moves
                .iter()
                // Handle promotions.
                .flat_map(|&Move { from, to, .. }| {
                    use Piece::*;
                    match (piece.is_white(), to / 8) {
                        // White pawn made it to rank 8. Expand the move to all possible promotions.
                        (true, 7) => [KnightWhite, BishopWhite, RookWhite, QueenWhite]
                            .iter()
                            .map(|&p| Move {
                                from,
                                to,
                                promote_to: Some(p),
                            })
                            .collect::<Vec<Move>>(),
                        // Black pawn made it to rank 1. Expand the move to all possible promotions.
                        (false, 0) => [KnightBlack, BishopBlack, RookBlack, QueenBlack]
                            .iter()
                            .map(|&p| Move {
                                from,
                                to,
                                promote_to: Some(p),
                            })
                            .collect::<Vec<Move>>(),

                        _ => vec![Move::new(from, to)],
                    }
                })
                .collect()
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
            .map(|square| Move::new(at, square))
            .collect()
        }
        Piece::KingWhite | Piece::KingBlack => {
            // The king may move to any surrounding square.
            let rank = at as i8 / 8;
            let file = at as i8 % 8;
            let mut moves: Vec<Move> = [
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
            .map(|square| Move::new(at, square))
            .collect();

            if !skip_castling {
                // Castling queen side for white.
                if board.white_to_move
                // Still has castling rights
                && board.can_white_castle_queen_side
                // No piece is obstructing the castling
                && board.pieces[3].is_none() && board.pieces[2].is_none() && board.pieces[1].is_none()
                // Player is not in check
                && !board.is_in_check()
                // Check that d1 is not attacked, since the king can't castle through check.
                && !board.would_leave_in_check(Move::new(4, 3))
                {
                    moves.push(Move::new(at, 2));
                }
                // Castling king side for white.
                if board.white_to_move
                // Still has castling rights
                && board.can_white_castle_king_side
                // No piece is obstructing the castling
                && board.pieces[5].is_none() && board.pieces[6].is_none()
                // Player is not in check
                && !board.is_in_check()
                // Check that e1 is not attacked, since the king can't castle through check.
                && !board.would_leave_in_check(Move::new(4, 5))
                {
                    moves.push(Move::new(at, 6));
                }
                // Castling queen side for black.
                if !board.white_to_move
                // Still has castling rights
                && board.can_black_castle_queen_side
                // No piece is obstructing the castling
                && board.pieces[59].is_none() && board.pieces[58].is_none() && board.pieces[57].is_none()
                // Player is not in check
                && !board.is_in_check()
                // Check that d8 is not attacked, since the king can't castle through check.
                && !board.would_leave_in_check(Move::new(60, 59))
                {
                    moves.push(Move::new(at, 58));
                }
                // Castling king side for black.
                if !board.white_to_move
                // Still has castling rights
                && board.can_black_castle_king_side
                // No piece is obstructing the castling
                && board.pieces[61].is_none() && board.pieces[62].is_none()
                // Player is not in check
                && !board.is_in_check()
                // Check that e8 is not attacked, since the king can't castle through check.
                && !board.would_leave_in_check(Move::new(60, 61))
                {
                    moves.push(Move::new(at, 62));
                }
            }

            moves
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
fn sliding_moves(directions: &[(i8, i8)], board: &Board, piece: Piece, at: Square) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
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
                None => moves.push(Move::new(at, square)),
                Some(other) => {
                    if other.is_white() != piece.is_white() {
                        moves.push(Move::new(at, square))
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
