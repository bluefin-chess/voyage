pub const fn get_slider_d1_cond(slider_square: u64, occupy: u64) -> u64 {
	let mut result: u64;
	match slider_square {
    0 => {
      result = 512u64;
      if (occupy & 512ull) == 0 { result |= 262144u64; }
      if (occupy & 262656ull) == 0 { result |= 134217728u64; }
      if (occupy & 134480384ull) == 0 { result |= 68719476736u64; }
      if (occupy & 68853957120ull) == 0 { result |= 35184372088832u64; }
      if (occupy & 35253226045952ull) == 0 { result |= 18014398509481984u64; }
      if (occupy & 18049651735527936ull) == 0 { result |= 9223372036854775808u64; }
      return result;
    }, 1 => {
      result = 1024u64;
      if (occupy & 1024ull) == 0 { result |= 524288u64; }
      if (occupy & 525312ull) == 0 { result |= 268435456u64; }
      if (occupy & 268960768ull) == 0 { result |= 137438953472u64; }
      if (occupy & 137707914240ull) == 0 { result |= 70368744177664u64; }
      if (occupy & 70506452091904ull) == 0 { result |= 36028797018963968u64; }
      return result;
    }, 2 => {
      result = 2048u64;
      if (occupy & 2048ull) == 0 { result |= 1048576u64; }
      if (occupy & 1050624ull) == 0 { result |= 536870912u64; }
      if (occupy & 537921536ull) == 0 { result |= 274877906944u64; }
      if (occupy & 275415828480ull) == 0 { result |= 140737488355328u64; }
      return result;
    }, 3 => {
      result = 4096u64;
      if (occupy & 4096ull) == 0 { result |= 2097152u64; }
      if (occupy & 2101248ull) == 0 { result |= 1073741824u64; }
      if (occupy & 1075843072ull) == 0 { result |= 549755813888u64; }
      return result;
    }, 4 => {
      result = 8192u64;
      if (occupy & 8192ull) == 0 { result |= 4194304u64; }
      if (occupy & 4202496ull) == 0 { result |= 2147483648u64; }
      return result;
    }, 5 => {
      result = 16384u64;
      if (occupy & 16384ull) == 0 { result |= 8388608u64; }
      return result;
    }, 6 => {
      result = 32768u64;
      return result;
    }, 7 => {
      result = 0u64;
      return result;
    }, 8 => {
      result = 131072u64;
      if (occupy & 131072ull) == 0 { result |= 67108864u64; }
      if (occupy & 67239936ull) == 0 { result |= 34359738368u64; }
      if (occupy & 34426978304ull) == 0 { result |= 17592186044416u64; }
      if (occupy & 17626613022720ull) == 0 { result |= 9007199254740992u64; }
      if (occupy & 9024825867763712ull) == 0 { result |= 4611686018427387904u64; }
      return result;
    }, 9 => {
      result = 262145u64;
      if (occupy & 262144ull) == 0 { result |= 134217728u64; }
      if (occupy & 134479872ull) == 0 { result |= 68719476736u64; }
      if (occupy & 68853956608ull) == 0 { result |= 35184372088832u64; }
      if (occupy & 35253226045440ull) == 0 { result |= 18014398509481984u64; }
      if (occupy & 18049651735527424ull) == 0 { result |= 9223372036854775808u64; }
      return result;
    }, 10 => {
      result = 524290u64;
      if (occupy & 524288ull) == 0 { result |= 268435456u64; }
      if (occupy & 268959744ull) == 0 { result |= 137438953472u64; }
      if (occupy & 137707913216ull) == 0 { result |= 70368744177664u64; }
      if (occupy & 70506452090880ull) == 0 { result |= 36028797018963968u64; }
      return result;
    }, 11 => {
      result = 1048580u64;
      if (occupy & 1048576ull) == 0 { result |= 536870912u64; }
      if (occupy & 537919488ull) == 0 { result |= 274877906944u64; }
      if (occupy & 275415826432ull) == 0 { result |= 140737488355328u64; }
      return result;
    }, 12 => {
      result = 2097160u64;
      if (occupy & 2097152ull) == 0 { result |= 1073741824u64; }
      if (occupy & 1075838976ull) == 0 { result |= 549755813888u64; }
      return result;
    }, 13 => {
      result = 4194320u64;
      if (occupy & 4194304ull) == 0 { result |= 2147483648u64; }
      return result;
    }, 14 => {
      result = 8388640u64;
      return result;
    }, 15 => {
      result = 64u64;
      return result;
    }, 16 => {
      result = 33554432u64;
      if (occupy & 33554432ull) == 0 { result |= 17179869184u64; }
      if (occupy & 17213423616ull) == 0 { result |= 8796093022208u64; }
      if (occupy & 8813306445824ull) == 0 { result |= 4503599627370496u64; }
      if (occupy & 4512412933816320ull) == 0 { result |= 2305843009213693952u64; }
      return result;
    }, 17 => {
      result = 67109120u64;
      if (occupy & 67108864ull) == 0 { result |= 34359738368u64; }
      if (occupy & 34426847232ull) == 0 { result |= 17592186044416u64; }
      if (occupy & 17626612891648ull) == 0 { result |= 9007199254740992u64; }
      if (occupy & 9024825867632640ull) == 0 { result |= 4611686018427387904u64; }
      return result;
    }, 18 => {
      result = 134218240u64;
      if (occupy & 512ull) == 0 { result |= 1u64; }
      if (occupy & 134217728ull) == 0 { result |= 68719476736u64; }
      if (occupy & 68853694464ull) == 0 { result |= 35184372088832u64; }
      if (occupy & 35253225783296ull) == 0 { result |= 18014398509481984u64; }
      if (occupy & 18049651735265280ull) == 0 { result |= 9223372036854775808u64; }
      return result;
    }, 19 => {
      result = 268436480u64;
      if (occupy & 1024ull) == 0 { result |= 2u64; }
      if (occupy & 268435456ull) == 0 { result |= 137438953472u64; }
      if (occupy & 137707388928ull) == 0 { result |= 70368744177664u64; }
      if (occupy & 70506451566592ull) == 0 { result |= 36028797018963968u64; }
      return result;
    }, 20 => {
      result = 536872960u64;
      if (occupy & 2048ull) == 0 { result |= 4u64; }
      if (occupy & 536870912ull) == 0 { result |= 274877906944u64; }
      if (occupy & 275414777856ull) == 0 { result |= 140737488355328u64; }
      return result;
    }, 21 => {
      result = 1073745920u64;
      if (occupy & 4096ull) == 0 { result |= 8u64; }
      if (occupy & 1073741824ull) == 0 { result |= 549755813888u64; }
      return result;
    }, 22 => {
      result = 2147491840u64;
      if (occupy & 8192ull) == 0 { result |= 16u64; }
      return result;
    }, 23 => {
      result = 16384u64;
      if (occupy & 16384ull) == 0 { result |= 32u64; }
      return result;
    }, 24 => {
      result = 8589934592u64;
      if (occupy & 8589934592ull) == 0 { result |= 4398046511104u64; }
      if (occupy & 4406636445696ull) == 0 { result |= 2251799813685248u64; }
      if (occupy & 2256206450130944ull) == 0 { result |= 1152921504606846976u64; }
      return result;
    }, 25 => {
      result = 17179934720u64;
      if (occupy & 17179869184ull) == 0 { result |= 8796093022208u64; }
      if (occupy & 8813272891392ull) == 0 { result |= 4503599627370496u64; }
      if (occupy & 4512412900261888ull) == 0 { result |= 2305843009213693952u64; }
      return result;
    }, 26 => {
      result = 34359869440u64;
      if (occupy & 131072ull) == 0 { result |= 256u64; }
      if (occupy & 34359738368ull) == 0 { result |= 17592186044416u64; }
      if (occupy & 17626545782784ull) == 0 { result |= 9007199254740992u64; }
      if (occupy & 9024825800523776ull) == 0 { result |= 4611686018427387904u64; }
      return result;
    }, 27 => {
      result = 68719738880u64;
      if (occupy & 262144ull) == 0 { result |= 512u64; }
      if (occupy & 262656ull) == 0 { result |= 1u64; }
      if (occupy & 68719476736ull) == 0 { result |= 35184372088832u64; }
      if (occupy & 35253091565568ull) == 0 { result |= 18014398509481984u64; }
      if (occupy & 18049651601047552ull) == 0 { result |= 9223372036854775808u64; }
      return result;
    }, 28 => {
      result = 137439477760u64;
      if (occupy & 524288ull) == 0 { result |= 1024u64; }
      if (occupy & 525312ull) == 0 { result |= 2u64; }
      if (occupy & 137438953472ull) == 0 { result |= 70368744177664u64; }
      if (occupy & 70506183131136ull) == 0 { result |= 36028797018963968u64; }
      return result;
    }, 29 => {
      result = 274878955520u64;
      if (occupy & 1048576ull) == 0 { result |= 2048u64; }
      if (occupy & 1050624ull) == 0 { result |= 4u64; }
      if (occupy & 274877906944ull) == 0 { result |= 140737488355328u64; }
      return result;
    }, 30 => {
      result = 549757911040u64;
      if (occupy & 2097152ull) == 0 { result |= 4096u64; }
      if (occupy & 2101248ull) == 0 { result |= 8u64; }
      return result;
    }, 31 => {
      result = 4194304u64;
      if (occupy & 4194304ull) == 0 { result |= 8192u64; }
      if (occupy & 4202496ull) == 0 { result |= 16u64; }
      return result;
    }, 32 => {
      result = 2199023255552u64;
      if (occupy & 2199023255552ull) == 0 { result |= 1125899906842624u64; }
      if (occupy & 1128098930098176ull) == 0 { result |= 576460752303423488u64; }
      return result;
    }, 33 => {
      result = 4398063288320u64;
      if (occupy & 4398046511104ull) == 0 { result |= 2251799813685248u64; }
      if (occupy & 2256197860196352ull) == 0 { result |= 1152921504606846976u64; }
      return result;
    }, 34 => {
      result = 8796126576640u64;
      if (occupy & 33554432ull) == 0 { result |= 65536u64; }
      if (occupy & 8796093022208ull) == 0 { result |= 4503599627370496u64; }
      if (occupy & 4512395720392704ull) == 0 { result |= 2305843009213693952u64; }
      return result;
    }, 35 => {
      result = 17592253153280u64;
      if (occupy & 67108864ull) == 0 { result |= 131072u64; }
      if (occupy & 67239936ull) == 0 { result |= 256u64; }
      if (occupy & 17592186044416ull) == 0 { result |= 9007199254740992u64; }
      if (occupy & 9024791440785408ull) == 0 { result |= 4611686018427387904u64; }
      return result;
    }, 36 => {
      result = 35184506306560u64;
      if (occupy & 134217728ull) == 0 { result |= 262144u64; }
      if (occupy & 134479872ull) == 0 { result |= 512u64; }
      if (occupy & 134480384ull) == 0 { result |= 1u64; }
      if (occupy & 35184372088832ull) == 0 { result |= 18014398509481984u64; }
      if (occupy & 18049582881570816ull) == 0 { result |= 9223372036854775808u64; }
      return result;
    }, 37 => {
      result = 70369012613120u64;
      if (occupy & 268435456ull) == 0 { result |= 524288u64; }
      if (occupy & 268959744ull) == 0 { result |= 1024u64; }
      if (occupy & 268960768ull) == 0 { result |= 2u64; }
      if (occupy & 70368744177664ull) == 0 { result |= 36028797018963968u64; }
      return result;
    }, 38 => {
      result = 140738025226240u64;
      if (occupy & 536870912ull) == 0 { result |= 1048576u64; }
      if (occupy & 537919488ull) == 0 { result |= 2048u64; }
      if (occupy & 537921536ull) == 0 { result |= 4u64; }
      return result;
    }, 39 => {
      result = 1073741824u64;
      if (occupy & 1073741824ull) == 0 { result |= 2097152u64; }
      if (occupy & 1075838976ull) == 0 { result |= 4096u64; }
      if (occupy & 1075843072ull) == 0 { result |= 8u64; }
      return result;
    }, 40 => {
      result = 562949953421312u64;
      if (occupy & 562949953421312ull) == 0 { result |= 288230376151711744u64; }
      return result;
    }, 41 => {
      result = 1125904201809920u64;
      if (occupy & 1125899906842624ull) == 0 { result |= 576460752303423488u64; }
      return result;
    }, 42 => {
      result = 2251808403619840u64;
      if (occupy & 8589934592ull) == 0 { result |= 16777216u64; }
      if (occupy & 2251799813685248ull) == 0 { result |= 1152921504606846976u64; }
      return result;
    }, 43 => {
      result = 4503616807239680u64;
      if (occupy & 17179869184ull) == 0 { result |= 33554432u64; }
      if (occupy & 17213423616ull) == 0 { result |= 65536u64; }
      if (occupy & 4503599627370496ull) == 0 { result |= 2305843009213693952u64; }
      return result;
    }, 44 => {
      result = 9007233614479360u64;
      if (occupy & 34359738368ull) == 0 { result |= 67108864u64; }
      if (occupy & 34426847232ull) == 0 { result |= 131072u64; }
      if (occupy & 34426978304ull) == 0 { result |= 256u64; }
      if (occupy & 9007199254740992ull) == 0 { result |= 4611686018427387904u64; }
      return result;
    }, 45 => {
      result = 18014467228958720u64;
      if (occupy & 68719476736ull) == 0 { result |= 134217728u64; }
      if (occupy & 68853694464ull) == 0 { result |= 262144u64; }
      if (occupy & 68853956608ull) == 0 { result |= 512u64; }
      if (occupy & 68853957120ull) == 0 { result |= 1u64; }
      if (occupy & 18014398509481984ull) == 0 { result |= 9223372036854775808u64; }
      return result;
    }, 46 => {
      result = 36028934457917440u64;
      if (occupy & 137438953472ull) == 0 { result |= 268435456u64; }
      if (occupy & 137707388928ull) == 0 { result |= 524288u64; }
      if (occupy & 137707913216ull) == 0 { result |= 1024u64; }
      if (occupy & 137707914240ull) == 0 { result |= 2u64; }
      return result;
    }, 47 => {
      result = 274877906944u64;
      if (occupy & 274877906944ull) == 0 { result |= 536870912u64; }
      if (occupy & 275414777856ull) == 0 { result |= 1048576u64; }
      if (occupy & 275415826432ull) == 0 { result |= 2048u64; }
      if (occupy & 275415828480ull) == 0 { result |= 4u64; }
      return result;
    }, 48 => {
      result = 144115188075855872u64;
      return result;
    }, 49 => {
      result = 288231475663339520u64;
      return result;
    }, 50 => {
      result = 576462951326679040u64;
      if (occupy & 2199023255552ull) == 0 { result |= 4294967296u64; }
      return result;
    }, 51 => {
      result = 1152925902653358080u64;
      if (occupy & 4398046511104ull) == 0 { result |= 8589934592u64; }
      if (occupy & 4406636445696ull) == 0 { result |= 16777216u64; }
      return result;
    }, 52 => {
      result = 2305851805306716160u64;
      if (occupy & 8796093022208ull) == 0 { result |= 17179869184u64; }
      if (occupy & 8813272891392ull) == 0 { result |= 33554432u64; }
      if (occupy & 8813306445824ull) == 0 { result |= 65536u64; }
      return result;
    }, 53 => {
      result = 4611703610613432320u64;
      if (occupy & 17592186044416ull) == 0 { result |= 34359738368u64; }
      if (occupy & 17626545782784ull) == 0 { result |= 67108864u64; }
      if (occupy & 17626612891648ull) == 0 { result |= 131072u64; }
      if (occupy & 17626613022720ull) == 0 { result |= 256u64; }
      return result;
    }, 54 => {
      result = 9223407221226864640u64;
      if (occupy & 35184372088832ull) == 0 { result |= 68719476736u64; }
      if (occupy & 35253091565568ull) == 0 { result |= 134217728u64; }
      if (occupy & 35253225783296ull) == 0 { result |= 262144u64; }
      if (occupy & 35253226045440ull) == 0 { result |= 512u64; }
      if (occupy & 35253226045952ull) == 0 { result |= 1u64; }
      return result;
    }, 55 => {
      result = 70368744177664u64;
      if (occupy & 70368744177664ull) == 0 { result |= 137438953472u64; }
      if (occupy & 70506183131136ull) == 0 { result |= 268435456u64; }
      if (occupy & 70506451566592ull) == 0 { result |= 524288u64; }
      if (occupy & 70506452090880ull) == 0 { result |= 1024u64; }
      if (occupy & 70506452091904ull) == 0 { result |= 2u64; }
      return result;
    }, 56 => {
      result = 0u64;
      return result;
    }, 57 => {
      result = 281474976710656u64;
      return result;
    }, 58 => {
      result = 562949953421312u64;
      if (occupy & 562949953421312ull) == 0 { result |= 1099511627776u64; }
      return result;
    }, 59 => {
      result = 1125899906842624u64;
      if (occupy & 1125899906842624ull) == 0 { result |= 2199023255552u64; }
      if (occupy & 1128098930098176ull) == 0 { result |= 4294967296u64; }
      return result;
    }, 60 => {
      result = 2251799813685248u64;
      if (occupy & 2251799813685248ull) == 0 { result |= 4398046511104u64; }
      if (occupy & 2256197860196352ull) == 0 { result |= 8589934592u64; }
      if (occupy & 2256206450130944ull) == 0 { result |= 16777216u64; }
      return result;
    }, 61 => {
      result = 4503599627370496u64;
      if (occupy & 4503599627370496ull) == 0 { result |= 8796093022208u64; }
      if (occupy & 4512395720392704ull) == 0 { result |= 17179869184u64; }
      if (occupy & 4512412900261888ull) == 0 { result |= 33554432u64; }
      if (occupy & 4512412933816320ull) == 0 { result |= 65536u64; }
      return result;
    }, 62 => {
      result = 9007199254740992u64;
      if (occupy & 9007199254740992ull) == 0 { result |= 17592186044416u64; }
      if (occupy & 9024791440785408ull) == 0 { result |= 34359738368u64; }
      if (occupy & 9024825800523776ull) == 0 { result |= 67108864u64; }
      if (occupy & 9024825867632640ull) == 0 { result |= 131072u64; }
      if (occupy & 9024825867763712ull) == 0 { result |= 256u64; }
      return result;
    }, 63 => {
      result = 18014398509481984u64;
      if (occupy & 18014398509481984ull) == 0 { result |= 35184372088832u64; }
      if (occupy & 18049582881570816ull) == 0 { result |= 68719476736u64; }
      if (occupy & 18049651601047552ull) == 0 { result |= 134217728u64; }
      if (occupy & 18049651735265280ull) == 0 { result |= 262144u64; }
      if (occupy & 18049651735527424ull) == 0 { result |= 512u64; }
      if (occupy & 18049651735527936ull) == 0 { result |= 1u64; }
      return result;
    }, _ => 0
	}
}
