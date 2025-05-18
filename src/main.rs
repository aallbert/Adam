use core::{minimax::minimax, movegen::possible_moves};
use std::io;

use models::{board::ChessBoard, chessmove::ChessMove};

mod core;
mod gui;
mod interface;
mod logics;
mod models;

fn main() {
    println!("Setup");
    let mut curr_board = ChessBoard::starting_position();
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
                    "ucinewgame" => {
                        //internal board clear?
                    }
                    l if l.starts_with("setoption") => {
                        //??
                    }
                    l if l.starts_with("position") => {
                        // Updating the current position everytime the position gets passed
                        let last_str = input.split_whitespace().last().unwrap();
                        if last_str != "1" {
                            let mv = ChessMove::new_with_str(input.split_whitespace().last().unwrap());
                            curr_board.make_move(mv);
                        }
                    }
                    l if l.starts_with("go") => {
                        // Calculating all positions to a certain depth
                        let possible_moves = possible_moves(&curr_board);
                        let mut best_mv = ChessMove::new(0u16); // todo: change
                        // Looking for the best eval for the color that the engine plays as
                        let mut best_eval = 0;
                        for mv in possible_moves {
                            let curr_eval =  minimax(curr_board.with_move(mv), 4);
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
