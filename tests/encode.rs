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
        Imm8(0x01),
    ) => "80 14 25 04 03 02 01 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(Some(Sreg::Cs), Offset::Disp(0x01020304))),
        Imm8(0x01),
    ) => "2e 80 14 25 04 03 02 01 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(Some(Sreg::Ss), Offset::Disp(0x01020304))),
        Imm8(0x01),
    ) => "36 80 14 25 04 03 02 01 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(Some(Sreg::Ds), Offset::Disp(0x01020304))),
        Imm8(0x01),
    ) => "3e 80 14 25 04 03 02 01 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(Some(Sreg::Es), Offset::Disp(0x01020304))),
        Imm8(0x01),
    ) => "26 80 14 25 04 03 02 01 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(Some(Sreg::Fs), Offset::Disp(0x01020304))),
        Imm8(0x01),
    ) => "64 80 14 25 04 03 02 01 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(Some(Sreg::Gs), Offset::Disp(0x01020304))),
        Imm8(0x01),
    ) => "65 80 14 25 04 03 02 01 01",

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
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rcx, Scale::X2))),
        Imm8(0x01),
    ) => "80 14 4d 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rcx, Scale::X4))),
        Imm8(0x01),
    ) => "80 14 8d 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rcx, Scale::X8))),
        Imm8(0x01),
    ) => "80 14 cd 00 00 00 00 01",

    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rdx, Scale::X1))),
        Imm8(0x01),
    ) => "80 14 15 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rdx, Scale::X2))),
        Imm8(0x01),
    ) => "80 14 55 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rdx, Scale::X4))),
        Imm8(0x01),
    ) => "80 14 95 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rdx, Scale::X8))),
        Imm8(0x01),
    ) => "80 14 d5 00 00 00 00 01",

    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rbx, Scale::X1))),
        Imm8(0x01),
    ) => "80 14 1d 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rbx, Scale::X2))),
        Imm8(0x01),
    ) => "80 14 5d 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rbx, Scale::X4))),
        Imm8(0x01),
    ) => "80 14 9d 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rbx, Scale::X8))),
        Imm8(0x01),
    ) => "80 14 dd 00 00 00 00 01",

    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rbp, Scale::X1))),
        Imm8(0x01),
    ) => "80 14 2d 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rbp, Scale::X2))),
        Imm8(0x01),
    ) => "80 14 6d 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rbp, Scale::X4))),
        Imm8(0x01),
    ) => "80 14 ad 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rbp, Scale::X8))),
        Imm8(0x01),
    ) => "80 14 ed 00 00 00 00 01",

    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rsi, Scale::X1))),
        Imm8(0x01),
    ) => "80 14 35 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rsi, Scale::X2))),
        Imm8(0x01),
    ) => "80 14 75 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rsi, Scale::X4))),
        Imm8(0x01),
    ) => "80 14 b5 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rsi, Scale::X8))),
        Imm8(0x01),
    ) => "80 14 f5 00 00 00 00 01",

    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rdi, Scale::X1))),
        Imm8(0x01),
    ) => "80 14 3d 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rdi, Scale::X2))),
        Imm8(0x01),
    ) => "80 14 7d 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rdi, Scale::X4))),
        Imm8(0x01),
    ) => "80 14 bd 00 00 00 00 01",
    Adc::Rm8Imm8(
        Rm8::Mem8(Memory::Offset64(None, Offset::Index(IndexReg64::Rdi, Scale::X8))),
        Imm8(0x01),
    ) => "80 14 fd 00 00 00 00 01",
}
