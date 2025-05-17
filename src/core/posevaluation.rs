use crate::models::{board::ChessBoard, piecesquaretables::PIECE_SQUARE_TABLES};

pub fn evaluate_position(chess_board: ChessBoard) -> i32 {
    let mut res = 0;
    for (pc,bitboard) in chess_board.get_bitboards().iter().enumerate() {
        for i  in bitboard {
            res += PIECE_SQUARE_TABLES[pc][i as usize];
        }
    };
    res
}