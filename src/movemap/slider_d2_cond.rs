pub const fn get_slider_d2_cond(slider_square: u64, occupy: u64) -> u64 {
	let mut result: u64;
	match slider_square {
    0 => {
      result = 0u64;
      return result;
    }, 1 => {
      result = 256u64;
      return result;
    }, 2 => {
      result = 512u64;
      if (occupy & 512u64) == 0 { result |= 65536u64; }
      return result;
    }, 3 => {
      result = 1024u64;
      if (occupy & 1024u64) == 0 { result |= 131072u64; }
      if (occupy & 132096u64) == 0 { result |= 16777216u64; }
      return result;
    }, 4 => {
      result = 2048u64;
      if (occupy & 2048u64) == 0 { result |= 262144u64; }
      if (occupy & 264192u64) == 0 { result |= 33554432u64; }
      if (occupy & 33818624u64) == 0 { result |= 4294967296u64; }
      return result;
    }, 5 => {
      result = 4096u64;
      if (occupy & 4096u64) == 0 { result |= 524288u64; }
      if (occupy & 528384u64) == 0 { result |= 67108864u64; }
      if (occupy & 67637248u64) == 0 { result |= 8589934592u64; }
      if (occupy & 8657571840u64) == 0 { result |= 1099511627776u64; }
      return result;
    }, 6 => {
      result = 8192u64;
      if (occupy & 8192u64) == 0 { result |= 1048576u64; }
      if (occupy & 1056768u64) == 0 { result |= 134217728u64; }
      if (occupy & 135274496u64) == 0 { result |= 17179869184u64; }
      if (occupy & 17315143680u64) == 0 { result |= 2199023255552u64; }
      if (occupy & 2216338399232u64) == 0 { result |= 281474976710656u64; }
      return result;
    }, 7 => {
      result = 16384u64;
      if (occupy & 16384u64) == 0 { result |= 2097152u64; }
      if (occupy & 2113536u64) == 0 { result |= 268435456u64; }
      if (occupy & 270548992u64) == 0 { result |= 34359738368u64; }
      if (occupy & 34630287360u64) == 0 { result |= 4398046511104u64; }
      if (occupy & 4432676798464u64) == 0 { result |= 562949953421312u64; }
      if (occupy & 567382630219776u64) == 0 { result |= 72057594037927936u64; }
      return result;
    }, 8 => {
      result = 2u64;
      return result;
    }, 9 => {
      result = 65540u64;
      return result;
    }, 10 => {
      result = 131080u64;
      if (occupy & 131072u64) == 0 { result |= 16777216u64; }
      return result;
    }, 11 => {
      result = 262160u64;
      if (occupy & 262144u64) == 0 { result |= 33554432u64; }
      if (occupy & 33816576u64) == 0 { result |= 4294967296u64; }
      return result;
    }, 12 => {
      result = 524320u64;
      if (occupy & 524288u64) == 0 { result |= 67108864u64; }
      if (occupy & 67633152u64) == 0 { result |= 8589934592u64; }
      if (occupy & 8657567744u64) == 0 { result |= 1099511627776u64; }
      return result;
    }, 13 => {
      result = 1048640u64;
      if (occupy & 1048576u64) == 0 { result |= 134217728u64; }
      if (occupy & 135266304u64) == 0 { result |= 17179869184u64; }
      if (occupy & 17315135488u64) == 0 { result |= 2199023255552u64; }
      if (occupy & 2216338391040u64) == 0 { result |= 281474976710656u64; }
      return result;
    }, 14 => {
      result = 2097280u64;
      if (occupy & 2097152u64) == 0 { result |= 268435456u64; }
      if (occupy & 270532608u64) == 0 { result |= 34359738368u64; }
      if (occupy & 34630270976u64) == 0 { result |= 4398046511104u64; }
      if (occupy & 4432676782080u64) == 0 { result |= 562949953421312u64; }
      if (occupy & 567382630203392u64) == 0 { result |= 72057594037927936u64; }
      return result;
    }, 15 => {
      result = 4194304u64;
      if (occupy & 4194304u64) == 0 { result |= 536870912u64; }
      if (occupy & 541065216u64) == 0 { result |= 68719476736u64; }
      if (occupy & 69260541952u64) == 0 { result |= 8796093022208u64; }
      if (occupy & 8865353564160u64) == 0 { result |= 1125899906842624u64; }
      if (occupy & 1134765260406784u64) == 0 { result |= 144115188075855872u64; }
      return result;
    }, 16 => {
      result = 512u64;
      if (occupy & 512u64) == 0 { result |= 4u64; }
      return result;
    }, 17 => {
      result = 16778240u64;
      if (occupy & 1024u64) == 0 { result |= 8u64; }
      return result;
    }, 18 => {
      result = 33556480u64;
      if (occupy & 2048u64) == 0 { result |= 16u64; }
      if (occupy & 33554432u64) == 0 { result |= 4294967296u64; }
      return result;
    }, 19 => {
      result = 67112960u64;
      if (occupy & 4096u64) == 0 { result |= 32u64; }
      if (occupy & 67108864u64) == 0 { result |= 8589934592u64; }
      if (occupy & 8657043456u64) == 0 { result |= 1099511627776u64; }
      return result;
    }, 20 => {
      result = 134225920u64;
      if (occupy & 8192u64) == 0 { result |= 64u64; }
      if (occupy & 134217728u64) == 0 { result |= 17179869184u64; }
      if (occupy & 17314086912u64) == 0 { result |= 2199023255552u64; }
      if (occupy & 2216337342464u64) == 0 { result |= 281474976710656u64; }
      return result;
    }, 21 => {
      result = 268451840u64;
      if (occupy & 16384u64) == 0 { result |= 128u64; }
      if (occupy & 268435456u64) == 0 { result |= 34359738368u64; }
      if (occupy & 34628173824u64) == 0 { result |= 4398046511104u64; }
      if (occupy & 4432674684928u64) == 0 { result |= 562949953421312u64; }
      if (occupy & 567382628106240u64) == 0 { result |= 72057594037927936u64; }
      return result;
    }, 22 => {
      result = 536903680u64;
      if (occupy & 536870912u64) == 0 { result |= 68719476736u64; }
      if (occupy & 69256347648u64) == 0 { result |= 8796093022208u64; }
      if (occupy & 8865349369856u64) == 0 { result |= 1125899906842624u64; }
      if (occupy & 1134765256212480u64) == 0 { result |= 144115188075855872u64; }
      return result;
    }, 23 => {
      result = 1073741824u64;
      if (occupy & 1073741824u64) == 0 { result |= 137438953472u64; }
      if (occupy & 138512695296u64) == 0 { result |= 17592186044416u64; }
      if (occupy & 17730698739712u64) == 0 { result |= 2251799813685248u64; }
      if (occupy & 2269530512424960u64) == 0 { result |= 288230376151711744u64; }
      return result;
    }, 24 => {
      result = 131072u64;
      if (occupy & 131072u64) == 0 { result |= 1024u64; }
      if (occupy & 132096u64) == 0 { result |= 8u64; }
      return result;
    }, 25 => {
      result = 4295229440u64;
      if (occupy & 262144u64) == 0 { result |= 2048u64; }
      if (occupy & 264192u64) == 0 { result |= 16u64; }
      return result;
    }, 26 => {
      result = 8590458880u64;
      if (occupy & 524288u64) == 0 { result |= 4096u64; }
      if (occupy & 528384u64) == 0 { result |= 32u64; }
      if (occupy & 8589934592u64) == 0 { result |= 1099511627776u64; }
      return result;
    }, 27 => {
      result = 17180917760u64;
      if (occupy & 1048576u64) == 0 { result |= 8192u64; }
      if (occupy & 1056768u64) == 0 { result |= 64u64; }
      if (occupy & 17179869184u64) == 0 { result |= 2199023255552u64; }
      if (occupy & 2216203124736u64) == 0 { result |= 281474976710656u64; }
      return result;
    }, 28 => {
      result = 34361835520u64;
      if (occupy & 2097152u64) == 0 { result |= 16384u64; }
      if (occupy & 2113536u64) == 0 { result |= 128u64; }
      if (occupy & 34359738368u64) == 0 { result |= 4398046511104u64; }
      if (occupy & 4432406249472u64) == 0 { result |= 562949953421312u64; }
      if (occupy & 567382359670784u64) == 0 { result |= 72057594037927936u64; }
      return result;
    }, 29 => {
      result = 68723671040u64;
      if (occupy & 4194304u64) == 0 { result |= 32768u64; }
      if (occupy & 68719476736u64) == 0 { result |= 8796093022208u64; }
      if (occupy & 8864812498944u64) == 0 { result |= 1125899906842624u64; }
      if (occupy & 1134764719341568u64) == 0 { result |= 144115188075855872u64; }
      return result;
    }, 30 => {
      result = 137447342080u64;
      if (occupy & 137438953472u64) == 0 { result |= 17592186044416u64; }
      if (occupy & 17729624997888u64) == 0 { result |= 2251799813685248u64; }
      if (occupy & 2269529438683136u64) == 0 { result |= 288230376151711744u64; }
      return result;
    }, 31 => {
      result = 274877906944u64;
      if (occupy & 274877906944u64) == 0 { result |= 35184372088832u64; }
      if (occupy & 35459249995776u64) == 0 { result |= 4503599627370496u64; }
      if (occupy & 4539058877366272u64) == 0 { result |= 576460752303423488u64; }
      return result;
    }, 32 => {
      result = 33554432u64;
      if (occupy & 33554432u64) == 0 { result |= 262144u64; }
      if (occupy & 33816576u64) == 0 { result |= 2048u64; }
      if (occupy & 33818624u64) == 0 { result |= 16u64; }
      return result;
    }, 33 => {
      result = 1099578736640u64;
      if (occupy & 67108864u64) == 0 { result |= 524288u64; }
      if (occupy & 67633152u64) == 0 { result |= 4096u64; }
      if (occupy & 67637248u64) == 0 { result |= 32u64; }
      return result;
    }, 34 => {
      result = 2199157473280u64;
      if (occupy & 134217728u64) == 0 { result |= 1048576u64; }
      if (occupy & 135266304u64) == 0 { result |= 8192u64; }
      if (occupy & 135274496u64) == 0 { result |= 64u64; }
      if (occupy & 2199023255552u64) == 0 { result |= 281474976710656u64; }
      return result;
    }, 35 => {
      result = 4398314946560u64;
      if (occupy & 268435456u64) == 0 { result |= 2097152u64; }
      if (occupy & 270532608u64) == 0 { result |= 16384u64; }
      if (occupy & 270548992u64) == 0 { result |= 128u64; }
      if (occupy & 4398046511104u64) == 0 { result |= 562949953421312u64; }
      if (occupy & 567347999932416u64) == 0 { result |= 72057594037927936u64; }
      return result;
    }, 36 => {
      result = 8796629893120u64;
      if (occupy & 536870912u64) == 0 { result |= 4194304u64; }
      if (occupy & 541065216u64) == 0 { result |= 32768u64; }
      if (occupy & 8796093022208u64) == 0 { result |= 1125899906842624u64; }
      if (occupy & 1134695999864832u64) == 0 { result |= 144115188075855872u64; }
      return result;
    }, 37 => {
      result = 17593259786240u64;
      if (occupy & 1073741824u64) == 0 { result |= 8388608u64; }
      if (occupy & 17592186044416u64) == 0 { result |= 2251799813685248u64; }
      if (occupy & 2269391999729664u64) == 0 { result |= 288230376151711744u64; }
      return result;
    }, 38 => {
      result = 35186519572480u64;
      if (occupy & 35184372088832u64) == 0 { result |= 4503599627370496u64; }
      if (occupy & 4538783999459328u64) == 0 { result |= 576460752303423488u64; }
      return result;
    }, 39 => {
      result = 70368744177664u64;
      if (occupy & 70368744177664u64) == 0 { result |= 9007199254740992u64; }
      if (occupy & 9077567998918656u64) == 0 { result |= 1152921504606846976u64; }
      return result;
    }, 40 => {
      result = 8589934592u64;
      if (occupy & 8589934592u64) == 0 { result |= 67108864u64; }
      if (occupy & 8657043456u64) == 0 { result |= 524288u64; }
      if (occupy & 8657567744u64) == 0 { result |= 4096u64; }
      if (occupy & 8657571840u64) == 0 { result |= 32u64; }
      return result;
    }, 41 => {
      result = 281492156579840u64;
      if (occupy & 17179869184u64) == 0 { result |= 134217728u64; }
      if (occupy & 17314086912u64) == 0 { result |= 1048576u64; }
      if (occupy & 17315135488u64) == 0 { result |= 8192u64; }
      if (occupy & 17315143680u64) == 0 { result |= 64u64; }
      return result;
    }, 42 => {
      result = 562984313159680u64;
      if (occupy & 34359738368u64) == 0 { result |= 268435456u64; }
      if (occupy & 34628173824u64) == 0 { result |= 2097152u64; }
      if (occupy & 34630270976u64) == 0 { result |= 16384u64; }
      if (occupy & 34630287360u64) == 0 { result |= 128u64; }
      if (occupy & 562949953421312u64) == 0 { result |= 72057594037927936u64; }
      return result;
    }, 43 => {
      result = 1125968626319360u64;
      if (occupy & 68719476736u64) == 0 { result |= 536870912u64; }
      if (occupy & 69256347648u64) == 0 { result |= 4194304u64; }
      if (occupy & 69260541952u64) == 0 { result |= 32768u64; }
      if (occupy & 1125899906842624u64) == 0 { result |= 144115188075855872u64; }
      return result;
    }, 44 => {
      result = 2251937252638720u64;
      if (occupy & 137438953472u64) == 0 { result |= 1073741824u64; }
      if (occupy & 138512695296u64) == 0 { result |= 8388608u64; }
      if (occupy & 2251799813685248u64) == 0 { result |= 288230376151711744u64; }
      return result;
    }, 45 => {
      result = 4503874505277440u64;
      if (occupy & 274877906944u64) == 0 { result |= 2147483648u64; }
      if (occupy & 4503599627370496u64) == 0 { result |= 576460752303423488u64; }
      return result;
    }, 46 => {
      result = 9007749010554880u64;
      if (occupy & 9007199254740992u64) == 0 { result |= 1152921504606846976u64; }
      return result;
    }, 47 => {
      result = 18014398509481984u64;
      if (occupy & 18014398509481984u64) == 0 { result |= 2305843009213693952u64; }
      return result;
    }, 48 => {
      result = 2199023255552u64;
      if (occupy & 2199023255552u64) == 0 { result |= 17179869184u64; }
      if (occupy & 2216203124736u64) == 0 { result |= 134217728u64; }
      if (occupy & 2216337342464u64) == 0 { result |= 1048576u64; }
      if (occupy & 2216338391040u64) == 0 { result |= 8192u64; }
      if (occupy & 2216338399232u64) == 0 { result |= 64u64; }
      return result;
    }, 49 => {
      result = 72061992084439040u64;
      if (occupy & 4398046511104u64) == 0 { result |= 34359738368u64; }
      if (occupy & 4432406249472u64) == 0 { result |= 268435456u64; }
      if (occupy & 4432674684928u64) == 0 { result |= 2097152u64; }
      if (occupy & 4432676782080u64) == 0 { result |= 16384u64; }
      if (occupy & 4432676798464u64) == 0 { result |= 128u64; }
      return result;
    }, 50 => {
      result = 144123984168878080u64;
      if (occupy & 8796093022208u64) == 0 { result |= 68719476736u64; }
      if (occupy & 8864812498944u64) == 0 { result |= 536870912u64; }
      if (occupy & 8865349369856u64) == 0 { result |= 4194304u64; }
      if (occupy & 8865353564160u64) == 0 { result |= 32768u64; }
      return result;
    }, 51 => {
      result = 288247968337756160u64;
      if (occupy & 17592186044416u64) == 0 { result |= 137438953472u64; }
      if (occupy & 17729624997888u64) == 0 { result |= 1073741824u64; }
      if (occupy & 17730698739712u64) == 0 { result |= 8388608u64; }
      return result;
    }, 52 => {
      result = 576495936675512320u64;
      if (occupy & 35184372088832u64) == 0 { result |= 274877906944u64; }
      if (occupy & 35459249995776u64) == 0 { result |= 2147483648u64; }
      return result;
    }, 53 => {
      result = 1152991873351024640u64;
      if (occupy & 70368744177664u64) == 0 { result |= 549755813888u64; }
      return result;
    }, 54 => {
      result = 2305983746702049280u64;
      return result;
    }, 55 => {
      result = 4611686018427387904u64;
      return result;
    }, 56 => {
      result = 562949953421312u64;
      if (occupy & 562949953421312u64) == 0 { result |= 4398046511104u64; }
      if (occupy & 567347999932416u64) == 0 { result |= 34359738368u64; }
      if (occupy & 567382359670784u64) == 0 { result |= 268435456u64; }
      if (occupy & 567382628106240u64) == 0 { result |= 2097152u64; }
      if (occupy & 567382630203392u64) == 0 { result |= 16384u64; }
      if (occupy & 567382630219776u64) == 0 { result |= 128u64; }
      return result;
    }, 57 => {
      result = 1125899906842624u64;
      if (occupy & 1125899906842624u64) == 0 { result |= 8796093022208u64; }
      if (occupy & 1134695999864832u64) == 0 { result |= 68719476736u64; }
      if (occupy & 1134764719341568u64) == 0 { result |= 536870912u64; }
      if (occupy & 1134765256212480u64) == 0 { result |= 4194304u64; }
      if (occupy & 1134765260406784u64) == 0 { result |= 32768u64; }
      return result;
    }, 58 => {
      result = 2251799813685248u64;
      if (occupy & 2251799813685248u64) == 0 { result |= 17592186044416u64; }
      if (occupy & 2269391999729664u64) == 0 { result |= 137438953472u64; }
      if (occupy & 2269529438683136u64) == 0 { result |= 1073741824u64; }
      if (occupy & 2269530512424960u64) == 0 { result |= 8388608u64; }
      return result;
    }, 59 => {
      result = 4503599627370496u64;
      if (occupy & 4503599627370496u64) == 0 { result |= 35184372088832u64; }
      if (occupy & 4538783999459328u64) == 0 { result |= 274877906944u64; }
      if (occupy & 4539058877366272u64) == 0 { result |= 2147483648u64; }
      return result;
    }, 60 => {
      result = 9007199254740992u64;
      if (occupy & 9007199254740992u64) == 0 { result |= 70368744177664u64; }
      if (occupy & 9077567998918656u64) == 0 { result |= 549755813888u64; }
      return result;
    }, 61 => {
      result = 18014398509481984u64;
      if (occupy & 18014398509481984u64) == 0 { result |= 140737488355328u64; }
      return result;
    }, 62 => {
      result = 36028797018963968u64;
      return result;
    }, 63 => {
      result = 0u64;
      return result;
    }, _ => 0
  }
}
