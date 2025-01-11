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
