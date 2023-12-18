use std::arch::asm;
mod check_between;
mod pin_between;
mod slider_h_cond;
mod slider_v_cond;
mod slider_hash;
mod slider_pext;
mod slider_d1_cond;
mod slider_d2_cond;
mod masks;
mod attacks;
mod offsets;
mod seeds;

struct SliderPext<'a> {
  attack_ptr: &'a [u64],
  mask: u64,
}

impl<'a> SliderPext<'a> {
  fn pext_u64_emulated(&self, val: u64, mask: u64) -> u64 {
    let mut res = 0;
    let mut bb = 1;
    let mut mask = mask;
    while mask != 0 {
      if val & mask & (0u64.wrapping_sub(mask)) != 0 {
        res |= bb;
      }
      mask &= mask - 1;
      bb += bb;
    }
    res
  }

  fn _pext_u64(&self, val: u64, mask: u64) -> u64 {
    let res: u64;
    unsafe {
      asm!(
        "pext {res}, {val}, {mask}",
        val = in(reg) val,
        mask = in(reg) mask,
        res = out(reg) res,
      );
    }
    res
  }

  const fn new(offset: usize, mask: u64, slider_pext: &'a [u64]) -> Self {
    Self {
      attack_ptr: &slider_pext[offset..],
      mask,
    }
  }

  const fn get(&self, blocker: u64) -> u64 {
    self.attack_ptr[self._pext_u64(blocker, self.mask) as usize] // Won't use emulated for now, I have a modern intel CPU.
  }
}

struct SliderHash<'a> {
  attack_ptr: &'a [u64],
  seed: u64,
  mask: u64,
}

impl<'a> SliderHash<'a> {
  const fn new (offset: usize, seed: u64, mask: u64, slider_hash: &'a [u64]) -> Self {
    Self {
      attack_ptr: &slider_hash[offset..],
      seed,
      mask,
    }
  }

  const fn get(&self, blocker: u64) -> u64 {
    self.attack_ptr[((self.mask | blocker).wrapping_mul(self.seed) >> (64 - 12)) as usize]
  }
}

// Slider Hash
macro_rules! init_rook_slider_hashes {
  ($($idx:expr),*) => {
    [$(
      SliderHash::new(
        offsets::ROOK_OFFSET[$idx] as usize,
        seeds::ROOK_SEED[$idx],
        !masks::RMASK[$idx],
        &slider_hash::SLIDER_HASH
      )
    ),*]
  };
}

pub static HASH_ROOK_ATTACKS: [SliderHash; 64] = init_rook_slider_hashes!(
  0, 1, 2, 3, 4, 5, 6, 7,
  8, 9, 10, 11, 12, 13, 14, 15,
  16, 17, 18, 19, 20, 21, 22, 23,
  24, 25, 26, 27, 28, 29, 30, 31,
  32, 33, 34, 35, 36, 37, 38, 39,
  40, 41, 42, 43, 44, 45, 46, 47,
  48, 49, 50, 51, 52, 53, 54, 55,
  56, 57, 58, 59, 60, 61, 62, 63
);

macro_rules! init_bishop_slider_hashes {
  ($($idx:expr),*) => {
    [$(
      SliderHash::new(
        offsets::BISHOP_OFFSET[$idx] as usize,
        seeds::BISHOP_SEED[$idx],
        !masks::BMASK[$idx],
        &slider_hash::SLIDER_HASH
      )
    ),*]
  };
}

pub static HASH_BISHOP_ATTACKS: [SliderHash; 64] = init_bishop_slider_hashes!(
  0, 1, 2, 3, 4, 5, 6, 7,
  8, 9, 10, 11, 12, 13, 14, 15,
  16, 17, 18, 19, 20, 21, 22, 23,
  24, 25, 26, 27, 28, 29, 30, 31,
  32, 33, 34, 35, 36, 37, 38, 39,
  40, 41, 42, 43, 44, 45, 46, 47,
  48, 49, 50, 51, 52, 53, 54, 55,
  56, 57, 58, 59, 60, 61, 62, 63
);

macro_rules! init_hash_rook_attacks_xray {
  ($($idx:expr),*) => {
    [$(
      SliderHash::new(
        offsets::ROOK_XRAY_OFFSET[$idx] as usize,
        seeds::ROOK_XRAY_SEED[$idx],
        !masks::RMASK[$idx],
        &slider_hash::SLIDER_HASH
      )
    ),*]
  };
}

