use crate::{
    logics::make_a_move_testing,
    models::{
        board::{ChessBoard, FenString},
        chessmove::ChessMoveChar,
    },
};
use rand::seq::{IndexedRandom, SliceRandom};
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
    // let mut constrains: Constrains = (true, true);
    // fen_string = FenBoard::new("8/5P2/6P1/1Q1RN3/1p1B4/8/2K5/8");
    // fen_string = FenBoard::new("8/8/8/4N3/8/8/8/8");

    loop {
        let fen_string = FenString::new(chess_board.to_fen());
        let fen_pieces = String::from(fen_string.get_pieces_part());
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
        let all_moves = chess_board.possible_moves();

        // todo: relevant for move evaluation
        // let mut new_positions: Vec<ChessBoard> = vec![];
        // for mv in all_moves {
        //     let new_board = chess_board.clone();
        //     new_board.make_move(mv);
        //     new_positions.push(new_board);
        // }

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
                    // println!("ChessBoard: {:?}", chess_board);
                }
                "all" => {
                    println!("Calculating all positions");
                    let mut possible_boards: Vec<ChessBoard> = vec![chess_board];
                    let mut debug_check_counter = 0;
                    for i in 1..=DEPTH {
                        let mut new_boards: Vec<ChessBoard> = vec![];
                        for board in possible_boards {
                            let all_moves = board.possible_moves();
                            for &mv in &all_moves {
                                let board_with_mv = board.with_move(mv);
                                let eval = board_with_mv.evaluate_position();
                                // eliminating checks
                                if eval < 10000 && eval > -10000{
                                    new_boards.push(board_with_mv);
                                }
                                else {
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
                _ => {
                    let chars: Vec<char> = input.chars().collect();
                    let curr_file = chars[0];
                    let curr_rank = chars[1];
                    let dest_file = chars[2];
                    let dest_rank = chars[3];
                    let mv_char =
                        ChessMoveChar::new_with_chars(curr_rank, curr_file, dest_rank, dest_file);
                    let mv = mv_char.to_chessmove();
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
