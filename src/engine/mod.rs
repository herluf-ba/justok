pub mod board;
pub mod r#move;
pub mod piece;

/// Letters of the eight files on a chess board.
pub const FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

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

pub fn square_to_algebraic(square: Square) -> String {
    let rank = (square / 8) + 1;
    let file = FILES[(square % 8) as usize];
    format!("{file}{rank}").to_owned()
}

pub fn square_from_algebraic(long_algebraic: &str) -> Square {
    let mut chars = long_algebraic.chars();
    let file = match chars.next() {
        Some('a') => 0,
        Some('b') => 1,
        Some('c') => 2,
        Some('d') => 3,
        Some('e') => 4,
        Some('f') => 5,
        Some('g') => 6,
        Some('h') => 7,
        _ => panic!("Invalid square {}", long_algebraic),
    };
    let rank = chars.next().and_then(|c| c.to_digit(10)).unwrap() - 1;
    u8::try_from(rank * 8 + file).unwrap()
}
