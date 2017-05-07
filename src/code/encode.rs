use set::*;
use set::imm::{Cc, Imm8, Imm16, Imm32, Imm64};
use set::mem::{self, Scale, Offs, Mem, Moffs, Rm};
use set::reg::Sreg;

use super::{Prefix1, Prefix2, Prefix3, Prefix4, Opcode, Disp, Imm, Inst};
use super::reg::Reg;

/// Encodes instructions.
pub trait Encode {
    /// Encodes the instruction.
    fn encode(&self) -> Inst;
}

/// Instruction encoding DSL.
pub trait InstExt {
    fn lock(self) -> Self;
    fn repne(self) -> Self;
    fn rep(self) -> Self;

    fn sreg(self, sreg: Sreg) -> Self;

    fn osz(self) -> Self;

    fn asz(self) -> Self;

    fn rex(self) -> Self;
    fn rex_w(self) -> Self;
    fn rex_r(self) -> Self;
    fn rex_x(self) -> Self;
    fn rex_b(self) -> Self;

    fn opcode(opcode: Opcode) -> Self;
    fn opcode1(a: u8) -> Self;
    fn opcode2(b: u8) -> Self;
    fn opcode3(b: u8, c: u8) -> Self;
    fn fopcode(b: u8) -> Self;

    fn cc(self, cc: Cc) -> Self;
    fn plus<R>(self, reg: R) -> Self where R: Reg;

    fn modrm_mod(self, mod_: u8) -> Self;
    fn modrm_reg(self, reg: u8) -> Self;
    fn modrm_rm(self, rm: u8) -> Self;

    fn reg<R>(self, reg: R) -> Self where R: Reg;
    fn rm_base<R>(self, reg: R) -> Self where R: Reg;
    fn rm_reg<R>(self, reg: R) -> Self where R: Reg;

    fn sib_scale(self, scale: u8) -> Self;
    fn sib_index(self, index: u8) -> Self;
    fn sib_base(self, base: u8) -> Self;

    fn scale(self, scale: Scale) -> Self;
    fn index<R>(self, reg: R) -> Self where R: Reg;
    fn base<R>(self, reg: R) -> Self where R: Reg;

    fn disp<D>(self, disp: D) -> Self where Disp: From<D>;

    fn offs_index_disp<Index>(self, index: Index, scale: Scale, disp: i32) -> Self
        where Index: Reg;
    fn offs_disp(self, disp: mem::Disp) -> Self;
    fn offs_base_index<Base, Index>(self, base: Base, index: Index, scale: Scale) -> Self
        where Base: Reg, Index: Reg;
    fn offs_base<Base>(self, base: Base) -> Self where Base: Reg;
    fn offs_rip_disp(self, disp: i32) -> Self;
    fn offs<Base, Index>(self, offs: Offs<Base, Index>) -> Self
        where Base: Reg, Index: Reg;
    fn mem<B32, I32, B64, I64>(self, mem: Mem<B32, I32, B64, I64>) -> Self
        where B32: Reg, I32: Reg, B64: Reg, I64: Reg;
    fn moffs(self, moffs: Moffs) -> Self;

    fn rm<R, B32, I32, B64, I64>(self, rm: Rm<R, B32, I32, B64, I64>) -> Self
        where R: Reg, B32: Reg, I32: Reg, B64: Reg, I64: Reg;

    fn imm8(self, imm: Imm8) -> Self;
    fn imm16(self, imm: Imm16) -> Self;
    fn imm32(self, imm: Imm32) -> Self;
    fn imm64(self, imm: Imm64) -> Self;
}

impl InstExt for Inst {
    #[inline]
    fn lock(mut self) -> Self {
        self.prefix1 = Some(Prefix1::Lock);
        self
    }
    #[inline]
    fn repne(mut self) -> Self {
        self.prefix1 = Some(Prefix1::Repne);
        self
    }
    #[inline]
    fn rep(mut self) -> Self {
        self.prefix1 = Some(Prefix1::Rep);
        self
    }

    #[inline]
    fn sreg(mut self, sreg: Sreg) -> Self {
        self.prefix2 = Some(
            match sreg {
                Sreg::Cs => Prefix2::Cs,
                Sreg::Ds => Prefix2::Ds,
                Sreg::Ss => Prefix2::Ss,
                Sreg::Es => Prefix2::Es,
                Sreg::Fs => Prefix2::Fs,
                Sreg::Gs => Prefix2::Gs,
            }
        );
        self
    }

    #[inline]
    fn osz(mut self) -> Self {
        self.prefix3 = Some(Prefix3::Osz);
        self
    }

    #[inline]
    fn asz(mut self) -> Self {
        self.prefix4 = Some(Prefix4::Asz);
        self
    }

    #[inline]
    fn rex(mut self) -> Self {
        self.rex = Some(self.rex.unwrap_or_default());
        self
    }
    #[inline]
    fn rex_w(mut self) -> Self {
        self.rex = Some(self.rex.unwrap_or_default().set_w());
        self
    }
    #[inline]
    fn rex_r(mut self) -> Self {
        self.rex = Some(self.rex.unwrap_or_default().set_r());
        self
    }
    #[inline]
    fn rex_x(mut self) -> Self {
        self.rex = Some(self.rex.unwrap_or_default().set_x());
        self
    }
    #[inline]
    fn rex_b(mut self) -> Self {
        self.rex = Some(self.rex.unwrap_or_default().set_b());
        self
    }

