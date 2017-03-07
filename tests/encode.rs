extern crate chasm;

use chasm::code::Encode;
use chasm::mnemonic::dsl::*;

macro_rules! test_encode {
    ($($mnemonic:expr => $encoding:expr,)+) => {
        #[test]
        fn encode() {
            $(assert_eq!($encoding, format!("{}", $mnemonic.encode()));)+
        }
    }
}

test_encode! {
    Adc::AlImm8(Imm8(0x01)) => "14 01",
    Adc::AxImm16(Imm16(0x0102)) => "66 15 02 01",
    Adc::EaxImm32(Imm32(0x01020304)) => "15 04 03 02 01",
    Adc::RaxImm32(Imm32(0x01020304)) => "48 15 04 03 02 01",

    Adc::Rm8lImm8(Rm8l::R8l(R8l::Al), Imm8(0x01)) => "80 d0 01",
    Adc::Rm8lImm8(Rm8l::R8l(R8l::Cl), Imm8(0x01)) => "80 d1 01",
    Adc::Rm8lImm8(Rm8l::R8l(R8l::Dl), Imm8(0x01)) => "80 d2 01",
    Adc::Rm8lImm8(Rm8l::R8l(R8l::Bl), Imm8(0x01)) => "80 d3 01",
    Adc::Rm8lImm8(Rm8l::R8l(R8l::Ah), Imm8(0x01)) => "80 d4 01",
    Adc::Rm8lImm8(Rm8l::R8l(R8l::Ch), Imm8(0x01)) => "80 d5 01",
    Adc::Rm8lImm8(Rm8l::R8l(R8l::Dh), Imm8(0x01)) => "80 d6 01",
    Adc::Rm8lImm8(Rm8l::R8l(R8l::Bh), Imm8(0x01)) => "80 d7 01",

    Adc::Rm8Imm8(Rm8::R8(Al), Imm8(0x01)) => "80 d0 01",
    Adc::Rm8Imm8(Rm8::R8(Cl), Imm8(0x01)) => "80 d1 01",
    Adc::Rm8Imm8(Rm8::R8(Dl), Imm8(0x01)) => "80 d2 01",
    Adc::Rm8Imm8(Rm8::R8(Bl), Imm8(0x01)) => "80 d3 01",
    Adc::Rm8Imm8(Rm8::R8(Spl), Imm8(0x01)) => "40 80 d4 01",
    Adc::Rm8Imm8(Rm8::R8(Bpl), Imm8(0x01)) => "40 80 d5 01",
    Adc::Rm8Imm8(Rm8::R8(Sil), Imm8(0x01)) => "40 80 d6 01",
    Adc::Rm8Imm8(Rm8::R8(Dil), Imm8(0x01)) => "40 80 d7 01",
    Adc::Rm8Imm8(Rm8::R8(R8l), Imm8(0x01)) => "41 80 d0 01",
    Adc::Rm8Imm8(Rm8::R8(R9l), Imm8(0x01)) => "41 80 d1 01",
    Adc::Rm8Imm8(Rm8::R8(R10l), Imm8(0x01)) => "41 80 d2 01",
    Adc::Rm8Imm8(Rm8::R8(R11l), Imm8(0x01)) => "41 80 d3 01",
    Adc::Rm8Imm8(Rm8::R8(R12l), Imm8(0x01)) => "41 80 d4 01",
    Adc::Rm8Imm8(Rm8::R8(R13l), Imm8(0x01)) => "41 80 d5 01",
    Adc::Rm8Imm8(Rm8::R8(R14l), Imm8(0x01)) => "41 80 d6 01",
    Adc::Rm8Imm8(Rm8::R8(R15l), Imm8(0x01)) => "41 80 d7 01",

    Adc::Rm8Imm8(M8(Offset64(Some(Cs), Base(Rax))), Imm8(0x01)) => "2e 80 10 01",
    Adc::Rm8Imm8(M8(Offset64(Some(Ss), Base(Rax))), Imm8(0x01)) => "36 80 10 01",
    Adc::Rm8Imm8(M8(Offset64(Some(Ds), Base(Rax))), Imm8(0x01)) => "3e 80 10 01",
    Adc::Rm8Imm8(M8(Offset64(Some(Es), Base(Rax))), Imm8(0x01)) => "26 80 10 01",
    Adc::Rm8Imm8(M8(Offset64(Some(Fs), Base(Rax))), Imm8(0x01)) => "64 80 10 01",
    Adc::Rm8Imm8(M8(Offset64(Some(Gs), Base(Rax))), Imm8(0x01)) => "65 80 10 01",

    Adc::Rm8Imm8(M8(Offset64(None, Base(Rax))), Imm8(0x01)) => "80 10 01",
    Adc::Rm8Imm8(M8(Offset64(None, Base(Rcx))), Imm8(0x01)) => "80 11 01",
    Adc::Rm8Imm8(M8(Offset64(None, Base(Rdx))), Imm8(0x01)) => "80 12 01",
    Adc::Rm8Imm8(M8(Offset64(None, Base(Rbx))), Imm8(0x01)) => "80 13 01",
    Adc::Rm8Imm8(M8(Offset64(None, Base(Rsp))), Imm8(0x01)) => "80 14 24 01",
    Adc::Rm8Imm8(M8(Offset64(None, Base(Rbp))), Imm8(0x01)) => "80 55 00 01",
    Adc::Rm8Imm8(M8(Offset64(None, Base(Rsi))), Imm8(0x01)) => "80 16 01",
    Adc::Rm8Imm8(M8(Offset64(None, Base(Rdi))), Imm8(0x01)) => "80 17 01",
    Adc::Rm8Imm8(M8(Offset64(None, Base(R8))), Imm8(0x01)) => "41 80 10 01",
    Adc::Rm8Imm8(M8(Offset64(None, Base(R9))), Imm8(0x01)) => "41 80 11 01",
    Adc::Rm8Imm8(M8(Offset64(None, Base(R10))), Imm8(0x01)) => "41 80 12 01",
    Adc::Rm8Imm8(M8(Offset64(None, Base(R11))), Imm8(0x01)) => "41 80 13 01",
    Adc::Rm8Imm8(M8(Offset64(None, Base(R12))), Imm8(0x01)) => "41 80 14 24 01",
    Adc::Rm8Imm8(M8(Offset64(None, Base(R13))), Imm8(0x01)) => "41 80 55 00 01",
    Adc::Rm8Imm8(M8(Offset64(None, Base(R14))), Imm8(0x01)) => "41 80 16 01",
    Adc::Rm8Imm8(M8(Offset64(None, Base(R15))), Imm8(0x01)) => "41 80 17 01",

    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(Rax, Disp8(0x01)))), Imm8(0x02)) => "80 50 01 02",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(Rcx, Disp8(0x01)))), Imm8(0x02)) => "80 51 01 02",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(Rdx, Disp8(0x01)))), Imm8(0x02)) => "80 52 01 02",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(Rbx, Disp8(0x01)))), Imm8(0x02)) => "80 53 01 02",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(Rsp, Disp8(0x01)))), Imm8(0x02)) => "80 54 24 01 02",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(Rbp, Disp8(0x01)))), Imm8(0x02)) => "80 55 01 02",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(Rsi, Disp8(0x01)))), Imm8(0x02)) => "80 56 01 02",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(Rdi, Disp8(0x01)))), Imm8(0x02)) => "80 57 01 02",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(R8, Disp8(0x01)))), Imm8(0x02)) => "41 80 50 01 02",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(R9, Disp8(0x01)))), Imm8(0x02)) => "41 80 51 01 02",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(R10, Disp8(0x01)))), Imm8(0x02)) => "41 80 52 01 02",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(R11, Disp8(0x01)))), Imm8(0x02)) => "41 80 53 01 02",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(R12, Disp8(0x01)))), Imm8(0x02)) => "41 80 54 24 01 02",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(R13, Disp8(0x01)))), Imm8(0x02)) => "41 80 55 01 02",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(R14, Disp8(0x01)))), Imm8(0x02)) => "41 80 56 01 02",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(R15, Disp8(0x01)))), Imm8(0x02)) => "41 80 57 01 02",

    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(Rax, Disp32(0x01020304)))), Imm8(0x05)) =>
        "80 90 04 03 02 01 05",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(Rcx, Disp32(0x01020304)))), Imm8(0x05)) =>
        "80 91 04 03 02 01 05",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(Rdx, Disp32(0x01020304)))), Imm8(0x05)) =>
        "80 92 04 03 02 01 05",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(Rbx, Disp32(0x01020304)))), Imm8(0x05)) =>
        "80 93 04 03 02 01 05",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(Rsp, Disp32(0x01020304)))), Imm8(0x05)) =>
        "80 94 24 04 03 02 01 05",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(Rbp, Disp32(0x01020304)))), Imm8(0x05)) =>
        "80 95 04 03 02 01 05",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(Rsi, Disp32(0x01020304)))), Imm8(0x05)) =>
        "80 96 04 03 02 01 05",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(Rdi, Disp32(0x01020304)))), Imm8(0x05)) =>
        "80 97 04 03 02 01 05",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(R8, Disp32(0x01020304)))), Imm8(0x05)) =>
        "41 80 90 04 03 02 01 05",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(R9, Disp32(0x01020304)))), Imm8(0x05)) =>
        "41 80 91 04 03 02 01 05",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(R10, Disp32(0x01020304)))), Imm8(0x05)) =>
        "41 80 92 04 03 02 01 05",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(R11, Disp32(0x01020304)))), Imm8(0x05)) =>
        "41 80 93 04 03 02 01 05",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(R12, Disp32(0x01020304)))), Imm8(0x05)) =>
        "41 80 94 24 04 03 02 01 05",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(R13, Disp32(0x01020304)))), Imm8(0x05)) =>
        "41 80 95 04 03 02 01 05",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(R14, Disp32(0x01020304)))), Imm8(0x05)) =>
        "41 80 96 04 03 02 01 05",
    Adc::Rm8Imm8(M8(Offset64(None, BaseDisp(R15, Disp32(0x01020304)))), Imm8(0x05)) =>
        "41 80 97 04 03 02 01 05",

    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::Rax, X1))), Imm8(0x01)) =>
        "80 14 00 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rcx, Index64::Rax, X1))), Imm8(0x01)) =>
        "80 14 01 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rdx, Index64::Rax, X1))), Imm8(0x01)) =>
        "80 14 02 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rbx, Index64::Rax, X1))), Imm8(0x01)) =>
        "80 14 03 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rsp, Index64::Rax, X1))), Imm8(0x01)) =>
        "80 14 04 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rbp, Index64::Rax, X1))), Imm8(0x01)) =>
        "80 54 05 00 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rsi, Index64::Rax, X1))), Imm8(0x01)) =>
        "80 14 06 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rdi, Index64::Rax, X1))), Imm8(0x01)) =>
        "80 14 07 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(R8, Index64::Rax, X1))), Imm8(0x01)) =>
        "41 80 14 00 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(R9, Index64::Rax, X1))), Imm8(0x01)) =>
        "41 80 14 01 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(R10, Index64::Rax, X1))), Imm8(0x01)) =>
        "41 80 14 02 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(R11, Index64::Rax, X1))), Imm8(0x01)) =>
        "41 80 14 03 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(R12, Index64::Rax, X1))), Imm8(0x01)) =>
        "41 80 14 04 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(R13, Index64::Rax, X1))), Imm8(0x01)) =>
        "41 80 54 05 00 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(R14, Index64::Rax, X1))), Imm8(0x01)) =>
        "41 80 14 06 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(R15, Index64::Rax, X1))), Imm8(0x01)) =>
        "41 80 14 07 01",

    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::Rcx, X1))), Imm8(0x01)) =>
        "80 14 08 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::Rdx, X1))), Imm8(0x01)) =>
        "80 14 10 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::Rbx, X1))), Imm8(0x01)) =>
        "80 14 18 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::Rbp, X1))), Imm8(0x01)) =>
        "80 14 28 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::Rsi, X1))), Imm8(0x01)) =>
        "80 14 30 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::Rdi, X1))), Imm8(0x01)) =>
        "80 14 38 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::R8, X1))), Imm8(0x01)) =>
        "42 80 14 00 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::R9, X1))), Imm8(0x01)) =>
        "42 80 14 08 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::R10, X1))), Imm8(0x01)) =>
        "42 80 14 10 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::R11, X1))), Imm8(0x01)) =>
        "42 80 14 18 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::R12, X1))), Imm8(0x01)) =>
        "42 80 14 20 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::R13, X1))), Imm8(0x01)) =>
        "42 80 14 28 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::R14, X1))), Imm8(0x01)) =>
        "42 80 14 30 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::R15, X1))), Imm8(0x01)) =>
        "42 80 14 38 01",

    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::Rax, X2))), Imm8(0x01)) =>
        "80 14 40 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::Rax, X4))), Imm8(0x01)) =>
        "80 14 80 01",
    Adc::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::Rax, X8))), Imm8(0x01)) =>
        "80 14 c0 01",

    Adc::Rm8Imm8(
        M8(Offset64(None, BaseIndexDisp(Rax, Index64::Rax, X1, Disp8(0x01)))),
        Imm8(0x02),
    ) => "80 54 00 01 02",
    Adc::Rm8Imm8(
        M8(Offset64(None, BaseIndexDisp(Rax, Index64::Rax, X1, Disp32(0x01020304)))),
        Imm8(0x05),
    ) => "80 94 00 04 03 02 01 05",

    Adc::Rm8Imm8(M8(Offset64(None, Index(Index64::Rax, X1))), Imm8(0x01)) =>
        "80 14 05 00 00 00 00 01",

    Adc::Rm8Imm8(M8(Offset64(None, IndexDisp(Index64::Rax, X1, 0x01020304))), Imm8(0x05)) =>
        "80 14 05 04 03 02 01 05",

    Adc::Rm8Imm8(M8(Offset64(None, Disp(0x01020304))), Imm8(0x05)) => "80 14 25 04 03 02 01 05",

    Adc::Rm8Imm8(M8(Offset64(None, RipDisp(0x01020304))), Imm8(0x05)) => "80 15 04 03 02 01 05",

    Adc::Rm8Imm8(M8(Offset32(None, RipDisp(0x01020304))), Imm8(0x05)) => "67 80 15 04 03 02 01 05",

    Adc::Rm16Imm16(R16(Cx), Imm16(0x0102)) => "66 81 d1 02 01",
    Adc::Rm32Imm32(R32(Ecx), Imm32(0x01020304)) => "81 d1 04 03 02 01",
    Adc::Rm64Imm32(R64(Rcx), Imm32(0x01020304)) => "48 81 d1 04 03 02 01",

    Adc::Rm16Imm8(R16(Cx), Imm8(0x01)) => "66 83 d1 01",
    Adc::Rm32Imm8(R32(Ecx), Imm8(0x01)) => "83 d1 01",
    Adc::Rm64Imm8(R64(Rcx), Imm8(0x01)) => "48 83 d1 01",

    Adc::Rm32Imm8(R32(Eax), Imm8(0x01)) => "83 d0 01",
    Adc::Rm32Imm8(R32(Edx), Imm8(0x01)) => "83 d2 01",
    Adc::Rm32Imm8(R32(Ebx), Imm8(0x01)) => "83 d3 01",
    Adc::Rm32Imm8(R32(Esp), Imm8(0x01)) => "83 d4 01",
    Adc::Rm32Imm8(R32(Ebp), Imm8(0x01)) => "83 d5 01",
    Adc::Rm32Imm8(R32(Esi), Imm8(0x01)) => "83 d6 01",
    Adc::Rm32Imm8(R32(Edi), Imm8(0x01)) => "83 d7 01",
    Adc::Rm32Imm8(R32(R8d), Imm8(0x01)) => "41 83 d0 01",
    Adc::Rm32Imm8(R32(R9d), Imm8(0x01)) => "41 83 d1 01",
    Adc::Rm32Imm8(R32(R10d), Imm8(0x01)) => "41 83 d2 01",
    Adc::Rm32Imm8(R32(R11d), Imm8(0x01)) => "41 83 d3 01",
    Adc::Rm32Imm8(R32(R12d), Imm8(0x01)) => "41 83 d4 01",
    Adc::Rm32Imm8(R32(R13d), Imm8(0x01)) => "41 83 d5 01",
    Adc::Rm32Imm8(R32(R14d), Imm8(0x01)) => "41 83 d6 01",
    Adc::Rm32Imm8(R32(R15d), Imm8(0x01)) => "41 83 d7 01",

    Adc::Rm8lR8l(Rm8l::R8l(R8l::Al), R8l::Cl) => "10 c8",
    Adc::Rm8R8(Rm8::R8(Al), Cl) => "10 c8",
    Adc::Rm16R16(R16(Ax), Cx) => "66 11 c8",
    Adc::Rm32R32(R32(Eax), Ecx) => "11 c8",
    Adc::Rm64R64(R64(Rax), Rcx) => "48 11 c8",

    Adc::Rm32R32(R32(Eax), Eax) => "11 c0",
    Adc::Rm32R32(R32(Eax), Edx) => "11 d0",
    Adc::Rm32R32(R32(Eax), Ebx) => "11 d8",
    Adc::Rm32R32(R32(Eax), Esp) => "11 e0",
    Adc::Rm32R32(R32(Eax), Ebp) => "11 e8",
    Adc::Rm32R32(R32(Eax), Esi) => "11 f0",
    Adc::Rm32R32(R32(Eax), Edi) => "11 f8",
    Adc::Rm32R32(R32(Eax), R8d) => "44 11 c0",
    Adc::Rm32R32(R32(Eax), R9d) => "44 11 c8",
    Adc::Rm32R32(R32(Eax), R10d) => "44 11 d0",
    Adc::Rm32R32(R32(Eax), R11d) => "44 11 d8",
    Adc::Rm32R32(R32(Eax), R12d) => "44 11 e0",
    Adc::Rm32R32(R32(Eax), R13d) => "44 11 e8",
    Adc::Rm32R32(R32(Eax), R14d) => "44 11 f0",
    Adc::Rm32R32(R32(Eax), R15d) => "44 11 f8",

    Adc::R8lRm8l(R8l::Cl, Rm8l::R8l(R8l::Al)) => "12 c8",
    Adc::R8Rm8(Cl, Rm8::R8(Al)) => "12 c8",
    Adc::R16Rm16(Cx, R16(Ax)) => "66 13 c8",
    Adc::R32Rm32(Ecx, R32(Eax)) => "13 c8",
    Adc::R64Rm64(Rcx, R64(Rax)) => "48 13 c8",
}
