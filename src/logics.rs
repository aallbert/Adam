use crate::core::movegen::possible_moves;
use crate::interface::{Bitboard, BoardType, FenBoard, Move, SimpleBoard, Square, file_to_i8};

fn fen_to_simple(fen_board: FenBoard) -> SimpleBoard {
    let mut fen_board_slice = fen_board.chars();
    let mut simple_board: SimpleBoard = vec![];

    while let Some(c) = fen_board_slice.next() {
        match c {
            '1'..='8' => {
                for _ in 0..c.to_digit(10).unwrap() {
                    simple_board.push('.');
                }
            }
            '/' => {}
            _ => {
                simple_board.push(c);
            }
        }
    }

    return simple_board;
}

fn simple_to_bitboard(board_type: BoardType, simple_board: &SimpleBoard) -> Bitboard {
    let mut bitboard: u64 = 0;
    match board_type {
        BoardType::WhiteAllPieces => {
            for j in 0..64 {
                bitboard <<= 1;
                if ('A'..='Z').contains(&simple_board[j]) {
                    bitboard += 1;
                }
                println!("Bitboard White, {0:b}", bitboard);
            }
            return bitboard;
        }
        BoardType::BlackAllPieces => {
            for j in 0..64 {
                bitboard <<= 1;
                if ('a'..='z').contains(&simple_board[j]) {
                    bitboard += 1;
                }
                println!("Bitboard Black, {0:b}", bitboard);
            }
            return bitboard;
        }
    }
}

fn simple_to_fen(simple_board: Vec<char>) -> FenBoard {
    let mut fen_board: FenBoard = String::from("");
    for i in 0..8 {
        let mut space_counter: u8 = 0;
        for j in 0..8 {
            let c = simple_board[j + 8 * i];
            if c == '.' {
                space_counter += 1;
            } else {
                if space_counter != 0 {
                    fen_board.push((space_counter + b'0') as char);
                    space_counter = 0;
                }
                fen_board.push(c);
            }
        }
        if space_counter != 0 {
            fen_board.push((space_counter + b'0') as char);
        }
        fen_board.push('/');
    }
    fen_board.pop();
    return fen_board;
}

///Validates a move on whether the piece can make this move
// fn piece_move_validation(pc: char, mv_chars: Vec<char>) -> bool {
//     let from_col = mv_chars[0];
//     let from_row = mv_chars[1];
//     let to_col = mv_chars[2];
//     let to_row = mv_chars[3];
//     match pc {
//         'P' => {
//             if (from_row - to_row)
//         }
//         'B' => {false}
//         'N' => {false}
//         'R' => {false}
//         'K' => {false}
//         'Q' => {false}
//         _ => {false}
//     }
// }

pub fn move_validation(fen_board: FenBoard, mv_string: String) -> FenBoard {
    let chars: Vec<char> = mv_string.chars().collect();
    let from_file = chars[0];
    let from_rank = chars[1];
    let to_file = chars[2];
    let to_rank = chars[3];

    let mv: Move = Move {
        from: Square {
            file: from_file,
            rank: from_rank,
        },
        to: Square {
            file: to_file,
            rank: to_rank,
        },
    };

    if !('a'..='h').contains(&from_file)
        || !('1'..='8').contains(&from_rank)
        || !('a'..='h').contains(&to_file)
        || !('1'..='8').contains(&to_rank)
    {
        println!(
            "Unvalid: indexing is out of range for move '{:?}'",
            mv_string
        );
        return String::from("");
    }

    let int_from_file: i8 = file_to_i8(mv.from.file);
    let int_to_file: i8 = file_to_i8(mv.to.file);
    let int_from_rank: i8 = mv.from.rank.to_digit(10).unwrap() as i8;
    let int_to_rank: i8 = mv.to.rank.to_digit(10).unwrap() as i8;
    let mut simple_board: SimpleBoard = fen_to_simple(fen_board);

    println!(
        "int_from_row: {}, int_from_col: {}",
        int_from_rank, int_from_file
    );

    let simple_index_from: usize = (-(int_from_rank - 8) * 8 + int_from_file - 1)
        .try_into()
        .unwrap();
    let simple_index_to: usize = (-(int_to_rank - 8) * 8 + int_to_file - 1)
        .try_into()
        .unwrap();

    println!(
        "simple_index_from: {}, simple_index_to: {}",
        simple_index_from, simple_index_to
    );

    let pc = simple_board[simple_index_from];

    // println!("simple board \n{:?}", simple_board);

    let b_all_pieces: Bitboard = simple_to_bitboard(BoardType::BlackAllPieces, &simple_board);
    let w_all_pieces: Bitboard = simple_to_bitboard(BoardType::WhiteAllPieces, &simple_board);

    possible_moves(pc, mv.from, b_all_pieces, w_all_pieces);

    simple_board[simple_index_to] = pc;
    simple_board[simple_index_from] = '.';

    // if b_all_pieces & w_all_pieces != 0 {
    //      println!("Capture!")
    //     };

    let new_fen = simple_to_fen(simple_board);
    println!("new fen \n{:?}", new_fen);

    return new_fen;
}
