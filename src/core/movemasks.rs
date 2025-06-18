use crate::{
    interface::abs_diff_u16,
    models::{
        board::{Bitboard, ChessBoard},
        piece::Piece,
    },
};

impl ChessBoard {
    /// Calculates the attackmask for every piece white piece. <br>
    /// The attackmask shows, which squares a piece could capture, if a piece of the enemy color was on it.
    pub fn calc_w_attackmask(&self) -> Bitboard {
        let mut attackmask = Bitboard::new(0);

        let w_pawns = self.get_bitboard(Piece::WhitePawn as usize);
        let w_bishops = self.get_bitboard(Piece::WhiteBishop as usize);
        let w_knights = self.get_bitboard(Piece::WhiteKnight as usize);
        let w_rooks = self.get_bitboard(Piece::WhiteRook as usize);
        let w_queens = self.get_bitboard(Piece::WhiteQueen as usize);
        let w_king = self.get_bitboard(Piece::WhiteKing as usize);

        for sq_index in w_pawns {
            attackmask |= ChessBoard::calc_w_pawn_attackmask(sq_index);
        }
        for sq_index in w_bishops {
            attackmask |= self.calc_bishop_attackmask(sq_index);
        }
        for sq_index in w_knights {
            attackmask |= ChessBoard::calc_knight_attackmask(sq_index);
        }
        for sq_index in w_rooks {
            attackmask |= self.calc_rook_attackmask(sq_index);
        }
        for sq_index in w_queens {
            attackmask |= self.calc_queen_attackmask(sq_index);
        }
        for sq_index in w_king {
            attackmask |= ChessBoard::calc_king_attackmask(sq_index);
        }

        attackmask
    }

    /// Calculates the attackmask for every black piece
    pub fn calc_b_attackmask(&self) -> Bitboard {
        let mut attackmask = Bitboard::new(0);

        let b_pawns = self.get_bitboard(Piece::BlackPawn as usize);
        let b_bishops = self.get_bitboard(Piece::BlackBishop as usize);
        let b_knights = self.get_bitboard(Piece::BlackKnight as usize);
        let b_rooks = self.get_bitboard(Piece::BlackRook as usize);
        let b_queens = self.get_bitboard(Piece::BlackQueen as usize);
        let b_king = self.get_bitboard(Piece::BlackKing as usize);

        for sq_index in b_pawns {
            attackmask |= ChessBoard::calc_b_pawn_attackmask(sq_index);
        }
        for sq_index in b_bishops {
            attackmask |= self.calc_bishop_attackmask(sq_index);
        }
        for sq_index in b_knights {
            attackmask |= ChessBoard::calc_knight_attackmask(sq_index);
        }
        for sq_index in b_rooks {
            attackmask |= self.calc_rook_attackmask(sq_index);
        }
        for sq_index in b_queens {
            attackmask |= self.calc_queen_attackmask(sq_index);
        }
        for sq_index in b_king {
            attackmask |= ChessBoard::calc_king_attackmask(sq_index);
        }

        attackmask
    }
    pub fn calc_w_pawn_movemask(&self, sq_index: u16) -> Bitboard {
        let mut movemask = Bitboard::new(0);
        let all_pieces = self.get_all_pieces();
        if !all_pieces.get_bit(sq_index - 8) {

            movemask.set_bit(sq_index - 8);
            if sq_index >> 3 == 6 && !all_pieces.get_bit(sq_index - 16) {
                movemask.set_bit(sq_index - 16);
            }
        }
        movemask
    }

    pub fn calc_w_pawn_attackmask(sq_index: u16) -> Bitboard {
        let mut attackmask = Bitboard::new(0);

        match sq_index & 0b111 {
            0 => {
                attackmask.set_bit(sq_index - 7);
            }
            7 => {
                attackmask.set_bit(sq_index - 9);
            }
            _ => {
                attackmask.set_bit(sq_index - 7);
                attackmask.set_bit(sq_index - 9);
            }
        }
        attackmask
    }

    pub fn calc_b_pawn_movemask(&self, sq_index: u16) -> Bitboard {
        let mut movemask = Bitboard::new(0);
        let all_pieces = self.get_all_pieces();
        if !all_pieces.get_bit(sq_index + 8) {

            movemask.set_bit(sq_index + 8);
            if sq_index >> 3 == 1 && !all_pieces.get_bit(sq_index + 16) {
                movemask.set_bit(sq_index + 16);
            }
        }
        movemask
    }

    pub fn calc_b_pawn_attackmask(sq_index: u16) -> Bitboard {
        let mut attackmask = Bitboard::new(0);

        match sq_index & 0b111 {
            0 => {
                attackmask.set_bit(sq_index + 9);
            }
            7 => {
                attackmask.set_bit(sq_index + 7);
            }
            _ => {
                attackmask.set_bit(sq_index + 9);
                attackmask.set_bit(sq_index + 7);
            }
        }
        attackmask
    }

