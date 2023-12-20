pub const fn get_slider_h_cond(slider_square: u64, occupy: u64) -> u64 {  
  let mut result: u64;
  match slider_square {
    0 => {
      result = 2u64;
      if (occupy & 2u64) == 0 { result |= 4u64; }
      if (occupy & 6u64) == 0 { result |= 8u64; }
      if (occupy & 14u64) == 0 { result |= 16u64; }
      if (occupy & 30u64) == 0 { result |= 32u64; }
      if (occupy & 62u64) == 0 { result |= 64u64; }
      if (occupy & 126u64) == 0 { result |= 128u64; }
      result
    },  1 => {
      result = 5u64;
      if (occupy & 4u64) == 0 { result |= 8u64; }
      if (occupy & 12u64) == 0 { result |= 16u64; }
      if (occupy & 28u64) == 0 { result |= 32u64; }
      if (occupy & 60u64) == 0 { result |= 64u64; }
      if (occupy & 124u64) == 0 { result |= 128u64; }
      result
    },  2 => {
      result = 10u64;
      if (occupy & 2u64) == 0 { result |= 1u64; }
      if (occupy & 8u64) == 0 { result |= 16u64; }
      if (occupy & 24u64) == 0 { result |= 32u64; }
      if (occupy & 56u64) == 0 { result |= 64u64; }
      if (occupy & 120u64) == 0 { result |= 128u64; }
      result
    },  3 => {
      result = 20u64;
      if (occupy & 4u64) == 0 { result |= 2u64; }
      if (occupy & 6u64) == 0 { result |= 1u64; }
      if (occupy & 16u64) == 0 { result |= 32u64; }
      if (occupy & 48u64) == 0 { result |= 64u64; }
      if (occupy & 112u64) == 0 { result |= 128u64; }
      result
    },  4 => {
      result = 40u64;
      if (occupy & 8u64) == 0 { result |= 4u64; }
      if (occupy & 12u64) == 0 { result |= 2u64; }
      if (occupy & 14u64) == 0 { result |= 1u64; }
      if (occupy & 32u64) == 0 { result |= 64u64; }
      if (occupy & 96u64) == 0 { result |= 128u64; }
      result
    },  5 => {
      result = 80u64;
      if (occupy & 16u64) == 0 { result |= 8u64; }
      if (occupy & 24u64) == 0 { result |= 4u64; }
      if (occupy & 28u64) == 0 { result |= 2u64; }
      if (occupy & 30u64) == 0 { result |= 1u64; }
      if (occupy & 64u64) == 0 { result |= 128u64; }
      result
    },  6 => {
      result = 160u64;
      if (occupy & 32u64) == 0 { result |= 16u64; }
      if (occupy & 48u64) == 0 { result |= 8u64; }
      if (occupy & 56u64) == 0 { result |= 4u64; }
      if (occupy & 60u64) == 0 { result |= 2u64; }
      if (occupy & 62u64) == 0 { result |= 1u64; }
      result
    },  7 => {
      result = 64u64;
      if (occupy & 64u64) == 0 { result |= 32u64; }
      if (occupy & 96u64) == 0 { result |= 16u64; }
      if (occupy & 112u64) == 0 { result |= 8u64; }
      if (occupy & 120u64) == 0 { result |= 4u64; }
      if (occupy & 124u64) == 0 { result |= 2u64; }
      if (occupy & 126u64) == 0 { result |= 1u64; }
      result
    },  8 => {
      result = 512u64;
      if (occupy & 512u64) == 0 { result |= 1024u64; }
      if (occupy & 1536u64) == 0 { result |= 2048u64; }
      if (occupy & 3584u64) == 0 { result |= 4096u64; }
      if (occupy & 7680u64) == 0 { result |= 8192u64; }
      if (occupy & 15872u64) == 0 { result |= 16384u64; }
      if (occupy & 32256u64) == 0 { result |= 32768u64; }
      result
    },  9 => {
      result = 1280u64;
      if (occupy & 1024u64) == 0 { result |= 2048u64; }
      if (occupy & 3072u64) == 0 { result |= 4096u64; }
      if (occupy & 7168u64) == 0 { result |= 8192u64; }
      if (occupy & 15360u64) == 0 { result |= 16384u64; }
      if (occupy & 31744u64) == 0 { result |= 32768u64; }
      result
    },  10 => {
      result = 2560u64;
      if (occupy & 512u64) == 0 { result |= 256u64; }
      if (occupy & 2048u64) == 0 { result |= 4096u64; }
      if (occupy & 6144u64) == 0 { result |= 8192u64; }
      if (occupy & 14336u64) == 0 { result |= 16384u64; }
      if (occupy & 30720u64) == 0 { result |= 32768u64; }
      result
    },  11 => {
      result = 5120u64;
      if (occupy & 1024u64) == 0 { result |= 512u64; }
      if (occupy & 1536u64) == 0 { result |= 256u64; }
      if (occupy & 4096u64) == 0 { result |= 8192u64; }
      if (occupy & 12288u64) == 0 { result |= 16384u64; }
      if (occupy & 28672u64) == 0 { result |= 32768u64; }
      result
    },  12 => {
      result = 10240u64;
      if (occupy & 2048u64) == 0 { result |= 1024u64; }
      if (occupy & 3072u64) == 0 { result |= 512u64; }
      if (occupy & 3584u64) == 0 { result |= 256u64; }
      if (occupy & 8192u64) == 0 { result |= 16384u64; }
      if (occupy & 24576u64) == 0 { result |= 32768u64; }
      result
    },  13 => {
      result = 20480u64;
      if (occupy & 4096u64) == 0 { result |= 2048u64; }
      if (occupy & 6144u64) == 0 { result |= 1024u64; }
      if (occupy & 7168u64) == 0 { result |= 512u64; }
      if (occupy & 7680u64) == 0 { result |= 256u64; }
      if (occupy & 16384u64) == 0 { result |= 32768u64; }
      result
    },  14 => {
      result = 40960u64;
      if (occupy & 8192u64) == 0 { result |= 4096u64; }
      if (occupy & 12288u64) == 0 { result |= 2048u64; }
      if (occupy & 14336u64) == 0 { result |= 1024u64; }
      if (occupy & 15360u64) == 0 { result |= 512u64; }
      if (occupy & 15872u64) == 0 { result |= 256u64; }
      result
    },  15 => {
      result = 16384u64;
      if (occupy & 16384u64) == 0 { result |= 8192u64; }
      if (occupy & 24576u64) == 0 { result |= 4096u64; }
      if (occupy & 28672u64) == 0 { result |= 2048u64; }
      if (occupy & 30720u64) == 0 { result |= 1024u64; }
      if (occupy & 31744u64) == 0 { result |= 512u64; }
      if (occupy & 32256u64) == 0 { result |= 256u64; }
      result
    },  16 => {
      result = 131072u64;
      if (occupy & 131072u64) == 0 { result |= 262144u64; }
      if (occupy & 393216u64) == 0 { result |= 524288u64; }
      if (occupy & 917504u64) == 0 { result |= 1048576u64; }
      if (occupy & 1966080u64) == 0 { result |= 2097152u64; }
      if (occupy & 4063232u64) == 0 { result |= 4194304u64; }
      if (occupy & 8257536u64) == 0 { result |= 8388608u64; }
      result
    },  17 => {
      result = 327680u64;
      if (occupy & 262144u64) == 0 { result |= 524288u64; }
      if (occupy & 786432u64) == 0 { result |= 1048576u64; }
      if (occupy & 1835008u64) == 0 { result |= 2097152u64; }
      if (occupy & 3932160u64) == 0 { result |= 4194304u64; }
      if (occupy & 8126464u64) == 0 { result |= 8388608u64; }
      result
    },  18 => {
      result = 655360u64;
      if (occupy & 131072u64) == 0 { result |= 65536u64; }
      if (occupy & 524288u64) == 0 { result |= 1048576u64; }
      if (occupy & 1572864u64) == 0 { result |= 2097152u64; }
      if (occupy & 3670016u64) == 0 { result |= 4194304u64; }
      if (occupy & 7864320u64) == 0 { result |= 8388608u64; }
      result
    },  19 => {
      result = 1310720u64;
      if (occupy & 262144u64) == 0 { result |= 131072u64; }
      if (occupy & 393216u64) == 0 { result |= 65536u64; }
      if (occupy & 1048576u64) == 0 { result |= 2097152u64; }
      if (occupy & 3145728u64) == 0 { result |= 4194304u64; }
      if (occupy & 7340032u64) == 0 { result |= 8388608u64; }
      result
    },  20 => {
      result = 2621440u64;
      if (occupy & 524288u64) == 0 { result |= 262144u64; }
      if (occupy & 786432u64) == 0 { result |= 131072u64; }
      if (occupy & 917504u64) == 0 { result |= 65536u64; }
      if (occupy & 2097152u64) == 0 { result |= 4194304u64; }
      if (occupy & 6291456u64) == 0 { result |= 8388608u64; }
      result
    },  21 => {
      result = 5242880u64;
      if (occupy & 1048576u64) == 0 { result |= 524288u64; }
      if (occupy & 1572864u64) == 0 { result |= 262144u64; }
      if (occupy & 1835008u64) == 0 { result |= 131072u64; }
      if (occupy & 1966080u64) == 0 { result |= 65536u64; }
      if (occupy & 4194304u64) == 0 { result |= 8388608u64; }
      result
    },  22 => {
      result = 10485760u64;
      if (occupy & 2097152u64) == 0 { result |= 1048576u64; }
      if (occupy & 3145728u64) == 0 { result |= 524288u64; }
      if (occupy & 3670016u64) == 0 { result |= 262144u64; }
      if (occupy & 3932160u64) == 0 { result |= 131072u64; }
      if (occupy & 4063232u64) == 0 { result |= 65536u64; }
      result
    },  23 => {
      result = 4194304u64;
      if (occupy & 4194304u64) == 0 { result |= 2097152u64; }
      if (occupy & 6291456u64) == 0 { result |= 1048576u64; }
      if (occupy & 7340032u64) == 0 { result |= 524288u64; }
      if (occupy & 7864320u64) == 0 { result |= 262144u64; }
      if (occupy & 8126464u64) == 0 { result |= 131072u64; }
      if (occupy & 8257536u64) == 0 { result |= 65536u64; }
      result
    },  24 => {
      result = 33554432u64;
      if (occupy & 33554432u64) == 0 { result |= 67108864u64; }
      if (occupy & 100663296u64) == 0 { result |= 134217728u64; }
      if (occupy & 234881024u64) == 0 { result |= 268435456u64; }
      if (occupy & 503316480u64) == 0 { result |= 536870912u64; }
      if (occupy & 1040187392u64) == 0 { result |= 1073741824u64; }
      if (occupy & 2113929216u64) == 0 { result |= 2147483648u64; }
      result
    },  25 => {
      result = 83886080u64;
      if (occupy & 67108864u64) == 0 { result |= 134217728u64; }
      if (occupy & 201326592u64) == 0 { result |= 268435456u64; }
      if (occupy & 469762048u64) == 0 { result |= 536870912u64; }
      if (occupy & 1006632960u64) == 0 { result |= 1073741824u64; }
      if (occupy & 2080374784u64) == 0 { result |= 2147483648u64; }
      result
    },  26 => {
      result = 167772160u64;
      if (occupy & 33554432u64) == 0 { result |= 16777216u64; }
      if (occupy & 134217728u64) == 0 { result |= 268435456u64; }
      if (occupy & 402653184u64) == 0 { result |= 536870912u64; }
      if (occupy & 939524096u64) == 0 { result |= 1073741824u64; }
      if (occupy & 2013265920u64) == 0 { result |= 2147483648u64; }
      result
    },  27 => {
      result = 335544320u64;
      if (occupy & 67108864u64) == 0 { result |= 33554432u64; }
      if (occupy & 100663296u64) == 0 { result |= 16777216u64; }
      if (occupy & 268435456u64) == 0 { result |= 536870912u64; }
      if (occupy & 805306368u64) == 0 { result |= 1073741824u64; }
      if (occupy & 1879048192u64) == 0 { result |= 2147483648u64; }
      result
    },  28 => {
      result = 671088640u64;
      if (occupy & 134217728u64) == 0 { result |= 67108864u64; }
      if (occupy & 201326592u64) == 0 { result |= 33554432u64; }
      if (occupy & 234881024u64) == 0 { result |= 16777216u64; }
      if (occupy & 536870912u64) == 0 { result |= 1073741824u64; }
      if (occupy & 1610612736u64) == 0 { result |= 2147483648u64; }
      result
    },  29 => {
      result = 1342177280u64;
      if (occupy & 268435456u64) == 0 { result |= 134217728u64; }
      if (occupy & 402653184u64) == 0 { result |= 67108864u64; }
      if (occupy & 469762048u64) == 0 { result |= 33554432u64; }
      if (occupy & 503316480u64) == 0 { result |= 16777216u64; }
      if (occupy & 1073741824u64) == 0 { result |= 2147483648u64; }
      result
    },  30 => {
      result = 2684354560u64;
      if (occupy & 536870912u64) == 0 { result |= 268435456u64; }
      if (occupy & 805306368u64) == 0 { result |= 134217728u64; }
      if (occupy & 939524096u64) == 0 { result |= 67108864u64; }
      if (occupy & 1006632960u64) == 0 { result |= 33554432u64; }
      if (occupy & 1040187392u64) == 0 { result |= 16777216u64; }
      result
    },  31 => {
      result = 1073741824u64;
      if (occupy & 1073741824u64) == 0 { result |= 536870912u64; }
      if (occupy & 1610612736u64) == 0 { result |= 268435456u64; }
      if (occupy & 1879048192u64) == 0 { result |= 134217728u64; }
      if (occupy & 2013265920u64) == 0 { result |= 67108864u64; }
      if (occupy & 2080374784u64) == 0 { result |= 33554432u64; }
      if (occupy & 2113929216u64) == 0 { result |= 16777216u64; }
      result
    },  32 => {
      result = 8589934592u64;
      if (occupy & 8589934592u64) == 0 { result |= 17179869184u64; }
      if (occupy & 25769803776u64) == 0 { result |= 34359738368u64; }
      if (occupy & 60129542144u64) == 0 { result |= 68719476736u64; }
      if (occupy & 128849018880u64) == 0 { result |= 137438953472u64; }
      if (occupy & 266287972352u64) == 0 { result |= 274877906944u64; }
      if (occupy & 541165879296u64) == 0 { result |= 549755813888u64; }
      result
    },  33 => {
      result = 21474836480u64;
      if (occupy & 17179869184u64) == 0 { result |= 34359738368u64; }
      if (occupy & 51539607552u64) == 0 { result |= 68719476736u64; }
      if (occupy & 120259084288u64) == 0 { result |= 137438953472u64; }
      if (occupy & 257698037760u64) == 0 { result |= 274877906944u64; }
      if (occupy & 532575944704u64) == 0 { result |= 549755813888u64; }
      result
    },  34 => {
      result = 42949672960u64;
      if (occupy & 8589934592u64) == 0 { result |= 4294967296u64; }
      if (occupy & 34359738368u64) == 0 { result |= 68719476736u64; }
      if (occupy & 103079215104u64) == 0 { result |= 137438953472u64; }
      if (occupy & 240518168576u64) == 0 { result |= 274877906944u64; }
      if (occupy & 515396075520u64) == 0 { result |= 549755813888u64; }
      result
    },  35 => {
      result = 85899345920u64;
      if (occupy & 17179869184u64) == 0 { result |= 8589934592u64; }
      if (occupy & 25769803776u64) == 0 { result |= 4294967296u64; }
      if (occupy & 68719476736u64) == 0 { result |= 137438953472u64; }
      if (occupy & 206158430208u64) == 0 { result |= 274877906944u64; }
      if (occupy & 481036337152u64) == 0 { result |= 549755813888u64; }
      result
    },  36 => {
      result = 171798691840u64;
      if (occupy & 34359738368u64) == 0 { result |= 17179869184u64; }
      if (occupy & 51539607552u64) == 0 { result |= 8589934592u64; }
      if (occupy & 60129542144u64) == 0 { result |= 4294967296u64; }
      if (occupy & 137438953472u64) == 0 { result |= 274877906944u64; }
      if (occupy & 412316860416u64) == 0 { result |= 549755813888u64; }
      result
    },  37 => {
      result = 343597383680u64;
      if (occupy & 68719476736u64) == 0 { result |= 34359738368u64; }
      if (occupy & 103079215104u64) == 0 { result |= 17179869184u64; }
      if (occupy & 120259084288u64) == 0 { result |= 8589934592u64; }
      if (occupy & 128849018880u64) == 0 { result |= 4294967296u64; }
      if (occupy & 274877906944u64) == 0 { result |= 549755813888u64; }
      result
    },  38 => {
      result = 687194767360u64;
      if (occupy & 137438953472u64) == 0 { result |= 68719476736u64; }
      if (occupy & 206158430208u64) == 0 { result |= 34359738368u64; }
      if (occupy & 240518168576u64) == 0 { result |= 17179869184u64; }
      if (occupy & 257698037760u64) == 0 { result |= 8589934592u64; }
      if (occupy & 266287972352u64) == 0 { result |= 4294967296u64; }
      result
    },  39 => {
      result = 274877906944u64;
      if (occupy & 274877906944u64) == 0 { result |= 137438953472u64; }
      if (occupy & 412316860416u64) == 0 { result |= 68719476736u64; }
      if (occupy & 481036337152u64) == 0 { result |= 34359738368u64; }
      if (occupy & 515396075520u64) == 0 { result |= 17179869184u64; }
      if (occupy & 532575944704u64) == 0 { result |= 8589934592u64; }
      if (occupy & 541165879296u64) == 0 { result |= 4294967296u64; }
      result
    },  40 => {
      result = 2199023255552u64;
      if (occupy & 2199023255552u64) == 0 { result |= 4398046511104u64; }
      if (occupy & 6597069766656u64) == 0 { result |= 8796093022208u64; }
      if (occupy & 15393162788864u64) == 0 { result |= 17592186044416u64; }
      if (occupy & 32985348833280u64) == 0 { result |= 35184372088832u64; }
      if (occupy & 68169720922112u64) == 0 { result |= 70368744177664u64; }
      if (occupy & 138538465099776u64) == 0 { result |= 140737488355328u64; }
      result
    },  41 => {
      result = 5497558138880u64;
      if (occupy & 4398046511104u64) == 0 { result |= 8796093022208u64; }
      if (occupy & 13194139533312u64) == 0 { result |= 17592186044416u64; }
      if (occupy & 30786325577728u64) == 0 { result |= 35184372088832u64; }
      if (occupy & 65970697666560u64) == 0 { result |= 70368744177664u64; }
      if (occupy & 136339441844224u64) == 0 { result |= 140737488355328u64; }
      result
    },  42 => {
      result = 10995116277760u64;
      if (occupy & 2199023255552u64) == 0 { result |= 1099511627776u64; }
      if (occupy & 8796093022208u64) == 0 { result |= 17592186044416u64; }
      if (occupy & 26388279066624u64) == 0 { result |= 35184372088832u64; }
      if (occupy & 61572651155456u64) == 0 { result |= 70368744177664u64; }
      if (occupy & 131941395333120u64) == 0 { result |= 140737488355328u64; }
      result
    },  43 => {
      result = 21990232555520u64;
      if (occupy & 4398046511104u64) == 0 { result |= 2199023255552u64; }
      if (occupy & 6597069766656u64) == 0 { result |= 1099511627776u64; }
      if (occupy & 17592186044416u64) == 0 { result |= 35184372088832u64; }
      if (occupy & 52776558133248u64) == 0 { result |= 70368744177664u64; }
      if (occupy & 123145302310912u64) == 0 { result |= 140737488355328u64; }
      result
    },  44 => {
      result = 43980465111040u64;
      if (occupy & 8796093022208u64) == 0 { result |= 4398046511104u64; }
      if (occupy & 13194139533312u64) == 0 { result |= 2199023255552u64; }
      if (occupy & 15393162788864u64) == 0 { result |= 1099511627776u64; }
      if (occupy & 35184372088832u64) == 0 { result |= 70368744177664u64; }
      if (occupy & 105553116266496u64) == 0 { result |= 140737488355328u64; }
      result
    },  45 => {
      result = 87960930222080u64;
      if (occupy & 17592186044416u64) == 0 { result |= 8796093022208u64; }
      if (occupy & 26388279066624u64) == 0 { result |= 4398046511104u64; }
      if (occupy & 30786325577728u64) == 0 { result |= 2199023255552u64; }
      if (occupy & 32985348833280u64) == 0 { result |= 1099511627776u64; }
      if (occupy & 70368744177664u64) == 0 { result |= 140737488355328u64; }
      result
    },  46 => {
      result = 175921860444160u64;
      if (occupy & 35184372088832u64) == 0 { result |= 17592186044416u64; }
      if (occupy & 52776558133248u64) == 0 { result |= 8796093022208u64; }
      if (occupy & 61572651155456u64) == 0 { result |= 4398046511104u64; }
      if (occupy & 65970697666560u64) == 0 { result |= 2199023255552u64; }
      if (occupy & 68169720922112u64) == 0 { result |= 1099511627776u64; }
      result
    },  47 => {
      result = 70368744177664u64;
      if (occupy & 70368744177664u64) == 0 { result |= 35184372088832u64; }
      if (occupy & 105553116266496u64) == 0 { result |= 17592186044416u64; }
      if (occupy & 123145302310912u64) == 0 { result |= 8796093022208u64; }
      if (occupy & 131941395333120u64) == 0 { result |= 4398046511104u64; }
      if (occupy & 136339441844224u64) == 0 { result |= 2199023255552u64; }
      if (occupy & 138538465099776u64) == 0 { result |= 1099511627776u64; }
      result
    },  48 => {
      result = 562949953421312u64;
      if (occupy & 562949953421312u64) == 0 { result |= 1125899906842624u64; }
      if (occupy & 1688849860263936u64) == 0 { result |= 2251799813685248u64; }
      if (occupy & 3940649673949184u64) == 0 { result |= 4503599627370496u64; }
      if (occupy & 8444249301319680u64) == 0 { result |= 9007199254740992u64; }
      if (occupy & 17451448556060672u64) == 0 { result |= 18014398509481984u64; }
      if (occupy & 35465847065542656u64) == 0 { result |= 36028797018963968u64; }
      result
    },  49 => {
      result = 1407374883553280u64;
      if (occupy & 1125899906842624u64) == 0 { result |= 2251799813685248u64; }
      if (occupy & 3377699720527872u64) == 0 { result |= 4503599627370496u64; }
      if (occupy & 7881299347898368u64) == 0 { result |= 9007199254740992u64; }
      if (occupy & 16888498602639360u64) == 0 { result |= 18014398509481984u64; }
      if (occupy & 34902897112121344u64) == 0 { result |= 36028797018963968u64; }
      result
    },  50 => {
      result = 2814749767106560u64;
      if (occupy & 562949953421312u64) == 0 { result |= 281474976710656u64; }
      if (occupy & 2251799813685248u64) == 0 { result |= 4503599627370496u64; }
      if (occupy & 6755399441055744u64) == 0 { result |= 9007199254740992u64; }
      if (occupy & 15762598695796736u64) == 0 { result |= 18014398509481984u64; }
      if (occupy & 33776997205278720u64) == 0 { result |= 36028797018963968u64; }
      result
    },  51 => {
      result = 5629499534213120u64;
      if (occupy & 1125899906842624u64) == 0 { result |= 562949953421312u64; }
      if (occupy & 1688849860263936u64) == 0 { result |= 281474976710656u64; }
      if (occupy & 4503599627370496u64) == 0 { result |= 9007199254740992u64; }
      if (occupy & 13510798882111488u64) == 0 { result |= 18014398509481984u64; }
      if (occupy & 31525197391593472u64) == 0 { result |= 36028797018963968u64; }
      result
    },  52 => {
      result = 11258999068426240u64;
      if (occupy & 2251799813685248u64) == 0 { result |= 1125899906842624u64; }
      if (occupy & 3377699720527872u64) == 0 { result |= 562949953421312u64; }
      if (occupy & 3940649673949184u64) == 0 { result |= 281474976710656u64; }
      if (occupy & 9007199254740992u64) == 0 { result |= 18014398509481984u64; }
      if (occupy & 27021597764222976u64) == 0 { result |= 36028797018963968u64; }
      result
    },  53 => {
      result = 22517998136852480u64;
      if (occupy & 4503599627370496u64) == 0 { result |= 2251799813685248u64; }
      if (occupy & 6755399441055744u64) == 0 { result |= 1125899906842624u64; }
      if (occupy & 7881299347898368u64) == 0 { result |= 562949953421312u64; }
      if (occupy & 8444249301319680u64) == 0 { result |= 281474976710656u64; }
      if (occupy & 18014398509481984u64) == 0 { result |= 36028797018963968u64; }
      result
    },  54 => {
      result = 45035996273704960u64;
      if (occupy & 9007199254740992u64) == 0 { result |= 4503599627370496u64; }
      if (occupy & 13510798882111488u64) == 0 { result |= 2251799813685248u64; }
      if (occupy & 15762598695796736u64) == 0 { result |= 1125899906842624u64; }
      if (occupy & 16888498602639360u64) == 0 { result |= 562949953421312u64; }
      if (occupy & 17451448556060672u64) == 0 { result |= 281474976710656u64; }
      result
    },  55 => {
      result = 18014398509481984u64;
      if (occupy & 18014398509481984u64) == 0 { result |= 9007199254740992u64; }
      if (occupy & 27021597764222976u64) == 0 { result |= 4503599627370496u64; }
      if (occupy & 31525197391593472u64) == 0 { result |= 2251799813685248u64; }
      if (occupy & 33776997205278720u64) == 0 { result |= 1125899906842624u64; }
      if (occupy & 34902897112121344u64) == 0 { result |= 562949953421312u64; }
      if (occupy & 35465847065542656u64) == 0 { result |= 281474976710656u64; }
      result
    },  56 => {
      result = 144115188075855872u64;
      if (occupy & 144115188075855872u64) == 0 { result |= 288230376151711744u64; }
      if (occupy & 432345564227567616u64) == 0 { result |= 576460752303423488u64; }
      if (occupy & 1008806316530991104u64) == 0 { result |= 1152921504606846976u64; }
      if (occupy & 2161727821137838080u64) == 0 { result |= 2305843009213693952u64; }
      if (occupy & 4467570830351532032u64) == 0 { result |= 4611686018427387904u64; }
      if (occupy & 9079256848778919936u64) == 0 { result |= 9223372036854775808u64; }
      result
    },  57 => {
      result = 360287970189639680u64;
      if (occupy & 288230376151711744u64) == 0 { result |= 576460752303423488u64; }
      if (occupy & 864691128455135232u64) == 0 { result |= 1152921504606846976u64; }
      if (occupy & 2017612633061982208u64) == 0 { result |= 2305843009213693952u64; }
      if (occupy & 4323455642275676160u64) == 0 { result |= 4611686018427387904u64; }
      if (occupy & 8935141660703064064u64) == 0 { result |= 9223372036854775808u64; }
      result
    },  58 => {
      result = 720575940379279360u64;
      if (occupy & 144115188075855872u64) == 0 { result |= 72057594037927936u64; }
      if (occupy & 576460752303423488u64) == 0 { result |= 1152921504606846976u64; }
      if (occupy & 1729382256910270464u64) == 0 { result |= 2305843009213693952u64; }
      if (occupy & 4035225266123964416u64) == 0 { result |= 4611686018427387904u64; }
      if (occupy & 8646911284551352320u64) == 0 { result |= 9223372036854775808u64; }
      result
    },  59 => {
      result = 1441151880758558720u64;
      if (occupy & 288230376151711744u64) == 0 { result |= 144115188075855872u64; }
      if (occupy & 432345564227567616u64) == 0 { result |= 72057594037927936u64; }
      if (occupy & 1152921504606846976u64) == 0 { result |= 2305843009213693952u64; }
      if (occupy & 3458764513820540928u64) == 0 { result |= 4611686018427387904u64; }
      if (occupy & 8070450532247928832u64) == 0 { result |= 9223372036854775808u64; }
      result
    },  60 => {
      result = 2882303761517117440u64;
      if (occupy & 576460752303423488u64) == 0 { result |= 288230376151711744u64; }
      if (occupy & 864691128455135232u64) == 0 { result |= 144115188075855872u64; }
      if (occupy & 1008806316530991104u64) == 0 { result |= 72057594037927936u64; }
      if (occupy & 2305843009213693952u64) == 0 { result |= 4611686018427387904u64; }
      if (occupy & 6917529027641081856u64) == 0 { result |= 9223372036854775808u64; }
      result
    },  61 => {
      result = 5764607523034234880u64;
      if (occupy & 1152921504606846976u64) == 0 { result |= 576460752303423488u64; }
      if (occupy & 1729382256910270464u64) == 0 { result |= 288230376151711744u64; }
      if (occupy & 2017612633061982208u64) == 0 { result |= 144115188075855872u64; }
      if (occupy & 2161727821137838080u64) == 0 { result |= 72057594037927936u64; }
      if (occupy & 4611686018427387904u64) == 0 { result |= 9223372036854775808u64; }
      result
    },  62 => {
      result = 11529215046068469760u64;
      if (occupy & 2305843009213693952u64) == 0 { result |= 1152921504606846976u64; }
      if (occupy & 3458764513820540928u64) == 0 { result |= 576460752303423488u64; }
      if (occupy & 4035225266123964416u64) == 0 { result |= 288230376151711744u64; }
      if (occupy & 4323455642275676160u64) == 0 { result |= 144115188075855872u64; }
      if (occupy & 4467570830351532032u64) == 0 { result |= 72057594037927936u64; }
      result
    },  63 => {
      result = 4611686018427387904u64;
      if (occupy & 4611686018427387904u64) == 0 { result |= 2305843009213693952u64; }
      if (occupy & 6917529027641081856u64) == 0 { result |= 1152921504606846976u64; }
      if (occupy & 8070450532247928832u64) == 0 { result |= 576460752303423488u64; }
      if (occupy & 8646911284551352320u64) == 0 { result |= 288230376151711744u64; }
      if (occupy & 8935141660703064064u64) == 0 { result |= 144115188075855872u64; }
      if (occupy & 9079256848778919936u64) == 0 { result |= 72057594037927936u64; }
      result
    },
    _ => 0
  }
}