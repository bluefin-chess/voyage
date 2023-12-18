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