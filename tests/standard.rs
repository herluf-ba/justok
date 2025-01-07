use termchess::board::Board;
use termchess::Move;

#[test]
///Test every possible move from the starting position.
fn opening_moves() {
    let standard_position = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let expected = vec![(
        Move::from_str("b1a3"),
        "rnbqkbnr/pppppppp/8/8/8/N7/PPPPPPPP/R1BQKBNR b KQkq - 1 1",
    )];
    let mut board = Board::from_fen(standard_position);
    for (r#move, expected_fen) in expected {
        println!("{:?}", r#move);
        board.apply(r#move);
        assert_eq!(board.to_fen(), expected_fen);
    }
}
