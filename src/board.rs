#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
// use crate::movemap::lookup_hash; // uses multiply look up
// use crate::movemap::lookup_switch; // uses ifchain to calculate seen squares
use crate::movemap::lookup_pext; // Fastest on hardware pext CPUs, Ryzen 5000+ & Intel
type Bit = u64;
type Square = u64;
type Map = u64;
use std::arch::x86_64::{_tzcnt_u64, _blsr_u64};
use std::fmt;

fn square_of(x: u64) -> u64 {
  unsafe { _tzcnt_u64(x) }
}

struct BoardStatus {
  white_move: bool,
  has_enpassant_pawn: bool,
  w_castle_l: bool,
  w_castle_r: bool,
  b_castle_l: bool,
  b_castle_r: bool
}

impl BoardStatus {
  const W_NOT_OCCUPIED_L: u64 = 0b01110000u64;
  const W_NOT_ATTACKED_L: u64 = 0b01110000u64;

  const W_NOT_OCCUPIED_R: u64 = 0b00000110u64;
  const W_NOT_ATTACKED_R: u64 = 0b00001110u64;

  const B_NOT_OCCUPIED_L: u64 = 0b01110000u64 << 56u64;
  const B_NOT_ATTACKED_L: u64 = 0b00111000u64 << 56u64;

  const B_NOT_OCCUPIED_R: u64 = 0b00000110u64 << 56u64;
  const B_NOT_ATTACKED_R: u64 = 0b00001110u64 << 56u64;

  const W_ROOK_L_CHANGE: u64 = 0b11111000u64;
  const B_ROOK_L_CHANGE: u64 = 0b11111000u64 << 56u64;
  const W_ROOK_R_CHANGE: u64 = 0b00000111u64;
  const B_ROOK_R_CHANGE: u64 = 0b00000111u64 << 56u64;

  const W_ROOK_L: u64 = 0b10000000u64;
  const B_ROOK_L: u64 = 0b10000000u64 << 56u64;
  const W_ROOK_R: u64 = 0b00000001u64;
  const B_ROOK_R: u64 = 0b00000001u64 << 56u64;

  const fn new(white: bool, enpassant: bool, wcl: bool, wcr: bool, bcl: bool, bcr: bool) -> Self {
    Self {
      white_move: white,
      has_enpassant_pawn: enpassant,
      w_castle_l: wcl,
      w_castle_r: wcr,
      b_castle_l: bcl,
      b_castle_r: bcr
    }
  }

  const fn update(pat: u32) -> Self {
    Self {
      white_move: (pat & 0b100000) != 0,
      has_enpassant_pawn: (pat & 0b010000) != 0,
      w_castle_l: (pat & 0b001000) != 0,
      w_castle_r: (pat & 0b000100) != 0,
      b_castle_l: (pat & 0b000010) != 0,
      b_castle_r: (pat & 0b000001) != 0
    }
  }

  const fn can_castle(&self) -> bool {
    if self.white_move {
      return self.w_castle_l | self.w_castle_r;
    }
    self.b_castle_l | self.b_castle_r
  }

  const fn can_castle_l(&self) -> bool {
    if self.white_move {
      return self.w_castle_l;
    }
    self.b_castle_l
  }

  const fn can_castle_r(&self) -> bool {
    if self.white_move {
      return self.w_castle_r;
    }
    self.b_castle_r
  }

  const fn castle_rookswitch_r(&self) -> u64 {
    if self.white_move {
      return 0b00000101u64;
    }
    0b00000101u64 << 56u64
  }

  const fn castle_rookswitch_l(&self) -> u64 {
    if self.white_move {
      return 0b10010000u64;
    }
    0b10010000u64 << 56u64
  }

  const fn can_castle_left(&self, attacked: Map, occupied: Map, rook: Map) -> bool {
    if self.white_move && self.w_castle_l {
      if (occupied & Self::W_NOT_OCCUPIED_L) != 0 { return false; }
      if (attacked & Self::W_NOT_ATTACKED_L) != 0 { return false; }
      if (rook & Self::W_ROOK_L) != 0 { return true; }
      return false;
    } else if self.b_castle_l {
      if (occupied & Self::B_NOT_OCCUPIED_L) != 0 { return false; }
      if (attacked & Self::B_NOT_ATTACKED_L) != 0 { return false; }
      if (rook & Self::B_ROOK_L) != 0 { return true; }
      return false;
    }
    false
  }

