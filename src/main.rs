use std::io;

mod gui;
mod interface;
mod logics;

fn main() {
    println!("Setup");
    // let mut mode = String::new();
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
                        println!("id author Albert");
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
                        //extract moves
                    }
                    l if l.starts_with("go") => {
                        //calc moves
                        // println!("info depth {} score cp {} time {} nodes {} nps {} pv {}");
                        // println!("info depth score cp time nodes nps pv");
                        println!("info pv e2e4");
                        println!("bestmove e2e4 ponder e7e5")
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
