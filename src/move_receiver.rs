use crate::{movelist::{Movelist, self}, board::{BoardStatus, Board, BoardPiece}};
use crate::movegen::*;

pub struct MoveCallback {
  nodes: u64,
  movelist: Movelist
}

impl MoveCallback {
  fn init(&self, board: Board, ep_init: u64) -> MoveCallback {
    MoveCallback {
      nodes: 0,
      movelist: Movelist::new(ep_init)
    }
  }

  fn perf_t0(&mut self) {
    self.nodes += 1;
  }

  fn perf_t1(&mut self, status: BoardStatus, board: Board) {
    self.nodes += Movelist::count(&mut self.movelist, status, board);
  }

  fn perf_t(&mut self, status: BoardStatus, depth: u64, board: Board) {
    if depth == 1 {
      self.perf_t1(status, board);
    } else {
      return;
      // movelist::enumerate_moves(status, movereciever, board, depth);
    }
  }

  fn kingmove(status: &BoardStatus, depth: u64, board: &Board, from: u64, to: u64) {
    let next: Board = Board::move_piece(
      BoardPiece::King, board, status.white_move, (to & enemy(board, status.white_move)) != 0, from, to
    );
  }
}