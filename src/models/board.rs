use core::fmt;

use super::piece::{castling, CastleRights};


pub struct ChessBoard {
    bitboards: [Bitboard; 12],
    all_pieces: Bitboard,
    white_to_move: bool,
    castling_rights: u8,
    en_passant: u8,
}
impl ChessBoard {
    pub fn new() -> Self {
        Self {
            bitboards: [Bitboard::new(0); 12],
            all_pieces: Bitboard::new(0),
            white_to_move: true,
            castling_rights: castling::ALL,
            en_passant: 64, // außerhalb des Brettes = kein En-Passant
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]

/// A Chess board, where each square is represented with one Bit.
/// - The MSB represents the a8 square
/// - The LSB represents the h1 square
/// - The index increments from left to right, then from top to bottom
///
/// ## Index Map
/// ```
/// 8 [ 0,  1,  2,  3,  4,  5,  6,  7 ]
/// 7 [ 8,  9, 10, 11, 12, 13, 14, 15 ]
/// 6 [16, 17, 18, 19, 20, 21, 22, 23 ]
/// 5 [24, 25, 26, 27, 28, 29, 30, 31 ]
/// 4 [32, 33, 34, 35, 36, 37, 38, 39 ]
/// 3 [40, 41, 42, 43, 44, 45, 46, 47 ]
/// 2 [48, 49, 50, 51, 52, 53, 54, 55 ]
/// 1 [56, 57, 58, 59, 60, 61, 62, 63 ]
///     a   b   c   d   e   f   g   h
/// ```
pub struct Bitboard(u64);
impl Bitboard {
    pub fn new(val: u64) -> Self {
        Self(val)
    }
    pub fn get_bit_on_bitboard(&self, index: u16) -> bool {
        let mask = 1u64 << (!index & 0b111111);
        (self.0 & mask) != 0
    }
    pub fn get_bit_on_bitboard_manual(&self, file: i8, rank: i8) -> bool {
        let index = -(rank - 8) * 8 + file - 1;
        if index >= 64 || index < 0 {
            panic!("Bit index out of range for u64: {}", index);
        }
        let mask = 1u64 << index;
        (self.0 & mask) != 0
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl std::ops::ShlAssign<usize> for Bitboard {
    fn shl_assign(&mut self, rhs: usize) {
        self.0 <<= rhs
    }
}

impl std::ops::AddAssign<u64> for Bitboard {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

impl std::fmt::Binary for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Binary::fmt(&self.0, f)
    }
}

// pub mod starting_bitboards {
//     use super::Bitboard;

//     pub const WHITE_PAWNS: Bitboard = Bitboard::new(0x00);
// }

#[derive(Debug)]
pub struct FenBoard(String);
impl FenBoard {
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn chars(&self) -> std::str::Chars<'_> {
        self.0.chars()
    }
    pub fn push(&mut self, c: char) {
        self.0.push(c);
    }
    pub fn pop(&mut self) {
        self.0.pop();
    }
}

pub struct SimpleBoard(Vec<char>);
impl SimpleBoard {
    pub fn new(val: Option<Vec<char>>) -> Self {
        match val {
            Some(vec) => Self(vec),
            None => Self(Vec::new()),
        }
    }
    pub fn as_vec_char(&self) -> &Vec<char> {
        &self.0
    }
    pub fn set(&mut self, index: usize, c: char) {
        self.0[index] = c;
    }
    pub fn push(&mut self, c: char) {
        self.0.push(c);
    }
}