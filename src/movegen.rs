#![allow(dead_code)] // Functions are only used outside of this module
use crate::board::Board;

const FILE1: u64 = 0b1000000010000000100000001000000010000000100000001000000010000000u64;
const FILE8: u64 = 0b0000000100000001000000010000000100000001000000010000000100000001u64;
const RANK2: u64 = 0b0000000000000000000000000000000000000000000000001111111100000000u64;
const RANK7: u64 = 0b0000000011111111000000000000000000000000000000000000000000000000u64;
const RANK_MID: u64 = 0x0000FFFFFFFF0000;
const RANK_18: u64 = 0xFF000000000000FF;

pub const fn pawns_notleft() -> u64 {
  !FILE1
}

pub const fn pawns_notright() -> u64 {
  !FILE8
}

pub const fn pawn_forward(mask: u64, is_white: bool) -> u64 {
  if is_white { mask << 8 } else { mask >> 8 }
}

pub const fn pawn_forward_2(mask: u64, is_white: bool) -> u64 {
  if is_white { mask << 16 } else { mask >> 16 }
}

pub const fn pawn_backward(mask: u64, is_white: bool) -> u64 {
  if is_white { mask >> 8 } else { mask << 8 }
}

pub const fn pawn_backward_2(mask: u64, is_white: bool) -> u64 {
  if is_white { mask >> 16 } else { mask << 16 }
}

pub const fn pawn_atk_left(mask: u64, is_white: bool) -> u64 {
  if is_white { mask << 9 } else { mask >> 7 }
}

pub const fn pawn_atk_right(mask: u64, is_white: bool) -> u64 {
  if is_white { mask << 7 } else { mask >> 9 }
}

pub const fn pawn_invert_left(mask: u64, is_white: bool) -> u64 {
  pawn_atk_right(mask, !is_white)
}

pub const fn pawn_invert_right(mask: u64, is_white: bool) -> u64 {
  pawn_atk_left(mask, !is_white)
}

pub const fn pawn_first_rank(is_white: bool) -> u64 {
  if is_white { RANK2 } else { RANK7 }
}

pub const fn pawn_last_rank(is_white: bool) -> u64 {
  if is_white { RANK7 } else { RANK2 }
}

pub const fn king(board: &Board, is_white: bool) -> u64 {
  if is_white { board.w_king } else { board.b_king }
}

pub const fn enemy_king(board: &Board, is_white: bool) -> u64 {
  if is_white { board.b_king } else { board.w_king }
}

pub const fn pawns(board: &Board, is_white: bool) -> u64 {
  if is_white { board.w_pawn } else { board.b_pawn }
}

pub const fn own_color(board: &Board, is_white: bool) -> u64 {
  if is_white { board.white } else { board.black }
}

pub const fn enemy(board: &Board, is_white: bool) -> u64 {
  if is_white { board.black } else { board.white }
}

pub const fn enemy_rook_queen(board: &Board, is_white: bool) -> u64 {
  if is_white { board.b_rook | board.b_queen } else { board.w_rook | board.w_queen }
}

pub const fn rook_queen(board: &Board, is_white: bool) -> u64 {
  if is_white { board.w_rook | board.w_queen } else { board.b_rook | board.b_queen }
}

pub const fn enemy_bishop_queen(board: &Board, is_white: bool) -> u64 {
  if is_white { board.b_bishop | board.b_queen } else { board.w_bishop | board.w_queen }
}

pub const fn bishop_queen(board: &Board, is_white: bool) -> u64 {
  if is_white { board.w_bishop | board.w_queen } else { board.b_bishop | board.b_queen }
}

pub const fn enemy_or_empty(board: &Board, is_white: bool) -> u64 {
  if is_white { !board.white } else { !board.black }
}

pub const fn empty(board: &Board) -> u64 {
  !board.occ
}

pub const fn knights(board: &Board, is_white: bool) -> u64 {
  if is_white { board.w_knight } else { board.b_knight }
}

pub const fn rooks(board: &Board, is_white: bool) -> u64 {
  if is_white { board.w_rook } else { board.b_rook }
}

pub const fn bishops(board: &Board, is_white: bool) -> u64 {
  if is_white { board.w_bishop } else { board.b_bishop }
}

pub const fn queens(board: &Board, is_white: bool) -> u64 {
  if is_white { board.w_queen } else { board.b_queen }
}