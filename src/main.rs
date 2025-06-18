use std::env;
use std::io;

use models::{board::ChessBoard, chessmove::ChessMove};
use testing::perft_test;

mod core;
mod gui;
mod interface;
mod models;
mod testing;

fn main() {
    // Testing with perftree-cli
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let depth_str = &args[1];
        let depth: i32 = depth_str.parse().unwrap();

        let fen_str = &args[2];
        let mut curr_board_testing = ChessBoard::from_fen(fen_str);
        if args.len() > 3 {
            let moves = &args[3];
            let moves_as_slices: Vec<&str> = moves.split_whitespace().collect();
            for mv_slice in moves_as_slices {
                curr_board_testing.make_move(ChessMove::from_str(mv_slice));
            }
        }

        perft_test(curr_board_testing, depth);
    }
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
                    "ucinewgame" => {}
                    l if l.starts_with("setoption") => {}
                    l if l.starts_with("position") => {
                        let fen_string = if let Some(remainder) = l.strip_prefix("position fen ") {
                            if let Some(moves_start) = remainder.find(" moves ") {
                                Some(&remainder[..moves_start])
                            } else {
                                Some(remainder)
                            }
                        } else {
                            None
                        };
                        curr_board = ChessBoard::from_fen(fen_string.unwrap());
                        let moves_strings: Vec<&str> = l
                            .split_whitespace()
                            .skip_while(|&part| part != "moves") // Skip until "moves" is found
                            .skip(1) // Skip "moves" itself
                            .collect();
                        // Calculate board (stateless)
                        for mv in moves_strings {
                            curr_board.make_move(ChessMove::from_str(mv));
                        }
                    }
                    l if l.starts_with("go") => {
                        // Calculating all positions to a certain depth
                        // !!! depth is hardcoded, but can be calculated depending on the time left for the engine
                        let best_mv = curr_board.best_mv(4);
                        println!("info pv {}", best_mv.to_str());
                        println!("bestmove {}", best_mv.to_str());
                    }
                    "stop" => {
                        continue;
                    }
                    "quit" => {
                        println!("bye");
                        break;
                    }
                    _ => {
                        println!("Invalid input: {}\nContinuing", input.trim());
                        continue;
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
