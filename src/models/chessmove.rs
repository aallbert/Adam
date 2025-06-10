use crate::interface::{file_to_i8, i8_to_file, i8_to_rank, rank_to_i8};

#[derive(Clone, Copy, Debug)]
/// struct representing a Square as a u16 with a value between 0..=63
pub struct Square(u16);
impl Square {
    pub fn new(u: u16) -> Self {
        // if !(0..=63).contains(&u) {
        //     panic!("Indexing out of range!");
        // }
        Self(u)
    }
    pub fn get_rank_as_index(&self) -> u16 {
        self.0 >> 3
    }
    pub fn get_file_as_index(&self) -> u16 {
        // A & 7 = A mod 8
        self.0 & 0b111
    }
    pub fn get_as_u16(&self) -> u16 {
        self.0
    }
    pub fn get_as_str(&self) -> String {
        let rank = match self.get_rank_as_index() {
            0 => '8',
            1 => '7',
            2 => '6',
            3 => '5',
            4 => '4',
            5 => '3',
            6 => '2',
            7 => '1',
            _ => panic!("out of bounds"),
        };
        let file = match self.get_file_as_index() {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => panic!("out of bounds"),
        };
        format!("{}{}", rank, file)
    }
}

#[derive(Clone, Copy, Debug)]
/// A Chessmove in coordinate Notation decoded in a 16 Bit unsigned Integer.
///
/// - The most significant four Bit are used for encoding specalties
/// - The following six Bit represent the current square as an index 0-63
/// - The least significant six Bit represent the destinated square as an index 0-63
///
/// ### Encoding of the four MSB
/// **Used Bit:** MSB <br>
/// **Description:** Indicates Castling <br>
/// **Usage:** ```0b1000_000000_00xxxx``` castles depending on which x-Bit is set
///
///
/// ### Reserved Move values
/// | Value | Description |
/// |:-----------|:-----------|
/// | Header | Title |
/// | Paragraph | Text |
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

        ChessMove::new_with_square(curr_sq, dest_sq)
    }
    pub fn set(&mut self, u: u16) {
        self.0 = u;
    }
    pub fn set_with_square(&mut self, curr: SquareChar, dest: SquareChar) {
        let u_from: u16 =
            (-(rank_to_i8(curr.rank) - 8) as u16 * 8 + file_to_i8(curr.file) as u16 - 1) << 6;
        let u_to: u16 = -(rank_to_i8(dest.rank) - 8) as u16 * 8 + file_to_i8(dest.file) as u16 - 1;
        self.0 = u_from + u_to
    }
    pub fn get_u16(&self) -> u16 {
        self.0
    }
    /// Returns a value between 0..=63
    pub fn get_curr_square_as_index(&self) -> u16 {
        (self.0 & 0b0000111111000000) >> 6
    }
    /// Returns a value between 0..=63
    pub fn get_dest_square_as_index(&self) -> u16 {
        self.0 & 0b0000000000111111
    }
    /// Returns a Square struct
    pub fn get_curr_square_as_struct(&self) -> Square {
        Square::new((self.0 & 0b0000111111000000) >> 6)
    }
    /// Returns a Square struct
    pub fn get_dest_square_as_struct(&self) -> Square {
        Square::new(self.0 & 0b0000000000111111)
    }
    pub fn get_curr_square_as_struct_char(&self) -> SquareChar {
        let index = (self.0 & 0b0000111111000000) >> 6;
        // println!("index: {}", index);
        let rank = i8_to_rank(-(((index >> 3) as i8 - 8) as i8));
        let file = i8_to_file(((index % 8) + 1) as i8);
        SquareChar {
            rank: rank,
            file: file,
        }
    }
    pub fn get_dest_square_as_struct_char(&self) -> SquareChar {
        let index = self.0 & 0b0000000000111111;
        let rank = i8_to_rank(-(((index >> 3) as i8 - 8) as i8));
        let file = i8_to_file(((index % 8) + 1) as i8);
        SquareChar {
            rank: rank,
            file: file,
        }
    }
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

        string
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
    pub fn get_rank(&self) -> char {
        self.rank
    }
    pub fn get_file(&self) -> char {
        self.file
    }
}

pub struct ChessMoveChar {
    curr: SquareChar,
    dest: SquareChar,
}
impl ChessMoveChar {
    pub fn new(curr: SquareChar, dest: SquareChar) -> Self {
        Self { curr, dest }
    }
    pub fn new_with_chars(
        curr_rank: char,
        curr_file: char,
        dest_rank: char,
        dest_file: char,
    ) -> Self {
        Self {
            curr: SquareChar {
                rank: curr_rank,
                file: curr_file,
            },
            dest: SquareChar {
                rank: dest_rank,
                file: dest_file,
            },
        }
    }
    pub fn to_chessmove(self) -> ChessMove {
        ChessMove::new_with_square(self.curr, self.dest)
    }
}
