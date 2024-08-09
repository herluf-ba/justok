mod board;

use board::{Board, Move, Square};

fn main() {
    let mut board = Board::new();
    let move1 = Move {
        from: Square::A1,
        to: Square::A2,
    };
    println!("{}", board);
    board.apply(move1);
    println!("{}", board);
}
