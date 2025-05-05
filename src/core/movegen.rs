use crate::interface::{
    Bitboard, Move, Square, file_to_i8, get_bit_on_bitboard, i8_to_file, i8_to_rank, rank_to_i8,
};

pub fn possible_moves(pc: char, pos: Square, b_pieces: Bitboard, w_pieces: Bitboard) -> Vec<Move> {
    let all_pieces: Bitboard = w_pieces | b_pieces;
    let mut res: Vec<Move> = Vec::new();
    let curr_file = pos.file;
    let curr_rank = pos.rank;
    let int_curr_file = file_to_i8(curr_file);
    let int_curr_rank = rank_to_i8(curr_rank);

    match pc {
        //WHITE PIECES
        'P' => {
            //normal move
            if !get_bit_on_bitboard(all_pieces, int_curr_file, int_curr_rank + 1){
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: curr_file,
                        rank: i8_to_rank(int_curr_rank + 1),
                    },
                });

            }
            if curr_rank == '2' && !get_bit_on_bitboard(all_pieces, int_curr_file, int_curr_rank + 2) {
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: curr_file,
                        rank: '4',
                    },
                });
            }

            //capture
            if int_curr_file <= 7 && get_bit_on_bitboard(b_pieces, int_curr_file + 1, int_curr_rank + 1){
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: i8_to_file(int_curr_file + 1),
                        rank: i8_to_rank(int_curr_rank + 1),
                    },
                });

            }
            if int_curr_file >= 2 && get_bit_on_bitboard(b_pieces, int_curr_file - 1, int_curr_rank + 1){
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: i8_to_file(int_curr_file - 1),
                        rank: i8_to_rank(int_curr_rank + 1),
                    },
                });

            }

            // en passent (todo)
        }
        'B' => {
            let mut to_rank = int_curr_rank + 1;
            let mut to_file = int_curr_file + 1;
            while to_rank <= 8 && to_file <= 8 {
                if get_bit_on_bitboard(w_pieces, to_file, to_rank) {
                    break;
                }
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: i8_to_file(to_file),
                        rank: i8_to_rank(to_rank),
                    },
                });
                //checking if a black piece is on the new position of the Bishop
                if get_bit_on_bitboard(b_pieces, to_file, to_rank) {
                    break;
                }
                to_rank += 1;
                to_file += 1;
            }
            to_rank = int_curr_rank - 1;
            to_file = int_curr_file + 1;
            while to_rank >= 1 && to_file <= 8 {
                if get_bit_on_bitboard(w_pieces, to_file, to_rank) {
                    break;
                }
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: i8_to_file(to_file),
                        rank: i8_to_rank(to_rank),
                    },
                });
                //checking if a black piece is on the new position of the Bishop
                if get_bit_on_bitboard(b_pieces, to_file, to_rank) {
                    break;
                }
                to_rank -= 1;
                to_file += 1;
            }
            to_rank = int_curr_rank - 1;
            to_file = int_curr_file - 1;
            while to_rank >= 1 && to_file >= 1 {
                if get_bit_on_bitboard(w_pieces, to_file, to_rank) {
                    break;
                }
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: i8_to_file(to_file),
                        rank: i8_to_rank(to_rank),
                    },
                });
                //checking if a black piece is on the new position of the Bishop
                if get_bit_on_bitboard(b_pieces, to_file, to_rank) {
                    break;
                }
                to_rank -= 1;
                to_file -= 1;
            }
            to_rank = int_curr_rank + 1;
            to_file = int_curr_file - 1;
            while to_rank <= 8 && to_file >= 1 {
                if get_bit_on_bitboard(w_pieces, to_file, to_rank) {
                    break;
                }
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: i8_to_file(to_file),
                        rank: i8_to_rank(to_rank),
                    },
                });
                //checking if a black piece is on the new position of the Bishop
                if get_bit_on_bitboard(b_pieces, to_file, to_rank) {
                    break;
                }
                to_rank += 1;
                to_file -= 1;
            }
        }
        'N' => {
            let mut offset_rank_a: i8 = 2;
            let mut offset_file_a: i8 = 1;
            let mut offset_rank_b: i8 = 1;
            let mut offset_file_b: i8 = 2;
            for _ in 0..4 {
                let to_rank_a = int_curr_rank + offset_rank_a;
                let to_file_a = int_curr_file + offset_file_a;
                let to_rank_b = int_curr_rank + offset_rank_b;
                let to_file_b = int_curr_file + offset_file_b;
                if to_file_a <= 8
                    && to_file_a >= 1
                    && to_rank_a <= 8
                    && to_rank_a >= 1
                    && !get_bit_on_bitboard(w_pieces, to_file_a, to_rank_a)
                {
                    res.push(Move {
                        from: pos,
                        to: Square {
                            file: i8_to_file(to_file_a),
                            rank: i8_to_rank(to_rank_a),
                        },
                    })
                }
                if to_file_b <= 8
                    && to_file_b >= 1
                    && to_rank_b <= 8
                    && to_rank_b >= 1
                    && !get_bit_on_bitboard(w_pieces, to_file_b, to_rank_b)
                {
                    res.push(Move {
                        from: pos,
                        to: Square {
                            file: i8_to_file(to_file_b),
                            rank: i8_to_rank(to_rank_b),
                        },
                    })
                }
                let temp_a = offset_rank_a;
                offset_rank_a = -offset_file_a;
                offset_file_a = temp_a;
                let temp_b = offset_rank_b;
                offset_rank_b = -offset_file_b;
                offset_file_b = temp_b;
            }
        }
        'R' => {
            for i in (int_curr_file + 1)..=8 {
                //checking if a white piece is on the right of the Rook
                if get_bit_on_bitboard(w_pieces, i, int_curr_rank) {
                    break;
                }
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: i8_to_file(i),
                        rank: curr_rank,
                    },
                });
                //checking if a black piece is on the new position of the Rook
                if get_bit_on_bitboard(b_pieces, i, int_curr_rank) {
                    break;
                }
            }
            for i in (1..int_curr_file).rev() {
                //checking if a white piece is on the left of the Rook
                if get_bit_on_bitboard(w_pieces, i, int_curr_rank) {
                    break;
                }
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: i8_to_file(i),
                        rank: curr_rank,
                    },
                });
                //checking if a black piece is on the new position of the Rook
                if get_bit_on_bitboard(b_pieces, i, int_curr_rank) {
                    break;
                }
            }
            for i in (int_curr_rank + 1)..=8 {
                //checking if a white piece is above the Rook
                if get_bit_on_bitboard(w_pieces, int_curr_file, i) {
                    break;
                }
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: curr_file,
                        rank: i8_to_rank(i),
                    },
                });
                //checking if a black piece is on the new position of the Rook
                if get_bit_on_bitboard(b_pieces, int_curr_file, i) {
                    break;
                }
            }
            for i in (1..int_curr_rank).rev() {
                //checking if a white piece is below the Rook
                if get_bit_on_bitboard(w_pieces, int_curr_file, i) {
                    break;
                }
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: curr_file,
                        rank: i8_to_rank(i),
                    },
                });
                //checking if a black piece is on the new position of the Rook
                if get_bit_on_bitboard(b_pieces, int_curr_file, i) {
                    break;
                }
            }
        }
        'K' => {
            let mut offset_rank_a: i8 = 1;
            let mut offset_file_a: i8 = 0;
            let mut offset_rank_b: i8 = 1;
            let mut offset_file_b: i8 = 1;
            for _ in 0..4 {
                let to_rank_a = int_curr_rank + offset_rank_a;
                let to_file_a = int_curr_file + offset_file_a;
                let to_rank_b = int_curr_rank + offset_rank_b;
                let to_file_b = int_curr_file + offset_file_b;
                if to_file_a <= 8
                    && to_file_a >= 1
                    && to_rank_a <= 8
                    && to_rank_a >= 1
                    && !get_bit_on_bitboard(w_pieces, to_file_a, to_rank_a)
                {
                    res.push(Move {
                        from: pos,
                        to: Square {
                            file: i8_to_file(to_file_a),
                            rank: i8_to_rank(to_rank_a),
                        },
                    })
                }
                if to_file_b <= 8
                    && to_file_b >= 1
                    && to_rank_b <= 8
                    && to_rank_b >= 1
                    && !get_bit_on_bitboard(w_pieces, to_file_b, to_rank_b)
                {
                    res.push(Move {
                        from: pos,
                        to: Square {
                            file: i8_to_file(to_file_b),
                            rank: i8_to_rank(to_rank_b),
                        },
                    })
                }
                let temp_a = offset_rank_a;
                offset_rank_a = -offset_file_a;
                offset_file_a = temp_a;
                let temp_b = offset_rank_b;
                offset_rank_b = -offset_file_b;
                offset_file_b = temp_b;
            }
        }
        'Q' => {
            let mut to_rank = int_curr_rank + 1;
            let mut to_file = int_curr_file + 1;
            while to_rank <= 8 && to_file <= 8 {
                if get_bit_on_bitboard(w_pieces, to_file, to_rank) {
                    break;
                }
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: i8_to_file(to_file),
                        rank: i8_to_rank(to_rank),
                    },
                });
                //checking if a black piece is on the new position of the Queen
                if get_bit_on_bitboard(b_pieces, to_file, to_rank) {
                    break;
                }
                to_rank += 1;
                to_file += 1;
            }
            to_rank = int_curr_rank - 1;
            to_file = int_curr_file + 1;
            while to_rank >= 1 && to_file <= 8 {
                if get_bit_on_bitboard(w_pieces, to_file, to_rank) {
                    break;
                }
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: i8_to_file(to_file),
                        rank: i8_to_rank(to_rank),
                    },
                });
                //checking if a black piece is on the new position of the Queen
                if get_bit_on_bitboard(b_pieces, to_file, to_rank) {
                    break;
                }
                to_rank -= 1;
                to_file += 1;
            }
            to_rank = int_curr_rank - 1;
            to_file = int_curr_file - 1;
            while to_rank >= 1 && to_file >= 1 {
                if get_bit_on_bitboard(w_pieces, to_file, to_rank) {
                    break;
                }
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: i8_to_file(to_file),
                        rank: i8_to_rank(to_rank),
                    },
                });
                //checking if a black piece is on the new position of the Queen
                if get_bit_on_bitboard(b_pieces, to_file, to_rank) {
                    break;
                }
                to_rank -= 1;
                to_file -= 1;
            }
            to_rank = int_curr_rank + 1;
            to_file = int_curr_file - 1;
            while to_rank <= 8 && to_file >= 1 {
                if get_bit_on_bitboard(w_pieces, to_file, to_rank) {
                    break;
                }
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: i8_to_file(to_file),
                        rank: i8_to_rank(to_rank),
                    },
                });
                //checking if a black piece is on the new position of the Queen
                if get_bit_on_bitboard(b_pieces, to_file, to_rank) {
                    break;
                }
                to_rank += 1;
                to_file -= 1;
            }
            for i in (int_curr_file + 1)..=8 {
                if get_bit_on_bitboard(w_pieces, i, int_curr_rank) {
                    break;
                }
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: i8_to_file(i),
                        rank: curr_rank,
                    },
                });
                //checking if a black piece is on the new position of the Queen
                if get_bit_on_bitboard(b_pieces, i, int_curr_rank) {
                    break;
                }
            }
            for i in (1..int_curr_file).rev() {
                if get_bit_on_bitboard(w_pieces, i, int_curr_rank) {
                    break;
                }
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: i8_to_file(i),
                        rank: curr_rank,
                    },
                });
                //checking if a black piece is on the new position of the Queen
                if get_bit_on_bitboard(b_pieces, i, int_curr_rank) {
                    break;
                }
            }
            for i in (int_curr_rank + 1)..=8 {
                if get_bit_on_bitboard(w_pieces, int_curr_file, i) {
                    break;
                }
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: curr_file,
                        rank: i8_to_rank(i),
                    },
                });
                //checking if a black piece is on the new position of the Queen
                if get_bit_on_bitboard(b_pieces, int_curr_file, i) {
                    break;
                }
            }
            for i in (1..int_curr_rank).rev() {
                if get_bit_on_bitboard(w_pieces, int_curr_file, i) {
                    break;
                }
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: curr_file,
                        rank: i8_to_rank(i),
                    },
                });
                //checking if a black piece is on the new position of the Queen
                if get_bit_on_bitboard(b_pieces, int_curr_file, i) {
                    break;
                }
            }
        }
        //BLACK PIECES
        'p' => {
            //normal move
            if !get_bit_on_bitboard(all_pieces, int_curr_file, int_curr_rank - 1){
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: curr_file,
                        rank: i8_to_rank(int_curr_rank + 1),
                    },
                });

            }
            if curr_rank == '7' && !get_bit_on_bitboard(all_pieces, int_curr_file, int_curr_rank - 2) {
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: curr_file,
                        rank: '5',
                    },
                });
            }

            //capture
            if int_curr_file <= 7 && get_bit_on_bitboard(w_pieces, int_curr_file + 1, int_curr_rank - 1){
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: i8_to_file(int_curr_file + 1),
                        rank: i8_to_rank(int_curr_rank - 1),
                    },
                });

            }
            if int_curr_file >= 2 && get_bit_on_bitboard(w_pieces, int_curr_file - 1, int_curr_rank - 1){
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: i8_to_file(int_curr_file - 1),
                        rank: i8_to_rank(int_curr_rank - 1),
                    },
                });

            }

            // en passent (todo)
        }
        'b' => {
            let mut to_rank = int_curr_rank + 1;
            let mut to_file = int_curr_file + 1;
            while to_rank <= 8 && to_file <= 8 {
                if get_bit_on_bitboard(b_pieces, to_file, to_rank) {
                    break;
                }
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: i8_to_file(to_file),
                        rank: i8_to_rank(to_rank),
                    },
                });
                //checking if a white piece is on the new position of the Bishop
                if get_bit_on_bitboard(w_pieces, to_file, to_rank) {
                    break;
                }
                to_rank += 1;
                to_file += 1;
            }
            to_rank = int_curr_rank - 1;
            to_file = int_curr_file + 1;
            while to_rank >= 1 && to_file <= 8 {
                if get_bit_on_bitboard(b_pieces, to_file, to_rank) {
                    break;
                }
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: i8_to_file(to_file),
                        rank: i8_to_rank(to_rank),
                    },
                });
                //checking if a white piece is on the new position of the Bishop
                if get_bit_on_bitboard(w_pieces, to_file, to_rank) {
                    break;
                }
                to_rank -= 1;
                to_file += 1;
            }
            to_rank = int_curr_rank - 1;
            to_file = int_curr_file - 1;
            while to_rank >= 1 && to_file >= 1 {
                if get_bit_on_bitboard(b_pieces, to_file, to_rank) {
                    break;
                }
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: i8_to_file(to_file),
                        rank: i8_to_rank(to_rank),
                    },
                });
                //checking if a white piece is on the new position of the Bishop
                if get_bit_on_bitboard(w_pieces, to_file, to_rank) {
                    break;
                }
                to_rank -= 1;
                to_file -= 1;
            }
            to_rank = int_curr_rank + 1;
            to_file = int_curr_file - 1;
            while to_rank <= 8 && to_file >= 1 {
                if get_bit_on_bitboard(b_pieces, to_file, to_rank) {
                    break;
                }
                res.push(Move {
                    from: pos,
                    to: Square {
                        file: i8_to_file(to_file),
                        rank: i8_to_rank(to_rank),
                    },
                });
                //checking if a white piece is on the new position of the Bishop
                if get_bit_on_bitboard(w_pieces, to_file, to_rank) {
                    break;
                }
                to_rank += 1;
                to_file -= 1;
            }

        }
        'n' => {

        }
        'r' => {

        }
        'k' => {

        }
        'q' => {

        }
        _ => {}
    }

    println!("Possible Squares to reach:");
    for mv in &res {
        println!("{}{}", mv.to.file, mv.to.rank);
    }
    return res;
}
