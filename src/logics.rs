use crate::core::movegen::possible_moves;
use crate::interface::BoardType;
use crate::models::board::{Bitboard, FenBoard, SimpleBoard};
use crate::models::chessmove::{ChessMove, ChessMoveChar};

fn fen_to_simple(fen_board: FenBoard) -> SimpleBoard {
    let mut fen_board_slice = fen_board.chars();
    let mut simple_board = SimpleBoard::new(None);

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
    let mut bitboard = Bitboard::new(0);
    match board_type {
        BoardType::WhiteAllPieces => {
            for j in 0..64 {
                bitboard <<= 1;
                if ('A'..='Z').contains(&simple_board.as_vec_char()[j]) {
                    bitboard += 1;
                }
            }
            return bitboard;
        }
        BoardType::BlackAllPieces => {
            for j in 0..64 {
                bitboard <<= 1;
                if ('a'..='z').contains(&simple_board.as_vec_char()[j]) {
                    bitboard += 1;
                }
            }
            return bitboard;
        }
    }
}

fn simple_to_fen(simple_board: &Vec<char>) -> FenBoard {
    let mut fen_board = FenBoard::new("");
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

pub fn make_a_move_testing(fen_board: FenBoard, mv_string: String) -> FenBoard {
    let chars: Vec<char> = mv_string.chars().collect();
    let curr_file = chars[0];
    let curr_rank = chars[1];
    let dest_file = chars[2];
    let dest_rank = chars[3];

    let mv_char = ChessMoveChar::new_with_chars(curr_rank, curr_file, dest_rank, dest_file);
    let mv = mv_char.to_chessmove();

    if !('a'..='h').contains(&curr_file)
        || !('1'..='8').contains(&curr_rank)
        || !('a'..='h').contains(&dest_file)
        || !('1'..='8').contains(&dest_rank)
    {
        println!(
            "Unvalid: indexing is out of range for move '{:?}'",
            mv_string
        );
        return FenBoard::new("");
    }
    let mut simple_board: SimpleBoard = fen_to_simple(fen_board);

    let simple_index_from = mv.get_curr_square_as_index();
    let simple_index_to = mv.get_dest_square_as_index();

    println!(
        "simple_index_from: {}, simple_index_to: {}",
        simple_index_from, simple_index_to
    );

    let pc = simple_board.as_vec_char()[simple_index_from as usize];

    // println!("simple board \n{:?}", simple_board);

    let b_all_pieces: Bitboard = simple_to_bitboard(BoardType::BlackAllPieces, &simple_board);
    let w_all_pieces: Bitboard = simple_to_bitboard(BoardType::WhiteAllPieces, &simple_board);

    possible_moves(pc, mv, b_all_pieces, w_all_pieces);

    simple_board.set(simple_index_to as usize, pc);
    simple_board.set(simple_index_from as usize, '.');

    // if b_all_pieces & w_all_pieces != 0 {
    //      println!("Capture!")
    //     };

    let new_fen = simple_to_fen(simple_board.as_vec_char());
    println!("new fen \n{:?}", new_fen);

    return new_fen;
}
