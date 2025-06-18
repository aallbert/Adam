use crate::models::board::ChessBoard;
use crate::models::chessmove::{CastleMove, ChessMove};
use crate::models::piece::{Piece, castling};

impl ChessBoard {
    /// Generates possible legal moves in the current position using the move- and attackmasks
    pub fn all_possible_moves(&self) -> Vec<ChessMove> {
        let mut all_moves: Vec<ChessMove> = Vec::new();
        let mut res: Vec<ChessMove> = Vec::new();
        // only generating pieces for the playing color
        if self.get_white_to_move() {
            all_moves.extend(self.w_pawn_moves());
            all_moves.extend(self.w_bishop_moves());
            all_moves.extend(self.w_knight_moves());
            all_moves.extend(self.w_rook_moves());
            all_moves.extend(self.w_king_moves());
            all_moves.extend(self.w_queen_moves());
            // filtering out moves that expose the king to a check (pins)
            for mv in all_moves {
                let new_board = self.with_move(mv);
                let w_king = new_board.get_bitboard(Piece::WhiteKing.into());
                if (w_king & new_board.calc_b_attackmask()).to_u64() == 0 {
                    res.push(mv);
                }
            }
            return res;
        } else {
            all_moves.extend(self.b_pawn_moves());
            all_moves.extend(self.b_bishop_moves());
            all_moves.extend(self.b_knight_moves());
            all_moves.extend(self.b_rook_moves());
            all_moves.extend(self.b_king_moves());
            all_moves.extend(self.b_queen_moves());
            // filtering out moves that expose the king to a check (pins)
            for mv in all_moves {
                let new_board = self.with_move(mv);
                let b_king = new_board.get_bitboard(Piece::BlackKing.into());
                if (b_king & new_board.calc_w_attackmask()).to_u64() == 0 {
                    res.push(mv);
                }
            }
            return res;
        }
    }

    pub fn w_pawn_moves(&self) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let b_pieces = self.get_b_pieces();
        let w_pawns = self.get_bitboard(Piece::WhitePawn.into());