    #[inline]
    fn opcode(opcode: Opcode) -> Self {
        Inst {
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
    #[inline]
    fn opcode1(a: u8) -> Self {
        Self::opcode(Opcode::B1([a]))
    }
    #[inline]
    fn opcode2(b: u8) -> Self {
        Self::opcode(Opcode::B2([0x0f, b]))
    }
    #[inline]
    fn opcode3(b: u8, c: u8) -> Self {
        Self::opcode(Opcode::B3([0x0f, b, c]))
    }
    #[inline]
    fn fopcode(b: u8) -> Self {
        Self::opcode(Opcode::B2([0xd9, b]))
    }

    #[inline]
    fn cc(mut self, cc: Cc) -> Self {
        self.opcode = self.opcode.plus(cc.encode());
        self
    }
    fn plus<R>(mut self, reg: R) -> Self where R: Reg {
        if reg.force_rex() { self = self.rex() }
        if reg.high_bit() { self = self.rex_b() }
        self.opcode = self.opcode.plus(reg.low_bits());
        self
    }

    #[inline]
    fn modrm_mod(mut self, mod_: u8) -> Self {
        self.modrm = Some(self.modrm.unwrap_or_default().set_mod(mod_));
        self
    }
    #[inline]
    fn modrm_reg(mut self, reg: u8) -> Self {
        self.modrm = Some(self.modrm.unwrap_or_default().set_reg(reg));
        self
    }
    #[inline]
    fn modrm_rm(mut self, rm: u8) -> Self {
        self.modrm = Some(self.modrm.unwrap_or_default().set_rm(rm));
        self
    }

    fn reg<R>(mut self, reg: R) -> Self where R: Reg {
        if reg.force_rex() { self = self.rex() }
        if reg.high_bit() { self = self.rex_r() }
        self.modrm_reg(reg.low_bits())
    }
    fn rm_base<R>(mut self, reg: R) -> Self where R: Reg {
        if reg.force_rex() { self = self.rex() }
        if reg.high_bit() { self = self.rex_b() }
        self.modrm_rm(reg.low_bits())
    }
    fn rm_reg<R>(self, reg: R) -> Self where R: Reg {
        self.modrm_mod(0b11).rm_base(reg)
    }

    #[inline]
    fn sib_scale(mut self, scale: u8) -> Self {
        self.sib = Some(self.sib.unwrap_or_default().set_scale(scale));
        self
    }
    #[inline]
    fn sib_index(mut self, index: u8) -> Self {
        self.sib = Some(self.sib.unwrap_or_default().set_index(index));
        self
    }
    #[inline]
    fn sib_base(mut self, base: u8) -> Self {
        self.sib = Some(self.sib.unwrap_or_default().set_base(base));
        self
    }

    #[inline]
    fn scale(self, scale: Scale) -> Self {
        match scale {
            Scale::X1 => self.sib_scale(0),
            Scale::X2 => self.sib_scale(1),
            Scale::X4 => self.sib_scale(2),
            Scale::X8 => self.sib_scale(3),
        }
    }
    fn index<R>(mut self, reg: R) -> Self where R: Reg {
        if reg.high_bit() { self = self.rex_x() }
        self.sib_index(reg.low_bits())
    }
    fn base<R>(mut self, reg: R) -> Self where R: Reg {
        if reg.high_bit() { self = self.rex_b() }
        self.sib_base(reg.low_bits())
    }

    fn disp<D>(mut self, disp: D) -> Self where Disp: From<D> {
        self.disp = Some(Disp::from(disp));
        self
    }

    fn offs_index_disp<Index>(self, index: Index, scale: Scale, disp: i32) -> Self
    where Index: Reg {
        self.modrm_mod(0b00)
            .modrm_rm(0b100)
            .sib_base(0b101)
            .scale(scale)
            .index(index)
            .disp(disp)
    }
    #[inline]
    fn offs_disp(self, disp: mem::Disp) -> Self {
        match disp {
            mem::Disp::Disp8(disp) => self.modrm_mod(0b01).disp(disp),
            mem::Disp::Disp32(disp) => self.modrm_mod(0b10).disp(disp),
        }
    }
    fn offs_base_index<Base, Index>(mut self, base: Base, index: Index, scale: Scale) -> Self
    where Base: Reg, Index: Reg {
        if base.low_bits() == 0b101 {
            self = self.offs_disp(mem::Disp::Disp8(0));
        }
        self.modrm_rm(0b100)
            .scale(scale)
            .index(index)
            .base(base)
    }
    fn offs_base<Base>(self, base: Base) -> Self where Base: Reg {
        match base.low_bits() {
            0b100 => self.offs_base_index(base, 0b100, Scale::X1),
            0b101 => self.rm_base(base).offs_disp(mem::Disp::Disp8(0)),
            _ => self.rm_base(base),
        }
    }
    #[inline]
    fn offs_rip_disp(self, disp: i32) -> Self {
        self.modrm_mod(0b00)
            .modrm_rm(0b101)
            .disp(disp)
    }
    fn offs<Base, Index>(self, offs: Offs<Base, Index>) -> Self
    where Base: Reg, Index: Reg {
        match offs {
            Offs::Base(b)                   => self.offs_base(b),
            Offs::BaseDisp(b, d)            => self.offs_base(b).offs_disp(d),
            Offs::BaseIndex(b, i, s)        => self.offs_base_index(b, i, s),
            Offs::BaseIndexDisp(b, i, s, d) => self.offs_base_index(b, i, s).offs_disp(d),
            Offs::IndexDisp(i, s, d)        => self.offs_index_disp(i, s, d),
            Offs::Disp(d)                   => self.offs_index_disp(0b100, Scale::X1, d),
            Offs::RipDisp(d)                => self.offs_rip_disp(d),
        }
    }
    fn mem<B32, I32, B64, I64>(self, mem: Mem<B32, I32, B64, I64>) -> Self
    where B32: Reg, I32: Reg, B64: Reg, I64: Reg {
        match mem {
            Mem::Offs32(None, offs)       => self.asz().offs(offs),
            Mem::Offs32(Some(sreg), offs) => self.asz().sreg(sreg).offs(offs),
            Mem::Offs64(None, offs)       => self.offs(offs),
            Mem::Offs64(Some(sreg), offs) => self.sreg(sreg).offs(offs),
        }
    }
    #[inline]
    fn moffs(self, moffs: Moffs) -> Self {
        match moffs {
            Moffs(None, offs)       => self.disp(offs as i64),
            Moffs(Some(sreg), offs) => self.sreg(sreg).disp(offs as i64),
        }
    }

    fn rm<R, B32, I32, B64, I64>(self, rm: Rm<R, B32, I32, B64, I64>) -> Self
    where R: Reg, B32: Reg, I32: Reg, B64: Reg, I64: Reg {
        match rm {
            Rm::R(r) => self.rm_reg(r),
            Rm::M(m) => self.mem(m),
        }
    }

    #[inline]
    fn imm8(mut self, imm: Imm8) -> Self {
        self.imm = Some(Imm::from(imm.0));
        self
    }
    #[inline]
    fn imm16(mut self, imm: Imm16) -> Self {
        self.imm = Some(Imm::from(imm.0));
        self
    }
    #[inline]
    fn imm32(mut self, imm: Imm32) -> Self {
        self.imm = Some(Imm::from(imm.0));
        self
    }
    #[inline]
    fn imm64(mut self, imm: Imm64) -> Self {
        self.imm = Some(Imm::from(imm.0));
        self
    }
}

macro_rules! impl_encode {
    ($($ty:ident $tt:tt,)+) => {
        $(impl_encode!($ty $tt);)+
    };

    ($ty:ident { $($pat:pat => $enc:expr,)+ }) => {
        impl Encode for $ty {
            fn encode(&self) -> Inst {
                use self::$ty::*;
                match *self {
                    $($pat => $enc,)+
                }
            }
        }
    };

    ($ty:ident($reg:expr)) => {
        impl_encode! {
            $ty {
                Rm8l(rm) => Inst::opcode1(0xf6).reg($reg).rm(rm),
                Rm8(rm)  => Inst::opcode1(0xf6).reg($reg).rm(rm),
                Rm16(rm) => Inst::opcode1(0xf7).reg($reg).rm(rm).osz(),
                Rm32(rm) => Inst::opcode1(0xf7).reg($reg).rm(rm),
                Rm64(rm) => Inst::opcode1(0xf7).reg($reg).rm(rm).rex_w(),
            }
        }
    };

    ($ty:ident($code:expr, $reg:expr)) => {
        impl_encode! {
            $ty {
                Rm8lR8l(rm, r) => Inst::opcode1($code).rm(rm).reg(r),
                Rm8R8(rm, r)   => Inst::opcode1($code).rm(rm).reg(r),
                Rm16R16(rm, r) => Inst::opcode1($code + 1).rm(rm).reg(r).osz(),
                Rm32R32(rm, r) => Inst::opcode1($code + 1).rm(rm).reg(r),
                Rm64R64(rm, r) => Inst::opcode1($code + 1).rm(rm).reg(r).rex_w(),

                R8lRm8l(r, rm) => Inst::opcode1($code + 2).reg(r).rm(rm),
                R8Rm8(r, rm)   => Inst::opcode1($code + 2).reg(r).rm(rm),
                R16Rm16(r, rm) => Inst::opcode1($code + 3).reg(r).rm(rm).osz(),
                R32Rm32(r, rm) => Inst::opcode1($code + 3).reg(r).rm(rm),
                R64Rm64(r, rm) => Inst::opcode1($code + 3).reg(r).rm(rm).rex_w(),

                AlImm8(imm)   => Inst::opcode1($code + 4).imm8(imm),
                AxImm16(imm)  => Inst::opcode1($code + 5).imm16(imm).osz(),
                EaxImm32(imm) => Inst::opcode1($code + 5).imm32(imm),
                RaxImm32(imm) => Inst::opcode1($code + 5).imm32(imm).rex_w(),

                Rm8lImm8(rm, imm)  => Inst::opcode1(0x80).reg($reg).rm(rm).imm8(imm),
                Rm8Imm8(rm, imm)   => Inst::opcode1(0x80).reg($reg).rm(rm).imm8(imm),
                Rm16Imm16(rm, imm) => Inst::opcode1(0x81).reg($reg).rm(rm).imm16(imm).osz(),
                Rm32Imm32(rm, imm) => Inst::opcode1(0x81).reg($reg).rm(rm).imm32(imm),
                Rm64Imm32(rm, imm) => Inst::opcode1(0x81).reg($reg).rm(rm).imm32(imm).rex_w(),

                Rm16Imm8(rm, imm) => Inst::opcode1(0x83).reg($reg).rm(rm).imm8(imm).osz(),
                Rm32Imm8(rm, imm) => Inst::opcode1(0x83).reg($reg).rm(rm).imm8(imm),
                Rm64Imm8(rm, imm) => Inst::opcode1(0x83).reg($reg).rm(rm).imm8(imm).rex_w(),
            }
        }
    };

    ($ty:ident $enc:expr) => {
        impl Encode for $ty {
            fn encode(&self) -> Inst {
                $enc
            }
        }
    };
}

impl_encode! {
    Adc(0x10, 2),

    Adcx {
        R32Rm32(r, rm) => Inst::opcode3(0x38, 0xf6).osz().reg(r).rm(rm),
        R64Rm64(r, rm) => Inst::opcode3(0x38, 0xf6).osz().reg(r).rm(rm).rex_w(),
    },

    Add(0x00, 0),

    Adox {
        R32Rm32(r, rm) => Inst::opcode3(0x38, 0xf6).rep().reg(r).rm(rm),
        R64Rm64(r, rm) => Inst::opcode3(0x38, 0xf6).rep().reg(r).rm(rm).rex_w(),
    },

    And(0x20, 4),

    Bsf {
        R16Rm16(r, rm) => Inst::opcode2(0xbc).reg(r).rm(rm).osz(),
        R32Rm32(r, rm) => Inst::opcode2(0xbc).reg(r).rm(rm),
        R64Rm64(r, rm) => Inst::opcode2(0xbc).reg(r).rm(rm).rex_w(),
    },

    Bsr {
        R16Rm16(r, rm) => Inst::opcode2(0xbd).reg(r).rm(rm).osz(),
        R32Rm32(r, rm) => Inst::opcode2(0xbd).reg(r).rm(rm),
        R64Rm64(r, rm) => Inst::opcode2(0xbd).reg(r).rm(rm).rex_w(),
    },

    Bswap {
        R32(r) => Inst::opcode2(0xc8).plus(r),
        R64(r) => Inst::opcode2(0xc8).plus(r).rex_w(),
    },

    Bt {
        Rm16R16(rm, r) => Inst::opcode2(0xa3).rm(rm).reg(r).osz(),
        Rm32R32(rm, r) => Inst::opcode2(0xa3).rm(rm).reg(r),
        Rm64R64(rm, r) => Inst::opcode2(0xa3).rm(rm).reg(r).rex_w(),

        Rm16Imm8(rm, imm) => Inst::opcode2(0xba).reg(4).rm(rm).imm8(imm).osz(),
        Rm32Imm8(rm, imm) => Inst::opcode2(0xba).reg(4).rm(rm).imm8(imm),
        Rm64Imm8(rm, imm) => Inst::opcode2(0xba).reg(4).rm(rm).imm8(imm).rex_w(),
    },

    Btc {
        Rm16R16(rm, r) => Inst::opcode2(0xbb).rm(rm).reg(r).osz(),
        Rm32R32(rm, r) => Inst::opcode2(0xbb).rm(rm).reg(r),
        Rm64R64(rm, r) => Inst::opcode2(0xbb).rm(rm).reg(r).rex_w(),

        Rm16Imm8(rm, imm) => Inst::opcode2(0xba).reg(7).rm(rm).imm8(imm).osz(),
        Rm32Imm8(rm, imm) => Inst::opcode2(0xba).reg(7).rm(rm).imm8(imm),
        Rm64Imm8(rm, imm) => Inst::opcode2(0xba).reg(7).rm(rm).imm8(imm).rex_w(),
    },

    Btr {
        Rm16R16(rm, r) => Inst::opcode2(0xb3).rm(rm).reg(r).osz(),
        Rm32R32(rm, r) => Inst::opcode2(0xb3).rm(rm).reg(r),
        Rm64R64(rm, r) => Inst::opcode2(0xb3).rm(rm).reg(r).rex_w(),

        Rm16Imm8(rm, imm) => Inst::opcode2(0xba).reg(6).rm(rm).imm8(imm).osz(),
        Rm32Imm8(rm, imm) => Inst::opcode2(0xba).reg(6).rm(rm).imm8(imm),
        Rm64Imm8(rm, imm) => Inst::opcode2(0xba).reg(6).rm(rm).imm8(imm).rex_w(),
    },

    Bts {
        Rm16R16(rm, r) => Inst::opcode2(0xab).rm(rm).reg(r).osz(),
        Rm32R32(rm, r) => Inst::opcode2(0xab).rm(rm).reg(r),
        Rm64R64(rm, r) => Inst::opcode2(0xab).rm(rm).reg(r).rex_w(),

        Rm16Imm8(rm, imm) => Inst::opcode2(0xba).reg(5).rm(rm).imm8(imm).osz(),
        Rm32Imm8(rm, imm) => Inst::opcode2(0xba).reg(5).rm(rm).imm8(imm),
        Rm64Imm8(rm, imm) => Inst::opcode2(0xba).reg(5).rm(rm).imm8(imm).rex_w(),
    },

    Call {
        Rel32(rel) => Inst::opcode1(0xe8).disp(rel.0),
        Rm64(rm)   => Inst::opcode1(0xff).reg(2).rm(rm),
        M16x16(m)  => Inst::opcode1(0xff).reg(3).mem(m).osz(),
        M16x32(m)  => Inst::opcode1(0xff).reg(3).mem(m),
        M16x64(m)  => Inst::opcode1(0xff).reg(3).mem(m).rex_w(),
    },

    Cbw  { Inst::opcode1(0x98).osz() },
    Cwde { Inst::opcode1(0x98) },
    Cdqe { Inst::opcode1(0x98).rex_w() },

    Clac { Inst::opcode3(0x01, 0xca) },

    Clc { Inst::opcode1(0xf8) },

    Cld { Inst::opcode1(0xfc) },

    Clflush {
        M8(m) => Inst::opcode2(0xae).reg(7).mem(m),
    },

    Clflushopt {
        M8(m) => Inst::opcode2(0xae).reg(7).mem(m).osz(),
    },

    Cli { Inst::opcode1(0xfa) },

    Clts { Inst::opcode1(0x06) },

    Cmc { Inst::opcode1(0xf5) },

    Cmov {
        CcR16Rm16(cc, r, rm) => Inst::opcode2(0x40).cc(cc).reg(r).rm(rm).osz(),
        CcR32Rm32(cc, r, rm) => Inst::opcode2(0x40).cc(cc).reg(r).rm(rm),
        CcR64Rm64(cc, r, rm) => Inst::opcode2(0x40).cc(cc).reg(r).rm(rm).rex_w(),
    },

    Cmp(0x38, 7),

    Cmps {
        B => Inst::opcode1(0xa6),
        W => Inst::opcode1(0xa7).osz(),
        D => Inst::opcode1(0xa7),
        Q => Inst::opcode1(0xa7).rex_w(),
    },

    Cmpxchg {
        Rm8lR8l(rm, r) => Inst::opcode2(0xb0).rm(rm).reg(r),
        Rm8R8(rm, r)   => Inst::opcode2(0xb0).rm(rm).reg(r),
        Rm16R16(rm, r) => Inst::opcode2(0xb1).rm(rm).reg(r).osz(),
        Rm32R32(rm, r) => Inst::opcode2(0xb1).rm(rm).reg(r),
        Rm64R64(rm, r) => Inst::opcode2(0xb1).rm(rm).reg(r).rex_w(),
    },

    Cmpxchg8b {
        M64(m) => Inst::opcode2(0xc7).reg(1).mem(m),
    },

    Cmpxchg16b {
        M128(m) => Inst::opcode2(0xc7).reg(1).mem(m).rex_w(),
    },

    Cpuid { Inst::opcode2(0xa2) },

    Crc32 {
        R32lRm8l(r, rm) => Inst::opcode3(0x38, 0xf0).repne().reg(r).rm(rm),
        R32Rm8(r, rm)   => Inst::opcode3(0x38, 0xf0).repne().reg(r).rm(rm),
        R32Rm16(r, rm)  => Inst::opcode3(0x38, 0xf1).repne().reg(r).rm(rm).osz(),
        R32Rm32(r, rm)  => Inst::opcode3(0x38, 0xf1).repne().reg(r).rm(rm),
        R64Rm8(r, rm)   => Inst::opcode3(0x38, 0xf0).repne().reg(r).rm(rm).rex_w(),
        R64Rm64(r, rm)  => Inst::opcode3(0x38, 0xf1).repne().reg(r).rm(rm).rex_w(),
    },

    Cwd { Inst::opcode1(0x99).osz() },
    Cdq { Inst::opcode1(0x99) },
    Cqo { Inst::opcode1(0x99).rex_w() },

    Dec {
        Rm8l(rm) => Inst::opcode1(0xfe).reg(1).rm(rm),
        Rm8(rm)  => Inst::opcode1(0xfe).reg(1).rm(rm),
        Rm16(rm) => Inst::opcode1(0xff).reg(1).rm(rm).osz(),
        Rm32(rm) => Inst::opcode1(0xff).reg(1).rm(rm),
        Rm64(rm) => Inst::opcode1(0xff).reg(1).rm(rm).rex_w(),
    },

    Div(6),

    F2xm1 { Inst::fopcode(0xf0) },

    Fabs { Inst::fopcode(0xe1) },

    Fadd {
        M32fp(m)  => Inst::opcode1(0xd8).reg(0).mem(m),
        M64fp(m)  => Inst::opcode1(0xdc).reg(0).mem(m),
        St0Sti(i) => Inst::opcode1(0xd8).reg(0).rm_reg(i),
        StiSt0(i) => Inst::opcode1(0xdc).reg(0).rm_reg(i),
    },
    Faddp {
        StiSt0(i) => Inst::opcode1(0xde).reg(0).rm_reg(i),
    },
    Fiadd {
        M32int(m) => Inst::opcode1(0xda).reg(0).mem(m),
        M16int(m) => Inst::opcode1(0xde).reg(0).mem(m),
    },

    Fbld {
        M80dec(m) => Inst::opcode1(0xdf).reg(4).mem(m),
    },

    Fbstp {
        M80bcd(m) => Inst::opcode1(0xdf).reg(6).mem(m),
    },

    Fchs { Inst::fopcode(0xe0) },

    Fclex { Inst::opcode(Opcode::B3([0x9b, 0xdb, 0xe2])) },
    Fnclex { Inst::opcode(Opcode::B2([0xdb, 0xe2])) },

    Fcmov {
        BSt0Sti(i)   => Inst::opcode1(0xda).reg(0).rm_reg(i),
        ESt0Sti(i)   => Inst::opcode1(0xda).reg(1).rm_reg(i),
        BeSt0Sti(i)  => Inst::opcode1(0xda).reg(2).rm_reg(i),
        USt0Sti(i)   => Inst::opcode1(0xda).reg(3).rm_reg(i),
        NbSt0Sti(i)  => Inst::opcode1(0xdb).reg(0).rm_reg(i),
        NeSt0Sti(i)  => Inst::opcode1(0xdb).reg(1).rm_reg(i),
        NbeSt0Sti(i) => Inst::opcode1(0xdb).reg(2).rm_reg(i),
        NuSt0Sti(i)  => Inst::opcode1(0xdb).reg(3).rm_reg(i),
    },

    Fcom {
        M32fp(m) => Inst::opcode1(0xd8).reg(2).mem(m),
        M64fp(m) => Inst::opcode1(0xdc).reg(2).mem(m),
        Sti(i)   => Inst::opcode1(0xd8).reg(2).rm_reg(i),
    },
    Fcomp {
        M32fp(m) => Inst::opcode1(0xd8).reg(3).mem(m),
        M64fp(m) => Inst::opcode1(0xdc).reg(3).mem(m),
        Sti(i)   => Inst::opcode1(0xd8).reg(3).rm_reg(i),
    },
    Fcompp { Inst::opcode(Opcode::B2([0xde, 0xd9])) },

    Fcomi {
        St0Sti(i) => Inst::opcode1(0xdb).reg(6).rm_reg(i),
    },
    Fcomip {
        St0Sti(i) => Inst::opcode1(0xdf).reg(6).rm_reg(i),
    },
    Fucomi {
        St0Sti(i) => Inst::opcode1(0xdb).reg(5).rm_reg(i),
    },
    Fucomip {
        St0Sti(i) => Inst::opcode1(0xdf).reg(5).rm_reg(i),
    },

    Fcos { Inst::fopcode(0xff) },

    Fdecstp { Inst::fopcode(0xf6) },

    Fdiv {
        M32fp(m)  => Inst::opcode1(0xd8).reg(6).mem(m),
        M64fp(m)  => Inst::opcode1(0xdc).reg(6).mem(m),
        St0Sti(i) => Inst::opcode1(0xd8).reg(6).rm_reg(i),
        StiSt0(i) => Inst::opcode1(0xdc).reg(7).rm_reg(i),
    },
    Fdivp {
        StiSt0(i) => Inst::opcode1(0xde).reg(7).rm_reg(i),
    },
    Fidiv {
        M32int(m) => Inst::opcode1(0xda).reg(6).mem(m),
        M16int(m) => Inst::opcode1(0xde).reg(6).mem(m),
    },

    Fdivr {
        M32fp(m)  => Inst::opcode1(0xd8).reg(7).mem(m),
        M64fp(m)  => Inst::opcode1(0xdc).reg(7).mem(m),
        St0Sti(i) => Inst::opcode1(0xd8).reg(7).rm_reg(i),
        StiSt0(i) => Inst::opcode1(0xdc).reg(6).rm_reg(i),
    },
    Fdivrp {
        StiSt0(i) => Inst::opcode1(0xde).reg(6).rm_reg(i),
    },
    Fidivr {
        M32int(m) => Inst::opcode1(0xda).reg(7).mem(m),
        M16int(m) => Inst::opcode1(0xde).reg(7).mem(m),
    },

    Ffree {
        Sti(i) => Inst::opcode1(0xdd).reg(0).rm_reg(i),
    },

    Ficom {
        M16int(m) => Inst::opcode1(0xde).reg(2).mem(m),
        M32int(m) => Inst::opcode1(0xda).reg(2).mem(m),
    },
    Ficomp {
        M16int(m) => Inst::opcode1(0xde).reg(3).mem(m),
        M32int(m) => Inst::opcode1(0xda).reg(3).mem(m),
    },

    Fild {
        M16int(m) => Inst::opcode1(0xdf).reg(0).mem(m),
        M32int(m) => Inst::opcode1(0xdb).reg(0).mem(m),
        M64int(m) => Inst::opcode1(0xdf).reg(5).mem(m),
    },

    Fincstp { Inst::fopcode(0xf7) },

    Finit { Inst::opcode(Opcode::B3([0x9b, 0xdb, 0xe3])) },
    Fninit { Inst::opcode(Opcode::B2([0xdb, 0xe3])) },

    Fist {
        M16int(m) => Inst::opcode1(0xdf).reg(2).mem(m),
        M32int(m) => Inst::opcode1(0xdb).reg(2).mem(m),
    },
    Fistp {
        M16int(m) => Inst::opcode1(0xdf).reg(3).mem(m),
        M32int(m) => Inst::opcode1(0xdb).reg(3).mem(m),
        M64int(m) => Inst::opcode1(0xdf).reg(7).mem(m),
    },

    Fisttp {
        M16int(m) => Inst::opcode1(0xdf).reg(1).mem(m),
        M32int(m) => Inst::opcode1(0xdb).reg(1).mem(m),
        M64int(m) => Inst::opcode1(0xdd).reg(1).mem(m),
    },

    Fld {
        M32fp(m) => Inst::opcode1(0xd9).reg(0).mem(m),
        M64fp(m) => Inst::opcode1(0xdd).reg(0).mem(m),
        M80fp(m) => Inst::opcode1(0xdb).reg(5).mem(m),
        Sti(i)   => Inst::opcode1(0xd9).reg(0).rm_reg(i),
    },

    Fld1   { Inst::fopcode(0xe8) },
    Fldl2t { Inst::fopcode(0xe9) },
    Fldl2e { Inst::fopcode(0xea) },
    Fldpi  { Inst::fopcode(0xeb) },
    Fldlg2 { Inst::fopcode(0xec) },
    Fldln2 { Inst::fopcode(0xed) },
    Fldz   { Inst::fopcode(0xee) },

    Fldcw {
        M2byte(m) => Inst::opcode1(0xd9).reg(5).mem(m),
    },

    Fldenv {
        M28byte(m) => Inst::opcode1(0xd9).reg(4).mem(m),
    },

    Fmul {
        M32fp(m)  => Inst::opcode1(0xd8).reg(1).mem(m),
        M64fp(m)  => Inst::opcode1(0xdc).reg(1).mem(m),
        St0Sti(i) => Inst::opcode1(0xd8).reg(1).rm_reg(i),
        StiSt0(i) => Inst::opcode1(0xdc).reg(1).rm_reg(i),
    },
    Fmulp {
        StiSt0(i) => Inst::opcode1(0xde).reg(1).rm_reg(i),
    },
    Fimul {
        M32int(m) => Inst::opcode1(0xda).reg(1).mem(m),
        M16int(m) => Inst::opcode1(0xde).reg(1).mem(m),
    },

    Fnop { Inst::fopcode(0xd0) },

    Fpatan { Inst::fopcode(0xf3) },

    Fprem { Inst::fopcode(0xf8) },

    Fprem1 { Inst::fopcode(0xf5) },

    Fptan { Inst::fopcode(0xf2) },

    Frndint { Inst::fopcode(0xfc) },

    Frstor {
        M108byte(m) => Inst::opcode1(0xdd).reg(4).mem(m),
    },

    Fsave {
        M108byte(m) => Inst::opcode(Opcode::B2([0x9b, 0xdd])).reg(6).mem(m),
    },
    Fnsave {
        M108byte(m) => Inst::opcode1(0xdd).reg(6).mem(m),
    },

    Fscale { Inst::fopcode(0xfd) },

    Fsin { Inst::fopcode(0xfe) },

    Fsincos { Inst::fopcode(0xfb) },

    Fsqrt { Inst::fopcode(0xfa) },

    Fst {
        M32fp(m) => Inst::opcode1(0xd9).reg(2).mem(m),
        M64fp(m) => Inst::opcode1(0xdd).reg(2).mem(m),
        Sti(i)   => Inst::opcode1(0xdd).reg(2).rm_reg(i),
    },
    Fstp {
        M32fp(m) => Inst::opcode1(0xd9).reg(3).mem(m),
        M64fp(m) => Inst::opcode1(0xdd).reg(3).mem(m),
        M80fp(m) => Inst::opcode1(0xdb).reg(7).mem(m),
        Sti(i)   => Inst::opcode1(0xdd).reg(3).rm_reg(i),
    },

    Fstcw {
        M2byte(m) => Inst::opcode(Opcode::B2([0x9b, 0xd9])).reg(7).mem(m),
    },
    Fnstcw {
        M2byte(m) => Inst::opcode1(0xd9).reg(7).mem(m),
    },

    Fstenv {
        M28byte(m) => Inst::opcode(Opcode::B2([0x9b, 0xd9])).reg(6).mem(m),
    },
    Fnstenv {
        M28byte(m) => Inst::opcode1(0xd9).reg(6).mem(m),
    },

    Fstsw {
        M2byte(m) => Inst::opcode(Opcode::B2([0x9b, 0xdd])).reg(7).mem(m),
        Ax        => Inst::opcode(Opcode::B3([0x9b, 0xdf, 0xe0])),
    },
    Fnstsw {
        M2byte(m) => Inst::opcode1(0xdd).reg(7).mem(m),
        Ax        => Inst::opcode(Opcode::B2([0xdf, 0xe0])),
    },

    Fsub {
        M32fp(m)  => Inst::opcode1(0xd8).reg(4).mem(m),
        M64fp(m)  => Inst::opcode1(0xdc).reg(4).mem(m),
        St0Sti(i) => Inst::opcode1(0xd8).reg(4).rm_reg(i),
        StiSt0(i) => Inst::opcode1(0xdc).reg(5).rm_reg(i),
    },
    Fsubp {
        StiSt0(i) => Inst::opcode1(0xde).reg(5).rm_reg(i),
    },
    Fisub {
        M32int(m) => Inst::opcode1(0xda).reg(4).mem(m),
        M16int(m) => Inst::opcode1(0xde).reg(4).mem(m),
    },

    Fsubr {
        M32fp(m)  => Inst::opcode1(0xd8).reg(5).mem(m),
        M64fp(m)  => Inst::opcode1(0xdc).reg(5).mem(m),
        St0Sti(i) => Inst::opcode1(0xd8).reg(5).rm_reg(i),
        StiSt0(i) => Inst::opcode1(0xdc).reg(4).rm_reg(i),
    },
    Fsubrp {
        StiSt0(i) => Inst::opcode1(0xde).reg(4).rm_reg(i),
    },
    Fisubr {
        M32int(m) => Inst::opcode1(0xda).reg(5).mem(m),
        M16int(m) => Inst::opcode1(0xde).reg(5).mem(m),
    },

    Ftst { Inst::fopcode(0xe4) },

    Fucom {
        Sti(i) => Inst::opcode1(0xdd).reg(4).rm_reg(i),
    },
    Fucomp {
        Sti(i) => Inst::opcode1(0xdd).reg(5).rm_reg(i),
    },
    Fucompp { Inst::opcode1(0xda).reg(5).rm_reg(1) },

    Fxam { Inst::fopcode(0xe5) },

    Fxch {
        Sti(i) => Inst::opcode1(0xd9).reg(1).rm_reg(i),
    },

    Fxrstor {
        M512byte(m) => Inst::opcode2(0xae).reg(1).mem(m),
    },
    Fxrstor64 {
        M512byte(m) => Inst::opcode2(0xae).reg(1).mem(m).rex_w(),
    },

    Fxsave {
        M512byte(m) => Inst::opcode2(0xae).reg(0).mem(m),
    },
    Fxsave64 {
        M512byte(m) => Inst::opcode2(0xae).reg(0).mem(m),
    },

    Fxtract { Inst::fopcode(0xf4) },

    Fyl2x { Inst::fopcode(0xf1) },

    Fyl2xp1 { Inst::fopcode(0xf9) },

    Hlt { Inst::opcode1(0xf4) },

    Idiv(7),

    Imul {
        Rm8l(rm) => Inst::opcode1(0xf6).reg(5).rm(rm),
        Rm8(rm)  => Inst::opcode1(0xf6).reg(5).rm(rm),
        Rm16(rm) => Inst::opcode1(0xf7).reg(5).rm(rm).osz(),
        Rm32(rm) => Inst::opcode1(0xf7).reg(5).rm(rm),
        Rm64(rm) => Inst::opcode1(0xf7).reg(5).rm(rm).rex_w(),

        R16Rm16(r, rm) => Inst::opcode2(0xaf).reg(r).rm(rm).osz(),
        R32Rm32(r, rm) => Inst::opcode2(0xaf).reg(r).rm(rm),
        R64Rm64(r, rm) => Inst::opcode2(0xaf).reg(r).rm(rm).rex_w(),

        R16Rm16Imm8(r, rm, imm) => Inst::opcode1(0x6b).reg(r).rm(rm).imm8(imm).osz(),
        R32Rm32Imm8(r, rm, imm) => Inst::opcode1(0x6b).reg(r).rm(rm).imm8(imm),
        R64Rm64Imm8(r, rm, imm) => Inst::opcode1(0x6b).reg(r).rm(rm).imm8(imm).rex_w(),

        R16Rm16Imm16(r, rm, imm) => Inst::opcode1(0x69).reg(r).rm(rm).imm16(imm).osz(),
        R32Rm32Imm32(r, rm, imm) => Inst::opcode1(0x69).reg(r).rm(rm).imm32(imm),
        R64Rm64Imm32(r, rm, imm) => Inst::opcode1(0x69).reg(r).rm(rm).imm32(imm).rex_w(),
    },

    In {
        AlImm8(imm)  => Inst::opcode1(0xe4).imm8(imm),
        AxImm8(imm)  => Inst::opcode1(0xe5).imm8(imm).osz(),
        EaxImm8(imm) => Inst::opcode1(0xe5).imm8(imm),

        AlDx  => Inst::opcode1(0xec),
        AxDx  => Inst::opcode1(0xed).osz(),
        EaxDx => Inst::opcode1(0xed),
    },

    Inc {
        Rm8l(rm) => Inst::opcode1(0xfe).reg(0).rm(rm),
        Rm8(rm)  => Inst::opcode1(0xfe).reg(0).rm(rm),
        Rm16(rm) => Inst::opcode1(0xff).reg(0).rm(rm).osz(),
        Rm32(rm) => Inst::opcode1(0xff).reg(0).rm(rm),
        Rm64(rm) => Inst::opcode1(0xff).reg(0).rm(rm).rex_w(),
    },

    Ins {
        B => Inst::opcode1(0x6c),
        W => Inst::opcode1(0x6d).osz(),
        D => Inst::opcode1(0x6d),
    },

    Int3 { Inst::opcode1(0xcc) },
    Int {
        Imm8(imm) => { Inst::opcode1(0xcd).imm8(imm) },
    },
    Into { Inst::opcode1(0xce) },

    Invd { Inst::opcode2(0x08) },

    Invlpg {
        M(m) => Inst::opcode2(0x01).reg(7).mem(m),
    },

    Invpcid {
        R64M128(r, m) => Inst::opcode3(0x38, 0x82).osz().reg(r).mem(m),
    },

    Iret {
        D => Inst::opcode1(0xcf),
        Q => Inst::opcode1(0xcf).rex_w(),
    },

    J {
        CcRel8(cc, rel)  => Inst::opcode1(0x70).cc(cc).disp(rel.0),
        RcxzRel8(rel)    => Inst::opcode1(0xe3).disp(rel.0),
        CcRel32(cc, rel) => Inst::opcode2(0x80).cc(cc).disp(rel.0),
    },

    Jmp {
        Rel8(rel)  => Inst::opcode1(0xeb).disp(rel.0),
        Rel32(rel) => Inst::opcode1(0xe9).disp(rel.0),
        Rm64(rm)   => Inst::opcode1(0xff).reg(4).rm(rm),
        M16x16(m)  => Inst::opcode1(0xff).reg(5).mem(m).osz(),
        M16x32(m)  => Inst::opcode1(0xff).reg(5).mem(m),
        M16x64(m)  => Inst::opcode1(0xff).reg(5).mem(m).rex_w(),
    },

    Lahf { Inst::opcode1(0x9f) },

    Lss {
        R16M16x16(r, m) => Inst::opcode2(0xb2).reg(r).mem(m).osz(),
        R32M16x32(r, m) => Inst::opcode2(0xb2).reg(r).mem(m),
        R64M16x64(r, m) => Inst::opcode2(0xb2).reg(r).mem(m).rex_w(),
    },
    Lfs {
        R16M16x16(r, m) => Inst::opcode2(0xb4).reg(r).mem(m).osz(),
        R32M16x32(r, m) => Inst::opcode2(0xb4).reg(r).mem(m),
        R64M16x64(r, m) => Inst::opcode2(0xb4).reg(r).mem(m).rex_w(),
    },
    Lgs {
        R16M16x16(r, m) => Inst::opcode2(0xb5).reg(r).mem(m).osz(),
        R32M16x32(r, m) => Inst::opcode2(0xb5).reg(r).mem(m),
        R64M16x64(r, m) => Inst::opcode2(0xb5).reg(r).mem(m).rex_w(),
    },

    Lea {
        R16M(r, m) => Inst::opcode1(0x8d).reg(r).mem(m).osz(),
        R32M(r, m) => Inst::opcode1(0x8d).reg(r).mem(m),
        R64M(r, m) => Inst::opcode1(0x8d).reg(r).mem(m).rex_w(),
    },

    Leave { Inst::opcode1(0xc9) },

    Lfence { Inst::opcode3(0xae, 0xe8) },

    Lgdt {
        M16x64(m) => Inst::opcode2(0x01).reg(2).mem(m),
    },
    Lidt {
        M16x64(m) => Inst::opcode2(0x01).reg(3).mem(m),
    },

    Lldt {
        Rm16(rm) => Inst::opcode2(0x00).reg(2).rm(rm),
    },

    Lmsw {
        Rm16(rm) => Inst::opcode2(0x01).reg(6).rm(rm),
    },

    Lods {
        B => Inst::opcode1(0xac),
        W => Inst::opcode1(0xad).osz(),
        D => Inst::opcode1(0xad),
        Q => Inst::opcode1(0xad).rex_w(),
    },

    Loop {
        Rel8(rel)   => Inst::opcode1(0xe2).disp(rel.0),
        ERel8(rel)  => Inst::opcode1(0xe1).disp(rel.0),
        NeRel8(rel) => Inst::opcode1(0xe0).disp(rel.0),
    },

    Ltr {
        Rm16(rm) => Inst::opcode2(0x00).reg(3).rm(rm),
    },

    Lzcnt {
        R16Rm16(r, rm) => Inst::opcode2(0xbd).rep().reg(r).rm(rm).osz(),
        R32Rm32(r, rm) => Inst::opcode2(0xbd).rep().reg(r).rm(rm),
        R64Rm64(r, rm) => Inst::opcode2(0xbd).rep().reg(r).rm(rm).rex_w(),
    },

    Mfence { Inst::opcode3(0xae, 0xf0) },

    Monitor { Inst::opcode3(0x01, 0xc8) },

    Mov {
        Rm8lR8l(rm, r) => Inst::opcode1(0x88).rm(rm).reg(r),
        Rm8R8(rm, r)   => Inst::opcode1(0x88).rm(rm).reg(r),
        Rm16R16(rm, r) => Inst::opcode1(0x89).rm(rm).reg(r).osz(),
        Rm32R32(rm, r) => Inst::opcode1(0x89).rm(rm).reg(r),
        Rm64R64(rm, r) => Inst::opcode1(0x89).rm(rm).reg(r).rex_w(),

        R8lRm8l(r, rm) => Inst::opcode1(0x8a).reg(r).rm(rm),
        R8Rm8(r, rm)   => Inst::opcode1(0x8a).reg(r).rm(rm),
        R16Rm16(r, rm) => Inst::opcode1(0x8b).reg(r).rm(rm).osz(),
        R32Rm32(r, rm) => Inst::opcode1(0x8b).reg(r).rm(rm),
        R64Rm64(r, rm) => Inst::opcode1(0x8b).reg(r).rm(rm).rex_w(),

        Rm16Sreg(rm, sreg) => Inst::opcode1(0x8c).rm(rm).reg(sreg),
        Rm64Sreg(rm, sreg) => Inst::opcode1(0x8c).rm(rm).reg(sreg).rex_w(),

        SregRm16(sreg, rm) => Inst::opcode1(0x8e).rm(rm).reg(sreg),
        SregRm64(sreg, rm) => Inst::opcode1(0x8e).rm(rm).reg(sreg).rex_w(),

        AlMoffs8(moffs)   => Inst::opcode1(0xa0).moffs(moffs),
        AxMoffs16(moffs)  => Inst::opcode1(0xa1).moffs(moffs).osz(),
        EaxMoffs32(moffs) => Inst::opcode1(0xa1).moffs(moffs),
        RaxMoffs64(moffs) => Inst::opcode1(0xa1).moffs(moffs).rex_w(),

        Moffs8Al(moffs)   => Inst::opcode1(0xa2).moffs(moffs),
        Moffs16Ax(moffs)  => Inst::opcode1(0xa3).moffs(moffs).osz(),
        Moffs32Eax(moffs) => Inst::opcode1(0xa3).moffs(moffs),
        Moffs64Rax(moffs) => Inst::opcode1(0xa3).moffs(moffs).rex_w(),

        R8lImm8(r, imm)  => Inst::opcode1(0xb0).plus(r).imm8(imm),
        R8Imm8(r, imm)   => Inst::opcode1(0xb0).plus(r).imm8(imm),
        R16Imm16(r, imm) => Inst::opcode1(0xb8).plus(r).imm16(imm).osz(),
        R32Imm32(r, imm) => Inst::opcode1(0xb8).plus(r).imm32(imm),
        R64Imm64(r, imm) => Inst::opcode1(0xb8).plus(r).imm64(imm).rex_w(),

        Rm8lImm8(rm, imm)  => Inst::opcode1(0xc6).reg(0).rm(rm).imm8(imm),
        Rm8Imm8(rm, imm)   => Inst::opcode1(0xc6).reg(0).rm(rm).imm8(imm),
        Rm16Imm16(rm, imm) => Inst::opcode1(0xc7).reg(0).rm(rm).imm16(imm).osz(),
        Rm32Imm32(rm, imm) => Inst::opcode1(0xc7).reg(0).rm(rm).imm32(imm),
        Rm64Imm32(rm, imm) => Inst::opcode1(0xc7).reg(0).rm(rm).imm32(imm).rex_w(),

        R64Cr(r, cr) => Inst::opcode2(0x20).rm_reg(r).reg(cr),
        CrR64(cr, r) => Inst::opcode2(0x22).reg(cr).rm_reg(r),

        R64Dr(r, dr) => Inst::opcode2(0x21).rm_reg(r).reg(dr),
        DrR64(dr, r) => Inst::opcode2(0x23).reg(dr).rm_reg(r),
    },

    Movbe {
        R16M16(r, m) => Inst::opcode3(0x38, 0xf0).reg(r).mem(m).osz(),
        R32M32(r, m) => Inst::opcode3(0x38, 0xf0).reg(r).mem(m),
        R64M64(r, m) => Inst::opcode3(0x38, 0xf0).reg(r).mem(m).rex_w(),

        M16R16(m, r) => Inst::opcode3(0x38, 0xf1).mem(m).reg(r).osz(),
        M32R32(m, r) => Inst::opcode3(0x38, 0xf1).mem(m).reg(r),
        M64R64(m, r) => Inst::opcode3(0x38, 0xf1).mem(m).reg(r).rex_w(),
    },

    Movs {
        B => Inst::opcode1(0xa4),
        W => Inst::opcode1(0xa5).osz(),
        D => Inst::opcode1(0xa5),
        Q => Inst::opcode1(0xa5).rex_w(),
    },

    Movsx {
        R16lRm8l(r, rm) => Inst::opcode2(0xbe).reg(r).rm(rm).osz(),
        R16Rm8(r, rm)   => Inst::opcode2(0xbe).reg(r).rm(rm).osz(),
        R32lRm8l(r, rm) => Inst::opcode2(0xbe).reg(r).rm(rm),
        R32Rm8(r, rm)   => Inst::opcode2(0xbe).reg(r).rm(rm),
        R64Rm8(r, rm)   => Inst::opcode2(0xbe).reg(r).rm(rm).rex_w(),

        R32Rm16(r, rm) => Inst::opcode2(0xbf).reg(r).rm(rm),
        R64Rm16(r, rm) => Inst::opcode2(0xbf).reg(r).rm(rm).rex_w(),

        R64Rm32(r, rm) => Inst::opcode1(0x63).reg(r).rm(rm).rex_w(),
    },

    Movzx {
        R16lRm8l(r, rm) => Inst::opcode2(0xb6).reg(r).rm(rm).osz(),
        R16Rm8(r, rm)   => Inst::opcode2(0xb6).reg(r).rm(rm).osz(),
        R32lRm8l(r, rm) => Inst::opcode2(0xb6).reg(r).rm(rm),
        R32Rm8(r, rm)   => Inst::opcode2(0xb6).reg(r).rm(rm),
        R64Rm8(r, rm)   => Inst::opcode2(0xb6).reg(r).rm(rm).rex_w(),

        R32Rm16(r, rm) => Inst::opcode2(0xb7).reg(r).rm(rm),
        R64Rm16(r, rm) => Inst::opcode2(0xb7).reg(r).rm(rm).rex_w(),
    },

    Mul(4),

    Mwait { Inst::opcode3(0x01, 0xc9) },

    Neg(3),

    Nop {
        Ax  => Inst::opcode1(0x90).osz(),
        Eax => Inst::opcode1(0x90),

        Rm16(rm) => Inst::opcode2(0x1f).reg(0).rm(rm).osz(),
        Rm32(rm) => Inst::opcode2(0x1f).reg(0).rm(rm),
    },

    Not(2),

    Or(0x08, 1),

    Out {
        Imm8Al(imm)  => Inst::opcode1(0xe6).imm8(imm),
        Imm8Ax(imm)  => Inst::opcode1(0xe7).imm8(imm).osz(),
        Imm8Eax(imm) => Inst::opcode1(0xe7).imm8(imm),

        DxAl  => Inst::opcode1(0xee),
        DxAx  => Inst::opcode1(0xef).osz(),
        DxEax => Inst::opcode1(0xef),
    },

    Outs {
        B => Inst::opcode1(0x6e),
        W => Inst::opcode1(0x6f).osz(),
        D => Inst::opcode1(0x6f),
    },

    Pause { Inst::opcode1(0x90).rep() },

    Pop {
        Rm16(rm) => Inst::opcode1(0x8f).reg(0).rm(rm).osz(),
        Rm64(rm) => Inst::opcode1(0x8f).reg(0).rm(rm),

        R16(r) => Inst::opcode1(0x58).plus(r).osz(),
        R64(r) => Inst::opcode1(0x58).plus(r),

        Fs16 => Inst::opcode2(0xa1).osz(),
        Fs64 => Inst::opcode2(0xa1),
        Gs16 => Inst::opcode2(0xa9).osz(),
        Gs64 => Inst::opcode2(0xa9),
    },

    Popcnt {
        R16Rm16(r, rm) => Inst::opcode2(0xb8).rep().reg(r).rm(rm).osz(),
        R32Rm32(r, rm) => Inst::opcode2(0xb8).rep().reg(r).rm(rm),
        R64Rm64(r, rm) => Inst::opcode2(0xb8).rep().reg(r).rm(rm).rex_w(),
    },

    Popf {
        W => Inst::opcode1(0x9d).osz(),
        Q => Inst::opcode1(0x9d),
    },

    Prefetch {
        T0M8(m)  => Inst::opcode2(0x18).reg(1).mem(m),
        T1M8(m)  => Inst::opcode2(0x18).reg(2).mem(m),
        T2M8(m)  => Inst::opcode2(0x18).reg(3).mem(m),
        NtaM8(m) => Inst::opcode2(0x18).reg(0).mem(m),
    },

    Prefetchw {
        M8(m) => Inst::opcode2(0x0d).reg(1).mem(m),
    },

    Prefetchwt1 {
        M8(m) => Inst::opcode2(0x0d).reg(2).mem(m),
    },

    Ptwrite {
        Rm32(rm) => Inst::opcode2(0xae).rep().reg(4).rm(rm),
        Rm64(rm) => Inst::opcode2(0xae).rep().reg(4).rm(rm).rex_w(),
    },

    Push {
        Rm16(rm) => Inst::opcode1(0xff).reg(6).rm(rm).osz(),
        Rm64(rm) => Inst::opcode1(0xff).reg(6).rm(rm),

        R16(r) => Inst::opcode1(0x50).plus(r).osz(),
        R64(r) => Inst::opcode1(0x50).plus(r),

        Imm8(imm)  => Inst::opcode1(0x6a).imm8(imm),
        Imm16(imm) => Inst::opcode1(0x68).imm16(imm).osz(),
        Imm32(imm) => Inst::opcode1(0x68).imm32(imm),

        Fs16 => Inst::opcode2(0xa0).osz(),
        Fs64 => Inst::opcode2(0xa0),
        Gs16 => Inst::opcode2(0xa8).osz(),
        Gs64 => Inst::opcode2(0xa8),
    },

    Pushf {
        W => Inst::opcode1(0x9c).osz(),
        Q => Inst::opcode1(0x9c),
    },

    Rcl {
        Rm8l(rm)          => Inst::opcode1(0xd0).reg(2).rm(rm),
        Rm8(rm)           => Inst::opcode1(0xd0).reg(2).rm(rm),
        Rm8lCl(rm)        => Inst::opcode1(0xd2).reg(2).rm(rm),
        Rm8Cl(rm)         => Inst::opcode1(0xd2).reg(2).rm(rm),
        Rm8lImm8(rm, imm) => Inst::opcode1(0xc0).reg(2).rm(rm).imm8(imm),
        Rm8Imm8(rm, imm)  => Inst::opcode1(0xc0).reg(2).rm(rm).imm8(imm),

        Rm16(rm)          => Inst::opcode1(0xd1).reg(2).rm(rm).osz(),
        Rm16Cl(rm)        => Inst::opcode1(0xd3).reg(2).rm(rm).osz(),
        Rm16Imm8(rm, imm) => Inst::opcode1(0xc1).reg(2).rm(rm).imm8(imm).osz(),

        Rm32(rm)          => Inst::opcode1(0xd1).reg(2).rm(rm),
        Rm64(rm)          => Inst::opcode1(0xd1).reg(2).rm(rm).rex_w(),
        Rm32Cl(rm)        => Inst::opcode1(0xd3).reg(2).rm(rm),
        Rm64Cl(rm)        => Inst::opcode1(0xd3).reg(2).rm(rm).rex_w(),
        Rm32Imm8(rm, imm) => Inst::opcode1(0xc1).reg(2).rm(rm).imm8(imm),
        Rm64Imm8(rm, imm) => Inst::opcode1(0xc1).reg(2).rm(rm).imm8(imm).rex_w(),
    },

    Rcr {
        Rm8l(rm)          => Inst::opcode1(0xd0).reg(3).rm(rm),
        Rm8(rm)           => Inst::opcode1(0xd0).reg(3).rm(rm),
        Rm8lCl(rm)        => Inst::opcode1(0xd2).reg(3).rm(rm),
        Rm8Cl(rm)         => Inst::opcode1(0xd2).reg(3).rm(rm),
        Rm8lImm8(rm, imm) => Inst::opcode1(0xc0).reg(3).rm(rm).imm8(imm),
        Rm8Imm8(rm, imm)  => Inst::opcode1(0xc0).reg(3).rm(rm).imm8(imm),

        Rm16(rm)          => Inst::opcode1(0xd1).reg(3).rm(rm).osz(),
        Rm16Cl(rm)        => Inst::opcode1(0xd3).reg(3).rm(rm).osz(),
        Rm16Imm8(rm, imm) => Inst::opcode1(0xc1).reg(3).rm(rm).imm8(imm).osz(),

        Rm32(rm)          => Inst::opcode1(0xd1).reg(3).rm(rm),
        Rm64(rm)          => Inst::opcode1(0xd1).reg(3).rm(rm).rex_w(),
        Rm32Cl(rm)        => Inst::opcode1(0xd3).reg(3).rm(rm),
        Rm64Cl(rm)        => Inst::opcode1(0xd3).reg(3).rm(rm).rex_w(),
        Rm32Imm8(rm, imm) => Inst::opcode1(0xc1).reg(3).rm(rm).imm8(imm),
        Rm64Imm8(rm, imm) => Inst::opcode1(0xc1).reg(3).rm(rm).imm8(imm).rex_w(),
    },

    Rol {
        Rm8l(rm)          => Inst::opcode1(0xd0).reg(0).rm(rm),
        Rm8(rm)           => Inst::opcode1(0xd0).reg(0).rm(rm),
        Rm8lCl(rm)        => Inst::opcode1(0xd2).reg(0).rm(rm),
        Rm8Cl(rm)         => Inst::opcode1(0xd2).reg(0).rm(rm),
        Rm8lImm8(rm, imm) => Inst::opcode1(0xc0).reg(0).rm(rm).imm8(imm),
        Rm8Imm8(rm, imm)  => Inst::opcode1(0xc0).reg(0).rm(rm).imm8(imm),

        Rm16(rm)          => Inst::opcode1(0xd1).reg(0).rm(rm).osz(),
        Rm16Cl(rm)        => Inst::opcode1(0xd3).reg(0).rm(rm).osz(),
        Rm16Imm8(rm, imm) => Inst::opcode1(0xc1).reg(0).rm(rm).imm8(imm).osz(),

        Rm32(rm)          => Inst::opcode1(0xd1).reg(0).rm(rm),
        Rm64(rm)          => Inst::opcode1(0xd1).reg(0).rm(rm).rex_w(),
        Rm32Cl(rm)        => Inst::opcode1(0xd3).reg(0).rm(rm),
        Rm64Cl(rm)        => Inst::opcode1(0xd3).reg(0).rm(rm).rex_w(),
        Rm32Imm8(rm, imm) => Inst::opcode1(0xc1).reg(0).rm(rm).imm8(imm),
        Rm64Imm8(rm, imm) => Inst::opcode1(0xc1).reg(0).rm(rm).imm8(imm).rex_w(),
    },

    Ror {
        Rm8l(rm)          => Inst::opcode1(0xd0).reg(1).rm(rm),
        Rm8(rm)           => Inst::opcode1(0xd0).reg(1).rm(rm),
        Rm8lCl(rm)        => Inst::opcode1(0xd2).reg(1).rm(rm),
        Rm8Cl(rm)         => Inst::opcode1(0xd2).reg(1).rm(rm),
        Rm8lImm8(rm, imm) => Inst::opcode1(0xc0).reg(1).rm(rm).imm8(imm),
        Rm8Imm8(rm, imm)  => Inst::opcode1(0xc0).reg(1).rm(rm).imm8(imm),

        Rm16(rm)          => Inst::opcode1(0xd1).reg(1).rm(rm).osz(),
        Rm16Cl(rm)        => Inst::opcode1(0xd3).reg(1).rm(rm).osz(),
        Rm16Imm8(rm, imm) => Inst::opcode1(0xc1).reg(1).rm(rm).imm8(imm).osz(),

        Rm32(rm)          => Inst::opcode1(0xd1).reg(1).rm(rm),
        Rm64(rm)          => Inst::opcode1(0xd1).reg(1).rm(rm).rex_w(),
        Rm32Cl(rm)        => Inst::opcode1(0xd3).reg(1).rm(rm),
        Rm64Cl(rm)        => Inst::opcode1(0xd3).reg(1).rm(rm).rex_w(),
        Rm32Imm8(rm, imm) => Inst::opcode1(0xc1).reg(1).rm(rm).imm8(imm),
        Rm64Imm8(rm, imm) => Inst::opcode1(0xc1).reg(1).rm(rm).imm8(imm).rex_w(),
    },

    Rdfsbase {
        R32(r) => Inst::opcode2(0xae).rep().reg(0).rm_reg(r),
        R64(r) => Inst::opcode2(0xae).rep().reg(0).rm_reg(r).rex_w(),
    },

    Rdgsbase {
        R32(r) => Inst::opcode2(0xae).rep().reg(1).rm_reg(r),
        R64(r) => Inst::opcode2(0xae).rep().reg(1).rm_reg(r).rex_w(),
    },

    Rdmsr { Inst::opcode2(0x32) },

    Rdpid {
        R64(r) => Inst::opcode2(0xc7).rep().reg(7).rm_reg(r),
    },

    Rdpkru { Inst::opcode3(0x01, 0xee) },

    Rdpmc { Inst::opcode2(0x33) },

    Rdrand {
        R16(r) => Inst::opcode2(0xc7).reg(6).rm_reg(r).osz(),
        R32(r) => Inst::opcode2(0xc7).reg(6).rm_reg(r),
        R64(r) => Inst::opcode2(0xc7).reg(6).rm_reg(r).rex_w(),
    },

    Rdseed {
        R16(r) => Inst::opcode2(0xc7).reg(7).rm_reg(r).osz(),
        R32(r) => Inst::opcode2(0xc7).reg(7).rm_reg(r),
        R64(r) => Inst::opcode2(0xc7).reg(7).rm_reg(r).rex_w(),
    },

    Rdtsc { Inst::opcode2(0x31) },

    Rdtscp { Inst::opcode3(0x01, 0xf9) },

    Ret {
        Near => Inst::opcode1(0xc3),
        Far  => Inst::opcode1(0xcb),

        NearImm16(imm) => Inst::opcode1(0xc2).imm16(imm),
        FarImm16(imm)  => Inst::opcode1(0xca).imm16(imm),
    },

    Rsm { Inst::opcode2(0xaa) },

    Sahf { Inst::opcode1(0x9e) },

    Sal {
        Rm8l(rm)          => Inst::opcode1(0xd0).reg(4).rm(rm),
        Rm8(rm)           => Inst::opcode1(0xd0).reg(4).rm(rm),
        Rm8lCl(rm)        => Inst::opcode1(0xd2).reg(4).rm(rm),
        Rm8Cl(rm)         => Inst::opcode1(0xd2).reg(4).rm(rm),
        Rm8lImm8(rm, imm) => Inst::opcode1(0xc0).reg(4).rm(rm).imm8(imm),
        Rm8Imm8(rm, imm)  => Inst::opcode1(0xc0).reg(4).rm(rm).imm8(imm),

        Rm16(rm)          => Inst::opcode1(0xd1).reg(4).rm(rm).osz(),
        Rm16Cl(rm)        => Inst::opcode1(0xd3).reg(4).rm(rm).osz(),
        Rm16Imm8(rm, imm) => Inst::opcode1(0xc1).reg(4).rm(rm).imm8(imm).osz(),

        Rm32(rm)          => Inst::opcode1(0xd1).reg(4).rm(rm),
        Rm64(rm)          => Inst::opcode1(0xd1).reg(4).rm(rm).rex_w(),
        Rm32Cl(rm)        => Inst::opcode1(0xd3).reg(4).rm(rm),
        Rm64Cl(rm)        => Inst::opcode1(0xd3).reg(4).rm(rm).rex_w(),
        Rm32Imm8(rm, imm) => Inst::opcode1(0xc1).reg(4).rm(rm).imm8(imm),
        Rm64Imm8(rm, imm) => Inst::opcode1(0xc1).reg(4).rm(rm).imm8(imm).rex_w(),
    },

    Sar {
        Rm8l(rm)          => Inst::opcode1(0xd0).reg(7).rm(rm),
        Rm8(rm)           => Inst::opcode1(0xd0).reg(7).rm(rm),
        Rm8lCl(rm)        => Inst::opcode1(0xd2).reg(7).rm(rm),
        Rm8Cl(rm)         => Inst::opcode1(0xd2).reg(7).rm(rm),
        Rm8lImm8(rm, imm) => Inst::opcode1(0xc0).reg(7).rm(rm).imm8(imm),
        Rm8Imm8(rm, imm)  => Inst::opcode1(0xc0).reg(7).rm(rm).imm8(imm),

        Rm16(rm)          => Inst::opcode1(0xd1).reg(7).rm(rm).osz(),
        Rm16Cl(rm)        => Inst::opcode1(0xd3).reg(7).rm(rm).osz(),
        Rm16Imm8(rm, imm) => Inst::opcode1(0xc1).reg(7).rm(rm).imm8(imm).osz(),

        Rm32(rm)          => Inst::opcode1(0xd1).reg(7).rm(rm),
        Rm64(rm)          => Inst::opcode1(0xd1).reg(7).rm(rm).rex_w(),
        Rm32Cl(rm)        => Inst::opcode1(0xd3).reg(7).rm(rm),
        Rm64Cl(rm)        => Inst::opcode1(0xd3).reg(7).rm(rm).rex_w(),
        Rm32Imm8(rm, imm) => Inst::opcode1(0xc1).reg(7).rm(rm).imm8(imm),
        Rm64Imm8(rm, imm) => Inst::opcode1(0xc1).reg(7).rm(rm).imm8(imm).rex_w(),
    },

    Shl {
        Rm8l(rm)          => Inst::opcode1(0xd0).reg(4).rm(rm),
        Rm8(rm)           => Inst::opcode1(0xd0).reg(4).rm(rm),
        Rm8lCl(rm)        => Inst::opcode1(0xd2).reg(4).rm(rm),
        Rm8Cl(rm)         => Inst::opcode1(0xd2).reg(4).rm(rm),
        Rm8lImm8(rm, imm) => Inst::opcode1(0xc0).reg(4).rm(rm).imm8(imm),
        Rm8Imm8(rm, imm)  => Inst::opcode1(0xc0).reg(4).rm(rm).imm8(imm),

        Rm16(rm)          => Inst::opcode1(0xd1).reg(4).rm(rm).osz(),
        Rm16Cl(rm)        => Inst::opcode1(0xd3).reg(4).rm(rm).osz(),
        Rm16Imm8(rm, imm) => Inst::opcode1(0xc1).reg(4).rm(rm).imm8(imm).osz(),

        Rm32(rm)          => Inst::opcode1(0xd1).reg(4).rm(rm),
        Rm64(rm)          => Inst::opcode1(0xd1).reg(4).rm(rm).rex_w(),
        Rm32Cl(rm)        => Inst::opcode1(0xd3).reg(4).rm(rm),
        Rm64Cl(rm)        => Inst::opcode1(0xd3).reg(4).rm(rm).rex_w(),
        Rm32Imm8(rm, imm) => Inst::opcode1(0xc1).reg(4).rm(rm).imm8(imm),
        Rm64Imm8(rm, imm) => Inst::opcode1(0xc1).reg(4).rm(rm).imm8(imm).rex_w(),
    },

    Shr {
        Rm8l(rm)          => Inst::opcode1(0xd0).reg(5).rm(rm),
        Rm8(rm)           => Inst::opcode1(0xd0).reg(5).rm(rm),
        Rm8lCl(rm)        => Inst::opcode1(0xd2).reg(5).rm(rm),
        Rm8Cl(rm)         => Inst::opcode1(0xd2).reg(5).rm(rm),
        Rm8lImm8(rm, imm) => Inst::opcode1(0xc0).reg(5).rm(rm).imm8(imm),
        Rm8Imm8(rm, imm)  => Inst::opcode1(0xc0).reg(5).rm(rm).imm8(imm),

        Rm16(rm)          => Inst::opcode1(0xd1).reg(5).rm(rm).osz(),
        Rm16Cl(rm)        => Inst::opcode1(0xd3).reg(5).rm(rm).osz(),
        Rm16Imm8(rm, imm) => Inst::opcode1(0xc1).reg(5).rm(rm).imm8(imm).osz(),

        Rm32(rm)          => Inst::opcode1(0xd1).reg(5).rm(rm),
        Rm64(rm)          => Inst::opcode1(0xd1).reg(5).rm(rm).rex_w(),
        Rm32Cl(rm)        => Inst::opcode1(0xd3).reg(5).rm(rm),
        Rm64Cl(rm)        => Inst::opcode1(0xd3).reg(5).rm(rm).rex_w(),
        Rm32Imm8(rm, imm) => Inst::opcode1(0xc1).reg(5).rm(rm).imm8(imm),
        Rm64Imm8(rm, imm) => Inst::opcode1(0xc1).reg(5).rm(rm).imm8(imm).rex_w(),
    },

    Sbb(0x18, 3),

    Scas {
        B => Inst::opcode1(0xae),
        W => Inst::opcode1(0xaf).osz(),
        D => Inst::opcode1(0xaf),
        Q => Inst::opcode1(0xaf).rex_w(),
    },

    Set {
        CcRm8l(cc, rm) => Inst::opcode2(0x90).cc(cc).rm(rm),
        CcRm8(cc, rm)  => Inst::opcode2(0x90).cc(cc).rm(rm),
    },

    Sfence { Inst::opcode3(0xae, 0xf8) },

    Sgdt {
        M(m) => Inst::opcode2(0x01).reg(0).mem(m),
    },

    Shld {
        Rm16R16Imm8(rm, r, imm) => Inst::opcode2(0xa4).rm(rm).reg(r).imm8(imm).osz(),
        Rm16R16Cl(rm, r)        => Inst::opcode2(0xa5).rm(rm).reg(r).osz(),
        Rm32R32Imm8(rm, r, imm) => Inst::opcode2(0xa4).rm(rm).reg(r).imm8(imm),
        Rm64R64Imm8(rm, r, imm) => Inst::opcode2(0xa4).rm(rm).reg(r).imm8(imm).rex_w(),
        Rm32R32Cl(rm, r)        => Inst::opcode2(0xa5).rm(rm).reg(r),
        Rm64R64Cl(rm, r)        => Inst::opcode2(0xa5).rm(rm).reg(r).rex_w(),
    },

    Shrd {
        Rm16R16Imm8(rm, r, imm) => Inst::opcode2(0xac).rm(rm).reg(r).imm8(imm).osz(),
        Rm16R16Cl(rm, r)        => Inst::opcode2(0xad).rm(rm).reg(r).osz(),
        Rm32R32Imm8(rm, r, imm) => Inst::opcode2(0xac).rm(rm).reg(r).imm8(imm),
        Rm64R64Imm8(rm, r, imm) => Inst::opcode2(0xac).rm(rm).reg(r).imm8(imm).rex_w(),
        Rm32R32Cl(rm, r)        => Inst::opcode2(0xad).rm(rm).reg(r),
        Rm64R64Cl(rm, r)        => Inst::opcode2(0xad).rm(rm).reg(r).rex_w(),
    },

    Sidt {
        M(m) => Inst::opcode2(0x01).reg(1).mem(m),
    },

    Sldt {
        Rm16(rm) => Inst::opcode2(0x00).reg(0).rm(rm),
        Rm64(rm) => Inst::opcode2(0x00).reg(0).rm(rm).rex_w(),
    },

    Smsw {
        Rm16(rm) => Inst::opcode2(0x01).reg(4).rm(rm).osz(),
        Rm32(rm) => Inst::opcode2(0x01).reg(4).rm(rm),
        Rm64(rm) => Inst::opcode2(0x01).reg(4).rm(rm).rex_w(),
    },

    Stac { Inst::opcode3(0x01, 0xCB) },

    Stc { Inst::opcode1(0xf9) },

    Std { Inst::opcode1(0xfd) },

    //Sti { Inst::opcode1(0xfb) },

    Stos {
        B => Inst::opcode1(0xaa),
        W => Inst::opcode1(0xab).osz(),
        D => Inst::opcode1(0xab),
        Q => Inst::opcode1(0xab).rex_w(),
    },

    Str {
        Rm16(rm) => Inst::opcode2(0x00).reg(1).rm(rm),
    },

    Sub(0x28, 5),
}
