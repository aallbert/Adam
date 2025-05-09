use std::io;

use crate::{logics::make_a_move, models::board::FenBoard};

pub fn parse_fen_to_board(fen: &str) -> Vec<Vec<char>> {
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
    // let mut constrains: Constrains = (true, true);
    let mut fen_string = FenBoard::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    fen_string = FenBoard::new("8/5P2/6P1/1Q1RN3/1p1B4/8/2K5/8");
    // fen_string = FenBoard::new("8/8/8/4N3/8/8/8/8");

    loop {
        let board = parse_fen_to_board(&fen_string.as_str());
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
        // println!("{:?}'s turn. Enter move: ", turn);
        let mut input = String::from("");
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim() {
                "quit" => {
                    println!("Exiting");
                    break;
                }
                _ => {
                    fen_string = make_a_move(fen_string, input);
                }
            },
            Err(error) => {
                println!("Error: {}\nExiting now", error);
                break;
            }
        }
    }
}
