use code::{Prefix2, Prefix3, Prefix4, Opcode, Disp, Imm, Instruction};
use code::Register;

use mnemonic::instruction::Adc;
use mnemonic::operand::{
    self,
    Imm8, Imm16, Imm32,
    Scale, Sreg, Offset, Memory,
    Rm8, Rm16, Rm32, Rm64,
    Rm8R8, Rm16R16, Rm32R32, Rm64R64,
    R8Rm8, R16Rm16, R32Rm32, R64Rm64,
};

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
    pub fn reg<R>(mut self, reg: R) -> Self where R: Register {
        if reg.force_rex() { self = self.rex() }
        match reg.rex_index() {
            (true, index) => self.rex_r().modrm_reg(index),
            (false, index) => self.modrm_reg(index),
        }
    }

    #[inline]
    pub fn rm<R>(mut self, reg: R) -> Self where R: Register {
        if reg.force_rex() { self = self.rex() }
        match reg.rex_index() {
            (true, index) => self.rex_b().modrm_rm(index),
            (false, index) => self.modrm_rm(index),
        }
    }

    #[inline]
    pub fn rm_reg<R>(self, reg: R) -> Self where R: Register {
        self.modrm_mode(0b11)
            .rm(reg)
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
    pub fn index<R>(self, reg: R) -> Self where R: Register {
        match reg.rex_index() {
            (true, index) => self.rex_x().sib_index(index),
            (false, index) => self.sib_index(index),
        }
    }

    #[inline]
    pub fn base<R>(self, reg: R) -> Self where R: Register {
        match reg.rex_index() {
            (true, index) => self.rex_b().sib_base(index),
            (false, index) => self.sib_base(index),
        }
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

    fn offset_index_disp<Index>(self, index: Index, scale: Scale, disp: i32) -> Self
    where Index: Register {
        self.modrm_mode(0b00)
            .modrm_rm(0b100)
            .sib_base(0b101)
            .scale(scale)
            .index(index)
            .disp32(disp)
    }

    fn offset_disp(self, disp: operand::Disp) -> Self {
        match disp {
            operand::Disp::Disp8(disp) => {
                self.modrm_mode(0b01)
                    .disp8(disp)
            },
            operand::Disp::Disp32(disp) => {
                self.modrm_mode(0b10)
                    .disp32(disp)
            },
        }
    }

    fn offset_base_index<Base, Index>(mut self, base: Base, index: Index, scale: Scale) -> Self
    where Base: Register, Index: Register {
        if let (_, 0b101) = base.rex_index() { self = self.offset_disp(operand::Disp::Disp8(0)) }
        self.modrm_rm(0b100)
            .scale(scale)
            .index(index)
            .base(base)
    }

    fn offset_base<Base>(self, base: Base) -> Self where Base: Register {
        match base.rex_index() {
            (_, 0b100) => self.offset_base_index(base, 0b100, Scale::X1),
            (_, 0b101) => self.rm(base).offset_disp(operand::Disp::Disp8(0)),
            _ => self.rm(base),
        }
    }

    fn offset_rip_disp(self, disp: i32) -> Self {
        self.modrm_mode(0b00)
            .modrm_rm(0b101)
            .disp32(disp)
    }

    #[inline]
    pub fn offset<Base, Index>(self, offset: Offset<Base, Index>) -> Self
    where Base: Register, Index: Register {
        match offset {
            Offset::Disp(disp) => {
                self.offset_index_disp(0b100, Scale::X1, disp)
            },
            Offset::Index(index, scale) => {
                self.offset_index_disp(index, scale, 0)
            },
            Offset::IndexDisp(index, scale, disp) => {
                self.offset_index_disp(index, scale, disp)
            },
            Offset::Base(base) => {
                self.offset_base(base)
            },
            Offset::BaseDisp(base, disp) => {
                self.offset_base(base)
                    .offset_disp(disp)
            },
            Offset::BaseIndex(base, index, scale) => {
                self.offset_base_index(base, index, scale)
            },
            Offset::BaseIndexDisp(base, index, scale, disp) => {
                self.offset_base_index(base, index, scale)
                    .offset_disp(disp)
            },
            Offset::RipDisp(disp) => {
                self.offset_rip_disp(disp)
            },
        }
    }

    #[inline]
    pub fn memory<Base32, Index32, Base64, Index64>(
        self,
        memory: Memory<Base32, Index32, Base64, Index64>,
    ) -> Self where Base32: Register, Index32: Register, Base64: Register, Index64: Register {
        match memory {
            Memory::Offset32(None, offset) => {
                self.address_size()
                    .offset(offset)
            },
            Memory::Offset32(Some(sreg), offset) => {
                self.address_size()
                    .sreg(sreg)
                    .offset(offset)
            },
            Memory::Offset64(None, offset) => {
                self.offset(offset)
            },
            Memory::Offset64(Some(sreg), offset) => {
                self.sreg(sreg)
                    .offset(offset)
            },
        }
    }

    #[inline]
    pub fn rm8(self, rm: Rm8) -> Self {
        match rm {
            Rm8::Reg8(reg) => self.rm_reg(reg),
            Rm8::Rex8(rex) => self.rm_reg(rex),
            Rm8::Mem8(mem) => self.memory(mem),
            Rm8::Mex8(mex) => self.memory(mex),
        }
    }

    #[inline]
    pub fn rm16(self, rm: Rm16) -> Self {
        match rm {
            Rm16::Reg16(reg) => self.rm_reg(reg),
            Rm16::Rex16(rex) => self.rm_reg(rex),
            Rm16::Mem16(mem) => self.memory(mem),
            Rm16::Mex16(mex) => self.memory(mex),
        }
    }

    #[inline]
    pub fn rm32(self, rm: Rm32) -> Self {
        match rm {
            Rm32::Reg32(reg) => self.rm_reg(reg),
            Rm32::Rex32(rex) => self.rm_reg(rex),
            Rm32::Mem32(mem) => self.memory(mem),
            Rm32::Mex32(mex) => self.memory(mex),
        }
    }

    #[inline]
    pub fn rm64(self, rm: Rm64) -> Self {
        match rm {
            Rm64::Rex64(rex) => self.rm_reg(rex),
            Rm64::Mem64(mem) => self.memory(mem),
            Rm64::Mex64(mex) => self.memory(mex),
        }
    }

    #[inline]
    pub fn rm8_r8(self, rmr: Rm8R8) -> Self {
        match rmr {
            Rm8R8::Reg8Reg8(rm, reg) => self.rm_reg(rm).reg(reg),
            Rm8R8::Rex8Rex8(rm, rex) => self.rm_reg(rm).reg(rex),
            Rm8R8::Mem8Reg8(mem, reg) => self.memory(mem).reg(reg),
            Rm8R8::Mex8Rex8(mex, rex) => self.memory(mex).reg(rex),
        }
    }

    #[inline]
    pub fn rm16_r16(self, rmr: Rm16R16) -> Self {
        match rmr {
            Rm16R16::Reg16Reg16(rm, reg) => self.rm_reg(rm).reg(reg),
            Rm16R16::Rex16Rex16(rm, rex) => self.rm_reg(rm).reg(rex),
            Rm16R16::Mem16Reg16(mem, reg) => self.memory(mem).reg(reg),
            Rm16R16::Mex16Rex16(mex, rex) => self.memory(mex).reg(rex),
        }
    }

    #[inline]
    pub fn rm32_r32(self, rmr: Rm32R32) -> Self {
        match rmr {
            Rm32R32::Reg32Reg32(rm, reg) => self.rm_reg(rm).reg(reg),
            Rm32R32::Rex32Rex32(rm, rex) => self.rm_reg(rm).reg(rex),
            Rm32R32::Mem32Reg32(mem, reg) => self.memory(mem).reg(reg),
            Rm32R32::Mex32Rex32(mex, rex) => self.memory(mex).reg(rex),
        }
    }

    #[inline]
    pub fn rm64_r64(self, rmr: Rm64R64) -> Self {
        match rmr {
            Rm64R64::Rex64Rex64(rm, rex) => self.rm_reg(rm).reg(rex),
            Rm64R64::Mex64Rex64(mex, rex) => self.memory(mex).reg(rex),
        }
    }

    #[inline]
    pub fn r8_rm8(self, rrm: R8Rm8) -> Self {
        match rrm {
            R8Rm8::Reg8Reg8(reg, rm) => self.reg(reg).rm_reg(rm),
            R8Rm8::Rex8Rex8(rex, rm) => self.reg(rex).rm_reg(rm),
            R8Rm8::Reg8Mem8(reg, mem) => self.reg(reg).memory(mem),
            R8Rm8::Rex8Mex8(rex, mex) => self.reg(rex).memory(mex),
        }
    }

    #[inline]
    pub fn r16_rm16(self, rrm: R16Rm16) -> Self {
        match rrm {
            R16Rm16::Reg16Reg16(reg, rm) => self.reg(reg).rm_reg(rm),
            R16Rm16::Rex16Rex16(rex, rm) => self.reg(rex).rm_reg(rm),
            R16Rm16::Reg16Mem16(reg, mem) => self.reg(reg).memory(mem),
            R16Rm16::Rex16Mex16(rex, mex) => self.reg(rex).memory(mex),
        }
    }

    #[inline]
    pub fn r32_rm32(self, rrm: R32Rm32) -> Self {
        match rrm {
            R32Rm32::Reg32Reg32(reg, rm) => self.reg(reg).rm_reg(rm),
            R32Rm32::Rex32Rex32(rex, rm) => self.reg(rex).rm_reg(rm),
            R32Rm32::Reg32Mem32(reg, mem) => self.reg(reg).memory(mem),
            R32Rm32::Rex32Mex32(rex, mex) => self.reg(rex).memory(mex),
        }
    }

    #[inline]
    pub fn r64_rm64(self, rrm: R64Rm64) -> Self {
        match rrm {
            R64Rm64::Rex64Rex64(rex, rm) => self.reg(rex).rm_reg(rm),
            R64Rm64::Rex64Mex64(rex, mex) => self.reg(rex).memory(mex),
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
            Adc::AlImm8(imm) =>
                Instruction::opcode1(0x14).imm8(imm),
            Adc::AxImm16(imm) =>
                Instruction::opcode1(0x15).operand_size().imm16(imm),
            Adc::EaxImm32(imm) =>
                Instruction::opcode1(0x15).imm32(imm),
            Adc::RaxImm32(imm) =>
                Instruction::opcode1(0x15).rex_w().imm32(imm),

            Adc::Rm8Imm8(rm, imm) =>
                Instruction::opcode1(0x80).reg(2).rm8(rm).imm8(imm),
            Adc::Rm16Imm16(rm, imm) =>
                Instruction::opcode1(0x81).operand_size().reg(2).rm16(rm).imm16(imm),
            Adc::Rm32Imm32(rm, imm) =>
                Instruction::opcode1(0x81).reg(2).rm32(rm).imm32(imm),
            Adc::Rm64Imm32(rm, imm) =>
                Instruction::opcode1(0x81).rex_w().reg(2).rm64(rm).imm32(imm),

            Adc::Rm16Imm8(rm, imm) =>
                Instruction::opcode1(0x83).operand_size().reg(2).rm16(rm).imm8(imm),
            Adc::Rm32Imm8(rm, imm) =>
                Instruction::opcode1(0x83).reg(2).rm32(rm).imm8(imm),
            Adc::Rm64Imm8(rm, imm) =>
                Instruction::opcode1(0x83).rex_w().reg(2).rm64(rm).imm8(imm),

            Adc::Rm8R8(rmr) =>
                Instruction::opcode1(0x10).rm8_r8(rmr),
            Adc::Rm16R16(rmr) =>
                Instruction::opcode1(0x11).operand_size().rm16_r16(rmr),
            Adc::Rm32R32(rmr) =>
                Instruction::opcode1(0x11).rm32_r32(rmr),
            Adc::Rm64R64(rmr) =>
                Instruction::opcode1(0x11).rex_w().rm64_r64(rmr),

            Adc::R8Rm8(rmr) =>
                Instruction::opcode1(0x12).r8_rm8(rmr),
            Adc::R16Rm16(rmr) =>
                Instruction::opcode1(0x13).operand_size().r16_rm16(rmr),
            Adc::R32Rm32(rmr) =>
                Instruction::opcode1(0x13).r32_rm32(rmr),
            Adc::R64Rm64(rmr) =>
                Instruction::opcode1(0x13).rex_w().r64_rm64(rmr),
        }
    }
}