        for curr_sq in w_pawns {
            for dest_sq in self.calc_w_pawn_movemask(curr_sq) {
                // Pawn Promotion, see encoding in ChessMove docu
                if (dest_sq >> 3) == 0 {
                    moves.push(ChessMove::from_curr_and_dest(
                        0b0001_000000 | curr_sq,
                        dest_sq,
                    ));
                    moves.push(ChessMove::from_curr_and_dest(
                        0b0010_000000 | curr_sq,
                        dest_sq,
                    ));
                    moves.push(ChessMove::from_curr_and_dest(
                        0b0100_000000 | curr_sq,
                        dest_sq,
                    ));
                    moves.push(ChessMove::from_curr_and_dest(
                        0b1000_000000 | curr_sq,
                        dest_sq,
                    ));
                } else {
                    moves.push(ChessMove::from_curr_and_dest(curr_sq, dest_sq))
                }
            }
            for dest_sq in ChessBoard::calc_w_pawn_attackmask(curr_sq) {
                if b_pieces.get_bit(dest_sq) || self.get_en_passant() == dest_sq {
                    // Pawn Promotion, see encoding in ChessMove docu
                    if (dest_sq >> 3) == 0 {
                        moves.push(ChessMove::from_curr_and_dest(
                            0b0001_000000 | curr_sq,
                            dest_sq,
                        ));
                        moves.push(ChessMove::from_curr_and_dest(
                            0b0010_000000 | curr_sq,
                            dest_sq,
                        ));
                        moves.push(ChessMove::from_curr_and_dest(
                            0b0100_000000 | curr_sq,
                            dest_sq,
                        ));
                        moves.push(ChessMove::from_curr_and_dest(
                            0b1000_000000 | curr_sq,
                            dest_sq,
                        ));
                    } else {
                        moves.push(ChessMove::from_curr_and_dest(curr_sq, dest_sq))
                    }
                }
            }
        }
        moves
    }

    pub fn b_pawn_moves(&self) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let w_pieces = self.get_w_pieces();
        let b_pawns = self.get_bitboard(Piece::BlackPawn.into());

        for curr_sq in b_pawns {
            for dest_sq in self.calc_b_pawn_movemask(curr_sq) {
                if (dest_sq >> 3) == 7 {
                    // Pawn Promotion, see encoding in ChessMove docu
                    moves.push(ChessMove::from_curr_and_dest(
                        0b0001_000000 | curr_sq,
                        dest_sq,
                    ));
                    moves.push(ChessMove::from_curr_and_dest(
                        0b0010_000000 | curr_sq,
                        dest_sq,
                    ));
                    moves.push(ChessMove::from_curr_and_dest(
                        0b0100_000000 | curr_sq,
                        dest_sq,
                    ));
                    moves.push(ChessMove::from_curr_and_dest(
                        0b1000_000000 | curr_sq,
                        dest_sq,
                    ));
                } else {
                    moves.push(ChessMove::from_curr_and_dest(curr_sq, dest_sq))
                }
            }
            for dest_sq in ChessBoard::calc_b_pawn_attackmask(curr_sq) {
                if w_pieces.get_bit(dest_sq) || self.get_en_passant() == dest_sq {
                    if (dest_sq >> 3) == 7 {
                        // Pawn Promotion, see encoding in ChessMove docu
                        moves.push(ChessMove::from_curr_and_dest(
                            0b0001_000000 | curr_sq,
                            dest_sq,
                        ));
                        moves.push(ChessMove::from_curr_and_dest(
                            0b0010_000000 | curr_sq,
                            dest_sq,
                        ));
                        moves.push(ChessMove::from_curr_and_dest(
                            0b0100_000000 | curr_sq,
                            dest_sq,
                        ));
                        moves.push(ChessMove::from_curr_and_dest(
                            0b1000_000000 | curr_sq,
                            dest_sq,
                        ));
                    } else {
                        moves.push(ChessMove::from_curr_and_dest(curr_sq, dest_sq))
                    }
                }
            }
        }
        moves
    }

    pub fn w_bishop_moves(&self) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let w_pieces = self.get_w_pieces();
        let w_bishops = self.get_bitboard(Piece::WhiteBishop.into());

        for curr_sq in w_bishops {
            for dest_sq in self.calc_bishop_attackmask(curr_sq) {
                if !w_pieces.get_bit(dest_sq) {
                    moves.push(ChessMove::from_curr_and_dest(curr_sq, dest_sq));
                }
            }
        }
        moves
    }

    pub fn b_bishop_moves(&self) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let b_pieces = self.get_b_pieces();
        let b_bishops = self.get_bitboard(Piece::BlackBishop.into());

        for curr_sq in b_bishops {
            for dest_sq in self.calc_bishop_attackmask(curr_sq) {
                if !b_pieces.get_bit(dest_sq) {
                    moves.push(ChessMove::from_curr_and_dest(curr_sq, dest_sq));
                }
            }
        }
        moves
    }

    pub fn w_knight_moves(&self) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let w_pieces = self.get_w_pieces();
        let w_knights = self.get_bitboard(Piece::WhiteKnight.into());

        for curr_sq in w_knights {
            for dest_sq in ChessBoard::calc_knight_attackmask(curr_sq) {
                if !w_pieces.get_bit(dest_sq) {
                    moves.push(ChessMove::from_curr_and_dest(curr_sq, dest_sq));
                }
            }
        }
        moves
    }

    pub fn b_knight_moves(&self) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let b_pieces = self.get_b_pieces();
        let b_knights = self.get_bitboard(Piece::BlackKnight.into());

        for curr_sq in b_knights {
            for dest_sq in ChessBoard::calc_knight_attackmask(curr_sq) {
                if !b_pieces.get_bit(dest_sq) {
                    moves.push(ChessMove::from_curr_and_dest(curr_sq, dest_sq));
                }
            }
        }
        moves
    }

    pub fn w_rook_moves(&self) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let w_pieces = self.get_w_pieces();
        let w_rooks = self.get_bitboard(Piece::WhiteRook.into());

        for curr_sq in w_rooks {
            for dest_sq in self.calc_rook_attackmask(curr_sq) {
                if !w_pieces.get_bit(dest_sq) {
                    moves.push(ChessMove::from_curr_and_dest(curr_sq, dest_sq));
                }
            }
        }
        moves
    }

    pub fn b_rook_moves(&self) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let b_pieces = self.get_b_pieces();
        let b_rooks = self.get_bitboard(Piece::BlackRook.into());

        for curr_sq in b_rooks {
            for dest_sq in self.calc_rook_attackmask(curr_sq) {
                if !b_pieces.get_bit(dest_sq) {
                    moves.push(ChessMove::from_curr_and_dest(curr_sq, dest_sq));
                }
            }
        }
        moves
    }

    pub fn w_king_moves(&self) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let w_pieces = self.get_w_pieces();
        let w_king = self.get_bitboard(Piece::WhiteKing.into());
        let b_attackmask = self.calc_b_attackmask();

        for curr_sq in w_king {
            for dest_sq in ChessBoard::calc_king_attackmask(curr_sq) {
                if !w_pieces.get_bit(dest_sq) && !b_attackmask.get_bit(dest_sq) {
                    moves.push(ChessMove::from_curr_and_dest(curr_sq, dest_sq));
                }
            }
            //castling
            if self.get_castling_rights() & castling::WHITE_K != 0
                && !self.castling_blocked(castling::WHITE_K)
            {
                moves.push(CastleMove::WHITE_K)
            }
            if self.get_castling_rights() & castling::WHITE_Q != 0
                && !self.castling_blocked(castling::WHITE_Q)
            {
                moves.push(CastleMove::WHITE_Q)
            }
        }
        moves
    }

    pub fn b_king_moves(&self) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let b_pieces = self.get_b_pieces();
        let b_king = self.get_bitboard(Piece::BlackKing.into());
        let w_attackmask = self.calc_w_attackmask();

        for curr_sq in b_king {
            for dest_sq in ChessBoard::calc_king_attackmask(curr_sq) {
                if !b_pieces.get_bit(dest_sq) && !w_attackmask.get_bit(dest_sq) {
                    moves.push(ChessMove::from_curr_and_dest(curr_sq, dest_sq));
                }
            }
            //castling
            if self.get_castling_rights() & castling::BLACK_K != 0
                && !self.castling_blocked(castling::BLACK_K)
            {
                moves.push(CastleMove::BLACK_K)
            }
            if self.get_castling_rights() & castling::BLACK_Q != 0
                && !self.castling_blocked(castling::BLACK_Q)
            {
                moves.push(CastleMove::BLACK_Q)
            }
        }
        moves
    }

    pub fn w_queen_moves(&self) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let w_pieces = self.get_w_pieces();
        let w_queens = self.get_bitboard(Piece::WhiteQueen.into());

        for curr_sq in w_queens {
            for dest_sq in self.calc_queen_attackmask(curr_sq) {
                if !w_pieces.get_bit(dest_sq) {
                    moves.push(ChessMove::from_curr_and_dest(curr_sq, dest_sq));
                }
            }
        }
        moves
    }

    pub fn b_queen_moves(&self) -> Vec<ChessMove> {
        let mut moves: Vec<ChessMove> = Vec::new();
        let b_pieces = self.get_b_pieces();
        let b_queens = self.get_bitboard(Piece::BlackQueen.into());

        for curr_sq in b_queens {
            for dest_sq in self.calc_queen_attackmask(curr_sq) {
                if !b_pieces.get_bit(dest_sq) {
                    moves.push(ChessMove::from_curr_and_dest(curr_sq, dest_sq));
                }
            }
        }
        moves
    }

    /// Checking if castling on a side is possible. <br>
    /// The following parameters are considered:
    /// - Is King in check?
    /// - Is the way blocked by a piece?
    /// - Does the player still have castling rights for the side?
    /// - Is the way attacked by an enemy piece?
    pub fn castling_blocked(&self, side: u8) -> bool {
        let all_pieces = self.get_all_pieces();
        let w_attackmask = self.calc_w_attackmask();
        let b_attackmask = self.calc_b_attackmask();
        // mask for checking the indexes for white castling
        let mask_white = all_pieces | b_attackmask;
        let mask_black = all_pieces | w_attackmask;
        match side {
            castling::WHITE_K => {
                (self.get_bitboard(Piece::WhiteKing.into()) & b_attackmask).to_u64() != 0
                    || self.get_castling_rights() & castling::WHITE_K == 0
                    || mask_white.get_bit(61u16)
                    || mask_white.get_bit(62u16)
            }
            castling::WHITE_Q => {
                (self.get_bitboard(Piece::WhiteKing.into()) & b_attackmask).to_u64() != 0
                    || self.get_castling_rights() & castling::WHITE_Q == 0
                    || mask_white.get_bit(59u16)
                    || mask_white.get_bit(58u16)
                    || mask_white.get_bit(57u16)
            }
            castling::BLACK_K => {
                (self.get_bitboard(Piece::BlackKing.into()) & w_attackmask).to_u64() != 0
                    || self.get_castling_rights() & castling::BLACK_K == 0
                    || mask_black.get_bit(5u16)
                    || mask_black.get_bit(6u16)
            }
            castling::BLACK_Q => {
                (self.get_bitboard(Piece::BlackKing.into()) & w_attackmask).to_u64() != 0
                    || self.get_castling_rights() & castling::BLACK_Q == 0
                    || mask_black.get_bit(3u16)
                    || mask_black.get_bit(2u16)
                    || mask_black.get_bit(1u16)
            }
            _ => panic!("Wrong Castling configuration provided!"),
        }
    }
}
