use core::fmt;

use crate::interface::abs_diff_u16;

use super::{
    chessmove::{CastleMove, ChessMove, Square},
    piece::{
        CastleRights, Piece,
        castling::{self, BLACK_BOTH},
    },
    piecesquaretables::PIECE_SQUARE_TABLES,
};

#[derive(Clone, Copy, Debug)]
pub struct ChessBoard {
    bitboards: [Bitboard; 12],
    white_to_move: bool,
    castling_rights: u8,
    // en_passant represents the index at which a pan should be captured
    // e.g.for e2e4, en_passant would be 44 (e3)
    en_passant: u16,
}

#[allow(dead_code)]
impl ChessBoard {
    pub fn new() -> Self {
        Self {
            bitboards: [Bitboard::new(0); 12],
            white_to_move: true,
            castling_rights: castling::ALL,
            en_passant: 64, // 64 = no en passant available
        }
    }

    pub fn starting_position() -> Self {
        let mut board = Self::new();

        // White pieces
        board.bitboards[Piece::WhitePawn as usize] = Bitboard::new(0x0000_0000_0000_ff00);
        board.bitboards[Piece::WhiteRook as usize] = Bitboard::new(0x0000_0000_0000_0081);
        board.bitboards[Piece::WhiteKnight as usize] = Bitboard::new(0x0000_0000_0000_0042);
        board.bitboards[Piece::WhiteBishop as usize] = Bitboard::new(0x0000_0000_0000_0024);
        board.bitboards[Piece::WhiteQueen as usize] = Bitboard::new(0x0000_0000_0000_0010);
        board.bitboards[Piece::WhiteKing as usize] = Bitboard::new(0x0000_0000_0000_0008);

        // Black pieces
        board.bitboards[Piece::BlackPawn as usize] = Bitboard::new(0x00ff_0000_0000_0000);
        board.bitboards[Piece::BlackRook as usize] = Bitboard::new(0x8100_0000_0000_0000);
        board.bitboards[Piece::BlackKnight as usize] = Bitboard::new(0x4200_0000_0000_0000);
        board.bitboards[Piece::BlackBishop as usize] = Bitboard::new(0x2400_0000_0000_0000);
        board.bitboards[Piece::BlackQueen as usize] = Bitboard::new(0x1000_0000_0000_0000);
        board.bitboards[Piece::BlackKing as usize] = Bitboard::new(0x0800_0000_0000_0000);

        board.white_to_move = true;
        board.castling_rights = castling::ALL;
        board.en_passant = 64;

        board
    }

