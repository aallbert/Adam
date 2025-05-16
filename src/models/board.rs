use core::fmt;

use super::{
    chessmove::{ChessMove, Square},
    piece::{castling, CastleRights, Piece},
};

#[derive(Clone, Copy, Debug)]
pub struct ChessBoard {
    bitboards: [Bitboard; 12],
    white_to_move: bool,
    castling_rights: u8,
    en_passant: Square,
}

#[allow(dead_code)]
impl ChessBoard {
    pub fn new() -> Self {
        Self {
            bitboards: [Bitboard::new(0); 12],
            white_to_move: true,
            castling_rights: castling::ALL,
            en_passant: Square::new(64), // 64 = no en passant available
        }
    }

    pub fn starting_position() -> Self {
        let mut board = Self::new();

        // White pieces
        board.bitboards[Piece::WhitePawn as usize] = Bitboard(0x0000_0000_0000_ff00);
        board.bitboards[Piece::WhiteRook as usize] = Bitboard(0x0000_0000_0000_0081);
        board.bitboards[Piece::WhiteKnight as usize] = Bitboard(0x0000_0000_0000_0042);
        board.bitboards[Piece::WhiteBishop as usize] = Bitboard(0x0000_0000_0000_0024);
        board.bitboards[Piece::WhiteQueen as usize] = Bitboard(0x0000_0000_0000_0010);
        board.bitboards[Piece::WhiteKing as usize] = Bitboard(0x0000_0000_0000_0008);

        // Black pieces
        board.bitboards[Piece::BlackPawn as usize] = Bitboard(0x00ff_0000_0000_0000);
        board.bitboards[Piece::BlackRook as usize] = Bitboard(0x8100_0000_0000_0000);
        board.bitboards[Piece::BlackKnight as usize] = Bitboard(0x4200_0000_0000_0000);
        board.bitboards[Piece::BlackBishop as usize] = Bitboard(0x2400_0000_0000_0000);
        board.bitboards[Piece::BlackQueen as usize] = Bitboard(0x1000_0000_0000_0000);
        board.bitboards[Piece::BlackKing as usize] = Bitboard(0x0800_0000_0000_0000);

        // All pieces

        board.white_to_move = true;
        board.castling_rights = castling::ALL;
        board.en_passant = Square::new(64);

        board
    }

    pub fn get_bitboards(&self) -> &[Bitboard; 12] {
        &self.bitboards
    }

    pub fn get_bitboard(&self, index: usize) -> Option<Bitboard> {
        self.bitboards.get(index).copied()
    }

    pub fn set_bitboard(&mut self, index: usize, bb: Bitboard) {
        if index < 12 {
            self.bitboards[index] = bb;
        }
    }

    pub fn set_bitboards(&mut self, bitboards: [Bitboard; 12]) {
        self.bitboards = bitboards;
    }

    pub fn get_all_pieces(&self) -> Bitboard {
        let all = self.bitboards.iter().fold(0u64, |acc, bb| acc | bb.0);
        Bitboard(all)
    }

    pub fn get_white_to_move(&self) -> bool {
        self.white_to_move
    }

    pub fn set_white_to_move(&mut self, white: bool) {
        self.white_to_move = white;
    }

    pub fn get_castling_rights(&self) -> u8 {
        self.castling_rights
    }

    pub fn set_castling_rights(&mut self, rights: u8) {
        self.castling_rights = rights;
    }

    pub fn get_en_passant(&self) -> Square {
        self.en_passant
    }

