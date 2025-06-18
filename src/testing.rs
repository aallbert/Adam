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
