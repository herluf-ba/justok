// TODO:
// - [x] Make a function for applying a move to the board
// - [ ] Read FEN positions into bitboards.
// - [ ] Generate a list of valid moves starting at a certain square.

use termchess::{bit_board::BitBoard, board::Board, Move};

const FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
fn format_moves(board: &BitBoard, moves: &Vec<Move>) -> Vec<String> {
    moves
        .iter()
        .map(|r#move| {
            let piece = board.at(r#move.from).unwrap();
            let rank = r#move.to / 8 + 1;
            let file = FILES[(r#move.to % 8) as usize];
            format!("{piece}{file}{rank}")
        })
        .collect::<Vec<_>>()
}

fn main() {
    let standard = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e3 0 1";
    // let standard = "8/8/8/8/8/8/8/6N1 w KQkq e3 0 1";
    let board = BitBoard::from_fen_string(standard);
    println!("{board}");
    let moves = board.generate_moves();
    let formatted = format_moves(&board, &moves).join(", ");
    println!("{formatted}");
}
