use code::{Encode, Instruction};
use mnemonic::instruction::Adc;

impl Encode for Adc {
    fn encode(&self) -> Instruction {
        match *self {
            Adc::AlImm8(imm) =>
                Instruction::opcode1(0x14).imm8(imm),
            Adc::AxImm16(imm) =>
                Instruction::opcode1(0x15).operand16().imm16(imm),
            Adc::EaxImm32(imm) =>
                Instruction::opcode1(0x15).imm32(imm),
            Adc::RaxImm32(imm) =>
                Instruction::opcode1(0x15).rex_w().imm32(imm),

            Adc::Rm8LImm8(rm, imm) =>
                Instruction::opcode1(0x80).reg(2).rm8l(rm).imm8(imm),
            Adc::Rm8Imm8(rm, imm) =>
                Instruction::opcode1(0x80).reg(2).rm8(rm).imm8(imm),
            Adc::Rm16Imm16(rm, imm) =>
                Instruction::opcode1(0x81).operand16().reg(2).rm16(rm).imm16(imm),
            Adc::Rm32Imm32(rm, imm) =>
                Instruction::opcode1(0x81).reg(2).rm32(rm).imm32(imm),
            Adc::Rm64Imm32(rm, imm) =>
                Instruction::opcode1(0x81).rex_w().reg(2).rm64(rm).imm32(imm),

            Adc::Rm16Imm8(rm, imm) =>
                Instruction::opcode1(0x83).operand16().reg(2).rm16(rm).imm8(imm),
            Adc::Rm32Imm8(rm, imm) =>
                Instruction::opcode1(0x83).reg(2).rm32(rm).imm8(imm),
            Adc::Rm64Imm8(rm, imm) =>
                Instruction::opcode1(0x83).rex_w().reg(2).rm64(rm).imm8(imm),

            Adc::Rm8LR8L(rm, r) =>
                Instruction::opcode1(0x10).rm8l(rm).reg(r),
            Adc::Rm8R8(rm, r) =>
                Instruction::opcode1(0x10).rm8(rm).reg(r),
            Adc::Rm16R16(rm, r) =>
                Instruction::opcode1(0x11).operand16().rm16(rm).reg(r),
            Adc::Rm32R32(rm, r) =>
                Instruction::opcode1(0x11).rm32(rm).reg(r),
            Adc::Rm64R64(rm, r) =>
                Instruction::opcode1(0x11).rex_w().rm64(rm).reg(r),

            Adc::R8LRm8L(r, rm) =>
                Instruction::opcode1(0x12).reg(r).rm8l(rm),
            Adc::R8Rm8(r, rm) =>
                Instruction::opcode1(0x12).reg(r).rm8(rm),
            Adc::R16Rm16(r, rm) =>
                Instruction::opcode1(0x13).operand16().reg(r).rm16(rm),
            Adc::R32Rm32(r, rm) =>
                Instruction::opcode1(0x13).reg(r).rm32(rm),
            Adc::R64Rm64(r, rm) =>
                Instruction::opcode1(0x13).rex_w().reg(r).rm64(rm),
        }
    }
}
