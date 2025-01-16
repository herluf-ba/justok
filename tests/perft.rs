use justok::board::Board;

fn perft(board: Board, depth: usize) -> u128 {
    let moves = board.generate_moves();
    if depth == 1 {
        return moves.len() as u128;
    };

    moves
        .iter()
        .map(|&moove| {
            let mut board_with_move = board.clone();
            board_with_move.apply(moove);
            perft(board_with_move, depth - 1)
        })
        .sum()
}

#[test]
fn perft_standard() {
    let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let expected: [u128; 6] = [20, 400, 8902, 197281, 4865609, 119060324];
    let actual = perft(board, 5);
    assert_eq!(expected[4], actual);
}

#[test]
fn perft_kiwi_pete() {
    let board =
        Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    let expected: [u128; 6] = [48, 2039, 97862, 4085603, 193690690, 8031647685];
    let actual = perft(board, 4);
    assert_eq!(expected[3], actual);
}

#[test]
fn perft_tricky() {
    let board = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    let expected: [u128; 5] = [44, 1486, 62379, 2103487, 89941194];
    let actual = perft(board, 4);
    assert_eq!(expected[3], actual);
}
