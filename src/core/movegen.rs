use crate::interface::abs_diff_u16;
use crate::models::board::{Bitboard, ChessBoard};
use crate::models::chessmove::{CastleMove, ChessMove, Square};
use crate::models::piece::{Piece, castling};

impl ChessBoard {
    pub fn possible_moves(&self) -> Vec<ChessMove> {
        let all_boards = self.get_bitboards();
        let player_boards: [Bitboard; 6] = if self.get_white_to_move() {
            all_boards[0..6].try_into().unwrap()
        } else {
            all_boards[6..12].try_into().unwrap()
        };
        let index_offset = if self.get_white_to_move() { 0 } else { 6 };
        let b_pieces_u64 = all_boards[6..12]
            .iter()
            .fold(0u64, |acc, b| acc | b.to_u64());
        let w_pieces_u64 = all_boards[0..6]
            .iter()
            .fold(0u64, |acc, b| acc | b.to_u64());
        let b_pieces = Bitboard::new(b_pieces_u64);
        let w_pieces = Bitboard::new(w_pieces_u64);
        let all_pieces: Bitboard = self.get_all_pieces();

        let w_attackmask = self.calc_w_attackmask();
        let b_attackmask = self.calc_b_attackmask();

        let mut all_moves: Vec<ChessMove> = Vec::new();

        for (i, bitboard) in (index_offset..).zip(player_boards.iter()) {
            let pc = Piece::try_from(i as u8).unwrap();
            for sq_index in bitboard {
                let curr_square = Square::new(sq_index);
                let curr_rank_index = curr_square.get_rank_as_index();
                let curr_file_index = curr_square.get_file_as_index();
                let curr_index = sq_index; // todo: remove

                match pc {
                    //WHITE PIECES
                    Piece::WhitePawn => {
                        //normal move
                        //check if the square in front of the pawn is free
                        let dest_index = curr_index - 8;
                        if !all_pieces.get_bit(dest_index) && curr_rank_index != 1 {
                            all_moves
                                .push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                            //check if pawn is on first move
                            if curr_rank_index == 6 {
                                let dest_index = curr_index - 16;
                                if !all_pieces.get_bit(dest_index) {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        curr_index, dest_index,
                                    ));
                                }
                            }
                        }
                        //checking if pawn is on seventh rank; if not, then promoting
                        else if !all_pieces.get_bit(dest_index) && curr_rank_index == 1 {
                            all_moves.push(ChessMove::new_with_curr_and_dest(
                                0b0001_000000 | curr_index,
                                dest_index,
                            ));
                            all_moves.push(ChessMove::new_with_curr_and_dest(
                                0b0010_000000 | curr_index,
                                dest_index,
                            ));
                            all_moves.push(ChessMove::new_with_curr_and_dest(
                                0b0100_000000 | curr_index,
                                dest_index,
                            ));
                            all_moves.push(ChessMove::new_with_curr_and_dest(
                                0b1000_000000 | curr_index,
                                dest_index,
                            ));
                        }
                        //capture right
                        if curr_file_index < 7 {
                            let dest_index = curr_index - 7;
                            if b_pieces.get_bit(dest_index) {
                                if curr_rank_index == 1 {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        0b0001_000000 | curr_index,
                                        dest_index,
                                    ));
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        0b0010_000000 | curr_index,
                                        dest_index,
                                    ));
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        0b0100_000000 | curr_index,
                                        dest_index,
                                    ));
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        0b1000_000000 | curr_index,
                                        dest_index,
                                    ));
                                } else {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        curr_index, dest_index,
                                    ));
                                }
                            }
                            if dest_index == self.get_en_passant() {
                                all_moves.push(ChessMove::new_with_curr_and_dest(
                                    curr_index, dest_index,
                                ));
                            }
                        }
                        //capture left
                        if curr_file_index > 0 {
                            let dest_index = curr_index - 9;
                            if b_pieces.get_bit(dest_index) {
                                if curr_rank_index == 1 {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        0b0001_000000 | curr_index,
                                        dest_index,
                                    ));
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        0b0010_000000 | curr_index,
                                        dest_index,
                                    ));
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        0b0100_000000 | curr_index,
                                        dest_index,
                                    ));
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        0b1000_000000 | curr_index,
                                        dest_index,
                                    ));
                                } else {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        curr_index, dest_index,
                                    ));
                                }
                            }
                            if dest_index == self.get_en_passant() {
                                all_moves.push(ChessMove::new_with_curr_and_dest(
                                    curr_index, dest_index,
                                ));
                            }
                        }
                    }
                    Piece::WhiteBishop => {
                        // Bitwise AND of (n-1) is equal to mod(n)
                        // Checking if piece is on h file
                        if curr_index & 0b111 != 7 {
                            // moving to upper right
                            // Checking if piece is on 8th rank
                            if curr_index > 7 {
                                let mut dest_index = curr_index - 7;
                                while !w_pieces.get_bit(dest_index) {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        curr_index, dest_index,
                                    ));
                                    // Checking if a black piece is on the new position of the Bishop
                                    if b_pieces.get_bit(dest_index)
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
                            if curr_index < 56 {
                                let mut dest_index = curr_index + 9;
                                while !w_pieces.get_bit(dest_index) {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        curr_index, dest_index,
                                    ));
                                    // Checking if a black piece is on the new position of the Bishop
                                    if b_pieces.get_bit(dest_index)
                                        || dest_index & 0b111 == 7
                                        || dest_index > 55
                                    {
                                        break;
                                    }
                                    dest_index += 9;
                                }
                            }
                        }

                        if curr_index & 0b111 != 0 {
                            // moving to upper left
                            // Checking if piece is on 8th rank
                            if curr_index > 7 {
                                let mut dest_index = curr_index - 9;
                                while !w_pieces.get_bit(dest_index) {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        curr_index, dest_index,
                                    ));
                                    // Checking if a black piece is on the new position of the Bishop
                                    if b_pieces.get_bit(dest_index)
                                        || dest_index & 0b111 == 0
                                        || dest_index < 8
                                    {
                                        break;
                                    }
                                    dest_index -= 9;
                                }
                            }

                            // moving to bottom left
                            if curr_index < 56 {
                                let mut dest_index = curr_index + 7;
                                while !w_pieces.get_bit(dest_index) {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        curr_index, dest_index,
                                    ));
                                    // Checking if a black piece is on the new position of the Bishop
                                    if b_pieces.get_bit(dest_index)
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
                    Piece::WhiteKnight => {
                        let dest_index_offsets_pos: [u16; 4] = [6, 10, 15, 17];
                        for offset in dest_index_offsets_pos {
                            let dest_index = curr_index + offset;
                            // Checking if the move would lead below the board
                            if dest_index > 63 {
                                break;
                            }
                            // Checking if the move would lead over the border
                            // a XOR b = |a - b|
                            if abs_diff_u16(dest_index & 0b111, curr_file_index) > 2 {
                                continue;
                            }
                            if !w_pieces.get_bit(dest_index) {
                                all_moves.push(ChessMove::new_with_curr_and_dest(
                                    curr_index, dest_index,
                                ));
                            }
                        }
                        let dest_index_offsets_neg: [i16; 4] = [-6, -10, -15, -17];
                        for offset in dest_index_offsets_neg {
                            let dest_index_i16 = curr_index as i16 + offset;
                            // Checking if the move would lead above the board
                            if dest_index_i16 < 0 {
                                break;
                            }
                            let dest_index = dest_index_i16 as u16;
                            // Checking if the move would lead over the border
                            if abs_diff_u16(dest_index & 0b111, curr_file_index) > 2 {
                                continue;
                            }
                            if !w_pieces.get_bit(dest_index) {
                                all_moves.push(ChessMove::new_with_curr_and_dest(
                                    curr_index, dest_index,
                                ));
                            }
                        }
                    }
                    Piece::WhiteRook => {
                        // Moving right
                        // Upper boundary of for loop is the index of h file of the rank of the rook
                        for dest_index in (curr_index + 1)..=curr_index | 0b111 {
                            // Checking if a white piece is on the right of the Rook
                            if w_pieces.get_bit(dest_index) {
                                break;
                            }
                            all_moves
                                .push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                            // Checking if a black piece is on the new position of the Rook
                            if b_pieces.get_bit(dest_index) {
                                break;
                            }
                        }

                        // Moving left
                        for dest_index in (curr_index & 0xFFF8..curr_index).rev() {
                            // Checking if a white piece is on the left of the Rook
                            if w_pieces.get_bit(dest_index) {
                                break;
                            }
                            all_moves
                                .push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                            // Checking if a black piece is on the new position of the Rook
                            if b_pieces.get_bit(dest_index) {
                                break;
                            }
                        }

                        // Moving up
                        for i in (0..(curr_index >> 3)).rev() {
                            let dest_index = i * 8 + curr_file_index;
                            // Checking if a white piece is above the Rook
                            if w_pieces.get_bit(dest_index) {
                                break;
                            }
                            all_moves
                                .push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                            // Checking if a black piece is on the new position of the Rook
                            if b_pieces.get_bit(dest_index) {
                                break;
                            }
                        }

                        // Moving down
                        for i in (curr_index >> 3) + 1..8 {
                            let dest_index = i * 8 + curr_file_index;
                            // Checking if a white piece is below the Rook
                            if w_pieces.get_bit(dest_index) {
                                break;
                            }
                            all_moves
                                .push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                            // Checking if a black piece is on the new position of the Rook
                            if b_pieces.get_bit(dest_index) {
                                break;
                            }
                        }
                    }
                    Piece::WhiteKing => {
                        let dest_index_offsets_pos: [u16; 4] = [1, 7, 8, 9];
                        for offset in dest_index_offsets_pos {
                            let dest_index = curr_index + offset;
                            // Checking if the move would lead below the board
                            if dest_index > 63 {
                                break;
                            }
                            // Checking if the move would lead over the border
                            if abs_diff_u16(dest_index & 0b111, curr_file_index) > 1 {
                                continue;
                            }
                            if !w_pieces.get_bit(dest_index) {
                                all_moves.push(ChessMove::new_with_curr_and_dest(
                                    curr_index, dest_index,
                                ));
                            }
                        }
                        let dest_index_offsets_neg: [i16; 4] = [-1, -7, -8, -9];
                        for offset in dest_index_offsets_neg {
                            let dest_index_i16 = curr_index as i16 + offset;
                            // Checking if the move would lead above the board
                            if dest_index_i16 < 0 {
                                break;
                            }
                            let dest_index = dest_index_i16 as u16;
                            // Checking if the move would lead over the border
                            if abs_diff_u16(dest_index & 0b111, curr_file_index) > 1 {
                                continue;
                            }
                            if !w_pieces.get_bit(dest_index) {
                                all_moves.push(ChessMove::new_with_curr_and_dest(
                                    curr_index, dest_index,
                                ));
                            }
                        }
                        if bitboard.to_u64() & b_attackmask.to_u64() == 0 {
                            // dbg!(b_attackmask.to_u64());
                            // dbg!(bitboard.to_u64());
                            if self.get_castling_rights() & castling::WHITE_K != 0
                                && !all_pieces.get_bit(61u16)
                                && !all_pieces.get_bit(62u16)
                                && !self.castling_blocked(castling::WHITE_K)
                            {
                                all_moves.push(CastleMove::WHITE_K)
                            }
                            if self.get_castling_rights() & castling::WHITE_Q != 0
                                && !all_pieces.get_bit(59u16)
                                && !all_pieces.get_bit(58u16)
                                && !all_pieces.get_bit(57u16)
                                && !self.castling_blocked(castling::WHITE_Q)
                            {
                                all_moves.push(CastleMove::WHITE_Q)
                            }
                        } else {
                            // dbg!(b_attackmask.to_u64());
                            // dbg!(bitboard.to_u64());
                        }
                    }
                    Piece::WhiteQueen => {
                        // Bitwise AND of (n-1) is equal to mod(n)
                        // Checking if piece is on h file
                        if curr_index & 0b111 != 7 {
                            // moving to upper right
                            // Checking if piece is on 8th rank
                            if curr_index > 7 {
                                let mut dest_index = curr_index - 7;
                                while !w_pieces.get_bit(dest_index) {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        curr_index, dest_index,
                                    ));
                                    // Checking if a black piece is on the new position of the Bishop
                                    if b_pieces.get_bit(dest_index)
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
                            if curr_index < 56 {
                                let mut dest_index = curr_index + 9;
                                while !w_pieces.get_bit(dest_index) {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        curr_index, dest_index,
                                    ));
                                    // Checking if a black piece is on the new position of the Bishop
                                    if b_pieces.get_bit(dest_index)
                                        || dest_index & 0b111 == 7
                                        || dest_index > 55
                                    {
                                        break;
                                    }
                                    dest_index += 9;
                                }
                            }
                        }

                        if curr_index & 0b111 != 0 {
                            // moving to upper left
                            // Checking if piece is on 8th rank
                            if curr_index > 7 {
                                let mut dest_index = curr_index - 9;
                                while !w_pieces.get_bit(dest_index) {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        curr_index, dest_index,
                                    ));
                                    // Checking if a black piece is on the new position of the Bishop
                                    if b_pieces.get_bit(dest_index)
                                        || dest_index & 0b111 == 0
                                        || dest_index < 8
                                    {
                                        break;
                                    }
                                    dest_index -= 9;
                                }
                            }

                            // moving to bottom left
                            if curr_index < 56 {
                                let mut dest_index = curr_index + 7;
                                while !w_pieces.get_bit(dest_index) {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        curr_index, dest_index,
                                    ));
                                    // Checking if a black piece is on the new position of the Bishop
                                    if b_pieces.get_bit(dest_index)
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
                        for dest_index in (curr_index + 1)..=curr_index | 0b111 {
                            // Checking if a white piece is on the right of the Queen
                            if w_pieces.get_bit(dest_index) {
                                break;
                            }
                            all_moves
                                .push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                            // Checking if a black piece is on the new position of the Queen
                            if b_pieces.get_bit(dest_index) {
                                break;
                            }
                        }

                        // Moving left
                        for dest_index in (curr_index & 0xFFF8..curr_index).rev() {
                            // Checking if a white piece is on the left of the Queen
                            if w_pieces.get_bit(dest_index) {
                                break;
                            }
                            all_moves
                                .push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                            // Checking if a black piece is on the new position of the Queen
                            if b_pieces.get_bit(dest_index) {
                                break;
                            }
                        }

                        // Moving up
                        for i in (0..(curr_index >> 3)).rev() {
                            let dest_index = i * 8 + curr_file_index;
                            // Checking if a white piece is above the Queen
                            if w_pieces.get_bit(dest_index) {
                                break;
                            }
                            all_moves
                                .push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                            // Checking if a black piece is on the new position of the Queen
                            if b_pieces.get_bit(dest_index) {
                                break;
                            }
                        }

                        // Moving down
                        for i in (curr_index >> 3) + 1..8 {
                            let dest_index = i * 8 + curr_file_index;
                            // Checking if a white piece is below the Queen
                            if w_pieces.get_bit(dest_index) {
                                break;
                            }
                            all_moves
                                .push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                            // Checking if a black piece is on the new position of the Queen
                            if b_pieces.get_bit(dest_index) {
                                break;
                            }
                        }
                    }

                    // -- BLACK PIECES
                    Piece::BlackPawn => {
                        //normal move
                        //check if the square in front of the pawn is free
                        let dest_index = curr_index + 8;
                        if !all_pieces.get_bit(dest_index) && curr_rank_index != 7 {
                            all_moves
                                .push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                            //check if pawn is on first move
                            if curr_rank_index == 1 {
                                let dest_index = curr_index + 16;
                                if !all_pieces.get_bit(dest_index) {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        curr_index, dest_index,
                                    ));
                                }
                            }
                        }
                        //promotion
                        //checking if a piece is on the file of the pawn in first rank
                        else if !all_pieces.get_bit(dest_index) && curr_rank_index == 7 {
                            all_moves.push(ChessMove::new_with_curr_and_dest(
                                0b0001_000000 | curr_index,
                                dest_index,
                            ));
                            all_moves.push(ChessMove::new_with_curr_and_dest(
                                0b0010_000000 | curr_index,
                                dest_index,
                            ));
                            all_moves.push(ChessMove::new_with_curr_and_dest(
                                0b0100_000000 | curr_index,
                                dest_index,
                            ));
                            all_moves.push(ChessMove::new_with_curr_and_dest(
                                0b1000_000000 | curr_index,
                                dest_index,
                            ));
                        }

                        //capture left (pawn perspective)
                        if curr_file_index < 7 {
                            let dest_index = curr_index + 9;
                            if w_pieces.get_bit(dest_index) {
                                if curr_rank_index == 7 {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        0b0001_000000 | curr_index,
                                        dest_index,
                                    ));
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        0b0010_000000 | curr_index,
                                        dest_index,
                                    ));
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        0b0100_000000 | curr_index,
                                        dest_index,
                                    ));
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        0b1000_000000 | curr_index,
                                        dest_index,
                                    ));
                                } else {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        curr_index, dest_index,
                                    ));
                                }
                            }
                            if dest_index == self.get_en_passant() {
                                all_moves.push(ChessMove::new_with_curr_and_dest(
                                    curr_index, dest_index,
                                ));
                            }
                        }
                        //capture right (pawn perspective)
                        if curr_file_index > 0 {
                            let dest_index = curr_index + 7;
                            if w_pieces.get_bit(dest_index) {
                                if curr_rank_index == 7 {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        0b0001_000000 | curr_index,
                                        dest_index,
                                    ));
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        0b0010_000000 | curr_index,
                                        dest_index,
                                    ));
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        0b0100_000000 | curr_index,
                                        dest_index,
                                    ));
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        0b1000_000000 | curr_index,
                                        dest_index,
                                    ));
                                } else {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        curr_index, dest_index,
                                    ));
                                }
                            }
                            if dest_index == self.get_en_passant() {
                                all_moves.push(ChessMove::new_with_curr_and_dest(
                                    curr_index, dest_index,
                                ));
                            }
                        }
                    }
                    Piece::BlackBishop => {
                        // Bitwise AND of (n-1) is equal to mod(n)
                        // Checking if piece is on h file
                        if curr_index & 0b111 != 7 {
                            // moving to upper right
                            // Checking if piece is on 8th rank
                            if curr_index > 7 {
                                let mut dest_index = curr_index - 7;
                                while !b_pieces.get_bit(dest_index) {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        curr_index, dest_index,
                                    ));
                                    // Checking if a black piece is on the new position of the Bishop
                                    if w_pieces.get_bit(dest_index)
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
                            if curr_index < 56 {
                                let mut dest_index = curr_index + 9;
                                while !b_pieces.get_bit(dest_index) {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        curr_index, dest_index,
                                    ));
                                    // Checking if a black piece is on the new position of the Bishop
                                    if w_pieces.get_bit(dest_index)
                                        || dest_index & 0b111 == 7
                                        || dest_index > 55
                                    {
                                        break;
                                    }
                                    dest_index += 9;
                                }
                            }
                        }

                        if curr_index & 0b111 != 0 {
                            // moving to upper left
                            // Checking if piece is on 8th rank
                            if curr_index > 7 {
                                let mut dest_index = curr_index - 9;
                                while !b_pieces.get_bit(dest_index) {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        curr_index, dest_index,
                                    ));
                                    // Checking if a black piece is on the new position of the Bishop
                                    if w_pieces.get_bit(dest_index)
                                        || dest_index & 0b111 == 0
                                        || dest_index < 8
                                    {
                                        break;
                                    }
                                    dest_index -= 9;
                                }
                            }

                            // moving to bottom left
                            if curr_index < 56 {
                                let mut dest_index = curr_index + 7;
                                while !b_pieces.get_bit(dest_index) {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        curr_index, dest_index,
                                    ));
                                    // Checking if a black piece is on the new position of the Bishop
                                    if w_pieces.get_bit(dest_index)
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
                    Piece::BlackKnight => {
                        let dest_index_offsets_pos: [u16; 4] = [6, 10, 15, 17];
                        for offset in dest_index_offsets_pos {
                            let dest_index = curr_index + offset;
                            // Checking if the move would lead below the board
                            if dest_index > 63 {
                                break;
                            }
                            // Checking if the move would lead over the border
                            // a XOR b = |a - b|
                            if abs_diff_u16(dest_index & 0b111, curr_file_index) > 2 {
                                continue;
                            }
                            if !b_pieces.get_bit(dest_index) {
                                all_moves.push(ChessMove::new_with_curr_and_dest(
                                    curr_index, dest_index,
                                ));
                            }
                        }
                        let dest_index_offsets_neg: [i16; 4] = [-6, -10, -15, -17];
                        for offset in dest_index_offsets_neg {
                            let dest_index_i16 = curr_index as i16 + offset;
                            // Checking if the move would lead above the board
                            if dest_index_i16 < 0 {
                                break;
                            }
                            let dest_index = dest_index_i16 as u16;
                            // Checking if the move would lead over the border
                            if abs_diff_u16(dest_index & 0b111, curr_file_index) > 2 {
                                continue;
                            }
                            if !b_pieces.get_bit(dest_index) {
                                all_moves.push(ChessMove::new_with_curr_and_dest(
                                    curr_index, dest_index,
                                ));
                            }
                        }
                    }
                    Piece::BlackRook => {
                        // Moving right
                        // Upper boundary of for loop is the index of h file of the rank of the rook
                        for dest_index in (curr_index + 1)..=curr_index | 0b111 {
                            // Checking if a white piece is on the right of the Rook
                            if b_pieces.get_bit(dest_index) {
                                break;
                            }
                            all_moves
                                .push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                            // Checking if a black piece is on the new position of the Rook
                            if w_pieces.get_bit(dest_index) {
                                break;
                            }
                        }

                        // Moving left
                        for dest_index in (curr_index & 0xFFF8..curr_index).rev() {
                            // Checking if a white piece is on the left of the Rook
                            if b_pieces.get_bit(dest_index) {
                                break;
                            }
                            all_moves
                                .push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                            // Checking if a black piece is on the new position of the Rook
                            if w_pieces.get_bit(dest_index) {
                                break;
                            }
                        }

                        // Moving up
                        for i in (0..(curr_index >> 3)).rev() {
                            let dest_index = i * 8 + curr_file_index;
                            // Checking if a white piece is above the Rook
                            if b_pieces.get_bit(dest_index) {
                                break;
                            }
                            all_moves
                                .push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                            // Checking if a black piece is on the new position of the Rook
                            if w_pieces.get_bit(dest_index) {
                                break;
                            }
                        }

                        // Moving down
                        for i in (curr_index >> 3) + 1..8 {
                            let dest_index = i * 8 + curr_file_index;
                            // Checking if a white piece is below the Rook
                            if b_pieces.get_bit(dest_index) {
                                break;
                            }
                            all_moves
                                .push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                            // Checking if a black piece is on the new position of the Rook
                            if w_pieces.get_bit(dest_index) {
                                break;
                            }
                        }
                    }
                    Piece::BlackKing => {
                        let dest_index_offsets_pos: [u16; 4] = [1, 7, 8, 9];
                        for offset in dest_index_offsets_pos {
                            let dest_index = curr_index + offset;
                            // Checking if the move would lead below the board
                            if dest_index > 63 {
                                break;
                            }
                            // Checking if the move would lead over the border
                            if abs_diff_u16(dest_index & 0b111, curr_file_index) > 1 {
                                continue;
                            }
                            if !b_pieces.get_bit(dest_index) {
                                all_moves.push(ChessMove::new_with_curr_and_dest(
                                    curr_index, dest_index,
                                ));
                            }
                        }
                        let dest_index_offsets_neg: [i16; 4] = [-1, -7, -8, -9];
                        for offset in dest_index_offsets_neg {
                            let dest_index_i16 = curr_index as i16 + offset;
                            // Checking if the move would lead above the board
                            if dest_index_i16 < 0 {
                                break;
                            }
                            let dest_index = dest_index_i16 as u16;
                            // Checking if the move would lead over the border
                            if abs_diff_u16(dest_index & 0b111, curr_file_index) > 1 {
                                continue;
                            }
                            if !b_pieces.get_bit(dest_index) {
                                all_moves.push(ChessMove::new_with_curr_and_dest(
                                    curr_index, dest_index,
                                ));
                            }
                        }
                        if bitboard.to_u64() & w_attackmask.to_u64() == 0 {
                            if self.get_castling_rights() & castling::BLACK_K != 0
                                && !all_pieces.get_bit(5u16)
                                && !all_pieces.get_bit(6u16)
                                && !self.castling_blocked(castling::BLACK_K)
                            {
                                all_moves.push(CastleMove::BLACK_K)
                            }
                            if self.get_castling_rights() & castling::BLACK_Q != 0
                                && !all_pieces.get_bit(3u16)
                                && !all_pieces.get_bit(2u16)
                                && !all_pieces.get_bit(1u16)
                                && !self.castling_blocked(castling::BLACK_Q)
                            {
                                all_moves.push(CastleMove::BLACK_Q)
                            }
                        }
                    }
                    Piece::BlackQueen => {
                        // Bitwise AND of (n-1) is equal to mod(n)
                        // Checking if piece is on h file
                        if curr_index & 0b111 != 7 {
                            // moving to upper right
                            // Checking if piece is on 8th rank
                            if curr_index > 7 {
                                let mut dest_index = curr_index - 7;
                                while !b_pieces.get_bit(dest_index) {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        curr_index, dest_index,
                                    ));
                                    // Checking if a black piece is on the new position of the Bishop
                                    if w_pieces.get_bit(dest_index)
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
                            if curr_index < 56 {
                                let mut dest_index = curr_index + 9;
                                while !b_pieces.get_bit(dest_index) {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        curr_index, dest_index,
                                    ));
                                    // Checking if a black piece is on the new position of the Bishop
                                    if w_pieces.get_bit(dest_index)
                                        || dest_index & 0b111 == 7
                                        || dest_index > 55
                                    {
                                        break;
                                    }
                                    dest_index += 9;
                                }
                            }
                        }

                        if curr_index & 0b111 != 0 {
                            // moving to upper left
                            // Checking if piece is on 8th rank
                            if curr_index > 7 {
                                let mut dest_index = curr_index - 9;
                                while !b_pieces.get_bit(dest_index) {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        curr_index, dest_index,
                                    ));
                                    // Checking if a black piece is on the new position of the Bishop
                                    if w_pieces.get_bit(dest_index)
                                        || dest_index & 0b111 == 0
                                        || dest_index < 8
                                    {
                                        break;
                                    }
                                    dest_index -= 9;
                                }
                            }

                            // moving to bottom left
                            if curr_index < 56 {
                                let mut dest_index = curr_index + 7;
                                while !b_pieces.get_bit(dest_index) {
                                    all_moves.push(ChessMove::new_with_curr_and_dest(
                                        curr_index, dest_index,
                                    ));
                                    // Checking if a black piece is on the new position of the Bishop
                                    if w_pieces.get_bit(dest_index)
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
                        for dest_index in (curr_index + 1)..=curr_index | 0b111 {
                            // Checking if a white piece is on the right of the Queen
                            if b_pieces.get_bit(dest_index) {
                                break;
                            }
                            all_moves
                                .push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                            // Checking if a black piece is on the new position of the Queen
                            if w_pieces.get_bit(dest_index) {
                                break;
                            }
                        }

                        // Moving left
                        for dest_index in (curr_index & 0xFFF8..curr_index).rev() {
                            // Checking if a white piece is on the left of the Queen
                            if b_pieces.get_bit(dest_index) {
                                break;
                            }
                            all_moves
                                .push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                            // Checking if a black piece is on the new position of the Queen
                            if w_pieces.get_bit(dest_index) {
                                break;
                            }
                        }

                        // Moving up
                        for i in (0..(curr_index >> 3)).rev() {
                            let dest_index = i * 8 + curr_file_index;
                            // Checking if a white piece is above the Queen
                            if b_pieces.get_bit(dest_index) {
                                break;
                            }
                            all_moves
                                .push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                            // Checking if a black piece is on the new position of the Queen
                            if w_pieces.get_bit(dest_index) {
                                break;
                            }
                        }

                        // Moving down
                        for i in (curr_index >> 3) + 1..8 {
                            let dest_index = i * 8 + curr_file_index;
                            // Checking if a white piece is below the Queen
                            if b_pieces.get_bit(dest_index) {
                                break;
                            }
                            all_moves
                                .push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                            // Checking if a black piece is on the new position of the Queen
                            if w_pieces.get_bit(dest_index) {
                                break;
                            }
                        }
                    }
                }
            }
        }

        // removing moves, that would open the king to a check (illegal)
        let mut res: Vec<ChessMove> = Vec::new();
        for mv in all_moves {
            let new_board = self.with_move(mv);
            // if it's whites turn and the Black King is in Check, the move is illegal
            if new_board.get_white_to_move()
                && new_board.get_bitboard(Piece::BlackKing as usize).unwrap()
                    & new_board.calc_w_attackmask()
                    != Bitboard::new(0)
                || !new_board.kings_exist()
            {
                continue;
            }
            // if it's blacks turn and the White King is in Check, the move is illegal
            if !new_board.get_white_to_move()
                && new_board.get_bitboard(Piece::WhiteKing as usize).unwrap()
                    & new_board.calc_b_attackmask()
                    != Bitboard::new(0)
                || !new_board.kings_exist()
            {
                continue;
            }
            res.push(mv);
        }
        return res;
    }

    pub fn kings_exist(&self) -> bool {
        if self
            .get_bitboard(Piece::WhiteKing as usize)
            .unwrap()
            .to_u64()
            != 0
            && self
                .get_bitboard(Piece::BlackKing as usize)
                .unwrap()
                .to_u64()
                != 0
        {
            return true;
        } else {
            return false;
        }
    }

    // checking of the square towards the castling is blocked, e.g. for White Kingside it checks if f1 is under attack by black
    // checking if king is checked (castling is not allowed then)
    pub fn castling_blocked(&self, side: u8) -> bool {
        let all_boards = self.get_bitboards();
        let b_pieces_u64 = all_boards[6..12]
            .iter()
            .fold(0u64, |acc, b| acc | b.to_u64());
        let w_pieces_u64 = all_boards[0..6]
            .iter()
            .fold(0u64, |acc, b| acc | b.to_u64());
        let b_pieces = Bitboard::new(b_pieces_u64);
        let w_pieces = Bitboard::new(w_pieces_u64);
        let b_diagonal_pieces =
            all_boards[Piece::BlackBishop as usize] | all_boards[Piece::BlackQueen as usize];
        let b_vertical_pieces =
            all_boards[Piece::BlackRook as usize] | all_boards[Piece::BlackQueen as usize];
        let w_diagonal_pieces =
            all_boards[Piece::WhiteBishop as usize] | all_boards[Piece::WhiteQueen as usize];
        let w_vertical_pieces =
            all_boards[Piece::WhiteRook as usize] | all_boards[Piece::WhiteQueen as usize];
        match side {
            castling::WHITE_K => {
                // handling all pieces that capture diagonally
                for sq in [52, 43, 34, 25, 16, 54, 47] {
                    if w_pieces.get_bit(sq) {
                        break;
                    } else if b_diagonal_pieces.get_bit(sq) {
                        return true;
                    }
                }
                // handling all pieces that capture vertically
                for sq in [53, 45, 37, 29, 21, 13, 5] {
                    if w_pieces.get_bit(sq) {
                        break;
                    } else if b_vertical_pieces.get_bit(sq) {
                        return true;
                    }
                }
                // handling knights
                for sq in [44, 46] {
                    if all_boards[Piece::BlackKnight as usize].get_bit(sq) {
                        return true;
                    }
                }
            }
            castling::WHITE_Q => {
                // handling all pieces that capture diagonally
                for sq in [50, 41, 32, 52, 45, 38, 31] {
                    if w_pieces.get_bit(sq) {
                        break;
                    } else if b_diagonal_pieces.get_bit(sq) {
                        return true;
                    }
                }
                // handling all pieces that capture vertically
                for sq in [51, 43, 35, 27, 19, 11, 3] {
                    if w_pieces.get_bit(sq) {
                        break;
                    } else if b_vertical_pieces.get_bit(sq) {
                        return true;
                    }
                }
                // handling knights
                for sq in [42, 44] {
                    if all_boards[Piece::BlackKnight as usize].get_bit(sq) {
                        return true;
                    }
                }
            }
            castling::BLACK_K => {
                // handling all pieces that capture diagonally
                for sq in [12, 19, 27, 33, 40, 14, 23] {
                    if b_pieces.get_bit(sq) {
                        break;
                    } else if w_diagonal_pieces.get_bit(sq) {
                        return true;
                    }
                }
                // handling all pieces that capture vertically
                for sq in [13, 21, 29, 37, 45, 53, 61] {
                    if b_pieces.get_bit(sq) {
                        break;
                    } else if w_vertical_pieces.get_bit(sq) {
                        return true;
                    }
                }
                // handling knights
                for sq in [20, 22] {
                    if all_boards[Piece::WhiteKnight as usize].get_bit(sq) {
                        return true;
                    }
                }
            }
            castling::BLACK_Q => {
                // handling all pieces that capture diagonally
                for sq in [10, 17, 24, 12, 21, 30, 39] {
                    if b_pieces.get_bit(sq) {
                        break;
                    } else if w_diagonal_pieces.get_bit(sq) {
                        return true;
                    }
                }
                // handling all pieces that capture vertically
                for sq in [11, 19, 27, 35, 43, 51, 59] {
                    if b_pieces.get_bit(sq) {
                        break;
                    } else if w_vertical_pieces.get_bit(sq) {
                        return true;
                    }
                }
                // handling knights
                for sq in [18, 20] {
                    if all_boards[Piece::WhiteKnight as usize].get_bit(sq) {
                        return true;
                    }
                }
            }
            castling::WHITE_BOTH => {
                // handling all pieces that capture diagonally
                for sq in [52, 43, 34, 25, 16, 54, 47] {
                    if w_pieces.get_bit(sq) {
                        break;
                    } else if b_diagonal_pieces.get_bit(sq) {
                        return true;
                    }
                }
                // handling all pieces that capture vertically
                for sq in [53, 45, 37, 29, 21, 13, 5] {
                    if w_pieces.get_bit(sq) {
                        break;
                    } else if b_vertical_pieces.get_bit(sq) {
                        return true;
                    }
                }
                // handling knights
                for sq in [44, 46] {
                    if all_boards[Piece::BlackKnight as usize].get_bit(sq) {
                        return true;
                    }
                }
            }
            _ => {}
        }
        false
    }
}
