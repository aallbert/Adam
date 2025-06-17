use crate::{
    interface::abs_diff_u16,
    models::{
        board::{Bitboard, ChessBoard},
        piece::Piece,
    },
};

impl ChessBoard {
    pub fn calc_w_attackmask(&self) -> Bitboard {
        let mut attackmask = Bitboard::new(0);

        let all_boards = self.get_bitboards();
        let all_pieces = self.get_all_pieces();

        let w_pawns = all_boards[Piece::WhitePawn as usize];
        let w_bishops = all_boards[Piece::WhiteBishop as usize];
        let w_knights = all_boards[Piece::WhiteKnight as usize];
        let w_rooks = all_boards[Piece::WhiteRook as usize];
        let w_queens = all_boards[Piece::WhiteQueen as usize];
        let w_king = all_boards[Piece::WhiteKing as usize];
        for sq_index in w_pawns {
            if sq_index > 7 {
                match sq_index | 0b111 {
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
            }
        }
        for sq_index in w_bishops {
            if (sq_index & 0b111) != 7 {
                // moving to upper right
                // Checking if piece is on 8th rank
                if sq_index > 7 {
                    let mut dest_index = sq_index - 7;
                    loop {
                        attackmask.set_bit(dest_index);
                        // Checking if a piece is on the new position of the Bishop
                        if all_pieces.get_bit(dest_index)
                            || dest_index & 0b111 == 7
                            || dest_index < 8
                        {
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
                        if all_pieces.get_bit(dest_index)
                            || dest_index & 0b111 == 7
                            || dest_index > 55
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
                        if all_pieces.get_bit(dest_index)
                            || dest_index & 0b111 == 0
                            || dest_index < 8
                        {
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
                        if all_pieces.get_bit(dest_index)
                            || dest_index & 0b111 == 0
                            || dest_index > 55
                        {
                            break;
                        }
                        dest_index += 7;
                    }
                }
            }
        }
        for sq_index in w_knights {
            let dest_index_offsets_pos: [u16; 4] = [6, 10, 15, 17];
            for offset in dest_index_offsets_pos {
                let dest_index = sq_index + offset;
                // Checking if the move would lead below the board
                if dest_index > 63 {
                    break;
                }
                // Checking if the move would lead over the border
                if abs_diff_u16(dest_index & 0b111, (sq_index & 0b111)) > 2 {
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
                if abs_diff_u16(dest_index & 0b111, (sq_index & 0b111)) > 2 {
                    continue;
                }
                attackmask.set_bit(dest_index);
            }
        }
        for sq_index in w_rooks {
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
        }
        for sq_index in w_queens {
            if (sq_index & 0b111) != 7 {
                // moving to upper right
                // Checking if piece is on 8th rank
                if sq_index > 7 {
                    let mut dest_index = sq_index - 7;
                    loop {
                        attackmask.set_bit(dest_index);
                        // Checking if a piece is on the new position of the Queen
                        if all_pieces.get_bit(dest_index)
                            || dest_index & 0b111 == 7
                            || dest_index < 8
                        {
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
                        // Checking if a black piece is on the new position of the Queen
                        if all_pieces.get_bit(dest_index)
                            || dest_index & 0b111 == 7
                            || dest_index > 55
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
                        // Checking if a black piece is on the new position of the Queen
                        if all_pieces.get_bit(dest_index)
                            || dest_index & 0b111 == 0
                            || dest_index < 8
                        {
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
                        // Checking if a black piece is on the new position of the Queen
                        if all_pieces.get_bit(dest_index)
                            || dest_index & 0b111 == 0
                            || dest_index > 55
                        {
                            break;
                        }
                        dest_index += 7;
                    }
                }
            }
            // Moving right
            // Upper boundary of for loop is the index of h file of the rank of the Queen
            for dest_index in (sq_index + 1)..=sq_index | 0b111 {
                attackmask.set_bit(dest_index);
                // Checking if a piece is on the new position of the Queen
                if all_pieces.get_bit(dest_index) {
                    break;
                }
            }

            // Moving left
            for dest_index in ((sq_index & 0xFFF8)..sq_index).rev() {
                attackmask.set_bit(dest_index);
                // Checking if a piece is on the new position of the Queen
                if all_pieces.get_bit(dest_index) {
                    break;
                }
            }

            // Moving up
            for i in (0..(sq_index >> 3)).rev() {
                let dest_index = i * 8 + (sq_index & 0b111);
                attackmask.set_bit(dest_index);
                // Checking if a piece is on the new position of the Queen
                if all_pieces.get_bit(dest_index) {
                    break;
                }
            }

            // Moving down
            for i in (sq_index >> 3) + 1..8 {
                let dest_index = i * 8 + (sq_index & 0b111);
                attackmask.set_bit(dest_index);
                // Checking if a piece is on the new position of the Queen
                if all_pieces.get_bit(dest_index) {
                    break;
                }
            }
        }
        for sq_index in w_king {
            let dest_index_offsets_pos: [u16; 4] = [1, 7, 8, 9];
            for offset in dest_index_offsets_pos {
                let dest_index = sq_index + offset;
                // Checking if the move would lead below the board
                if dest_index > 63 {
                    break;
                }
                // Checking if the move would lead over the border
                if abs_diff_u16(dest_index & 0b111, (sq_index & 0b111)) > 1 {
                    continue;
                }
                attackmask.set_bit(sq_index);
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
                if abs_diff_u16(dest_index & 0b111, (sq_index & 0b111)) > 1 {
                    continue;
                }
                attackmask.set_bit(sq_index);
            }
        }
        return attackmask;
    }

    pub fn calc_b_attackmask(&self) -> Bitboard {
        let mut attackmask = Bitboard::new(0);

        let all_boards = self.get_bitboards();
        let all_pieces = self.get_all_pieces();

        let b_pawns = all_boards[Piece::BlackPawn as usize];
        let b_bishops = all_boards[Piece::BlackBishop as usize];
        let b_knights = all_boards[Piece::BlackKnight as usize];
        let b_rooks = all_boards[Piece::BlackRook as usize];
        let b_queens = all_boards[Piece::BlackQueen as usize];
        let b_king = all_boards[Piece::BlackKing as usize];
        for sq_index in b_pawns {
            if sq_index > 7 {
                match sq_index | 0b111 {
                    0 => {
                        attackmask.set_bit(sq_index + 9);
                    }
                    7 => {
                        attackmask.set_bit(sq_index + 7);
                    }
                    _ => {
                        attackmask.set_bit(sq_index + 7);
                        attackmask.set_bit(sq_index + 9);
                    }
                }
            }
        }
        for sq_index in b_bishops {
            if (sq_index & 0b111) != 7 {
                // moving to upper right
                // Checking if piece is on 8th rank
                if sq_index > 7 {
                    let mut dest_index = sq_index - 7;
                    loop {
                        attackmask.set_bit(dest_index);
                        // Checking if a piece is on the new position of the Bishop
                        if all_pieces.get_bit(dest_index)
                            || dest_index & 0b111 == 7
                            || dest_index < 8
                        {
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
                        if all_pieces.get_bit(dest_index)
                            || dest_index & 0b111 == 7
                            || dest_index > 55
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
                        if all_pieces.get_bit(dest_index)
                            || dest_index & 0b111 == 0
                            || dest_index < 8
                        {
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
                        if all_pieces.get_bit(dest_index)
                            || dest_index & 0b111 == 0
                            || dest_index > 55
                        {
                            break;
                        }
                        dest_index += 7;
                    }
                }
            }
        }
        for sq_index in b_knights {
            let dest_index_offsets_pos: [u16; 4] = [6, 10, 15, 17];
            for offset in dest_index_offsets_pos {
                let dest_index = sq_index + offset;
                // Checking if the move would lead below the board
                if dest_index > 63 {
                    break;
                }
                // Checking if the move would lead over the border
                if abs_diff_u16(dest_index & 0b111, (sq_index & 0b111)) > 2 {
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
                if abs_diff_u16(dest_index & 0b111, (sq_index & 0b111)) > 2 {
                    continue;
                }
                attackmask.set_bit(dest_index);
            }
        }
        for sq_index in b_rooks {
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
        }
        for sq_index in b_queens {
            if (sq_index & 0b111) != 7 {
                // moving to upper right
                // Checking if piece is on 8th rank
                if sq_index > 7 {
                    let mut dest_index = sq_index - 7;
                    loop {
                        attackmask.set_bit(dest_index);
                        // Checking if a piece is on the new position of the Queen
                        if all_pieces.get_bit(dest_index)
                            || dest_index & 0b111 == 7
                            || dest_index < 8
                        {
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
                        // Checking if a black piece is on the new position of the Queen
                        if all_pieces.get_bit(dest_index)
                            || dest_index & 0b111 == 7
                            || dest_index > 55
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
                        // Checking if a black piece is on the new position of the Queen
                        if all_pieces.get_bit(dest_index)
                            || dest_index & 0b111 == 0
                            || dest_index < 8
                        {
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
                        // Checking if a black piece is on the new position of the Queen
                        if all_pieces.get_bit(dest_index)
                            || dest_index & 0b111 == 0
                            || dest_index > 55
                        {
                            break;
                        }
                        dest_index += 7;
                    }
                }
            }
            // Moving right
            // Upper boundary of for loop is the index of h file of the rank of the Queen
            for dest_index in (sq_index + 1)..=(sq_index | 0b111) {
                attackmask.set_bit(dest_index);
                // Checking if a piece is on the new position of the Queen
                if all_pieces.get_bit(dest_index) {
                    break;
                }
            }

            // Moving left
            for dest_index in ((sq_index & 0xFFF8)..sq_index).rev() {
                attackmask.set_bit(dest_index);
                // Checking if a piece is on the new position of the Queen
                if all_pieces.get_bit(dest_index) {
                    break;
                }
            }

            // Moving up
            for i in (0..(sq_index >> 3)).rev() {
                let dest_index = i * 8 + (sq_index & 0b111);
                attackmask.set_bit(dest_index);
                // Checking if a piece is on the new position of the Queen
                if all_pieces.get_bit(dest_index) {
                    break;
                }
            }

            // Moving down
            for i in (sq_index >> 3) + 1..8 {
                let dest_index = i * 8 + (sq_index & 0b111);
                attackmask.set_bit(dest_index);
                // Checking if a piece is on the new position of the Queen
                if all_pieces.get_bit(dest_index) {
                    break;
                }
            }
        }
        for sq_index in b_king {
            let dest_index_offsets_pos: [u16; 4] = [1, 7, 8, 9];
            for offset in dest_index_offsets_pos {
                let dest_index = sq_index + offset;
                // Checking if the move would lead below the board
                if dest_index > 63 {
                    break;
                }
                // Checking if the move would lead over the border
                if abs_diff_u16(dest_index & 0b111, (sq_index & 0b111)) > 1 {
                    continue;
                }
                attackmask.set_bit(sq_index);
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
                if abs_diff_u16(dest_index & 0b111, (sq_index & 0b111)) > 1 {
                    continue;
                }
                attackmask.set_bit(sq_index);
            }
        }
        return attackmask;
    }
}