pub static HASH_ROOK_ATTACKS_XRAY: [SliderHash; 64] = init_hash_rook_attacks_xray!(
  0, 1, 2, 3, 4, 5, 6, 7,
  8, 9, 10, 11, 12, 13, 14, 15,
  16, 17, 18, 19, 20, 21, 22, 23,
  24, 25, 26, 27, 28, 29, 30, 31,
  32, 33, 34, 35, 36, 37, 38, 39,
  40, 41, 42, 43, 44, 45, 46, 47,
  48, 49, 50, 51, 52, 53, 54, 55,
  56, 57, 58, 59, 60, 61, 62, 63
);

macro_rules! init_hash_bishop_attacks_xray {
  ($($idx:expr),*) => {
    [$(
      SliderHash::new(
        offsets::BISHOP_XRAY_OFFSET[$idx] as usize,
        seeds::BISHOP_XRAY_SEED[$idx],
        !masks::BMASK[$idx],
        &slider_hash::SLIDER_HASH
      )
    ),*]
  };
}

pub static HASH_BISHOP_ATTACKS_XRAY: [SliderHash; 64] = init_hash_bishop_attacks_xray!(
  0, 1, 2, 3, 4, 5, 6, 7,
  8, 9, 10, 11, 12, 13, 14, 15,
  16, 17, 18, 19, 20, 21, 22, 23,
  24, 25, 26, 27, 28, 29, 30, 31,
  32, 33, 34, 35, 36, 37, 38, 39,
  40, 41, 42, 43, 44, 45, 46, 47,
  48, 49, 50, 51, 52, 53, 54, 55,
  56, 57, 58, 59, 60, 61, 62, 63
);

// Slider Pext
macro_rules! init_pext_rook_attack {
  ($($idx:expr),*) => {
    [$(
      SliderPext::new(
        offsets::ROOK_OFFSET_PEXT[$idx] as usize,
        !masks::RMASK[$idx],
        &slider_pext::SLIDER_PEXT
      )
    ),*]
  };
}

pub static PEXT_ROOK_ATTACKS: [SliderPext; 64] = init_pext_rook_attack!(
  0, 1, 2, 3, 4, 5, 6, 7,
  8, 9, 10, 11, 12, 13, 14, 15,
  16, 17, 18, 19, 20, 21, 22, 23,
  24, 25, 26, 27, 28, 29, 30, 31,
  32, 33, 34, 35, 36, 37, 38, 39,
  40, 41, 42, 43, 44, 45, 46, 47,
  48, 49, 50, 51, 52, 53, 54, 55,
  56, 57, 58, 59, 60, 61, 62, 63
);

macro_rules! init_pext_bishop_attack {
  ($($idx:expr),*) => {
    [$(
      SliderPext::new(
        offsets::BISHOP_OFFSET_PEXT[$idx] as usize,
        !masks::BMASK[$idx],
        &slider_pext::SLIDER_PEXT
      )
    ),*]
  };
}

pub static PEXT_BISHOP_ATTACKS: [SliderPext; 64] = init_pext_bishop_attack!(
  0, 1, 2, 3, 4, 5, 6, 7,
  8, 9, 10, 11, 12, 13, 14, 15,
  16, 17, 18, 19, 20, 21, 22, 23,
  24, 25, 26, 27, 28, 29, 30, 31,
  32, 33, 34, 35, 36, 37, 38, 39,
  40, 41, 42, 43, 44, 45, 46, 47,
  48, 49, 50, 51, 52, 53, 54, 55,
  56, 57, 58, 59, 60, 61, 62, 63
);

macro_rules! init_pext_rook_attacks_xray {
  ($($idx:expr),*) => {
    [$(
      SliderPext::new(
        offsets::ROOK_XRAY_OFFSET_PEXT[$idx] as usize,
        !masks::RMASK[$idx],
        &slider_pext::SLIDER_PEXT
      )
    ),*]
  };
}

pub static PEXT_ROOK_ATTACKS_XRAY: [SliderPext; 64] = init_pext_rook_attacks_xray!(
  0, 1, 2, 3, 4, 5, 6, 7,
  8, 9, 10, 11, 12, 13, 14, 15,
  16, 17, 18, 19, 20, 21, 22, 23,
  24, 25, 26, 27, 28, 29, 30, 31,
  32, 33, 34, 35, 36, 37, 38, 39,
  40, 41, 42, 43, 44, 45, 46, 47,
  48, 49, 50, 51, 52, 53, 54, 55,
  56, 57, 58, 59, 60, 61, 62, 63
); 

