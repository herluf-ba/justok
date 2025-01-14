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

#[test]
fn castling_1() {
    let position = "8/4k3/8/8/8/8/r6r/R3K2R w KQ - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("a1b1"), "8/4k3/8/8/8/8/r6r/1R2K2R b K - 1 1"),
        (Move::from_str("a1c1"), "8/4k3/8/8/8/8/r6r/2R1K2R b K - 1 1"),
        (Move::from_str("a1d1"), "8/4k3/8/8/8/8/r6r/3RK2R b K - 1 1"),
        (Move::from_str("a1a2"), "8/4k3/8/8/8/8/R6r/4K2R b K - 0 1"),
        (Move::from_str("e1d1"), "8/4k3/8/8/8/8/r6r/R2K3R b - - 1 1"),
        (Move::from_str("e1f1"), "8/4k3/8/8/8/8/r6r/R4K1R b - - 1 1"),
        (Move::from_str("e1c1"), "8/4k3/8/8/8/8/r6r/2KR3R b - - 1 1"),
        (Move::from_str("e1g1"), "8/4k3/8/8/8/8/r6r/R4RK1 b - - 1 1"),
        (Move::from_str("h1g1"), "8/4k3/8/8/8/8/r6r/R3K1R1 b Q - 1 1"),
        (Move::from_str("h1f1"), "8/4k3/8/8/8/8/r6r/R3KR2 b Q - 1 1"),
        (Move::from_str("h1h2"), "8/4k3/8/8/8/8/r6R/R3K3 b Q - 0 1"),
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
        (Move::from_str("a8a7"), "4k2r/r6R/8/8/8/8/4K3/8 w k - 0 2"),
        (Move::from_str("a8b8"), "1r2k2r/R6R/8/8/8/8/4K3/8 w k - 1 2"),
        (Move::from_str("a8c8"), "2r1k2r/R6R/8/8/8/8/4K3/8 w k - 1 2"),
        (Move::from_str("a8d8"), "3rk2r/R6R/8/8/8/8/4K3/8 w k - 1 2"),
        (Move::from_str("e8d8"), "r2k3r/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("e8f8"), "r4k1r/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("e8c8"), "2kr3r/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("e8g8"), "r4rk1/R6R/8/8/8/8/4K3/8 w - - 1 2"),
        (Move::from_str("h8h7"), "r3k3/R6r/8/8/8/8/4K3/8 w q - 0 2"),
        (Move::from_str("h8g8"), "r3k1r1/R6R/8/8/8/8/4K3/8 w q - 1 2"),
        (Move::from_str("h8f8"), "r3kr2/R6R/8/8/8/8/4K3/8 w q - 1 2"),
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
        (Move::from_str("a1b1"), "8/4k3/8/8/8/8/r6r/1R2K2R b - - 1 1"),
        (Move::from_str("a1c1"), "8/4k3/8/8/8/8/r6r/2R1K2R b - - 1 1"),
        (Move::from_str("a1d1"), "8/4k3/8/8/8/8/r6r/3RK2R b - - 1 1"),
        (Move::from_str("a1a2"), "8/4k3/8/8/8/8/R6r/4K2R b - - 0 1"),
        (Move::from_str("e1d1"), "8/4k3/8/8/8/8/r6r/R2K3R b - - 1 1"),
        (Move::from_str("e1f1"), "8/4k3/8/8/8/8/r6r/R4K1R b - - 1 1"),
        (Move::from_str("e1c1"), "8/4k3/8/8/8/8/r6r/2KR3R b - - 1 1"),
        (Move::from_str("h1g1"), "8/4k3/8/8/8/8/r6r/R3K1R1 b Q - 1 1"),
        (Move::from_str("h1f1"), "8/4k3/8/8/8/8/r6r/R3KR2 b Q - 1 1"),
        (Move::from_str("h1h2"), "8/4k3/8/8/8/8/r6R/R3K3 b Q - 0 1"),
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
    let position = "8/8/4k3/8/8/8/2p3p1/R3K2R w KQ - 0 1";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("a1b1"),
            "8/8/4k3/8/8/8/2p3p1/1R2K2R b K - 1 1",
        ),
        (
            Move::from_str("a1c1"),
            "8/8/4k3/8/8/8/2p3p1/2R1K2R b K - 1 1",
        ),
        (
            Move::from_str("a1d1"),
            "8/8/4k3/8/8/8/2p3p1/3RK2R b K - 1 1",
        ),
        (
            Move::from_str("a1a2"),
            "8/8/4k3/8/8/8/R1p3p1/4K2R b K - 1 1",
        ),
        (
            Move::from_str("a1a3"),
            "8/8/4k3/8/8/R7/2p3p1/4K2R b K - 1 1",
        ),
        (
            Move::from_str("a1a4"),
            "8/8/4k3/8/R7/8/2p3p1/4K2R b K - 1 1",
        ),
        (
            Move::from_str("a1a5"),
            "8/8/4k3/R7/8/8/2p3p1/4K2R b K - 1 1",
        ),
        (
            Move::from_str("a1a6"),
            "8/8/R3k3/8/8/8/2p3p1/4K2R b K - 1 1",
        ),
        (
            Move::from_str("a1a7"),
            "8/R7/4k3/8/8/8/2p3p1/4K2R b K - 1 1",
        ),
        (
            Move::from_str("a1a8"),
            "R7/8/4k3/8/8/8/2p3p1/4K2R b K - 1 1",
        ),
        (Move::from_str("e1d2"), "8/8/4k3/8/8/8/2pK2p1/R6R b - - 1 1"),
        (
            Move::from_str("e1e2"),
            "8/8/4k3/8/8/8/2p1K1p1/R6R b - - 1 1",
        ),
        (Move::from_str("e1f2"), "8/8/4k3/8/8/8/2p2Kp1/R6R b - - 1 1"),
        (
            Move::from_str("h1g1"),
            "8/8/4k3/8/8/8/2p3p1/R3K1R1 b Q - 1 1",
        ),
        (
            Move::from_str("h1f1"),
            "8/8/4k3/8/8/8/2p3p1/R3KR2 b Q - 1 1",
        ),
        (Move::from_str("h1h2"), "8/8/4k3/8/8/8/2p3pR/R3K3 b Q - 1 1"),
        (
            Move::from_str("h1h3"),
            "8/8/4k3/8/8/7R/2p3p1/R3K3 b Q - 1 1",
        ),
        (
            Move::from_str("h1h4"),
            "8/8/4k3/8/7R/8/2p3p1/R3K3 b Q - 1 1",
        ),
        (
            Move::from_str("h1h5"),
            "8/8/4k3/7R/8/8/2p3p1/R3K3 b Q - 1 1",
        ),
        (
            Move::from_str("h1h6"),
            "8/8/4k2R/8/8/8/2p3p1/R3K3 b Q - 1 1",
        ),
        (
            Move::from_str("h1h7"),
            "8/7R/4k3/8/8/8/2p3p1/R3K3 b Q - 1 1",
        ),
        (
            Move::from_str("h1h8"),
            "7R/8/4k3/8/8/8/2p3p1/R3K3 b Q - 1 1",
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
fn castling_5() {
    let position = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2n w Q - 1 8";

    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("b1d2"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPPNN1PP/R1BQK2n b Q - 2 8",
        ),
        (
            Move::from_str("b1a3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/N7/PPP1N1PP/R1BQK2n b Q - 2 8",
        ),
        (
            Move::from_str("b1c3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/2N5/PPP1N1PP/R1BQK2n b Q - 2 8",
        ),
        (
            Move::from_str("c1d2"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPPBN1PP/RN1QK2n b Q - 2 8",
        ),
        (
            Move::from_str("c1e3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/4B3/PPP1N1PP/RN1QK2n b Q - 2 8",
        ),
        (
            Move::from_str("c1f4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B2B2/8/PPP1N1PP/RN1QK2n b Q - 2 8",
        ),
        (
            Move::from_str("c1g5"),
            "rnbq1k1r/pp1Pbppp/2p5/6B1/2B5/8/PPP1N1PP/RN1QK2n b Q - 2 8",
        ),
        (
            Move::from_str("c1h6"),
            "rnbq1k1r/pp1Pbppp/2p4B/8/2B5/8/PPP1N1PP/RN1QK2n b Q - 2 8",
        ),
        (
            Move::from_str("d1d2"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPPQN1PP/RNB1K2n b Q - 2 8",
        ),
        (
            Move::from_str("d1d3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/3Q4/PPP1N1PP/RNB1K2n b Q - 2 8",
        ),
        (
            Move::from_str("d1d4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2BQ4/8/PPP1N1PP/RNB1K2n b Q - 2 8",
        ),
        (
            Move::from_str("d1d5"),
            "rnbq1k1r/pp1Pbppp/2p5/3Q4/2B5/8/PPP1N1PP/RNB1K2n b Q - 2 8",
        ),
        (
            Move::from_str("d1d6"),
            "rnbq1k1r/pp1Pbppp/2pQ4/8/2B5/8/PPP1N1PP/RNB1K2n b Q - 2 8",
        ),
        (
            Move::from_str("e1f1"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1N1PP/RNBQ1K1n b - - 2 8",
        ),
        (
            Move::from_str("e1d2"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPPKN1PP/RNBQ3n b - - 2 8",
        ),
        (
            Move::from_str("a2a3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/P7/1PP1N1PP/RNBQK2n b Q - 0 8",
        ),
        (
            Move::from_str("a2a4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/P1B5/8/1PP1N1PP/RNBQK2n b Q a3 0 8",
        ),
        (
            Move::from_str("b2b3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/1P6/P1P1N1PP/RNBQK2n b Q - 0 8",
        ),
        (
            Move::from_str("b2b4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/1PB5/8/P1P1N1PP/RNBQK2n b Q b3 0 8",
        ),
        (
            Move::from_str("c2c3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/2P5/PP2N1PP/RNBQK2n b Q - 0 8",
        ),
        (
            Move::from_str("e2g1"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP3PP/RNBQK1Nn b Q - 2 8",
        ),
        (
            Move::from_str("e2c3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/2N5/PPP3PP/RNBQK2n b Q - 2 8",
        ),
        (
            Move::from_str("e2g3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/6N1/PPP3PP/RNBQK2n b Q - 2 8",
        ),
        (
            Move::from_str("e2d4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2BN4/8/PPP3PP/RNBQK2n b Q - 2 8",
        ),
        (
            Move::from_str("e2f4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B2N2/8/PPP3PP/RNBQK2n b Q - 2 8",
        ),
        (
            Move::from_str("g2g3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/6P1/PPP1N2P/RNBQK2n b Q - 0 8",
        ),
        (
            Move::from_str("g2g4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B3P1/8/PPP1N2P/RNBQK2n b Q g3 0 8",
        ),
        (
            Move::from_str("h2h3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/7P/PPP1N1P1/RNBQK2n b Q - 0 8",
        ),
        (
            Move::from_str("h2h4"),
            "rnbq1k1r/pp1Pbppp/2p5/8/2B4P/8/PPP1N1P1/RNBQK2n b Q h3 0 8",
        ),
        (
            Move::from_str("c4b3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/8/1B6/PPP1N1PP/RNBQK2n b Q - 2 8",
        ),
        (
            Move::from_str("c4d3"),
            "rnbq1k1r/pp1Pbppp/2p5/8/8/3B4/PPP1N1PP/RNBQK2n b Q - 2 8",
        ),
        (
            Move::from_str("c4b5"),
            "rnbq1k1r/pp1Pbppp/2p5/1B6/8/8/PPP1N1PP/RNBQK2n b Q - 2 8",
        ),
        (
            Move::from_str("c4a6"),
            "rnbq1k1r/pp1Pbppp/B1p5/8/8/8/PPP1N1PP/RNBQK2n b Q - 2 8",
        ),
        (
            Move::from_str("c4d5"),
            "rnbq1k1r/pp1Pbppp/2p5/3B4/8/8/PPP1N1PP/RNBQK2n b Q - 2 8",
        ),
        (
            Move::from_str("c4e6"),
            "rnbq1k1r/pp1Pbppp/2p1B3/8/8/8/PPP1N1PP/RNBQK2n b Q - 2 8",
        ),
        (
            Move::from_str("c4f7"),
            "rnbq1k1r/pp1PbBpp/2p5/8/8/8/PPP1N1PP/RNBQK2n b Q - 0 8",
        ),
        (
            Move::from_str("d7c8Q"),
            "rnQq1k1r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2n b Q - 0 8",
        ),
        (
            Move::from_str("d7c8R"),
            "rnRq1k1r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2n b Q - 0 8",
        ),
        (
            Move::from_str("d7c8N"),
            "rnNq1k1r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2n b Q - 0 8",
        ),
        (
            Move::from_str("d7c8B"),
            "rnBq1k1r/pp2bppp/2p5/8/2B5/8/PPP1N1PP/RNBQK2n b Q - 0 8",
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
fn promotion_1() {
    let position = "1k6/5P2/8/8/8/8/8/4K3 w - - 20 1";
    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("e1d1"), "1k6/5P2/8/8/8/8/8/3K4 b - - 21 1"),
        (Move::from_str("e1f1"), "1k6/5P2/8/8/8/8/8/5K2 b - - 21 1"),
        (Move::from_str("e1d2"), "1k6/5P2/8/8/8/8/3K4/8 b - - 21 1"),
        (Move::from_str("e1e2"), "1k6/5P2/8/8/8/8/4K3/8 b - - 21 1"),
        (Move::from_str("e1f2"), "1k6/5P2/8/8/8/8/5K2/8 b - - 21 1"),
        (Move::from_str("f7f8Q"), "1k3Q2/8/8/8/8/8/8/4K3 b - - 0 1"),
        (Move::from_str("f7f8R"), "1k3R2/8/8/8/8/8/8/4K3 b - - 0 1"),
        (Move::from_str("f7f8N"), "1k3N2/8/8/8/8/8/8/4K3 b - - 0 1"),
        (Move::from_str("f7f8B"), "1k3B2/8/8/8/8/8/8/4K3 b - - 0 1"),
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
fn promotion_2() {
    let position = "4k3/8/8/8/8/8/5p2/1K6 b - - 20 1";
    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (Move::from_str("f2f1q"), "4k3/8/8/8/8/8/8/1K3q2 w - - 0 2"),
        (Move::from_str("f2f1r"), "4k3/8/8/8/8/8/8/1K3r2 w - - 0 2"),
        (Move::from_str("f2f1n"), "4k3/8/8/8/8/8/8/1K3n2 w - - 0 2"),
        (Move::from_str("f2f1b"), "4k3/8/8/8/8/8/8/1K3b2 w - - 0 2"),
        (Move::from_str("e8d7"), "8/3k4/8/8/8/8/5p2/1K6 w - - 21 2"),
        (Move::from_str("e8e7"), "8/4k3/8/8/8/8/5p2/1K6 w - - 21 2"),
        (Move::from_str("e8f7"), "8/5k2/8/8/8/8/5p2/1K6 w - - 21 2"),
        (Move::from_str("e8d8"), "3k4/8/8/8/8/8/5p2/1K6 w - - 21 2"),
        (Move::from_str("e8f8"), "5k2/8/8/8/8/8/5p2/1K6 w - - 21 2"),
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
fn promotion_3() {
    let position = "3k4/8/1K6/8/8/8/pppppppp/RRRRRRRR b - - 0 1";
    let generated_moves: HashSet<Move> = Board::from_fen(position)
        .generate_moves()
        .into_iter()
        .collect();
    let expected = vec![
        (
            Move::from_str("a2b1q"),
            "3k4/8/1K6/8/8/8/1ppppppp/RqRRRRRR w - - 0 2",
        ),
        (
            Move::from_str("a2b1r"),
            "3k4/8/1K6/8/8/8/1ppppppp/RrRRRRRR w - - 0 2",
        ),
        (
            Move::from_str("a2b1n"),
            "3k4/8/1K6/8/8/8/1ppppppp/RnRRRRRR w - - 0 2",
        ),
        (
            Move::from_str("a2b1b"),
            "3k4/8/1K6/8/8/8/1ppppppp/RbRRRRRR w - - 0 2",
        ),
        (
            Move::from_str("b2a1q"),
            "3k4/8/1K6/8/8/8/p1pppppp/qRRRRRRR w - - 0 2",
        ),
        (
            Move::from_str("b2a1r"),
            "3k4/8/1K6/8/8/8/p1pppppp/rRRRRRRR w - - 0 2",
        ),
        (
            Move::from_str("b2a1n"),
            "3k4/8/1K6/8/8/8/p1pppppp/nRRRRRRR w - - 0 2",
        ),
        (
            Move::from_str("b2a1b"),
            "3k4/8/1K6/8/8/8/p1pppppp/bRRRRRRR w - - 0 2",
        ),
        (
            Move::from_str("b2c1q"),
            "3k4/8/1K6/8/8/8/p1pppppp/RRqRRRRR w - - 0 2",
        ),
        (
            Move::from_str("b2c1r"),
            "3k4/8/1K6/8/8/8/p1pppppp/RRrRRRRR w - - 0 2",
        ),
        (
            Move::from_str("b2c1n"),
            "3k4/8/1K6/8/8/8/p1pppppp/RRnRRRRR w - - 0 2",
        ),
        (
            Move::from_str("b2c1b"),
            "3k4/8/1K6/8/8/8/p1pppppp/RRbRRRRR w - - 0 2",
        ),
        (
            Move::from_str("c2b1q"),
            "3k4/8/1K6/8/8/8/pp1ppppp/RqRRRRRR w - - 0 2",
        ),
        (
            Move::from_str("c2b1r"),
            "3k4/8/1K6/8/8/8/pp1ppppp/RrRRRRRR w - - 0 2",
        ),
        (
            Move::from_str("c2b1n"),
            "3k4/8/1K6/8/8/8/pp1ppppp/RnRRRRRR w - - 0 2",
        ),
        (
            Move::from_str("c2b1b"),
            "3k4/8/1K6/8/8/8/pp1ppppp/RbRRRRRR w - - 0 2",
        ),
        (
            Move::from_str("c2d1q"),
            "3k4/8/1K6/8/8/8/pp1ppppp/RRRqRRRR w - - 0 2",
        ),
        (
            Move::from_str("c2d1r"),
            "3k4/8/1K6/8/8/8/pp1ppppp/RRRrRRRR w - - 0 2",
        ),
        (
            Move::from_str("c2d1n"),
            "3k4/8/1K6/8/8/8/pp1ppppp/RRRnRRRR w - - 0 2",
        ),
        (
            Move::from_str("c2d1b"),
            "3k4/8/1K6/8/8/8/pp1ppppp/RRRbRRRR w - - 0 2",
        ),
        (
            Move::from_str("e2d1q"),
            "3k4/8/1K6/8/8/8/pppp1ppp/RRRqRRRR w - - 0 2",
        ),
        (
            Move::from_str("e2d1r"),
            "3k4/8/1K6/8/8/8/pppp1ppp/RRRrRRRR w - - 0 2",
        ),
        (
            Move::from_str("e2d1n"),
            "3k4/8/1K6/8/8/8/pppp1ppp/RRRnRRRR w - - 0 2",
        ),
        (
            Move::from_str("e2d1b"),
            "3k4/8/1K6/8/8/8/pppp1ppp/RRRbRRRR w - - 0 2",
        ),
        (
            Move::from_str("e2f1q"),
            "3k4/8/1K6/8/8/8/pppp1ppp/RRRRRqRR w - - 0 2",
        ),
        (
            Move::from_str("e2f1r"),
            "3k4/8/1K6/8/8/8/pppp1ppp/RRRRRrRR w - - 0 2",
        ),
        (
            Move::from_str("e2f1n"),
            "3k4/8/1K6/8/8/8/pppp1ppp/RRRRRnRR w - - 0 2",
        ),
        (
            Move::from_str("e2f1b"),
            "3k4/8/1K6/8/8/8/pppp1ppp/RRRRRbRR w - - 0 2",
        ),
        (
            Move::from_str("f2e1q"),
            "3k4/8/1K6/8/8/8/ppppp1pp/RRRRqRRR w - - 0 2",
        ),
        (
            Move::from_str("f2e1r"),
            "3k4/8/1K6/8/8/8/ppppp1pp/RRRRrRRR w - - 0 2",
        ),
        (
            Move::from_str("f2e1n"),
            "3k4/8/1K6/8/8/8/ppppp1pp/RRRRnRRR w - - 0 2",
        ),
        (
            Move::from_str("f2e1b"),
            "3k4/8/1K6/8/8/8/ppppp1pp/RRRRbRRR w - - 0 2",
        ),
        (
            Move::from_str("f2g1q"),
            "3k4/8/1K6/8/8/8/ppppp1pp/RRRRRRqR w - - 0 2",
        ),
        (
            Move::from_str("f2g1r"),
            "3k4/8/1K6/8/8/8/ppppp1pp/RRRRRRrR w - - 0 2",
        ),
        (
            Move::from_str("f2g1n"),
            "3k4/8/1K6/8/8/8/ppppp1pp/RRRRRRnR w - - 0 2",
        ),
        (
            Move::from_str("f2g1b"),
            "3k4/8/1K6/8/8/8/ppppp1pp/RRRRRRbR w - - 0 2",
        ),
        (
            Move::from_str("g2f1q"),
            "3k4/8/1K6/8/8/8/pppppp1p/RRRRRqRR w - - 0 2",
        ),
        (
            Move::from_str("g2f1r"),
            "3k4/8/1K6/8/8/8/pppppp1p/RRRRRrRR w - - 0 2",
        ),
        (
            Move::from_str("g2f1n"),
            "3k4/8/1K6/8/8/8/pppppp1p/RRRRRnRR w - - 0 2",
        ),
        (
            Move::from_str("g2f1b"),
            "3k4/8/1K6/8/8/8/pppppp1p/RRRRRbRR w - - 0 2",
        ),
        (
            Move::from_str("g2h1q"),
            "3k4/8/1K6/8/8/8/pppppp1p/RRRRRRRq w - - 0 2",
        ),
        (
            Move::from_str("g2h1r"),
            "3k4/8/1K6/8/8/8/pppppp1p/RRRRRRRr w - - 0 2",
        ),
        (
            Move::from_str("g2h1n"),
            "3k4/8/1K6/8/8/8/pppppp1p/RRRRRRRn w - - 0 2",
        ),
        (
            Move::from_str("g2h1b"),
            "3k4/8/1K6/8/8/8/pppppp1p/RRRRRRRb w - - 0 2",
        ),
        (
            Move::from_str("h2g1q"),
            "3k4/8/1K6/8/8/8/ppppppp1/RRRRRRqR w - - 0 2",
        ),
        (
            Move::from_str("h2g1r"),
            "3k4/8/1K6/8/8/8/ppppppp1/RRRRRRrR w - - 0 2",
        ),
        (
            Move::from_str("h2g1n"),
            "3k4/8/1K6/8/8/8/ppppppp1/RRRRRRnR w - - 0 2",
        ),
        (
            Move::from_str("h2g1b"),
            "3k4/8/1K6/8/8/8/ppppppp1/RRRRRRbR w - - 0 2",
        ),
        (
            Move::from_str("d8d7"),
            "8/3k4/1K6/8/8/8/pppppppp/RRRRRRRR w - - 1 2",
        ),
        (
            Move::from_str("d8e7"),
            "8/4k3/1K6/8/8/8/pppppppp/RRRRRRRR w - - 1 2",
        ),
        (
            Move::from_str("d8c8"),
            "2k5/8/1K6/8/8/8/pppppppp/RRRRRRRR w - - 1 2",
        ),
        (
            Move::from_str("d8e8"),
            "4k3/8/1K6/8/8/8/pppppppp/RRRRRRRR w - - 1 2",
        ),
    ];
    // Check that the generated moves match the expected ones.
    let expected_moves: HashSet<Move> = expected.iter().map(|(r#move, _)| *r#move).collect();
    let extra_moves = generated_moves.difference(&expected_moves);
    assert!(extra_moves.count() == 0);
    let missing_moves = expected_moves.difference(&generated_moves);
    for m in missing_moves.clone() {
        println!("{m}");
    }
    assert!(missing_moves.count() == 0);

    // Check that applying each move results in the expected board state.
    for (r#move, expected_fen) in expected {
        let mut board = Board::from_fen(position);
        board.apply(r#move);
        if board.to_fen() != expected_fen {
            println!("{}", r#move);
        }
        assert_eq!(board.to_fen(), expected_fen);
        // TODO: Test undo's too
    }
}
