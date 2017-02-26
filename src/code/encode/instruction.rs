use code::{Encode, Instruction};
use mnemonic::instruction::Adc;

impl Encode for Adc {
    fn encode(&self) -> Instruction {
        use self::Adc::*;
        match *self {
            AlImm8(imm)   => Instruction::opcode1(0x14).imm8(imm),
            AxImm16(imm)  => Instruction::opcode1(0x15).imm16(imm).oper16(),
            EaxImm32(imm) => Instruction::opcode1(0x15).imm32(imm),
            RaxImm32(imm) => Instruction::opcode1(0x15).imm32(imm).rex_w(),

            Rm8LImm8(rm, imm)  => Instruction::opcode1(0x80).reg(2).rm8l(rm).imm8(imm),
            Rm8Imm8(rm, imm)   => Instruction::opcode1(0x80).reg(2).rm8(rm).imm8(imm),
            Rm16Imm16(rm, imm) => Instruction::opcode1(0x81).reg(2).rm16(rm).imm16(imm).oper16(),
            Rm32Imm32(rm, imm) => Instruction::opcode1(0x81).reg(2).rm32(rm).imm32(imm),
            Rm64Imm32(rm, imm) => Instruction::opcode1(0x81).reg(2).rm64(rm).imm32(imm).rex_w(),

            Rm16Imm8(rm, imm) => Instruction::opcode1(0x83).reg(2).rm16(rm).imm8(imm).oper16(),
            Rm32Imm8(rm, imm) => Instruction::opcode1(0x83).reg(2).rm32(rm).imm8(imm),
            Rm64Imm8(rm, imm) => Instruction::opcode1(0x83).reg(2).rm64(rm).imm8(imm).rex_w(),

            Rm8LR8L(rm, r) => Instruction::opcode1(0x10).rm8l(rm).reg(r),
            Rm8R8(rm, r)   => Instruction::opcode1(0x10).rm8(rm).reg(r),
            Rm16R16(rm, r) => Instruction::opcode1(0x11).rm16(rm).reg(r).oper16(),
            Rm32R32(rm, r) => Instruction::opcode1(0x11).rm32(rm).reg(r),
            Rm64R64(rm, r) => Instruction::opcode1(0x11).rm64(rm).reg(r).rex_w(),

            R8LRm8L(r, rm) => Instruction::opcode1(0x12).reg(r).rm8l(rm),
            R8Rm8(r, rm)   => Instruction::opcode1(0x12).reg(r).rm8(rm),
            R16Rm16(r, rm) => Instruction::opcode1(0x13).reg(r).rm16(rm).oper16(),
            R32Rm32(r, rm) => Instruction::opcode1(0x13).reg(r).rm32(rm),
            R64Rm64(r, rm) => Instruction::opcode1(0x13).reg(r).rm64(rm).rex_w(),
        }
    }
}
