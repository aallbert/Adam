pub type Bitboard = u64;
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

#[derive(Debug, Copy, Clone)]
pub struct Square {
    pub file: char,
    pub rank: char,
}

#[derive(Debug)]
pub struct Move {
    pub from: Square,
    pub to: Square,
}

pub fn file_to_i8(c: char) -> i8 {
    (c as i8 - b'a' as i8) + 1
}

pub fn rank_to_i8(c: char) -> i8 {
    (c as i8 - b'1' as i8) + 1
}

pub fn i8_to_file(i: i8) -> char {
    (b'a' + (i - 1) as u8) as char
}

pub fn i8_to_rank(i: i8) -> char {
    (b'1' + (i - 1) as u8) as char
}

pub fn get_bit_on_bitboard(bitboard: Bitboard, file: i8, rank: i8) -> bool {
    let index = -(rank - 8) * 8 + file - 1;
    if index >= 64 {
        panic!("Bit index out of range for u64");
    }
    (bitboard & (1 << -(index - 63) as u8)) != 0
}
