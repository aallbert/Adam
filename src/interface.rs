pub type Bitboard = u64;
pub type Move = String;
pub type FenBoard = String;
pub type SimpleBoard = Vec<char>;

#[derive(Debug)]
pub enum Turn {
    White,
    Black,
}

pub enum BoardType {
    WhiteAllPieces,
    BlackAllPieces,
}

pub struct Constrains {
    w_castle: bool,
    b_castle: bool,
}

pub fn char_to_number(c: char) -> i8 {
    (c as i8 - b'a' as i8) + 1
}