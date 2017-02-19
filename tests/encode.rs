extern crate chasm;

use std::fmt::Write;

use chasm::code::Instruction;
use chasm::mnemonic::instruction::Adc;
use chasm::mnemonic::operand::{Imm8, Imm16, Imm32};

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
}
