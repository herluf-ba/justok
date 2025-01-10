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
    let fen = "r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R b - - 11 7";
    let board = Board::from_fen(fen);
    println!("{board}");
    let moves = board.generate_moves();
    let formatted = format_moves(&board, &moves).join(", ");
    println!("{formatted}");
    println!("{}", board.to_fen());
}
