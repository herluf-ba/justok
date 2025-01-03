use std::fmt::Display;

use crate::{to_board_square, Move, Piece, Square};

pub trait Board: PartialEq + Eq + Clone + Copy + Display {
    /// Create a new chess board with no pieces placed.
    fn blank() -> Self;

    /// Create a board from a Forsyth-Edwards-Notation (FEN) string.
    fn from_fen_string(fen: &str) -> Self;

    /// Clear a square within the board.
    fn clear(&mut self, square: Square);

    /// Place a [Piece] within the board.
    fn place(&mut self, piece: Piece, at: Square);

    /// Applies a move to the board with _no_ validation.
    fn apply(&mut self, r#move: Move);

    /// Lookup what piece is at a particular square in the board.
    fn at(&self, square: Square) -> Option<Piece>;

    /// Is it whites turn to move?
    fn white_to_move(&self) -> bool;

    /// Generate all [Move]s possible within the current [Board].
    fn generate_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = rank * 8 + file;
                match self.at(square) {
                    // Square had a piece with the color whose turn it is
                    Some(piece) if self.white_to_move() == piece.is_white() => {
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
fn generate_piece_moves<T: Board>(board: &T, piece: Piece, at: Square) -> Vec<Square> {
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
            let new_rank = rank + direction;
            let mut candidates = if new_rank < 8 && new_rank > 0 {
                vec![(rank + direction) * 8 + file]
            } else {
                vec![]
            };
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
        Piece::KnightWhite | Piece::KnightBlack => {
            let rank = at as i8 / 8;
            let file = at as i8 % 8;
            // Knights may move two squares orthogonally and then one square along the other orthogonal axis.
            // That comes out to 8 unique squares to land on.
            // . x . x .
            // x . . . x
            // . . n . .
            // x . . . x
            // . x . x .
            [
                (rank < 6 && file < 7, (rank + 2) * 8 + file + 1),
                (rank < 6 && file > 0, (rank + 2) * 8 + file - 1),
                (rank > 2 && file < 7, (rank - 2) * 8 + file + 1),
                (rank > 2 && file > 0, (rank - 2) * 8 + file - 1),
                (rank < 7 && file > 2, (rank + 1) * 8 + file - 2),
                (rank > 0 && file > 2, (rank - 1) * 8 + file - 2),
                (rank < 7 && file < 6, (rank + 1) * 8 + file + 2),
                (rank > 0 && file < 6, (rank - 1) * 8 + file + 2),
            ]
            .into_iter()
            // filter off the squares outside the board
            .filter_map(|(has_space, square)| {
                has_space
                    .then_some(square)
                    .map(|s| u8::try_from(s).ok())
                    .flatten()
            })
            // A knight may land on a square with a opposite colored piece or no piece.
            .filter(|&square| match board.at(square) {
                Some(other) if other.is_white() != piece.is_white() => true,
                None => true,
                _ => false,
            })
            .collect()
        }
        Piece::KingWhite | Piece::KingBlack => {
            // The king may move to any surrounding square.
            let rank = at as i8 / 8;
            let file = at as i8 % 8;
            let (left, right, top, bottom) = (file > 0, file < 7, rank < 7, rank > 0);
            [
                (top && right, (rank + 1) * 8 + file + 1),
                (top && left, (rank + 1) * 8 + file - 1),
                (top, (rank + 1) * 8 + file),
                (left, rank * 8 + file - 1),
                (right, rank * 8 + file + 1),
                (bottom && right, (rank - 1) * 8 + file + 1),
                (bottom && left, (rank - 1) * 8 + file - 1),
                (bottom, (rank - 1) * 8 + file),
            ]
            .into_iter()
            // filter off the squares outside the board
            .filter_map(|(has_space, square)| {
                has_space
                    .then_some(square)
                    .map(|s| u8::try_from(s).ok())
                    .flatten()
            })
            // The king may land on a square with a opposite colored piece or no piece.
            .filter(|&square| match board.at(square) {
                Some(other) if other.is_white() != piece.is_white() => true,
                None => true,
                _ => false,
            })
            .collect()
        }
        Piece::RookWhite | Piece::RookBlack => orthogonal_moves(board, piece, at),
        Piece::BishopWhite | Piece::BishopBlack => diagonal_moves(board, piece, at),
        Piece::QueenWhite | Piece::QueenBlack => {
            let mut moves = orthogonal_moves(board, piece, at);
            let diagonal = diagonal_moves(board, piece, at);
            moves.extend(diagonal);
            moves
        }
    }
}

fn orthogonal_moves<T: Board>(board: &T, piece: Piece, at: Square) -> Vec<Square> {
    let rank = at / 8;
    let file = at % 8;

    let mut moves = Vec::new();
    for f in (0..file).rev() {
        let square = rank * 8 + f;
        match board.at(square) {
            Some(other) => {
                if other.is_white() != piece.is_white() {
                    moves.push(square)
                }
                break;
            }
            None => {
                moves.push(square);
            }
        }
    }
    for f in (file + 1)..8 {
        let square = rank * 8 + f;
        match board.at(square) {
            Some(other) => {
                if other.is_white() != piece.is_white() {
                    moves.push(square)
                }
                break;
            }
            None => {
                moves.push(square);
            }
        }
    }
    for r in (0..rank).rev() {
        let square = r * 8 + file;
        match board.at(square) {
            Some(other) => {
                if other.is_white() != piece.is_white() {
                    moves.push(square)
                }
                break;
            }
            None => {
                moves.push(square);
            }
        }
    }
    for r in (rank + 1)..8 {
        let square = r * 8 + file;
        match board.at(square) {
            Some(other) => {
                if other.is_white() != piece.is_white() {
                    moves.push(square)
                }
                break;
            }
            None => {
                moves.push(square);
            }
        }
    }

    moves
}

fn diagonal_moves<T: Board>(board: &T, piece: Piece, at: Square) -> Vec<Square> {
    let mut moves = Vec::new();
    let rank = at / 8;
    let file = at % 8;

    let mut r = rank;
    let mut f = file;
    loop {
        r += 1;
        f += 1;
        if r > 7 || f > 7 {
            break;
        }
        let square = r * 8 + f;
        match board.at(square) {
            Some(other) => {
                if other.is_white() != piece.is_white() {
                    moves.push(square)
                }
                break;
            }
            None => moves.push(square),
        }
    }

    moves
}
