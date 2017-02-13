use encode::{Encode, prefix};
use encode::operand::Prefix;
use mnemonic::instruction::Adc;
use output::Output;

use self::Adc::*;

impl Encode for Adc {
    fn encode_prefix2<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        match *self {
            Rm8Imm8(ref rm8, _) => rm8.encode_prefix2(out),
            Rm16Imm16(ref rm16, _) | Rm16Imm8(ref rm16, _) => rm16.encode_prefix2(out),
            Rm32Imm32(ref rm32, _) | Rm32Imm8(ref rm32, _) => rm32.encode_prefix2(out),
            Rm64Imm32(ref rm64, _) | Rm64Imm8(ref rm64, _) => rm64.encode_prefix2(out),
            Rm8R8(ref rm8r8) => rm8r8.encode_prefix2(out),
            Rm16R16(ref rm16r16) => rm16r16.encode_prefix2(out),
            Rm32R32(ref rm32r32) => rm32r32.encode_prefix2(out),
            Rm64R64(ref rm64r64) => rm64r64.encode_prefix2(out),
            R8Rm8(ref r8rm8) => r8rm8.encode_prefix2(out),
            R16Rm16(ref r16rm16) => r16rm16.encode_prefix2(out),
            R32Rm32(ref r32rm32) => r32rm32.encode_prefix2(out),
            R64Rm64(ref r64rm64) => r64rm64.encode_prefix2(out),
            _ => Ok(()),
        }
    }

    fn encode_prefix3<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        match *self {
            AxImm16(..) | Rm16Imm16(..) | Rm16Imm8(..) | Rm16R16(..) | R16Rm16(..) => {
                out.write_u8(prefix::OPERAND_SIZE)
            },
            _ => Ok(()),
        }
    }

    fn encode_prefix4<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        match *self {
            Rm8Imm8(ref rm8, _) => rm8.encode_prefix4(out),
            Rm16Imm16(ref rm16, _) | Rm16Imm8(ref rm16, _) => rm16.encode_prefix4(out),
            Rm32Imm32(ref rm32, _) | Rm32Imm8(ref rm32, _) => rm32.encode_prefix4(out),
            Rm64Imm32(ref rm64, _) | Rm64Imm8(ref rm64, _) => rm64.encode_prefix4(out),
            Rm8R8(ref rm8r8) => rm8r8.encode_prefix4(out),
            Rm16R16(ref rm16r16) => rm16r16.encode_prefix4(out),
            Rm32R32(ref rm32r32) => rm32r32.encode_prefix4(out),
            Rm64R64(ref rm64r64) => rm64r64.encode_prefix4(out),
            R8Rm8(ref r8rm8) => r8rm8.encode_prefix4(out),
            R16Rm16(ref r16rm16) => r16rm16.encode_prefix4(out),
            R32Rm32(ref r32rm32) => r32rm32.encode_prefix4(out),
            R64Rm64(ref r64rm64) => r64rm64.encode_prefix4(out),
            _ => Ok(()),
        }
    }

    fn encode_opcode<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        match *self {
            AlImm8(..)                                    => out.write_u8(0x14),
            AxImm16(..) | EaxImm32(..) | RaxImm32(..)     => out.write_u8(0x15),
            Rm8Imm8(..)                                   => out.write_u8(0x80),
            Rm16Imm16(..) | Rm32Imm32(..) | Rm64Imm32(..) => out.write_u8(0x81),
            Rm16Imm8(..) | Rm32Imm8(..) | Rm64Imm8(..)    => out.write_u8(0x83),
            Rm8R8(..)                                     => out.write_u8(0x10),
            Rm16R16(..) | Rm32R32(..) | Rm64R64(..)       => out.write_u8(0x11),
            R8Rm8(..)                                     => out.write_u8(0x12),
            R16Rm16(..) | R32Rm32(..) | R64Rm64(..)       => out.write_u8(0x13),
        }
    }

    fn encode_imm<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        match *self {
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
}
