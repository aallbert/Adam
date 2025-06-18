use crate::models::board::ChessBoard;

pub fn perft_test(board: ChessBoard, depth: i32) {
    let all_moves = board.all_possible_moves();
    for mv in all_moves {
        let pos_count = count_positions(board.with_move(mv), depth - 1);
        println!("{} {}", mv.to_str(), pos_count);
    }
    println!();
    println!("{}", count_positions(board, depth));
}

fn count_positions(board: ChessBoard, depth: i32) -> u64 {
    let all_moves = board.all_possible_moves();
    if depth == 1 {
        return all_moves.len() as u64;
    };
    if depth < 1 {
        return 1;
    };
    let mut pos_count = 0;
    for mv in all_moves {
        pos_count += count_positions(board.with_move(mv), depth - 1);
    }
    return pos_count;
}

#[cfg(test)]
mod tests {
    use super::count_positions;
    use crate::models::{board::ChessBoard, chessmove::ChessMove};

    // Testing for Shannons number
    // See for reference: https://en.wikipedia.org/wiki/Shannon_number

    #[test]
    fn perft_depth_1() {
        let board = ChessBoard::starting_position();
        assert_eq!(count_positions(board, 1), 20);
    }

    #[test]
    fn perft_depth_2() {
        let board = ChessBoard::starting_position();
        assert_eq!(count_positions(board, 2), 400);
    }

    #[test]
    fn perft_depth_5() {
        let board = ChessBoard::starting_position();
        assert_eq!(count_positions(board, 5), 4_865_609);
    }

    #[test]
    /// Compares the `ChessBoard::starting_position()` Chessboard with the one created with `ChessBoard::from_fen()`
    fn from_fen_is_starting_pos() {
        let start_board = ChessBoard::starting_position();
        let fen_board = ChessBoard::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert_eq!(start_board, fen_board);
    }

    #[test]
    /// Runs some moves and compares it to the corresponding fen position
    fn a_couple_moves() {
        let mut start_board = ChessBoard::starting_position();
        start_board.make_move(ChessMove::from_str("e2e4"));
        start_board.make_move(ChessMove::from_str("e7e5"));
        start_board.make_move(ChessMove::from_str("g1f3"));
        start_board.make_move(ChessMove::from_str("b8c6"));
        start_board.make_move(ChessMove::from_str("f1c4"));
        start_board.make_move(ChessMove::from_str("d8f6"));
        start_board.make_move(ChessMove::from_str("e1g1"));
        let fen_board = ChessBoard::from_fen("r1b1kbnr/pppp1ppp/2n2q2/4p3/2B1P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 5 4");
        assert_eq!(start_board, fen_board);
    }
}