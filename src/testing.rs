use crate::models::board::ChessBoard;

pub fn perft_test (board: ChessBoard, depth: i32) {
    let all_moves = board.possible_moves();
    for mv in all_moves {
        let pos_count = count_positions(board.with_move(mv), depth - 1);
        println!("{} {}", mv.to_str(), pos_count);
    }
    println!();
    println!("{}", count_positions(board, depth));
}

fn count_positions (board: ChessBoard, depth: i32) -> u64 {
    let all_moves = board.possible_moves();
    if depth == 1 {
        return all_moves.len() as u64;
    };
    if depth < 1 {
        return 1;
    };
    let mut pos_count = 0;
    for mv in all_moves {
        pos_count += count_positions(board.with_move(mv), depth - 1);
    };
    return pos_count;
}

// todo: remove
// "all" => {
//     println!("Calculating all positions");
//     let mut possible_boards: Vec<ChessBoard> = vec![chess_board];
//     let mut debug_check_counter = 0;
//     for i in 1..=DEPTH {
//         let mut new_boards: Vec<ChessBoard> = vec![];
//         for board in possible_boards {
//             let all_moves = board.possible_moves();
//             for &mv in &all_moves {
//                 let board_with_mv = board.with_move(mv);
//                 let eval = board_with_mv.evaluate_position();
//                 // eliminating checks
//                 if eval < 10000 && eval > -10000{
//                     new_boards.push(board_with_mv);
//                 }
//                 else {
//                     debug_check_counter += 1;
//                 }
//             }
//         }
//         possible_boards = new_boards;
//         println!(
//             "Depth: {}\nCount of possible positions: {}",
//             i,
//             possible_boards.len()
//         );
//         println!("Positions with check: {}", debug_check_counter)
//     }
// }