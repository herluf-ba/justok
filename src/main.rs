mod board;

use board::{Board, Move, Square};

// TODO:
// -[x] Make a function for applying a move to the board
// -[ ] Generate a list of valid moves starting at a certain square.

fn main() {
    let mut board = Board::new();
    let move1 = Move {
        from: Square::A1,
        to: Square::D5,
    };
    println!("{}", board);
    board.apply(move1);
    println!("{}", board);
}
