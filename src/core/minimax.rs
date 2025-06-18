use std::cmp;

use crate::models::board::ChessBoard;
impl ChessBoard {
    pub fn minimax(&self, mut depth: u8) -> i32 {
        if depth == 0 {
            return self.evaluate_position();
        };
        let all_moves = self.all_possible_moves();
        // If the tree gets too big, reduce the depth so performance gets safed
        // if all_moves.len() > 35 {
        //     depth -= 1;
        // }
        if self.get_white_to_move() {
            let mut max_eval = i32::MIN;
            for mv in all_moves {
                let eval = self.with_move(mv).minimax(depth - 1);
                max_eval = cmp::max(max_eval, eval);
            }
            return max_eval;
        } else {
            let mut min_eval = i32::MAX;
            for mv in all_moves {
                let eval = self.with_move(mv).minimax(depth - 1);
                min_eval = cmp::min(min_eval, eval);
            }
            return min_eval;
        };
    }
}
