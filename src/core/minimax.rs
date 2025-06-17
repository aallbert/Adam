use std::cmp;

use crate::models::board::ChessBoard;
impl ChessBoard {
    pub fn minimax(&self, depth: u8) -> i32 {
        if depth == 0 {
            return self.evaluate_position();
        };
        if self.get_white_to_move() {
            let mut max_eval = i32::MIN;
            // todo: consider changing
            // maybe more efficient way??
            for mv in self.possible_moves() {
                let eval = self.with_move(mv).minimax(depth - 1);
                max_eval = cmp::max(max_eval, eval);
            }
            return max_eval;
        } else {
            let mut min_eval = i32::MAX;
            // todo: consider changing
            for mv in self.possible_moves() {
                let eval = self.with_move(mv).minimax(depth - 1);
                min_eval = cmp::min(min_eval, eval);
            }
            return min_eval;
        };
    }
}