    pub fn calc_bishop_attackmask(&self, sq_index: u16) -> Bitboard {
        let mut attackmask = Bitboard::new(0);
        let all_pieces = self.get_all_pieces();

        if (sq_index & 0b111) != 7 {
            // moving to upper right
            // Checking if piece is on 8th rank
            if sq_index > 7 {
                let mut dest_index = sq_index - 7;
                loop {
                    attackmask.set_bit(dest_index);
                    // Checking if a piece is on the new position of the Bishop
                    if all_pieces.get_bit(dest_index) || dest_index & 0b111 == 7 || dest_index < 8 {
                        break;
                    }
                    dest_index -= 7;
                }
            }

            // moving to bottom right
            // Checking if piece is on 1st rank
            if sq_index < 56 {
                let mut dest_index = sq_index + 9;
                loop {
                    attackmask.set_bit(dest_index);
                    // Checking if a black piece is on the new position of the Bishop
                    if all_pieces.get_bit(dest_index) || dest_index & 0b111 == 7 || dest_index > 55
                    {
                        break;
                    }
                    dest_index += 9;
                }
            }
        }

        if (sq_index & 0b111) != 0 {
            // moving to upper left
            // Checking if piece is on 8th rank
            if sq_index > 7 {
                let mut dest_index = sq_index - 9;
                loop {
                    attackmask.set_bit(dest_index);
                    // Checking if a black piece is on the new position of the Bishop
                    if all_pieces.get_bit(dest_index) || dest_index & 0b111 == 0 || dest_index < 8 {
                        break;
                    }
                    dest_index -= 9;
                }
            }

            // moving to bottom left
            if sq_index < 56 {
                let mut dest_index = sq_index + 7;
                loop {
                    attackmask.set_bit(dest_index);
                    // Checking if a black piece is on the new position of the Bishop
                    if all_pieces.get_bit(dest_index) || dest_index & 0b111 == 0 || dest_index > 55
                    {
                        break;
                    }
                    dest_index += 7;
                }
            }
        }
        attackmask
    }

    pub fn calc_knight_attackmask(sq_index: u16) -> Bitboard {
        let mut attackmask = Bitboard::new(0);

        let dest_index_offsets_pos: [u16; 4] = [6, 10, 15, 17];
        for offset in dest_index_offsets_pos {
            let dest_index = sq_index + offset;
            // Checking if the move would lead below the board
            if dest_index > 63 {
                break;
            }
            // Checking if the move would lead over the border
            if abs_diff_u16(dest_index & 0b111, sq_index & 0b111) > 2 {
                continue;
            }
            attackmask.set_bit(dest_index);
        }
        let dest_index_offsets_neg: [i16; 4] = [-6, -10, -15, -17];
        for offset in dest_index_offsets_neg {
            let dest_index_i16 = sq_index as i16 + offset;
            // Checking if the move would lead above the board
            if dest_index_i16 < 0 {
                break;
            }
            let dest_index = dest_index_i16 as u16;
            // Checking if the move would lead over the border
            if abs_diff_u16(dest_index & 0b111, sq_index & 0b111) > 2 {
                continue;
            }
            attackmask.set_bit(dest_index);
        }
        attackmask
    }

    pub fn calc_rook_attackmask(&self, sq_index: u16) -> Bitboard {
        let mut attackmask = Bitboard::new(0);
        let all_pieces = self.get_all_pieces();

        // Moving right
        // Upper boundary of for loop is the index of h file of the rank of the rook
        for dest_index in (sq_index + 1)..=(sq_index | 0b111) {
            attackmask.set_bit(dest_index);
            // Checking if a piece is on the new position of the Rook
            if all_pieces.get_bit(dest_index) {
                break;
            }
        }

        // Moving left
        for dest_index in ((sq_index & 0xFFF8)..sq_index).rev() {
            attackmask.set_bit(dest_index);
            // Checking if a piece is on the new position of the Rook
            if all_pieces.get_bit(dest_index) {
                break;
            }
        }

        // Moving up
        for i in (0..(sq_index >> 3)).rev() {
            let dest_index = i * 8 + (sq_index & 0b111);
            attackmask.set_bit(dest_index);
            // Checking if a piece is on the new position of the Rook
            if all_pieces.get_bit(dest_index) {
                break;
            }
        }

        // Moving down
        for i in (sq_index >> 3) + 1..8 {
            let dest_index = i * 8 + (sq_index & 0b111);
            attackmask.set_bit(dest_index);
            // Checking if a piece is on the new position of the Rook
            if all_pieces.get_bit(dest_index) {
                break;
            }
        }
        attackmask
    }

    pub fn calc_queen_attackmask(&self, sq_index: u16) -> Bitboard {
        self.calc_bishop_attackmask(sq_index) | self.calc_rook_attackmask(sq_index)
    }

    pub fn calc_king_attackmask(sq_index: u16) -> Bitboard {
        let mut attackmask = Bitboard::new(0);
        let dest_index_offsets_pos: [u16; 4] = [1, 7, 8, 9];
        for offset in dest_index_offsets_pos {
            let dest_index = sq_index + offset;
            // Checking if the move would lead below the board
            if dest_index > 63 {
                break;
            }
            // Checking if the move would lead over the border
            if abs_diff_u16(dest_index & 0b111, sq_index & 0b111) > 1 {
                continue;
            }
            attackmask.set_bit(dest_index);
        }
        let dest_index_offsets_neg: [i16; 4] = [-1, -7, -8, -9];
        for offset in dest_index_offsets_neg {
            let dest_index_i16 = sq_index as i16 + offset;
            // Checking if the move would lead above the board
            if dest_index_i16 < 0 {
                break;
            }
            let dest_index = dest_index_i16 as u16;
            // Checking if the move would lead over the border
            if abs_diff_u16(dest_index & 0b111, sq_index & 0b111) > 1 {
                continue;
            }
            attackmask.set_bit(dest_index);
        }
        attackmask
    }
}
