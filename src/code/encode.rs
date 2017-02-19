use code::{Prefix3, Rex, Opcode, Imm, Instruction};

use mnemonic::instruction::Adc;
use mnemonic::operand::{Imm8, Imm16, Imm32};

const REX: u8   = 0b0100_0000;
const REX_W: u8 = 0b0100_1000;
const REX_R: u8 = 0b0100_0100;
const REX_X: u8 = 0b0100_0010;
const REX_B: u8 = 0b0100_0001;

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
    pub fn rex_w(mut self) -> Self {
        self.rex = Some(self.rex.map_or(Rex(REX_W), |Rex(x)| Rex(x | REX_W)));
        self
    }

    #[inline]
    pub fn imm8(mut self, imm: Imm8) -> Self {
        self.imm = Some(Imm::B1([imm.0]));
        self
    }

    #[inline]
    pub fn imm16(mut self, imm: Imm16) -> Self {
        let bytes = [
            imm.0 as u8,
            (imm.0 >> 8) as u8,
        ];
        self.imm = Some(Imm::B2(bytes));
        self
    }

    #[inline]
    pub fn imm32(mut self, imm: Imm32) -> Self {
        let bytes = [
            imm.0 as u8,
            (imm.0 >> 8) as u8,
            (imm.0 >> 16) as u8,
            (imm.0 >> 24) as u8,
        ];
        self.imm = Some(Imm::B4(bytes));
        self
    }
}

impl<'a> From<&'a Adc> for Instruction {
    fn from(adc: &'a Adc) -> Self {
        match *adc {
            Adc::AlImm8(imm)   => Instruction::opcode1(0x14).imm8(imm),
            Adc::AxImm16(imm)  => Instruction::opcode1(0x15).operand_size().imm16(imm),
            Adc::EaxImm32(imm) => Instruction::opcode1(0x15).imm32(imm),
            Adc::RaxImm32(imm) => Instruction::opcode1(0x15).rex_w().imm32(imm),
            _ => unimplemented!(),
        }
    }
}
