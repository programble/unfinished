mod instruction;
mod register;

use code::{Prefix1, Prefix2, Prefix3, Prefix4, Rex, Opcode, Modrm, Sib, Disp, Imm, Instruction};
use code::encode::register::Register;
use set::imm::{Imm8, Imm16, Imm32, Imm64, Cc};
use set::mem::{self, Scale, Offset, Mem, Moffs, Rm8l, Rm8, Rm16, Rm32, Rm64};
use set::reg::Sreg;

pub trait Encode {
    fn encode(&self) -> Instruction;
}

impl Default for Rex {
    #[inline]
    fn default() -> Self {
        Rex(0x40)
    }
}

impl Rex {
    fn w(self) -> Self { Rex(self.0 | 0b1000) }
    fn r(self) -> Self { Rex(self.0 | 0b0100) }
    fn x(self) -> Self { Rex(self.0 | 0b0010) }
    fn b(self) -> Self { Rex(self.0 | 0b0001) }
}

impl Modrm {
    fn mode(self, mode: u8) -> Self {
        debug_assert_eq!(0, mode & !0b11);
        Modrm(self.0 & 0b00_111_111 | mode << 6)
    }

    fn reg(self, reg: u8) -> Self {
        debug_assert_eq!(0, reg & !0b111);
        Modrm(self.0 & 0b11_000_111 | (reg << 3 & 0b00_111_000))
    }

    fn rm(self, rm: u8) -> Self {
        debug_assert_eq!(0, rm & !0b111);
        Modrm(self.0 & 0b11_111_000 | (rm & 0b00_000_111))
    }
}

impl Sib {
    fn scale(self, scale: u8) -> Self {
        debug_assert_eq!(0, scale & !0b11);
        Sib(self.0 & 0b00_111_111 | scale << 6)
    }

    fn index(self, index: u8) -> Self {
        debug_assert_eq!(0, index & !0b111);
        Sib(self.0 & 0b11_000_111 | (index << 3 & 0b00_111_000))
    }

    fn base(self, base: u8) -> Self {
        debug_assert_eq!(0, base & !0b111);
        Sib(self.0 & 0b11_111_000 | (base & 0b00_000_111))
    }
}

impl Instruction {
    fn new(opcode: Opcode) -> Self {
        Instruction {
            prefix1: None,
            prefix2: None,
            prefix3: None,
            prefix4: None,
            rex: None,
            opcode: opcode,
            modrm: None,
            sib: None,
            disp: None,
            imm: None,
        }
    }

    fn rex(mut self) -> Self {
        self.rex = Some(self.rex.unwrap_or_default());
        self
    }

    fn rex_w(mut self) -> Self {
        self.rex = Some(self.rex.unwrap_or_default().w());
        self
    }

    fn rex_r(mut self) -> Self {
        self.rex = Some(self.rex.unwrap_or_default().r());
        self
    }

    fn rex_x(mut self) -> Self {
        self.rex = Some(self.rex.unwrap_or_default().x());
        self
    }

    fn rex_b(mut self) -> Self {
        self.rex = Some(self.rex.unwrap_or_default().b());
        self
    }

    fn modrm_mode(mut self, mode: u8) -> Self {
        self.modrm = Some(self.modrm.unwrap_or_default().mode(mode));
        self
    }

    fn modrm_reg(mut self, reg: u8) -> Self {
        self.modrm = Some(self.modrm.unwrap_or_default().reg(reg));
        self
    }

    fn modrm_rm(mut self, rm: u8) -> Self {
        self.modrm = Some(self.modrm.unwrap_or_default().rm(rm));
        self
    }

    fn sib_scale(mut self, scale: u8) -> Self {
        self.sib = Some(self.sib.unwrap_or_default().scale(scale));
        self
    }

    fn sib_index(mut self, index: u8) -> Self {
        self.sib = Some(self.sib.unwrap_or_default().index(index));
        self
    }

    fn sib_base(mut self, base: u8) -> Self {
        self.sib = Some(self.sib.unwrap_or_default().base(base));
        self
    }

