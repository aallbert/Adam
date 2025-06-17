use std::cmp;

use crate::models::board::ChessBoard;

pub fn minimax(chess_board: ChessBoard, depth: u8) -> i32 {
    if depth == 0 {
        return chess_board.evaluate_position();
    };
    if chess_board.get_white_to_move() {
        let mut max_eval = i32::MIN;
        // todo: consider changing
        for mv in chess_board.possible_moves(false) {
            let eval = minimax(chess_board.with_move(mv), depth - 1);
            max_eval = cmp::max(max_eval, eval);
        }
        return max_eval;
    } else {
        let mut min_eval = i32::MAX;
        // todo: consider changing
        for mv in chess_board.possible_moves(false) {
            let eval = minimax(chess_board.with_move(mv), depth - 1);
            min_eval = cmp::min(min_eval, eval);
        }
        return min_eval;
    };
}
