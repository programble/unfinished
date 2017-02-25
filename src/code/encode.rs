use code::{Prefix2, Prefix3, Prefix4, Opcode, Disp, Imm, Instruction};
use code::Register;

use mnemonic::instruction::Adc;
use mnemonic::operand::{
    self,
    Imm8, Imm16, Imm32,
    Scale, Sreg, Offset, Mem,
    Rm8L, Rm8, Rm16, Rm32, Rm64,
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
            Offset::Index(index, scale) => {
                self.offset_index_disp(index, scale, 0)
            },
            Offset::IndexDisp(index, scale, disp) => {
                self.offset_index_disp(index, scale, disp)
            },
            Offset::Disp(disp) => {
                self.offset_index_disp(0b100, Scale::X1, disp)
            },
            Offset::RipDisp(disp) => {
                self.offset_rip_disp(disp)
            },
        }
    }

    #[inline]
    pub fn mem<B32, I32, B64, I64>(self, mem: Mem<B32, I32, B64, I64>) -> Self
    where B32: Register, I32: Register, B64: Register, I64: Register {
        match mem {
            Mem::Offset32(None, offset) => {
                self.address_size()
                    .offset(offset)
            },
            Mem::Offset32(Some(sreg), offset) => {
                self.address_size()
                    .sreg(sreg)
                    .offset(offset)
            },
            Mem::Offset64(None, offset) => {
                self.offset(offset)
            },
            Mem::Offset64(Some(sreg), offset) => {
                self.sreg(sreg)
                    .offset(offset)
            },
        }
    }

    #[inline]
    pub fn rm8l(self, rm: Rm8L) -> Self {
        match rm {
            Rm8L::R8L(r) => self.rm_reg(r),
            Rm8L::M8L(m) => self.mem(m),
        }
    }

    #[inline]
    pub fn rm8(self, rm: Rm8) -> Self {
        match rm {
            Rm8::R8(r) => self.rm_reg(r),
            Rm8::M8(m) => self.mem(m),
        }
    }

    #[inline]
    pub fn rm16(self, rm: Rm16) -> Self {
        match rm {
            Rm16::R16(r) => self.rm_reg(r),
            Rm16::M16(m) => self.mem(m),
        }
    }

    #[inline]
    pub fn rm32(self, rm: Rm32) -> Self {
        match rm {
            Rm32::R32(r) => self.rm_reg(r),
            Rm32::M32(m) => self.mem(m),
        }
    }

    #[inline]
    pub fn rm64(self, rm: Rm64) -> Self {
        match rm {
            Rm64::R64(r) => self.rm_reg(r),
            Rm64::M64(m) => self.mem(m),
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

            Adc::Rm8LImm8(rm, imm) =>
                Instruction::opcode1(0x80).reg(2).rm8l(rm).imm8(imm),
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

            Adc::Rm8LR8L(rm, r) =>
                Instruction::opcode1(0x10).rm8l(rm).reg(r),
            Adc::Rm8R8(rm, r) =>
                Instruction::opcode1(0x10).rm8(rm).reg(r),
            Adc::Rm16R16(rm, r) =>
                Instruction::opcode1(0x11).operand_size().rm16(rm).reg(r),
            Adc::Rm32R32(rm, r) =>
                Instruction::opcode1(0x11).rm32(rm).reg(r),
            Adc::Rm64R64(rm, r) =>
                Instruction::opcode1(0x11).rex_w().rm64(rm).reg(r),

            Adc::R8LRm8L(r, rm) =>
                Instruction::opcode1(0x12).reg(r).rm8l(rm),
            Adc::R8Rm8(r, rm) =>
                Instruction::opcode1(0x12).reg(r).rm8(rm),
            Adc::R16Rm16(r, rm) =>
                Instruction::opcode1(0x13).operand_size().reg(r).rm16(rm),
            Adc::R32Rm32(r, rm) =>
                Instruction::opcode1(0x13).reg(r).rm32(rm),
            Adc::R64Rm64(r, rm) =>
                Instruction::opcode1(0x13).rex_w().reg(r).rm64(rm),
        }
    }
}
