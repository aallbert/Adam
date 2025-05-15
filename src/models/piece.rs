pub enum Piece {
    WhitePawn = 0,
    WhiteBishop = 1,
    WhiteKnight = 2,
    WhiteRook = 3,
    WhiteKing = 4,
    WhiteQueen = 5,
    BlackPawn = 6,
    BlackBishop = 7,
    BlackKnight = 8,
    BlackRook = 9,
    BlackKing = 10,
    BlackQueen = 11,
}

impl Piece {
    pub fn to_char(self) -> char {
        match self {
            Piece::WhitePawn => 'P',
            Piece::WhiteBishop => 'B',
            Piece::WhiteKnight => 'N',
            Piece::WhiteRook => 'R',
            Piece::WhiteKing => 'K',
            Piece::WhiteQueen => 'Q',
            Piece::BlackPawn => 'p',
            Piece::BlackBishop => 'b',
            Piece::BlackKnight => 'n',
            Piece::BlackRook => 'r',
            Piece::BlackKing => 'k',
            Piece::BlackQueen => 'q',
        }
    }
}

impl From<Piece> for u8 {
    fn from(p: Piece) -> u8 {
        p as u8
    }
}

impl TryFrom<u8> for Piece {
    type Error = ();
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        use Piece::*;
        Ok(match val {
            0 => WhitePawn,
            1 => WhiteBishop,
            2 => WhiteKnight,
            3 => WhiteRook,
            4 => WhiteKing,
            5 => WhiteQueen,
            6 => BlackPawn,
            7 => BlackBishop,
            8 => BlackKnight,
            9 => BlackRook,
            10 => BlackKing,
            11 => BlackQueen,
            _ => return Err(()),
        })
    }
}

pub mod castling {
    pub const WHITE_K: u8 = 0b0001;
    pub const WHITE_Q: u8 = 0b0010;
    pub const BLACK_K: u8 = 0b0100;
    pub const BLACK_Q: u8 = 0b1000;

    pub const ALL: u8 = 0b1111;
}

pub enum CastleRights {
    WhiteShort = castling::WHITE_K as isize,
    WhiteLong  = castling::WHITE_Q as isize,
    BlackShort = castling::BLACK_K as isize,
    BlackLong  = castling::BLACK_Q as isize,
}