use code::{Prefix2, Prefix3, Prefix4, Rex, Opcode, Modrm, Sib, Disp, Imm, Instruction};

use mnemonic::instruction::Adc;
use mnemonic::operand::{
    self,
    Imm8, Imm16, Imm32,
    Reg8, Rex8, Reg16, Rex16, Reg32, Rex32, Reg64, Rex64,
    Scale, IndexReg32, IndexRex32, IndexReg64, IndexRex64,
    Sreg, Offset, Memory, Mem, Mex,
    Rm8,
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
    pub fn rex(mut self) -> Self {
        self.rex = Some(self.rex.unwrap_or_default());
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

    #[inline]
    pub fn disp8(mut self, disp: i8) -> Self {
        self.disp = Some(Disp::B1([disp as u8]));
        self
    }

    #[inline]
    pub fn disp32(mut self, disp: i32) -> Self {
        let bytes = [
            disp as u8,
            (disp >> 8) as u8,
            (disp >> 16) as u8,
            (disp >> 24) as u8,
        ];
        self.disp = Some(Disp::B4(bytes));
        self
    }
}

impl Instruction {
    #[inline]
    pub fn operand_size(mut self) -> Self {
        self.prefix3 = Some(Prefix3::OperandSize);
        self
    }

    #[inline]
    pub fn address_size(mut self) -> Self {
        self.prefix4 = Some(Prefix4::AddressSize);
        self
    }

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
    pub fn reg8(self, reg: Reg8) -> Self {
        match reg {
            Reg8::Al => self.modrm_reg(0),
            Reg8::Cl => self.modrm_reg(1),
            Reg8::Dl => self.modrm_reg(2),
            Reg8::Bl => self.modrm_reg(3),
            Reg8::Ah => self.modrm_reg(4),
            Reg8::Ch => self.modrm_reg(5),
            Reg8::Dh => self.modrm_reg(6),
            Reg8::Bh => self.modrm_reg(7),
        }
    }

    #[inline]
    pub fn rex8(self, rex: Rex8) -> Self {
        match rex {
            Rex8::Al => self.modrm_reg(0),
            Rex8::Cl => self.modrm_reg(1),
            Rex8::Dl => self.modrm_reg(2),
            Rex8::Bl => self.modrm_reg(3),
            Rex8::Spl => self.rex().modrm_reg(4),
            Rex8::Bpl => self.rex().modrm_reg(5),
            Rex8::Sil => self.rex().modrm_reg(6),
            Rex8::Dil => self.rex().modrm_reg(7),
            Rex8::R8l => self.rex_r().modrm_reg(0),
            Rex8::R9l => self.rex_r().modrm_reg(1),
            Rex8::R10l => self.rex_r().modrm_reg(2),
            Rex8::R11l => self.rex_r().modrm_reg(3),
            Rex8::R12l => self.rex_r().modrm_reg(4),
            Rex8::R13l => self.rex_r().modrm_reg(5),
            Rex8::R14l => self.rex_r().modrm_reg(6),
            Rex8::R15l => self.rex_r().modrm_reg(7),
        }
    }

    #[inline]
    pub fn reg16(self, reg: Reg16) -> Self {
        match reg {
            Reg16::Ax => self.modrm_reg(0),
            Reg16::Cx => self.modrm_reg(1),
            Reg16::Dx => self.modrm_reg(2),
            Reg16::Bx => self.modrm_reg(3),
            Reg16::Sp => self.modrm_reg(4),
            Reg16::Bp => self.modrm_reg(5),
            Reg16::Si => self.modrm_reg(6),
            Reg16::Di => self.modrm_reg(7),
        }
    }

    #[inline]
    pub fn rex16(self, rex: Rex16) -> Self {
        match rex {
            Rex16::Ax => self.modrm_reg(0),
            Rex16::Cx => self.modrm_reg(1),
            Rex16::Dx => self.modrm_reg(2),
            Rex16::Bx => self.modrm_reg(3),
            Rex16::Sp => self.modrm_reg(4),
            Rex16::Bp => self.modrm_reg(5),
            Rex16::Si => self.modrm_reg(6),
            Rex16::Di => self.modrm_reg(7),
            Rex16::R8w => self.rex_r().modrm_reg(0),
            Rex16::R9w => self.rex_r().modrm_reg(1),
            Rex16::R10w => self.rex_r().modrm_reg(2),
            Rex16::R11w => self.rex_r().modrm_reg(3),
            Rex16::R12w => self.rex_r().modrm_reg(4),
            Rex16::R13w => self.rex_r().modrm_reg(5),
            Rex16::R14w => self.rex_r().modrm_reg(6),
            Rex16::R15w => self.rex_r().modrm_reg(7),
        }
    }

    #[inline]
    pub fn reg32(self, reg: Reg32) -> Self {
        match reg {
            Reg32::Eax => self.modrm_reg(0),
            Reg32::Ecx => self.modrm_reg(1),
            Reg32::Edx => self.modrm_reg(2),
            Reg32::Ebx => self.modrm_reg(3),
            Reg32::Esp => self.modrm_reg(4),
            Reg32::Ebp => self.modrm_reg(5),
            Reg32::Esi => self.modrm_reg(6),
            Reg32::Edi => self.modrm_reg(7),
        }
    }

    #[inline]
    pub fn rex32(self, rex: Rex32) -> Self {
        match rex {
            Rex32::Eax => self.modrm_reg(0),
            Rex32::Ecx => self.modrm_reg(1),
            Rex32::Edx => self.modrm_reg(2),
            Rex32::Ebx => self.modrm_reg(3),
            Rex32::Esp => self.modrm_reg(4),
            Rex32::Ebp => self.modrm_reg(5),
            Rex32::Esi => self.modrm_reg(6),
            Rex32::Edi => self.modrm_reg(7),
            Rex32::R8d => self.rex_r().modrm_reg(0),
            Rex32::R9d => self.rex_r().modrm_reg(1),
            Rex32::R10d => self.rex_r().modrm_reg(2),
            Rex32::R11d => self.rex_r().modrm_reg(3),
            Rex32::R12d => self.rex_r().modrm_reg(4),
            Rex32::R13d => self.rex_r().modrm_reg(5),
            Rex32::R14d => self.rex_r().modrm_reg(6),
            Rex32::R15d => self.rex_r().modrm_reg(7),
        }
    }

    #[inline]
    pub fn reg64(self, reg: Reg64) -> Self {
        match reg {
            Reg64::Rax => self.modrm_reg(0),
            Reg64::Rcx => self.modrm_reg(1),
            Reg64::Rdx => self.modrm_reg(2),
            Reg64::Rbx => self.modrm_reg(3),
            Reg64::Rsp => self.modrm_reg(4),
            Reg64::Rbp => self.modrm_reg(5),
            Reg64::Rsi => self.modrm_reg(6),
            Reg64::Rdi => self.modrm_reg(7),
        }
    }

    #[inline]
    pub fn rex64(self, rex: Rex64) -> Self {
        match rex {
            Rex64::Rax => self.modrm_reg(0),
            Rex64::Rcx => self.modrm_reg(1),
            Rex64::Rdx => self.modrm_reg(2),
            Rex64::Rbx => self.modrm_reg(3),
            Rex64::Rsp => self.modrm_reg(4),
            Rex64::Rbp => self.modrm_reg(5),
            Rex64::Rsi => self.modrm_reg(6),
            Rex64::Rdi => self.modrm_reg(7),
            Rex64::R8 => self.rex_r().modrm_reg(0),
            Rex64::R9 => self.rex_r().modrm_reg(1),
            Rex64::R10 => self.rex_r().modrm_reg(2),
            Rex64::R11 => self.rex_r().modrm_reg(3),
            Rex64::R12 => self.rex_r().modrm_reg(4),
            Rex64::R13 => self.rex_r().modrm_reg(5),
            Rex64::R14 => self.rex_r().modrm_reg(6),
            Rex64::R15 => self.rex_r().modrm_reg(7),
        }
    }

    #[inline]
    pub fn rm_reg8(self, reg: Reg8) -> Self {
        match reg {
            Reg8::Al => self.modrm_rm(0),
            Reg8::Cl => self.modrm_rm(1),
            Reg8::Dl => self.modrm_rm(2),
            Reg8::Bl => self.modrm_rm(3),
            Reg8::Ah => self.modrm_rm(4),
            Reg8::Ch => self.modrm_rm(5),
            Reg8::Dh => self.modrm_rm(6),
            Reg8::Bh => self.modrm_rm(7),
        }
    }

    #[inline]
    pub fn rm_rex8(self, rex: Rex8) -> Self {
        match rex {
            Rex8::Al => self.modrm_rm(0),
            Rex8::Cl => self.modrm_rm(1),
            Rex8::Dl => self.modrm_rm(2),
            Rex8::Bl => self.modrm_rm(3),
            Rex8::Spl => self.rex().modrm_rm(4),
            Rex8::Bpl => self.rex().modrm_rm(5),
            Rex8::Sil => self.rex().modrm_rm(6),
            Rex8::Dil => self.rex().modrm_rm(7),
            Rex8::R8l => self.rex_b().modrm_rm(0),
            Rex8::R9l => self.rex_b().modrm_rm(1),
            Rex8::R10l => self.rex_b().modrm_rm(2),
            Rex8::R11l => self.rex_b().modrm_rm(3),
            Rex8::R12l => self.rex_b().modrm_rm(4),
            Rex8::R13l => self.rex_b().modrm_rm(5),
            Rex8::R14l => self.rex_b().modrm_rm(6),
            Rex8::R15l => self.rex_b().modrm_rm(7),
        }
    }

    #[inline]
    pub fn rm_reg16(self, reg: Reg16) -> Self {
        match reg {
            Reg16::Ax => self.modrm_rm(0),
            Reg16::Cx => self.modrm_rm(1),
            Reg16::Dx => self.modrm_rm(2),
            Reg16::Bx => self.modrm_rm(3),
            Reg16::Sp => self.modrm_rm(4),
            Reg16::Bp => self.modrm_rm(5),
            Reg16::Si => self.modrm_rm(6),
            Reg16::Di => self.modrm_rm(7),
        }
    }

    #[inline]
    pub fn rm_rex16(self, rex: Rex16) -> Self {
        match rex {
            Rex16::Ax => self.modrm_rm(0),
            Rex16::Cx => self.modrm_rm(1),
            Rex16::Dx => self.modrm_rm(2),
            Rex16::Bx => self.modrm_rm(3),
            Rex16::Sp => self.modrm_rm(4),
            Rex16::Bp => self.modrm_rm(5),
            Rex16::Si => self.modrm_rm(6),
            Rex16::Di => self.modrm_rm(7),
            Rex16::R8w => self.rex_b().modrm_rm(0),
            Rex16::R9w => self.rex_b().modrm_rm(1),
            Rex16::R10w => self.rex_b().modrm_rm(2),
            Rex16::R11w => self.rex_b().modrm_rm(3),
            Rex16::R12w => self.rex_b().modrm_rm(4),
            Rex16::R13w => self.rex_b().modrm_rm(5),
            Rex16::R14w => self.rex_b().modrm_rm(6),
            Rex16::R15w => self.rex_b().modrm_rm(7),
        }
    }

    #[inline]
    pub fn rm_reg32(self, reg: Reg32) -> Self {
        match reg {
            Reg32::Eax => self.modrm_rm(0),
            Reg32::Ecx => self.modrm_rm(1),
            Reg32::Edx => self.modrm_rm(2),
            Reg32::Ebx => self.modrm_rm(3),
            Reg32::Esp => self.modrm_rm(4),
            Reg32::Ebp => self.modrm_rm(5),
            Reg32::Esi => self.modrm_rm(6),
            Reg32::Edi => self.modrm_rm(7),
        }
    }

    #[inline]
    pub fn rm_rex32(self, rex: Rex32) -> Self {
        match rex {
            Rex32::Eax => self.modrm_rm(0),
            Rex32::Ecx => self.modrm_rm(1),
            Rex32::Edx => self.modrm_rm(2),
            Rex32::Ebx => self.modrm_rm(3),
            Rex32::Esp => self.modrm_rm(4),
            Rex32::Ebp => self.modrm_rm(5),
            Rex32::Esi => self.modrm_rm(6),
            Rex32::Edi => self.modrm_rm(7),
            Rex32::R8d => self.rex_b().modrm_rm(0),
            Rex32::R9d => self.rex_b().modrm_rm(1),
            Rex32::R10d => self.rex_b().modrm_rm(2),
            Rex32::R11d => self.rex_b().modrm_rm(3),
            Rex32::R12d => self.rex_b().modrm_rm(4),
            Rex32::R13d => self.rex_b().modrm_rm(5),
            Rex32::R14d => self.rex_b().modrm_rm(6),
            Rex32::R15d => self.rex_b().modrm_rm(7),
        }
    }

    #[inline]
    pub fn rm_reg64(self, reg: Reg64) -> Self {
        match reg {
            Reg64::Rax => self.modrm_rm(0),
            Reg64::Rcx => self.modrm_rm(1),
            Reg64::Rdx => self.modrm_rm(2),
            Reg64::Rbx => self.modrm_rm(3),
            Reg64::Rsp => self.modrm_rm(4),
            Reg64::Rbp => self.modrm_rm(5),
            Reg64::Rsi => self.modrm_rm(6),
            Reg64::Rdi => self.modrm_rm(7),
        }
    }

    #[inline]
    pub fn rm_rex64(self, rex: Rex64) -> Self {
        match rex {
            Rex64::Rax => self.modrm_rm(0),
            Rex64::Rcx => self.modrm_rm(1),
            Rex64::Rdx => self.modrm_rm(2),
            Rex64::Rbx => self.modrm_rm(3),
            Rex64::Rsp => self.modrm_rm(4),
            Rex64::Rbp => self.modrm_rm(5),
            Rex64::Rsi => self.modrm_rm(6),
            Rex64::Rdi => self.modrm_rm(7),
            Rex64::R8 => self.rex_b().modrm_rm(0),
            Rex64::R9 => self.rex_b().modrm_rm(1),
            Rex64::R10 => self.rex_b().modrm_rm(2),
            Rex64::R11 => self.rex_b().modrm_rm(3),
            Rex64::R12 => self.rex_b().modrm_rm(4),
            Rex64::R13 => self.rex_b().modrm_rm(5),
            Rex64::R14 => self.rex_b().modrm_rm(6),
            Rex64::R15 => self.rex_b().modrm_rm(7),
        }
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
    pub fn base_reg64(self, base: Reg64) -> Self {
        match base {
            Reg64::Rax => self.sib_base(0),
            Reg64::Rcx => self.sib_base(1),
            Reg64::Rdx => self.sib_base(2),
            Reg64::Rbx => self.sib_base(3),
            Reg64::Rsp => self.sib_base(4),
            Reg64::Rbp => self.sib_base(5),
            Reg64::Rsi => self.sib_base(6),
            Reg64::Rdi => self.sib_base(7),
        }
    }

    #[inline]
    pub fn offset_mem32(self, offset: Offset<Reg32, IndexReg32>) -> Self {
        unimplemented!()
    }

    #[inline]
    pub fn offset_mem64(self, offset: Offset<Reg64, IndexReg64>) -> Self {
        match offset {
            Offset::Disp(disp) => {
                self.modrm_mode(0b00)
                    .modrm_rm(0b100)
                    .sib_scale(0b00)
                    .sib_index(0b100)
                    .sib_base(0b101)
                    .disp32(disp)
            },
            Offset::Index(index, scale) => {
                self.modrm_mode(0b00)
                    .modrm_rm(0b100)
                    .scale(scale)
                    .index_reg64(index)
                    .sib_base(0b101)
                    .disp32(0)
            },
            Offset::IndexDisp(index, scale, disp) => {
                self.modrm_mode(0b00)
                    .modrm_rm(0b100)
                    .scale(scale)
                    .index_reg64(index)
                    .sib_base(0b101)
                    .disp32(disp)
            },
            Offset::Base(Reg64::Rsp) => {
                self.modrm_mode(0b00)
                    .modrm_rm(0b100)
                    .sib_index(0b100)
                    .base_reg64(Reg64::Rsp)
            },
            Offset::Base(Reg64::Rbp) => {
                self.modrm_mode(0b01)
                    .rm_reg64(Reg64::Rbp)
                    .disp8(0)
            },
            Offset::Base(base) => {
                self.modrm_mode(0b00)
                    .rm_reg64(base)
            },
            Offset::BaseDisp(Reg64::Rsp, operand::Disp::Disp8(disp)) => {
                self.modrm_mode(0b01)
                    .modrm_rm(0b100)
                    .sib_index(0b100)
                    .base_reg64(Reg64::Rsp)
                    .disp8(disp)
            },
            Offset::BaseDisp(base, operand::Disp::Disp8(disp)) => {
                self.modrm_mode(0b01)
                    .rm_reg64(base)
                    .disp8(disp)
            },
            Offset::BaseDisp(Reg64::Rsp, operand::Disp::Disp32(disp)) => {
                self.modrm_mode(0b10)
                    .modrm_rm(0b100)
                    .sib_index(0b100)
                    .base_reg64(Reg64::Rsp)
                    .disp32(disp)
            },
            Offset::BaseDisp(base, operand::Disp::Disp32(disp)) => {
                self.modrm_mode(0b10)
                    .rm_reg64(base)
                    .disp32(disp)
            },
            Offset::BaseIndex(Reg64::Rbp, index, scale) => {
                self.modrm_mode(0b01)
                    .modrm_rm(0b100)
                    .scale(scale)
                    .index_reg64(index)
                    .base_reg64(Reg64::Rbp)
                    .disp8(0)
            },
            Offset::BaseIndex(base, index, scale) => {
                self.modrm_mode(0b00)
                    .modrm_rm(0b100)
                    .scale(scale)
                    .index_reg64(index)
                    .base_reg64(base)
            },
            Offset::RipDisp(disp) => {
                self.modrm_mode(0b00)
                    .modrm_rm(0b101)
                    .disp32(disp)
            },
            _ => unimplemented!(),
        }
    }

    #[inline]
    pub fn mem(self, mem: Mem) -> Self {
        match mem {
            Memory::Offset32(None, offset) => {
                self.address_size().offset_mem32(offset)
            },
            Memory::Offset32(Some(sreg), offset) => {
                self.address_size().sreg(sreg).offset_mem32(offset)
            },
            Memory::Offset64(None, offset) => {
                self.offset_mem64(offset)
            },
            Memory::Offset64(Some(sreg), offset) => {
                self.sreg(sreg).offset_mem64(offset)
            },
        }
    }

    #[inline]
    pub fn mex(self, mex: Mex) -> Self {
        unimplemented!()
    }

    #[inline]
    pub fn rm8(self, rm8: Rm8) -> Self {
        match rm8 {
            Rm8::Reg8(reg) => self.modrm_mode(0b11).rm_reg8(reg),
            Rm8::Rex8(rex) => self.modrm_mode(0b11).rm_rex8(rex),
            Rm8::Mem8(mem) => self.mem(mem),
            Rm8::Mex8(mex) => self.mex(mex),
        }
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
            Adc::AlImm8(imm)       => Instruction::opcode1(0x14).imm8(imm),
            Adc::AxImm16(imm)      => Instruction::opcode1(0x15).operand_size().imm16(imm),
            Adc::EaxImm32(imm)     => Instruction::opcode1(0x15).imm32(imm),
            Adc::RaxImm32(imm)     => Instruction::opcode1(0x15).rex_w().imm32(imm),
            Adc::Rm8Imm8(rm8, imm) => Instruction::opcode1(0x80).modrm_reg(2).rm8(rm8).imm8(imm),
            _ => unimplemented!(),
        }
    }
}
