use crate::movemap::*;
use crate::movemap::lookup_hash::bishop;
use crate::movemap::lookup_pext as lookup;
use crate::movegen::*;
use crate::board::{BoardStatus, Board, square_of, bitloop};

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

  fn initstack(&mut self, status: &BoardStatus, board: &Board, depth: usize) {
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

  // Pin HV D1 D2 |= path from enemy to excluding king + enemy, has see map from king as input
  // Must have enemy slider and own piece or -- very special: can clear enemy enpassant pawn
  fn register_pin_d12(&mut self, status: &BoardStatus, king: u64, enemy: u64, board: &Board) {
    let pinmask: u64 = pin_between::PIN_BETWEEN[(king * 64 + enemy) as usize];

    // Deep possible chess problem:
    // Enemy EP pawn gives check, and could be taken by own EP pawn, but it's pinned so it's not valid
    // Therefore we delete EP status
    // https://lichess.org/editor/6q1/8/8/3pP3/8/1K6/8/8_w_-_-_0_1?color=white

    if status.has_enpassant_pawn && pinmask & self.en_passant_target != 0 {
      self.en_passant_target = 0;
    }

    if pinmask & own_color(board, status.white_move) != 0 {
      self.bishop_pin |= pinmask;
    }
  }

  // Pin HV D1 D2 |= Path from enemy to exluding king + enemy, has see map from king as input
  fn register_pin_hv(&mut self, status: &BoardStatus, king: u64, enemy: u64, board: &Board) {
    let pinmask: u64 = pin_between::PIN_BETWEEN[(king * 64 + enemy) as usize];

    if pinmask & own_color(board, status.white_move) != 0 {
      self.rook_pin |= pinmask;
    }
  }

  fn register_pin_ep(&mut self, status: &BoardStatus, kingsquare: u64, king: u64, enemy_rq: u64, board: &Board) {
    fn ep_rank(is_white: bool) -> u64 {
      0xffu64 << if is_white { 32 } else { 24 } 
    }

    let white: bool = status.white_move;
    let pawns: u64 = pawns(board, white);

    // check for horizontal en passant targets that are pinned
    if (ep_rank(white) & king) != 0 && (ep_rank(white) & enemy_rq) != 0 && (ep_rank(white) & pawns) != 0 {
      let epl_pawn: u64 = pawns & ((self.en_passant_target & pawns_notright()) >> 1);
      let epr_pawn: u64 = pawns & ((self.en_passant_target & pawns_notleft()) << 1);

      let after_ep_occ: u64 = board.occ & !(self.en_passant_target | epl_pawn | epr_pawn);
      
      if lookup::rook(kingsquare, after_ep_occ) & enemy_rq != 0 {
        self.en_passant_target = 0;
      }
    }
  }

  fn refresh(&mut self, status: &BoardStatus, depth: u64, board: &Board, kingban: &mut u64, checkmask: &mut u64) -> u64 {    
    fn check_by_slider(king: u64, enemy: u64, kingban: &mut u64, checkmask: &mut u64) {
      *checkmask = if *checkmask == 0xffffffffffffffffu64 { pin_between::PIN_BETWEEN[(king * 64 + enemy) as usize] } else { 0 };
      *kingban |= check_between::CHECK_BETWEEN[(king * 64 + enemy) as usize];
    }

    let (white, enemy) = (status.white_move, !status.white_move);
    let king = king(board, white);
    let kingsq = square_of(king);

    {
      (self.rook_pin, self.bishop_pin) = (0, 0);

      if (masks::ROOK_MASK[kingsq as usize] & enemy_rook_queen(board, white)) != 0 {
        let atk_hv = lookup::rook(kingsq, board.occ) & enemy_rook_queen(board, white);
        bitloop(atk_hv, |atk_hv: u64|
          {
            let sq = square_of(atk_hv);
            check_by_slider(kingsq, sq, kingban, checkmask);
          }
        );

        let pinners_hv = lookup::rook_xray(kingsq, board.occ) & enemy_rook_queen(board, white);
        bitloop(pinners_hv, |pinners_hv: u64|
          {
            self.register_pin_hv(status, kingsq, square_of(pinners_hv), board);
          }
        );
      }

      if (masks::BISHOP_MASK[kingsq as usize] & enemy_bishop_queen(board, white)) != 0 {
        let atk_d12: u64 = lookup::bishop(kingsq, board.occ) & enemy_bishop_queen(board, white);
        bitloop(atk_d12, |atk_d12: u64|
          {
            let sq = square_of(atk_d12);
            check_by_slider(kingsq, sq, kingban, checkmask);
          }
        );

        let pinner_d12: u64 = lookup::bishop_xray(kingsq, board.occ) & enemy_bishop_queen(board, white);
        bitloop(pinner_d12, |pinner_d12: u64|
          {
            self.register_pin_d12(status, kingsq, square_of(pinner_d12), board);
          }
        );
      }

      if status.has_enpassant_pawn {
        self.register_pin_ep(status, kingsq, king, enemy_rook_queen(board, white), board);
      }
    }

    self.movestack.check_status[(depth - 1) as usize] = 0xffffffffffffffffu64;

    let king_atk = self.movestack.king_attacks[depth as usize] & enemy_or_empty(board, white) & !*kingban;
    if king_atk == 0 { return 0; }

    // Calculate enemy knights
    {
      let knights = knights(board, enemy);
      bitloop(knights, |knights: u64|
        {
          *kingban |= lookup::knight(square_of(knights));
        }
      )
    }

    // Calculate check from enemy pawns
    {
      let pl = pawn_atk_left(pawns(board, enemy) & pawns_notleft(), enemy);
      let pr = pawn_atk_right(pawns(board, enemy) & pawns_notright(), enemy);

      *kingban |= pl | pr;
    }

    // Calculate enmy bishops
    {
      let bishops = bishop_queen(board, enemy);
      bitloop(bishops, |bishops: u64| { *kingban |= lookup::bishop(square_of(bishops), board.occ); });
    }

    // Calculate enemy rooks
    {
      let rooks = rook_queen(board, enemy);
      bitloop(rooks, |rooks: u64| { *kingban |= lookup::rook(square_of(rooks), board.occ); });
    }

    king_atk & !*kingban
  }

  fn pawn_prune_left(pawn: &mut u64, pin_d1d2: u64, white: bool) {
    let pinned: u64 = *pawn & pawn_invert_left(pin_d1d2 & pawns_notright(), white);
    let unpinned: u64 = *pawn & !pin_d1d2;
    *pawn = pinned | unpinned;
  }

  fn pawn_prune_right(pawn: &mut u64, pin_d1d2: u64, white: bool) {
    let pinned: u64 = *pawn & pawn_invert_right(pin_d1d2 & pawns_notleft(), white);
    let unpinned: u64 = *pawn & !pin_d1d2;
    *pawn = pinned | unpinned;
  }

  // Original has pawn_prune_left_ep and pawn_prune_right_ep that are identical to pawn_prune_left and pawn_prune_right
  fn pawn_prune_move(pawn: &mut u64, pin_hv: u64, white: bool) {
    let pinned: u64 = *pawn & pawn_backward(pin_hv, white);
    let unpinned: u64 = *pawn & !pin_hv;
    *pawn = pinned | unpinned;
  }

  fn pawn_prune_move_2(pawn: &mut u64, pin_hv: u64, white: bool) {
    let pinned: u64 = *pawn & pawn_backward_2(pin_hv, white);
    let unpinned: u64 = *pawn & !pin_hv;
    *pawn = pinned | unpinned;
  }

}