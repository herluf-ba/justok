// TODO:
// - [x] Make a function for applying a move to the board
// - [ ] Read FEN positions into bitboards.
// - [ ] Generate a list of valid moves starting at a certain square.

use termchess::{board::Board, square_to_algebraic, Move};

fn format_moves(board: &Board, moves: &Vec<Move>) -> Vec<String> {
    moves
        .iter()
        .map(|r#move| {
            let piece = board.at(r#move.from).unwrap();
            let m = square_to_algebraic(r#move.to);
            format!("{piece}{m}")
        })
        .collect::<Vec<_>>()
}

fn main() {
    let fen = "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP4/Q1n2N2/P4PPP/3R1K1R w - - 4 18";
    let board = Board::from_fen(fen);
    println!("{board}");
    let moves = board.generate_moves();
    let formatted = format_moves(&board, &moves).join(", ");
    println!("{formatted}");
    println!("{}", board.to_fen());
}
