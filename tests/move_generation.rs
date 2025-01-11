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
fn game_of_the_century() {
    let position = "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP4/Q1n2N2/P4PPP/3R1K1R w - - 4 18";
    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();

    let expected = vec![
        (
            Move::from_str("d1c1"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP4/Q1n2N2/P4PPP/2R2K1R b - - 5 18",
        ),
        (
            Move::from_str("d1b1"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP4/Q1n2N2/P4PPP/1R3K1R b - - 5 18",
        ),
        (
            Move::from_str("d1a1"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP4/Q1n2N2/P4PPP/R4K1R b - - 5 18",
        ),
        (
            Move::from_str("d1e1"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP4/Q1n2N2/P4PPP/4RK1R b - - 5 18",
        ),
        (
            Move::from_str("d1d2"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP4/Q1n2N2/P2R1PPP/5K1R b - - 5 18",
        ),
        (
            Move::from_str("d1d3"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP4/Q1nR1N2/P4PPP/5K1R b - - 5 18",
        ),
        (
            Move::from_str("f1e1"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP4/Q1n2N2/P4PPP/3RK2R b - - 5 18",
        ),
        (
            Move::from_str("f1g1"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP4/Q1n2N2/P4PPP/3R2KR b - - 5 18",
        ),
        (
            Move::from_str("h1g1"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP4/Q1n2N2/P4PPP/3R1KR1 b - - 5 18",
        ),
        (
            Move::from_str("g2g3"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP4/Q1n2NP1/P4P1P/3R1K1R b - - 0 18",
        ),
        (
            Move::from_str("g2g4"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP2P1/Q1n2N2/P4P1P/3R1K1R b - g3 0 18",
        ),
        (
            Move::from_str("h2h3"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP4/Q1n2N1P/P4PP1/3R1K1R b - - 0 18",
        ),
        (
            Move::from_str("h2h4"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP3P/Q1n2N2/P4PP1/3R1K1R b - h3 0 18",
        ),
        (
            Move::from_str("a3b2"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP4/2n2N2/PQ3PPP/3R1K1R b - - 5 18",
        ),
        (
            Move::from_str("a3c1"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP4/2n2N2/P4PPP/2QR1K1R b - - 5 18",
        ),
        (
            Move::from_str("a3b3"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP4/1Qn2N2/P4PPP/3R1K1R b - - 5 18",
        ),
        (
            Move::from_str("a3c3"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP4/2Q2N2/P4PPP/3R1K1R b - - 0 18",
        ),
        (
            Move::from_str("a3a4"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/Q1BP4/2n2N2/P4PPP/3R1K1R b - - 5 18",
        ),
        (
            Move::from_str("a3a5"),
            "r3r1k1/pp3pbp/1qp1b1p1/Q1B5/2BP4/2n2N2/P4PPP/3R1K1R b - - 5 18",
        ),
        (
            Move::from_str("a3a6"),
            "r3r1k1/pp3pbp/Qqp1b1p1/2B5/2BP4/2n2N2/P4PPP/3R1K1R b - - 5 18",
        ),
        (
            Move::from_str("a3a7"),
            "r3r1k1/Qp3pbp/1qp1b1p1/2B5/2BP4/2n2N2/P4PPP/3R1K1R b - - 0 18",
        ),
        (
            Move::from_str("a3b4"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/1QBP4/2n2N2/P4PPP/3R1K1R b - - 5 18",
        ),
        (
            Move::from_str("f3e1"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP4/Q1n5/P4PPP/3RNK1R b - - 5 18",
        ),
        (
            Move::from_str("f3g1"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP4/Q1n5/P4PPP/3R1KNR b - - 5 18",
        ),
        (
            Move::from_str("f3d2"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP4/Q1n5/P2N1PPP/3R1K1R b - - 5 18",
        ),
        (
            Move::from_str("f3h4"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/2BP3N/Q1n5/P4PPP/3R1K1R b - - 5 18",
        ),
        (
            Move::from_str("f3e5"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B1N3/2BP4/Q1n5/P4PPP/3R1K1R b - - 5 18",
        ),
        (
            Move::from_str("f3g5"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B3N1/2BP4/Q1n5/P4PPP/3R1K1R b - - 5 18",
        ),
        (
            Move::from_str("c4b3"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/3P4/QBn2N2/P4PPP/3R1K1R b - - 5 18",
        ),
        (
            Move::from_str("c4d3"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/3P4/Q1nB1N2/P4PPP/3R1K1R b - - 5 18",
        ),
        (
            Move::from_str("c4e2"),
            "r3r1k1/pp3pbp/1qp1b1p1/2B5/3P4/Q1n2N2/P3BPPP/3R1K1R b - - 5 18",
        ),
        (
            Move::from_str("c4b5"),
            "r3r1k1/pp3pbp/1qp1b1p1/1BB5/3P4/Q1n2N2/P4PPP/3R1K1R b - - 5 18",
        ),
        (
            Move::from_str("c4a6"),
            "r3r1k1/pp3pbp/Bqp1b1p1/2B5/3P4/Q1n2N2/P4PPP/3R1K1R b - - 5 18",
        ),
        (
            Move::from_str("c4d5"),
            "r3r1k1/pp3pbp/1qp1b1p1/2BB4/3P4/Q1n2N2/P4PPP/3R1K1R b - - 5 18",
        ),
        (
            Move::from_str("c4e6"),
            "r3r1k1/pp3pbp/1qp1B1p1/2B5/3P4/Q1n2N2/P4PPP/3R1K1R b - - 0 18",
        ),
        (
            Move::from_str("d4d5"),
            "r3r1k1/pp3pbp/1qp1b1p1/2BP4/2B5/Q1n2N2/P4PPP/3R1K1R b - - 0 18",
        ),
        (
            Move::from_str("c5b4"),
            "r3r1k1/pp3pbp/1qp1b1p1/8/1BBP4/Q1n2N2/P4PPP/3R1K1R b - - 5 18",
        ),
        (
            Move::from_str("c5b6"),
            "r3r1k1/pp3pbp/1Bp1b1p1/8/2BP4/Q1n2N2/P4PPP/3R1K1R b - - 0 18",
        ),
        (
            Move::from_str("c5d6"),
            "r3r1k1/pp3pbp/1qpBb1p1/8/2BP4/Q1n2N2/P4PPP/3R1K1R b - - 5 18",
        ),
        (
            Move::from_str("c5e7"),
            "r3r1k1/pp2Bpbp/1qp1b1p1/8/2BP4/Q1n2N2/P4PPP/3R1K1R b - - 5 18",
        ),
        (
            Move::from_str("c5f8"),
            "r3rBk1/pp3pbp/1qp1b1p1/8/2BP4/Q1n2N2/P4PPP/3R1K1R b - - 5 18",
        ),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    for m in extra_moves.clone() {
        println!("{m}");
    }
    assert!(extra_moves.count() == 0);

    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}

#[test]
fn gold_coins() {
    let position = "5rk1/pp4pp/4p3/2R3Q1/3n4/2q4r/P1P2PPP/5RK1 b - - 0 1";
    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();

    let expected = vec![
        (
            Move::from_str("c3b2"),
            "5rk1/pp4pp/4p3/2R3Q1/3n4/7r/PqP2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("c3a1"),
            "5rk1/pp4pp/4p3/2R3Q1/3n4/7r/P1P2PPP/q4RK1 w - - 1 2",
        ),
        (
            Move::from_str("c3c2"),
            "5rk1/pp4pp/4p3/2R3Q1/3n4/7r/P1q2PPP/5RK1 w - - 0 2",
        ),
        (
            Move::from_str("c3d2"),
            "5rk1/pp4pp/4p3/2R3Q1/3n4/7r/P1Pq1PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("c3e1"),
            "5rk1/pp4pp/4p3/2R3Q1/3n4/7r/P1P2PPP/4qRK1 w - - 1 2",
        ),
        (
            Move::from_str("c3b3"),
            "5rk1/pp4pp/4p3/2R3Q1/3n4/1q5r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("c3a3"),
            "5rk1/pp4pp/4p3/2R3Q1/3n4/q6r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("c3d3"),
            "5rk1/pp4pp/4p3/2R3Q1/3n4/3q3r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("c3e3"),
            "5rk1/pp4pp/4p3/2R3Q1/3n4/4q2r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("c3f3"),
            "5rk1/pp4pp/4p3/2R3Q1/3n4/5q1r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("c3g3"),
            "5rk1/pp4pp/4p3/2R3Q1/3n4/6qr/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("c3b4"),
            "5rk1/pp4pp/4p3/2R3Q1/1q1n4/7r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("c3a5"),
            "5rk1/pp4pp/4p3/q1R3Q1/3n4/7r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("c3c4"),
            "5rk1/pp4pp/4p3/2R3Q1/2qn4/7r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("c3c5"),
            "5rk1/pp4pp/4p3/2q3Q1/3n4/7r/P1P2PPP/5RK1 w - - 0 2",
        ),
        (
            Move::from_str("h3h2"),
            "5rk1/pp4pp/4p3/2R3Q1/3n4/2q5/P1P2PPr/5RK1 w - - 0 2",
        ),
        (
            Move::from_str("h3g3"),
            "5rk1/pp4pp/4p3/2R3Q1/3n4/2q3r1/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("h3f3"),
            "5rk1/pp4pp/4p3/2R3Q1/3n4/2q2r2/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("h3e3"),
            "5rk1/pp4pp/4p3/2R3Q1/3n4/2q1r3/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("h3d3"),
            "5rk1/pp4pp/4p3/2R3Q1/3n4/2qr4/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("h3h4"),
            "5rk1/pp4pp/4p3/2R3Q1/3n3r/2q5/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("h3h5"),
            "5rk1/pp4pp/4p3/2R3Qr/3n4/2q5/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("h3h6"),
            "5rk1/pp4pp/4p2r/2R3Q1/3n4/2q5/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("d4c2"),
            "5rk1/pp4pp/4p3/2R3Q1/8/2q4r/P1n2PPP/5RK1 w - - 0 2",
        ),
        (
            Move::from_str("d4e2"),
            "5rk1/pp4pp/4p3/2R3Q1/8/2q4r/P1P1nPPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("d4b3"),
            "5rk1/pp4pp/4p3/2R3Q1/8/1nq4r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("d4f3"),
            "5rk1/pp4pp/4p3/2R3Q1/8/2q2n1r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("d4b5"),
            "5rk1/pp4pp/4p3/1nR3Q1/8/2q4r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("d4f5"),
            "5rk1/pp4pp/4p3/2R2nQ1/8/2q4r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("d4c6"),
            "5rk1/pp4pp/2n1p3/2R3Q1/8/2q4r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("e6e5"),
            "5rk1/pp4pp/8/2R1p1Q1/3n4/2q4r/P1P2PPP/5RK1 w - - 0 2",
        ),
        (
            Move::from_str("a7a6"),
            "5rk1/1p4pp/p3p3/2R3Q1/3n4/2q4r/P1P2PPP/5RK1 w - - 0 2",
        ),
        (
            Move::from_str("a7a5"),
            "5rk1/1p4pp/4p3/p1R3Q1/3n4/2q4r/P1P2PPP/5RK1 w - a6 0 2",
        ),
        (
            Move::from_str("b7b6"),
            "5rk1/p5pp/1p2p3/2R3Q1/3n4/2q4r/P1P2PPP/5RK1 w - - 0 2",
        ),
        (
            Move::from_str("b7b5"),
            "5rk1/p5pp/4p3/1pR3Q1/3n4/2q4r/P1P2PPP/5RK1 w - b6 0 2",
        ),
        (
            Move::from_str("g7g6"),
            "5rk1/pp5p/4p1p1/2R3Q1/3n4/2q4r/P1P2PPP/5RK1 w - - 0 2",
        ),
        (
            Move::from_str("h7h6"),
            "5rk1/pp4p1/4p2p/2R3Q1/3n4/2q4r/P1P2PPP/5RK1 w - - 0 2",
        ),
        (
            Move::from_str("h7h5"),
            "5rk1/pp4p1/4p3/2R3Qp/3n4/2q4r/P1P2PPP/5RK1 w - h6 0 2",
        ),
        (
            Move::from_str("f8f7"),
            "6k1/pp3rpp/4p3/2R3Q1/3n4/2q4r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("f8f6"),
            "6k1/pp4pp/4pr2/2R3Q1/3n4/2q4r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("f8f5"),
            "6k1/pp4pp/4p3/2R2rQ1/3n4/2q4r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("f8f4"),
            "6k1/pp4pp/4p3/2R3Q1/3n1r2/2q4r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("f8f3"),
            "6k1/pp4pp/4p3/2R3Q1/3n4/2q2r1r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("f8f2"),
            "6k1/pp4pp/4p3/2R3Q1/3n4/2q4r/P1P2rPP/5RK1 w - - 0 2",
        ),
        (
            Move::from_str("f8e8"),
            "4r1k1/pp4pp/4p3/2R3Q1/3n4/2q4r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("f8d8"),
            "3r2k1/pp4pp/4p3/2R3Q1/3n4/2q4r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("f8c8"),
            "2r3k1/pp4pp/4p3/2R3Q1/3n4/2q4r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("f8b8"),
            "1r4k1/pp4pp/4p3/2R3Q1/3n4/2q4r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("f8a8"),
            "r5k1/pp4pp/4p3/2R3Q1/3n4/2q4r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("g8f7"),
            "5r2/pp3kpp/4p3/2R3Q1/3n4/2q4r/P1P2PPP/5RK1 w - - 1 2",
        ),
        (
            Move::from_str("g8h8"),
            "5r1k/pp4pp/4p3/2R3Q1/3n4/2q4r/P1P2PPP/5RK1 w - - 1 2",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}

#[test]
fn en_pessant_1() {
    let position = "7k/8/8/8/pPp5/8/8/7K b - b3 0 1";
    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();

    let expected = vec![
        (Move::from_str("a4a3"), "7k/8/8/8/1Pp5/p7/8/7K w - - 0 2"),
        (Move::from_str("a4b3"), "7k/8/8/8/2p5/1p6/8/7K w - - 0 2"),
        (Move::from_str("c4c3"), "7k/8/8/8/pP6/2p5/8/7K w - - 0 2"),
        (Move::from_str("c4b3"), "7k/8/8/8/p7/1p6/8/7K w - - 0 2"),
        (Move::from_str("h8g7"), "8/6k1/8/8/pPp5/8/8/7K w - - 1 2"),
        (Move::from_str("h8h7"), "8/7k/8/8/pPp5/8/8/7K w - - 1 2"),
        (Move::from_str("h8g8"), "6k1/8/8/8/pPp5/8/8/7K w - - 1 2"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}

#[test]
fn en_pessant_2() {
    let position = "7k/8/8/PpP5/8/8/8/7K w - b6 0 1";
    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();

    let expected = vec![
        (Move::from_str("h1g1"), "7k/8/8/PpP5/8/8/8/6K1 b - - 1 1"),
        (Move::from_str("h1g2"), "7k/8/8/PpP5/8/8/6K1/8 b - - 1 1"),
        (Move::from_str("h1h2"), "7k/8/8/PpP5/8/8/7K/8 b - - 1 1"),
        (Move::from_str("a5a6"), "7k/8/P7/1pP5/8/8/8/7K b - - 0 1"),
        (Move::from_str("a5b6"), "7k/8/1P6/2P5/8/8/8/7K b - - 0 1"),
        (Move::from_str("c5c6"), "7k/8/2P5/Pp6/8/8/8/7K b - - 0 1"),
        (Move::from_str("c5b6"), "7k/8/1P6/P7/8/8/8/7K b - - 0 1"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}

#[test]
fn en_pessant_3() {
    let position = "8/8/4k3/8/2pPp3/8/B7/7K b - d3 0 1";
    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();

    let expected = vec![
        (Move::from_str("e4e3"), "8/8/4k3/8/2pP4/4p3/B7/7K w - - 0 2"),
        (Move::from_str("e4d3"), "8/8/4k3/8/2p5/3p4/B7/7K w - - 0 2"),
        (Move::from_str("e6d5"), "8/8/8/3k4/2pPp3/8/B7/7K w - - 1 2"),
        (Move::from_str("e6f5"), "8/8/8/5k2/2pPp3/8/B7/7K w - - 1 2"),
        (Move::from_str("e6d6"), "8/8/3k4/8/2pPp3/8/B7/7K w - - 1 2"),
        (Move::from_str("e6f6"), "8/8/5k2/8/2pPp3/8/B7/7K w - - 1 2"),
        (Move::from_str("e6d7"), "8/3k4/8/8/2pPp3/8/B7/7K w - - 1 2"),
        (Move::from_str("e6e7"), "8/4k3/8/8/2pPp3/8/B7/7K w - - 1 2"),
        (Move::from_str("e6f7"), "8/5k2/8/8/2pPp3/8/B7/7K w - - 1 2"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}

#[test]
fn en_pessant_4() {
    let position = "7k/b7/8/2PpP3/8/4K3/8/8 w - d6 0 1";
    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();

    let expected = vec![
        (Move::from_str("e3d2"), "7k/b7/8/2PpP3/8/8/3K4/8 b - - 1 1"),
        (Move::from_str("e3e2"), "7k/b7/8/2PpP3/8/8/4K3/8 b - - 1 1"),
        (Move::from_str("e3f2"), "7k/b7/8/2PpP3/8/8/5K2/8 b - - 1 1"),
        (Move::from_str("e3d3"), "7k/b7/8/2PpP3/8/3K4/8/8 b - - 1 1"),
        (Move::from_str("e3f3"), "7k/b7/8/2PpP3/8/5K2/8/8 b - - 1 1"),
        (Move::from_str("e3d4"), "7k/b7/8/2PpP3/3K4/8/8/8 b - - 1 1"),
        (Move::from_str("e3f4"), "7k/b7/8/2PpP3/5K2/8/8/8 b - - 1 1"),
        (Move::from_str("e5e6"), "7k/b7/4P3/2Pp4/8/4K3/8/8 b - - 0 1"),
        (Move::from_str("e5d6"), "7k/b7/3P4/2P5/8/4K3/8/8 b - - 0 1"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}

#[test]
fn checkmate_1() {
    let position = "1R3k2/2R5/8/8/8/1K6/8/8 b - - 0 1 ";
    assert!(Board::from_fen(position).generate_moves().len() == 0);
}

#[test]
fn checkmate_2() {
    let position = "8/8/1k6/8/8/8/2r5/1r3K2 w - - 0 1";
    assert!(Board::from_fen(position).generate_moves().len() == 0);
}

#[test]
fn checkmate_3() {
    let position = "8/6N1/3R4/6k1/5Pp1/1K2P3/8/4B1R1 b - f3 0 1";
    assert!(Board::from_fen(position).generate_moves().len() == 0);
}

#[test]
fn checkmate_4() {
    let position = "4b1r1/8/1k2p3/5pP1/6K1/3r4/6n1/8 w - f6 0 1";
    assert!(Board::from_fen(position).generate_moves().len() == 0);
}

#[test]
fn checkmate_5() {
    let position = "kr6/ppN5/8/8/8/8/2K5/8 b - - 0 1";
    assert!(Board::from_fen(position).generate_moves().len() == 0);
}

#[test]
fn checkmate_6() {
    let position = "8/2k5/8/8/8/8/PPn5/KR6 w - - 0 1";
    assert!(Board::from_fen(position).generate_moves().len() == 0);
}

#[test]
fn checkmate_7() {
    let position = "k1K5/p1N5/8/8/8/8/8/8 b - - 0 1";
    assert!(Board::from_fen(position).generate_moves().len() == 0);
}

#[test]
fn checkmate_8() {
    let position = "8/8/8/8/8/8/P1n5/K1k5 w - - 0 1";
    assert!(Board::from_fen(position).generate_moves().len() == 0);
}

#[test]
fn stalemate_1() {
    let position = "8/8/8/8/8/8/P1n5/K1k5 w - - 0 1";
    assert!(Board::from_fen(position).generate_moves().len() == 0);
}

#[test]
fn stalemate_2() {
    let position = "k7/1R6/2K5/8/8/8/8/8 b - - 0 1";
    assert!(Board::from_fen(position).generate_moves().len() == 0);
}
#[test]
fn stalemate_3() {
    let position = "8/8/8/8/8/2k5/1r6/K7 w - - 0 1";
    assert!(Board::from_fen(position).generate_moves().len() == 0);
}
#[test]
fn stalemate_4() {
    let position = "k7/8/2N5/8/8/2K5/1R6/8 b - - 0 1";
    assert!(Board::from_fen(position).generate_moves().len() == 0);
}
#[test]
fn stalemate_5() {
    let position = "8/1r6/2k5/8/8/2n5/8/K7 w - - 0 1";
    assert!(Board::from_fen(position).generate_moves().len() == 0);
}
#[test]
fn stalemate_6() {
    let position = "k7/2Q5/8/8/8/2K5/8/8 b - - 0 1";
    assert!(Board::from_fen(position).generate_moves().len() == 0);
}
#[test]
fn stalemate_7() {
    let position = "8/8/5R2/4k1P1/3R4/2K5/8/8 b - - 0 1";
    assert!(Board::from_fen(position).generate_moves().len() == 0);
}
#[test]
fn stalemate_8() {
    let position = "8/8/2k5/8/8/8/2q5/K7 w - - 0 1";
    assert!(Board::from_fen(position).generate_moves().len() == 0);
}
#[test]
fn stalemate_9() {
    let position = "8/8/2k5/3r4/4K1p1/5r2/8/8 w - - 0 1";
    assert!(Board::from_fen(position).generate_moves().len() == 0);
}
#[test]
fn stalemate_10() {
    let position = "5bnr/4p1pq/4Qpkr/7p/7P/4P3/PPPP1PP1/RNB1KBNR b KQ - 2 10";
    assert!(Board::from_fen(position).generate_moves().len() == 0);
}
#[test]
fn stalemate_11() {
    let position = "rnb1kbnr/pppp1pp1/4p3/7p/7P/4qPKR/4P1PQ/5BNR w kq - 2 10";
    assert!(Board::from_fen(position).generate_moves().len() == 0);
}
#[test]
fn stalemate_12() {
    let position = "8/8/R7/4k3/4Pp2/2P2P2/7B/1K6 b - e3 0 1";
    assert!(Board::from_fen(position).generate_moves().len() == 0);
}

#[test]
fn stalemate_13() {
    let position = "1k6/7b/2p2p2/4pP2/4K3/r7/8/8 w - e6 0 1";
    assert!(Board::from_fen(position).generate_moves().len() == 0);
}

// TODO: convert to long fen!
#[test]
fn castling_1() {
    let position = "8/4k3/8/8/8/8/r6r/R3K2R w KQ - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rb1"), "8/4k3/8/8/8/8/r6r/1R2K2R b K - 1 1"),
        (Move::from_str("Rc1"), "8/4k3/8/8/8/8/r6r/2R1K2R b K - 1 1"),
        (Move::from_str("Rd1"), "8/4k3/8/8/8/8/r6r/3RK2R b K - 1 1"),
        (Move::from_str("Rxa2"), "8/4k3/8/8/8/8/R6r/4K2R b K - 0 1"),
        (Move::from_str("Kd1"), "8/4k3/8/8/8/8/r6r/R2K3R b - - 1 1"),
        (Move::from_str("Kf1"), "8/4k3/8/8/8/8/r6r/R4K1R b - - 1 1"),
        (Move::from_str("O-O-O"), "8/4k3/8/8/8/8/r6r/2KR3R b - - 1 1"),
        (Move::from_str("O-O"), "8/4k3/8/8/8/8/r6r/R4RK1 b - - 1 1"),
        (Move::from_str("Rg1"), "8/4k3/8/8/8/8/r6r/R3K1R1 b Q - 1 1"),
        (Move::from_str("Rf1"), "8/4k3/8/8/8/8/r6r/R3KR2 b Q - 1 1"),
        (Move::from_str("Rxh2"), "8/4k3/8/8/8/8/r6R/R3K3 b Q - 0 1"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_2() {
    let position = "r3k2r/R6R/8/8/8/8/4K3/8 b kq - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rxa7"), "4k2r/r6R/8/8/8/8/4K3/8 w k - 0 2"),
        (Move::from_str("Rb8"), "1r2k2r/R6R/8/8/8/8/4K3/8 w k - 1 2"),
        (Move::from_str("Rc8"), "2r1k2r/R6R/8/8/8/8/4K3/8 w k - 1 2"),
        (Move::from_str("Rd8"), "3rk2r/R6R/8/8/8/8/4K3/8 w k - 1 2"),
        (Move::from_str("Kd8"), "r2k3r/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Kf8"), "r4k1r/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("O-O-O"), "2kr3r/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("O-O"), "r4rk1/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Rxh7"), "r3k3/R6r/8/8/8/8/4K3/8 w q - 0 2"),
        (Move::from_str("Rg8"), "r3k1r1/R6R/8/8/8/8/4K3/8 w q - 1 2"),
        (Move::from_str("Rf8"), "r3kr2/R6R/8/8/8/8/4K3/8 w q - 1 2"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_3() {
    let position = "8/4k3/8/8/8/8/r6r/R3K2R w Q - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rb1"), "8/4k3/8/8/8/8/r6r/1R2K2R b - - 1 1"),
        (Move::from_str("Rc1"), "8/4k3/8/8/8/8/r6r/2R1K2R b - - 1 1"),
        (Move::from_str("Rd1"), "8/4k3/8/8/8/8/r6r/3RK2R b - - 1 1"),
        (Move::from_str("Rxa2"), "8/4k3/8/8/8/8/R6r/4K2R b - - 0 1"),
        (Move::from_str("Kd1"), "8/4k3/8/8/8/8/r6r/R2K3R b - - 1 1"),
        (Move::from_str("Kf1"), "8/4k3/8/8/8/8/r6r/R4K1R b - - 1 1"),
        (Move::from_str("O-O-O"), "8/4k3/8/8/8/8/r6r/2KR3R b - - 1 1"),
        (Move::from_str("Rg1"), "8/4k3/8/8/8/8/r6r/R3K1R1 b Q - 1 1"),
        (Move::from_str("Rf1"), "8/4k3/8/8/8/8/r6r/R3KR2 b Q - 1 1"),
        (Move::from_str("Rxh2"), "8/4k3/8/8/8/8/r6R/R3K3 b Q - 0 1"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_4() {
    let position = "r3k2r/R6R/8/8/8/8/4K3/8 b q - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rxa7"), "4k2r/r6R/8/8/8/8/4K3/8 w - - 0 2"),
        (Move::from_str("Rb8"), "1r2k2r/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Rc8"), "2r1k2r/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Rd8"), "3rk2r/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Kd8"), "r2k3r/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Kf8"), "r4k1r/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("O-O-O"), "2kr3r/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Rxh7"), "r3k3/R6r/8/8/8/8/4K3/8 w q - 0 2"),
        (Move::from_str("Rg8"), "r3k1r1/R6R/8/8/8/8/4K3/8 w q - 1 2"),
        (Move::from_str("Rf8"), "r3kr2/R6R/8/8/8/8/4K3/8 w q - 1 2"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_5() {
    let position = "8/4k3/8/8/8/8/r6r/R3K2R w K - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rb1"), "8/4k3/8/8/8/8/r6r/1R2K2R b K - 1 1"),
        (Move::from_str("Rc1"), "8/4k3/8/8/8/8/r6r/2R1K2R b K - 1 1"),
        (Move::from_str("Rd1"), "8/4k3/8/8/8/8/r6r/3RK2R b K - 1 1"),
        (Move::from_str("Rxa2"), "8/4k3/8/8/8/8/R6r/4K2R b K - 0 1"),
        (Move::from_str("Kd1"), "8/4k3/8/8/8/8/r6r/R2K3R b - - 1 1"),
        (Move::from_str("Kf1"), "8/4k3/8/8/8/8/r6r/R4K1R b - - 1 1"),
        (Move::from_str("O-O"), "8/4k3/8/8/8/8/r6r/R4RK1 b - - 1 1"),
        (Move::from_str("Rg1"), "8/4k3/8/8/8/8/r6r/R3K1R1 b - - 1 1"),
        (Move::from_str("Rf1"), "8/4k3/8/8/8/8/r6r/R3KR2 b - - 1 1"),
        (Move::from_str("Rxh2"), "8/4k3/8/8/8/8/r6R/R3K3 b - - 0 1"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_6() {
    let position = "r3k2r/R6R/8/8/8/8/4K3/8 b k - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rxa7"), "4k2r/r6R/8/8/8/8/4K3/8 w k - 0 2"),
        (Move::from_str("Rb8"), "1r2k2r/R6R/8/8/8/8/4K3/8 w k - 1 2"),
        (Move::from_str("Rc8"), "2r1k2r/R6R/8/8/8/8/4K3/8 w k - 1 2"),
        (Move::from_str("Rd8"), "3rk2r/R6R/8/8/8/8/4K3/8 w k - 1 2"),
        (Move::from_str("Kd8"), "r2k3r/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Kf8"), "r4k1r/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("O-O"), "r4rk1/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Rxh7"), "r3k3/R6r/8/8/8/8/4K3/8 w - - 0 2"),
        (Move::from_str("Rg8"), "r3k1r1/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Rf8"), "r3kr2/R6R/8/8/8/8/4K3/8 w - - 1 2"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_7() {
    let position = "8/4k3/8/8/8/8/r6r/R3K2R w - - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rb1"), "8/4k3/8/8/8/8/r6r/1R2K2R b - - 1 1"),
        (Move::from_str("Rc1"), "8/4k3/8/8/8/8/r6r/2R1K2R b - - 1 1"),
        (Move::from_str("Rd1"), "8/4k3/8/8/8/8/r6r/3RK2R b - - 1 1"),
        (Move::from_str("Rxa2"), "8/4k3/8/8/8/8/R6r/4K2R b - - 0 1"),
        (Move::from_str("Kd1"), "8/4k3/8/8/8/8/r6r/R2K3R b - - 1 1"),
        (Move::from_str("Kf1"), "8/4k3/8/8/8/8/r6r/R4K1R b - - 1 1"),
        (Move::from_str("Rg1"), "8/4k3/8/8/8/8/r6r/R3K1R1 b - - 1 1"),
        (Move::from_str("Rf1"), "8/4k3/8/8/8/8/r6r/R3KR2 b - - 1 1"),
        (Move::from_str("Rxh2"), "8/4k3/8/8/8/8/r6R/R3K3 b - - 0 1"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_8() {
    let position = "r3k2r/R6R/8/8/8/8/4K3/8 b - - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rxa7"), "4k2r/r6R/8/8/8/8/4K3/8 w - - 0 2"),
        (Move::from_str("Rb8"), "1r2k2r/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Rc8"), "2r1k2r/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Rd8"), "3rk2r/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Kd8"), "r2k3r/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Kf8"), "r4k1r/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Rxh7"), "r3k3/R6r/8/8/8/8/4K3/8 w - - 0 2"),
        (Move::from_str("Rg8"), "r3k1r1/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Rf8"), "r3kr2/R6R/8/8/8/8/4K3/8 w - - 1 2"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_9() {
    let position = "1r6/4k3/8/8/8/8/r7/R3K3 w Q - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rb1"), "1r6/4k3/8/8/8/8/r7/1R2K3 b - - 1 1"),
        (Move::from_str("Rc1"), "1r6/4k3/8/8/8/8/r7/2R1K3 b - - 1 1"),
        (Move::from_str("Rd1"), "1r6/4k3/8/8/8/8/r7/3RK3 b - - 1 1"),
        (Move::from_str("Rxa2"), "1r6/4k3/8/8/8/8/R7/4K3 b - - 0 1"),
        (Move::from_str("Kd1"), "1r6/4k3/8/8/8/8/r7/R2K4 b - - 1 1"),
        (Move::from_str("Kf1"), "1r6/4k3/8/8/8/8/r7/R4K2 b - - 1 1"),
        (Move::from_str("O-O-O"), "1r6/4k3/8/8/8/8/r7/2KR4 b - - 1 1"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_10() {
    let position = "r3k3/R7/8/8/8/8/4K3/1R6 b q - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rxa7"), "4k3/r7/8/8/8/8/4K3/1R6 w - - 0 2"),
        (Move::from_str("Rb8"), "1r2k3/R7/8/8/8/8/4K3/1R6 w - - 1 2"),
        (Move::from_str("Rc8"), "2r1k3/R7/8/8/8/8/4K3/1R6 w - - 1 2"),
        (Move::from_str("Rd8"), "3rk3/R7/8/8/8/8/4K3/1R6 w - - 1 2"),
        (Move::from_str("Kd8"), "r2k4/R7/8/8/8/8/4K3/1R6 w - - 1 2"),
        (Move::from_str("Kf8"), "r4k2/R7/8/8/8/8/4K3/1R6 w - - 1 2"),
        (Move::from_str("O-O-O"), "2kr4/R7/8/8/8/8/4K3/1R6 w - - 1 2"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_11() {
    let position = "2r5/4k3/8/8/8/8/r7/R3K3 w Q - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rb1"), "2r5/4k3/8/8/8/8/r7/1R2K3 b - - 1 1"),
        (Move::from_str("Rc1"), "2r5/4k3/8/8/8/8/r7/2R1K3 b - - 1 1"),
        (Move::from_str("Rd1"), "2r5/4k3/8/8/8/8/r7/3RK3 b - - 1 1"),
        (Move::from_str("Rxa2"), "2r5/4k3/8/8/8/8/R7/4K3 b - - 0 1"),
        (Move::from_str("Kd1"), "2r5/4k3/8/8/8/8/r7/R2K4 b - - 1 1"),
        (Move::from_str("Kf1"), "2r5/4k3/8/8/8/8/r7/R4K2 b - - 1 1"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_12() {
    let position = "r3k3/R7/8/8/8/8/4K3/2R5 b q - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rxa7"), "4k3/r7/8/8/8/8/4K3/2R5 w - - 0 2"),
        (Move::from_str("Rb8"), "1r2k3/R7/8/8/8/8/4K3/2R5 w - - 1 2"),
        (Move::from_str("Rc8"), "2r1k3/R7/8/8/8/8/4K3/2R5 w - - 1 2"),
        (Move::from_str("Rd8"), "3rk3/R7/8/8/8/8/4K3/2R5 w - - 1 2"),
        (Move::from_str("Kd8"), "r2k4/R7/8/8/8/8/4K3/2R5 w - - 1 2"),
        (Move::from_str("Kf8"), "r4k2/R7/8/8/8/8/4K3/2R5 w - - 1 2"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_13() {
    let position = "8/4k3/8/8/8/b7/r7/R3K3 w Q - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rb1"), "8/4k3/8/8/8/b7/r7/1R2K3 b - - 1 1"),
        (Move::from_str("Rc1"), "8/4k3/8/8/8/b7/r7/2R1K3 b - - 1 1"),
        (Move::from_str("Rd1"), "8/4k3/8/8/8/b7/r7/3RK3 b - - 1 1"),
        (Move::from_str("Rxa2"), "8/4k3/8/8/8/b7/R7/4K3 b - - 0 1"),
        (Move::from_str("Kd1"), "8/4k3/8/8/8/b7/r7/R2K4 b - - 1 1"),
        (Move::from_str("Kf1"), "8/4k3/8/8/8/b7/r7/R4K2 b - - 1 1"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_14() {
    let position = "r3k3/R7/B7/8/8/8/4K3/8 b q - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rxa7"), "4k3/r7/B7/8/8/8/4K3/8 w - - 0 2"),
        (Move::from_str("Rb8"), "1r2k3/R7/B7/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Rc8"), "2r1k3/R7/B7/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Rd8"), "3rk3/R7/B7/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Kd8"), "r2k4/R7/B7/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Kf8"), "r4k2/R7/B7/8/8/8/4K3/8 w - - 1 2"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_15() {
    let position = "8/4k3/8/8/8/8/n7/R3K3 w Q - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rb1"), "8/4k3/8/8/8/8/n7/1R2K3 b - - 1 1"),
        (Move::from_str("Rc1"), "8/4k3/8/8/8/8/n7/2R1K3 b - - 1 1"),
        (Move::from_str("Rd1"), "8/4k3/8/8/8/8/n7/3RK3 b - - 1 1"),
        (Move::from_str("Rxa2"), "8/4k3/8/8/8/8/R7/4K3 b - - 0 1"),
        (Move::from_str("Kd1"), "8/4k3/8/8/8/8/n7/R2K4 b - - 1 1"),
        (Move::from_str("Kf1"), "8/4k3/8/8/8/8/n7/R4K2 b - - 1 1"),
        (Move::from_str("Kd2"), "8/4k3/8/8/8/8/n2K4/R7 b - - 1 1"),
        (Move::from_str("Ke2"), "8/4k3/8/8/8/8/n3K3/R7 b - - 1 1"),
        (Move::from_str("Kf2"), "8/4k3/8/8/8/8/n4K2/R7 b - - 1 1"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_16() {
    let position = "r3k3/N7/8/8/8/8/4K3/8 b q - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rxa7"), "4k3/r7/8/8/8/8/4K3/8 w - - 0 2"),
        (Move::from_str("Rb8"), "1r2k3/N7/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Rc8"), "2r1k3/N7/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Rd8"), "3rk3/N7/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Kd7"), "r7/N2k4/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Ke7"), "r7/N3k3/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Kf7"), "r7/N4k2/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Kd8"), "r2k4/N7/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Kf8"), "r4k2/N7/8/8/8/8/4K3/8 w - - 1 2"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_17() {
    let position = "8/4k3/8/8/5q2/8/r7/R3K3 w Q - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rb1"), "8/4k3/8/8/5q2/8/r7/1R2K3 b - - 1 1"),
        (Move::from_str("Rc1"), "8/4k3/8/8/5q2/8/r7/2R1K3 b - - 1 1"),
        (Move::from_str("Rd1"), "8/4k3/8/8/5q2/8/r7/3RK3 b - - 1 1"),
        (Move::from_str("Rxa2"), "8/4k3/8/8/5q2/8/R7/4K3 b - - 0 1"),
        (Move::from_str("Kd1"), "8/4k3/8/8/5q2/8/r7/R2K4 b - - 1 1"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_18() {
    let position = "r3k3/R7/8/5Q2/8/8/4K3/8 b q - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rxa7"), "4k3/r7/8/5Q2/8/8/4K3/8 w - - 0 2"),
        (Move::from_str("Rb8"), "1r2k3/R7/8/5Q2/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Rc8"), "2r1k3/R7/8/5Q2/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Rd8"), "3rk3/R7/8/5Q2/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("Kd8"), "r2k4/R7/8/5Q2/8/8/4K3/8 w - - 1 2"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_19() {
    let position = "8/8/8/8/8/8/rk6/R3K3 w Q - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rb1+"), "8/8/8/8/8/8/rk6/1R2K3 b - - 1 1"),
        (Move::from_str("Rc1"), "8/8/8/8/8/8/rk6/2R1K3 b - - 1 1"),
        (Move::from_str("Rd1"), "8/8/8/8/8/8/rk6/3RK3 b - - 1 1"),
        (Move::from_str("Rxa2+"), "8/8/8/8/8/8/Rk6/4K3 b - - 0 1"),
        (Move::from_str("Kd1"), "8/8/8/8/8/8/rk6/R2K4 b - - 1 1"),
        (Move::from_str("Kf1"), "8/8/8/8/8/8/rk6/R4K2 b - - 1 1"),
        (Move::from_str("Kd2"), "8/8/8/8/8/8/rk1K4/R7 b - - 1 1"),
        (Move::from_str("Ke2"), "8/8/8/8/8/8/rk2K3/R7 b - - 1 1"),
        (Move::from_str("Kf2"), "8/8/8/8/8/8/rk3K2/R7 b - - 1 1"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_20() {
    let position = "r3k3/RK6/8/8/8/8/8/8 b q - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rxa7+"), "4k3/rK6/8/8/8/8/8/8 w - - 0 2"),
        (Move::from_str("Rb8+"), "1r2k3/RK6/8/8/8/8/8/8 w - - 1 2"),
        (Move::from_str("Rc8"), "2r1k3/RK6/8/8/8/8/8/8 w - - 1 2"),
        (Move::from_str("Rd8"), "3rk3/RK6/8/8/8/8/8/8 w - - 1 2"),
        (Move::from_str("Kd7"), "r7/RK1k4/8/8/8/8/8/8 w - - 1 2"),
        (Move::from_str("Ke7"), "r7/RK2k3/8/8/8/8/8/8 w - - 1 2"),
        (Move::from_str("Kf7"), "r7/RK3k2/8/8/8/8/8/8 w - - 1 2"),
        (Move::from_str("Kd8"), "r2k4/RK6/8/8/8/8/8/8 w - - 1 2"),
        (Move::from_str("Kf8"), "r4k2/RK6/8/8/8/8/8/8 w - - 1 2"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_21() {
    let position = "8/8/8/8/3k4/8/rp6/R3K3 w Q - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rb1"), "8/8/8/8/3k4/8/rp6/1R2K3 b - - 1 1"),
        (Move::from_str("Rc1"), "8/8/8/8/3k4/8/rp6/2R1K3 b - - 1 1"),
        (Move::from_str("Rd1+"), "8/8/8/8/3k4/8/rp6/3RK3 b - - 1 1"),
        (Move::from_str("Rxa2"), "8/8/8/8/3k4/8/Rp6/4K3 b - - 0 1"),
        (Move::from_str("Kd1"), "8/8/8/8/3k4/8/rp6/R2K4 b - - 1 1"),
        (Move::from_str("Kf1"), "8/8/8/8/3k4/8/rp6/R4K2 b - - 1 1"),
        (Move::from_str("Kd2"), "8/8/8/8/3k4/8/rp1K4/R7 b - - 1 1"),
        (Move::from_str("Ke2"), "8/8/8/8/3k4/8/rp2K3/R7 b - - 1 1"),
        (Move::from_str("Kf2"), "8/8/8/8/3k4/8/rp3K2/R7 b - - 1 1"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_22() {
    let position = "r3k3/RP6/8/3K4/8/8/8/8 b q - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rxa7"), "4k3/rP6/8/3K4/8/8/8/8 w - - 0 2"),
        (Move::from_str("Rb8"), "1r2k3/RP6/8/3K4/8/8/8/8 w - - 1 2"),
        (Move::from_str("Rc8"), "2r1k3/RP6/8/3K4/8/8/8/8 w - - 1 2"),
        (Move::from_str("Rd8+"), "3rk3/RP6/8/3K4/8/8/8/8 w - - 1 2"),
        (Move::from_str("Kd7"), "r7/RP1k4/8/3K4/8/8/8/8 w - - 1 2"),
        (Move::from_str("Ke7"), "r7/RP2k3/8/3K4/8/8/8/8 w - - 1 2"),
        (Move::from_str("Kf7"), "r7/RP3k2/8/3K4/8/8/8/8 w - - 1 2"),
        (Move::from_str("Kd8"), "r2k4/RP6/8/3K4/8/8/8/8 w - - 1 2"),
        (Move::from_str("Kf8"), "r4k2/RP6/8/3K4/8/8/8/8 w - - 1 2"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_23() {
    let position = "8/8/8/4r3/3k4/8/8/R3K2R w Q - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Kd1"), "8/8/8/4r3/3k4/8/8/R2K3R b - - 1 1"),
        (Move::from_str("Kf1"), "8/8/8/4r3/3k4/8/8/R4K1R b - - 1 1"),
        (Move::from_str("Kd2"), "8/8/8/4r3/3k4/8/3K4/R6R b - - 1 1"),
        (Move::from_str("Kf2"), "8/8/8/4r3/3k4/8/5K2/R6R b - - 1 1"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_24() {
    let position = "r3k2r/8/8/3K4/4R3/8/8/8 b q - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Kd7"), "r6r/3k4/8/3K4/4R3/8/8/8 w - - 1 2"),
        (Move::from_str("Kf7"), "r6r/5k2/8/3K4/4R3/8/8/8 w - - 1 2"),
        (Move::from_str("Kd8"), "r2k3r/8/8/3K4/4R3/8/8/8 w - - 1 2"),
        (Move::from_str("Kf8"), "r4k1r/8/8/3K4/4R3/8/8/8 w - - 1 2"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_25() {
    let position = "8/8/4k3/8/8/8/4p3/R3K2R w KQ - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rb1"), "8/8/4k3/8/8/8/4p3/1R2K2R b K - 1 1"),
        (Move::from_str("Rc1"), "8/8/4k3/8/8/8/4p3/2R1K2R b K - 1 1"),
        (Move::from_str("Rd1"), "8/8/4k3/8/8/8/4p3/3RK2R b K - 1 1"),
        (Move::from_str("Ra2"), "8/8/4k3/8/8/8/R3p3/4K2R b K - 1 1"),
        (Move::from_str("Ra3"), "8/8/4k3/8/8/R7/4p3/4K2R b K - 1 1"),
        (Move::from_str("Ra4"), "8/8/4k3/8/R7/8/4p3/4K2R b K - 1 1"),
        (Move::from_str("Ra5"), "8/8/4k3/R7/8/8/4p3/4K2R b K - 1 1"),
        (Move::from_str("Ra6+"), "8/8/R3k3/8/8/8/4p3/4K2R b K - 1 1"),
        (Move::from_str("Ra7"), "8/R7/4k3/8/8/8/4p3/4K2R b K - 1 1"),
        (Move::from_str("Ra8"), "R7/8/4k3/8/8/8/4p3/4K2R b K - 1 1"),
        (Move::from_str("Kd2"), "8/8/4k3/8/8/8/3Kp3/R6R b - - 1 1"),
        (Move::from_str("Kxe2"), "8/8/4k3/8/8/8/4K3/R6R b - - 0 1"),
        (Move::from_str("Kf2"), "8/8/4k3/8/8/8/4pK2/R6R b - - 1 1"),
        (Move::from_str("Rg1"), "8/8/4k3/8/8/8/4p3/R3K1R1 b Q - 1 1"),
        (Move::from_str("Rf1"), "8/8/4k3/8/8/8/4p3/R3KR2 b Q - 1 1"),
        (Move::from_str("Rh2"), "8/8/4k3/8/8/8/4p2R/R3K3 b Q - 1 1"),
        (Move::from_str("Rh3"), "8/8/4k3/8/8/7R/4p3/R3K3 b Q - 1 1"),
        (Move::from_str("Rh4"), "8/8/4k3/8/7R/8/4p3/R3K3 b Q - 1 1"),
        (Move::from_str("Rh5"), "8/8/4k3/7R/8/8/4p3/R3K3 b Q - 1 1"),
        (Move::from_str("Rh6+"), "8/8/4k2R/8/8/8/4p3/R3K3 b Q - 1 1"),
        (Move::from_str("Rh7"), "8/7R/4k3/8/8/8/4p3/R3K3 b Q - 1 1"),
        (Move::from_str("Rh8"), "7R/8/4k3/8/8/8/4p3/R3K3 b Q - 1 1"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_26() {
    let position = "r3k2r/4P3/8/8/8/4K3/8/8 b kq - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Ra7"), "4k2r/r3P3/8/8/8/4K3/8/8 w k - 1 2"),
        (Move::from_str("Ra6"), "4k2r/4P3/r7/8/8/4K3/8/8 w k - 1 2"),
        (Move::from_str("Ra5"), "4k2r/4P3/8/r7/8/4K3/8/8 w k - 1 2"),
        (Move::from_str("Ra4"), "4k2r/4P3/8/8/r7/4K3/8/8 w k - 1 2"),
        (Move::from_str("Ra3+"), "4k2r/4P3/8/8/8/r3K3/8/8 w k - 1 2"),
        (Move::from_str("Ra2"), "4k2r/4P3/8/8/8/4K3/r7/8 w k - 1 2"),
        (Move::from_str("Ra1"), "4k2r/4P3/8/8/8/4K3/8/r7 w k - 1 2"),
        (Move::from_str("Rb8"), "1r2k2r/4P3/8/8/8/4K3/8/8 w k - 1 2"),
        (Move::from_str("Rc8"), "2r1k2r/4P3/8/8/8/4K3/8/8 w k - 1 2"),
        (Move::from_str("Rd8"), "3rk2r/4P3/8/8/8/4K3/8/8 w k - 1 2"),
        (Move::from_str("Kd7"), "r6r/3kP3/8/8/8/4K3/8/8 w - - 1 2"),
        (Move::from_str("Kxe7"), "r6r/4k3/8/8/8/4K3/8/8 w - - 0 2"),
        (Move::from_str("Kf7"), "r6r/4Pk2/8/8/8/4K3/8/8 w - - 1 2"),
        (Move::from_str("Rh7"), "r3k3/4P2r/8/8/8/4K3/8/8 w q - 1 2"),
        (Move::from_str("Rh6"), "r3k3/4P3/7r/8/8/4K3/8/8 w q - 1 2"),
        (Move::from_str("Rh5"), "r3k3/4P3/8/7r/8/4K3/8/8 w q - 1 2"),
        (Move::from_str("Rh4"), "r3k3/4P3/8/8/7r/4K3/8/8 w q - 1 2"),
        (Move::from_str("Rh3+"), "r3k3/4P3/8/8/8/4K2r/8/8 w q - 1 2"),
        (Move::from_str("Rh2"), "r3k3/4P3/8/8/8/4K3/7r/8 w q - 1 2"),
        (Move::from_str("Rh1"), "r3k3/4P3/8/8/8/4K3/8/7r w q - 1 2"),
        (Move::from_str("Rg8"), "r3k1r1/4P3/8/8/8/4K3/8/8 w q - 1 2"),
        (Move::from_str("Rf8"), "r3kr2/4P3/8/8/8/4K3/8/8 w q - 1 2"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_27() {
    let position = "8/8/4k3/3bb3/8/8/8/R3K2R w KQ - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rb1"), "8/8/4k3/3bb3/8/8/8/1R2K2R b K - 1 1"),
        (Move::from_str("Rc1"), "8/8/4k3/3bb3/8/8/8/2R1K2R b K - 1 1"),
        (Move::from_str("Rd1"), "8/8/4k3/3bb3/8/8/8/3RK2R b K - 1 1"),
        (Move::from_str("Ra2"), "8/8/4k3/3bb3/8/8/R7/4K2R b K - 1 1"),
        (Move::from_str("Ra3"), "8/8/4k3/3bb3/8/R7/8/4K2R b K - 1 1"),
        (Move::from_str("Ra4"), "8/8/4k3/3bb3/R7/8/8/4K2R b K - 1 1"),
        (Move::from_str("Ra5"), "8/8/4k3/R2bb3/8/8/8/4K2R b K - 1 1"),
        (Move::from_str("Ra6+"), "8/8/R3k3/3bb3/8/8/8/4K2R b K - 1 1"),
        (Move::from_str("Ra7"), "8/R7/4k3/3bb3/8/8/8/4K2R b K - 1 1"),
        (Move::from_str("Ra8"), "R7/8/4k3/3bb3/8/8/8/4K2R b K - 1 1"),
        (Move::from_str("Kd1"), "8/8/4k3/3bb3/8/8/8/R2K3R b - - 1 1"),
        (Move::from_str("Kf1"), "8/8/4k3/3bb3/8/8/8/R4K1R b - - 1 1"),
        (Move::from_str("Kd2"), "8/8/4k3/3bb3/8/8/3K4/R6R b - - 1 1"),
        (Move::from_str("Ke2"), "8/8/4k3/3bb3/8/8/4K3/R6R b - - 1 1"),
        (Move::from_str("Kf2"), "8/8/4k3/3bb3/8/8/5K2/R6R b - - 1 1"),
        (
            Move::from_str("O-O-O"),
            "8/8/4k3/3bb3/8/8/8/2KR3R b - - 1 1",
        ),
        (Move::from_str("O-O"), "8/8/4k3/3bb3/8/8/8/R4RK1 b - - 1 1"),
        (Move::from_str("Rg1"), "8/8/4k3/3bb3/8/8/8/R3K1R1 b Q - 1 1"),
        (Move::from_str("Rf1"), "8/8/4k3/3bb3/8/8/8/R3KR2 b Q - 1 1"),
        (Move::from_str("Rh2"), "8/8/4k3/3bb3/8/8/7R/R3K3 b Q - 1 1"),
        (Move::from_str("Rh3"), "8/8/4k3/3bb3/8/7R/8/R3K3 b Q - 1 1"),
        (Move::from_str("Rh4"), "8/8/4k3/3bb3/7R/8/8/R3K3 b Q - 1 1"),
        (Move::from_str("Rh5"), "8/8/4k3/3bb2R/8/8/8/R3K3 b Q - 1 1"),
        (Move::from_str("Rh6+"), "8/8/4k2R/3bb3/8/8/8/R3K3 b Q - 1 1"),
        (Move::from_str("Rh7"), "8/7R/4k3/3bb3/8/8/8/R3K3 b Q - 1 1"),
        (Move::from_str("Rh8"), "7R/8/4k3/3bb3/8/8/8/R3K3 b Q - 1 1"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_28() {
    let position = "r3k2r/8/8/8/3BB3/4K3/8/8 b kq - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Ra7"), "4k2r/r7/8/8/3BB3/4K3/8/8 w k - 1 2"),
        (Move::from_str("Ra6"), "4k2r/8/r7/8/3BB3/4K3/8/8 w k - 1 2"),
        (Move::from_str("Ra5"), "4k2r/8/8/r7/3BB3/4K3/8/8 w k - 1 2"),
        (Move::from_str("Ra4"), "4k2r/8/8/8/r2BB3/4K3/8/8 w k - 1 2"),
        (Move::from_str("Ra3+"), "4k2r/8/8/8/3BB3/r3K3/8/8 w k - 1 2"),
        (Move::from_str("Ra2"), "4k2r/8/8/8/3BB3/4K3/r7/8 w k - 1 2"),
        (Move::from_str("Ra1"), "4k2r/8/8/8/3BB3/4K3/8/r7 w k - 1 2"),
        (Move::from_str("Rb8"), "1r2k2r/8/8/8/3BB3/4K3/8/8 w k - 1 2"),
        (Move::from_str("Rc8"), "2r1k2r/8/8/8/3BB3/4K3/8/8 w k - 1 2"),
        (Move::from_str("Rd8"), "3rk2r/8/8/8/3BB3/4K3/8/8 w k - 1 2"),
        (Move::from_str("Kd7"), "r6r/3k4/8/8/3BB3/4K3/8/8 w - - 1 2"),
        (Move::from_str("Ke7"), "r6r/4k3/8/8/3BB3/4K3/8/8 w - - 1 2"),
        (Move::from_str("Kf7"), "r6r/5k2/8/8/3BB3/4K3/8/8 w - - 1 2"),
        (Move::from_str("Kd8"), "r2k3r/8/8/8/3BB3/4K3/8/8 w - - 1 2"),
        (Move::from_str("Kf8"), "r4k1r/8/8/8/3BB3/4K3/8/8 w - - 1 2"),
        (
            Move::from_str("O-O-O"),
            "2kr3r/8/8/8/3BB3/4K3/8/8 w - - 1 2",
        ),
        (Move::from_str("O-O"), "r4rk1/8/8/8/3BB3/4K3/8/8 w - - 1 2"),
        (Move::from_str("Rh7"), "r3k3/7r/8/8/3BB3/4K3/8/8 w q - 1 2"),
        (Move::from_str("Rh6"), "r3k3/8/7r/8/3BB3/4K3/8/8 w q - 1 2"),
        (Move::from_str("Rh5"), "r3k3/8/8/7r/3BB3/4K3/8/8 w q - 1 2"),
        (Move::from_str("Rh4"), "r3k3/8/8/8/3BB2r/4K3/8/8 w q - 1 2"),
        (Move::from_str("Rh3+"), "r3k3/8/8/8/3BB3/4K2r/8/8 w q - 1 2"),
        (Move::from_str("Rh2"), "r3k3/8/8/8/3BB3/4K3/7r/8 w q - 1 2"),
        (Move::from_str("Rh1"), "r3k3/8/8/8/3BB3/4K3/8/7r w q - 1 2"),
        (Move::from_str("Rg8"), "r3k1r1/8/8/8/3BB3/4K3/8/8 w q - 1 2"),
        (Move::from_str("Rf8"), "r3kr2/8/8/8/3BB3/4K3/8/8 w q - 1 2"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_29() {
    let position = "8/8/4k3/8/8/8/2p3p1/R3K2R w KQ - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("Rb1"),
            "8/8/4k3/8/8/8/2p3p1/1R2K2R b K - 1 1",
        ),
        (
            Move::from_str("Rc1"),
            "8/8/4k3/8/8/8/2p3p1/2R1K2R b K - 1 1",
        ),
        (Move::from_str("Rd1"), "8/8/4k3/8/8/8/2p3p1/3RK2R b K - 1 1"),
        (Move::from_str("Ra2"), "8/8/4k3/8/8/8/R1p3p1/4K2R b K - 1 1"),
        (Move::from_str("Ra3"), "8/8/4k3/8/8/R7/2p3p1/4K2R b K - 1 1"),
        (Move::from_str("Ra4"), "8/8/4k3/8/R7/8/2p3p1/4K2R b K - 1 1"),
        (Move::from_str("Ra5"), "8/8/4k3/R7/8/8/2p3p1/4K2R b K - 1 1"),
        (
            Move::from_str("Ra6+"),
            "8/8/R3k3/8/8/8/2p3p1/4K2R b K - 1 1",
        ),
        (Move::from_str("Ra7"), "8/R7/4k3/8/8/8/2p3p1/4K2R b K - 1 1"),
        (Move::from_str("Ra8"), "R7/8/4k3/8/8/8/2p3p1/4K2R b K - 1 1"),
        (Move::from_str("Kd2"), "8/8/4k3/8/8/8/2pK2p1/R6R b - - 1 1"),
        (Move::from_str("Ke2"), "8/8/4k3/8/8/8/2p1K1p1/R6R b - - 1 1"),
        (Move::from_str("Kf2"), "8/8/4k3/8/8/8/2p2Kp1/R6R b - - 1 1"),
        (
            Move::from_str("Rg1"),
            "8/8/4k3/8/8/8/2p3p1/R3K1R1 b Q - 1 1",
        ),
        (Move::from_str("Rf1"), "8/8/4k3/8/8/8/2p3p1/R3KR2 b Q - 1 1"),
        (Move::from_str("Rh2"), "8/8/4k3/8/8/8/2p3pR/R3K3 b Q - 1 1"),
        (Move::from_str("Rh3"), "8/8/4k3/8/8/7R/2p3p1/R3K3 b Q - 1 1"),
        (Move::from_str("Rh4"), "8/8/4k3/8/7R/8/2p3p1/R3K3 b Q - 1 1"),
        (Move::from_str("Rh5"), "8/8/4k3/7R/8/8/2p3p1/R3K3 b Q - 1 1"),
        (
            Move::from_str("Rh6+"),
            "8/8/4k2R/8/8/8/2p3p1/R3K3 b Q - 1 1",
        ),
        (Move::from_str("Rh7"), "8/7R/4k3/8/8/8/2p3p1/R3K3 b Q - 1 1"),
        (Move::from_str("Rh8"), "7R/8/4k3/8/8/8/2p3p1/R3K3 b Q - 1 1"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_30() {
    let position = "r3k2r/2P3P1/8/8/8/4K3/8/8 b kq - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Ra7"), "4k2r/r1P3P1/8/8/8/4K3/8/8 w k - 1 2"),
        (Move::from_str("Ra6"), "4k2r/2P3P1/r7/8/8/4K3/8/8 w k - 1 2"),
        (Move::from_str("Ra5"), "4k2r/2P3P1/8/r7/8/4K3/8/8 w k - 1 2"),
        (Move::from_str("Ra4"), "4k2r/2P3P1/8/8/r7/4K3/8/8 w k - 1 2"),
        (
            Move::from_str("Ra3+"),
            "4k2r/2P3P1/8/8/8/r3K3/8/8 w k - 1 2",
        ),
        (Move::from_str("Ra2"), "4k2r/2P3P1/8/8/8/4K3/r7/8 w k - 1 2"),
        (Move::from_str("Ra1"), "4k2r/2P3P1/8/8/8/4K3/8/r7 w k - 1 2"),
        (
            Move::from_str("Rb8"),
            "1r2k2r/2P3P1/8/8/8/4K3/8/8 w k - 1 2",
        ),
        (
            Move::from_str("Rc8"),
            "2r1k2r/2P3P1/8/8/8/4K3/8/8 w k - 1 2",
        ),
        (Move::from_str("Rd8"), "3rk2r/2P3P1/8/8/8/4K3/8/8 w k - 1 2"),
        (Move::from_str("Kd7"), "r6r/2Pk2P1/8/8/8/4K3/8/8 w - - 1 2"),
        (Move::from_str("Ke7"), "r6r/2P1k1P1/8/8/8/4K3/8/8 w - - 1 2"),
        (Move::from_str("Kf7"), "r6r/2P2kP1/8/8/8/4K3/8/8 w - - 1 2"),
        (Move::from_str("Rh7"), "r3k3/2P3Pr/8/8/8/4K3/8/8 w q - 1 2"),
        (Move::from_str("Rh6"), "r3k3/2P3P1/7r/8/8/4K3/8/8 w q - 1 2"),
        (Move::from_str("Rh5"), "r3k3/2P3P1/8/7r/8/4K3/8/8 w q - 1 2"),
        (Move::from_str("Rh4"), "r3k3/2P3P1/8/8/7r/4K3/8/8 w q - 1 2"),
        (
            Move::from_str("Rh3+"),
            "r3k3/2P3P1/8/8/8/4K2r/8/8 w q - 1 2",
        ),
        (Move::from_str("Rh2"), "r3k3/2P3P1/8/8/8/4K3/7r/8 w q - 1 2"),
        (Move::from_str("Rh1"), "r3k3/2P3P1/8/8/8/4K3/8/7r w q - 1 2"),
        (
            Move::from_str("Rg8"),
            "r3k1r1/2P3P1/8/8/8/4K3/8/8 w q - 1 2",
        ),
        (Move::from_str("Rf8"), "r3kr2/2P3P1/8/8/8/4K3/8/8 w q - 1 2"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_31() {
    let position = "8/8/4k3/8/8/pr6/8/R3K3 w Q - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rb1"), "8/8/4k3/8/8/pr6/8/1R2K3 b - - 1 1"),
        (Move::from_str("Rc1"), "8/8/4k3/8/8/pr6/8/2R1K3 b - - 1 1"),
        (Move::from_str("Rd1"), "8/8/4k3/8/8/pr6/8/3RK3 b - - 1 1"),
        (Move::from_str("Ra2"), "8/8/4k3/8/8/pr6/R7/4K3 b - - 1 1"),
        (Move::from_str("Rxa3"), "8/8/4k3/8/8/Rr6/8/4K3 b - - 0 1"),
        (Move::from_str("Kd1"), "8/8/4k3/8/8/pr6/8/R2K4 b - - 1 1"),
        (Move::from_str("Kf1"), "8/8/4k3/8/8/pr6/8/R4K2 b - - 1 1"),
        (Move::from_str("Kd2"), "8/8/4k3/8/8/pr6/3K4/R7 b - - 1 1"),
        (Move::from_str("Ke2"), "8/8/4k3/8/8/pr6/4K3/R7 b - - 1 1"),
        (Move::from_str("Kf2"), "8/8/4k3/8/8/pr6/5K2/R7 b - - 1 1"),
        (Move::from_str("O-O-O"), "8/8/4k3/8/8/pr6/8/2KR4 b - - 1 1"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_32() {
    let position = "r3k3/8/PR6/8/8/4K3/8/8 b q - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Ra7"), "4k3/r7/PR6/8/8/4K3/8/8 w - - 1 2"),
        (Move::from_str("Rxa6"), "4k3/8/rR6/8/8/4K3/8/8 w - - 0 2"),
        (Move::from_str("Rb8"), "1r2k3/8/PR6/8/8/4K3/8/8 w - - 1 2"),
        (Move::from_str("Rc8"), "2r1k3/8/PR6/8/8/4K3/8/8 w - - 1 2"),
        (Move::from_str("Rd8"), "3rk3/8/PR6/8/8/4K3/8/8 w - - 1 2"),
        (Move::from_str("Kd7"), "r7/3k4/PR6/8/8/4K3/8/8 w - - 1 2"),
        (Move::from_str("Ke7"), "r7/4k3/PR6/8/8/4K3/8/8 w - - 1 2"),
        (Move::from_str("Kf7"), "r7/5k2/PR6/8/8/4K3/8/8 w - - 1 2"),
        (Move::from_str("Kd8"), "r2k4/8/PR6/8/8/4K3/8/8 w - - 1 2"),
        (Move::from_str("Kf8"), "r4k2/8/PR6/8/8/4K3/8/8 w - - 1 2"),
        (Move::from_str("O-O-O"), "2kr4/8/PR6/8/8/4K3/8/8 w - - 1 2"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_33() {
    let position = "r3k2r/R3P2R/8/8/8/8/8/4K3 b kq - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("Rxa7"), "4k2r/r3P2R/8/8/8/8/8/4K3 w k - 0 2"),
        (
            Move::from_str("Rb8"),
            "1r2k2r/R3P2R/8/8/8/8/8/4K3 w k - 1 2",
        ),
        (
            Move::from_str("Rc8"),
            "2r1k2r/R3P2R/8/8/8/8/8/4K3 w k - 1 2",
        ),
        (Move::from_str("Rd8"), "3rk2r/R3P2R/8/8/8/8/8/4K3 w k - 1 2"),
        (Move::from_str("Rxh7"), "r3k3/R3P2r/8/8/8/8/8/4K3 w q - 0 2"),
        (
            Move::from_str("Rg8"),
            "r3k1r1/R3P2R/8/8/8/8/8/4K3 w q - 1 2",
        ),
        (Move::from_str("Rf8"), "r3kr2/R3P2R/8/8/8/8/8/4K3 w q - 1 2"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_34() {
    let position = "4k3/8/8/8/8/8/r3p2r/R3K2R w KQ - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("Rb1"),
            "4k3/8/8/8/8/8/r3p2r/1R2K2R b K - 1 1",
        ),
        (
            Move::from_str("Rc1"),
            "4k3/8/8/8/8/8/r3p2r/2R1K2R b K - 1 1",
        ),
        (Move::from_str("Rd1"), "4k3/8/8/8/8/8/r3p2r/3RK2R b K - 1 1"),
        (Move::from_str("Rxa2"), "4k3/8/8/8/8/8/R3p2r/4K2R b K - 0 1"),
        (
            Move::from_str("Rg1"),
            "4k3/8/8/8/8/8/r3p2r/R3K1R1 b Q - 1 1",
        ),
        (Move::from_str("Rf1"), "4k3/8/8/8/8/8/r3p2r/R3KR2 b Q - 1 1"),
        (Move::from_str("Rxh2"), "4k3/8/8/8/8/8/r3p2R/R3K3 b Q - 0 1"),
    ];

    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_35() {
    let position = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2n w Q - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("Nd2"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPPNN1PP/R1BQK2n b Q - 2 8",
        ),
        (
            Move::from_str("Na3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/N7/PPP1N1PP/R1BQK2n b Q - 2 8",
        ),
        (
            Move::from_str("Nbc3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/2N5/PPP1N1PP/R1BQK2n b Q - 2 8",
        ),
        (
            Move::from_str("Bd2"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPPBN1PP/RN1QK2n b Q - 2 8",
        ),
        (
            Move::from_str("Be3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/4B3/PPP1N1PP/RN1QK2n b Q - 2 8",
        ),
        (
            Move::from_str("Bf4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B2B2/8/PPP1N1PP/RN1QK2n b Q - 2 8",
        ),
        (
            Move::from_str("Bg5"),
            "rnbq1k1r/pp1Pbppp/2p5/6B1/2B5/8/PPP1N1PP/RN1QK2n b Q - 2 8",
        ),
        (
            Move::from_str("Bh6"),
            "rnbq1k1r/pp1Pbppp/2p4B/8/2B5/8/PPP1N1PP/RN1QK2n b Q - 2 8",
        ),
        (
            Move::from_str("Qd2"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPPQN1PP/RNB1K2n b Q - 2 8",
        ),
        (
            Move::from_str("Qd3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/3Q4/PPP1N1PP/RNB1K2n b Q - 2 8",
        ),
        (
            Move::from_str("Qd4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2BQ4/8/PPP1N1PP/RNB1K2n b Q - 2 8",
        ),
        (
            Move::from_str("Qd5"),
            "rnbq1k1r/pp1Pbppp/2p5/3Q4/2B5/8/PPP1N1PP/RNB1K2n b Q - 2 8",
        ),
        (
            Move::from_str("Qd6"),
            "rnbq1k1r/pp1Pbppp/2pQ4/8/2B5/8/PPP1N1PP/RNB1K2n b Q - 2 8",
        ),
        (
            Move::from_str("Kf1"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1N1PP/RNBQ1K1n b - - 2 8",
        ),
        (
            Move::from_str("Kd2"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPPKN1PP/RNBQ3n b - - 2 8",
        ),
        (
            Move::from_str("a3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/P7/1PP1N1PP/RNBQK2n b Q - 0 8",
        ),
        (
            Move::from_str("a4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/P1B5/8/1PP1N1PP/RNBQK2n b Q a3 0 8",
        ),
        (
            Move::from_str("b3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/1P6/P1P1N1PP/RNBQK2n b Q - 0 8",
        ),
        (
            Move::from_str("b4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/1PB5/8/P1P1N1PP/RNBQK2n b Q b3 0 8",
        ),
        (
            Move::from_str("c3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/2P5/PP2N1PP/RNBQK2n b Q - 0 8",
        ),
        (
            Move::from_str("Ng1"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP3PP/RNBQK1Nn b Q - 2 8",
        ),
        (
            Move::from_str("Nec3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/2N5/PPP3PP/RNBQK2n b Q - 2 8",
        ),
        (
            Move::from_str("Ng3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/6N1/PPP3PP/RNBQK2n b Q - 2 8",
        ),
        (
            Move::from_str("Nd4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2BN4/8/PPP3PP/RNBQK2n b Q - 2 8",
        ),
        (
            Move::from_str("Nf4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B2N2/8/PPP3PP/RNBQK2n b Q - 2 8",
        ),
        (
            Move::from_str("g3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/6P1/PPP1N2P/RNBQK2n b Q - 0 8",
        ),
        (
            Move::from_str("g4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B3P1/8/PPP1N2P/RNBQK2n b Q g3 0 8",
        ),
        (
            Move::from_str("h3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/7P/PPP1N1P1/RNBQK2n b Q - 0 8",
        ),
        (
            Move::from_str("h4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B4P/8/PPP1N1P1/RNBQK2n b Q h3 0 8",
        ),
        (
            Move::from_str("Bb3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/8/1B6/PPP1N1PP/RNBQK2n b Q - 2 8",
        ),
        (
            Move::from_str("Bd3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/8/3B4/PPP1N1PP/RNBQK2n b Q - 2 8",
        ),
        (
            Move::from_str("Bb5"),
            "rnbq1k1r/pp1Pbppp/2p5/1B6/8/8/PPP1N1PP/RNBQK2n b Q - 2 8",
        ),
        (
            Move::from_str("Ba6"),
            "rnbq1k1r/pp1Pbppp/B1p5/8/8/8/PPP1N1PP/RNBQK2n b Q - 2 8",
        ),
        (
            Move::from_str("Bd5"),
            "rnbq1k1r/pp1Pbppp/2p5/3B4/8/8/PPP1N1PP/RNBQK2n b Q - 2 8",
        ),
        (
            Move::from_str("Be6"),
            "rnbq1k1r/pp1Pbppp/2p1B3/8/8/8/PPP1N1PP/RNBQK2n b Q - 2 8",
        ),
        (
            Move::from_str("Bxf7"),
            "rnbq1k1r/pp1PbBpp/2p5/8/8/8/PPP1N1PP/RNBQK2n b Q - 0 8",
        ),
        (
            Move::from_str("dxc8=Q"),
            "rnQq1k1r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2n b Q - 0 8",
        ),
        (
            Move::from_str("dxc8=R"),
            "rnRq1k1r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2n b Q - 0 8",
        ),
        (
            Move::from_str("dxc8=N"),
            "rnNq1k1r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2n b Q - 0 8",
        ),
        (
            Move::from_str("dxc8=B"),
            "rnBq1k1r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2n b Q - 0 8",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_36() {
    let position = "rnbqk2N/ppp1n1pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R b q - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("dxc1=Q"),
            "rnbqk2N/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/RNqQ1K1R w q - 0 9",
        ),
        (
            Move::from_str("dxc1=R"),
            "rnbqk2N/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/RNrQ1K1R w q - 0 9",
        ),
        (
            Move::from_str("dxc1=N"),
            "rnbqk2N/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/RNnQ1K1R w q - 0 9",
        ),
        (
            Move::from_str("dxc1=B"),
            "rnbqk2N/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/RNbQ1K1R w q - 0 9",
        ),
        (
            Move::from_str("Bb4"),
            "rnbqk2N/ppp1n1pp/8/8/1b6/2P5/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Ba3"),
            "rnbqk2N/ppp1n1pp/8/8/8/b1P5/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Bd4"),
            "rnbqk2N/ppp1n1pp/8/8/3b4/2P5/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Be3"),
            "rnbqk2N/ppp1n1pp/8/8/8/2P1b3/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Bxf2"),
            "rnbqk2N/ppp1n1pp/8/8/8/2P5/PP1pBbPP/RNBQ1K1R w q - 0 9",
        ),
        (
            Move::from_str("Bb6"),
            "rnbqk2N/ppp1n1pp/1b6/8/8/2P5/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Bd6"),
            "rnbqk2N/ppp1n1pp/3b4/8/8/2P5/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("a6"),
            "rnbqk2N/1pp1n1pp/p7/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w q - 0 9",
        ),
        (
            Move::from_str("a5"),
            "rnbqk2N/1pp1n1pp/8/p1b5/8/2P5/PP1pBPPP/RNBQ1K1R w q a6 0 9",
        ),
        (
            Move::from_str("b6"),
            "rnbqk2N/p1p1n1pp/1p6/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w q - 0 9",
        ),
        (
            Move::from_str("b5"),
            "rnbqk2N/p1p1n1pp/8/1pb5/8/2P5/PP1pBPPP/RNBQ1K1R w q b6 0 9",
        ),
        (
            Move::from_str("c6"),
            "rnbqk2N/pp2n1pp/2p5/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w q - 0 9",
        ),
        (
            Move::from_str("Nd5"),
            "rnbqk2N/ppp3pp/8/2bn4/8/2P5/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Nf5"),
            "rnbqk2N/ppp3pp/8/2b2n2/8/2P5/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Nec6"),
            "rnbqk2N/ppp3pp/2n5/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Ng6"),
            "rnbqk2N/ppp3pp/6n1/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Ng8"),
            "rnbqk1nN/ppp3pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("g6"),
            "rnbqk2N/ppp1n2p/6p1/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w q - 0 9",
        ),
        (
            Move::from_str("g5"),
            "rnbqk2N/ppp1n2p/8/2b3p1/8/2P5/PP1pBPPP/RNBQ1K1R w q g6 0 9",
        ),
        (
            Move::from_str("h6"),
            "rnbqk2N/ppp1n1p1/7p/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w q - 0 9",
        ),
        (
            Move::from_str("h5"),
            "rnbqk2N/ppp1n1p1/8/2b4p/8/2P5/PP1pBPPP/RNBQ1K1R w q h6 0 9",
        ),
        (
            Move::from_str("Na6"),
            "r1bqk2N/ppp1n1pp/n7/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Nbc6"),
            "r1bqk2N/ppp1n1pp/2n5/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Nd7"),
            "r1bqk2N/pppnn1pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Bd7"),
            "rn1qk2N/pppbn1pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Be6"),
            "rn1qk2N/ppp1n1pp/4b3/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Bf5"),
            "rn1qk2N/ppp1n1pp/8/2b2b2/8/2P5/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Bg4"),
            "rn1qk2N/ppp1n1pp/8/2b5/6b1/2P5/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Bh3"),
            "rn1qk2N/ppp1n1pp/8/2b5/8/2P4b/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Qd7"),
            "rnb1k2N/pppqn1pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Qd6"),
            "rnb1k2N/ppp1n1pp/3q4/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Qd5"),
            "rnb1k2N/ppp1n1pp/8/2bq4/8/2P5/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Qd4"),
            "rnb1k2N/ppp1n1pp/8/2b5/3q4/2P5/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Qd3"),
            "rnb1k2N/ppp1n1pp/8/2b5/8/2Pq4/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Kd7"),
            "rnbq3N/pppkn1pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w - - 2 9",
        ),
        (
            Move::from_str("Kf8"),
            "rnbq1k1N/ppp1n1pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w - - 2 9",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_37() {
    let position = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1N1PP/RNBQKn1R w KQ - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("Nd2"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPPNN1PP/R1BQKn1R b KQ - 2 8",
        ),
        (
            Move::from_str("Na3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/N7/PPP1N1PP/R1BQKn1R b KQ - 2 8",
        ),
        (
            Move::from_str("Nbc3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/2N5/PPP1N1PP/R1BQKn1R b KQ - 2 8",
        ),
        (
            Move::from_str("Bd2"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPPBN1PP/RN1QKn1R b KQ - 2 8",
        ),
        (
            Move::from_str("Be3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/4B3/PPP1N1PP/RN1QKn1R b KQ - 2 8",
        ),
        (
            Move::from_str("Bf4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B2B2/8/PPP1N1PP/RN1QKn1R b KQ - 2 8",
        ),
        (
            Move::from_str("Bg5"),
            "rnbq1k1r/pp1Pbppp/2p5/6B1/2B5/8/PPP1N1PP/RN1QKn1R b KQ - 2 8",
        ),
        (
            Move::from_str("Bh6"),
            "rnbq1k1r/pp1Pbppp/2p4B/8/2B5/8/PPP1N1PP/RN1QKn1R b KQ - 2 8",
        ),
        (
            Move::from_str("Qd2"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPPQN1PP/RNB1Kn1R b KQ - 2 8",
        ),
        (
            Move::from_str("Qd3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/3Q4/PPP1N1PP/RNB1Kn1R b KQ - 2 8",
        ),
        (
            Move::from_str("Qd4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2BQ4/8/PPP1N1PP/RNB1Kn1R b KQ - 2 8",
        ),
        (
            Move::from_str("Qd5"),
            "rnbq1k1r/pp1Pbppp/2p5/3Q4/2B5/8/PPP1N1PP/RNB1Kn1R b KQ - 2 8",
        ),
        (
            Move::from_str("Qd6"),
            "rnbq1k1r/pp1Pbppp/2pQ4/8/2B5/8/PPP1N1PP/RNB1Kn1R b KQ - 2 8",
        ),
        (
            Move::from_str("Kxf1"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1N1PP/RNBQ1K1R b - - 0 8",
        ),
        (
            Move::from_str("Kf2"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NKPP/RNBQ1n1R b - - 2 8",
        ),
        (
            Move::from_str("Rg1"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1N1PP/RNBQKnR1 b Q - 2 8",
        ),
        (
            Move::from_str("Rxf1"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1N1PP/RNBQKR2 b Q - 0 8",
        ),
        (
            Move::from_str("a3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/P7/1PP1N1PP/RNBQKn1R b KQ - 0 8",
        ),
        (
            Move::from_str("a4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/P1B5/8/1PP1N1PP/RNBQKn1R b KQ a3 0 8",
        ),
        (
            Move::from_str("b3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/1P6/P1P1N1PP/RNBQKn1R b KQ - 0 8",
        ),
        (
            Move::from_str("b4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/1PB5/8/P1P1N1PP/RNBQKn1R b KQ b3 0 8",
        ),
        (
            Move::from_str("c3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/2P5/PP2N1PP/RNBQKn1R b KQ - 0 8",
        ),
        (
            Move::from_str("Ng1"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP3PP/RNBQKnNR b KQ - 2 8",
        ),
        (
            Move::from_str("Nec3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/2N5/PPP3PP/RNBQKn1R b KQ - 2 8",
        ),
        (
            Move::from_str("Ng3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/6N1/PPP3PP/RNBQKn1R b KQ - 2 8",
        ),
        (
            Move::from_str("Nd4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2BN4/8/PPP3PP/RNBQKn1R b KQ - 2 8",
        ),
        (
            Move::from_str("Nf4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B2N2/8/PPP3PP/RNBQKn1R b KQ - 2 8",
        ),
        (
            Move::from_str("g3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/6P1/PPP1N2P/RNBQKn1R b KQ - 0 8",
        ),
        (
            Move::from_str("g4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B3P1/8/PPP1N2P/RNBQKn1R b KQ g3 0 8",
        ),
        (
            Move::from_str("h3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/7P/PPP1N1P1/RNBQKn1R b KQ - 0 8",
        ),
        (
            Move::from_str("h4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B4P/8/PPP1N1P1/RNBQKn1R b KQ h3 0 8",
        ),
        (
            Move::from_str("Bb3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/8/1B6/PPP1N1PP/RNBQKn1R b KQ - 2 8",
        ),
        (
            Move::from_str("Bd3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/8/3B4/PPP1N1PP/RNBQKn1R b KQ - 2 8",
        ),
        (
            Move::from_str("Bb5"),
            "rnbq1k1r/pp1Pbppp/2p5/1B6/8/8/PPP1N1PP/RNBQKn1R b KQ - 2 8",
        ),
        (
            Move::from_str("Ba6"),
            "rnbq1k1r/pp1Pbppp/B1p5/8/8/8/PPP1N1PP/RNBQKn1R b KQ - 2 8",
        ),
        (
            Move::from_str("Bd5"),
            "rnbq1k1r/pp1Pbppp/2p5/3B4/8/8/PPP1N1PP/RNBQKn1R b KQ - 2 8",
        ),
        (
            Move::from_str("Be6"),
            "rnbq1k1r/pp1Pbppp/2p1B3/8/8/8/PPP1N1PP/RNBQKn1R b KQ - 2 8",
        ),
        (
            Move::from_str("Bxf7"),
            "rnbq1k1r/pp1PbBpp/2p5/8/8/8/PPP1N1PP/RNBQKn1R b KQ - 0 8",
        ),
        (
            Move::from_str("dxc8=Q"),
            "rnQq1k1r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQKn1R b KQ - 0 8",
        ),
        (
            Move::from_str("dxc8=R"),
            "rnRq1k1r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQKn1R b KQ - 0 8",
        ),
        (
            Move::from_str("dxc8=N"),
            "rnNq1k1r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQKn1R b KQ - 0 8",
        ),
        (
            Move::from_str("dxc8=B"),
            "rnBq1k1r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQKn1R b KQ - 0 8",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_38() {
    let position = "rnbqkN1r/ppp1n1pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R b kq - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("dxc1=Q"),
            "rnbqkN1r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/RNqQ1K1R w kq - 0 9",
        ),
        (
            Move::from_str("dxc1=R"),
            "rnbqkN1r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/RNrQ1K1R w kq - 0 9",
        ),
        (
            Move::from_str("dxc1=N"),
            "rnbqkN1r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/RNnQ1K1R w kq - 0 9",
        ),
        (
            Move::from_str("dxc1=B"),
            "rnbqkN1r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/RNbQ1K1R w kq - 0 9",
        ),
        (
            Move::from_str("Bb4"),
            "rnbqkN1r/ppp1n1pp/8/8/1b6/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Ba3"),
            "rnbqkN1r/ppp1n1pp/8/8/8/b1P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Bd4"),
            "rnbqkN1r/ppp1n1pp/8/8/3b4/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Be3"),
            "rnbqkN1r/ppp1n1pp/8/8/8/2P1b3/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Bxf2"),
            "rnbqkN1r/ppp1n1pp/8/8/8/2P5/PP1pBbPP/RNBQ1K1R w kq - 0 9",
        ),
        (
            Move::from_str("Bb6"),
            "rnbqkN1r/ppp1n1pp/1b6/8/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Bd6"),
            "rnbqkN1r/ppp1n1pp/3b4/8/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("a6"),
            "rnbqkN1r/1pp1n1pp/p7/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 0 9",
        ),
        (
            Move::from_str("a5"),
            "rnbqkN1r/1pp1n1pp/8/p1b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq a6 0 9",
        ),
        (
            Move::from_str("b6"),
            "rnbqkN1r/p1p1n1pp/1p6/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 0 9",
        ),
        (
            Move::from_str("b5"),
            "rnbqkN1r/p1p1n1pp/8/1pb5/8/2P5/PP1pBPPP/RNBQ1K1R w kq b6 0 9",
        ),
        (
            Move::from_str("c6"),
            "rnbqkN1r/pp2n1pp/2p5/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 0 9",
        ),
        (
            Move::from_str("Nd5"),
            "rnbqkN1r/ppp3pp/8/2bn4/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Nf5"),
            "rnbqkN1r/ppp3pp/8/2b2n2/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Nec6"),
            "rnbqkN1r/ppp3pp/2n5/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Ng6"),
            "rnbqkN1r/ppp3pp/6n1/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Ng8"),
            "rnbqkNnr/ppp3pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("g6"),
            "rnbqkN1r/ppp1n2p/6p1/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 0 9",
        ),
        (
            Move::from_str("g5"),
            "rnbqkN1r/ppp1n2p/8/2b3p1/8/2P5/PP1pBPPP/RNBQ1K1R w kq g6 0 9",
        ),
        (
            Move::from_str("h6"),
            "rnbqkN1r/ppp1n1p1/7p/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 0 9",
        ),
        (
            Move::from_str("h5"),
            "rnbqkN1r/ppp1n1p1/8/2b4p/8/2P5/PP1pBPPP/RNBQ1K1R w kq h6 0 9",
        ),
        (
            Move::from_str("Na6"),
            "r1bqkN1r/ppp1n1pp/n7/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Nbc6"),
            "r1bqkN1r/ppp1n1pp/2n5/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Nd7"),
            "r1bqkN1r/pppnn1pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Bd7"),
            "rn1qkN1r/pppbn1pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Be6"),
            "rn1qkN1r/ppp1n1pp/4b3/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Bf5"),
            "rn1qkN1r/ppp1n1pp/8/2b2b2/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Bg4"),
            "rn1qkN1r/ppp1n1pp/8/2b5/6b1/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Bh3"),
            "rn1qkN1r/ppp1n1pp/8/2b5/8/2P4b/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Qd7"),
            "rnb1kN1r/pppqn1pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Qd6"),
            "rnb1kN1r/ppp1n1pp/3q4/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Qd5"),
            "rnb1kN1r/ppp1n1pp/8/2bq4/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Qd4"),
            "rnb1kN1r/ppp1n1pp/8/2b5/3q4/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Qd3"),
            "rnb1kN1r/ppp1n1pp/8/2b5/8/2Pq4/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Kf7"),
            "rnbq1N1r/ppp1nkpp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w - - 2 9",
        ),
        (
            Move::from_str("Kxf8"),
            "rnbq1k1r/ppp1n1pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w - - 0 9",
        ),
        (
            Move::from_str("Rg8"),
            "rnbqkNr1/ppp1n1pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w q - 2 9",
        ),
        (
            Move::from_str("Rxf8"),
            "rnbqkr2/ppp1n1pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w q - 0 9",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_39() {
    let position = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1N1PP/RNBQK1nR w KQ - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("Nd2"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPPNN1PP/R1BQK1nR b KQ - 2 8",
        ),
        (
            Move::from_str("Na3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/N7/PPP1N1PP/R1BQK1nR b KQ - 2 8",
        ),
        (
            Move::from_str("Nbc3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/2N5/PPP1N1PP/R1BQK1nR b KQ - 2 8",
        ),
        (
            Move::from_str("Bd2"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPPBN1PP/RN1QK1nR b KQ - 2 8",
        ),
        (
            Move::from_str("Be3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/4B3/PPP1N1PP/RN1QK1nR b KQ - 2 8",
        ),
        (
            Move::from_str("Bf4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B2B2/8/PPP1N1PP/RN1QK1nR b KQ - 2 8",
        ),
        (
            Move::from_str("Bg5"),
            "rnbq1k1r/pp1Pbppp/2p5/6B1/2B5/8/PPP1N1PP/RN1QK1nR b KQ - 2 8",
        ),
        (
            Move::from_str("Bh6"),
            "rnbq1k1r/pp1Pbppp/2p4B/8/2B5/8/PPP1N1PP/RN1QK1nR b KQ - 2 8",
        ),
        (
            Move::from_str("Qd2"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPPQN1PP/RNB1K1nR b KQ - 2 8",
        ),
        (
            Move::from_str("Qd3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/3Q4/PPP1N1PP/RNB1K1nR b KQ - 2 8",
        ),
        (
            Move::from_str("Qd4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2BQ4/8/PPP1N1PP/RNB1K1nR b KQ - 2 8",
        ),
        (
            Move::from_str("Qd5"),
            "rnbq1k1r/pp1Pbppp/2p5/3Q4/2B5/8/PPP1N1PP/RNB1K1nR b KQ - 2 8",
        ),
        (
            Move::from_str("Qd6"),
            "rnbq1k1r/pp1Pbppp/2pQ4/8/2B5/8/PPP1N1PP/RNB1K1nR b KQ - 2 8",
        ),
        (
            Move::from_str("Kf1"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1N1PP/RNBQ1KnR b - - 2 8",
        ),
        (
            Move::from_str("Kd2"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPPKN1PP/RNBQ2nR b - - 2 8",
        ),
        (
            Move::from_str("Kf2"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NKPP/RNBQ2nR b - - 2 8",
        ),
        (
            Move::from_str("Rxg1"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1N1PP/RNBQK1R1 b Q - 0 8",
        ),
        (
            Move::from_str("a3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/P7/1PP1N1PP/RNBQK1nR b KQ - 0 8",
        ),
        (
            Move::from_str("a4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/P1B5/8/1PP1N1PP/RNBQK1nR b KQ a3 0 8",
        ),
        (
            Move::from_str("b3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/1P6/P1P1N1PP/RNBQK1nR b KQ - 0 8",
        ),
        (
            Move::from_str("b4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/1PB5/8/P1P1N1PP/RNBQK1nR b KQ b3 0 8",
        ),
        (
            Move::from_str("c3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/2P5/PP2N1PP/RNBQK1nR b KQ - 0 8",
        ),
        (
            Move::from_str("Nxg1"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP3PP/RNBQK1NR b KQ - 0 8",
        ),
        (
            Move::from_str("Nec3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/2N5/PPP3PP/RNBQK1nR b KQ - 2 8",
        ),
        (
            Move::from_str("Ng3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/6N1/PPP3PP/RNBQK1nR b KQ - 2 8",
        ),
        (
            Move::from_str("Nd4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2BN4/8/PPP3PP/RNBQK1nR b KQ - 2 8",
        ),
        (
            Move::from_str("Nf4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B2N2/8/PPP3PP/RNBQK1nR b KQ - 2 8",
        ),
        (
            Move::from_str("g3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/6P1/PPP1N2P/RNBQK1nR b KQ - 0 8",
        ),
        (
            Move::from_str("g4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B3P1/8/PPP1N2P/RNBQK1nR b KQ g3 0 8",
        ),
        (
            Move::from_str("h3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/7P/PPP1N1P1/RNBQK1nR b KQ - 0 8",
        ),
        (
            Move::from_str("h4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B4P/8/PPP1N1P1/RNBQK1nR b KQ h3 0 8",
        ),
        (
            Move::from_str("Bb3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/8/1B6/PPP1N1PP/RNBQK1nR b KQ - 2 8",
        ),
        (
            Move::from_str("Bd3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/8/3B4/PPP1N1PP/RNBQK1nR b KQ - 2 8",
        ),
        (
            Move::from_str("Bb5"),
            "rnbq1k1r/pp1Pbppp/2p5/1B6/8/8/PPP1N1PP/RNBQK1nR b KQ - 2 8",
        ),
        (
            Move::from_str("Ba6"),
            "rnbq1k1r/pp1Pbppp/B1p5/8/8/8/PPP1N1PP/RNBQK1nR b KQ - 2 8",
        ),
        (
            Move::from_str("Bd5"),
            "rnbq1k1r/pp1Pbppp/2p5/3B4/8/8/PPP1N1PP/RNBQK1nR b KQ - 2 8",
        ),
        (
            Move::from_str("Be6"),
            "rnbq1k1r/pp1Pbppp/2p1B3/8/8/8/PPP1N1PP/RNBQK1nR b KQ - 2 8",
        ),
        (
            Move::from_str("Bxf7"),
            "rnbq1k1r/pp1PbBpp/2p5/8/8/8/PPP1N1PP/RNBQK1nR b KQ - 0 8",
        ),
        (
            Move::from_str("dxc8=Q"),
            "rnQq1k1r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK1nR b KQ - 0 8",
        ),
        (
            Move::from_str("dxc8=R"),
            "rnRq1k1r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK1nR b KQ - 0 8",
        ),
        (
            Move::from_str("dxc8=N"),
            "rnNq1k1r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK1nR b KQ - 0 8",
        ),
        (
            Move::from_str("dxc8=B"),
            "rnBq1k1r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK1nR b KQ - 0 8",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_40() {
    let position = "rnbqk1Nr/ppp1n1pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R b kq - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("dxc1=Q"),
            "rnbqk1Nr/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/RNqQ1K1R w kq - 0 9",
        ),
        (
            Move::from_str("dxc1=R"),
            "rnbqk1Nr/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/RNrQ1K1R w kq - 0 9",
        ),
        (
            Move::from_str("dxc1=N"),
            "rnbqk1Nr/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/RNnQ1K1R w kq - 0 9",
        ),
        (
            Move::from_str("dxc1=B"),
            "rnbqk1Nr/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/RNbQ1K1R w kq - 0 9",
        ),
        (
            Move::from_str("Bb4"),
            "rnbqk1Nr/ppp1n1pp/8/8/1b6/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Ba3"),
            "rnbqk1Nr/ppp1n1pp/8/8/8/b1P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Bd4"),
            "rnbqk1Nr/ppp1n1pp/8/8/3b4/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Be3"),
            "rnbqk1Nr/ppp1n1pp/8/8/8/2P1b3/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Bxf2"),
            "rnbqk1Nr/ppp1n1pp/8/8/8/2P5/PP1pBbPP/RNBQ1K1R w kq - 0 9",
        ),
        (
            Move::from_str("Bb6"),
            "rnbqk1Nr/ppp1n1pp/1b6/8/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Bd6"),
            "rnbqk1Nr/ppp1n1pp/3b4/8/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("a6"),
            "rnbqk1Nr/1pp1n1pp/p7/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 0 9",
        ),
        (
            Move::from_str("a5"),
            "rnbqk1Nr/1pp1n1pp/8/p1b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq a6 0 9",
        ),
        (
            Move::from_str("b6"),
            "rnbqk1Nr/p1p1n1pp/1p6/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 0 9",
        ),
        (
            Move::from_str("b5"),
            "rnbqk1Nr/p1p1n1pp/8/1pb5/8/2P5/PP1pBPPP/RNBQ1K1R w kq b6 0 9",
        ),
        (
            Move::from_str("c6"),
            "rnbqk1Nr/pp2n1pp/2p5/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 0 9",
        ),
        (
            Move::from_str("Nd5"),
            "rnbqk1Nr/ppp3pp/8/2bn4/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Nf5"),
            "rnbqk1Nr/ppp3pp/8/2b2n2/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Nec6"),
            "rnbqk1Nr/ppp3pp/2n5/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Ng6"),
            "rnbqk1Nr/ppp3pp/6n1/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Nxg8"),
            "rnbqk1nr/ppp3pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 0 9",
        ),
        (
            Move::from_str("g6"),
            "rnbqk1Nr/ppp1n2p/6p1/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 0 9",
        ),
        (
            Move::from_str("g5"),
            "rnbqk1Nr/ppp1n2p/8/2b3p1/8/2P5/PP1pBPPP/RNBQ1K1R w kq g6 0 9",
        ),
        (
            Move::from_str("h6"),
            "rnbqk1Nr/ppp1n1p1/7p/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 0 9",
        ),
        (
            Move::from_str("h5"),
            "rnbqk1Nr/ppp1n1p1/8/2b4p/8/2P5/PP1pBPPP/RNBQ1K1R w kq h6 0 9",
        ),
        (
            Move::from_str("Na6"),
            "r1bqk1Nr/ppp1n1pp/n7/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Nbc6"),
            "r1bqk1Nr/ppp1n1pp/2n5/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Nd7"),
            "r1bqk1Nr/pppnn1pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Bd7"),
            "rn1qk1Nr/pppbn1pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Be6"),
            "rn1qk1Nr/ppp1n1pp/4b3/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Bf5"),
            "rn1qk1Nr/ppp1n1pp/8/2b2b2/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Bg4"),
            "rn1qk1Nr/ppp1n1pp/8/2b5/6b1/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Bh3"),
            "rn1qk1Nr/ppp1n1pp/8/2b5/8/2P4b/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Qd7"),
            "rnb1k1Nr/pppqn1pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Qd6"),
            "rnb1k1Nr/ppp1n1pp/3q4/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Qd5"),
            "rnb1k1Nr/ppp1n1pp/8/2bq4/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Qd4"),
            "rnb1k1Nr/ppp1n1pp/8/2b5/3q4/2P5/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Qd3"),
            "rnb1k1Nr/ppp1n1pp/8/2b5/8/2Pq4/PP1pBPPP/RNBQ1K1R w kq - 2 9",
        ),
        (
            Move::from_str("Kd7"),
            "rnbq2Nr/pppkn1pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w - - 2 9",
        ),
        (
            Move::from_str("Kf7"),
            "rnbq2Nr/ppp1nkpp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w - - 2 9",
        ),
        (
            Move::from_str("Kf8"),
            "rnbq1kNr/ppp1n1pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w - - 2 9",
        ),
        (
            Move::from_str("Rxg8"),
            "rnbqk1r1/ppp1n1pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w q - 0 9",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_41() {
    let position = "rnbqkN1r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R b KQkq - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("c5"),
            "rnbqkN1r/pp2bppp/8/2p5/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("a6"),
            "rnbqkN1r/1p2bppp/p1p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("a5"),
            "rnbqkN1r/1p2bppp/2p5/p7/2B5/8/PPP1N1PP/RNBQK2R w KQkq a6 0 9",
        ),
        (
            Move::from_str("b6"),
            "rnbqkN1r/p3bppp/1pp5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("b5"),
            "rnbqkN1r/p3bppp/2p5/1p6/2B5/8/PPP1N1PP/RNBQK2R w KQkq b6 0 9",
        ),
        (
            Move::from_str("Bd6"),
            "rnbqkN1r/pp3ppp/2pb4/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bc5"),
            "rnbqkN1r/pp3ppp/2p5/2b5/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bb4+"),
            "rnbqkN1r/pp3ppp/2p5/8/1bB5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Ba3"),
            "rnbqkN1r/pp3ppp/2p5/8/2B5/b7/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bf6"),
            "rnbqkN1r/pp3ppp/2p2b2/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bg5"),
            "rnbqkN1r/pp3ppp/2p5/6b1/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bh4+"),
            "rnbqkN1r/pp3ppp/2p5/8/2B4b/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bxf8"),
            "rnbqkb1r/pp3ppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("f6"),
            "rnbqkN1r/pp2b1pp/2p2p2/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("f5"),
            "rnbqkN1r/pp2b1pp/2p5/5p2/2B5/8/PPP1N1PP/RNBQK2R w KQkq f6 0 9",
        ),
        (
            Move::from_str("g6"),
            "rnbqkN1r/pp2bp1p/2p3p1/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("g5"),
            "rnbqkN1r/pp2bp1p/2p5/6p1/2B5/8/PPP1N1PP/RNBQK2R w KQkq g6 0 9",
        ),
        (
            Move::from_str("h6"),
            "rnbqkN1r/pp2bpp1/2p4p/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("h5"),
            "rnbqkN1r/pp2bpp1/2p5/7p/2B5/8/PPP1N1PP/RNBQK2R w KQkq h6 0 9",
        ),
        (
            Move::from_str("Na6"),
            "r1bqkN1r/pp2bppp/n1p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Nd7"),
            "r1bqkN1r/pp1nbppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bd7"),
            "rn1qkN1r/pp1bbppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Be6"),
            "rn1qkN1r/pp2bppp/2p1b3/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bf5"),
            "rn1qkN1r/pp2bppp/2p5/5b2/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bg4"),
            "rn1qkN1r/pp2bppp/2p5/8/2B3b1/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bh3"),
            "rn1qkN1r/pp2bppp/2p5/8/2B5/7b/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Qc7"),
            "rnb1kN1r/ppq1bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Qb6"),
            "rnb1kN1r/pp2bppp/1qp5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Qa5+"),
            "rnb1kN1r/pp2bppp/2p5/q7/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Qd7"),
            "rnb1kN1r/pp1qbppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Qd6"),
            "rnb1kN1r/pp2bppp/2pq4/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Qd5"),
            "rnb1kN1r/pp2bppp/2p5/3q4/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Qd4"),
            "rnb1kN1r/pp2bppp/2p5/8/2Bq4/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Qd3"),
            "rnb1kN1r/pp2bppp/2p5/8/2B5/3q4/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Qd2+"),
            "rnb1kN1r/pp2bppp/2p5/8/2B5/8/PPPqN1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Qxd1+"),
            "rnb1kN1r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBqK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("Kxf8"),
            "rnbq1k1r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQ - 0 9",
        ),
        (
            Move::from_str("Rg8"),
            "rnbqkNr1/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQq - 2 9",
        ),
        (
            Move::from_str("Rxf8"),
            "rnbqkr2/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQq - 0 9",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_42() {
    let position = "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/RNBQKn1R w KQkq - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("Nd2"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP1NBPPP/R1BQKn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Na3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/N1P5/PP2BPPP/R1BQKn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bd2"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP1BBPPP/RN1QKn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Be3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P1B3/PP2BPPP/RN1QKn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bf4"),
            "rnbqk2r/ppp1n1pp/8/2b5/5B2/2P5/PP2BPPP/RN1QKn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bg5"),
            "rnbqk2r/ppp1n1pp/8/2b3B1/8/2P5/PP2BPPP/RN1QKn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bh6"),
            "rnbqk2r/ppp1n1pp/7B/2b5/8/2P5/PP2BPPP/RN1QKn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Qc2"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PPQ1BPPP/RNB1Kn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Qb3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/1QP5/PP2BPPP/RNB1Kn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Qa4+"),
            "rnbqk2r/ppp1n1pp/8/2b5/Q7/2P5/PP2BPPP/RNB1Kn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Qd2"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP1QBPPP/RNB1Kn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Qd3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2PQ4/PP2BPPP/RNB1Kn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Qd4"),
            "rnbqk2r/ppp1n1pp/8/2b5/3Q4/2P5/PP2BPPP/RNB1Kn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Qd5"),
            "rnbqk2r/ppp1n1pp/8/2bQ4/8/2P5/PP2BPPP/RNB1Kn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Qd6"),
            "rnbqk2r/ppp1n1pp/3Q4/2b5/8/2P5/PP2BPPP/RNB1Kn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Qd7+"),
            "rnbqk2r/pppQn1pp/8/2b5/8/2P5/PP2BPPP/RNB1Kn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Qxd8+"),
            "rnbQk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/RNB1Kn1R b KQkq - 0 8",
        ),
        (
            Move::from_str("Kxf1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/RNBQ1K1R b kq - 0 8",
        ),
        (
            Move::from_str("Rg1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/RNBQKnR1 b Qkq - 2 8",
        ),
        (
            Move::from_str("Rxf1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/RNBQKR2 b Qkq - 0 8",
        ),
        (
            Move::from_str("a3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/P1P5/1P2BPPP/RNBQKn1R b KQkq - 0 8",
        ),
        (
            Move::from_str("a4"),
            "rnbqk2r/ppp1n1pp/8/2b5/P7/2P5/1P2BPPP/RNBQKn1R b KQkq a3 0 8",
        ),
        (
            Move::from_str("b3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/1PP5/P3BPPP/RNBQKn1R b KQkq - 0 8",
        ),
        (
            Move::from_str("b4"),
            "rnbqk2r/ppp1n1pp/8/2b5/1P6/2P5/P3BPPP/RNBQKn1R b KQkq b3 0 8",
        ),
        (
            Move::from_str("Bxf1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP3PPP/RNBQKB1R b KQkq - 0 8",
        ),
        (
            Move::from_str("Bd3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2PB4/PP3PPP/RNBQKn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bc4"),
            "rnbqk2r/ppp1n1pp/8/2b5/2B5/2P5/PP3PPP/RNBQKn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bb5+"),
            "rnbqk2r/ppp1n1pp/8/1Bb5/8/2P5/PP3PPP/RNBQKn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Ba6"),
            "rnbqk2r/ppp1n1pp/B7/2b5/8/2P5/PP3PPP/RNBQKn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bf3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P2B2/PP3PPP/RNBQKn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bg4"),
            "rnbqk2r/ppp1n1pp/8/2b5/6B1/2P5/PP3PPP/RNBQKn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bh5+"),
            "rnbqk2r/ppp1n1pp/8/2b4B/8/2P5/PP3PPP/RNBQKn1R b KQkq - 2 8",
        ),
        (
            Move::from_str("f3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P2P2/PP2B1PP/RNBQKn1R b KQkq - 0 8",
        ),
        (
            Move::from_str("f4"),
            "rnbqk2r/ppp1n1pp/8/2b5/5P2/2P5/PP2B1PP/RNBQKn1R b KQkq f3 0 8",
        ),
        (
            Move::from_str("g3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P3P1/PP2BP1P/RNBQKn1R b KQkq - 0 8",
        ),
        (
            Move::from_str("g4"),
            "rnbqk2r/ppp1n1pp/8/2b5/6P1/2P5/PP2BP1P/RNBQKn1R b KQkq g3 0 8",
        ),
        (
            Move::from_str("h3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P4P/PP2BPP1/RNBQKn1R b KQkq - 0 8",
        ),
        (
            Move::from_str("h4"),
            "rnbqk2r/ppp1n1pp/8/2b5/7P/2P5/PP2BPP1/RNBQKn1R b KQkq h3 0 8",
        ),
        (
            Move::from_str("c4"),
            "rnbqk2r/ppp1n1pp/8/2b5/2P5/8/PP2BPPP/RNBQKn1R b KQkq - 0 8",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_43() {
    let position = "rnbqk1Nr/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R b KQkq - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("c5"),
            "rnbqk1Nr/pp2bppp/8/2p5/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("a6"),
            "rnbqk1Nr/1p2bppp/p1p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("a5"),
            "rnbqk1Nr/1p2bppp/2p5/p7/2B5/8/PPP1N1PP/RNBQK2R w KQkq a6 0 9",
        ),
        (
            Move::from_str("b6"),
            "rnbqk1Nr/p3bppp/1pp5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("b5"),
            "rnbqk1Nr/p3bppp/2p5/1p6/2B5/8/PPP1N1PP/RNBQK2R w KQkq b6 0 9",
        ),
        (
            Move::from_str("Bd6"),
            "rnbqk1Nr/pp3ppp/2pb4/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bc5"),
            "rnbqk1Nr/pp3ppp/2p5/2b5/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bb4+"),
            "rnbqk1Nr/pp3ppp/2p5/8/1bB5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Ba3"),
            "rnbqk1Nr/pp3ppp/2p5/8/2B5/b7/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bf6"),
            "rnbqk1Nr/pp3ppp/2p2b2/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bg5"),
            "rnbqk1Nr/pp3ppp/2p5/6b1/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bh4+"),
            "rnbqk1Nr/pp3ppp/2p5/8/2B4b/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bf8"),
            "rnbqkbNr/pp3ppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("f6"),
            "rnbqk1Nr/pp2b1pp/2p2p2/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("f5"),
            "rnbqk1Nr/pp2b1pp/2p5/5p2/2B5/8/PPP1N1PP/RNBQK2R w KQkq f6 0 9",
        ),
        (
            Move::from_str("g6"),
            "rnbqk1Nr/pp2bp1p/2p3p1/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("g5"),
            "rnbqk1Nr/pp2bp1p/2p5/6p1/2B5/8/PPP1N1PP/RNBQK2R w KQkq g6 0 9",
        ),
        (
            Move::from_str("h6"),
            "rnbqk1Nr/pp2bpp1/2p4p/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("h5"),
            "rnbqk1Nr/pp2bpp1/2p5/7p/2B5/8/PPP1N1PP/RNBQK2R w KQkq h6 0 9",
        ),
        (
            Move::from_str("Na6"),
            "r1bqk1Nr/pp2bppp/n1p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Nd7"),
            "r1bqk1Nr/pp1nbppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bd7"),
            "rn1qk1Nr/pp1bbppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Be6"),
            "rn1qk1Nr/pp2bppp/2p1b3/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bf5"),
            "rn1qk1Nr/pp2bppp/2p5/5b2/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bg4"),
            "rn1qk1Nr/pp2bppp/2p5/8/2B3b1/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bh3"),
            "rn1qk1Nr/pp2bppp/2p5/8/2B5/7b/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Qc7"),
            "rnb1k1Nr/ppq1bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Qb6"),
            "rnb1k1Nr/pp2bppp/1qp5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Qa5+"),
            "rnb1k1Nr/pp2bppp/2p5/q7/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Qd7"),
            "rnb1k1Nr/pp1qbppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Qd6"),
            "rnb1k1Nr/pp2bppp/2pq4/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Qd5"),
            "rnb1k1Nr/pp2bppp/2p5/3q4/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Qd4"),
            "rnb1k1Nr/pp2bppp/2p5/8/2Bq4/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Qd3"),
            "rnb1k1Nr/pp2bppp/2p5/8/2B5/3q4/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Qd2+"),
            "rnb1k1Nr/pp2bppp/2p5/8/2B5/8/PPPqN1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Qxd1+"),
            "rnb1k1Nr/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBqK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("Kf8"),
            "rnbq1kNr/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("Rxg8"),
            "rnbqk1r1/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQq - 0 9",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_44() {
    let position = "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/RNBQK1nR w KQkq - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("Nd2"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP1NBPPP/R1BQK1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("Na3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/N1P5/PP2BPPP/R1BQK1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("Bd2"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP1BBPPP/RN1QK1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("Be3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P1B3/PP2BPPP/RN1QK1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("Bf4"),
            "rnbqk2r/ppp1n1pp/8/2b5/5B2/2P5/PP2BPPP/RN1QK1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("Bg5"),
            "rnbqk2r/ppp1n1pp/8/2b3B1/8/2P5/PP2BPPP/RN1QK1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("Bh6"),
            "rnbqk2r/ppp1n1pp/7B/2b5/8/2P5/PP2BPPP/RN1QK1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("Qc2"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PPQ1BPPP/RNB1K1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("Qb3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/1QP5/PP2BPPP/RNB1K1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("Qa4+"),
            "rnbqk2r/ppp1n1pp/8/2b5/Q7/2P5/PP2BPPP/RNB1K1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("Qd2"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP1QBPPP/RNB1K1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("Qd3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2PQ4/PP2BPPP/RNB1K1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("Qd4"),
            "rnbqk2r/ppp1n1pp/8/2b5/3Q4/2P5/PP2BPPP/RNB1K1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("Qd5"),
            "rnbqk2r/ppp1n1pp/8/2bQ4/8/2P5/PP2BPPP/RNB1K1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("Qd6"),
            "rnbqk2r/ppp1n1pp/3Q4/2b5/8/2P5/PP2BPPP/RNB1K1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("Qd7+"),
            "rnbqk2r/pppQn1pp/8/2b5/8/2P5/PP2BPPP/RNB1K1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("Qxd8+"),
            "rnbQk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/RNB1K1nR b KQkq - 0 8",
        ),
        (
            Move::from_str("Kf1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/RNBQ1KnR b kq - 2 8",
        ),
        (
            Move::from_str("Rxg1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/RNBQK1R1 b Qkq - 0 8",
        ),
        (
            Move::from_str("a3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/P1P5/1P2BPPP/RNBQK1nR b KQkq - 0 8",
        ),
        (
            Move::from_str("a4"),
            "rnbqk2r/ppp1n1pp/8/2b5/P7/2P5/1P2BPPP/RNBQK1nR b KQkq a3 0 8",
        ),
        (
            Move::from_str("b3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/1PP5/P3BPPP/RNBQK1nR b KQkq - 0 8",
        ),
        (
            Move::from_str("b4"),
            "rnbqk2r/ppp1n1pp/8/2b5/1P6/2P5/P3BPPP/RNBQK1nR b KQkq b3 0 8",
        ),
        (
            Move::from_str("Bf1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP3PPP/RNBQKBnR b KQkq - 2 8",
        ),
        (
            Move::from_str("Bd3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2PB4/PP3PPP/RNBQK1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("Bc4"),
            "rnbqk2r/ppp1n1pp/8/2b5/2B5/2P5/PP3PPP/RNBQK1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("Bb5+"),
            "rnbqk2r/ppp1n1pp/8/1Bb5/8/2P5/PP3PPP/RNBQK1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("Ba6"),
            "rnbqk2r/ppp1n1pp/B7/2b5/8/2P5/PP3PPP/RNBQK1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("Bf3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P2B2/PP3PPP/RNBQK1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("Bg4"),
            "rnbqk2r/ppp1n1pp/8/2b5/6B1/2P5/PP3PPP/RNBQK1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("Bh5+"),
            "rnbqk2r/ppp1n1pp/8/2b4B/8/2P5/PP3PPP/RNBQK1nR b KQkq - 2 8",
        ),
        (
            Move::from_str("f3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P2P2/PP2B1PP/RNBQK1nR b KQkq - 0 8",
        ),
        (
            Move::from_str("f4"),
            "rnbqk2r/ppp1n1pp/8/2b5/5P2/2P5/PP2B1PP/RNBQK1nR b KQkq f3 0 8",
        ),
        (
            Move::from_str("g3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P3P1/PP2BP1P/RNBQK1nR b KQkq - 0 8",
        ),
        (
            Move::from_str("g4"),
            "rnbqk2r/ppp1n1pp/8/2b5/6P1/2P5/PP2BP1P/RNBQK1nR b KQkq g3 0 8",
        ),
        (
            Move::from_str("h3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P4P/PP2BPP1/RNBQK1nR b KQkq - 0 8",
        ),
        (
            Move::from_str("h4"),
            "rnbqk2r/ppp1n1pp/8/2b5/7P/2P5/PP2BPP1/RNBQK1nR b KQkq h3 0 8",
        ),
        (
            Move::from_str("c4"),
            "rnbqk2r/ppp1n1pp/8/2b5/2P5/8/PP2BPPP/RNBQK1nR b KQkq - 0 8",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_45() {
    let position = "r2Nk2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R b KQkq - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("c5"),
            "r2Nk2r/pp2bppp/8/2p5/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("a6"),
            "r2Nk2r/1p2bppp/p1p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("a5"),
            "r2Nk2r/1p2bppp/2p5/p7/2B5/8/PPP1N1PP/RNBQK2R w KQkq a6 0 9",
        ),
        (
            Move::from_str("b6"),
            "r2Nk2r/p3bppp/1pp5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("b5"),
            "r2Nk2r/p3bppp/2p5/1p6/2B5/8/PPP1N1PP/RNBQK2R w KQkq b6 0 9",
        ),
        (
            Move::from_str("Bd6"),
            "r2Nk2r/pp3ppp/2pb4/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bc5"),
            "r2Nk2r/pp3ppp/2p5/2b5/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bb4+"),
            "r2Nk2r/pp3ppp/2p5/8/1bB5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Ba3"),
            "r2Nk2r/pp3ppp/2p5/8/2B5/b7/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bf6"),
            "r2Nk2r/pp3ppp/2p2b2/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bg5"),
            "r2Nk2r/pp3ppp/2p5/6b1/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bh4+"),
            "r2Nk2r/pp3ppp/2p5/8/2B4b/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bxd8"),
            "r2bk2r/pp3ppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("Bf8"),
            "r2Nkb1r/pp3ppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("f6"),
            "r2Nk2r/pp2b1pp/2p2p2/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("f5"),
            "r2Nk2r/pp2b1pp/2p5/5p2/2B5/8/PPP1N1PP/RNBQK2R w KQkq f6 0 9",
        ),
        (
            Move::from_str("g6"),
            "r2Nk2r/pp2bp1p/2p3p1/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("g5"),
            "r2Nk2r/pp2bp1p/2p5/6p1/2B5/8/PPP1N1PP/RNBQK2R w KQkq g6 0 9",
        ),
        (
            Move::from_str("h6"),
            "r2Nk2r/pp2bpp1/2p4p/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("h5"),
            "r2Nk2r/pp2bpp1/2p5/7p/2B5/8/PPP1N1PP/RNBQK2R w KQkq h6 0 9",
        ),
        (
            Move::from_str("Rb8"),
            "1r1Nk2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQk - 2 9",
        ),
        (
            Move::from_str("Rc8"),
            "2rNk2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQk - 2 9",
        ),
        (
            Move::from_str("Rxd8"),
            "3rk2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQk - 0 9",
        ),
        (
            Move::from_str("Kf8"),
            "r2N1k1r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("O-O"),
            "r2N1rk1/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("Rg8"),
            "r2Nk1r1/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQq - 2 9",
        ),
        (
            Move::from_str("Rf8"),
            "r2Nkr2/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQq - 2 9",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_46() {
    let position = "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R2nK2R w KQkq - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("Rb1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/1R1nK2R b Kkq - 2 8",
        ),
        (
            Move::from_str("Rc1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/2RnK2R b Kkq - 2 8",
        ),
        (
            Move::from_str("Rxd1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/3RK2R b Kkq - 0 8",
        ),
        (
            Move::from_str("Kf1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R2n1K1R b kq - 2 8",
        ),
        (
            Move::from_str("O-O"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R2n1RK1 b kq - 2 8",
        ),
        (
            Move::from_str("Rg1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R2nK1R1 b Qkq - 2 8",
        ),
        (
            Move::from_str("Rf1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R2nKR2 b Qkq - 2 8",
        ),
        (
            Move::from_str("a3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/P1P5/1P2BPPP/R2nK2R b KQkq - 0 8",
        ),
        (
            Move::from_str("a4"),
            "rnbqk2r/ppp1n1pp/8/2b5/P7/2P5/1P2BPPP/R2nK2R b KQkq a3 0 8",
        ),
        (
            Move::from_str("b3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/1PP5/P3BPPP/R2nK2R b KQkq - 0 8",
        ),
        (
            Move::from_str("b4"),
            "rnbqk2r/ppp1n1pp/8/2b5/1P6/2P5/P3BPPP/R2nK2R b KQkq b3 0 8",
        ),
        (
            Move::from_str("Bxd1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP3PPP/R2BK2R b KQkq - 0 8",
        ),
        (
            Move::from_str("Bf1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP3PPP/R2nKB1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bd3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2PB4/PP3PPP/R2nK2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bc4"),
            "rnbqk2r/ppp1n1pp/8/2b5/2B5/2P5/PP3PPP/R2nK2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bb5+"),
            "rnbqk2r/ppp1n1pp/8/1Bb5/8/2P5/PP3PPP/R2nK2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Ba6"),
            "rnbqk2r/ppp1n1pp/B7/2b5/8/2P5/PP3PPP/R2nK2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bf3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P2B2/PP3PPP/R2nK2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bg4"),
            "rnbqk2r/ppp1n1pp/8/2b5/6B1/2P5/PP3PPP/R2nK2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bh5+"),
            "rnbqk2r/ppp1n1pp/8/2b4B/8/2P5/PP3PPP/R2nK2R b KQkq - 2 8",
        ),
        (
            Move::from_str("f3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P2P2/PP2B1PP/R2nK2R b KQkq - 0 8",
        ),
        (
            Move::from_str("f4"),
            "rnbqk2r/ppp1n1pp/8/2b5/5P2/2P5/PP2B1PP/R2nK2R b KQkq f3 0 8",
        ),
        (
            Move::from_str("g3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P3P1/PP2BP1P/R2nK2R b KQkq - 0 8",
        ),
        (
            Move::from_str("g4"),
            "rnbqk2r/ppp1n1pp/8/2b5/6P1/2P5/PP2BP1P/R2nK2R b KQkq g3 0 8",
        ),
        (
            Move::from_str("h3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P4P/PP2BPP1/R2nK2R b KQkq - 0 8",
        ),
        (
            Move::from_str("h4"),
            "rnbqk2r/ppp1n1pp/8/2b5/7P/2P5/PP2BPP1/R2nK2R b KQkq h3 0 8",
        ),
        (
            Move::from_str("c4"),
            "rnbqk2r/ppp1n1pp/8/2b5/2P5/8/PP2BPPP/R2nK2R b KQkq - 0 8",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_47() {
    let position = "r1N1k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R b KQkq - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("c5"),
            "r1N1k2r/pp2bppp/8/2p5/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("a6"),
            "r1N1k2r/1p2bppp/p1p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("a5"),
            "r1N1k2r/1p2bppp/2p5/p7/2B5/8/PPP1N1PP/RNBQK2R w KQkq a6 0 9",
        ),
        (
            Move::from_str("b6"),
            "r1N1k2r/p3bppp/1pp5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("b5"),
            "r1N1k2r/p3bppp/2p5/1p6/2B5/8/PPP1N1PP/RNBQK2R w KQkq b6 0 9",
        ),
        (
            Move::from_str("Bd6"),
            "r1N1k2r/pp3ppp/2pb4/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bc5"),
            "r1N1k2r/pp3ppp/2p5/2b5/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bb4+"),
            "r1N1k2r/pp3ppp/2p5/8/1bB5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Ba3"),
            "r1N1k2r/pp3ppp/2p5/8/2B5/b7/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bf6"),
            "r1N1k2r/pp3ppp/2p2b2/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bg5"),
            "r1N1k2r/pp3ppp/2p5/6b1/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bh4+"),
            "r1N1k2r/pp3ppp/2p5/8/2B4b/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bd8"),
            "r1Nbk2r/pp3ppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bf8"),
            "r1N1kb1r/pp3ppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("f6"),
            "r1N1k2r/pp2b1pp/2p2p2/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("f5"),
            "r1N1k2r/pp2b1pp/2p5/5p2/2B5/8/PPP1N1PP/RNBQK2R w KQkq f6 0 9",
        ),
        (
            Move::from_str("g6"),
            "r1N1k2r/pp2bp1p/2p3p1/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("g5"),
            "r1N1k2r/pp2bp1p/2p5/6p1/2B5/8/PPP1N1PP/RNBQK2R w KQkq g6 0 9",
        ),
        (
            Move::from_str("h6"),
            "r1N1k2r/pp2bpp1/2p4p/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("h5"),
            "r1N1k2r/pp2bpp1/2p5/7p/2B5/8/PPP1N1PP/RNBQK2R w KQkq h6 0 9",
        ),
        (
            Move::from_str("Rb8"),
            "1rN1k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQk - 2 9",
        ),
        (
            Move::from_str("Rxc8"),
            "2r1k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQk - 0 9",
        ),
        (
            Move::from_str("Kf8"),
            "r1N2k1r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("O-O"),
            "r1N2rk1/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("Rg8"),
            "r1N1k1r1/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQq - 2 9",
        ),
        (
            Move::from_str("Rf8"),
            "r1N1kr2/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQq - 2 9",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_48() {
    let position = "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R1n1K2R w KQkq - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("Rb1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/1Rn1K2R b Kkq - 2 8",
        ),
        (
            Move::from_str("Rxc1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/2R1K2R b Kkq - 0 8",
        ),
        (
            Move::from_str("Kf1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R1n2K1R b kq - 2 8",
        ),
        (
            Move::from_str("O-O"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R1n2RK1 b kq - 2 8",
        ),
        (
            Move::from_str("Rg1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R1n1K1R1 b Qkq - 2 8",
        ),
        (
            Move::from_str("Rf1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R1n1KR2 b Qkq - 2 8",
        ),
        (
            Move::from_str("a3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/P1P5/1P2BPPP/R1n1K2R b KQkq - 0 8",
        ),
        (
            Move::from_str("a4"),
            "rnbqk2r/ppp1n1pp/8/2b5/P7/2P5/1P2BPPP/R1n1K2R b KQkq a3 0 8",
        ),
        (
            Move::from_str("b3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/1PP5/P3BPPP/R1n1K2R b KQkq - 0 8",
        ),
        (
            Move::from_str("b4"),
            "rnbqk2r/ppp1n1pp/8/2b5/1P6/2P5/P3BPPP/R1n1K2R b KQkq b3 0 8",
        ),
        (
            Move::from_str("Bd1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP3PPP/R1nBK2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bf1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP3PPP/R1n1KB1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bd3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2PB4/PP3PPP/R1n1K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bc4"),
            "rnbqk2r/ppp1n1pp/8/2b5/2B5/2P5/PP3PPP/R1n1K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bb5+"),
            "rnbqk2r/ppp1n1pp/8/1Bb5/8/2P5/PP3PPP/R1n1K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Ba6"),
            "rnbqk2r/ppp1n1pp/B7/2b5/8/2P5/PP3PPP/R1n1K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bf3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P2B2/PP3PPP/R1n1K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bg4"),
            "rnbqk2r/ppp1n1pp/8/2b5/6B1/2P5/PP3PPP/R1n1K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bh5+"),
            "rnbqk2r/ppp1n1pp/8/2b4B/8/2P5/PP3PPP/R1n1K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("f3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P2P2/PP2B1PP/R1n1K2R b KQkq - 0 8",
        ),
        (
            Move::from_str("f4"),
            "rnbqk2r/ppp1n1pp/8/2b5/5P2/2P5/PP2B1PP/R1n1K2R b KQkq f3 0 8",
        ),
        (
            Move::from_str("g3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P3P1/PP2BP1P/R1n1K2R b KQkq - 0 8",
        ),
        (
            Move::from_str("g4"),
            "rnbqk2r/ppp1n1pp/8/2b5/6P1/2P5/PP2BP1P/R1n1K2R b KQkq g3 0 8",
        ),
        (
            Move::from_str("h3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P4P/PP2BPP1/R1n1K2R b KQkq - 0 8",
        ),
        (
            Move::from_str("h4"),
            "rnbqk2r/ppp1n1pp/8/2b5/7P/2P5/PP2BPP1/R1n1K2R b KQkq h3 0 8",
        ),
        (
            Move::from_str("c4"),
            "rnbqk2r/ppp1n1pp/8/2b5/2P5/8/PP2BPPP/R1n1K2R b KQkq - 0 8",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_49() {
    let position = "rN2k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R b KQkq - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("c5"),
            "rN2k2r/pp2bppp/8/2p5/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("a6"),
            "rN2k2r/1p2bppp/p1p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("a5"),
            "rN2k2r/1p2bppp/2p5/p7/2B5/8/PPP1N1PP/RNBQK2R w KQkq a6 0 9",
        ),
        (
            Move::from_str("b6"),
            "rN2k2r/p3bppp/1pp5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("b5"),
            "rN2k2r/p3bppp/2p5/1p6/2B5/8/PPP1N1PP/RNBQK2R w KQkq b6 0 9",
        ),
        (
            Move::from_str("Bd6"),
            "rN2k2r/pp3ppp/2pb4/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bc5"),
            "rN2k2r/pp3ppp/2p5/2b5/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bb4+"),
            "rN2k2r/pp3ppp/2p5/8/1bB5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Ba3"),
            "rN2k2r/pp3ppp/2p5/8/2B5/b7/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bf6"),
            "rN2k2r/pp3ppp/2p2b2/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bg5"),
            "rN2k2r/pp3ppp/2p5/6b1/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bh4+"),
            "rN2k2r/pp3ppp/2p5/8/2B4b/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bd8"),
            "rN1bk2r/pp3ppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bf8"),
            "rN2kb1r/pp3ppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 2 9",
        ),
        (
            Move::from_str("f6"),
            "rN2k2r/pp2b1pp/2p2p2/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("f5"),
            "rN2k2r/pp2b1pp/2p5/5p2/2B5/8/PPP1N1PP/RNBQK2R w KQkq f6 0 9",
        ),
        (
            Move::from_str("g6"),
            "rN2k2r/pp2bp1p/2p3p1/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("g5"),
            "rN2k2r/pp2bp1p/2p5/6p1/2B5/8/PPP1N1PP/RNBQK2R w KQkq g6 0 9",
        ),
        (
            Move::from_str("h6"),
            "rN2k2r/pp2bpp1/2p4p/8/2B5/8/PPP1N1PP/RNBQK2R w KQkq - 0 9",
        ),
        (
            Move::from_str("h5"),
            "rN2k2r/pp2bpp1/2p5/7p/2B5/8/PPP1N1PP/RNBQK2R w KQkq h6 0 9",
        ),
        (
            Move::from_str("Rxb8"),
            "1r2k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQk - 0 9",
        ),
        (
            Move::from_str("Kf8"),
            "rN3k1r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("O-O"),
            "rN3rk1/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("Rg8"),
            "rN2k1r1/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQq - 2 9",
        ),
        (
            Move::from_str("Rf8"),
            "rN2kr2/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2R w KQq - 2 9",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_50() {
    let position = "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/Rn2K2R w KQkq - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("Rxb1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/1R2K2R b Kkq - 0 8",
        ),
        (
            Move::from_str("Kf1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/Rn3K1R b kq - 2 8",
        ),
        (
            Move::from_str("O-O"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/Rn3RK1 b kq - 2 8",
        ),
        (
            Move::from_str("Rg1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/Rn2K1R1 b Qkq - 2 8",
        ),
        (
            Move::from_str("Rf1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/Rn2KR2 b Qkq - 2 8",
        ),
        (
            Move::from_str("a3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/P1P5/1P2BPPP/Rn2K2R b KQkq - 0 8",
        ),
        (
            Move::from_str("a4"),
            "rnbqk2r/ppp1n1pp/8/2b5/P7/2P5/1P2BPPP/Rn2K2R b KQkq a3 0 8",
        ),
        (
            Move::from_str("b3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/1PP5/P3BPPP/Rn2K2R b KQkq - 0 8",
        ),
        (
            Move::from_str("b4"),
            "rnbqk2r/ppp1n1pp/8/2b5/1P6/2P5/P3BPPP/Rn2K2R b KQkq b3 0 8",
        ),
        (
            Move::from_str("Bd1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP3PPP/Rn1BK2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bf1"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P5/PP3PPP/Rn2KB1R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bd3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2PB4/PP3PPP/Rn2K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bc4"),
            "rnbqk2r/ppp1n1pp/8/2b5/2B5/2P5/PP3PPP/Rn2K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bb5+"),
            "rnbqk2r/ppp1n1pp/8/1Bb5/8/2P5/PP3PPP/Rn2K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Ba6"),
            "rnbqk2r/ppp1n1pp/B7/2b5/8/2P5/PP3PPP/Rn2K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bf3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P2B2/PP3PPP/Rn2K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bg4"),
            "rnbqk2r/ppp1n1pp/8/2b5/6B1/2P5/PP3PPP/Rn2K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bh5+"),
            "rnbqk2r/ppp1n1pp/8/2b4B/8/2P5/PP3PPP/Rn2K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("f3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P2P2/PP2B1PP/Rn2K2R b KQkq - 0 8",
        ),
        (
            Move::from_str("f4"),
            "rnbqk2r/ppp1n1pp/8/2b5/5P2/2P5/PP2B1PP/Rn2K2R b KQkq f3 0 8",
        ),
        (
            Move::from_str("g3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P3P1/PP2BP1P/Rn2K2R b KQkq - 0 8",
        ),
        (
            Move::from_str("g4"),
            "rnbqk2r/ppp1n1pp/8/2b5/6P1/2P5/PP2BP1P/Rn2K2R b KQkq g3 0 8",
        ),
        (
            Move::from_str("h3"),
            "rnbqk2r/ppp1n1pp/8/2b5/8/2P4P/PP2BPP1/Rn2K2R b KQkq - 0 8",
        ),
        (
            Move::from_str("h4"),
            "rnbqk2r/ppp1n1pp/8/2b5/7P/2P5/PP2BPP1/Rn2K2R b KQkq h3 0 8",
        ),
        (
            Move::from_str("c4"),
            "rnbqk2r/ppp1n1pp/8/2b5/2P5/8/PP2BPPP/Rn2K2R b KQkq - 0 8",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_51() {
    let position = "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/Rn2K2R w KQkq - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("Rxb1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/1R2K2R b Kkq - 0 8",
        ),
        (
            Move::from_str("Kd1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/Rn1K3R b kq - 2 8",
        ),
        (
            Move::from_str("Kf1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/Rn3K1R b kq - 2 8",
        ),
        (
            Move::from_str("Kf2"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1NKPP/Rn5R b kq - 2 8",
        ),
        (
            Move::from_str("O-O"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/Rn3RK1 b kq - 2 8",
        ),
        (
            Move::from_str("Rg1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/Rn2K1R1 b Qkq - 2 8",
        ),
        (
            Move::from_str("Rf1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/Rn2KR2 b Qkq - 2 8",
        ),
        (
            Move::from_str("a3"),
            "r3k2r/pp2bppp/2p5/8/2B5/P7/1PP1N1PP/Rn2K2R b KQkq - 0 8",
        ),
        (
            Move::from_str("a4"),
            "r3k2r/pp2bppp/2p5/8/P1B5/8/1PP1N1PP/Rn2K2R b KQkq a3 0 8",
        ),
        (
            Move::from_str("b3"),
            "r3k2r/pp2bppp/2p5/8/2B5/1P6/P1P1N1PP/Rn2K2R b KQkq - 0 8",
        ),
        (
            Move::from_str("b4"),
            "r3k2r/pp2bppp/2p5/8/1PB5/8/P1P1N1PP/Rn2K2R b KQkq b3 0 8",
        ),
        (
            Move::from_str("c3"),
            "r3k2r/pp2bppp/2p5/8/2B5/2P5/PP2N1PP/Rn2K2R b KQkq - 0 8",
        ),
        (
            Move::from_str("Nc1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP3PP/RnN1K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Ng1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP3PP/Rn2K1NR b KQkq - 2 8",
        ),
        (
            Move::from_str("Nc3"),
            "r3k2r/pp2bppp/2p5/8/2B5/2N5/PPP3PP/Rn2K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Ng3"),
            "r3k2r/pp2bppp/2p5/8/2B5/6N1/PPP3PP/Rn2K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Nd4"),
            "r3k2r/pp2bppp/2p5/8/2BN4/8/PPP3PP/Rn2K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Nf4"),
            "r3k2r/pp2bppp/2p5/8/2B2N2/8/PPP3PP/Rn2K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("g3"),
            "r3k2r/pp2bppp/2p5/8/2B5/6P1/PPP1N2P/Rn2K2R b KQkq - 0 8",
        ),
        (
            Move::from_str("g4"),
            "r3k2r/pp2bppp/2p5/8/2B3P1/8/PPP1N2P/Rn2K2R b KQkq g3 0 8",
        ),
        (
            Move::from_str("h3"),
            "r3k2r/pp2bppp/2p5/8/2B5/7P/PPP1N1P1/Rn2K2R b KQkq - 0 8",
        ),
        (
            Move::from_str("h4"),
            "r3k2r/pp2bppp/2p5/8/2B4P/8/PPP1N1P1/Rn2K2R b KQkq h3 0 8",
        ),
        (
            Move::from_str("Bb3"),
            "r3k2r/pp2bppp/2p5/8/8/1B6/PPP1N1PP/Rn2K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bd3"),
            "r3k2r/pp2bppp/2p5/8/8/3B4/PPP1N1PP/Rn2K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bb5"),
            "r3k2r/pp2bppp/2p5/1B6/8/8/PPP1N1PP/Rn2K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Ba6"),
            "r3k2r/pp2bppp/B1p5/8/8/8/PPP1N1PP/Rn2K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bd5"),
            "r3k2r/pp2bppp/2p5/3B4/8/8/PPP1N1PP/Rn2K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Be6"),
            "r3k2r/pp2bppp/2p1B3/8/8/8/PPP1N1PP/Rn2K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bxf7+"),
            "r3k2r/pp2bBpp/2p5/8/8/8/PPP1N1PP/Rn2K2R b KQkq - 0 8",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_52() {
    let position = "rN2k2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R b KQkq - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("Bb4"),
            "rN2k2r/ppp1n1pp/8/8/1b6/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Ba3"),
            "rN2k2r/ppp1n1pp/8/8/8/b1P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bd4"),
            "rN2k2r/ppp1n1pp/8/8/3b4/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Be3"),
            "rN2k2r/ppp1n1pp/8/8/8/2P1b3/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bxf2+"),
            "rN2k2r/ppp1n1pp/8/8/8/2P5/PP2BbPP/R3K2R w KQkq - 0 9",
        ),
        (
            Move::from_str("Bb6"),
            "rN2k2r/ppp1n1pp/1b6/8/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bd6"),
            "rN2k2r/ppp1n1pp/3b4/8/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("a6"),
            "rN2k2r/1pp1n1pp/p7/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 0 9",
        ),
        (
            Move::from_str("a5"),
            "rN2k2r/1pp1n1pp/8/p1b5/8/2P5/PP2BPPP/R3K2R w KQkq a6 0 9",
        ),
        (
            Move::from_str("b6"),
            "rN2k2r/p1p1n1pp/1p6/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 0 9",
        ),
        (
            Move::from_str("b5"),
            "rN2k2r/p1p1n1pp/8/1pb5/8/2P5/PP2BPPP/R3K2R w KQkq b6 0 9",
        ),
        (
            Move::from_str("c6"),
            "rN2k2r/pp2n1pp/2p5/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 0 9",
        ),
        (
            Move::from_str("Nd5"),
            "rN2k2r/ppp3pp/8/2bn4/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Nf5"),
            "rN2k2r/ppp3pp/8/2b2n2/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Nc6"),
            "rN2k2r/ppp3pp/2n5/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Ng6"),
            "rN2k2r/ppp3pp/6n1/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Nc8"),
            "rNn1k2r/ppp3pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Ng8"),
            "rN2k1nr/ppp3pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("g6"),
            "rN2k2r/ppp1n2p/6p1/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 0 9",
        ),
        (
            Move::from_str("g5"),
            "rN2k2r/ppp1n2p/8/2b3p1/8/2P5/PP2BPPP/R3K2R w KQkq g6 0 9",
        ),
        (
            Move::from_str("h6"),
            "rN2k2r/ppp1n1p1/7p/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 0 9",
        ),
        (
            Move::from_str("h5"),
            "rN2k2r/ppp1n1p1/8/2b4p/8/2P5/PP2BPPP/R3K2R w KQkq h6 0 9",
        ),
        (
            Move::from_str("Rxb8"),
            "1r2k2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQk - 0 9",
        ),
        (
            Move::from_str("Kf7"),
            "rN5r/ppp1nkpp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQ - 2 9",
        ),
        (
            Move::from_str("Kd8"),
            "rN1k3r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQ - 2 9",
        ),
        (
            Move::from_str("Kf8"),
            "rN3k1r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQ - 2 9",
        ),
        (
            Move::from_str("O-O"),
            "rN3rk1/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQ - 2 9",
        ),
        (
            Move::from_str("Rg8"),
            "rN2k1r1/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQq - 2 9",
        ),
        (
            Move::from_str("Rf8"),
            "rN2kr2/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQq - 2 9",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_53() {
    let position = "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/R1n1K2R w KQkq - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("Rb1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/1Rn1K2R b Kkq - 2 8",
        ),
        (
            Move::from_str("Rxc1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/2R1K2R b Kkq - 0 8",
        ),
        (
            Move::from_str("Kd1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/R1nK3R b kq - 2 8",
        ),
        (
            Move::from_str("Kf1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/R1n2K1R b kq - 2 8",
        ),
        (
            Move::from_str("Kd2"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPPKN1PP/R1n4R b kq - 2 8",
        ),
        (
            Move::from_str("Kf2"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1NKPP/R1n4R b kq - 2 8",
        ),
        (
            Move::from_str("O-O"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/R1n2RK1 b kq - 2 8",
        ),
        (
            Move::from_str("Rg1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/R1n1K1R1 b Qkq - 2 8",
        ),
        (
            Move::from_str("Rf1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/R1n1KR2 b Qkq - 2 8",
        ),
        (
            Move::from_str("a3"),
            "r3k2r/pp2bppp/2p5/8/2B5/P7/1PP1N1PP/R1n1K2R b KQkq - 0 8",
        ),
        (
            Move::from_str("a4"),
            "r3k2r/pp2bppp/2p5/8/P1B5/8/1PP1N1PP/R1n1K2R b KQkq a3 0 8",
        ),
        (
            Move::from_str("b3"),
            "r3k2r/pp2bppp/2p5/8/2B5/1P6/P1P1N1PP/R1n1K2R b KQkq - 0 8",
        ),
        (
            Move::from_str("b4"),
            "r3k2r/pp2bppp/2p5/8/1PB5/8/P1P1N1PP/R1n1K2R b KQkq b3 0 8",
        ),
        (
            Move::from_str("c3"),
            "r3k2r/pp2bppp/2p5/8/2B5/2P5/PP2N1PP/R1n1K2R b KQkq - 0 8",
        ),
        (
            Move::from_str("Nxc1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP3PP/R1N1K2R b KQkq - 0 8",
        ),
        (
            Move::from_str("Ng1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP3PP/R1n1K1NR b KQkq - 2 8",
        ),
        (
            Move::from_str("Nc3"),
            "r3k2r/pp2bppp/2p5/8/2B5/2N5/PPP3PP/R1n1K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Ng3"),
            "r3k2r/pp2bppp/2p5/8/2B5/6N1/PPP3PP/R1n1K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Nd4"),
            "r3k2r/pp2bppp/2p5/8/2BN4/8/PPP3PP/R1n1K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Nf4"),
            "r3k2r/pp2bppp/2p5/8/2B2N2/8/PPP3PP/R1n1K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("g3"),
            "r3k2r/pp2bppp/2p5/8/2B5/6P1/PPP1N2P/R1n1K2R b KQkq - 0 8",
        ),
        (
            Move::from_str("g4"),
            "r3k2r/pp2bppp/2p5/8/2B3P1/8/PPP1N2P/R1n1K2R b KQkq g3 0 8",
        ),
        (
            Move::from_str("h3"),
            "r3k2r/pp2bppp/2p5/8/2B5/7P/PPP1N1P1/R1n1K2R b KQkq - 0 8",
        ),
        (
            Move::from_str("h4"),
            "r3k2r/pp2bppp/2p5/8/2B4P/8/PPP1N1P1/R1n1K2R b KQkq h3 0 8",
        ),
        (
            Move::from_str("Bb3"),
            "r3k2r/pp2bppp/2p5/8/8/1B6/PPP1N1PP/R1n1K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bd3"),
            "r3k2r/pp2bppp/2p5/8/8/3B4/PPP1N1PP/R1n1K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bb5"),
            "r3k2r/pp2bppp/2p5/1B6/8/8/PPP1N1PP/R1n1K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Ba6"),
            "r3k2r/pp2bppp/B1p5/8/8/8/PPP1N1PP/R1n1K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bd5"),
            "r3k2r/pp2bppp/2p5/3B4/8/8/PPP1N1PP/R1n1K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Be6"),
            "r3k2r/pp2bppp/2p1B3/8/8/8/PPP1N1PP/R1n1K2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bxf7+"),
            "r3k2r/pp2bBpp/2p5/8/8/8/PPP1N1PP/R1n1K2R b KQkq - 0 8",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_54() {
    let position = "r1N1k2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R b KQkq - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("Bb4"),
            "r1N1k2r/ppp1n1pp/8/8/1b6/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Ba3"),
            "r1N1k2r/ppp1n1pp/8/8/8/b1P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bd4"),
            "r1N1k2r/ppp1n1pp/8/8/3b4/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Be3"),
            "r1N1k2r/ppp1n1pp/8/8/8/2P1b3/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bxf2+"),
            "r1N1k2r/ppp1n1pp/8/8/8/2P5/PP2BbPP/R3K2R w KQkq - 0 9",
        ),
        (
            Move::from_str("Bb6"),
            "r1N1k2r/ppp1n1pp/1b6/8/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bd6"),
            "r1N1k2r/ppp1n1pp/3b4/8/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("a6"),
            "r1N1k2r/1pp1n1pp/p7/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 0 9",
        ),
        (
            Move::from_str("a5"),
            "r1N1k2r/1pp1n1pp/8/p1b5/8/2P5/PP2BPPP/R3K2R w KQkq a6 0 9",
        ),
        (
            Move::from_str("b6"),
            "r1N1k2r/p1p1n1pp/1p6/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 0 9",
        ),
        (
            Move::from_str("b5"),
            "r1N1k2r/p1p1n1pp/8/1pb5/8/2P5/PP2BPPP/R3K2R w KQkq b6 0 9",
        ),
        (
            Move::from_str("c6"),
            "r1N1k2r/pp2n1pp/2p5/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 0 9",
        ),
        (
            Move::from_str("Nd5"),
            "r1N1k2r/ppp3pp/8/2bn4/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Nf5"),
            "r1N1k2r/ppp3pp/8/2b2n2/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Nc6"),
            "r1N1k2r/ppp3pp/2n5/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Ng6"),
            "r1N1k2r/ppp3pp/6n1/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Nxc8"),
            "r1n1k2r/ppp3pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 0 9",
        ),
        (
            Move::from_str("Ng8"),
            "r1N1k1nr/ppp3pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("g6"),
            "r1N1k2r/ppp1n2p/6p1/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 0 9",
        ),
        (
            Move::from_str("g5"),
            "r1N1k2r/ppp1n2p/8/2b3p1/8/2P5/PP2BPPP/R3K2R w KQkq g6 0 9",
        ),
        (
            Move::from_str("h6"),
            "r1N1k2r/ppp1n1p1/7p/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 0 9",
        ),
        (
            Move::from_str("h5"),
            "r1N1k2r/ppp1n1p1/8/2b4p/8/2P5/PP2BPPP/R3K2R w KQkq h6 0 9",
        ),
        (
            Move::from_str("Rb8"),
            "1rN1k2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQk - 2 9",
        ),
        (
            Move::from_str("Rxc8"),
            "2r1k2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQk - 0 9",
        ),
        (
            Move::from_str("Kd7"),
            "r1N4r/pppkn1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQ - 2 9",
        ),
        (
            Move::from_str("Kf7"),
            "r1N4r/ppp1nkpp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQ - 2 9",
        ),
        (
            Move::from_str("Kd8"),
            "r1Nk3r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQ - 2 9",
        ),
        (
            Move::from_str("Kf8"),
            "r1N2k1r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQ - 2 9",
        ),
        (
            Move::from_str("O-O"),
            "r1N2rk1/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQ - 2 9",
        ),
        (
            Move::from_str("Rg8"),
            "r1N1k1r1/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQq - 2 9",
        ),
        (
            Move::from_str("Rf8"),
            "r1N1kr2/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQq - 2 9",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_55() {
    let position = "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/R2nK2R w KQkq - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("Rb1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/1R1nK2R b Kkq - 2 8",
        ),
        (
            Move::from_str("Rc1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/2RnK2R b Kkq - 2 8",
        ),
        (
            Move::from_str("Rxd1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/3RK2R b Kkq - 0 8",
        ),
        (
            Move::from_str("Kxd1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/R2K3R b kq - 0 8",
        ),
        (
            Move::from_str("Kf1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/R2n1K1R b kq - 2 8",
        ),
        (
            Move::from_str("Kd2"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPPKN1PP/R2n3R b kq - 2 8",
        ),
        (
            Move::from_str("O-O"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/R2n1RK1 b kq - 2 8",
        ),
        (
            Move::from_str("Rg1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/R2nK1R1 b Qkq - 2 8",
        ),
        (
            Move::from_str("Rf1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/R2nKR2 b Qkq - 2 8",
        ),
        (
            Move::from_str("a3"),
            "r3k2r/pp2bppp/2p5/8/2B5/P7/1PP1N1PP/R2nK2R b KQkq - 0 8",
        ),
        (
            Move::from_str("a4"),
            "r3k2r/pp2bppp/2p5/8/P1B5/8/1PP1N1PP/R2nK2R b KQkq a3 0 8",
        ),
        (
            Move::from_str("b3"),
            "r3k2r/pp2bppp/2p5/8/2B5/1P6/P1P1N1PP/R2nK2R b KQkq - 0 8",
        ),
        (
            Move::from_str("b4"),
            "r3k2r/pp2bppp/2p5/8/1PB5/8/P1P1N1PP/R2nK2R b KQkq b3 0 8",
        ),
        (
            Move::from_str("c3"),
            "r3k2r/pp2bppp/2p5/8/2B5/2P5/PP2N1PP/R2nK2R b KQkq - 0 8",
        ),
        (
            Move::from_str("Nc1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP3PP/R1NnK2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Ng1"),
            "r3k2r/pp2bppp/2p5/8/2B5/8/PPP3PP/R2nK1NR b KQkq - 2 8",
        ),
        (
            Move::from_str("Nc3"),
            "r3k2r/pp2bppp/2p5/8/2B5/2N5/PPP3PP/R2nK2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Ng3"),
            "r3k2r/pp2bppp/2p5/8/2B5/6N1/PPP3PP/R2nK2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Nd4"),
            "r3k2r/pp2bppp/2p5/8/2BN4/8/PPP3PP/R2nK2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Nf4"),
            "r3k2r/pp2bppp/2p5/8/2B2N2/8/PPP3PP/R2nK2R b KQkq - 2 8",
        ),
        (
            Move::from_str("g3"),
            "r3k2r/pp2bppp/2p5/8/2B5/6P1/PPP1N2P/R2nK2R b KQkq - 0 8",
        ),
        (
            Move::from_str("g4"),
            "r3k2r/pp2bppp/2p5/8/2B3P1/8/PPP1N2P/R2nK2R b KQkq g3 0 8",
        ),
        (
            Move::from_str("h3"),
            "r3k2r/pp2bppp/2p5/8/2B5/7P/PPP1N1P1/R2nK2R b KQkq - 0 8",
        ),
        (
            Move::from_str("h4"),
            "r3k2r/pp2bppp/2p5/8/2B4P/8/PPP1N1P1/R2nK2R b KQkq h3 0 8",
        ),
        (
            Move::from_str("Bb3"),
            "r3k2r/pp2bppp/2p5/8/8/1B6/PPP1N1PP/R2nK2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bd3"),
            "r3k2r/pp2bppp/2p5/8/8/3B4/PPP1N1PP/R2nK2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bb5"),
            "r3k2r/pp2bppp/2p5/1B6/8/8/PPP1N1PP/R2nK2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Ba6"),
            "r3k2r/pp2bppp/B1p5/8/8/8/PPP1N1PP/R2nK2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bd5"),
            "r3k2r/pp2bppp/2p5/3B4/8/8/PPP1N1PP/R2nK2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Be6"),
            "r3k2r/pp2bppp/2p1B3/8/8/8/PPP1N1PP/R2nK2R b KQkq - 2 8",
        ),
        (
            Move::from_str("Bxf7+"),
            "r3k2r/pp2bBpp/2p5/8/8/8/PPP1N1PP/R2nK2R b KQkq - 0 8",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_56() {
    let position = "r2Nk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R b KQkq - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("Bb4"),
            "r2Nk2r/ppp1n1pp/8/8/1b6/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Ba3"),
            "r2Nk2r/ppp1n1pp/8/8/8/b1P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bd4"),
            "r2Nk2r/ppp1n1pp/8/8/3b4/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Be3"),
            "r2Nk2r/ppp1n1pp/8/8/8/2P1b3/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bxf2+"),
            "r2Nk2r/ppp1n1pp/8/8/8/2P5/PP2BbPP/R3K2R w KQkq - 0 9",
        ),
        (
            Move::from_str("Bb6"),
            "r2Nk2r/ppp1n1pp/1b6/8/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Bd6"),
            "r2Nk2r/ppp1n1pp/3b4/8/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("a6"),
            "r2Nk2r/1pp1n1pp/p7/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 0 9",
        ),
        (
            Move::from_str("a5"),
            "r2Nk2r/1pp1n1pp/8/p1b5/8/2P5/PP2BPPP/R3K2R w KQkq a6 0 9",
        ),
        (
            Move::from_str("b6"),
            "r2Nk2r/p1p1n1pp/1p6/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 0 9",
        ),
        (
            Move::from_str("b5"),
            "r2Nk2r/p1p1n1pp/8/1pb5/8/2P5/PP2BPPP/R3K2R w KQkq b6 0 9",
        ),
        (
            Move::from_str("c6"),
            "r2Nk2r/pp2n1pp/2p5/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 0 9",
        ),
        (
            Move::from_str("Nd5"),
            "r2Nk2r/ppp3pp/8/2bn4/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Nf5"),
            "r2Nk2r/ppp3pp/8/2b2n2/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Nc6"),
            "r2Nk2r/ppp3pp/2n5/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Ng6"),
            "r2Nk2r/ppp3pp/6n1/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Nc8"),
            "r1nNk2r/ppp3pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("Ng8"),
            "r2Nk1nr/ppp3pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 2 9",
        ),
        (
            Move::from_str("g6"),
            "r2Nk2r/ppp1n2p/6p1/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 0 9",
        ),
        (
            Move::from_str("g5"),
            "r2Nk2r/ppp1n2p/8/2b3p1/8/2P5/PP2BPPP/R3K2R w KQkq g6 0 9",
        ),
        (
            Move::from_str("h6"),
            "r2Nk2r/ppp1n1p1/7p/2b5/8/2P5/PP2BPPP/R3K2R w KQkq - 0 9",
        ),
        (
            Move::from_str("h5"),
            "r2Nk2r/ppp1n1p1/8/2b4p/8/2P5/PP2BPPP/R3K2R w KQkq h6 0 9",
        ),
        (
            Move::from_str("Rb8"),
            "1r1Nk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQk - 2 9",
        ),
        (
            Move::from_str("Rc8"),
            "2rNk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQk - 2 9",
        ),
        (
            Move::from_str("Rxd8"),
            "3rk2r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQk - 0 9",
        ),
        (
            Move::from_str("Kd7"),
            "r2N3r/pppkn1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQ - 2 9",
        ),
        (
            Move::from_str("Kxd8"),
            "r2k3r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQ - 0 9",
        ),
        (
            Move::from_str("Kf8"),
            "r2N1k1r/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQ - 2 9",
        ),
        (
            Move::from_str("O-O"),
            "r2N1rk1/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQ - 2 9",
        ),
        (
            Move::from_str("Rg8"),
            "r2Nk1r1/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQq - 2 9",
        ),
        (
            Move::from_str("Rf8"),
            "r2Nkr2/ppp1n1pp/8/2b5/8/2P5/PP2BPPP/R3K2R w KQq - 2 9",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
#[test]
fn castling_57() {
    let position = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R b KQ - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("Nxd1"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1N1PP/RNBnK2R w KQ - 0 9",
        ),
        (
            Move::from_str("Nxh1"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2n w Q - 0 9",
        ),
        (
            Move::from_str("Nd3+"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/3n4/PPP1N1PP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("Nh3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/7n/PPP1N1PP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("Ne4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B1n3/8/PPP1N1PP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("Ng4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B3n1/8/PPP1N1PP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("c5"),
            "rnbq1k1r/pp1Pbppp/8/2p5/2B5/8/PPP1NnPP/RNBQK2R w KQ - 0 9",
        ),
        (
            Move::from_str("a6"),
            "rnbq1k1r/1p1Pbppp/p1p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 0 9",
        ),
        (
            Move::from_str("a5"),
            "rnbq1k1r/1p1Pbppp/2p5/p7/2B5/8/PPP1NnPP/RNBQK2R w KQ a6 0 9",
        ),
        (
            Move::from_str("b6"),
            "rnbq1k1r/p2Pbppp/1pp5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 0 9",
        ),
        (
            Move::from_str("b5"),
            "rnbq1k1r/p2Pbppp/2p5/1p6/2B5/8/PPP1NnPP/RNBQK2R w KQ b6 0 9",
        ),
        (
            Move::from_str("Bd6"),
            "rnbq1k1r/pp1P1ppp/2pb4/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("Bc5"),
            "rnbq1k1r/pp1P1ppp/2p5/2b5/2B5/8/PPP1NnPP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("Bb4+"),
            "rnbq1k1r/pp1P1ppp/2p5/8/1bB5/8/PPP1NnPP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("Ba3"),
            "rnbq1k1r/pp1P1ppp/2p5/8/2B5/b7/PPP1NnPP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("Bf6"),
            "rnbq1k1r/pp1P1ppp/2p2b2/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("Bg5"),
            "rnbq1k1r/pp1P1ppp/2p5/6b1/2B5/8/PPP1NnPP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("Bh4"),
            "rnbq1k1r/pp1P1ppp/2p5/8/2B4b/8/PPP1NnPP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("f6"),
            "rnbq1k1r/pp1Pb1pp/2p2p2/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 0 9",
        ),
        (
            Move::from_str("f5"),
            "rnbq1k1r/pp1Pb1pp/2p5/5p2/2B5/8/PPP1NnPP/RNBQK2R w KQ f6 0 9",
        ),
        (
            Move::from_str("g6"),
            "rnbq1k1r/pp1Pbp1p/2p3p1/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 0 9",
        ),
        (
            Move::from_str("g5"),
            "rnbq1k1r/pp1Pbp1p/2p5/6p1/2B5/8/PPP1NnPP/RNBQK2R w KQ g6 0 9",
        ),
        (
            Move::from_str("h6"),
            "rnbq1k1r/pp1Pbpp1/2p4p/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 0 9",
        ),
        (
            Move::from_str("h5"),
            "rnbq1k1r/pp1Pbpp1/2p5/7p/2B5/8/PPP1NnPP/RNBQK2R w KQ h6 0 9",
        ),
        (
            Move::from_str("Na6"),
            "r1bq1k1r/pp1Pbppp/n1p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("Nxd7"),
            "r1bq1k1r/pp1nbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 0 9",
        ),
        (
            Move::from_str("Bxd7"),
            "rn1q1k1r/pp1bbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 0 9",
        ),
        (
            Move::from_str("Qc7"),
            "rnb2k1r/ppqPbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("Qb6"),
            "rnb2k1r/pp1Pbppp/1qp5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("Qa5+"),
            "rnb2k1r/pp1Pbppp/2p5/q7/2B5/8/PPP1NnPP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("Qxd7"),
            "rnb2k1r/pp1qbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 0 9",
        ),
        (
            Move::from_str("Qe8"),
            "rnb1qk1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("Kg8"),
            "rnbq2kr/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 2 9",
        ),
        (
            Move::from_str("Rg8"),
            "rnbq1kr1/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 2 9",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}

#[test]
fn castling_58() {
    let position = "rnbqk2r/ppp1nNpp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R w kq - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("Nxd2"),
            "rnbqk2r/ppp1nNpp/8/2b5/8/2P5/PP1NBPPP/R1BQ1K1R b kq - 0 8",
        ),
        (
            Move::from_str("Na3"),
            "rnbqk2r/ppp1nNpp/8/2b5/8/N1P5/PP1pBPPP/R1BQ1K1R b kq - 2 8",
        ),
        (
            Move::from_str("Bxd2"),
            "rnbqk2r/ppp1nNpp/8/2b5/8/2P5/PP1BBPPP/RN1Q1K1R b kq - 0 8",
        ),
        (
            Move::from_str("Qe1"),
            "rnbqk2r/ppp1nNpp/8/2b5/8/2P5/PP1pBPPP/RNB1QK1R b kq - 2 8",
        ),
        (
            Move::from_str("Qc2"),
            "rnbqk2r/ppp1nNpp/8/2b5/8/2P5/PPQpBPPP/RNB2K1R b kq - 2 8",
        ),
        (
            Move::from_str("Qb3"),
            "rnbqk2r/ppp1nNpp/8/2b5/8/1QP5/PP1pBPPP/RNB2K1R b kq - 2 8",
        ),
        (
            Move::from_str("Qa4+"),
            "rnbqk2r/ppp1nNpp/8/2b5/Q7/2P5/PP1pBPPP/RNB2K1R b kq - 2 8",
        ),
        (
            Move::from_str("Qxd2"),
            "rnbqk2r/ppp1nNpp/8/2b5/8/2P5/PP1QBPPP/RNB2K1R b kq - 0 8",
        ),
        (
            Move::from_str("Kg1"),
            "rnbqk2r/ppp1nNpp/8/2b5/8/2P5/PP1pBPPP/RNBQ2KR b kq - 2 8",
        ),
        (
            Move::from_str("Rg1"),
            "rnbqk2r/ppp1nNpp/8/2b5/8/2P5/PP1pBPPP/RNBQ1KR1 b kq - 2 8",
        ),
        (
            Move::from_str("a3"),
            "rnbqk2r/ppp1nNpp/8/2b5/8/P1P5/1P1pBPPP/RNBQ1K1R b kq - 0 8",
        ),
        (
            Move::from_str("a4"),
            "rnbqk2r/ppp1nNpp/8/2b5/P7/2P5/1P1pBPPP/RNBQ1K1R b kq a3 0 8",
        ),
        (
            Move::from_str("b3"),
            "rnbqk2r/ppp1nNpp/8/2b5/8/1PP5/P2pBPPP/RNBQ1K1R b kq - 0 8",
        ),
        (
            Move::from_str("b4"),
            "rnbqk2r/ppp1nNpp/8/2b5/1P6/2P5/P2pBPPP/RNBQ1K1R b kq b3 0 8",
        ),
        (
            Move::from_str("Bd3"),
            "rnbqk2r/ppp1nNpp/8/2b5/8/2PB4/PP1p1PPP/RNBQ1K1R b kq - 2 8",
        ),
        (
            Move::from_str("Bc4"),
            "rnbqk2r/ppp1nNpp/8/2b5/2B5/2P5/PP1p1PPP/RNBQ1K1R b kq - 2 8",
        ),
        (
            Move::from_str("Bb5+"),
            "rnbqk2r/ppp1nNpp/8/1Bb5/8/2P5/PP1p1PPP/RNBQ1K1R b kq - 2 8",
        ),
        (
            Move::from_str("Ba6"),
            "rnbqk2r/ppp1nNpp/B7/2b5/8/2P5/PP1p1PPP/RNBQ1K1R b kq - 2 8",
        ),
        (
            Move::from_str("Bf3"),
            "rnbqk2r/ppp1nNpp/8/2b5/8/2P2B2/PP1p1PPP/RNBQ1K1R b kq - 2 8",
        ),
        (
            Move::from_str("Bg4"),
            "rnbqk2r/ppp1nNpp/8/2b5/6B1/2P5/PP1p1PPP/RNBQ1K1R b kq - 2 8",
        ),
        (
            Move::from_str("Bh5"),
            "rnbqk2r/ppp1nNpp/8/2b4B/8/2P5/PP1p1PPP/RNBQ1K1R b kq - 2 8",
        ),
        (
            Move::from_str("f3"),
            "rnbqk2r/ppp1nNpp/8/2b5/8/2P2P2/PP1pB1PP/RNBQ1K1R b kq - 0 8",
        ),
        (
            Move::from_str("f4"),
            "rnbqk2r/ppp1nNpp/8/2b5/5P2/2P5/PP1pB1PP/RNBQ1K1R b kq f3 0 8",
        ),
        (
            Move::from_str("g3"),
            "rnbqk2r/ppp1nNpp/8/2b5/8/2P3P1/PP1pBP1P/RNBQ1K1R b kq - 0 8",
        ),
        (
            Move::from_str("g4"),
            "rnbqk2r/ppp1nNpp/8/2b5/6P1/2P5/PP1pBP1P/RNBQ1K1R b kq g3 0 8",
        ),
        (
            Move::from_str("h3"),
            "rnbqk2r/ppp1nNpp/8/2b5/8/2P4P/PP1pBPP1/RNBQ1K1R b kq - 0 8",
        ),
        (
            Move::from_str("h4"),
            "rnbqk2r/ppp1nNpp/8/2b5/7P/2P5/PP1pBPP1/RNBQ1K1R b kq h3 0 8",
        ),
        (
            Move::from_str("c4"),
            "rnbqk2r/ppp1nNpp/8/2b5/2P5/8/PP1pBPPP/RNBQ1K1R b kq - 0 8",
        ),
        (
            Move::from_str("Ne5"),
            "rnbqk2r/ppp1n1pp/8/2b1N3/8/2P5/PP1pBPPP/RNBQ1K1R b kq - 2 8",
        ),
        (
            Move::from_str("Ng5"),
            "rnbqk2r/ppp1n1pp/8/2b3N1/8/2P5/PP1pBPPP/RNBQ1K1R b kq - 2 8",
        ),
        (
            Move::from_str("Nd6+"),
            "rnbqk2r/ppp1n1pp/3N4/2b5/8/2P5/PP1pBPPP/RNBQ1K1R b kq - 2 8",
        ),
        (
            Move::from_str("Nh6"),
            "rnbqk2r/ppp1n1pp/7N/2b5/8/2P5/PP1pBPPP/RNBQ1K1R b kq - 2 8",
        ),
        (
            Move::from_str("Nxd8"),
            "rnbNk2r/ppp1n1pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R b kq - 0 8",
        ),
        (
            Move::from_str("Nxh8"),
            "rnbqk2N/ppp1n1pp/8/2b5/8/2P5/PP1pBPPP/RNBQ1K1R b q - 0 8",
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
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