    pub fn set_en_passant(&mut self, square: Square) {
        self.en_passant = square;
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        for rank_index in 0..8 {
            let mut empty = 0;
            for file_index in 0..8 {
                let sq = rank_index * 8 + file_index;
                let mut found = false;

                for (i, bitboard) in self.bitboards.iter().enumerate() {
                    if bitboard.get_bit(sq) {
                        if empty > 0 {
                            fen.push_str(&empty.to_string());
                            empty = 0;
                        }
                        let c = Piece::to_char(Piece::try_from(i as u8).unwrap());
                        fen.push(c);
                        found = true;
                        break;
                    }
                }

                if !found {
                    empty += 1;
                }
            }

            if empty > 0 {
                fen.push_str(&empty.to_string());
            }

            if rank_index != 7 {
                fen.push('/');
            }
        }

        // Side to move
        fen.push(' ');
        fen.push(if self.white_to_move { 'w' } else { 'b' });

        // Castling rights
        fen.push(' ');
        let mut castle = String::new();
        use crate::models::piece::castling::*;
        if self.castling_rights & WHITE_K != 0 {
            castle.push('K');
        }
        if self.castling_rights & WHITE_Q != 0 {
            castle.push('Q');
        }
        if self.castling_rights & BLACK_K != 0 {
            castle.push('k');
        }
        if self.castling_rights & BLACK_Q != 0 {
            castle.push('q');
        }
        if castle.is_empty() {
            castle.push('-');
        }
        fen.push_str(&castle);

        // En passant
        fen.push(' ');
        if self.en_passant.get_as_u16() < 64 {
            fen.push_str(&self.en_passant.get_as_str());
        } else {
            fen.push('-');
        }

        // Halfmove clock / fullmove number (set to 0/1 for now)
        fen.push_str(" 0 1");

        return fen;
    }
    pub fn make_move(&mut self, mv: ChessMove) {
        let curr_sq = mv.get_curr_square_as_index();
        let dest_sq = mv.get_dest_square_as_index();

        for bitboard in self.bitboards.iter_mut() {
            // clearing bit to cover capturing
            bitboard.clear_bit(dest_sq);
            if bitboard.get_bit(curr_sq) {
                bitboard.clear_bit(curr_sq);
                bitboard.set_bit(dest_sq);
            }
        }
        self.white_to_move = !self.white_to_move;
    }

    /// Copies the board, makes the move, returns new board
    pub fn with_move(mut self, mv: ChessMove) -> Self {
        self.make_move(mv);
        self
    }
    // todo: const function for castling
    // todo: pawn promotion
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
    pub fn get_bit(&self, index: u16) -> bool {
        let mask = 1u64 << (!index & 0b111111);
        (self.0 & mask) != 0
    }
    pub fn get_bit_manual(&self, file: i8, rank: i8) -> bool {
        let index = -(rank - 8) * 8 + file - 1;
        if index >= 64 || index < 0 {
            panic!("Bit index out of range for u64: {}", index);
        }
        let mask = 1u64 << index;
        (self.0 & mask) != 0
    }
    pub fn set_bit(&mut self, index: u16) {
        self.0 |= 1 << (!index & 0b111111);
    }
    pub fn clear_bit(&mut self, index: u16) {
        self.0 &= !(1 << (!index & 0b111111));
    }
    pub fn to_u64(self) -> u64 {
        self.0
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

pub struct BitboardIterator {
    bits: u64,
}

// Returns the Index of each set Bit in a Bitboard for every iteration
impl Iterator for BitboardIterator {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bits == 0 {
            None
        } else {
            let index = self.bits.leading_zeros() as u16;
            self.bits &= u64::MAX.checked_shr(index as u32 + 1).unwrap_or(0); // Deletes most significant set bit
            Some(index)
        }
    }
}

impl IntoIterator for Bitboard {
    type Item = u16;
    type IntoIter = BitboardIterator;

    fn into_iter(self) -> Self::IntoIter {
        BitboardIterator { bits: self.0 }
    }
}

impl<'a> IntoIterator for &'a Bitboard {
    type Item = u16;
    type IntoIter = BitboardIterator;

    fn into_iter(self) -> Self::IntoIter {
        BitboardIterator { bits: self.0 }
    }
}

// pub mod starting_bitboards {
//     use super::Bitboard;

//     pub const WHITE_PAWNS: Bitboard = Bitboard::new(0x00);
// }

#[derive(Debug)]
pub struct FenString(String);
impl FenString {
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self(s.into())
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
    pub fn get_pieces_part(&self) -> &str {
        match self.0.find(' ') {
            Some(index) => &self.0[0..index],
            None => &self.0[..],
        }
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
