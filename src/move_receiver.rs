use crate::{movelist::Movelist, board::{BoardStatus, Board, BoardPiece}};
use crate::movegen::*;

pub trait MoveCallback {
  fn kingmove(board: &BoardStatus, depth: u64, board: &Board, from: u64, to: u64);
}

impl MoveCallback for Movelist {
  fn kingmove(status: &BoardStatus, depth: u64, board: &Board, from: u64, to: u64) {
    let next: Board = Board::move_piece(
      BoardPiece::King, board, status.white_move, (to & enemy(board, status.white_move)) != 0, from, to
    );
  }
}