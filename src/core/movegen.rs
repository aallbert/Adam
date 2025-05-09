use crate::interface::abs_diff_u16;
use crate::models::board::Bitboard;
use crate::models::chessmove::ChessMove;
use crate::models::piece::Piece;

pub fn possible_moves(
    pc: char,
    init_mv: ChessMove,
    b_pieces: Bitboard,
    w_pieces: Bitboard,
) -> Vec<ChessMove> {
    let all_pieces: Bitboard = w_pieces | b_pieces;
    let mut res: Vec<ChessMove> = Vec::new();
    let curr_square = init_mv.get_curr_square_as_struct();
    let curr_rank_index = curr_square.get_rank_as_index();
    let curr_file_index = curr_square.get_file_as_index();
    let curr_index = curr_square.to_u16();

    match pc {
        //WHITE PIECES
        'P' => {
            //normal move
            //check if the square in front of the pawn is free
            let dest_index = curr_index - 8;
            if !all_pieces.get_bit_on_bitboard(dest_index) {
                res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                //check if pawn is on first move
                if curr_rank_index == 6 {
                    let dest_index = curr_index - 16;
                    if !all_pieces.get_bit_on_bitboard(dest_index) {
                        res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                    }
                }
            }

            //capture right
            if curr_file_index < 7 {
                let dest_index = curr_index - 7;
                if b_pieces.get_bit_on_bitboard(dest_index) {
                    res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                }
            }
            //capture left
            if curr_file_index > 0 {
                let dest_index = curr_index - 9;
                if b_pieces.get_bit_on_bitboard(dest_index) {
                    res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                }
            }

            // en passent (todo)
        }
        'B' => {
            // Bitwise AND of (n-1) is equal to mod(n)
            // Checking if piece is on h file
            if curr_index & 0b111 != 7 {
                // moving to upper right
                // Checking if piece is on 8th rank
                if curr_index > 7 {
                    let mut dest_index = curr_index - 7;
                    while !w_pieces.get_bit_on_bitboard(dest_index) {
                        res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                        // Checking if a black piece is on the new position of the Bishop
                        if b_pieces.get_bit_on_bitboard(dest_index)
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
                    while !w_pieces.get_bit_on_bitboard(dest_index) {
                        res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                        // Checking if a black piece is on the new position of the Bishop
                        if b_pieces.get_bit_on_bitboard(dest_index)
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
                    while !w_pieces.get_bit_on_bitboard(dest_index) {
                        res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                        // Checking if a black piece is on the new position of the Bishop
                        if b_pieces.get_bit_on_bitboard(dest_index)
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
                    while !w_pieces.get_bit_on_bitboard(dest_index) {
                        res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                        // Checking if a black piece is on the new position of the Bishop
                        if b_pieces.get_bit_on_bitboard(dest_index)
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
        'N' => {
            let dest_index_offsets_pos: [u16; 4] = [6, 10, 15, 17];
            for offset in dest_index_offsets_pos {
                let dest_index = curr_index + offset;
                // Checking if the move would lead below the board
                if dest_index > 63 {
                    break;
                }
                // Checking if the move would lead over the border
                // a XOR b = |a - b|
                println!("dest_index {}", dest_index);
                if abs_diff_u16(dest_index & 0b111, curr_file_index) > 2 {
                    println!("abs diff greater than 2");
                    continue;
                }
                if !w_pieces.get_bit_on_bitboard(dest_index) {
                    res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
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
                if !w_pieces.get_bit_on_bitboard(dest_index) {
                    res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                }
            }
        }
        'R' => {
            // Moving right
            // Upper boundary of for loop is the index of h file of the rank of the rook
            for dest_index in (curr_index + 1)..=curr_index | 0b111 {
                // Checking if a white piece is on the right of the Rook
                if w_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                // Checking if a black piece is on the new position of the Rook
                if b_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
            }

            // Moving left
            for dest_index in (curr_index & 0xFFF8..=(curr_index - 1)).rev() {
                // Checking if a white piece is on the left of the Rook
                if w_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                // Checking if a black piece is on the new position of the Rook
                if b_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
            }

            // Moving up
            for i in (0..(curr_index >> 3)).rev() {
                let dest_index = i * 8 + curr_file_index;
                // Checking if a white piece is above the Rook
                if w_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                // Checking if a black piece is on the new position of the Rook
                if b_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
            }

            // Moving down
            for i in (curr_index >> 3) + 1..8 {
                let dest_index = i * 8 + curr_file_index;
                // Checking if a white piece is below the Rook
                if w_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                // Checking if a black piece is on the new position of the Rook
                if b_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
            }
        }
        'K' => {
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
                if !w_pieces.get_bit_on_bitboard(dest_index) {
                    res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
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
                if !w_pieces.get_bit_on_bitboard(dest_index) {
                    res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                }
            }
        }
        'Q' => {
            // Bitwise AND of (n-1) is equal to mod(n)
            // Checking if piece is on h file
            if curr_index & 0b111 != 7 {
                // moving to upper right
                // Checking if piece is on 8th rank
                if curr_index > 7 {
                    let mut dest_index = curr_index - 7;
                    while !w_pieces.get_bit_on_bitboard(dest_index) {
                        res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                        // Checking if a black piece is on the new position of the Bishop
                        if b_pieces.get_bit_on_bitboard(dest_index)
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
                    while !w_pieces.get_bit_on_bitboard(dest_index) {
                        res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                        // Checking if a black piece is on the new position of the Bishop
                        if b_pieces.get_bit_on_bitboard(dest_index)
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
                    while !w_pieces.get_bit_on_bitboard(dest_index) {
                        res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                        // Checking if a black piece is on the new position of the Bishop
                        if b_pieces.get_bit_on_bitboard(dest_index)
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
                    while !w_pieces.get_bit_on_bitboard(dest_index) {
                        res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                        // Checking if a black piece is on the new position of the Bishop
                        if b_pieces.get_bit_on_bitboard(dest_index)
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
                if w_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                // Checking if a black piece is on the new position of the Queen
                if b_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
            }

            // Moving left
            for dest_index in (curr_index & 0xFFF8..=(curr_index - 1)).rev() {
                // Checking if a white piece is on the left of the Queen
                if w_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                // Checking if a black piece is on the new position of the Queen
                if b_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
            }

            // Moving up
            for i in (0..(curr_index >> 3)).rev() {
                let dest_index = i * 8 + curr_file_index;
                // Checking if a white piece is above the Queen
                if w_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                // Checking if a black piece is on the new position of the Queen
                if b_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
            }

            // Moving down
            for i in (curr_index >> 3) + 1..8 {
                let dest_index = i * 8 + curr_file_index;
                // Checking if a white piece is below the Queen
                if w_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                // Checking if a black piece is on the new position of the Queen
                if b_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
            }
        }

        // -- BLACK PIECES
        'p' => {
            //normal move
            //check if the square in front of the pawn is free
            let dest_index = curr_index + 8;
            if !all_pieces.get_bit_on_bitboard(dest_index) {
                res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                //check if pawn is on first move
                if curr_rank_index == 1 {
                    let dest_index = curr_index + 16;
                    if !all_pieces.get_bit_on_bitboard(dest_index) {
                        res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                    }
                }
            }

            //capture left (pawn perspective)
            if curr_file_index < 7 {
                let dest_index = curr_index + 9;
                if w_pieces.get_bit_on_bitboard(dest_index) {
                    res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                }
            }
            //capture right (pawn perspective)
            if curr_file_index > 1 {
                let dest_index = curr_index + 7;
                if w_pieces.get_bit_on_bitboard(dest_index) {
                    res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                }
            }

            // en passent (todo)
        }
        'b' => {
            // moving to upper right
            let mut dest_index = curr_index - 7;
            // Bitwise AND of (n-1) is equal to mod(n)
            while dest_index > 7 && dest_index & 0b111 != 7 {
                if b_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                // Checking if a black piece is on the new position of the Bishop
                if w_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                dest_index -= 7;
            }

            // moving to upper left
            dest_index = curr_index - 9;
            while dest_index > 7 && dest_index & 0b111 != 0 {
                if b_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                // Checking if a black piece is on the new position of the Bishop
                if w_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                dest_index -= 9;
            }

            // moving to bottom right
            dest_index = curr_index + 9;
            while dest_index < 56 && dest_index & 0b111 != 7 {
                if b_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                // Checking if a black piece is on the new position of the Bishop
                if w_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                dest_index += 9;
            }

            // moving to bottom left
            dest_index = curr_index + 7;
            while dest_index < 56 && dest_index & 0b111 != 0 {
                if b_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                // Checking if a black piece is on the new position of the Bishop
                if w_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                dest_index += 7;
            }
        }
        'n' => {
            let dest_index_offsets_pos: [u16; 4] = [6, 10, 15, 17];
            for offset in dest_index_offsets_pos {
                let dest_index = curr_index + offset;
                // Checking if the move would lead below the board
                if dest_index > 63 {
                    break;
                }
                // Checking if the move would lead over the border
                // a XOR b = |a - b|
                println!("dest_index {}", dest_index);
                if abs_diff_u16(dest_index & 0b111, curr_file_index) > 2 {
                    println!("abs diff greater than 2");
                    continue;
                }
                if !b_pieces.get_bit_on_bitboard(dest_index) {
                    res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
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
                if !b_pieces.get_bit_on_bitboard(dest_index) {
                    res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                }
            }
        }
        'r' => {
            // Moving right
            // Upper boundary of for loop is the index of h file of the rank of the rook
            for dest_index in (curr_index + 1)..=curr_index | 0b111 {
                // Checking if a white piece is on the right of the Rook
                if b_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                // Checking if a black piece is on the new position of the Rook
                if w_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
            }

            // Moving left
            for dest_index in (curr_index & 0xFFF8..=(curr_index - 1)).rev() {
                // Checking if a white piece is on the left of the Rook
                if b_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                // Checking if a black piece is on the new position of the Rook
                if w_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
            }

            // Moving up
            for i in (0..(curr_index >> 3)).rev() {
                let dest_index = i * 8 + curr_file_index;
                // Checking if a white piece is above the Rook
                if b_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                // Checking if a black piece is on the new position of the Rook
                if w_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
            }

            // Moving down
            for i in (curr_index >> 3) + 1..8 {
                let dest_index = i * 8 + curr_file_index;
                // Checking if a white piece is below the Rook
                if b_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                // Checking if a black piece is on the new position of the Rook
                if w_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
            }
        }
        'k' => {
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
                if !b_pieces.get_bit_on_bitboard(dest_index) {
                    res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
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
                if !b_pieces.get_bit_on_bitboard(dest_index) {
                    res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                }
            }
        }
        'q' => {
            // Bitwise AND of (n-1) is equal to mod(n)
            // Checking if piece is on h file
            if curr_index & 0b111 != 7 {
                // moving to upper right
                // Checking if piece is on 8th rank
                if curr_index > 7 {
                    let mut dest_index = curr_index - 7;
                    while !b_pieces.get_bit_on_bitboard(dest_index) {
                        res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                        // Checking if a black piece is on the new position of the Bishop
                        if w_pieces.get_bit_on_bitboard(dest_index)
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
                    while !b_pieces.get_bit_on_bitboard(dest_index) {
                        res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                        // Checking if a black piece is on the new position of the Bishop
                        if w_pieces.get_bit_on_bitboard(dest_index)
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
                    while !b_pieces.get_bit_on_bitboard(dest_index) {
                        res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                        // Checking if a black piece is on the new position of the Bishop
                        if w_pieces.get_bit_on_bitboard(dest_index)
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
                    while !b_pieces.get_bit_on_bitboard(dest_index) {
                        res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                        // Checking if a black piece is on the new position of the Bishop
                        if w_pieces.get_bit_on_bitboard(dest_index)
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
                if b_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                // Checking if a black piece is on the new position of the Queen
                if w_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
            }

            // Moving left
            for dest_index in (curr_index & 0xFFF8..=(curr_index - 1)).rev() {
                // Checking if a white piece is on the left of the Queen
                if b_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                // Checking if a black piece is on the new position of the Queen
                if w_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
            }

            // Moving up
            for i in (0..(curr_index >> 3)).rev() {
                let dest_index = i * 8 + curr_file_index;
                // Checking if a white piece is above the Queen
                if b_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                // Checking if a black piece is on the new position of the Queen
                if w_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
            }

            // Moving down
            for i in (curr_index >> 3) + 1..8 {
                let dest_index = i * 8 + curr_file_index;
                // Checking if a white piece is below the Queen
                if b_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
                res.push(ChessMove::new_with_curr_and_dest(curr_index, dest_index));
                // Checking if a black piece is on the new position of the Queen
                if w_pieces.get_bit_on_bitboard(dest_index) {
                    break;
                }
            }
        }
        _ => {
            panic!("Unexpected Symbol passed!")
        }
    }

    println!("Count of moves: {}", res.len());

    println!("Possible Squares to reach:");
    for mv in &res {
        let sq = mv.get_dest_square_as_struct_char();
        println!("{}{}", sq.get_file(), sq.get_rank());
    }
    return res;
}