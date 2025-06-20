use crate::models::{board::ChessBoard, chessmove::ChessMove};
use rand::seq::IndexedRandom;
use std::io;

const DEPTH: i32 = 5;

pub fn parse_fen_pieces_to_board(fen: &str) -> Vec<Vec<char>> {
    fen.split('/')
        .map(|rank| {
            let mut row = Vec::new();
            for ch in rank.chars() {
                if ch.is_digit(10) {
                    let empty_squares = ch.to_digit(10).unwrap();
                    row.extend(std::iter::repeat('·').take(empty_squares as usize));
                } else {
                    row.push(ch);
                }
            }
            row
        })
        .collect()
}

pub fn testing() {
    let mut chess_board = ChessBoard::starting_position();
    let mut depth = 0;

    loop {
        let fen_string = String::from(chess_board.to_fen());
        let fen_pieces = match fen_string.find(' ') {
            Some(index) => fen_string[0..index].to_string(),
            None => fen_string[..].to_string(),
        };
        let board = parse_fen_pieces_to_board(&fen_pieces.as_str());
        println!("\n\n  +------------------------+");
        for (i, row) in board.iter().enumerate() {
            print!("{} |", 8 - i);
            for piece in row {
                print!(" {} ", piece);
            }
            println!("|");
        }
        println!("  +------------------------+");
        println!("    a  b  c  d  e  f  g  h");

        let mut input = String::from("");
        let all_moves = chess_board.all_possible_moves();

        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim() {
                "quit" => {
                    println!("Exiting");
                    break;
                }
                "rand" => {
                    println!("Picking random move");
                    let mut rng = rand::rng();
                    let rand_mv = all_moves.choose(&mut rng).cloned().unwrap();
                    println!("Chosen Move: {:?}\n", rand_mv);
                    chess_board.make_move(rand_mv);
                }
                "all" => {
                    println!("Calculating all positions");
                    let mut possible_boards: Vec<ChessBoard> = vec![chess_board];
                    let mut debug_check_counter = 0;
                    for i in 1..=DEPTH {
                        let mut new_boards: Vec<ChessBoard> = vec![];
                        for board in possible_boards {
                            let all_moves = board.all_possible_moves();
                            for &mv in &all_moves {
                                let board_with_mv = board.with_move(mv);
                                let eval = board_with_mv.evaluate_position();
                                // eliminating checks
                                if eval < 10000 && eval > -10000 {
                                    new_boards.push(board_with_mv);
                                } else {
                                    debug_check_counter += 1;
                                }
                            }
                        }
                        possible_boards = new_boards;
                        println!(
                            "Depth: {}\nCount of possible positions: {}",
                            i,
                            possible_boards.len()
                        );
                        println!("Positions with check: {}", debug_check_counter)
                    }
                }
                "best" => {
                    let best_mv = chess_board.best_mv(depth);
                    println!("best move: {}", best_mv.to_str());
                }
                "fen" => {
                    println!("input fen");
                    let mut fen_string = String::new();
                    io::stdin()
                        .read_line(&mut fen_string)
                        .expect("Failed to read line");
                    chess_board = ChessBoard::from_fen(&fen_string)
                }
                "depth" => {
                    println!("input depth");
                    let mut depth_string = String::new();
                    io::stdin()
                        .read_line(&mut depth_string)
                        .expect("Failed to read line");
                    depth = depth_string.trim().parse::<u8>().unwrap_or(0);
                }
                _ => {
                    let mv = ChessMove::from_str(&input);
                    chess_board.make_move(mv);
                }
            },
            Err(error) => {
                println!("Error: {}\nExiting now", error);
                break;
            }
        }
    }
}
