use code::{Prefix2, Prefix3, Rex, Opcode, Modrm, Sib, Disp, Imm, Instruction};

use mnemonic::instruction::Adc;
use mnemonic::operand::{
    self,
    Imm8, Imm16, Imm32,
    Sreg,
    Scale, IndexReg32, IndexRex32, IndexReg64, IndexRex64,
};

impl Default for Rex {
    #[inline]
    fn default() -> Self {
        Rex(0x40)
    }
}

impl Rex {
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

impl Modrm {
    #[inline]
    pub fn mode(self, mode: u8) -> Self {
        debug_assert_eq!(0, mode & !0b11);
        Modrm(self.0 & 0b00_111_111 | mode << 6)
    }

    #[inline]
    pub fn reg(self, reg: u8) -> Self {
        debug_assert_eq!(0, reg & !0b111);
        Modrm(self.0 & 0b11_000_111 | (reg << 3 & 0b00_111_000))
    }

    #[inline]
    pub fn rm(self, rm: u8) -> Self {
        debug_assert_eq!(0, rm & !0b111);
        Modrm(self.0 & 0b11_111_000 | (rm & 0b00_000_111))
    }
}

impl Sib {
    #[inline]
    pub fn scale(self, scale: u8) -> Self {
        debug_assert_eq!(0, scale & !0b11);
        Sib(self.0 & 0b00_111_111 | scale << 6)
    }

    #[inline]
    pub fn index(self, index: u8) -> Self {
        debug_assert_eq!(0, index & !0b111);
        Sib(self.0 & 0b11_000_111 | (index << 3 & 0b00_111_000))
    }

    #[inline]
    pub fn base(self, base: u8) -> Self {
        debug_assert_eq!(0, base & !0b111);
        Sib(self.0 & 0b11_111_000 | (base & 0b00_000_111))
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
    pub fn rex_r(mut self) -> Self {
        self.rex = Some(self.rex.unwrap_or_default().r());
        self
    }

    #[inline]
    pub fn rex_x(mut self) -> Self {
        self.rex = Some(self.rex.unwrap_or_default().x());
        self
    }

    #[inline]
    pub fn rex_b(mut self) -> Self {
        self.rex = Some(self.rex.unwrap_or_default().b());
        self
    }

    #[inline]
    pub fn modrm_mode(mut self, mode: u8) -> Self {
        self.modrm = Some(self.modrm.unwrap_or_default().mode(mode));
        self
    }

    #[inline]
    pub fn modrm_reg(mut self, reg: u8) -> Self {
        self.modrm = Some(self.modrm.unwrap_or_default().reg(reg));
        self
    }

    #[inline]
    pub fn modrm_rm(mut self, rm: u8) -> Self {
        self.modrm = Some(self.modrm.unwrap_or_default().rm(rm));
        self
    }

    #[inline]
    pub fn sib_scale(mut self, scale: u8) -> Self {
        self.sib = Some(self.sib.unwrap_or_default().scale(scale));
        self
    }

    #[inline]
    pub fn sib_index(mut self, index: u8) -> Self {
        self.sib = Some(self.sib.unwrap_or_default().index(index));
        self
    }

    #[inline]
    pub fn sib_base(mut self, base: u8) -> Self {
        self.sib = Some(self.sib.unwrap_or_default().base(base));
        self
    }
}

impl Instruction {
    #[inline]
    pub fn sreg(mut self, sreg: Sreg) -> Self {
        self.prefix2 = match sreg {
            Sreg::Cs => Some(Prefix2::Cs),
            Sreg::Ds => Some(Prefix2::Ds),
            Sreg::Ss => Some(Prefix2::Ss),
            Sreg::Es => Some(Prefix2::Es),
            Sreg::Fs => Some(Prefix2::Fs),
            Sreg::Gs => Some(Prefix2::Gs),
        };
        self
    }

