use std::collections::HashSet;

use termchess::board::Board;
use termchess::Move;

#[test]
fn opening_moves() {
    // Initialize the board and generate all possible moves.
    let standard = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let generated_moves: HashSet<Move> = Board::from_fen(standard)
        .generate_moves()
        .into_iter()
        .collect();

    let expected = vec![
        (
            Move::from_str("b1a3"),
            "rnbqkbnr/pppppppp/8/8/8/N7/PPPPPPPP/R1BQKBNR b KQkq - 1 1",
        ),
        (
            Move::from_str("b1c3"),
            "rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR b KQkq - 1 1",
        ),
        (
            Move::from_str("g1f3"),
            "rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b KQkq - 1 1",
        ),
        (
            Move::from_str("g1h3"),
            "rnbqkbnr/pppppppp/8/8/8/7N/PPPPPPPP/RNBQKB1R b KQkq - 1 1",
        ),
        (
            Move::from_str("a2a3"),
            "rnbqkbnr/pppppppp/8/8/8/P7/1PPPPPPP/RNBQKBNR b KQkq - 0 1",
        ),
        (
            Move::from_str("a2a4"),
            "rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq a3 0 1",
        ),
        (
            Move::from_str("b2b3"),
            "rnbqkbnr/pppppppp/8/8/8/1P6/P1PPPPPP/RNBQKBNR b KQkq - 0 1",
        ),
        (
            Move::from_str("b2b4"),
            "rnbqkbnr/pppppppp/8/8/1P6/8/P1PPPPPP/RNBQKBNR b KQkq b3 0 1",
        ),
        (
            Move::from_str("c2c3"),
            "rnbqkbnr/pppppppp/8/8/8/2P5/PP1PPPPP/RNBQKBNR b KQkq - 0 1",
        ),
        (
            Move::from_str("c2c4"),
            "rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR b KQkq c3 0 1",
        ),
        (
            Move::from_str("d2d3"),
            "rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR b KQkq - 0 1",
        ),
        (
            Move::from_str("d2d4"),
            "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1",
        ),
        (
            Move::from_str("e2e3"),
            "rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR b KQkq - 0 1",
        ),
        (
            Move::from_str("e2e4"),
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        ),
        (
            Move::from_str("f2f3"),
            "rnbqkbnr/pppppppp/8/8/8/5P2/PPPPP1PP/RNBQKBNR b KQkq - 0 1",
        ),
        (
            Move::from_str("f2f4"),
            "rnbqkbnr/pppppppp/8/8/5P2/8/PPPPP1PP/RNBQKBNR b KQkq f3 0 1",
        ),
        (
            Move::from_str("g2g3"),
            "rnbqkbnr/pppppppp/8/8/8/6P1/PPPPPP1P/RNBQKBNR b KQkq - 0 1",
        ),
        (
            Move::from_str("g2g4"),
            "rnbqkbnr/pppppppp/8/8/6P1/8/PPPPPP1P/RNBQKBNR b KQkq g3 0 1",
        ),
        (
            Move::from_str("h2h3"),
            "rnbqkbnr/pppppppp/8/8/8/7P/PPPPPPP1/RNBQKBNR b KQkq - 0 1",
        ),
        (
            Move::from_str("h2h4"),
            "rnbqkbnr/pppppppp/8/8/7P/8/PPPPPPP1/RNBQKBNR b KQkq h3 0 1",
        ),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(standard);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}

#[test]
fn ruy_lopez() {
    // Check possible moves after the popular Ruy Lopez opening.
    let standard = "r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R b - - 11 7";
    let generated_moves: HashSet<Move> = Board::from_fen(standard)
        .generate_moves()
        .into_iter()
        .collect();

    let expected = vec![
        (
            Move::from_str("c6b4"),
            "r1bqkbnr/pppp1ppp/8/1B2p3/1n2P3/5N2/PPPP1PPP/RNBQK2R w - - 12 8",
        ),
        (
            Move::from_str("c6d4"),
            "r1bqkbnr/pppp1ppp/8/1B2p3/3nP3/5N2/PPPP1PPP/RNBQK2R w - - 12 8",
        ),
        (
            Move::from_str("c6a5"),
            "r1bqkbnr/pppp1ppp/8/nB2p3/4P3/5N2/PPPP1PPP/RNBQK2R w - - 12 8",
        ),
        (
            Move::from_str("c6e7"),
            "r1bqkbnr/ppppnppp/8/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w - - 12 8",
        ),
        (
            Move::from_str("c6b8"),
            "rnbqkbnr/pppp1ppp/8/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w - - 12 8",
        ),
        (
            Move::from_str("a7a6"),
            "r1bqkbnr/1ppp1ppp/p1n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w - - 0 8",
        ),
        (
            Move::from_str("a7a5"),
            "r1bqkbnr/1ppp1ppp/2n5/pB2p3/4P3/5N2/PPPP1PPP/RNBQK2R w - a6 0 8",
        ),
        (
            Move::from_str("b7b6"),
            "r1bqkbnr/p1pp1ppp/1pn5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w - - 0 8",
        ),
        (
            Move::from_str("d7d6"),
            "r1bqkbnr/ppp2ppp/2np4/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w - - 0 8",
        ),
        (
            Move::from_str("d7d5"),
            "r1bqkbnr/ppp2ppp/2n5/1B1pp3/4P3/5N2/PPPP1PPP/RNBQK2R w - d6 0 8",
        ),
        (
            Move::from_str("f7f6"),
            "r1bqkbnr/pppp2pp/2n2p2/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w - - 0 8",
        ),
        (
            Move::from_str("f7f5"),
            "r1bqkbnr/pppp2pp/2n5/1B2pp2/4P3/5N2/PPPP1PPP/RNBQK2R w - f6 0 8",
        ),
        (
            Move::from_str("g7g6"),
            "r1bqkbnr/pppp1p1p/2n3p1/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w - - 0 8",
        ),
        (
            Move::from_str("g7g5"),
            "r1bqkbnr/pppp1p1p/2n5/1B2p1p1/4P3/5N2/PPPP1PPP/RNBQK2R w - g6 0 8",
        ),
        (
            Move::from_str("h7h6"),
            "r1bqkbnr/pppp1pp1/2n4p/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w - - 0 8",
        ),
        (
            Move::from_str("h7h5"),
            "r1bqkbnr/pppp1pp1/2n5/1B2p2p/4P3/5N2/PPPP1PPP/RNBQK2R w - h6 0 8",
        ),
        (
            Move::from_str("a8b8"),
            "1rbqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w - - 12 8",
        ),
        (
            Move::from_str("d8e7"),
            "r1b1kbnr/ppppqppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w - - 12 8",
        ),
        (
            Move::from_str("d8f6"),
            "r1b1kbnr/pppp1ppp/2n2q2/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w - - 12 8",
        ),
        (
            Move::from_str("d8g5"),
            "r1b1kbnr/pppp1ppp/2n5/1B2p1q1/4P3/5N2/PPPP1PPP/RNBQK2R w - - 12 8",
        ),
        (
            Move::from_str("d8h4"),
            "r1b1kbnr/pppp1ppp/2n5/1B2p3/4P2q/5N2/PPPP1PPP/RNBQK2R w - - 12 8",
        ),
        (
            Move::from_str("e8e7"),
            "r1bq1bnr/ppppkppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w - - 12 8",
        ),
        (
            Move::from_str("f8e7"),
            "r1bqk1nr/ppppbppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w - - 12 8",
        ),
        (
            Move::from_str("f8d6"),
            "r1bqk1nr/pppp1ppp/2nb4/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w - - 12 8",
        ),
        (
            Move::from_str("f8c5"),
            "r1bqk1nr/pppp1ppp/2n5/1Bb1p3/4P3/5N2/PPPP1PPP/RNBQK2R w - - 12 8",
        ),
        (
            Move::from_str("f8b4"),
            "r1bqk1nr/pppp1ppp/2n5/1B2p3/1b2P3/5N2/PPPP1PPP/RNBQK2R w - - 12 8",
        ),
        (
            Move::from_str("f8a3"),
            "r1bqk1nr/pppp1ppp/2n5/1B2p3/4P3/b4N2/PPPP1PPP/RNBQK2R w - - 12 8",
        ),
        (
            Move::from_str("g8f6"),
            "r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w - - 12 8",
        ),
        (
            Move::from_str("g8h6"),
            "r1bqkb1r/pppp1ppp/2n4n/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w - - 12 8",
        ),
        (
            Move::from_str("g8e7"),
            "r1bqkb1r/ppppnppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w - - 12 8",
        ),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    // assert!(missing_moves.count() == 0);

    for m in missing_moves {
        println!("{m}");
    }
    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(standard);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