macro_rules! init_pext_bishop_attacks_xray {
  ($($idx:expr),*) => {
    [$(
      SliderPext::new(
        offsets::BISHOP_XRAY_OFFSET_PEXT[$idx] as usize,
        !masks::BMASK[$idx],
        &slider_pext::SLIDER_PEXT
      )
    ),*]
  };
}

pub static PEXT_BISHOP_ATTACKS_XRAY: [SliderPext; 64] = init_pext_bishop_attacks_xray!(
  0, 1, 2, 3, 4, 5, 6, 7,
  8, 9, 10, 11, 12, 13, 14, 15,
  16, 17, 18, 19, 20, 21, 22, 23,
  24, 25, 26, 27, 28, 29, 30, 31,
  32, 33, 34, 35, 36, 37, 38, 39,
  40, 41, 42, 43, 44, 45, 46, 47,
  48, 49, 50, 51, 52, 53, 54, 55,
  56, 57, 58, 59, 60, 61, 62, 63
);

mod lookup_switch {
  use super::{attacks, slider_h_cond::get_slider_h_cond, slider_d1_cond::get_slider_d1_cond, slider_d2_cond::get_slider_d2_cond};

  const fn king(square: u64) -> u64 {
    attacks::KING_ATTACKS[square as usize]
  }

  const fn knight(square: u64) -> u64 {
    attacks::KNIGHT_ATTACKS[square as usize]
  }

  const fn rook(square: u64, occ: u64) -> u64 {
    get_slider_h_cond(square, occ) | get_slider_h_cond(square, occ)
  }

  const fn rook_xray(square: u64, occ: u64) -> u64 {
    let attacks = rook(square, occ);
    attacks | get_slider_h_cond(square, attacks)
  }

  const fn bishop(square: u64, occ: u64) -> u64 {
    get_slider_d1_cond(square, occ) | get_slider_d2_cond(square, occ)
  }

  const fn bishop_xray(square: u64, occ: u64) -> u64 {
    let attacks = bishop(square, occ);
    attacks | get_slider_d1_cond(square, attacks) | get_slider_d2_cond(square, attacks)
  }

  const fn queen(square: u64, occ: u64) -> u64 {
    rook(square, occ) | bishop(square, occ)
  }

  const fn queen_xray(square: u64, occ: u64) -> u64 {
    rook_xray(square, occ) | bishop_xray(square, occ)
  }
}

mod lookup_hash {
  const fn king(square: u64) -> u64 {
    super::attacks::KING_ATTACKS[square as usize]
  }

  const fn knight(square: u64) -> u64 {
    super::attacks::KNIGHT_ATTACKS[square as usize]
  }

  const fn rook(square: u64, occ: u64) -> u64 {
    super::HASH_ROOK_ATTACKS[square as usize].get(occ)
  }

  const fn rook_xray(square: u64, occ: u64) -> u64 {
    super::HASH_ROOK_ATTACKS_XRAY[square as usize].get(occ)
  }

  const fn bishop(square: u64, occ: u64) -> u64 {
    super::HASH_BISHOP_ATTACKS[square as usize].get(occ)
  }

  const fn bishop_xray(square: u64, occ: u64) -> u64 {
    super::HASH_BISHOP_ATTACKS_XRAY[square as usize].get(occ)
  }

  const fn queen(square: u64, occ: u64) -> u64 {
    rook(square, occ) | bishop(square, occ)
  }

  const fn queen_xray(square: u64, occ: u64) -> u64 {
    rook_xray(square, occ) | bishop_xray(square, occ)
  }
}

mod lookup_pext {
  const fn king(square: u64) -> u64 {
    super::attacks::KING_ATTACKS[square as usize]
  }

  const fn knight(square: u64) -> u64 {
    super::attacks::KNIGHT_ATTACKS[square as usize]
  }

  const fn rook(square: u64, occ: u64) -> u64 {
    super::PEXT_ROOK_ATTACKS[square as usize].get(occ)
  }

  const fn rook_xray(square: u64, occ: u64) -> u64 {
    super::PEXT_ROOK_ATTACKS_XRAY[square as usize].get(occ)
  }

  const fn bishop(square: u64, occ: u64) -> u64 {
    super::PEXT_BISHOP_ATTACKS[square as usize].get(occ)
  }

  const fn bishop_xray(square: u64, occ: u64) -> u64 {
    super::PEXT_BISHOP_ATTACKS_XRAY[square as usize].get(occ)
  }

  const fn queen(square: u64, occ: u64) -> u64 {
    rook(square, occ) | bishop(square, occ)
  }

  const fn queen_xray(square: u64, occ: u64) -> u64 {
    rook_xray(square, occ) | bishop_xray(square, occ)
  }
}