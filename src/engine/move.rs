use std::fmt::Display;

use super::{Square, piece::Piece, square_from_algebraic, square_to_algebraic};

/// A move between two squares.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promote_to: Option<Piece>,
}

#[allow(dead_code)]
impl Move {
    pub fn new(from: Square, to: Square) -> Self {
        Self {
            from,
            to,
            promote_to: None,
        }
    }

    /// Read a move from long algebraic notation.
    /// Will panic if the move is not valid (has rank or file outside normal chess board).
    /// TODO: Read promotions
    /// TODO: is_white parameter
    pub fn from_str(is_white: bool, long_algebraic: &str) -> Move {
        assert!(long_algebraic.len() >= 4);
        let from = square_from_algebraic(&long_algebraic[0..2]);
        let to = square_from_algebraic(&long_algebraic[2..4]);
        let promote_to = long_algebraic.chars().nth(4).and_then(|c| {
            let cm = if is_white {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            };
            Piece::from_char(&cm)
        });

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
            self.promote_to.map_or("".to_owned(), |p| p
                .to_char()
                // UCI move format always has piece in lower case.
                .to_ascii_lowercase()
                .to_string())
        )
    }
}