    pub fn from_fen(fen: &str) -> Self {
        let parts: Vec<&str> = fen.split_whitespace().collect();

        let mut board = Self::new();
        // Clear all bitboards before setting
        for i in 0..12 {
            board.bitboards[i] = Bitboard::new(0);
        }

        // Piece placement
        let ranks: Vec<&str> = parts[0].split('/').collect();

        for (rank_idx, rank_str) in ranks.iter().enumerate() {
            let mut file_idx = 0;
            for char_code in rank_str.chars() {
                if char_code.is_digit(10) {
                    let empty_squares = char_code.to_digit(10).unwrap() as usize;
                    file_idx += empty_squares;
                } else {
                    let piece_type = match char_code {
                        'P' => Some(Piece::WhitePawn),
                        'N' => Some(Piece::WhiteKnight),
                        'B' => Some(Piece::WhiteBishop),
                        'R' => Some(Piece::WhiteRook),
                        'Q' => Some(Piece::WhiteQueen),
                        'K' => Some(Piece::WhiteKing),
                        'p' => Some(Piece::BlackPawn),
                        'n' => Some(Piece::BlackKnight),
                        'b' => Some(Piece::BlackBishop),
                        'r' => Some(Piece::BlackRook),
                        'q' => Some(Piece::BlackQueen),
                        'k' => Some(Piece::BlackKing),
                        _ => {continue;},
                    };

                    if let Some(piece) = piece_type {
                        let square_index = (rank_idx * 8) + file_idx;
                        board.bitboards[piece as usize].set_bit(square_index as u16);
                    }
                    file_idx += 1;
                }
            }
        }

        // Active color
        board.white_to_move = match parts[1] {
            "w" => true,
            "b" => false,
            _ => true
        };

        // Castling availability
        board.castling_rights = !castling::ALL;
        for c in parts[2].chars() {
            match c {
                'K' => board.castling_rights |= castling::WHITE_K,
                'Q' => board.castling_rights |= castling::WHITE_Q,
                'k' => board.castling_rights |= castling::BLACK_K,
                'q' => board.castling_rights |= castling::BLACK_Q,
                '-' => {} // No castling rights
                _ => {},
            }
        }

        // En passant target square
        board.en_passant = 64;
        if parts[3] != "-" {
            let file_char = parts[3].chars().next().unwrap();
            let rank_char = parts[3].chars().nth(1).unwrap();

            let file = match file_char {
                'a' => 0, 'b' => 1, 'c' => 2, 'd' => 3,
                'e' => 4, 'f' => 5, 'g' => 6, 'h' => 7,
                _ => 0,
            };

            let rank = match rank_char {
                '1' => 7, '2' => 6, '3' => 5, '4' => 4,
                '5' => 3, '6' => 2, '7' => 1, '8' => 0,
                _ => 0,
            };

            // Convert file/rank to your bitboard index (0=a8, 63=h1)
            board.en_passant = (rank * 8 + file) as u16;
        }
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

    pub fn get_en_passant(&self) -> u16 {
        self.en_passant
    }

    pub fn set_en_passant(&mut self, index: u16) {
        self.en_passant = index;
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
        if self.en_passant < 64 {
            fen.push_str(&self.en_passant.to_string());
        } else {
            fen.push('-');
        }

        // Halfmove clock / fullmove number (set to 0/1 for now)
        fen.push_str(" 0 1");

        return fen;
    }

    pub fn evaluate_position(&self) -> i32 {
        let mut res = 0;
        for (pc, bitboard) in self.get_bitboards().iter().enumerate() {
            for i in bitboard {
                res += PIECE_SQUARE_TABLES[pc][i as usize];
            }
        }
        res
    }

    pub fn make_move(&mut self, mv: ChessMove) {
        self.white_to_move = !self.white_to_move;

        let curr_sq = mv.get_curr_square_as_index();
        let dest_sq = mv.get_dest_square_as_index();

        // todo: delete bc of weird ass gui
        // todo: delete counterparts in movegen!!!!!!!!!!!!!!
        // match msb {
        //     castling::WHITE_K => {self.castling_rights &= !castling::WHITE_K }
        //     castling::WHITE_Q => {self.castling_rights &= !castling::WHITE_Q }
        //     castling::BLACK_K => {self.castling_rights &= !castling::BLACK_K }
        //     castling::BLACK_Q => {self.castling_rights &= !castling::BLACK_Q }
        //     _ => {}
        // }

        // checking if it is a castling move, removing castling rights for kingmoves
        if self.bitboards[Piece::WhiteKing as usize].get_bit(curr_sq) {
            match dest_sq {
                // g1
                62u16 => {
                    self.white_castle_kingside();
                    return;
                }
                // c1
                58u16 => {
                    self.white_castle_queenside();
                    return;
                }
                _ => {
                    self.castling_rights &= castling::BLACK_BOTH;
                }
            }
        }

        if self.bitboards[Piece::BlackKing as usize].get_bit(curr_sq) {
            match dest_sq {
                // g8
                6u16 => {
                    self.black_castle_kingside();
                    return;
                }
                // c8
                2u16 => {
                    self.black_castle_queenside();
                    return;
                }
                _ => {
                    self.castling_rights &= castling::WHITE_BOTH;
                }
            }
        }

        // checking for castling rights when rook moves/moved
        match curr_sq {
            56 => self.castling_rights &= !castling::WHITE_K,
            63 => self.castling_rights &= !castling::WHITE_Q,
            0 => self.castling_rights &= !castling::BLACK_K,
            7 => self.castling_rights &= !castling::BLACK_Q,
            _ => {}
        }

        // handling promotion moves, intern and if passed by gui
        let four_msb = mv.get_four_msb();
        if four_msb != 0 {
            if curr_sq > dest_sq {
                self.bitboards[Piece::WhitePawn as usize].clear_bit(curr_sq);
                self.bitboards[Piece::BlackBishop as usize].clear_bit(dest_sq);
                self.bitboards[Piece::BlackKnight as usize].clear_bit(dest_sq);
                self.bitboards[Piece::BlackRook as usize].clear_bit(dest_sq);
                self.bitboards[Piece::BlackQueen as usize].clear_bit(dest_sq);
                match four_msb {
                    0b0001 => {
                        self.bitboards[Piece::WhiteKnight as usize].set_bit(dest_sq);
                        self.en_passant = 64;
                        return;
                    }
                    0b0010 => {
                        self.bitboards[Piece::WhiteBishop as usize].set_bit(dest_sq);
                        self.en_passant = 64;
                        return;
                    }
                    0b0100 => {
                        self.bitboards[Piece::WhiteRook as usize].set_bit(dest_sq);
                        self.en_passant = 64;
                        return;
                    }
                    0b1000 => {
                        self.bitboards[Piece::WhiteQueen as usize].set_bit(dest_sq);
                        self.en_passant = 64;
                        return;
                    }
                    _ => {}
                }
            } else {
                self.bitboards[Piece::BlackPawn as usize].clear_bit(curr_sq);
                self.bitboards[Piece::WhiteBishop as usize].clear_bit(dest_sq);
                self.bitboards[Piece::WhiteKnight as usize].clear_bit(dest_sq);
                self.bitboards[Piece::WhiteRook as usize].clear_bit(dest_sq);
                self.bitboards[Piece::WhiteQueen as usize].clear_bit(dest_sq);
                match four_msb {
                    0b0001 => {
                        self.bitboards[Piece::BlackKnight as usize].set_bit(dest_sq);
                        self.en_passant = 64;
                        return;
                    }
                    0b0010 => {
                        self.bitboards[Piece::BlackBishop as usize].set_bit(dest_sq);
                        self.en_passant = 64;
                        return;
                    }
                    0b0100 => {
                        self.bitboards[Piece::BlackRook as usize].set_bit(dest_sq);
                        self.en_passant = 64;
                        return;
                    }
                    0b1000 => {
                        self.bitboards[Piece::BlackQueen as usize].set_bit(dest_sq);
                        self.en_passant = 64;
                        return;
                    }
                    _ => {}
                }
            }
        }

        // checking for en passant availability
        if abs_diff_u16(curr_sq, dest_sq) == 16
            && (self.bitboards[Piece::WhitePawn as usize].get_bit(curr_sq)
                || self.bitboards[Piece::BlackPawn as usize].get_bit(curr_sq))
        {
            self.en_passant = (curr_sq + dest_sq) >> 1;
            // already can make move here, since we know it's a pawn move
            // checking for pawn color move
            if curr_sq > dest_sq {
                self.bitboards[Piece::WhitePawn as usize].clear_bit(curr_sq);
                self.bitboards[Piece::WhitePawn as usize].set_bit(dest_sq);
            } else {
                self.bitboards[Piece::BlackPawn as usize].clear_bit(curr_sq);
                self.bitboards[Piece::BlackPawn as usize].set_bit(dest_sq);
            }
            return;
        }
        // checking for taking a pawn with en passant
        if dest_sq == self.en_passant
            && (self.bitboards[Piece::WhitePawn as usize].get_bit(curr_sq)
                || self.bitboards[Piece::BlackPawn as usize].get_bit(curr_sq))
        {
            self.en_passant = 64;
            // already can make move here, since we know it's a pawn move
            // checking for pawn color move
            if curr_sq > dest_sq {
                self.bitboards[Piece::WhitePawn as usize].clear_bit(curr_sq);
                self.bitboards[Piece::WhitePawn as usize].set_bit(dest_sq);
                self.bitboards[Piece::BlackPawn as usize].clear_bit(dest_sq + 8);
            } else {
                self.bitboards[Piece::BlackPawn as usize].clear_bit(curr_sq);
                self.bitboards[Piece::BlackPawn as usize].set_bit(dest_sq);
                self.bitboards[Piece::WhitePawn as usize].clear_bit(dest_sq - 8);
            }
            return;
        }
        self.en_passant = 64;

        for bitboard in self.bitboards.iter_mut() {
            // clearing bit to cover capturing
            bitboard.clear_bit(dest_sq);
            if bitboard.get_bit(curr_sq) {
                bitboard.clear_bit(curr_sq);
                bitboard.set_bit(dest_sq);
            }
        }

        // todo remove
        // self.set_w_attackmask(self.calc_w_attackmask());
        // self.set_b_attackmask(self.calc_b_attackmask());
    }

    pub fn white_castle_kingside(&mut self) {
        self.bitboards[Piece::WhiteKing as usize] ^= 0x0000_0000_0000_000Au64; // Flipping the bits of the squares affected by the Kings position
        self.bitboards[Piece::WhiteRook as usize] ^= 0x0000_0000_0000_0005u64; // Flipping the bits of the squares affected by the Rooks position
        self.castling_rights &= castling::BLACK_BOTH;
        self.en_passant = 64;
    }

    pub fn white_castle_queenside(&mut self) {
        self.bitboards[Piece::WhiteQueen as usize] ^= 0x0000_0000_0000_0028u64;
        self.bitboards[Piece::WhiteRook as usize] ^= 0x0000_0000_0000_0090u64;
        self.castling_rights &= castling::BLACK_BOTH;
        self.en_passant = 64;
    }

    pub fn black_castle_kingside(&mut self) {
        self.bitboards[Piece::BlackKing as usize] ^= 0x000A_0000_0000_0000u64;
        self.bitboards[Piece::BlackRook as usize] ^= 0x0005_0000_0000_0000u64;
        self.castling_rights &= castling::WHITE_BOTH;
        self.en_passant = 64;
    }

    pub fn black_castle_queenside(&mut self) {
        self.bitboards[Piece::BlackQueen as usize] ^= 0x0028_0000_0000_0000u64;
        self.bitboards[Piece::BlackRook as usize] ^= 0x0090_0000_0000_0000u64;
        self.castling_rights &= castling::WHITE_BOTH;
        self.en_passant = 64;
    }

    /// Copies the board, makes the move, returns new board
    pub fn with_move(mut self, mv: ChessMove) -> Self {
        self.make_move(mv);
        self
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

impl std::ops::BitXorAssign<u64> for Bitboard {
    fn bitxor_assign(&mut self, rhs: u64) {
        self.0 ^= rhs;
    }
}

impl std::ops::BitOrAssign<u64> for Bitboard {
    fn bitor_assign(&mut self, rhs: u64) {
        self.0 |= rhs;
    }
}

impl std::ops::BitOrAssign<Bitboard> for Bitboard {
    fn bitor_assign(&mut self, rhs: Bitboard) {
        self.0 |= rhs.0;
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