  const fn can_castle_right(&self, attacked: Map, occupied: Map, rook: Map) -> bool {
    if self.white_move && self.w_castle_r {
      if (occupied & Self::W_NOT_OCCUPIED_R) != 0 { return false; }
      if (attacked & Self::W_NOT_ATTACKED_R) != 0 { return false; }
      if (rook & Self::W_ROOK_R) != 0 { return true; }
      return false;
    } else if self.b_castle_r {
      if (occupied & Self::B_NOT_OCCUPIED_R) != 0 { return false; }
      if (attacked & Self::B_NOT_ATTACKED_R) != 0 { return false; }
      if (rook & Self::B_ROOK_R) != 0 { return true; }
      return false;
    }
    false
  }

  const fn is_left_rook(&self, rook: Bit) -> bool {
    if self.white_move {
      return rook == Self::W_ROOK_L;
    } else {
      rook == Self::B_ROOK_L      
    }
  }

  const fn is_right_rook(&self, rook: Bit) -> bool {
    if self.white_move {
      return rook == Self::W_ROOK_R;
    } else {
      return rook == Self::B_ROOK_R;
    }
  }

  // Enable en passant for all pawn moves, part of pseudolegal generation.
  const fn pawn_push(&self) -> BoardStatus {
    return Self::new(!self.white_move, true, self.w_castle_l, self.w_castle_r, self.b_castle_l, self.b_castle_r);
  }

  // Disable castling rights for whoever's king moved.
  const fn king_move(&self) -> BoardStatus {
    if self.white_move {
      return Self::new(!self.white_move, false, false, false, self.b_castle_l, self.b_castle_r);
    } else {
      return Self::new(!self.white_move, false, self.w_castle_l, self.w_castle_r, false, false);
    }
  }

  // Disable left castling rights for mover.
  const fn rook_move_left(&self) -> BoardStatus {
    if self.white_move {
      return Self::new(!self.white_move, false, false, self.w_castle_r, self.b_castle_l, self.b_castle_r);
    } else {
      return Self::new(!self.white_move, false, self.w_castle_l, self.w_castle_r, false, self.b_castle_r);
    }
  }

  // Disable right castling rights for mover.
  const fn rook_move_right(&self) -> BoardStatus {
    if self.white_move {
      return Self::new(!self.white_move, false, self.w_castle_l, false, self.b_castle_l, self.b_castle_r);
    } else {
      return Self::new(!self.white_move, false, self.w_castle_l, self.w_castle_r, self.b_castle_l, false);
    }
  }

  // Non rook or king move, no castling rights lost.
  const fn silent_move(&self) -> BoardStatus {
    return Self::new(!self.white_move, false, self.w_castle_l, self.w_castle_r, self.b_castle_l, self.b_castle_r);
  }

  // Default state: white to move, not in check and all castling rights + not en passant.
  const fn default(&self) -> BoardStatus {
    return Self::new(true, false, true, true, true, true);
  }
}

// fen writer
impl fmt::Display for BoardStatus {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", if self.white_move { "w" } else { "b" })?;
    write!(f, "{}", if self.has_enpassant_pawn { " ep:1" } else { " ep:0" })?;

    if !self.w_castle_l && !self.w_castle_r && !self.b_castle_l && !self.b_castle_r {
      write!(f, " castle:0")?;
    } else {
      write!(f, " castle:")?;
      if self.w_castle_l { write!(f, "K")?; }
      if self.w_castle_r { write!(f, "Q")?; }
      if self.b_castle_l { write!(f, "q")?; }
      if self.b_castle_r { write!(f, "k")?; }
    }
    Ok(())
  }
}

fn map3(val1: u64, val2: u64, val3: u64) -> String {
  let mut board = vec!['o'; 64 * 3 + 3 * 8];

  let mut c = 0;
  for i in 0..64 {
    let bitmask: u64 = (1u64 << 63) >> i;
    if (bitmask & val1) != 0 {
      board[c] = 'X';
    } else {
      board[c] = '.';
    }

    if (bitmask & val2) != 0 {
      board[c + 9] = 'X';
    } else {
      board[c + 9] = '.';
    }

    if (bitmask & val3) != 0 {
      board[c + 18] = 'X';
    } else {
      board[c + 18] = '.';
    }

    c += 1;

    if (i + 1) % 8 == 0 {
      board[c] = ' ';
      board[c + 9] = ' ';
      board[c + 18] = '\n';
      c += 19;
    }
  }
  board.iter().collect::<String>()
}