    #[inline]
    pub fn scale(self, scale: Scale) -> Self {
        match scale {
            Scale::X1 => self.sib_scale(0),
            Scale::X2 => self.sib_scale(1),
            Scale::X4 => self.sib_scale(2),
            Scale::X8 => self.sib_scale(3),
        }
    }

    #[inline]
    pub fn index_reg32(self, index: IndexReg32) -> Self {
        match index {
            IndexReg32::Eax => self.sib_index(0),
            IndexReg32::Ecx => self.sib_index(1),
            IndexReg32::Edx => self.sib_index(2),
            IndexReg32::Ebx => self.sib_index(3),
            IndexReg32::Ebp => self.sib_index(5),
            IndexReg32::Esi => self.sib_index(6),
            IndexReg32::Edi => self.sib_index(7),
        }
    }

    #[inline]
    pub fn index_rex32(self, index: IndexRex32) -> Self {
        match index {
            IndexRex32::Eax => self.sib_index(0),
            IndexRex32::Ecx => self.sib_index(1),
            IndexRex32::Edx => self.sib_index(2),
            IndexRex32::Ebx => self.sib_index(3),
            IndexRex32::Ebp => self.sib_index(5),
            IndexRex32::Esi => self.sib_index(6),
            IndexRex32::Edi => self.sib_index(7),
            IndexRex32::R8d => self.rex_x().sib_index(0),
            IndexRex32::R9d => self.rex_x().sib_index(1),
            IndexRex32::R10d => self.rex_x().sib_index(2),
            IndexRex32::R11d => self.rex_x().sib_index(3),
            IndexRex32::R12d => self.rex_x().sib_index(4),
            IndexRex32::R13d => self.rex_x().sib_index(5),
            IndexRex32::R14d => self.rex_x().sib_index(6),
            IndexRex32::R15d => self.rex_x().sib_index(7),
        }
    }

    #[inline]
    pub fn index_reg64(self, index: IndexReg64) -> Self {
        match index {
            IndexReg64::Rax => self.sib_index(0),
            IndexReg64::Rcx => self.sib_index(1),
            IndexReg64::Rdx => self.sib_index(2),
            IndexReg64::Rbx => self.sib_index(3),
            IndexReg64::Rbp => self.sib_index(5),
            IndexReg64::Rsi => self.sib_index(6),
            IndexReg64::Rdi => self.sib_index(7),
        }
    }

    #[inline]
    pub fn index_rex64(self, index: IndexRex64) -> Self {
        match index {
            IndexRex64::Rax => self.sib_index(0),
            IndexRex64::Rcx => self.sib_index(1),
            IndexRex64::Rdx => self.sib_index(2),
            IndexRex64::Rbx => self.sib_index(3),
            IndexRex64::Rbp => self.sib_index(5),
            IndexRex64::Rsi => self.sib_index(6),
            IndexRex64::Rdi => self.sib_index(7),
            IndexRex64::R8 => self.rex_x().sib_index(0),
            IndexRex64::R9 => self.rex_x().sib_index(1),
            IndexRex64::R10 => self.rex_x().sib_index(2),
            IndexRex64::R11 => self.rex_x().sib_index(3),
            IndexRex64::R12 => self.rex_x().sib_index(4),
            IndexRex64::R13 => self.rex_x().sib_index(5),
            IndexRex64::R14 => self.rex_x().sib_index(6),
            IndexRex64::R15 => self.rex_x().sib_index(7),
        }
    }

    #[inline]
    pub fn disp(mut self, disp: operand::Disp) -> Self {
        match disp {
            operand::Disp::Disp8(disp) => {
                self.disp = Some(Disp::B1([disp as u8]));
            },
            operand::Disp::Disp32(disp) => {
                let bytes = [
                    disp as u8,
                    (disp >> 8) as u8,
                    (disp >> 16) as u8,
                    (disp >> 24) as u8,
                ];
                self.disp = Some(Disp::B4(bytes));
            },
        }
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
