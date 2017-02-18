use code::{Prefix3, Opcode, Imm, Instruction};

use mnemonic::instruction::Adc;
use mnemonic::operand::{Imm8, Imm16, Imm32};

impl Instruction {
    #[inline]
    pub fn opcode1(a: u8) -> Self {
        Instruction {
            prefix1: None,
            prefix2: None,
            prefix3: None,
            prefix4: None,
            rex: None,
            opcode: Opcode::B1([a]),
            modrm: None,
            sib: None,
            disp: None,
            imm: None,
        }
    }

    #[inline]
    pub fn operand_size(mut self) -> Self {
        self.prefix3 = Some(Prefix3::OperandSize);
        self
    }

    #[inline]
    pub fn imm8(mut self, imm: Imm8) -> Self {
        self.imm = Some(Imm::B1([imm]));
        self
    }

    #[inline]
    pub fn imm16(mut self, imm: Imm16) -> Self {
        self.imm = Some(Imm::B2([(imm & 0xff) as u8, (imm >> 8) as u8]));
        self
    }

    #[inline]
    pub fn imm32(mut self, imm: Imm32) -> Self {
        self.imm = Some(Imm::B4([
            (imm & 0xff) as u8,
            (imm >> 8 & 0xff) as u8,
            (imm >> 16 & 0xff) as u8,
            (imm >> 24) as u8,
        ]));
        self
    }
}

impl<'a> From<&'a Adc> for Instruction {
    fn from(adc: &'a Adc) -> Self {
        use self::Adc::*;
        match *adc {
            AlImm8(imm) => Instruction::opcode1(0x14).imm8(imm),
            AxImm16(imm) => Instruction::opcode1(0x15).operand_size().imm16(imm),
            EaxImm32(imm) => Instruction::opcode1(0x15).imm32(imm),
            _ => unimplemented!(),
        }
    }
}
