pub const fn get_slider_d1_cond(slider_square: u64, occupy: u64) -> u64 {
	let mut result: u64;
	match slider_square {
    0 => {
      result = 512u64;
      if (occupy & 512u64) == 0 { result |= 262144u64; }
      if (occupy & 262656u64) == 0 { result |= 134217728u64; }
      if (occupy & 134480384u64) == 0 { result |= 68719476736u64; }
      if (occupy & 68853957120u64) == 0 { result |= 35184372088832u64; }
      if (occupy & 35253226045952u64) == 0 { result |= 18014398509481984u64; }
      if (occupy & 18049651735527936u64) == 0 { result |= 9223372036854775808u64; }
      result
    }, 1 => {
      result = 1024u64;
      if (occupy & 1024u64) == 0 { result |= 524288u64; }
      if (occupy & 525312u64) == 0 { result |= 268435456u64; }
      if (occupy & 268960768u64) == 0 { result |= 137438953472u64; }
      if (occupy & 137707914240u64) == 0 { result |= 70368744177664u64; }
      if (occupy & 70506452091904u64) == 0 { result |= 36028797018963968u64; }
      result
    }, 2 => {
      result = 2048u64;
      if (occupy & 2048u64) == 0 { result |= 1048576u64; }
      if (occupy & 1050624u64) == 0 { result |= 536870912u64; }
      if (occupy & 537921536u64) == 0 { result |= 274877906944u64; }
      if (occupy & 275415828480u64) == 0 { result |= 140737488355328u64; }
      result
    }, 3 => {
      result = 4096u64;
      if (occupy & 4096u64) == 0 { result |= 2097152u64; }
      if (occupy & 2101248u64) == 0 { result |= 1073741824u64; }
      if (occupy & 1075843072u64) == 0 { result |= 549755813888u64; }
      result
    }, 4 => {
      result = 8192u64;
      if (occupy & 8192u64) == 0 { result |= 4194304u64; }
      if (occupy & 4202496u64) == 0 { result |= 2147483648u64; }
      result
    }, 5 => {
      result = 16384u64;
      if (occupy & 16384u64) == 0 { result |= 8388608u64; }
      result
    }, 6 => {
      result = 32768u64;
      result
    }, 7 => {
      result = 0u64;
      result
    }, 8 => {
      result = 131072u64;
      if (occupy & 131072u64) == 0 { result |= 67108864u64; }
      if (occupy & 67239936u64) == 0 { result |= 34359738368u64; }
      if (occupy & 34426978304u64) == 0 { result |= 17592186044416u64; }
      if (occupy & 17626613022720u64) == 0 { result |= 9007199254740992u64; }
      if (occupy & 9024825867763712u64) == 0 { result |= 4611686018427387904u64; }
      result
    }, 9 => {
      result = 262145u64;
      if (occupy & 262144u64) == 0 { result |= 134217728u64; }
      if (occupy & 134479872u64) == 0 { result |= 68719476736u64; }
      if (occupy & 68853956608u64) == 0 { result |= 35184372088832u64; }
      if (occupy & 35253226045440u64) == 0 { result |= 18014398509481984u64; }
      if (occupy & 18049651735527424u64) == 0 { result |= 9223372036854775808u64; }
      result
    }, 10 => {
      result = 524290u64;
      if (occupy & 524288u64) == 0 { result |= 268435456u64; }
      if (occupy & 268959744u64) == 0 { result |= 137438953472u64; }
      if (occupy & 137707913216u64) == 0 { result |= 70368744177664u64; }
      if (occupy & 70506452090880u64) == 0 { result |= 36028797018963968u64; }
      result
    }, 11 => {
      result = 1048580u64;
      if (occupy & 1048576u64) == 0 { result |= 536870912u64; }
      if (occupy & 537919488u64) == 0 { result |= 274877906944u64; }
      if (occupy & 275415826432u64) == 0 { result |= 140737488355328u64; }
      result
    }, 12 => {
      result = 2097160u64;
      if (occupy & 2097152u64) == 0 { result |= 1073741824u64; }
      if (occupy & 1075838976u64) == 0 { result |= 549755813888u64; }
      result
    }, 13 => {
      result = 4194320u64;
      if (occupy & 4194304u64) == 0 { result |= 2147483648u64; }
      result
    }, 14 => {
      result = 8388640u64;
      result
    }, 15 => {
      result = 64u64;
      result
    }, 16 => {
      result = 33554432u64;
      if (occupy & 33554432u64) == 0 { result |= 17179869184u64; }
      if (occupy & 17213423616u64) == 0 { result |= 8796093022208u64; }
      if (occupy & 8813306445824u64) == 0 { result |= 4503599627370496u64; }
      if (occupy & 4512412933816320u64) == 0 { result |= 2305843009213693952u64; }
      result
    }, 17 => {
      result = 67109120u64;
      if (occupy & 67108864u64) == 0 { result |= 34359738368u64; }
      if (occupy & 34426847232u64) == 0 { result |= 17592186044416u64; }
      if (occupy & 17626612891648u64) == 0 { result |= 9007199254740992u64; }
      if (occupy & 9024825867632640u64) == 0 { result |= 4611686018427387904u64; }
      result
    }, 18 => {
      result = 134218240u64;
      if (occupy & 512u64) == 0 { result |= 1u64; }
      if (occupy & 134217728u64) == 0 { result |= 68719476736u64; }
      if (occupy & 68853694464u64) == 0 { result |= 35184372088832u64; }
      if (occupy & 35253225783296u64) == 0 { result |= 18014398509481984u64; }
      if (occupy & 18049651735265280u64) == 0 { result |= 9223372036854775808u64; }
      result
    }, 19 => {
      result = 268436480u64;
      if (occupy & 1024u64) == 0 { result |= 2u64; }
      if (occupy & 268435456u64) == 0 { result |= 137438953472u64; }
      if (occupy & 137707388928u64) == 0 { result |= 70368744177664u64; }
      if (occupy & 70506451566592u64) == 0 { result |= 36028797018963968u64; }
      result
    }, 20 => {
      result = 536872960u64;
      if (occupy & 2048u64) == 0 { result |= 4u64; }
      if (occupy & 536870912u64) == 0 { result |= 274877906944u64; }
      if (occupy & 275414777856u64) == 0 { result |= 140737488355328u64; }
      result
    }, 21 => {
      result = 1073745920u64;
      if (occupy & 4096u64) == 0 { result |= 8u64; }
      if (occupy & 1073741824u64) == 0 { result |= 549755813888u64; }
      result
    }, 22 => {
      result = 2147491840u64;
      if (occupy & 8192u64) == 0 { result |= 16u64; }
      result
    }, 23 => {
      result = 16384u64;
      if (occupy & 16384u64) == 0 { result |= 32u64; }
      result
    }, 24 => {
      result = 8589934592u64;
      if (occupy & 8589934592u64) == 0 { result |= 4398046511104u64; }
      if (occupy & 4406636445696u64) == 0 { result |= 2251799813685248u64; }
      if (occupy & 2256206450130944u64) == 0 { result |= 1152921504606846976u64; }
      result
    }, 25 => {
      result = 17179934720u64;
      if (occupy & 17179869184u64) == 0 { result |= 8796093022208u64; }
      if (occupy & 8813272891392u64) == 0 { result |= 4503599627370496u64; }
      if (occupy & 4512412900261888u64) == 0 { result |= 2305843009213693952u64; }
      result
    }, 26 => {
      result = 34359869440u64;
      if (occupy & 131072u64) == 0 { result |= 256u64; }
      if (occupy & 34359738368u64) == 0 { result |= 17592186044416u64; }
      if (occupy & 17626545782784u64) == 0 { result |= 9007199254740992u64; }
      if (occupy & 9024825800523776u64) == 0 { result |= 4611686018427387904u64; }
      result
    }, 27 => {
      result = 68719738880u64;
      if (occupy & 262144u64) == 0 { result |= 512u64; }
      if (occupy & 262656u64) == 0 { result |= 1u64; }
      if (occupy & 68719476736u64) == 0 { result |= 35184372088832u64; }
      if (occupy & 35253091565568u64) == 0 { result |= 18014398509481984u64; }
      if (occupy & 18049651601047552u64) == 0 { result |= 9223372036854775808u64; }
      result
    }, 28 => {
      result = 137439477760u64;
      if (occupy & 524288u64) == 0 { result |= 1024u64; }
      if (occupy & 525312u64) == 0 { result |= 2u64; }
      if (occupy & 137438953472u64) == 0 { result |= 70368744177664u64; }
      if (occupy & 70506183131136u64) == 0 { result |= 36028797018963968u64; }
      result
    }, 29 => {
      result = 274878955520u64;
      if (occupy & 1048576u64) == 0 { result |= 2048u64; }
      if (occupy & 1050624u64) == 0 { result |= 4u64; }
      if (occupy & 274877906944u64) == 0 { result |= 140737488355328u64; }
      result
    }, 30 => {
      result = 549757911040u64;
      if (occupy & 2097152u64) == 0 { result |= 4096u64; }
      if (occupy & 2101248u64) == 0 { result |= 8u64; }
      result
    }, 31 => {
      result = 4194304u64;
      if (occupy & 4194304u64) == 0 { result |= 8192u64; }
      if (occupy & 4202496u64) == 0 { result |= 16u64; }
      result
    }, 32 => {
      result = 2199023255552u64;
      if (occupy & 2199023255552u64) == 0 { result |= 1125899906842624u64; }
      if (occupy & 1128098930098176u64) == 0 { result |= 576460752303423488u64; }
      result
    }, 33 => {
      result = 4398063288320u64;
      if (occupy & 4398046511104u64) == 0 { result |= 2251799813685248u64; }
      if (occupy & 2256197860196352u64) == 0 { result |= 1152921504606846976u64; }
      result
    }, 34 => {
      result = 8796126576640u64;
      if (occupy & 33554432u64) == 0 { result |= 65536u64; }
      if (occupy & 8796093022208u64) == 0 { result |= 4503599627370496u64; }
      if (occupy & 4512395720392704u64) == 0 { result |= 2305843009213693952u64; }
      result
    }, 35 => {
      result = 17592253153280u64;
      if (occupy & 67108864u64) == 0 { result |= 131072u64; }
      if (occupy & 67239936u64) == 0 { result |= 256u64; }
      if (occupy & 17592186044416u64) == 0 { result |= 9007199254740992u64; }
      if (occupy & 9024791440785408u64) == 0 { result |= 4611686018427387904u64; }
      result
    }, 36 => {
      result = 35184506306560u64;
      if (occupy & 134217728u64) == 0 { result |= 262144u64; }
      if (occupy & 134479872u64) == 0 { result |= 512u64; }
      if (occupy & 134480384u64) == 0 { result |= 1u64; }
      if (occupy & 35184372088832u64) == 0 { result |= 18014398509481984u64; }
      if (occupy & 18049582881570816u64) == 0 { result |= 9223372036854775808u64; }
      result
    }, 37 => {
      result = 70369012613120u64;
      if (occupy & 268435456u64) == 0 { result |= 524288u64; }
      if (occupy & 268959744u64) == 0 { result |= 1024u64; }
      if (occupy & 268960768u64) == 0 { result |= 2u64; }
      if (occupy & 70368744177664u64) == 0 { result |= 36028797018963968u64; }
      result
    }, 38 => {
      result = 140738025226240u64;
      if (occupy & 536870912u64) == 0 { result |= 1048576u64; }
      if (occupy & 537919488u64) == 0 { result |= 2048u64; }
      if (occupy & 537921536u64) == 0 { result |= 4u64; }
      result
    }, 39 => {
      result = 1073741824u64;
      if (occupy & 1073741824u64) == 0 { result |= 2097152u64; }
      if (occupy & 1075838976u64) == 0 { result |= 4096u64; }
      if (occupy & 1075843072u64) == 0 { result |= 8u64; }
      result
    }, 40 => {
      result = 562949953421312u64;
      if (occupy & 562949953421312u64) == 0 { result |= 288230376151711744u64; }
      result
    }, 41 => {
      result = 1125904201809920u64;
      if (occupy & 1125899906842624u64) == 0 { result |= 576460752303423488u64; }
      result
    }, 42 => {
      result = 2251808403619840u64;
      if (occupy & 8589934592u64) == 0 { result |= 16777216u64; }
      if (occupy & 2251799813685248u64) == 0 { result |= 1152921504606846976u64; }
      result
    }, 43 => {
      result = 4503616807239680u64;
      if (occupy & 17179869184u64) == 0 { result |= 33554432u64; }
      if (occupy & 17213423616u64) == 0 { result |= 65536u64; }
      if (occupy & 4503599627370496u64) == 0 { result |= 2305843009213693952u64; }
      result
    }, 44 => {
      result = 9007233614479360u64;
      if (occupy & 34359738368u64) == 0 { result |= 67108864u64; }
      if (occupy & 34426847232u64) == 0 { result |= 131072u64; }
      if (occupy & 34426978304u64) == 0 { result |= 256u64; }
      if (occupy & 9007199254740992u64) == 0 { result |= 4611686018427387904u64; }
      result
    }, 45 => {
      result = 18014467228958720u64;
      if (occupy & 68719476736u64) == 0 { result |= 134217728u64; }
      if (occupy & 68853694464u64) == 0 { result |= 262144u64; }
      if (occupy & 68853956608u64) == 0 { result |= 512u64; }
      if (occupy & 68853957120u64) == 0 { result |= 1u64; }
      if (occupy & 18014398509481984u64) == 0 { result |= 9223372036854775808u64; }
      result
    }, 46 => {
      result = 36028934457917440u64;
      if (occupy & 137438953472u64) == 0 { result |= 268435456u64; }
      if (occupy & 137707388928u64) == 0 { result |= 524288u64; }
      if (occupy & 137707913216u64) == 0 { result |= 1024u64; }
      if (occupy & 137707914240u64) == 0 { result |= 2u64; }
      result
    }, 47 => {
      result = 274877906944u64;
      if (occupy & 274877906944u64) == 0 { result |= 536870912u64; }
      if (occupy & 275414777856u64) == 0 { result |= 1048576u64; }
      if (occupy & 275415826432u64) == 0 { result |= 2048u64; }
      if (occupy & 275415828480u64) == 0 { result |= 4u64; }
      result
    }, 48 => {
      result = 144115188075855872u64;
      result
    }, 49 => {
      result = 288231475663339520u64;
      result
    }, 50 => {
      result = 576462951326679040u64;
      if (occupy & 2199023255552u64) == 0 { result |= 4294967296u64; }
      result
    }, 51 => {
      result = 1152925902653358080u64;
      if (occupy & 4398046511104u64) == 0 { result |= 8589934592u64; }
      if (occupy & 4406636445696u64) == 0 { result |= 16777216u64; }
      result
    }, 52 => {
      result = 2305851805306716160u64;
      if (occupy & 8796093022208u64) == 0 { result |= 17179869184u64; }
      if (occupy & 8813272891392u64) == 0 { result |= 33554432u64; }
      if (occupy & 8813306445824u64) == 0 { result |= 65536u64; }
      result
    }, 53 => {
      result = 4611703610613432320u64;
      if (occupy & 17592186044416u64) == 0 { result |= 34359738368u64; }
      if (occupy & 17626545782784u64) == 0 { result |= 67108864u64; }
      if (occupy & 17626612891648u64) == 0 { result |= 131072u64; }
      if (occupy & 17626613022720u64) == 0 { result |= 256u64; }
      result
    }, 54 => {
      result = 9223407221226864640u64;
      if (occupy & 35184372088832u64) == 0 { result |= 68719476736u64; }
      if (occupy & 35253091565568u64) == 0 { result |= 134217728u64; }
      if (occupy & 35253225783296u64) == 0 { result |= 262144u64; }
      if (occupy & 35253226045440u64) == 0 { result |= 512u64; }
      if (occupy & 35253226045952u64) == 0 { result |= 1u64; }
      result
    }, 55 => {
      result = 70368744177664u64;
      if (occupy & 70368744177664u64) == 0 { result |= 137438953472u64; }
      if (occupy & 70506183131136u64) == 0 { result |= 268435456u64; }
      if (occupy & 70506451566592u64) == 0 { result |= 524288u64; }
      if (occupy & 70506452090880u64) == 0 { result |= 1024u64; }
      if (occupy & 70506452091904u64) == 0 { result |= 2u64; }
      result
    }, 56 => {
      result = 0u64;
      result
    }, 57 => {
      result = 281474976710656u64;
      result
    }, 58 => {
      result = 562949953421312u64;
      if (occupy & 562949953421312u64) == 0 { result |= 1099511627776u64; }
      result
    }, 59 => {
      result = 1125899906842624u64;
      if (occupy & 1125899906842624u64) == 0 { result |= 2199023255552u64; }
      if (occupy & 1128098930098176u64) == 0 { result |= 4294967296u64; }
      result
    }, 60 => {
      result = 2251799813685248u64;
      if (occupy & 2251799813685248u64) == 0 { result |= 4398046511104u64; }
      if (occupy & 2256197860196352u64) == 0 { result |= 8589934592u64; }
      if (occupy & 2256206450130944u64) == 0 { result |= 16777216u64; }
      result
    }, 61 => {
      result = 4503599627370496u64;
      if (occupy & 4503599627370496u64) == 0 { result |= 8796093022208u64; }
      if (occupy & 4512395720392704u64) == 0 { result |= 17179869184u64; }
      if (occupy & 4512412900261888u64) == 0 { result |= 33554432u64; }
      if (occupy & 4512412933816320u64) == 0 { result |= 65536u64; }
      result
    }, 62 => {
      result = 9007199254740992u64;
      if (occupy & 9007199254740992u64) == 0 { result |= 17592186044416u64; }
      if (occupy & 9024791440785408u64) == 0 { result |= 34359738368u64; }
      if (occupy & 9024825800523776u64) == 0 { result |= 67108864u64; }
      if (occupy & 9024825867632640u64) == 0 { result |= 131072u64; }
      if (occupy & 9024825867763712u64) == 0 { result |= 256u64; }
      result
    }, 63 => {
      result = 18014398509481984u64;
      if (occupy & 18014398509481984u64) == 0 { result |= 35184372088832u64; }
      if (occupy & 18049582881570816u64) == 0 { result |= 68719476736u64; }
      if (occupy & 18049651601047552u64) == 0 { result |= 134217728u64; }
      if (occupy & 18049651735265280u64) == 0 { result |= 262144u64; }
      if (occupy & 18049651735527424u64) == 0 { result |= 512u64; }
      if (occupy & 18049651735527936u64) == 0 { result |= 1u64; }
      result
    }, _ => 0
	}
}
