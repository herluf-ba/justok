use std::io;

use justok::{board::Board, Move};

fn main() -> io::Result<()> {
    let mut board = Board::blank();
    let mut best_move: Option<Move> = None;

    let mut input = String::new();
    let stdin = io::stdin();
    loop {
        stdin.read_line(&mut input)?;
        let mut parts = input.trim_end().split_whitespace();
        match parts.next() {
            Some("quit") => break,
            Some("uci") => {
                println!("id name justok 1.0.0");
                println!("id author herlufba");
                println!("uciok");
            }
            Some("isready") => println!("readyok"),
            Some("position") => {
                // Read either fen string or 'startpos' which is the standard position.
                match parts.next() {
                    Some("fen") => {
                        let fen = parts.clone().collect::<Vec<_>>().join(" ");
                        board = Board::from_fen(fen.as_str());
                    }
                    Some("startpos") => {
                        board = Board::from_fen(
                            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                        );
                    }
                    _ => println!("Could not read position."),
                }

                // Read optional list of moves and apply them to the board.
                if let Some("moves") = parts.next() {
                    let moves: Vec<Move> = parts
                        .enumerate()
                        .map(|(idx, move_str)| Move::from_str(idx % 2 == 0, move_str))
                        .collect();
                    for m in moves {
                        board.apply(m);
                    }
                }
            }
            Some("go") => match parts.next() {
                Some(_) => {
                    // TODO: implement all the sub commands like infinite, wtime and so on
                    best_move = board.generate_moves().first().copied();
                    // Just print the best move as soon as it is computed,
                    // since theres no concept of time control yet.
                    if let Some(m) = best_move {
                        println!("info score cp 0");
                        println!("bestmove {m}");
                    }
                }
                _ => println!("Unknown subcommand to 'go'"),
            },
            Some("stop") => {
                if let Some(m) = best_move {
                    println!("info score cp 0");
                    println!("bestmove {m}");
                }
            }
            // NON-UCI DEBUGGING COMMANDS
            Some("board") => {
                println!("{}", board);
                println!("fen: {}", board.to_fen());
                println!(
                    "moves: {}",
                    board
                        .generate_moves()
                        .iter()
                        .map(|m| format!("{}", m))
                        .collect::<Vec<String>>()
                        .join(", ")
                );
            }
            Some(c) => println!("Unknown command '{c}'"),
            None => {}
        }
        input.clear();
    }
    Ok(())
}
