use crate::movemap;
use crate::movemap::lookup_pext as lookup;
use crate::movegen::*;
use crate::board::{BoardStatus, Board, square_of};

struct Movestack {
  king_attacks: [u64; 32], 
  e_king_attacks: [u64; 32],
  check_status: [u64; 32],
}

impl Movestack {
  fn new() -> Self {
    Movestack {
      king_attacks: [0; 32],
      e_king_attacks: [0; 32],
      check_status: [0; 32],
    }
  }

  fn set_king_attacks(&mut self, index: usize, king_attacks: u64) {
    self.king_attacks[index] = king_attacks;
  }

  fn set_e_king_attacks(&mut self, index: usize, e_king_attacks: u64) {
    self.e_king_attacks[index] = e_king_attacks;
  }

  fn set_check_status(&mut self, index: usize, check_status: u64) {
    self.check_status[index] = check_status;
  }
}

struct Movelist {
  en_passant_target: u64,
  rook_pin: u64, // straight pins
  bishop_pin: u64, // diagonal pins
  movestack: Movestack,
}

impl Movelist {
  fn new() -> Self {
    Movelist {
      en_passant_target: 0,
      rook_pin: 0,
      bishop_pin: 0,
      movestack: Movestack::new(),
    }
  }

  fn initstack(&mut self, status: BoardStatus, board: &Board, depth: usize) {
    let (white, enemy) = (status.white_move, !status.white_move);

    self.movestack.king_attacks[depth] = lookup::king(square_of(king(board, white)));
    self.movestack.e_king_attacks[depth] = lookup::king(square_of(king(board, enemy)));

    // Calculate checks from enemy pawns
    {
      let pl = pawn_atk_left(pawns(board, enemy) & pawns_notleft(), enemy);
      let pr = pawn_atk_right(pawns(board, enemy) & pawns_notright(), enemy);

      if pl & king(board, white) != 0 {
        self.movestack.check_status[depth] = pawn_atk_right(king(board, white), white);
      } else if (pr & king(board, white)) != 0 {
        self.movestack.check_status[depth] = pawn_atk_left(king(board, white), white);
      } else {
        self.movestack.check_status[depth] = 0xffffffffffffffffu64;
      }
    }

    // Calculate checks from enemy knights
    {
      let knightcheck = lookup::knight(square_of(king(board, white))) & knights(board, enemy);
      if knightcheck != 0 {
        self.movestack.check_status[depth] = knightcheck;
      }
    }
  }

  fn init(&mut self, ep_init: u64) {
    self.en_passant_target = ep_init;
  }

  // PinHVD1D2 |= path from enemy to ecluding king + enemy, has see map from king as input
  // Must have enemy slider and own piece or -- very special: can clear enemy enpassant pawn
  fn register_pin_d12(&mut self, status: BoardStatus, king: u64, enemy: u64, board: &Board) {
    let pinmask: u64 = movemap::pin_between::PIN_BETWEEN[(king * 64 + enemy) as usize];

    if status.has_enpassant_pawn && pinmask & self.en_passant_target != 0 {
      self.en_passant_target = 0;
    }

    if pinmask & own_color(board, status.white_move) != 0 {
      self.bishop_pin |= pinmask;
    }
  }

}