    fn repne(mut self) -> Self {
        self.prefix1 = Some(Prefix1::Repne);
        self
    }

    fn rep(mut self) -> Self {
        self.prefix1 = Some(Prefix1::Rep);
        self
    }

    fn sreg(mut self, sreg: Sreg) -> Self {
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

    fn osz(mut self) -> Self {
        self.prefix3 = Some(Prefix3::Osz);
        self
    }

    fn asz(mut self) -> Self {
        self.prefix4 = Some(Prefix4::Asz);
        self
    }

    fn cc(mut self, cc: Cc) -> Self {
        self.opcode = match self.opcode {
            Opcode::B1(opcode) => Opcode::B1([opcode[0] + cc.code()]),
            Opcode::B2(opcode) => Opcode::B2([opcode[0], opcode[1] + cc.code()]),
            Opcode::B3(opcode) => Opcode::B3([opcode[0], opcode[1], opcode[2] + cc.code()]),
        };
        self
    }

    fn plus<R>(mut self, reg: R) -> Self where R: Register {
        if reg.force_rex() { self = self.rex() }
        let (rex, code) = reg.rex_code();
        if rex { self = self.rex_b() }
        self.opcode = match self.opcode {
            Opcode::B1(opcode) => Opcode::B1([opcode[0] + code]),
            Opcode::B2(opcode) => Opcode::B2([opcode[0], opcode[1] + code]),
            Opcode::B3(opcode) => Opcode::B3([opcode[0], opcode[1], opcode[2] + code]),
        };
        self
    }

    fn reg<R>(mut self, reg: R) -> Self where R: Register {
        if reg.force_rex() { self = self.rex() }
        match reg.rex_code() {
            (true, code) => self.rex_r().modrm_reg(code),
            (false, code) => self.modrm_reg(code),
        }
    }

    fn rm<R>(mut self, reg: R) -> Self where R: Register {
        if reg.force_rex() { self = self.rex() }
        match reg.rex_code() {
            (true, code) => self.rex_b().modrm_rm(code),
            (false, code) => self.modrm_rm(code),
        }
    }

    fn rm_reg<R>(self, reg: R) -> Self where R: Register {
        self.modrm_mode(0b11)
            .rm(reg)
    }

    fn scale(self, scale: Scale) -> Self {
        match scale {
            Scale::X1 => self.sib_scale(0),
            Scale::X2 => self.sib_scale(1),
            Scale::X4 => self.sib_scale(2),
            Scale::X8 => self.sib_scale(3),
        }
    }

    fn index<R>(self, reg: R) -> Self where R: Register {
        match reg.rex_code() {
            (true, code) => self.rex_x().sib_index(code),
            (false, code) => self.sib_index(code),
        }
    }

    fn base<R>(self, reg: R) -> Self where R: Register {
        match reg.rex_code() {
            (true, code) => self.rex_b().sib_base(code),
            (false, code) => self.sib_base(code),
        }
    }

    fn disp8(mut self, disp: i8) -> Self {
        self.disp = Some(Disp::B1([disp as u8]));
        self
    }

    fn disp32(mut self, disp: i32) -> Self {
        let bytes = [
            disp as u8,
            (disp >> 8) as u8,
            (disp >> 16) as u8,
            (disp >> 24) as u8,
        ];
        self.disp = Some(Disp::B4(bytes));
        self
    }

    fn disp64(mut self, disp: i64) -> Self {
        let bytes = [
            disp as u8,
            (disp >> 8) as u8,
            (disp >> 16) as u8,
            (disp >> 24) as u8,
            (disp >> 32) as u8,
            (disp >> 40) as u8,
            (disp >> 48) as u8,
            (disp >> 56) as u8,
        ];
        self.disp = Some(Disp::B8(bytes));
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

    fn offset_disp(self, disp: mem::Disp) -> Self {
        match disp {
            mem::Disp::Disp8(disp) => {
                self.modrm_mode(0b01)
                    .disp8(disp)
            },
            mem::Disp::Disp32(disp) => {
                self.modrm_mode(0b10)
                    .disp32(disp)
            },
        }
    }

    fn offset_base_index<Base, Index>(mut self, base: Base, index: Index, scale: Scale) -> Self
    where Base: Register, Index: Register {
        if let (_, 0b101) = base.rex_code() { self = self.offset_disp(mem::Disp::Disp8(0)) }
        self.modrm_rm(0b100)
            .scale(scale)
            .index(index)
            .base(base)
    }

    fn offset_base<Base>(self, base: Base) -> Self where Base: Register {
        match base.rex_code() {
            (_, 0b100) => self.offset_base_index(base, 0b100, Scale::X1),
            (_, 0b101) => self.rm(base).offset_disp(mem::Disp::Disp8(0)),
            _ => self.rm(base),
        }
    }

    fn offset_rip_disp(self, disp: i32) -> Self {
        self.modrm_mode(0b00)
            .modrm_rm(0b101)
            .disp32(disp)
    }

    fn offset<Base, Index>(self, offset: Offset<Base, Index>) -> Self
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

    fn mem<B32, I32, B64, I64>(self, mem: Mem<B32, I32, B64, I64>) -> Self
    where B32: Register, I32: Register, B64: Register, I64: Register {
        match mem {
            Mem::Offset32(None, offset) => {
                self.asz()
                    .offset(offset)
            },
            Mem::Offset32(Some(sreg), offset) => {
                self.asz()
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

    fn moffs(self, moffs: Moffs) -> Self {
        match moffs {
            Moffs::Moffset32(None, offset) => {
                self.asz()
                    .disp32(offset as i32)
            },
            Moffs::Moffset32(Some(sreg), offset) => {
                self.asz()
                    .sreg(sreg)
                    .disp32(offset as i32)
            },
            Moffs::Moffset64(None, offset) => {
                self.disp64(offset as i64)
            },
            Moffs::Moffset64(Some(sreg), offset) => {
                self.sreg(sreg)
                    .disp64(offset as i64)
            },
        }
    }

    fn rm8l(self, rm: Rm8l) -> Self {
        match rm {
            Rm8l::R8l(r) => self.rm_reg(r),
            Rm8l::M8l(m) => self.mem(m),
        }
    }

    fn rm8(self, rm: Rm8) -> Self {
        match rm {
            Rm8::R8(r) => self.rm_reg(r),
            Rm8::M8(m) => self.mem(m),
        }
    }

    fn rm16(self, rm: Rm16) -> Self {
        match rm {
            Rm16::R16(r) => self.rm_reg(r),
            Rm16::M16(m) => self.mem(m),
        }
    }

    fn rm32(self, rm: Rm32) -> Self {
        match rm {
            Rm32::R32(r) => self.rm_reg(r),
            Rm32::M32(m) => self.mem(m),
        }
    }

    fn rm64(self, rm: Rm64) -> Self {
        match rm {
            Rm64::R64(r) => self.rm_reg(r),
            Rm64::M64(m) => self.mem(m),
        }
    }

    fn imm8(mut self, imm: Imm8) -> Self {
        self.imm = Some(Imm::B1([imm.0]));
        self
    }

    fn imm16(mut self, imm: Imm16) -> Self {
        let bytes = [
            imm.0 as u8,
            (imm.0 >> 8) as u8,
        ];
        self.imm = Some(Imm::B2(bytes));
        self
    }

    fn imm32(mut self, imm: Imm32) -> Self {
        let bytes = [
            imm.0 as u8,
            (imm.0 >> 8) as u8,
            (imm.0 >> 16) as u8,
            (imm.0 >> 24) as u8,
        ];
        self.imm = Some(Imm::B4(bytes));
        self
    }

    fn imm64(mut self, imm: Imm64) -> Self {
        let bytes = [
            imm.0 as u8,
            (imm.0 >> 8) as u8,
            (imm.0 >> 16) as u8,
            (imm.0 >> 24) as u8,
            (imm.0 >> 32) as u8,
            (imm.0 >> 40) as u8,
            (imm.0 >> 48) as u8,
            (imm.0 >> 56) as u8,
        ];
        self.imm = Some(Imm::B8(bytes));
        self
    }
}
