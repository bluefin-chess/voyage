#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
// use crate::movemap::lookup_hash; // uses multiply look up
// use crate::movemap::lookup_switch; // uses ifchain to calculate seen squares
use crate::movemap::lookup_pext as lookup; // Fastest on hardware pext CPUs, Ryzen 5000+ & Intel
use crate::movegen;
type Bit = u64;
type Square = u64;
type Map = u64;
use std::arch::x86_64::{_tzcnt_u64, _blsr_u64, _popcnt64};
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
      rook == Self::W_ROOK_L
    } else {
      rook == Self::B_ROOK_L
    }
  }

  const fn is_right_rook(&self, rook: Bit) -> bool {
    if self.white_move {
      rook == Self::W_ROOK_R
    } else {
      rook == Self::B_ROOK_R
    }
  }

  // Enable en passant for all pawn moves, part of pseudolegal generation.
  const fn pawn_push(&self) -> BoardStatus {
    Self::new(!self.white_move, true, self.w_castle_l, self.w_castle_r, self.b_castle_l, self.b_castle_r)
  }

  // Disable castling rights for whoever's king moved.F
  const fn king_move(&self) -> BoardStatus {
    if self.white_move {
      Self::new(!self.white_move, false, false, false, self.b_castle_l, self.b_castle_r)
    } else {
      Self::new(!self.white_move, false, self.w_castle_l, self.w_castle_r, false, false)
    }
  }

  // Disable left castling rights for mover.
  const fn rook_move_left(&self) -> BoardStatus {
    if self.white_move {
      Self::new(!self.white_move, false, false, self.w_castle_r, self.b_castle_l, self.b_castle_r)
    } else {
      Self::new(!self.white_move, false, self.w_castle_l, self.w_castle_r, false, self.b_castle_r)
    }
  }

  // Disable right castling rights for mover.
  const fn rook_move_right(&self) -> BoardStatus {
    if self.white_move {
      Self::new(!self.white_move, false, self.w_castle_l, false, self.b_castle_l, self.b_castle_r)
    } else {
      Self::new(!self.white_move, false, self.w_castle_l, self.w_castle_r, self.b_castle_l, false)
    }
  }

  // Non rook or king move, no castling rights lost.
  const fn silent_move(&self) -> BoardStatus {
    Self::new(!self.white_move, false, self.w_castle_l, self.w_castle_r, self.b_castle_l, self.b_castle_r)
  }

  // Default state: white to move, not in check and all castling rights + not en passant.
  const fn default(&self) -> BoardStatus {
    Self::new(true, false, true, true, true, true)
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

pub struct Fen;

impl Fen {
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
      let file_offset = (b'h' - e_or_minus as u8) as u64;
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
        for c in chars.by_ref() {
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

  // Transform Fen character 'n' or 'Q' into bitmap where the bits correspond to the field
  fn fen_to_bmp(fen: &str, p: char) -> u64 {
    let mut field: u64 = 63;

    let mut res: u64 = 0;
    while let Some(c) = fen.chars().next() {
      match c {
        ' ' => break,
        '/' => {
          break;
        }
        '1'..='8' => {
          field -= c.to_digit(10).unwrap() as u64 - 1; // field = field - (c - 1)
        }
        _ => {
          if c == p {
            res |= 1u64 << field;
          }
          field -= 1;
        }
      }
    }
    res
  }

}

enum BoardPiece {
  Pawn, Knight, Bishop, Rook, Queen, King
}

pub struct Board {
  pub b_pawn: u64, pub b_knight: u64, pub b_bishop: u64, pub b_rook: u64, pub b_queen: u64, pub b_king: u64,
  pub w_pawn: u64, pub w_knight: u64, pub w_bishop: u64, pub w_rook: u64, pub w_queen: u64, pub w_king: u64,
  pub black: u64, pub white: u64,
  pub occ: u64
}

impl Board {
  #[allow(clippy::too_many_arguments)]
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

  fn from_fen(fen: &str) -> Self {
    Self::new(
      Fen::fen_to_bmp(fen, 'p'), Fen::fen_to_bmp(fen, 'n'), Fen::fen_to_bmp(fen, 'b'), Fen::fen_to_bmp(fen, 'r'),
      Fen::fen_to_bmp(fen, 'q'), Fen::fen_to_bmp(fen, 'k'),
      Fen::fen_to_bmp(fen, 'P'), Fen::fen_to_bmp(fen, 'N'), Fen::fen_to_bmp(fen, 'B'), Fen::fen_to_bmp(fen, 'R'),
      Fen::fen_to_bmp(fen, 'Q'), Fen::fen_to_bmp(fen, 'K'),
    )
  }

  // update board based on promotion
  fn move_promote(piece: BoardPiece, is_white: bool, board: Board, from: u64, to: u64) -> Self {
    let rem: u64 = !to;
    let (bp, bn, bb, br, bq, bk, wp, wn, wb, wr, wq, wk) = (
      board.b_pawn, board.b_knight, board.b_bishop, board.b_rook, board.b_queen, board.b_king,
      board.w_pawn, board.w_knight, board.w_bishop, board.w_rook, board.w_queen, board.w_king
    );

    if is_white {
      match piece {
        BoardPiece::Queen => {
          Self::new(bp & rem, bn & rem, bb & rem, br & rem, bq & rem, bk,
            wp ^ from, wn, wb, wr, wq ^ to, wk)
        }
        BoardPiece::Rook => {
          Self::new(bp & rem, bn & rem, bb & rem, br & rem, bq & rem, bk,
            wp ^ from, wn, wb, wr ^ to, wq, wk)
        }
        BoardPiece::Bishop => {
          Self::new(bp & rem, bn & rem, bb & rem, br & rem, bq & rem, bk,
            wp ^ from, wn, wb ^ to, wr, wq, wk)
        }
        BoardPiece::Knight => {
          Self::new(bp & rem, bn & rem, bb & rem, br & rem, bq & rem, bk,
            wp ^ from, wn ^ to, wb, wr, wq, wk)
        }
        _ => Self::new(bp, bn, bb, br, bq, bk, wp, wn, wb, wr, wq, wk)
      }
    } else {
      match piece {
        BoardPiece::Queen => {
          Self::new(bp ^ from, bn, bb, br, bq ^ to, bk,
            wp & rem, wn & rem, wb & rem, wr & rem, wq & rem, wk)
        }
        BoardPiece::Rook => {
          Self::new(bp ^ from, bn, bb, br ^ to, bq, bk,
            wp & rem, wn & rem, wb & rem, wr & rem, wq & rem, wk)
        }
        BoardPiece::Bishop => {
          Self::new(bp ^ from, bn, bb ^ to, br, bq, bk,
            wp & rem, wn & rem, wb & rem, wr & rem, wq & rem, wk)
        }
        BoardPiece::Knight => {
          Self::new(bp ^ from, bn ^ to, bb, br, bq, bk,
            wp & rem, wn & rem, wb & rem, wr & rem, wq & rem, wk)
        }
        _=> Self::new(bp, bn, bb, br, bq, bk, wp, wn, wb, wr, wq, wk)
      }
    }
  }

  fn move_castle(board: Board, is_white: bool, king: u64, rook: u64) -> Self {
    let w_king = board.w_king ^ (if is_white { king } else { 0 });
    let w_rook = board.w_rook ^ (if is_white { rook } else { 0 });
    let b_king = board.b_king ^ (if !is_white { king } else { 0 });
    let b_rook = board.b_rook ^ (if !is_white { rook } else { 0 });
    Self::new(board.b_pawn, board.b_knight, board.b_bishop, b_rook, board.b_queen, b_king,
      board.w_pawn, board.w_knight, board.w_bishop, w_rook, board.w_queen, w_king)
  }

  fn move_enpassant(board: Board, is_white: bool, from: u64, to: u64, enemy: u64) -> Self {
    let rem: u64 = !enemy;

    let (bp, bn, bb, br, bq, bk, wp, wn, wb, wr, wq, wk) = (
      board.b_pawn, board.b_knight, board.b_bishop, board.b_rook, board.b_queen, board.b_king,
      board.w_pawn, board.w_knight, board.w_bishop, board.w_rook, board.w_queen, board.w_king
    );

    let mov = from | to;

    if is_white {
      Self::new(bp & rem, bn & rem, bb & rem, br & rem, bq & rem, bk,
        wp ^ mov, wn, wb, wr, wq, wk)
    } else {
      Self::new(bp ^ mov, bn, bb, br, bq, bk,
        wp & rem, wn & rem, wb & rem, wr & rem, wq & rem, wk)
    }
  }



  fn assert_board_move(before: &Board, after: &Board, is_taking: bool, is_white: bool) {
    fn blackcount(board: &Board) -> i32 {
      unsafe { _popcnt64(board.black as i64) }
    }

    fn whitecount(board: &Board) -> i32 {
      unsafe { _popcnt64(board.white as i64) }
    }

    fn allcount(board: &Board) -> i32 {
      unsafe { _popcnt64(board.occ as i64) }
    }

    debug_assert!(blackcount(before) + whitecount(before) == allcount(before), "Some squares are taken twice.");
    debug_assert!(blackcount(after) + whitecount(after) == allcount(after), "Some squares are taken twice after a move.");
    if is_taking {
      debug_assert!(allcount(before) == allcount(after) + 1, "A piece was taken but not removed.");
    } else {
      debug_assert!(allcount(before) == allcount(after), "Piece count did not say the same after a non-capturing move.");
    }
    debug_assert!(after.b_king != 0, "Black king is missing.");
    debug_assert!(after.w_king != 0, "White king is missing.");
  
    // TODO: check that king not in check after move
    let kingpos = unsafe { _tzcnt_u64(if is_white { after.w_king } else { after.b_king }) };

    let diagonal_check = lookup::bishop(kingpos, after.occ) & movegen::enemy_bishop_queen(after, is_white);
    let straight_check = lookup::rook(kingpos, after.occ) & movegen::enemy_rook_queen(after, is_white);
    let knight_check = lookup::knight(kingpos) & movegen::knights(after, is_white);

    debug_assert!(diagonal_check == 0, "In check by bishop/queen after move.");
    debug_assert!(straight_check == 0, "In check by rook/queen after move.");
    debug_assert!(knight_check == 0, "In check by knight after move.");
  }

  fn move_piece(piece: BoardPiece, board: &Board, is_white: bool, is_taking: bool, from: u64, to: u64) -> Board {
    let (bp, bn, bb, br, bq, bk, wp, wn, wb, wr, wq, wk) = (
      board.b_pawn, board.b_knight, board.b_bishop, board.b_rook, board.b_queen, board.b_king,
      board.w_pawn, board.w_knight, board.w_bishop, board.w_rook, board.w_queen, board.w_king
    );

    let mov = from | to;

    if is_taking {
      let rem = !to;
      if is_white {
        debug_assert!((bk & mov) == 0, "Taking the black king is not legal.");
        debug_assert!((to & board.white) == 0, "Cannot move to square with same white color piece.");
        match piece {
          BoardPiece::Pawn => { Board::new(bp & rem, bn & rem, bb & rem, br & rem, bq & rem, bk, wp ^ mov, wn, wb, wr, wq, wk) }
          BoardPiece::Knight => { Board::new(bp, bn & rem, bb & rem, br & rem, bq & rem, bk, wp, wn & mov, wb, wr, wq, wk) }
          BoardPiece::Bishop => { Board::new(bp, bn, bb & rem, br & rem, bq & rem, bk, wp, wn, wb & mov, wr, wq, wk) }
          BoardPiece::Rook => { Board::new(bp, bn, bb, br & rem, bq & rem, bk, wp, wn, wb, wr & mov, wq, wk) }
          BoardPiece::Queen => { Board::new(bp, bn, bb, br, bq & rem, bk, wp, wn, wb, wr, wq & mov, wk) }
          BoardPiece::King => { Board::new(bp, bn, bb, br, bq, bk & rem, wp, wn, wb, wr, wq, wk) }
        }
      } else {
        debug_assert!((wk & mov) == 0, "Taking the white king is not legal.");
        debug_assert!((to & board.black) == 0, "Cannot move to square with same black color piece.");
        match piece {
          BoardPiece::Pawn => { Board::new(bp, bn & rem, bb & rem, br & rem, bq & rem, bk, wp & rem, wn, wb, wr, wq, wk ^ mov) }
          BoardPiece::Knight => { Board::new(bp, bn & rem, bb & rem, br & rem, bq & rem, bk, wp, wn & mov, wb, wr, wq, wk) }
          BoardPiece::Bishop => { Board::new(bp, bn, bb & rem, br & rem, bq & rem, bk, wp, wn, wb & mov, wr, wq, wk) }
          BoardPiece::Rook => { Board::new(bp, bn, bb, br & rem, bq & rem, bk, wp, wn, wb, wr & mov, wq, wk) }
          BoardPiece::Queen => { Board::new(bp, bn, bb, br, bq & rem, bk, wp, wn, wb, wr, wq & mov, wk) }
          BoardPiece::King => { Board::new(bp, bn, bb, br, bq, bk & rem, wp, wn, wb, wr, wq, wk ^ mov) }
        }
      }
    } else {
      #[allow(clippy::if_same_then_else)]
      if is_white {
        debug_assert!((bk & mov) == 0, "Moving the black king is not legal.");
        debug_assert!((to & board.white) == 0, "Cannot move to square with same white color piece.");
        match piece {
          BoardPiece::Pawn => { Board::new(bp, bn, bb, br, bq, bk, wp ^ mov, wn, wb, wr, wq, wk) }
          BoardPiece::Knight => { Board::new(bp, bn, bb, br, bq, bk, wp, wn ^ mov, wb, wr, wq, wk) }
          BoardPiece::Bishop => { Board::new(bp, bn, bb, br, bq, bk, wp, wn, wb ^ mov, wr, wq, wk) }
          BoardPiece::Rook => { Board::new(bp, bn, bb, br, bq, bk, wp, wn, wb, wr ^ mov, wq, wk) }
          BoardPiece::Queen => { Board::new(bp, bn, bb, br, bq, bk, wp, wn, wb, wr, wq ^ mov, wk) }
          BoardPiece::King => { Board::new(bp, bn, bb, br, bq, bk, wp, wn, wb, wr, wq, wk ^ mov) }
        }
      } else {
        debug_assert!((wk & mov) == 0, "Moving the white king is not legal.");
        debug_assert!((to & board.black) == 0, "Cannot move to square with same black color piece.");
        match piece {
          BoardPiece::Pawn => { Board::new(bp ^ mov, bn, bb, br, bq, bk, wp, wn, wb, wr, wq, wk) }
          BoardPiece::Knight => { Board::new(bp, bn ^ mov, bb, br, bq, bk, wp, wn, wb, wr, wq, wk) }
          BoardPiece::Bishop => { Board::new(bp, bn, bb ^ mov, br, bq, bk, wp, wn, wb, wr, wq, wk) }
          BoardPiece::Rook => { Board::new(bp, bn, bb, br ^ mov, bq, bk, wp, wn, wb, wr, wq, wk) }
          BoardPiece::Queen => { Board::new(bp, bn, bb, br, bq ^ mov, bk, wp, wn, wb, wr, wq, wk) }
          BoardPiece::King => { Board::new(bp, bn, bb, br, bq, bk ^ mov, wp, wn, wb, wr, wq, wk) }
        }
      }
    }
  }

  fn default() -> Self {
    Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
  }
}

const fn map_bit_to_char(bit: u64, board: &Board) -> char {
  if (bit & board.b_pawn) != 0 { return 'p'; }
  if (bit & board.b_knight) != 0 { return 'n'; }
  if (bit & board.b_bishop) != 0 { return 'b'; }
  if (bit & board.b_rook) != 0 { return 'r'; }
  if (bit & board.b_queen) != 0 { return 'q'; }
  if (bit & board.b_king) != 0 { return 'k'; }
  if (bit & board.w_pawn) != 0 { return 'P'; }
  if (bit & board.w_knight) != 0 { return 'N'; }
  if (bit & board.w_bishop) != 0 { return 'B'; }
  if (bit & board.w_rook) != 0 { return 'R'; }
  if (bit & board.w_queen) != 0 { return 'Q'; }
  if (bit & board.w_king) != 0 { return 'K'; }
  '.'
}

fn map(val1: u64, val2: u64, val3: &Board, val4: &Board) -> String {
  let mut board = vec!['o'; 64 * 3 + 3 * 8];

  let mut c = 0;
  for i in 0..64 {
    let bitmask: u64 = (1u64 << 63) >> i;

    board[c] = if (bitmask & val1) != 0 { 'X' } else { '.' };
    board[c + 9] = if (bitmask & val2) != 0 { 'X' } else { '.' };
    board[c + 18] = map_bit_to_char(bitmask, val3);
    board[c + 27] = map_bit_to_char(bitmask, val4);

    c += 1;

    if (i + 1) % 8 == 0 {
      board[c] = ' ';
      board[c + 9] = ' ';
      board[c + 18] = ' ';
      board[c + 27] = '\n';
      c += 28;
    }
  }
  board.iter().collect::<String>()
}
