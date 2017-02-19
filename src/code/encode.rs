use code::{Prefix3, Rex, Opcode, Imm, Instruction};

use mnemonic::instruction::Adc;
use mnemonic::operand::{Imm8, Imm16, Imm32};

impl Rex {
    #[inline]
    pub fn new() -> Self {
        Rex(0x40)
    }

    #[inline]
    pub fn w(self) -> Self {
        Rex(self.0 | 0b1000)
    }

    #[inline]
    pub fn r(self) -> Self {
        Rex(self.0 | 0b0100)
    }

    #[inline]
    pub fn x(self) -> Self {
        Rex(self.0 | 0b0010)
    }

    #[inline]
    pub fn b(self) -> Self {
        Rex(self.0 | 0b0001)
    }
}

impl Default for Rex {
    #[inline]
    fn default() -> Self {
        Rex::new()
    }
}

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
        self.rex = Some(self.rex.unwrap_or_default().w());
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
