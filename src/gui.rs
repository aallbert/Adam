use std::io;

use crate::{
    interface::{Bitboard, Constrains, Turn},
    logics::move_validation,
};

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
    let mut turn: Turn = Turn::White;
    // let mut constrains: Constrains = (true, true);
    let mut fen_string = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    let mut w_pawnboard: Bitboard = 0x000000000000FF00;
    let mut b_pawnboard: Bitboard = 0x00FF000000000000;

    loop {
        let board = parse_fen_to_board(&fen_string);
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
        println!("{:?}'s turn. Enter move: ", turn);
        let mut input = String::from("");
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim() {
                "quit" => {
                    println!("Exiting");
                    break;
                }
                _ => {
                    fen_string = move_validation(fen_string, input);
                }
            },
            Err(error) => {
                println!("Error: {}\nExiting now", error);
                break;
            }
        }
    }
}
