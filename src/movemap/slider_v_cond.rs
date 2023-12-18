pub const fn get_slider_v_cond(slider_square: u64, occupy: u64) -> u64 {
	let mut result: u64;
	match slider_square {
    0 => {
      result = 256u64;
      if (occupy & 256u64) == 0 { result |= 65536u64; }
      if (occupy & 65792u64) == 0 { result |= 16777216u64; }
      if (occupy & 16843008u64) == 0 { result |= 4294967296u64; }
      if (occupy & 4311810304u64) == 0 { result |= 1099511627776u64; }
      if (occupy & 1103823438080u64) == 0 { result |= 281474976710656u64; }
      if (occupy & 282578800148736u64) == 0 { result |= 72057594037927936u64; }
      return result;
    }, 1 => {
      result = 512u64;
      if (occupy & 512u64) == 0 { result |= 131072u64; }
      if (occupy & 131584u64) == 0 { result |= 33554432u64; }
      if (occupy & 33686016u64) == 0 { result |= 8589934592u64; }
      if (occupy & 8623620608u64) == 0 { result |= 2199023255552u64; }
      if (occupy & 2207646876160u64) == 0 { result |= 562949953421312u64; }
      if (occupy & 565157600297472u64) == 0 { result |= 144115188075855872u64; }
      return result;
    }, 2 => {
      result = 1024u64;
      if (occupy & 1024u64) == 0 { result |= 262144u64; }
      if (occupy & 263168u64) == 0 { result |= 67108864u64; }
      if (occupy & 67372032u64) == 0 { result |= 17179869184u64; }
      if (occupy & 17247241216u64) == 0 { result |= 4398046511104u64; }
      if (occupy & 4415293752320u64) == 0 { result |= 1125899906842624u64; }
      if (occupy & 1130315200594944u64) == 0 { result |= 288230376151711744u64; }
      return result;
    }, 3 => {
      result = 2048u64;
      if (occupy & 2048u64) == 0 { result |= 524288u64; }
      if (occupy & 526336u64) == 0 { result |= 134217728u64; }
      if (occupy & 134744064u64) == 0 { result |= 34359738368u64; }
      if (occupy & 34494482432u64) == 0 { result |= 8796093022208u64; }
      if (occupy & 8830587504640u64) == 0 { result |= 2251799813685248u64; }
      if (occupy & 2260630401189888u64) == 0 { result |= 576460752303423488u64; }
      return result;
    }, 4 => {
      result = 4096u64;
      if (occupy & 4096u64) == 0 { result |= 1048576u64; }
      if (occupy & 1052672u64) == 0 { result |= 268435456u64; }
      if (occupy & 269488128u64) == 0 { result |= 68719476736u64; }
      if (occupy & 68988964864u64) == 0 { result |= 17592186044416u64; }
      if (occupy & 17661175009280u64) == 0 { result |= 4503599627370496u64; }
      if (occupy & 4521260802379776u64) == 0 { result |= 1152921504606846976u64; }
      return result;
    }, 5 => {
      result = 8192u64;
      if (occupy & 8192u64) == 0 { result |= 2097152u64; }
      if (occupy & 2105344u64) == 0 { result |= 536870912u64; }
      if (occupy & 538976256u64) == 0 { result |= 137438953472u64; }
      if (occupy & 137977929728u64) == 0 { result |= 35184372088832u64; }
      if (occupy & 35322350018560u64) == 0 { result |= 9007199254740992u64; }
      if (occupy & 9042521604759552u64) == 0 { result |= 2305843009213693952u64; }
      return result;
    }, 6 => {
      result = 16384u64;
      if (occupy & 16384u64) == 0 { result |= 4194304u64; }
      if (occupy & 4210688u64) == 0 { result |= 1073741824u64; }
      if (occupy & 1077952512u64) == 0 { result |= 274877906944u64; }
      if (occupy & 275955859456u64) == 0 { result |= 70368744177664u64; }
      if (occupy & 70644700037120u64) == 0 { result |= 18014398509481984u64; }
      if (occupy & 18085043209519104u64) == 0 { result |= 4611686018427387904u64; }
      return result;
    }, 7 => {
      result = 32768u64;
      if (occupy & 32768u64) == 0 { result |= 8388608u64; }
      if (occupy & 8421376u64) == 0 { result |= 2147483648u64; }
      if (occupy & 2155905024u64) == 0 { result |= 549755813888u64; }
      if (occupy & 551911718912u64) == 0 { result |= 140737488355328u64; }
      if (occupy & 141289400074240u64) == 0 { result |= 36028797018963968u64; }
      if (occupy & 36170086419038208u64) == 0 { result |= 9223372036854775808u64; }
      return result;
    }, 8 => {
      result = 65537u64;
      if (occupy & 65536u64) == 0 { result |= 16777216u64; }
      if (occupy & 16842752u64) == 0 { result |= 4294967296u64; }
      if (occupy & 4311810048u64) == 0 { result |= 1099511627776u64; }
      if (occupy & 1103823437824u64) == 0 { result |= 281474976710656u64; }
      if (occupy & 282578800148480u64) == 0 { result |= 72057594037927936u64; }
      return result;
    }, 9 => {
      result = 131074u64;
      if (occupy & 131072u64) == 0 { result |= 33554432u64; }
      if (occupy & 33685504u64) == 0 { result |= 8589934592u64; }
      if (occupy & 8623620096u64) == 0 { result |= 2199023255552u64; }
      if (occupy & 2207646875648u64) == 0 { result |= 562949953421312u64; }
      if (occupy & 565157600296960u64) == 0 { result |= 144115188075855872u64; }
      return result;
    }, 10 => {
      result = 262148u64;
      if (occupy & 262144u64) == 0 { result |= 67108864u64; }
      if (occupy & 67371008u64) == 0 { result |= 17179869184u64; }
      if (occupy & 17247240192u64) == 0 { result |= 4398046511104u64; }
      if (occupy & 4415293751296u64) == 0 { result |= 1125899906842624u64; }
      if (occupy & 1130315200593920u64) == 0 { result |= 288230376151711744u64; }
      return result;
    }, 11 => {
      result = 524296u64;
      if (occupy & 524288u64) == 0 { result |= 134217728u64; }
      if (occupy & 134742016u64) == 0 { result |= 34359738368u64; }
      if (occupy & 34494480384u64) == 0 { result |= 8796093022208u64; }
      if (occupy & 8830587502592u64) == 0 { result |= 2251799813685248u64; }
      if (occupy & 2260630401187840u64) == 0 { result |= 576460752303423488u64; }
      return result;
    }, 12 => {
      result = 1048592u64;
      if (occupy & 1048576u64) == 0 { result |= 268435456u64; }
      if (occupy & 269484032u64) == 0 { result |= 68719476736u64; }
      if (occupy & 68988960768u64) == 0 { result |= 17592186044416u64; }
      if (occupy & 17661175005184u64) == 0 { result |= 4503599627370496u64; }
      if (occupy & 4521260802375680u64) == 0 { result |= 1152921504606846976u64; }
      return result;
    }, 13 => {
      result = 2097184u64;
      if (occupy & 2097152u64) == 0 { result |= 536870912u64; }
      if (occupy & 538968064u64) == 0 { result |= 137438953472u64; }
      if (occupy & 137977921536u64) == 0 { result |= 35184372088832u64; }
      if (occupy & 35322350010368u64) == 0 { result |= 9007199254740992u64; }
      if (occupy & 9042521604751360u64) == 0 { result |= 2305843009213693952u64; }
      return result;
    }, 14 => {
      result = 4194368u64;
      if (occupy & 4194304u64) == 0 { result |= 1073741824u64; }
      if (occupy & 1077936128u64) == 0 { result |= 274877906944u64; }
      if (occupy & 275955843072u64) == 0 { result |= 70368744177664u64; }
      if (occupy & 70644700020736u64) == 0 { result |= 18014398509481984u64; }
      if (occupy & 18085043209502720u64) == 0 { result |= 4611686018427387904u64; }
      return result;
    }, 15 => {
      result = 8388736u64;
      if (occupy & 8388608u64) == 0 { result |= 2147483648u64; }
      if (occupy & 2155872256u64) == 0 { result |= 549755813888u64; }
      if (occupy & 551911686144u64) == 0 { result |= 140737488355328u64; }
      if (occupy & 141289400041472u64) == 0 { result |= 36028797018963968u64; }
      if (occupy & 36170086419005440u64) == 0 { result |= 9223372036854775808u64; }
      return result;
    }, 16 => {
      result = 16777472u64;
      if (occupy & 256u64) == 0 { result |= 1u64; }
      if (occupy & 16777216u64) == 0 { result |= 4294967296u64; }
      if (occupy & 4311744512u64) == 0 { result |= 1099511627776u64; }
      if (occupy & 1103823372288u64) == 0 { result |= 281474976710656u64; }
      if (occupy & 282578800082944u64) == 0 { result |= 72057594037927936u64; }
      return result;
    }, 17 => {
      result = 33554944u64;
      if (occupy & 512u64) == 0 { result |= 2u64; }
      if (occupy & 33554432u64) == 0 { result |= 8589934592u64; }
      if (occupy & 8623489024u64) == 0 { result |= 2199023255552u64; }
      if (occupy & 2207646744576u64) == 0 { result |= 562949953421312u64; }
      if (occupy & 565157600165888u64) == 0 { result |= 144115188075855872u64; }
      return result;
    }, 18 => {
      result = 67109888u64;
      if (occupy & 1024u64) == 0 { result |= 4u64; }
      if (occupy & 67108864u64) == 0 { result |= 17179869184u64; }
      if (occupy & 17246978048u64) == 0 { result |= 4398046511104u64; }
      if (occupy & 4415293489152u64) == 0 { result |= 1125899906842624u64; }
      if (occupy & 1130315200331776u64) == 0 { result |= 288230376151711744u64; }
      return result;
    }, 19 => {
      result = 134219776u64;
      if (occupy & 2048u64) == 0 { result |= 8u64; }
      if (occupy & 134217728u64) == 0 { result |= 34359738368u64; }
      if (occupy & 34493956096u64) == 0 { result |= 8796093022208u64; }
      if (occupy & 8830586978304u64) == 0 { result |= 2251799813685248u64; }
      if (occupy & 2260630400663552u64) == 0 { result |= 576460752303423488u64; }
      return result;
    }, 20 => {
      result = 268439552u64;
      if (occupy & 4096u64) == 0 { result |= 16u64; }
      if (occupy & 268435456u64) == 0 { result |= 68719476736u64; }
      if (occupy & 68987912192u64) == 0 { result |= 17592186044416u64; }
      if (occupy & 17661173956608u64) == 0 { result |= 4503599627370496u64; }
      if (occupy & 4521260801327104u64) == 0 { result |= 1152921504606846976u64; }
      return result;
    }, 21 => {
      result = 536879104u64;
      if (occupy & 8192u64) == 0 { result |= 32u64; }
      if (occupy & 536870912u64) == 0 { result |= 137438953472u64; }
      if (occupy & 137975824384u64) == 0 { result |= 35184372088832u64; }
      if (occupy & 35322347913216u64) == 0 { result |= 9007199254740992u64; }
      if (occupy & 9042521602654208u64) == 0 { result |= 2305843009213693952u64; }
      return result;
    }, 22 => {
      result = 1073758208u64;
      if (occupy & 16384u64) == 0 { result |= 64u64; }
      if (occupy & 1073741824u64) == 0 { result |= 274877906944u64; }
      if (occupy & 275951648768u64) == 0 { result |= 70368744177664u64; }
      if (occupy & 70644695826432u64) == 0 { result |= 18014398509481984u64; }
      if (occupy & 18085043205308416u64) == 0 { result |= 4611686018427387904u64; }
      return result;
    }, 23 => {
      result = 2147516416u64;
      if (occupy & 32768u64) == 0 { result |= 128u64; }
      if (occupy & 2147483648u64) == 0 { result |= 549755813888u64; }
      if (occupy & 551903297536u64) == 0 { result |= 140737488355328u64; }
      if (occupy & 141289391652864u64) == 0 { result |= 36028797018963968u64; }
      if (occupy & 36170086410616832u64) == 0 { result |= 9223372036854775808u64; }
      return result;
    }, 24 => {
      result = 4295032832u64;
      if (occupy & 65536u64) == 0 { result |= 256u64; }
      if (occupy & 65792u64) == 0 { result |= 1u64; }
      if (occupy & 4294967296u64) == 0 { result |= 1099511627776u64; }
      if (occupy & 1103806595072u64) == 0 { result |= 281474976710656u64; }
      if (occupy & 282578783305728u64) == 0 { result |= 72057594037927936u64; }
      return result;
    }, 25 => {
      result = 8590065664u64;
      if (occupy & 131072u64) == 0 { result |= 512u64; }
      if (occupy & 131584u64) == 0 { result |= 2u64; }
      if (occupy & 8589934592u64) == 0 { result |= 2199023255552u64; }
      if (occupy & 2207613190144u64) == 0 { result |= 562949953421312u64; }
      if (occupy & 565157566611456u64) == 0 { result |= 144115188075855872u64; }
      return result;
    }, 26 => {
      result = 17180131328u64;
      if (occupy & 262144u64) == 0 { result |= 1024u64; }
      if (occupy & 263168u64) == 0 { result |= 4u64; }
      if (occupy & 17179869184u64) == 0 { result |= 4398046511104u64; }
      if (occupy & 4415226380288u64) == 0 { result |= 1125899906842624u64; }
      if (occupy & 1130315133222912u64) == 0 { result |= 288230376151711744u64; }
      return result;
    }, 27 => {
      result = 34360262656u64;
      if (occupy & 524288u64) == 0 { result |= 2048u64; }
      if (occupy & 526336u64) == 0 { result |= 8u64; }
      if (occupy & 34359738368u64) == 0 { result |= 8796093022208u64; }
      if (occupy & 8830452760576u64) == 0 { result |= 2251799813685248u64; }
      if (occupy & 2260630266445824u64) == 0 { result |= 576460752303423488u64; }
      return result;
    }, 28 => {
      result = 68720525312u64;
      if (occupy & 1048576u64) == 0 { result |= 4096u64; }
      if (occupy & 1052672u64) == 0 { result |= 16u64; }
      if (occupy & 68719476736u64) == 0 { result |= 17592186044416u64; }
      if (occupy & 17660905521152u64) == 0 { result |= 4503599627370496u64; }
      if (occupy & 4521260532891648u64) == 0 { result |= 1152921504606846976u64; }
      return result;
    }, 29 => {
      result = 137441050624u64;
      if (occupy & 2097152u64) == 0 { result |= 8192u64; }
      if (occupy & 2105344u64) == 0 { result |= 32u64; }
      if (occupy & 137438953472u64) == 0 { result |= 35184372088832u64; }
      if (occupy & 35321811042304u64) == 0 { result |= 9007199254740992u64; }
      if (occupy & 9042521065783296u64) == 0 { result |= 2305843009213693952u64; }
      return result;
    }, 30 => {
      result = 274882101248u64;
      if (occupy & 4194304u64) == 0 { result |= 16384u64; }
      if (occupy & 4210688u64) == 0 { result |= 64u64; }
      if (occupy & 274877906944u64) == 0 { result |= 70368744177664u64; }
      if (occupy & 70643622084608u64) == 0 { result |= 18014398509481984u64; }
      if (occupy & 18085042131566592u64) == 0 { result |= 4611686018427387904u64; }
      return result;
    }, 31 => {
      result = 549764202496u64;
      if (occupy & 8388608u64) == 0 { result |= 32768u64; }
      if (occupy & 8421376u64) == 0 { result |= 128u64; }
      if (occupy & 549755813888u64) == 0 { result |= 140737488355328u64; }
      if (occupy & 141287244169216u64) == 0 { result |= 36028797018963968u64; }
      if (occupy & 36170084263133184u64) == 0 { result |= 9223372036854775808u64; }
      return result;
    }, 32 => {
      result = 1099528404992u64;
      if (occupy & 16777216u64) == 0 { result |= 65536u64; }
      if (occupy & 16842752u64) == 0 { result |= 256u64; }
      if (occupy & 16843008u64) == 0 { result |= 1u64; }
      if (occupy & 1099511627776u64) == 0 { result |= 281474976710656u64; }
      if (occupy & 282574488338432u64) == 0 { result |= 72057594037927936u64; }
      return result;
    }, 33 => {
      result = 2199056809984u64;
      if (occupy & 33554432u64) == 0 { result |= 131072u64; }
      if (occupy & 33685504u64) == 0 { result |= 512u64; }
      if (occupy & 33686016u64) == 0 { result |= 2u64; }
      if (occupy & 2199023255552u64) == 0 { result |= 562949953421312u64; }
      if (occupy & 565148976676864u64) == 0 { result |= 144115188075855872u64; }
      return result;
    }, 34 => {
      result = 4398113619968u64;
      if (occupy & 67108864u64) == 0 { result |= 262144u64; }
      if (occupy & 67371008u64) == 0 { result |= 1024u64; }
      if (occupy & 67372032u64) == 0 { result |= 4u64; }
      if (occupy & 4398046511104u64) == 0 { result |= 1125899906842624u64; }
      if (occupy & 1130297953353728u64) == 0 { result |= 288230376151711744u64; }
      return result;
    }, 35 => {
      result = 8796227239936u64;
      if (occupy & 134217728u64) == 0 { result |= 524288u64; }
      if (occupy & 134742016u64) == 0 { result |= 2048u64; }
      if (occupy & 134744064u64) == 0 { result |= 8u64; }
      if (occupy & 8796093022208u64) == 0 { result |= 2251799813685248u64; }
      if (occupy & 2260595906707456u64) == 0 { result |= 576460752303423488u64; }
      return result;
    }, 36 => {
      result = 17592454479872u64;
      if (occupy & 268435456u64) == 0 { result |= 1048576u64; }
      if (occupy & 269484032u64) == 0 { result |= 4096u64; }
      if (occupy & 269488128u64) == 0 { result |= 16u64; }
      if (occupy & 17592186044416u64) == 0 { result |= 4503599627370496u64; }
      if (occupy & 4521191813414912u64) == 0 { result |= 1152921504606846976u64; }
      return result;
    }, 37 => {
      result = 35184908959744u64;
      if (occupy & 536870912u64) == 0 { result |= 2097152u64; }
      if (occupy & 538968064u64) == 0 { result |= 8192u64; }
      if (occupy & 538976256u64) == 0 { result |= 32u64; }
      if (occupy & 35184372088832u64) == 0 { result |= 9007199254740992u64; }
      if (occupy & 9042383626829824u64) == 0 { result |= 2305843009213693952u64; }
      return result;
    }, 38 => {
      result = 70369817919488u64;
      if (occupy & 1073741824u64) == 0 { result |= 4194304u64; }
      if (occupy & 1077936128u64) == 0 { result |= 16384u64; }
      if (occupy & 1077952512u64) == 0 { result |= 64u64; }
      if (occupy & 70368744177664u64) == 0 { result |= 18014398509481984u64; }
      if (occupy & 18084767253659648u64) == 0 { result |= 4611686018427387904u64; }
      return result;
    }, 39 => {
      result = 140739635838976u64;
      if (occupy & 2147483648u64) == 0 { result |= 8388608u64; }
      if (occupy & 2155872256u64) == 0 { result |= 32768u64; }
      if (occupy & 2155905024u64) == 0 { result |= 128u64; }
      if (occupy & 140737488355328u64) == 0 { result |= 36028797018963968u64; }
      if (occupy & 36169534507319296u64) == 0 { result |= 9223372036854775808u64; }
      return result;
    }, 40 => {
      result = 281479271677952u64;
      if (occupy & 4294967296u64) == 0 { result |= 16777216u64; }
      if (occupy & 4311744512u64) == 0 { result |= 65536u64; }
      if (occupy & 4311810048u64) == 0 { result |= 256u64; }
      if (occupy & 4311810304u64) == 0 { result |= 1u64; }
      if (occupy & 281474976710656u64) == 0 { result |= 72057594037927936u64; }
      return result;
    }, 41 => {
      result = 562958543355904u64;
      if (occupy & 8589934592u64) == 0 { result |= 33554432u64; }
      if (occupy & 8623489024u64) == 0 { result |= 131072u64; }
      if (occupy & 8623620096u64) == 0 { result |= 512u64; }
      if (occupy & 8623620608u64) == 0 { result |= 2u64; }
      if (occupy & 562949953421312u64) == 0 { result |= 144115188075855872u64; }
      return result;
    }, 42 => {
      result = 1125917086711808u64;
      if (occupy & 17179869184u64) == 0 { result |= 67108864u64; }
      if (occupy & 17246978048u64) == 0 { result |= 262144u64; }
      if (occupy & 17247240192u64) == 0 { result |= 1024u64; }
      if (occupy & 17247241216u64) == 0 { result |= 4u64; }
      if (occupy & 1125899906842624u64) == 0 { result |= 288230376151711744u64; }
      return result;
    }, 43 => {
      result = 2251834173423616u64;
      if (occupy & 34359738368u64) == 0 { result |= 134217728u64; }
      if (occupy & 34493956096u64) == 0 { result |= 524288u64; }
      if (occupy & 34494480384u64) == 0 { result |= 2048u64; }
      if (occupy & 34494482432u64) == 0 { result |= 8u64; }
      if (occupy & 2251799813685248u64) == 0 { result |= 576460752303423488u64; }
      return result;
    }, 44 => {
      result = 4503668346847232u64;
      if (occupy & 68719476736u64) == 0 { result |= 268435456u64; }
      if (occupy & 68987912192u64) == 0 { result |= 1048576u64; }
      if (occupy & 68988960768u64) == 0 { result |= 4096u64; }
      if (occupy & 68988964864u64) == 0 { result |= 16u64; }
      if (occupy & 4503599627370496u64) == 0 { result |= 1152921504606846976u64; }
      return result;
    }, 45 => {
      result = 9007336693694464u64;
      if (occupy & 137438953472u64) == 0 { result |= 536870912u64; }
      if (occupy & 137975824384u64) == 0 { result |= 2097152u64; }
      if (occupy & 137977921536u64) == 0 { result |= 8192u64; }
      if (occupy & 137977929728u64) == 0 { result |= 32u64; }
      if (occupy & 9007199254740992u64) == 0 { result |= 2305843009213693952u64; }
      return result;
    }, 46 => {
      result = 18014673387388928u64;
      if (occupy & 274877906944u64) == 0 { result |= 1073741824u64; }
      if (occupy & 275951648768u64) == 0 { result |= 4194304u64; }
      if (occupy & 275955843072u64) == 0 { result |= 16384u64; }
      if (occupy & 275955859456u64) == 0 { result |= 64u64; }
      if (occupy & 18014398509481984u64) == 0 { result |= 4611686018427387904u64; }
      return result;
    }, 47 => {
      result = 36029346774777856u64;
      if (occupy & 549755813888u64) == 0 { result |= 2147483648u64; }
      if (occupy & 551903297536u64) == 0 { result |= 8388608u64; }
      if (occupy & 551911686144u64) == 0 { result |= 32768u64; }
      if (occupy & 551911718912u64) == 0 { result |= 128u64; }
      if (occupy & 36028797018963968u64) == 0 { result |= 9223372036854775808u64; }
      return result;
    }, 48 => {
      result = 72058693549555712u64;
      if (occupy & 1099511627776u64) == 0 { result |= 4294967296u64; }
      if (occupy & 1103806595072u64) == 0 { result |= 16777216u64; }
      if (occupy & 1103823372288u64) == 0 { result |= 65536u64; }
      if (occupy & 1103823437824u64) == 0 { result |= 256u64; }
      if (occupy & 1103823438080u64) == 0 { result |= 1u64; }
      return result;
    }, 49 => {
      result = 144117387099111424u64;
      if (occupy & 2199023255552u64) == 0 { result |= 8589934592u64; }
      if (occupy & 2207613190144u64) == 0 { result |= 33554432u64; }
      if (occupy & 2207646744576u64) == 0 { result |= 131072u64; }
      if (occupy & 2207646875648u64) == 0 { result |= 512u64; }
      if (occupy & 2207646876160u64) == 0 { result |= 2u64; }
      return result;
    }, 50 => {
      result = 288234774198222848u64;
      if (occupy & 4398046511104u64) == 0 { result |= 17179869184u64; }
      if (occupy & 4415226380288u64) == 0 { result |= 67108864u64; }
      if (occupy & 4415293489152u64) == 0 { result |= 262144u64; }
      if (occupy & 4415293751296u64) == 0 { result |= 1024u64; }
      if (occupy & 4415293752320u64) == 0 { result |= 4u64; }
      return result;
    }, 51 => {
      result = 576469548396445696u64;
      if (occupy & 8796093022208u64) == 0 { result |= 34359738368u64; }
      if (occupy & 8830452760576u64) == 0 { result |= 134217728u64; }
      if (occupy & 8830586978304u64) == 0 { result |= 524288u64; }
      if (occupy & 8830587502592u64) == 0 { result |= 2048u64; }
      if (occupy & 8830587504640u64) == 0 { result |= 8u64; }
      return result;
    }, 52 => {
      result = 1152939096792891392u64;
      if (occupy & 17592186044416u64) == 0 { result |= 68719476736u64; }
      if (occupy & 17660905521152u64) == 0 { result |= 268435456u64; }
      if (occupy & 17661173956608u64) == 0 { result |= 1048576u64; }
      if (occupy & 17661175005184u64) == 0 { result |= 4096u64; }
      if (occupy & 17661175009280u64) == 0 { result |= 16u64; }
      return result;
    }, 53 => {
      result = 2305878193585782784u64;
      if (occupy & 35184372088832u64) == 0 { result |= 137438953472u64; }
      if (occupy & 35321811042304u64) == 0 { result |= 536870912u64; }
      if (occupy & 35322347913216u64) == 0 { result |= 2097152u64; }
      if (occupy & 35322350010368u64) == 0 { result |= 8192u64; }
      if (occupy & 35322350018560u64) == 0 { result |= 32u64; }
      return result;
    }, 54 => {
      result = 4611756387171565568u64;
      if (occupy & 70368744177664u64) == 0 { result |= 274877906944u64; }
      if (occupy & 70643622084608u64) == 0 { result |= 1073741824u64; }
      if (occupy & 70644695826432u64) == 0 { result |= 4194304u64; }
      if (occupy & 70644700020736u64) == 0 { result |= 16384u64; }
      if (occupy & 70644700037120u64) == 0 { result |= 64u64; }
      return result;
    }, 55 => {
      result = 9223512774343131136u64;
      if (occupy & 140737488355328u64) == 0 { result |= 549755813888u64; }
      if (occupy & 141287244169216u64) == 0 { result |= 2147483648u64; }
      if (occupy & 141289391652864u64) == 0 { result |= 8388608u64; }
      if (occupy & 141289400041472u64) == 0 { result |= 32768u64; }
      if (occupy & 141289400074240u64) == 0 { result |= 128u64; }
      return result;
    }, 56 => {
      result = 281474976710656u64;
      if (occupy & 281474976710656u64) == 0 { result |= 1099511627776u64; }
      if (occupy & 282574488338432u64) == 0 { result |= 4294967296u64; }
      if (occupy & 282578783305728u64) == 0 { result |= 16777216u64; }
      if (occupy & 282578800082944u64) == 0 { result |= 65536u64; }
      if (occupy & 282578800148480u64) == 0 { result |= 256u64; }
      if (occupy & 282578800148736u64) == 0 { result |= 1u64; }
      return result;
    }, 57 => {
      result = 562949953421312u64;
      if (occupy & 562949953421312u64) == 0 { result |= 2199023255552u64; }
      if (occupy & 565148976676864u64) == 0 { result |= 8589934592u64; }
      if (occupy & 565157566611456u64) == 0 { result |= 33554432u64; }
      if (occupy & 565157600165888u64) == 0 { result |= 131072u64; }
      if (occupy & 565157600296960u64) == 0 { result |= 512u64; }
      if (occupy & 565157600297472u64) == 0 { result |= 2u64; }
      return result;
    }, 58 => {
      result = 1125899906842624u64;
      if (occupy & 1125899906842624u64) == 0 { result |= 4398046511104u64; }
      if (occupy & 1130297953353728u64) == 0 { result |= 17179869184u64; }
      if (occupy & 1130315133222912u64) == 0 { result |= 67108864u64; }
      if (occupy & 1130315200331776u64) == 0 { result |= 262144u64; }
      if (occupy & 1130315200593920u64) == 0 { result |= 1024u64; }
      if (occupy & 1130315200594944u64) == 0 { result |= 4u64; }
      return result;
    }, 59 => {
      result = 2251799813685248u64;
      if (occupy & 2251799813685248u64) == 0 { result |= 8796093022208u64; }
      if (occupy & 2260595906707456u64) == 0 { result |= 34359738368u64; }
      if (occupy & 2260630266445824u64) == 0 { result |= 134217728u64; }
      if (occupy & 2260630400663552u64) == 0 { result |= 524288u64; }
      if (occupy & 2260630401187840u64) == 0 { result |= 2048u64; }
      if (occupy & 2260630401189888u64) == 0 { result |= 8u64; }
      return result;
    }, 60 => {
      result = 4503599627370496u64;
      if (occupy & 4503599627370496u64) == 0 { result |= 17592186044416u64; }
      if (occupy & 4521191813414912u64) == 0 { result |= 68719476736u64; }
      if (occupy & 4521260532891648u64) == 0 { result |= 268435456u64; }
      if (occupy & 4521260801327104u64) == 0 { result |= 1048576u64; }
      if (occupy & 4521260802375680u64) == 0 { result |= 4096u64; }
      if (occupy & 4521260802379776u64) == 0 { result |= 16u64; }
      return result;
    }, 61 => {
      result = 9007199254740992u64;
      if (occupy & 9007199254740992u64) == 0 { result |= 35184372088832u64; }
      if (occupy & 9042383626829824u64) == 0 { result |= 137438953472u64; }
      if (occupy & 9042521065783296u64) == 0 { result |= 536870912u64; }
      if (occupy & 9042521602654208u64) == 0 { result |= 2097152u64; }
      if (occupy & 9042521604751360u64) == 0 { result |= 8192u64; }
      if (occupy & 9042521604759552u64) == 0 { result |= 32u64; }
      return result;
    }, 62 => {
      result = 18014398509481984u64;
      if (occupy & 18014398509481984u64) == 0 { result |= 70368744177664u64; }
      if (occupy & 18084767253659648u64) == 0 { result |= 274877906944u64; }
      if (occupy & 18085042131566592u64) == 0 { result |= 1073741824u64; }
      if (occupy & 18085043205308416u64) == 0 { result |= 4194304u64; }
      if (occupy & 18085043209502720u64) == 0 { result |= 16384u64; }
      if (occupy & 18085043209519104u64) == 0 { result |= 64u64; }
      return result;
    }, 63 => {
      result = 36028797018963968u64;
      if (occupy & 36028797018963968u64) == 0 { result |= 140737488355328u64; }
      if (occupy & 36169534507319296u64) == 0 { result |= 549755813888u64; }
      if (occupy & 36170084263133184u64) == 0 { result |= 2147483648u64; }
      if (occupy & 36170086410616832u64) == 0 { result |= 8388608u64; }
      if (occupy & 36170086419005440u64) == 0 { result |= 32768u64; }
      if (occupy & 36170086419038208u64) == 0 { result |= 128u64; }
      return result;
    }, _ => 0u64
  }
}