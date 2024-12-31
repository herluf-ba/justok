mod board;

use board::Board;

// TODO:
// - [x] Make a function for applying a move to the board
// - [ ] Read FEN positions into bitboards.
// - [ ] Generate a list of valid moves starting at a certain square.

fn main() {
    let standard_with_e4 = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
    let board = Board::from_fen_string(standard_with_e4);
    board.generate_moves();
}
