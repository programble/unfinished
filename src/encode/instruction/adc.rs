use encode::{Encode, prefix};
use mnemonic::instruction::Adc;
use output::Output;

use self::Adc::*;

fn opcode(adc: &Adc) -> u8 {
    match *adc {
        AlImm8(..)                                    => 0x14,
        AxImm16(..) | EaxImm32(..) | RaxImm32(..)     => 0x15,
        Rm8Imm8(..)                                   => 0x80,
        Rm16Imm16(..) | Rm32Imm32(..) | Rm64Imm32(..) => 0x81,
        Rm16Imm8(..) | Rm32Imm8(..) | Rm64Imm8(..)    => 0x83,
        Rm8R8(..)                                     => 0x10,
        Rm16R16(..) | Rm32R32(..) | Rm64R64(..)       => 0x11,
        R8Rm8(..)                                     => 0x12,
        R16Rm16(..) | R32Rm32(..) | R64Rm64(..)       => 0x13,
    }
}

fn encode_imm<O>(adc: &Adc, out: &mut O) -> Result<(), O::Err> where O: Output {
    match *adc {
        AlImm8(x) | Rm8Imm8(_, x) | Rm16Imm8(_, x) | Rm32Imm8(_, x) | Rm64Imm8(_, x) => {
            out.write_u8(x)
        },
        AxImm16(x) | Rm16Imm16(_, x) => {
            out.write_u16(x)
        },
        EaxImm32(x) | RaxImm32(x) | Rm32Imm32(_, x) | Rm64Imm32(_, x) => {
            out.write_u32(x)
        },
        _ => Ok(()),
    }
}

impl Encode for Adc {
    fn encode<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        out.write_u8(opcode(self))?;
        encode_imm(self, out)
    }
}
