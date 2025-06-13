use core::minimax::minimax;
use std::io;
use std::env;

use models::{board::ChessBoard, chessmove::ChessMove};
use testing::perft_test;

mod core;
mod gui;
mod interface;
mod logics;
mod models;
mod testing;

fn main() {
    let mut curr_board = ChessBoard::starting_position();
    // Testing with perftree-cli
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        dbg!(&args);
        let depth_str = &args[1];
        let depth: i32 = depth_str.parse().unwrap();

        // let fen_str = &args[2];
        // todo: implement from_fen()
        // curr_board = match ChessBoard::from_fen(fen_str) {
        //     Ok(b) => b,
        //     Err(e) => {
        //         std::process::exit(1);
        //     }
        // };

        if args.len() > 3 {
            let moves = &args[3];
            let moves_as_slices: Vec<&str> = moves.split_whitespace().collect();
            for mv_slice in moves_as_slices {
                curr_board.make_move(ChessMove::new_with_str(mv_slice));
            }
        }

        perft_test(curr_board, depth);
    }
    loop {
        let mut input = String::from("");
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim() {
                    "test" => {
                        gui::testing();
                    }
                    "uci" => {
                        println!("id name Adam 0.1");
                        println!("id author aallbert");
                        println!("option name Hash type spin default 1 min 1 max 128");
                        println!("uciok")
                    }
                    "isready" => {
                        println!("readyok")
                    }
                    "ucinewgame" => {}
                    l if l.starts_with("setoption") => {}
                    l if l.starts_with("position") => {
                        // Updating the current position everytime the position gets passed
                        let last_str = input.split_whitespace().last().unwrap();
                        if last_str != "1" {
                            let mv =
                                ChessMove::new_with_str(input.split_whitespace().last().unwrap());
                            curr_board.make_move(mv);
                        }
                    }
                    l if l.starts_with("go") => {
                        // Calculating all positions to a certain depth
                        let possible_moves = curr_board.possible_moves();
                        let mut best_mv = ChessMove::new(0u16); // todo: change
                        // Looking for the best eval for the color that the engine plays as
                        let mut best_eval = 0;
                        for mv in possible_moves {
                            let curr_eval = minimax(curr_board.with_move(mv), 4);
                            if curr_board.get_white_to_move() {
                                if best_eval < curr_eval {
                                    best_eval = curr_eval;
                                    best_mv = mv;
                                }
                            } else {
                                if best_eval > curr_eval {
                                    best_eval = curr_eval;
                                    best_mv = mv;
                                }
                            }
                        }
                        // println!("Debug - best_eval: {}", best_eval);
                        // println!("Debug - white_to_move: {}", curr_board.get_white_to_move());
                        println!("info pv {}", best_mv.to_str());
                        println!("bestmove {}", best_mv.to_str());
                        curr_board.make_move(best_mv);
                    }
                    "stop" => {
                        continue;
                    }
                    _ => {
                        println!("Invalid input: {}\nExiting now", input.trim());
                        break;
                    }
                }
            }
            Err(error) => {
                println!("Error: {}\nExiting now", error);
                break;
            }
        }
    }
}
