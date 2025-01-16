use clap::{Parser, Subcommand};
use termchess::board::Board;

/// Command line interface for justok.
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "justok")]
#[command(about = "A chess engine that does just ok.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Computes the number of leaf nodes at [depth]
    /// in the tree of possible positions given the starting [position].
    #[command(arg_required_else_help = true)]
    Perft {
        /// The position to compute perft for in FEN notation.
        position: String,
        /// The depth to compute to.
        depth: u32,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Perft { position, depth } => {
            let board = Board::from_fen(&position);
            let mut total: u128 = 0;
            for m in board.generate_moves() {
                let mut b = board.clone();
                b.apply(m);
                let perft_val = perft(b, depth - 1);
                println!("{m}\t{perft_val}");
                total += perft_val;
            }

            println!("total:\t{total}");
        }
    }
}

fn perft(board: Board, depth: u32) -> u128 {
    let moves = board.generate_moves();
    if depth == 1 {
        return moves.len() as u128;
    } else if depth < 1 {
        return 1;
    }

    moves
        .iter()
        .map(|&moove| {
            let mut board_with_move = board.clone();
            board_with_move.apply(moove);
            perft(board_with_move, depth - 1)
        })
        .sum()
}
