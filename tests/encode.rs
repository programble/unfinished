extern crate chasm;

use std::fmt::Write;

use chasm::code::Instruction;
use chasm::mnemonic::instruction::Adc;
use chasm::mnemonic::operand::*;

fn hex<I>(iter: I) -> String where I: IntoIterator<Item = u8> {
    let mut string = String::new();
    for byte in iter {
        write!(string, "{:02x} ", byte).unwrap();
    }
    string.pop();
    string
}

macro_rules! test_encode {
    ($($mnemonic:expr => $encoding:expr,)+) => {
        #[test]
        fn encode() {
            $(assert_eq!($encoding, hex(&Instruction::from(&$mnemonic)));)+
        }
    }
}

test_encode! {
    Adc::AlImm8(Imm8(0x01)) => "14 01",
    Adc::AxImm16(Imm16(0x0102)) => "66 15 02 01",
    Adc::EaxImm32(Imm32(0x01020304)) => "15 04 03 02 01",
    Adc::RaxImm32(Imm32(0x01020304)) => "48 15 04 03 02 01",

    Adc::Rm8Imm8(Rm8::Reg8(Reg8::Al), Imm8(0x01)) => "80 d0 01",
    Adc::Rm8Imm8(Rm8::Reg8(Reg8::Cl), Imm8(0x01)) => "80 d1 01",
    Adc::Rm8Imm8(Rm8::Reg8(Reg8::Dl), Imm8(0x01)) => "80 d2 01",
    Adc::Rm8Imm8(Rm8::Reg8(Reg8::Bl), Imm8(0x01)) => "80 d3 01",
    Adc::Rm8Imm8(Rm8::Reg8(Reg8::Ah), Imm8(0x01)) => "80 d4 01",
    Adc::Rm8Imm8(Rm8::Reg8(Reg8::Ch), Imm8(0x01)) => "80 d5 01",
    Adc::Rm8Imm8(Rm8::Reg8(Reg8::Dh), Imm8(0x01)) => "80 d6 01",
    Adc::Rm8Imm8(Rm8::Reg8(Reg8::Bh), Imm8(0x01)) => "80 d7 01",

    Adc::Rm8Imm8(Rm8::Rex8(Rex8::Al), Imm8(0x01)) => "80 d0 01",
    Adc::Rm8Imm8(Rm8::Rex8(Rex8::Cl), Imm8(0x01)) => "80 d1 01",
    Adc::Rm8Imm8(Rm8::Rex8(Rex8::Dl), Imm8(0x01)) => "80 d2 01",
    Adc::Rm8Imm8(Rm8::Rex8(Rex8::Bl), Imm8(0x01)) => "80 d3 01",
    Adc::Rm8Imm8(Rm8::Rex8(Rex8::Spl), Imm8(0x01)) => "40 80 d4 01",
    Adc::Rm8Imm8(Rm8::Rex8(Rex8::Bpl), Imm8(0x01)) => "40 80 d5 01",
    Adc::Rm8Imm8(Rm8::Rex8(Rex8::Sil), Imm8(0x01)) => "40 80 d6 01",
    Adc::Rm8Imm8(Rm8::Rex8(Rex8::Dil), Imm8(0x01)) => "40 80 d7 01",
    Adc::Rm8Imm8(Rm8::Rex8(Rex8::R8l), Imm8(0x01)) => "41 80 d0 01",
    Adc::Rm8Imm8(Rm8::Rex8(Rex8::R9l), Imm8(0x01)) => "41 80 d1 01",
    Adc::Rm8Imm8(Rm8::Rex8(Rex8::R10l), Imm8(0x01)) => "41 80 d2 01",
    Adc::Rm8Imm8(Rm8::Rex8(Rex8::R11l), Imm8(0x01)) => "41 80 d3 01",
    Adc::Rm8Imm8(Rm8::Rex8(Rex8::R12l), Imm8(0x01)) => "41 80 d4 01",
    Adc::Rm8Imm8(Rm8::Rex8(Rex8::R13l), Imm8(0x01)) => "41 80 d5 01",
    Adc::Rm8Imm8(Rm8::Rex8(Rex8::R14l), Imm8(0x01)) => "41 80 d6 01",
    Adc::Rm8Imm8(Rm8::Rex8(Rex8::R15l), Imm8(0x01)) => "41 80 d7 01",

    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Disp(0x01020304))),
        Imm8(0x05),
    ) => "80 14 25 04 03 02 01 05",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(Some(Sreg::Cs), Offset::Disp(0x01020304))),
        Imm8(0x05),
    ) => "2e 80 14 25 04 03 02 01 05",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(Some(Sreg::Ss), Offset::Disp(0x01020304))),
        Imm8(0x05),
    ) => "36 80 14 25 04 03 02 01 05",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(Some(Sreg::Ds), Offset::Disp(0x01020304))),
        Imm8(0x05),
    ) => "3e 80 14 25 04 03 02 01 05",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(Some(Sreg::Es), Offset::Disp(0x01020304))),
        Imm8(0x05),
    ) => "26 80 14 25 04 03 02 01 05",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(Some(Sreg::Fs), Offset::Disp(0x01020304))),
        Imm8(0x05),
    ) => "64 80 14 25 04 03 02 01 05",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(Some(Sreg::Gs), Offset::Disp(0x01020304))),
        Imm8(0x05),
    ) => "65 80 14 25 04 03 02 01 05",

    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rax, Scale::X1))),
        Imm8(0x01),
    ) => "80 14 05 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rax, Scale::X2))),
        Imm8(0x01),
    ) => "80 14 45 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rax, Scale::X4))),
        Imm8(0x01),
    ) => "80 14 85 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rax, Scale::X8))),
        Imm8(0x01),
    ) => "80 14 c5 00 00 00 00 01",

    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rcx, Scale::X1))),
        Imm8(0x01),
    ) => "80 14 0d 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rdx, Scale::X1))),
        Imm8(0x01),
    ) => "80 14 15 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rbx, Scale::X1))),
        Imm8(0x01),
    ) => "80 14 1d 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rbp, Scale::X1))),
        Imm8(0x01),
    ) => "80 14 2d 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rsi, Scale::X1))),
        Imm8(0x01),
    ) => "80 14 35 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rdi, Scale::X1))),
        Imm8(0x01),
    ) => "80 14 3d 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::Index(IndexRex64::R8, Scale::X1))),
        Imm8(0x01),
    ) => "42 80 14 05 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::Index(IndexRex64::R9, Scale::X1))),
        Imm8(0x01),
    ) => "42 80 14 0d 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::Index(IndexRex64::R10, Scale::X1))),
        Imm8(0x01),
    ) => "42 80 14 15 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::Index(IndexRex64::R11, Scale::X1))),
        Imm8(0x01),
    ) => "42 80 14 1d 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::Index(IndexRex64::R12, Scale::X1))),
        Imm8(0x01),
    ) => "42 80 14 25 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::Index(IndexRex64::R13, Scale::X1))),
        Imm8(0x01),
    ) => "42 80 14 2d 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::Index(IndexRex64::R14, Scale::X1))),
        Imm8(0x01),
    ) => "42 80 14 35 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::Index(IndexRex64::R15, Scale::X1))),
        Imm8(0x01),
    ) => "42 80 14 3d 00 00 00 00 01",

    Adc::Rm8Imm8(
        Rm8::Mem8(
            Memory::Offset64(None, Offset::IndexDisp(IndexReg64::Rax, Scale::X1, 0x01020304))
        ),
        Imm8(0x05),
    ) => "80 14 05 04 03 02 01 05",

    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Base(Reg64::Rax))),
        Imm8(0x01),
    ) => "80 10 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Base(Reg64::Rcx))),
        Imm8(0x01),
    ) => "80 11 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Base(Reg64::Rdx))),
        Imm8(0x01),
    ) => "80 12 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Base(Reg64::Rbx))),
        Imm8(0x01),
    ) => "80 13 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Base(Reg64::Rsp))),
        Imm8(0x01),
    ) => "80 14 24 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Base(Reg64::Rbp))),
        Imm8(0x01),
    ) => "80 55 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Base(Reg64::Rsi))),
        Imm8(0x01),
    ) => "80 16 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Base(Reg64::Rdi))),
        Imm8(0x01),
    ) => "80 17 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::Base(Rex64::R8))),
        Imm8(0x01),
    ) => "41 80 10 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::Base(Rex64::R9))),
        Imm8(0x01),
    ) => "41 80 11 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::Base(Rex64::R10))),
        Imm8(0x01),
    ) => "41 80 12 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::Base(Rex64::R11))),
        Imm8(0x01),
    ) => "41 80 13 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::Base(Rex64::R12))),
        Imm8(0x01),
    ) => "41 80 14 24 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::Base(Rex64::R13))),
        Imm8(0x01),
    ) => "41 80 55 00 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::Base(Rex64::R14))),
        Imm8(0x01),
    ) => "41 80 16 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::Base(Rex64::R15))),
        Imm8(0x01),
    ) => "41 80 17 01",

    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::BaseDisp(Reg64::Rax, Disp::Disp8(0x01)))),
        Imm8(0x02),
    ) => "80 50 01 02",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::BaseDisp(Reg64::Rcx, Disp::Disp8(0x01)))),
        Imm8(0x02),
    ) => "80 51 01 02",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::BaseDisp(Reg64::Rdx, Disp::Disp8(0x01)))),
        Imm8(0x02),
    ) => "80 52 01 02",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::BaseDisp(Reg64::Rbx, Disp::Disp8(0x01)))),
        Imm8(0x02),
    ) => "80 53 01 02",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::BaseDisp(Reg64::Rsp, Disp::Disp8(0x01)))),
        Imm8(0x02),
    ) => "80 54 24 01 02",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::BaseDisp(Reg64::Rbp, Disp::Disp8(0x01)))),
        Imm8(0x02),
    ) => "80 55 01 02",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::BaseDisp(Reg64::Rsi, Disp::Disp8(0x01)))),
        Imm8(0x02),
    ) => "80 56 01 02",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::BaseDisp(Reg64::Rdi, Disp::Disp8(0x01)))),
        Imm8(0x02),
    ) => "80 57 01 02",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::BaseDisp(Rex64::R8, Disp::Disp8(0x01)))),
        Imm8(0x02),
    ) => "41 80 50 01 02",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::BaseDisp(Rex64::R9, Disp::Disp8(0x01)))),
        Imm8(0x02),
    ) => "41 80 51 01 02",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::BaseDisp(Rex64::R10, Disp::Disp8(0x01)))),
        Imm8(0x02),
    ) => "41 80 52 01 02",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::BaseDisp(Rex64::R11, Disp::Disp8(0x01)))),
        Imm8(0x02),
    ) => "41 80 53 01 02",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::BaseDisp(Rex64::R12, Disp::Disp8(0x01)))),
        Imm8(0x02),
    ) => "41 80 54 24 01 02",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::BaseDisp(Rex64::R13, Disp::Disp8(0x01)))),
        Imm8(0x02),
    ) => "41 80 55 01 02",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::BaseDisp(Rex64::R14, Disp::Disp8(0x01)))),
        Imm8(0x02),
    ) => "41 80 56 01 02",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::BaseDisp(Rex64::R15, Disp::Disp8(0x01)))),
        Imm8(0x02),
    ) => "41 80 57 01 02",

    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::BaseDisp(Reg64::Rax, Disp::Disp32(0x01020304)))),
        Imm8(0x05),
    ) => "80 90 04 03 02 01 05",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::BaseDisp(Reg64::Rcx, Disp::Disp32(0x01020304)))),
        Imm8(0x05),
    ) => "80 91 04 03 02 01 05",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::BaseDisp(Reg64::Rdx, Disp::Disp32(0x01020304)))),
        Imm8(0x05),
    ) => "80 92 04 03 02 01 05",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::BaseDisp(Reg64::Rbx, Disp::Disp32(0x01020304)))),
        Imm8(0x05),
    ) => "80 93 04 03 02 01 05",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::BaseDisp(Reg64::Rsp, Disp::Disp32(0x01020304)))),
        Imm8(0x05),
    ) => "80 94 24 04 03 02 01 05",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::BaseDisp(Reg64::Rbp, Disp::Disp32(0x01020304)))),
        Imm8(0x05),
    ) => "80 95 04 03 02 01 05",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::BaseDisp(Reg64::Rsi, Disp::Disp32(0x01020304)))),
        Imm8(0x05),
    ) => "80 96 04 03 02 01 05",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::BaseDisp(Reg64::Rdi, Disp::Disp32(0x01020304)))),
        Imm8(0x05),
    ) => "80 97 04 03 02 01 05",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::BaseDisp(Rex64::R8, Disp::Disp32(0x01020304)))),
        Imm8(0x05),
    ) => "41 80 90 04 03 02 01 05",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::BaseDisp(Rex64::R9, Disp::Disp32(0x01020304)))),
        Imm8(0x05),
    ) => "41 80 91 04 03 02 01 05",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::BaseDisp(Rex64::R10, Disp::Disp32(0x01020304)))),
        Imm8(0x05),
    ) => "41 80 92 04 03 02 01 05",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::BaseDisp(Rex64::R11, Disp::Disp32(0x01020304)))),
        Imm8(0x05),
    ) => "41 80 93 04 03 02 01 05",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::BaseDisp(Rex64::R12, Disp::Disp32(0x01020304)))),
        Imm8(0x05),
    ) => "41 80 94 24 04 03 02 01 05",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::BaseDisp(Rex64::R13, Disp::Disp32(0x01020304)))),
        Imm8(0x05),
    ) => "41 80 95 04 03 02 01 05",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::BaseDisp(Rex64::R14, Disp::Disp32(0x01020304)))),
        Imm8(0x05),
    ) => "41 80 96 04 03 02 01 05",
    Adc::Rm8Imm8(
        Rm8::Mex8(Memory::Offset64(None, Offset::BaseDisp(Rex64::R15, Disp::Disp32(0x01020304)))),
        Imm8(0x05),
    ) => "41 80 97 04 03 02 01 05",

    Adc::Rm8Imm8(
        Rm8::Mem8(
            Memory::Offset64(None, Offset::BaseIndex(Reg64::Rax, IndexReg64::Rax, Scale::X1))
        ),
        Imm8(0x01),
    ) => "80 14 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(
            Memory::Offset64(None, Offset::BaseIndex(Reg64::Rcx, IndexReg64::Rax, Scale::X1))
        ),
        Imm8(0x01),
    ) => "80 14 01 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(
            Memory::Offset64(None, Offset::BaseIndex(Reg64::Rdx, IndexReg64::Rax, Scale::X1))
        ),
        Imm8(0x01),
    ) => "80 14 02 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(
            Memory::Offset64(None, Offset::BaseIndex(Reg64::Rbx, IndexReg64::Rax, Scale::X1))
        ),
        Imm8(0x01),
    ) => "80 14 03 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(
            Memory::Offset64(None, Offset::BaseIndex(Reg64::Rsp, IndexReg64::Rax, Scale::X1))
        ),
        Imm8(0x01),
    ) => "80 14 04 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(
            Memory::Offset64(None, Offset::BaseIndex(Reg64::Rbp, IndexReg64::Rax, Scale::X1))
        ),
        Imm8(0x01),
    ) => "80 54 05 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(
            Memory::Offset64(None, Offset::BaseIndex(Reg64::Rsi, IndexReg64::Rax, Scale::X1))
        ),
        Imm8(0x01),
    ) => "80 14 06 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(
            Memory::Offset64(None, Offset::BaseIndex(Reg64::Rdi, IndexReg64::Rax, Scale::X1))
        ),
        Imm8(0x01),
    ) => "80 14 07 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(
            Memory::Offset64(None, Offset::BaseIndex(Rex64::R8, IndexRex64::R8, Scale::X1))
        ),
        Imm8(0x01),
    ) => "43 80 14 00 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(
            Memory::Offset64(None, Offset::BaseIndex(Rex64::R9, IndexRex64::R8, Scale::X1))
        ),
        Imm8(0x01),
    ) => "43 80 14 01 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(
            Memory::Offset64(None, Offset::BaseIndex(Rex64::R10, IndexRex64::R8, Scale::X1))
        ),
        Imm8(0x01),
    ) => "43 80 14 02 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(
            Memory::Offset64(None, Offset::BaseIndex(Rex64::R11, IndexRex64::R8, Scale::X1))
        ),
        Imm8(0x01),
    ) => "43 80 14 03 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(
            Memory::Offset64(None, Offset::BaseIndex(Rex64::R12, IndexRex64::R8, Scale::X1))
        ),
        Imm8(0x01),
    ) => "43 80 14 04 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(
            Memory::Offset64(None, Offset::BaseIndex(Rex64::R13, IndexRex64::R8, Scale::X1))
        ),
        Imm8(0x01),
    ) => "43 80 54 05 00 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(
            Memory::Offset64(None, Offset::BaseIndex(Rex64::R14, IndexRex64::R8, Scale::X1))
        ),
        Imm8(0x01),
    ) => "43 80 14 06 01",
    Adc::Rm8Imm8(
        Rm8::Mex8(
            Memory::Offset64(None, Offset::BaseIndex(Rex64::R15, IndexRex64::R8, Scale::X1))
        ),
        Imm8(0x01),
    ) => "43 80 14 07 01",

    Adc::Rm8Imm8(
        Rm8::Mem8(
            Memory::Offset64(
                None,
                Offset::BaseIndexDisp(Reg64::Rax, IndexReg64::Rax, Scale::X1, Disp::Disp8(0x01)),
            ),
        ),
        Imm8(0x02),
    ) => "80 54 00 01 02",
    Adc::Rm8Imm8(
        Rm8::Mex8(
            Memory::Offset64(
                None,
                Offset::BaseIndexDisp(Rex64::R8, IndexRex64::R8, Scale::X1, Disp::Disp8(0x01)),
            ),
        ),
        Imm8(0x02),
    ) => "43 80 54 00 01 02",
    Adc::Rm8Imm8(
        Rm8::Mem8(
            Memory::Offset64(
                None,
                Offset::BaseIndexDisp(
                    Reg64::Rax,
                    IndexReg64::Rax,
                    Scale::X1,
                    Disp::Disp32(0x01020304),
                ),
            ),
        ),
        Imm8(0x05),
    ) => "80 94 00 04 03 02 01 05",
    Adc::Rm8Imm8(
        Rm8::Mex8(
            Memory::Offset64(
                None,
                Offset::BaseIndexDisp(
                    Rex64::R8,
                    IndexRex64::R8,
                    Scale::X1,
                    Disp::Disp32(0x01020304),
                ),
            ),
        ),
        Imm8(0x05),
    ) => "43 80 94 00 04 03 02 01 05",

    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::RipDisp(0x01020304))),
        Imm8(0x05),
    ) => "80 15 04 03 02 01 05",
}