fn map2(val1: u64, val2: u64) -> String {
  let mut board = vec!['o'; 64 * 2 + 2 * 8];

  let mut c = 0;
  for i in 0..64 {
    let bitmask: u64 = (1u64 << 63) >> i;
    if (bitmask & val1) != 0 {
      board[c] = 'X';
    } else {
      board[c] = '.';
    }

    if (bitmask & val2) != 0 {
      board[c + 9] = 'X';
    } else {
      board[c + 9] = '.';
    }

    c += 1;

    if (i + 1) % 8 == 0 {
      board[c] = ' ';
      board[c + 9] = '\n';
      c += 10;
    }
  }
  board.iter().collect::<String>()
}

fn map1(value: u64) -> String {
  let mut board = vec!['o'; 64 + 8];

  let mut c = 0;
  for i in 0..64 {
    let bitmask: u64 = (1u64 << 63) >> i;

    if (bitmask & value) != 0 {
      board[c] = 'X';
    } else {
      board[c] = '.';
    }

    c += 1;

    if (i + 1) % 8 == 0 {
      board[c] = '\n';
      c += 1;
    }
  }

  board.iter().collect::<String>()
}

#[derive(PartialEq)]
enum FenField {
  White,
  HasEp,
  WCastleL,
  WCastleR,
  BCastleL,
  BCastleR
}

pub struct FEN;

impl FEN {
  fn fen_enpassant(fen: &str) -> u64 {
    let mut chars = fen.chars();

    // Skip piece positions to active color field
    while chars.next().unwrap_or(' ') != ' ' {}

    // Skip space, set to active color
    let col = chars.next().unwrap_or(' ');

    // Skip castling
    while chars.next().unwrap_or(' ') != ' ' {}

    // En passant
    let e_or_minus = chars.next().unwrap_or(' ');
    if e_or_minus != '-' {
      let file_offset = ('h' as u8 - e_or_minus as u8) as u64;
      return match col {
        'w' => 1u64 << (32 + file_offset),
        'b' => 1u64 << (24 + file_offset),
        _ => 0,
      }
    }
    0
  }

  // parse info from fen string, I think it looks nicer than the C++ actually :)
  fn fen_info(field: FenField, fen: &str) -> bool {
    let mut chars = fen.chars();

    // Skip piece positions to active color field
    while chars.next().unwrap_or(' ') != ' ' {}

    let col = chars.next().unwrap_or(' ');

    match field {
      FenField::White => col == 'w',
      FenField::WCastleL | FenField::WCastleR | FenField::BCastleL | FenField::BCastleR => {
        let mut cr = false;
        while let Some(c) = chars.next() {
          match c {
            'K' if field == FenField::WCastleR => cr = true,
            'Q' if field == FenField::WCastleL => cr = true,
            'k' if field == FenField::BCastleR => cr = true,
            'q' if field == FenField::BCastleL => cr = true,
            ' ' => break,
            _ => {}
          }
        }
        cr
      }
      FenField::HasEp => {
        chars.next().unwrap_or('-') != '-'
      }
    }
  }

}

pub struct Board {
  b_pawn: u64, b_knight: u64, b_bishop: u64, b_rook: u64, b_queen: u64, b_king: u64,
  w_pawn: u64, w_knight: u64, w_bishop: u64, w_rook: u64, w_queen: u64, w_king: u64,
  black: u64, white: u64,
  occ: u64
}

impl Board {
  const fn new(
    bp: u64, bn: u64, bb: u64, br: u64, bq: u64, bk: u64,
    wp: u64, wn: u64, wb: u64, wr: u64, wq: u64, wk: u64,
  ) -> Self {
    let black = bp | bn | bb | br | bq | bk;
    let white = wp | wn | wb | wr | wq | wk;
    let occ = black | white;

    Board {
      b_pawn: bp, b_knight: bn, b_bishop: bb, b_rook: br, b_queen: bq, b_king: bk,
      w_pawn: wp, w_knight: wn, w_bishop: wb, w_rook: wr, w_queen: wq, w_king: wk,
      black, white,
      occ
    }
  }
}