use crate::models::{board::ChessBoard, chessmove::ChessMove};

impl ChessBoard {
    pub fn best_mv(&self, depth: u8) -> ChessMove {
        let mut best_mv = ChessMove::new(0u16);
        // Looking for the best eval for the color that the engine plays as
        let mut best_eval = if self.get_white_to_move() {
            i32::MIN
        } else {
            i32::MAX
        };
        for mv in self.all_possible_moves() {
            let curr_eval = self.with_move(mv).minimax(depth);
            // dbg!(curr_eval);
            // dbg!(mv.to_str());
            if self.get_white_to_move() {
                if best_eval <= curr_eval {
                    best_eval = curr_eval;
                    best_mv = mv;
                }
            } else {
                if best_eval >= curr_eval {
                    best_eval = curr_eval;
                    best_mv = mv;
                }
            }
        }
        if best_mv.to_u16() == 0 {
            panic!("Cannot create move with value 0")
        }
        best_mv
    }
}