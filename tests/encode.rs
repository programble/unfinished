extern crate chasm;

use chasm::code::Encode;
use chasm::mnemonic::dsl::*;

macro_rules! test_encode {
    ($($test:ident { $($mnemonic:expr => $encoding:expr,)+ },)+) => {
        $(
            #[test]
            fn $test() {
                $(assert_eq!($encoding, format!("{}", $mnemonic.encode()));)+
            }
        )+
    }
}

test_encode! {
    add_eax_imm {
        Add::AlImm8(Imm8(0x01)) => "04 01",
        Add::AxImm16(Imm16(0x0102)) => "66 05 02 01",
        Add::EaxImm32(Imm32(0x01020304)) => "05 04 03 02 01",
        Add::RaxImm32(Imm32(0x01020304)) => "48 05 04 03 02 01",
    },

    add_rm8l_imm8 {
        Add::Rm8lImm8(Rm8l::R8l(R8l::Al), Imm8(0x01)) => "80 c0 01",
        Add::Rm8lImm8(Rm8l::R8l(R8l::Cl), Imm8(0x01)) => "80 c1 01",
        Add::Rm8lImm8(Rm8l::R8l(R8l::Dl), Imm8(0x01)) => "80 c2 01",
        Add::Rm8lImm8(Rm8l::R8l(R8l::Bl), Imm8(0x01)) => "80 c3 01",
        Add::Rm8lImm8(Rm8l::R8l(R8l::Ah), Imm8(0x01)) => "80 c4 01",
        Add::Rm8lImm8(Rm8l::R8l(R8l::Ch), Imm8(0x01)) => "80 c5 01",
        Add::Rm8lImm8(Rm8l::R8l(R8l::Dh), Imm8(0x01)) => "80 c6 01",
        Add::Rm8lImm8(Rm8l::R8l(R8l::Bh), Imm8(0x01)) => "80 c7 01",
    },

    add_rm8_imm8_r8 {
        Add::Rm8Imm8(Rm8::R8(Al), Imm8(0x01)) => "80 c0 01",
        Add::Rm8Imm8(Rm8::R8(Cl), Imm8(0x01)) => "80 c1 01",
        Add::Rm8Imm8(Rm8::R8(Dl), Imm8(0x01)) => "80 c2 01",
        Add::Rm8Imm8(Rm8::R8(Bl), Imm8(0x01)) => "80 c3 01",
        Add::Rm8Imm8(Rm8::R8(Spl), Imm8(0x01)) => "40 80 c4 01",
        Add::Rm8Imm8(Rm8::R8(Bpl), Imm8(0x01)) => "40 80 c5 01",
        Add::Rm8Imm8(Rm8::R8(Sil), Imm8(0x01)) => "40 80 c6 01",
        Add::Rm8Imm8(Rm8::R8(Dil), Imm8(0x01)) => "40 80 c7 01",
        Add::Rm8Imm8(Rm8::R8(R8l), Imm8(0x01)) => "41 80 c0 01",
        Add::Rm8Imm8(Rm8::R8(R9l), Imm8(0x01)) => "41 80 c1 01",
        Add::Rm8Imm8(Rm8::R8(R10l), Imm8(0x01)) => "41 80 c2 01",
        Add::Rm8Imm8(Rm8::R8(R11l), Imm8(0x01)) => "41 80 c3 01",
        Add::Rm8Imm8(Rm8::R8(R12l), Imm8(0x01)) => "41 80 c4 01",
        Add::Rm8Imm8(Rm8::R8(R13l), Imm8(0x01)) => "41 80 c5 01",
        Add::Rm8Imm8(Rm8::R8(R14l), Imm8(0x01)) => "41 80 c6 01",
        Add::Rm8Imm8(Rm8::R8(R15l), Imm8(0x01)) => "41 80 c7 01",
    },

    add_rm8_imm8_m8_sreg {
        Add::Rm8Imm8(M8(Offset64(Some(Cs), Base(Rax))), Imm8(0x01)) => "2e 80 00 01",
        Add::Rm8Imm8(M8(Offset64(Some(Ss), Base(Rax))), Imm8(0x01)) => "36 80 00 01",
        Add::Rm8Imm8(M8(Offset64(Some(Ds), Base(Rax))), Imm8(0x01)) => "3e 80 00 01",
        Add::Rm8Imm8(M8(Offset64(Some(Es), Base(Rax))), Imm8(0x01)) => "26 80 00 01",
        Add::Rm8Imm8(M8(Offset64(Some(Fs), Base(Rax))), Imm8(0x01)) => "64 80 00 01",
        Add::Rm8Imm8(M8(Offset64(Some(Gs), Base(Rax))), Imm8(0x01)) => "65 80 00 01",
    },

    add_rm8_imm8_m8_base {
        Add::Rm8Imm8(M8(Offset64(None, Base(Rax))), Imm8(0x01)) => "80 00 01",
        Add::Rm8Imm8(M8(Offset64(None, Base(Rcx))), Imm8(0x01)) => "80 01 01",
        Add::Rm8Imm8(M8(Offset64(None, Base(Rdx))), Imm8(0x01)) => "80 02 01",
        Add::Rm8Imm8(M8(Offset64(None, Base(Rbx))), Imm8(0x01)) => "80 03 01",
        Add::Rm8Imm8(M8(Offset64(None, Base(Rsp))), Imm8(0x01)) => "80 04 24 01",
        Add::Rm8Imm8(M8(Offset64(None, Base(Rbp))), Imm8(0x01)) => "80 45 00 01",
        Add::Rm8Imm8(M8(Offset64(None, Base(Rsi))), Imm8(0x01)) => "80 06 01",
        Add::Rm8Imm8(M8(Offset64(None, Base(Rdi))), Imm8(0x01)) => "80 07 01",
        Add::Rm8Imm8(M8(Offset64(None, Base(R8))), Imm8(0x01)) => "41 80 00 01",
        Add::Rm8Imm8(M8(Offset64(None, Base(R9))), Imm8(0x01)) => "41 80 01 01",
        Add::Rm8Imm8(M8(Offset64(None, Base(R10))), Imm8(0x01)) => "41 80 02 01",
        Add::Rm8Imm8(M8(Offset64(None, Base(R11))), Imm8(0x01)) => "41 80 03 01",
        Add::Rm8Imm8(M8(Offset64(None, Base(R12))), Imm8(0x01)) => "41 80 04 24 01",
        Add::Rm8Imm8(M8(Offset64(None, Base(R13))), Imm8(0x01)) => "41 80 45 00 01",
        Add::Rm8Imm8(M8(Offset64(None, Base(R14))), Imm8(0x01)) => "41 80 06 01",
        Add::Rm8Imm8(M8(Offset64(None, Base(R15))), Imm8(0x01)) => "41 80 07 01",
    },

    add_rm8_imm8_m8_base_disp8 {
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(Rax, Disp8(0x01)))), Imm8(0x02)) =>
            "80 40 01 02",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(Rcx, Disp8(0x01)))), Imm8(0x02)) =>
            "80 41 01 02",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(Rdx, Disp8(0x01)))), Imm8(0x02)) =>
            "80 42 01 02",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(Rbx, Disp8(0x01)))), Imm8(0x02)) =>
            "80 43 01 02",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(Rsp, Disp8(0x01)))), Imm8(0x02)) =>
            "80 44 24 01 02",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(Rbp, Disp8(0x01)))), Imm8(0x02)) =>
            "80 45 01 02",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(Rsi, Disp8(0x01)))), Imm8(0x02)) =>
            "80 46 01 02",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(Rdi, Disp8(0x01)))), Imm8(0x02)) =>
            "80 47 01 02",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(R8, Disp8(0x01)))), Imm8(0x02)) =>
            "41 80 40 01 02",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(R9, Disp8(0x01)))), Imm8(0x02)) =>
            "41 80 41 01 02",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(R10, Disp8(0x01)))), Imm8(0x02)) =>
            "41 80 42 01 02",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(R11, Disp8(0x01)))), Imm8(0x02)) =>
            "41 80 43 01 02",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(R12, Disp8(0x01)))), Imm8(0x02)) =>
            "41 80 44 24 01 02",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(R13, Disp8(0x01)))), Imm8(0x02)) =>
            "41 80 45 01 02",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(R14, Disp8(0x01)))), Imm8(0x02)) =>
            "41 80 46 01 02",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(R15, Disp8(0x01)))), Imm8(0x02)) =>
            "41 80 47 01 02",
    },

    add_rm8_imm8_m8_base_disp32 {
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(Rax, Disp32(0x01020304)))), Imm8(0x05)) =>
            "80 80 04 03 02 01 05",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(Rcx, Disp32(0x01020304)))), Imm8(0x05)) =>
            "80 81 04 03 02 01 05",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(Rdx, Disp32(0x01020304)))), Imm8(0x05)) =>
            "80 82 04 03 02 01 05",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(Rbx, Disp32(0x01020304)))), Imm8(0x05)) =>
            "80 83 04 03 02 01 05",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(Rsp, Disp32(0x01020304)))), Imm8(0x05)) =>
            "80 84 24 04 03 02 01 05",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(Rbp, Disp32(0x01020304)))), Imm8(0x05)) =>
            "80 85 04 03 02 01 05",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(Rsi, Disp32(0x01020304)))), Imm8(0x05)) =>
            "80 86 04 03 02 01 05",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(Rdi, Disp32(0x01020304)))), Imm8(0x05)) =>
            "80 87 04 03 02 01 05",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(R8, Disp32(0x01020304)))), Imm8(0x05)) =>
            "41 80 80 04 03 02 01 05",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(R9, Disp32(0x01020304)))), Imm8(0x05)) =>
            "41 80 81 04 03 02 01 05",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(R10, Disp32(0x01020304)))), Imm8(0x05)) =>
            "41 80 82 04 03 02 01 05",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(R11, Disp32(0x01020304)))), Imm8(0x05)) =>
            "41 80 83 04 03 02 01 05",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(R12, Disp32(0x01020304)))), Imm8(0x05)) =>
            "41 80 84 24 04 03 02 01 05",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(R13, Disp32(0x01020304)))), Imm8(0x05)) =>
            "41 80 85 04 03 02 01 05",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(R14, Disp32(0x01020304)))), Imm8(0x05)) =>
            "41 80 86 04 03 02 01 05",
        Add::Rm8Imm8(M8(Offset64(None, BaseDisp(R15, Disp32(0x01020304)))), Imm8(0x05)) =>
            "41 80 87 04 03 02 01 05",
    },

    add_rm8_imm8_m8_base_index_base {
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::Rax, X1))), Imm8(0x01)) =>
            "80 04 00 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rcx, Index64::Rax, X1))), Imm8(0x01)) =>
            "80 04 01 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rdx, Index64::Rax, X1))), Imm8(0x01)) =>
            "80 04 02 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rbx, Index64::Rax, X1))), Imm8(0x01)) =>
            "80 04 03 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rsp, Index64::Rax, X1))), Imm8(0x01)) =>
            "80 04 04 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rbp, Index64::Rax, X1))), Imm8(0x01)) =>
            "80 44 05 00 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rsi, Index64::Rax, X1))), Imm8(0x01)) =>
            "80 04 06 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rdi, Index64::Rax, X1))), Imm8(0x01)) =>
            "80 04 07 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(R8, Index64::Rax, X1))), Imm8(0x01)) =>
            "41 80 04 00 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(R9, Index64::Rax, X1))), Imm8(0x01)) =>
            "41 80 04 01 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(R10, Index64::Rax, X1))), Imm8(0x01)) =>
            "41 80 04 02 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(R11, Index64::Rax, X1))), Imm8(0x01)) =>
            "41 80 04 03 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(R12, Index64::Rax, X1))), Imm8(0x01)) =>
            "41 80 04 04 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(R13, Index64::Rax, X1))), Imm8(0x01)) =>
            "41 80 44 05 00 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(R14, Index64::Rax, X1))), Imm8(0x01)) =>
            "41 80 04 06 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(R15, Index64::Rax, X1))), Imm8(0x01)) =>
            "41 80 04 07 01",
    },

    add_rm8_imm8_m8_base_index_index {
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::Rcx, X1))), Imm8(0x01)) =>
            "80 04 08 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::Rdx, X1))), Imm8(0x01)) =>
            "80 04 10 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::Rbx, X1))), Imm8(0x01)) =>
            "80 04 18 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::Rbp, X1))), Imm8(0x01)) =>
            "80 04 28 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::Rsi, X1))), Imm8(0x01)) =>
            "80 04 30 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::Rdi, X1))), Imm8(0x01)) =>
            "80 04 38 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::R8, X1))), Imm8(0x01)) =>
            "42 80 04 00 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::R9, X1))), Imm8(0x01)) =>
            "42 80 04 08 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::R10, X1))), Imm8(0x01)) =>
            "42 80 04 10 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::R11, X1))), Imm8(0x01)) =>
            "42 80 04 18 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::R12, X1))), Imm8(0x01)) =>
            "42 80 04 20 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::R13, X1))), Imm8(0x01)) =>
            "42 80 04 28 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::R14, X1))), Imm8(0x01)) =>
            "42 80 04 30 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::R15, X1))), Imm8(0x01)) =>
            "42 80 04 38 01",
    },

    add_rm8_imm8_m8_base_index_scale {
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::Rax, X2))), Imm8(0x01)) =>
            "80 04 40 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::Rax, X4))), Imm8(0x01)) =>
            "80 04 80 01",
        Add::Rm8Imm8(M8(Offset64(None, BaseIndex(Rax, Index64::Rax, X8))), Imm8(0x01)) =>
            "80 04 c0 01",
    },

    add_rm8_imm8_m8_base_index_disp {
        Add::Rm8Imm8(
            M8(Offset64(None, BaseIndexDisp(Rax, Index64::Rax, X1, Disp8(0x01)))),
            Imm8(0x02),
        ) => "80 44 00 01 02",
        Add::Rm8Imm8(
            M8(Offset64(None, BaseIndexDisp(Rax, Index64::Rax, X1, Disp32(0x01020304)))),
            Imm8(0x05),
        ) => "80 84 00 04 03 02 01 05",
    },

    add_rm8_imm8_m8_index_disp {
        Add::Rm8Imm8(M8(Offset64(None, IndexDisp(Index64::Rax, X1, 0x01020304))), Imm8(0x05)) =>
            "80 04 05 04 03 02 01 05",
    },

    add_rm8_imm8_m8_disp {
        Add::Rm8Imm8(M8(Offset64(None, Disp(0x01020304))), Imm8(0x05)) =>
            "80 04 25 04 03 02 01 05",
    },

    add_rm8_imm8_m8_rip_disp {
        Add::Rm8Imm8(M8(Offset64(None, RipDisp(0x01020304))), Imm8(0x05)) =>
            "80 05 04 03 02 01 05",
    },

    add_rm8_imm8_m8_offset32 {
        Add::Rm8Imm8(M8(Offset32(None, RipDisp(0x01020304))), Imm8(0x05)) =>
            "67 80 05 04 03 02 01 05",
    },

    add_rm_imm {
        Add::Rm16Imm16(R16(Cx), Imm16(0x0102)) => "66 81 c1 02 01",
        Add::Rm32Imm32(R32(Ecx), Imm32(0x01020304)) => "81 c1 04 03 02 01",
        Add::Rm64Imm32(R64(Rcx), Imm32(0x01020304)) => "48 81 c1 04 03 02 01",
    },

    add_rm_imm8 {
        Add::Rm16Imm8(R16(Cx), Imm8(0x01)) => "66 83 c1 01",
        Add::Rm32Imm8(R32(Ecx), Imm8(0x01)) => "83 c1 01",
        Add::Rm64Imm8(R64(Rcx), Imm8(0x01)) => "48 83 c1 01",
    },

    add_rm32_imm8 {
        Add::Rm32Imm8(R32(Eax), Imm8(0x01)) => "83 c0 01",
        Add::Rm32Imm8(R32(Edx), Imm8(0x01)) => "83 c2 01",
        Add::Rm32Imm8(R32(Ebx), Imm8(0x01)) => "83 c3 01",
        Add::Rm32Imm8(R32(Esp), Imm8(0x01)) => "83 c4 01",
        Add::Rm32Imm8(R32(Ebp), Imm8(0x01)) => "83 c5 01",
        Add::Rm32Imm8(R32(Esi), Imm8(0x01)) => "83 c6 01",
        Add::Rm32Imm8(R32(Edi), Imm8(0x01)) => "83 c7 01",
        Add::Rm32Imm8(R32(R8d), Imm8(0x01)) => "41 83 c0 01",
        Add::Rm32Imm8(R32(R9d), Imm8(0x01)) => "41 83 c1 01",
        Add::Rm32Imm8(R32(R10d), Imm8(0x01)) => "41 83 c2 01",
        Add::Rm32Imm8(R32(R11d), Imm8(0x01)) => "41 83 c3 01",
        Add::Rm32Imm8(R32(R12d), Imm8(0x01)) => "41 83 c4 01",
        Add::Rm32Imm8(R32(R13d), Imm8(0x01)) => "41 83 c5 01",
        Add::Rm32Imm8(R32(R14d), Imm8(0x01)) => "41 83 c6 01",
        Add::Rm32Imm8(R32(R15d), Imm8(0x01)) => "41 83 c7 01",
    },

    add_rm_r {
        Add::Rm8lR8l(Rm8l::R8l(R8l::Al), R8l::Cl) => "00 c8",
        Add::Rm8R8(Rm8::R8(Al), Cl) => "00 c8",
        Add::Rm16R16(R16(Ax), Cx) => "66 01 c8",
        Add::Rm32R32(R32(Eax), Ecx) => "01 c8",
        Add::Rm64R64(R64(Rax), Rcx) => "48 01 c8",
    },

    add_rm32_r32 {
        Add::Rm32R32(R32(Eax), Eax) => "01 c0",
        Add::Rm32R32(R32(Eax), Edx) => "01 d0",
        Add::Rm32R32(R32(Eax), Ebx) => "01 d8",
        Add::Rm32R32(R32(Eax), Esp) => "01 e0",
        Add::Rm32R32(R32(Eax), Ebp) => "01 e8",
        Add::Rm32R32(R32(Eax), Esi) => "01 f0",
        Add::Rm32R32(R32(Eax), Edi) => "01 f8",
        Add::Rm32R32(R32(Eax), R8d) => "44 01 c0",
        Add::Rm32R32(R32(Eax), R9d) => "44 01 c8",
        Add::Rm32R32(R32(Eax), R10d) => "44 01 d0",
        Add::Rm32R32(R32(Eax), R11d) => "44 01 d8",
        Add::Rm32R32(R32(Eax), R12d) => "44 01 e0",
        Add::Rm32R32(R32(Eax), R13d) => "44 01 e8",
        Add::Rm32R32(R32(Eax), R14d) => "44 01 f0",
        Add::Rm32R32(R32(Eax), R15d) => "44 01 f8",
    },

    add_r_rm {
        Add::R8lRm8l(R8l::Cl, Rm8l::R8l(R8l::Al)) => "02 c8",
        Add::R8Rm8(Cl, Rm8::R8(Al)) => "02 c8",
        Add::R16Rm16(Cx, R16(Ax)) => "66 03 c8",
        Add::R32Rm32(Ecx, R32(Eax)) => "03 c8",
        Add::R64Rm64(Rcx, R64(Rax)) => "48 03 c8",
    },

    mov_rm16_sreg {
        Mov::Rm16Sreg(R16(Ax), Es) => "8c c0",
        Mov::Rm16Sreg(R16(Ax), Cs) => "8c c8",
        Mov::Rm16Sreg(R16(Ax), Ss) => "8c d0",
        Mov::Rm16Sreg(R16(Ax), Ds) => "8c d8",
        Mov::Rm16Sreg(R16(Ax), Fs) => "8c e0",
        Mov::Rm16Sreg(R16(Ax), Gs) => "8c e8",
    },

    mov_rm64_sreg {
        Mov::Rm64Sreg(R64(Rax), Es) => "48 8c c0",
    },

    mov_sreg_rm16 {
        Mov::SregRm16(Es, R16(Ax)) => "8e c0",
    },

    mov_sreg_rm64 {
        Mov::SregRm64(Es, R64(Rax)) => "48 8e c0",
    },

    mov_al_moffs8 {
        Mov::AlMoffs8(Moffset32(None, 0x01020304)) => "67 a0 04 03 02 01",
        Mov::AlMoffs8(Moffset32(Some(Es), 0x01020304)) => "26 67 a0 04 03 02 01",
        Mov::AlMoffs8(Moffset64(None, 0x0102030405060708)) => "a0 08 07 06 05 04 03 02 01",
        Mov::AlMoffs8(Moffset64(Some(Es), 0x0102030405060708)) => "26 a0 08 07 06 05 04 03 02 01",
    },

    mov_eax_moffs {
        Mov::AxMoffs16(Moffset64(None, 0x0102030405060708)) => "66 a1 08 07 06 05 04 03 02 01",
        Mov::EaxMoffs32(Moffset64(None, 0x0102030405060708)) => "a1 08 07 06 05 04 03 02 01",
        Mov::RaxMoffs64(Moffset64(None, 0x0102030405060708)) => "48 a1 08 07 06 05 04 03 02 01",
    },

    mov_moffs_eax {
        Mov::Moffs8Al(Moffset64(None, 0x0102030405060708)) => "a2 08 07 06 05 04 03 02 01",
        Mov::Moffs16Ax(Moffset64(None, 0x0102030405060708)) => "66 a3 08 07 06 05 04 03 02 01",
        Mov::Moffs32Eax(Moffset64(None, 0x0102030405060708)) => "a3 08 07 06 05 04 03 02 01",
        Mov::Moffs64Rax(Moffset64(None, 0x0102030405060708)) => "48 a3 08 07 06 05 04 03 02 01",
    },

    mov_r_imm {
        Mov::R8lImm8(R8l::Al, Imm8(0x01)) => "b0 01",
        Mov::R8lImm8(R8l::Cl, Imm8(0x01)) => "b1 01",
        Mov::R8lImm8(R8l::Dl, Imm8(0x01)) => "b2 01",
        Mov::R8lImm8(R8l::Bl, Imm8(0x01)) => "b3 01",
        Mov::R8lImm8(R8l::Ah, Imm8(0x01)) => "b4 01",
        Mov::R8lImm8(R8l::Ch, Imm8(0x01)) => "b5 01",
        Mov::R8lImm8(R8l::Dh, Imm8(0x01)) => "b6 01",
        Mov::R8lImm8(R8l::Bh, Imm8(0x01)) => "b7 01",
        Mov::R8Imm8(Al, Imm8(0x01)) => "b0 01",
        Mov::R16Imm16(Ax, Imm16(0x0102)) => "66 b8 02 01",
        Mov::R32Imm32(Eax, Imm32(0x01020304)) => "b8 04 03 02 01",
        Mov::R64Imm64(Rax, Imm64(0x0102030405060708)) => "48 b8 08 07 06 05 04 03 02 01",
    },

    mov_rm_imm {
        Mov::Rm8lImm8(M8l(Offset64(None, Base(R64l::Rax))), Imm8(0x01)) => "c6 00 01",
        Mov::Rm8Imm8(M8(Offset64(None, Base(Rax))), Imm8(0x01)) => "c6 00 01",
        Mov::Rm16Imm16(M16(Offset64(None, Base(Rax))), Imm16(0x0102)) => "66 c7 00 02 01",
        Mov::Rm32Imm32(M32(Offset64(None, Base(Rax))), Imm32(0x01020304)) => "c7 00 04 03 02 01",
        Mov::Rm64Imm32(M64(Offset64(None, Base(Rax))), Imm32(0x01020304)) => "48 c7 00 04 03 02 01",
    },

    mov_r64_cr {
        Mov::R64Cr(Rax, Cr0) => "0f 20 c0",
        Mov::R64Cr(Rax, Cr2) => "0f 20 d0",
        Mov::R64Cr(Rax, Cr3) => "0f 20 d8",
        Mov::R64Cr(Rax, Cr4) => "0f 20 e0",
        Mov::R64Cr(Rax, Cr8) => "44 0f 20 c0",
    },

    mov_cr_r64 {
        Mov::CrR64(Cr0, Rax) => "0f 22 c0",
    },

    mov_r64_dr {
        Mov::R64Dr(Rax, Dr0) => "0f 21 c0",
        Mov::R64Dr(Rax, Dr1) => "0f 21 c8",
        Mov::R64Dr(Rax, Dr2) => "0f 21 d0",
        Mov::R64Dr(Rax, Dr3) => "0f 21 d8",
        Mov::R64Dr(Rax, Dr4) => "0f 21 e0",
        Mov::R64Dr(Rax, Dr5) => "0f 21 e8",
        Mov::R64Dr(Rax, Dr6) => "0f 21 f0",
        Mov::R64Dr(Rax, Dr7) => "0f 21 f8",
    },

    mov_dr_r64 {
        Mov::DrR64(Dr0, Rax) => "0f 23 c0",
    },
}
