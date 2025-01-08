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
