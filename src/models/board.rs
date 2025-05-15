use core::fmt;

use super::{chessmove::Square, piece::{castling, CastleRights, Piece}};


pub struct ChessBoard {
    bitboards: [Bitboard; 12],
    all_pieces: Bitboard,
    white_to_move: bool,
    castling_rights: u8,
    en_passant: Square,
}

#[allow(dead_code)]
impl ChessBoard {
    pub fn new() -> Self {
        Self {
            bitboards: [Bitboard::new(0); 12],
            all_pieces: Bitboard::new(0),
            white_to_move: true,
            castling_rights: castling::ALL,
            en_passant: Square::new(64), // 64 = no en passant available
        }
    }

    pub fn starting_position() -> Self {
        let mut board = Self::new();

        // White pieces
        board.bitboards[Piece::WhitePawn as usize]   = Bitboard(0x0000_0000_0000_ff00);
        board.bitboards[Piece::WhiteRook as usize]   = Bitboard(0x0000_0000_0000_0081);
        board.bitboards[Piece::WhiteKnight as usize] = Bitboard(0x0000_0000_0000_0042);
        board.bitboards[Piece::WhiteBishop as usize] = Bitboard(0x0000_0000_0000_0024);
        board.bitboards[Piece::WhiteQueen as usize]  = Bitboard(0x0000_0000_0000_0008);
        board.bitboards[Piece::WhiteKing as usize]   = Bitboard(0x0000_0000_0000_0010);

        // Black pieces
        board.bitboards[Piece::BlackPawn as usize]   = Bitboard(0x00ff_0000_0000_0000);
        board.bitboards[Piece::BlackRook as usize]   = Bitboard(0x8100_0000_0000_0000);
        board.bitboards[Piece::BlackKnight as usize] = Bitboard(0x4200_0000_0000_0000);
        board.bitboards[Piece::BlackBishop as usize] = Bitboard(0x2400_0000_0000_0000);
        board.bitboards[Piece::BlackQueen as usize]  = Bitboard(0x0800_0000_0000_0000);
        board.bitboards[Piece::BlackKing as usize]   = Bitboard(0x1000_0000_0000_0000);

        // All pieces
        let all = board.bitboards.iter().fold(0u64, |acc, bb| acc | bb.0);
        board.all_pieces = Bitboard(all);

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
        self.all_pieces
    }

    pub fn set_all_pieces(&mut self, bb: Bitboard) {
        self.all_pieces = bb;
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

        for rank in (0..8).rev() {
            let mut empty = 0;
            for file in 0..8 {
                let sq = rank * 8 + file;
                let mut found = false;

                for (i, bb) in self.bitboards.iter().enumerate() {
                    if (bb.0 >> sq) & 1 != 0 {
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

            if rank != 0 {
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
        if self.castling_rights & WHITE_K != 0 { castle.push('K'); }
        if self.castling_rights & WHITE_Q != 0 { castle.push('Q'); }
        if self.castling_rights & BLACK_K != 0 { castle.push('k'); }
        if self.castling_rights & BLACK_Q != 0 { castle.push('q'); }
        if castle.is_empty() { castle.push('-'); }
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

        return fen
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