use code::{Prefix2, Prefix3, Prefix4, Opcode, Disp, Imm, Instruction};
use code::Register;

use mnemonic::instruction::Adc;
use mnemonic::operand::{
    self,
    Imm8, Imm16, Imm32,
    Reg32, Reg64,
    Scale, IndexReg32, IndexReg64,
    Sreg, Offset, Memory, Mem, Mex,
    Rm8,
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

    fn offset_disp(self, disp: i32) -> Self {
        self.modrm_mode(0b00)
            .modrm_rm(0b100)
            .sib_scale(0b00)
            .sib_index(0b100)
            .sib_base(0b101)
            .disp32(disp)
    }

    fn offset_index<Index>(self, index: Index, scale: Scale) -> Self where Index: Register {
        self.offset_disp(0)
            .scale(scale)
            .index(index)
    }

    fn offset_index_disp<Index>(self, index: Index, scale: Scale, disp: i32) -> Self
    where Index: Register {
        self.offset_index(index, scale)
            .disp32(disp)
    }

    fn offset_base<Base>(self, base: Base) -> Self where Base: Register {
        match base.index() {
            4 => { // rsp
                self.modrm_mode(0b00)
                    .modrm_rm(0b100)
                    .sib_index(0b100)
                    .base(base)
            },
            5 => { // rbp
                self.modrm_mode(0b01)
                    .rm(base)
                    .disp8(0)
            },
            _ => {
                self.modrm_mode(0b00)
                    .rm(base)
            },
        }
    }

    fn offset_base_disp<Base>(self, base: Base, disp: operand::Disp) -> Self
    where Base: Register {
        match disp {
            operand::Disp::Disp8(disp) => {
                self.offset_base(base)
                    .modrm_mode(0b01)
                    .disp8(disp)
            },
            operand::Disp::Disp32(disp) => {
                self.offset_base(base)
                    .modrm_mode(0b10)
                    .disp32(disp)
            },
        }
    }

    fn offset_base_index<Base, Index>(self, base: Base, index: Index, scale: Scale) -> Self
    where Base: Register, Index: Register {
        if base.index() == 5 { // rbp
            self.offset_index(index, scale)
                .modrm_mode(0b01)
                .base(base)
                .disp8(0)
        } else {
            self.modrm_mode(0b00)
                .modrm_rm(0b100)
                .scale(scale)
                .index(index)
                .base(base)
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
            Offset::Disp(disp) => self.offset_disp(disp),
            Offset::Index(index, scale) => self.offset_index(index, scale),
            Offset::IndexDisp(index, scale, disp) => self.offset_index_disp(index, scale, disp),
            Offset::Base(base) => self.offset_base(base),
            Offset::BaseDisp(base, disp) => self.offset_base_disp(base, disp),
            Offset::BaseIndex(base, index, scale) => self.offset_base_index(base, index, scale),
            Offset::RipDisp(disp) => self.offset_rip_disp(disp),
            _ => unimplemented!(),
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

impl Instruction {
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
                    .index(index)
                    .sib_base(0b101)
                    .disp32(0)
            },
            Offset::IndexDisp(index, scale, disp) => {
                self.modrm_mode(0b00)
                    .modrm_rm(0b100)
                    .scale(scale)
                    .index(index)
                    .sib_base(0b101)
                    .disp32(disp)
            },
            Offset::Base(Reg64::Rsp) => {
                self.modrm_mode(0b00)
                    .modrm_rm(0b100)
                    .sib_index(0b100)
                    .base(Reg64::Rsp)
            },
            Offset::Base(Reg64::Rbp) => {
                self.modrm_mode(0b01)
                    .rm(Reg64::Rbp)
                    .disp8(0)
            },
            Offset::Base(base) => {
                self.modrm_mode(0b00)
                    .rm(base)
            },
            Offset::BaseDisp(Reg64::Rsp, operand::Disp::Disp8(disp)) => {
                self.modrm_mode(0b01)
                    .modrm_rm(0b100)
                    .sib_index(0b100)
                    .base(Reg64::Rsp)
                    .disp8(disp)
            },
            Offset::BaseDisp(base, operand::Disp::Disp8(disp)) => {
                self.modrm_mode(0b01)
                    .rm(base)
                    .disp8(disp)
            },
            Offset::BaseDisp(Reg64::Rsp, operand::Disp::Disp32(disp)) => {
                self.modrm_mode(0b10)
                    .modrm_rm(0b100)
                    .sib_index(0b100)
                    .base(Reg64::Rsp)
                    .disp32(disp)
            },
            Offset::BaseDisp(base, operand::Disp::Disp32(disp)) => {
                self.modrm_mode(0b10)
                    .rm(base)
                    .disp32(disp)
            },
            Offset::BaseIndex(Reg64::Rbp, index, scale) => {
                self.modrm_mode(0b01)
                    .modrm_rm(0b100)
                    .scale(scale)
                    .index(index)
                    .base(Reg64::Rbp)
                    .disp8(0)
            },
            Offset::BaseIndex(base, index, scale) => {
                self.modrm_mode(0b00)
                    .modrm_rm(0b100)
                    .scale(scale)
                    .index(index)
                    .base(base)
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
                self.address_size().offset(offset)
            },
            Memory::Offset32(Some(sreg), offset) => {
                self.address_size().sreg(sreg).offset(offset)
            },
            Memory::Offset64(None, offset) => {
                self.offset(offset)
            },
            Memory::Offset64(Some(sreg), offset) => {
                self.sreg(sreg).offset(offset)
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
            Rm8::Reg8(reg) => self.modrm_mode(0b11).rm(reg),
            Rm8::Rex8(rex) => self.modrm_mode(0b11).rm(rex),
            Rm8::Mem8(mem) => self.mem(mem),
            Rm8::Mex8(mex) => self.mex(mex),
        }
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
