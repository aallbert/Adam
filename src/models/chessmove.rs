use crate::interface::{file_to_i8, i8_to_file, i8_to_rank, rank_to_i8};

#[derive(Clone, Copy, Debug)]
/// A Chessmove in coordinate Notation decoded in a 16 Bit unsigned Integer.
///
/// - The most significant four Bit are used for encoding specalties
/// - The following six Bit represent the current square as an index 0-63
/// - The least significant six Bit represent the destinated square as an index 0-63
///
/// ### Encoding of the four MSB
/// **Used Bit:** all <br>
/// **Description:** Indicates pawn promotion <br>
/// **Usage:**<br>
/// ```0b0001_xxxxxx_yyyyyy``` Knight <br>
/// ```0b0010_xxxxxx_yyyyyy``` Bishop <br>
/// ```0b0100_xxxxxx_yyyyyy``` Rook <br>
/// ```0b1000_xxxxxx_yyyyyy``` Queen <br>

pub struct ChessMove(u16);
impl ChessMove {
    pub fn new(u: u16) -> Self {
        Self(u)
    }
    pub fn new_with_curr_and_dest(u_curr: u16, u_dest: u16) -> Self {
        Self((u_curr << 6) + u_dest)
    }
    pub fn new_with_square(curr: SquareChar, dest: SquareChar) -> Self {
        let u_from: u16 =
            (-(rank_to_i8(curr.rank) - 8) as u16 * 8 + file_to_i8(curr.file) as u16 - 1) << 6;
        let u_to: u16 = -(rank_to_i8(dest.rank) - 8) as u16 * 8 + file_to_i8(dest.file) as u16 - 1;
        Self(u_from + u_to)
    }
    pub fn new_with_str(mv: &str) -> Self {
        let mut chars = mv.chars();

        let curr_file = chars.next().unwrap();
        let curr_rank = chars.next().unwrap();
        let dest_file = chars.next().unwrap();
        let dest_rank = chars.next().unwrap();

        let curr_sq = SquareChar::new(curr_rank, curr_file);
        let dest_sq = SquareChar::new(dest_rank, dest_file);

        // encoding promotions
        if mv.len() == 5 {
            let encoding_str = chars.next().unwrap();
            let mv_int = ChessMove::new_with_square(curr_sq, dest_sq).0;
            match encoding_str {
                'n' => {
                    return ChessMove::new(0b0001_000000_000000 | mv_int);
                }
                'b' => {
                    return ChessMove::new(0b0010_000000_000000 | mv_int);
                }
                'r' => {
                    return ChessMove::new(0b0100_000000_000000 | mv_int);
                }
                'q' => {
                    return ChessMove::new(0b1000_000000_000000 | mv_int);
                }
                _ => {}
            }
        }
        ChessMove::new_with_square(curr_sq, dest_sq)
    }
    pub fn get_curr_square_as_index(&self) -> u16 {
        (self.0 & 0b0000_111111_000000) >> 6
    }
    pub fn get_dest_square_as_index(&self) -> u16 {
        self.0 & 0b0000_000000_111111
    }
    pub fn get_four_msb(&self) -> u8 {
        ((self.0 & 0b1111_000000_000000) >> 12) as u8
    }
    /// Returns the Move as a String for the UCI Protocol
    pub fn to_str(self) -> String {
        let dest_index = self.0 & 0b0000_000000_111111;
        let dest_rank = i8_to_rank(-(((dest_index >> 3) as i8 - 8) as i8));
        let dest_file = i8_to_file(((dest_index % 8) + 1) as i8);
        let curr_index = (self.0 & 0b0000_111111_000000) >> 6;
        let curr_rank = i8_to_rank(-(((curr_index >> 3) as i8 - 8) as i8));
        let curr_file = i8_to_file(((curr_index % 8) + 1) as i8);

        let mut string = String::new();
        string.push(curr_file);
        string.push(curr_rank);
        string.push(dest_file);
        string.push(dest_rank);

        let encoding = (self.0 & 0b1111_000000_000000) >> 12;
        match encoding {
            0b0001 => string.push('n'),
            0b0010 => string.push('b'),
            0b0100 => string.push('r'),
            0b1000 => string.push('q'),
            _ => {}
        };

        string
    }

    pub fn to_u16(&self) -> u16 {
        self.0
    }
}

impl std::ops::BitAnd for ChessMove {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        ChessMove(self.0 & rhs.0)
    }
}

pub enum CastleMove {}

impl CastleMove {
    pub const WHITE_K: ChessMove = ChessMove(0b0000_111100_111110);
    pub const WHITE_Q: ChessMove = ChessMove(0b0000_111100_111010);
    pub const BLACK_K: ChessMove = ChessMove(0b0000_000100_000110);
    pub const BLACK_Q: ChessMove = ChessMove(0b0000_000100_000010);
}
#[derive(Debug, Copy, Clone)]
pub struct SquareChar {
    rank: char,
    file: char,
}
impl SquareChar {
    pub fn new(rank: char, file: char) -> Self {
        Self { rank, file }
    }
}