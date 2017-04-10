use code::{Encode, Instruction, Opcode};
use mnemonic::instruction::*;

fn opcode1(a: u8) -> Instruction {
    Instruction::new(Opcode::B1([a]))
}

fn opcode2(b: u8) -> Instruction {
    Instruction::new(Opcode::B2([0x0f, b]))
}

fn opcode3(b: u8, c: u8) -> Instruction {
    Instruction::new(Opcode::B3([0x0f, b, c]))
}

fn fopcode(b: u8) -> Instruction {
    Instruction::new(Opcode::B2([0xd9, b]))
}

macro_rules! impl_encode {
    ($($ty:ident $tt:tt,)+) => {
        $(impl_encode!($ty $tt);)+
    };

    ($ty:ident { $($pat:pat => $enc:expr,)+ }) => {
        impl Encode for $ty {
            fn encode(&self) -> Instruction {
                use self::$ty::*;
                match *self {
                    $($pat => $enc,)+
                }
            }
        }
    };

    ($ty:ident($opcode:expr, $reg:expr)) => {
        impl_encode! {
            $ty {
                Rm8lR8l(rm, r) => opcode1($opcode).rm8l(rm).reg(r),
                Rm8R8(rm, r)   => opcode1($opcode).rm8(rm).reg(r),
                Rm16R16(rm, r) => opcode1($opcode + 1).rm16(rm).reg(r).osz(),
                Rm32R32(rm, r) => opcode1($opcode + 1).rm32(rm).reg(r),
                Rm64R64(rm, r) => opcode1($opcode + 1).rm64(rm).reg(r).rex_w(),

                R8lRm8l(r, rm) => opcode1($opcode + 2).reg(r).rm8l(rm),
                R8Rm8(r, rm)   => opcode1($opcode + 2).reg(r).rm8(rm),
                R16Rm16(r, rm) => opcode1($opcode + 3).reg(r).rm16(rm).osz(),
                R32Rm32(r, rm) => opcode1($opcode + 3).reg(r).rm32(rm),
                R64Rm64(r, rm) => opcode1($opcode + 3).reg(r).rm64(rm).rex_w(),

                AlImm8(imm)   => opcode1($opcode + 4).imm8(imm),
                AxImm16(imm)  => opcode1($opcode + 5).imm16(imm).osz(),
                EaxImm32(imm) => opcode1($opcode + 5).imm32(imm),
                RaxImm32(imm) => opcode1($opcode + 5).imm32(imm).rex_w(),

                Rm8lImm8(rm, imm)  => opcode1(0x80).reg($reg).rm8l(rm).imm8(imm),
                Rm8Imm8(rm, imm)   => opcode1(0x80).reg($reg).rm8(rm).imm8(imm),
                Rm16Imm16(rm, imm) => opcode1(0x81).reg($reg).rm16(rm).imm16(imm).osz(),
                Rm32Imm32(rm, imm) => opcode1(0x81).reg($reg).rm32(rm).imm32(imm),
                Rm64Imm32(rm, imm) => opcode1(0x81).reg($reg).rm64(rm).imm32(imm).rex_w(),

                Rm16Imm8(rm, imm) => opcode1(0x83).reg($reg).rm16(rm).imm8(imm).osz(),
                Rm32Imm8(rm, imm) => opcode1(0x83).reg($reg).rm32(rm).imm8(imm),
                Rm64Imm8(rm, imm) => opcode1(0x83).reg($reg).rm64(rm).imm8(imm).rex_w(),
            }
        }
    };

    ($ty:ident $enc:expr) => {
        impl Encode for $ty {
            fn encode(&self) -> Instruction {
                $enc
            }
        }
    };
}

impl_encode! {
    Adc(0x10, 2),

    Adcx {
        R32Rm32(r, rm) => opcode3(0x38, 0xf6).osz().reg(r).rm32(rm),
        R64Rm64(r, rm) => opcode3(0x38, 0xf6).osz().reg(r).rm64(rm).rex_w(),
    },

    Add(0x00, 0),

    Adox {
        R32Rm32(r, rm) => opcode3(0x38, 0xf6).rep().reg(r).rm32(rm),
        R64Rm64(r, rm) => opcode3(0x38, 0xf6).rep().reg(r).rm64(rm).rex_w(),
    },

    And(0x20, 4),

    Bsf {
        R16Rm16(r, rm) => opcode2(0xbc).reg(r).rm16(rm).osz(),
        R32Rm32(r, rm) => opcode2(0xbc).reg(r).rm32(rm),
        R64Rm64(r, rm) => opcode2(0xbc).reg(r).rm64(rm).rex_w(),
    },

    Bsr {
        R16Rm16(r, rm) => opcode2(0xbd).reg(r).rm16(rm).osz(),
        R32Rm32(r, rm) => opcode2(0xbd).reg(r).rm32(rm),
        R64Rm64(r, rm) => opcode2(0xbd).reg(r).rm64(rm).rex_w(),
    },

    Bswap {
        R32(r) => opcode2(0xc8).plus(r),
        R64(r) => opcode2(0xc8).plus(r).rex_w(),
    },

    Bt {
        Rm16R16(rm, r) => opcode2(0xa3).rm16(rm).reg(r).osz(),
        Rm32R32(rm, r) => opcode2(0xa3).rm32(rm).reg(r),
        Rm64R64(rm, r) => opcode2(0xa3).rm64(rm).reg(r).rex_w(),

        Rm16Imm8(rm, imm) => opcode2(0xba).reg(4).rm16(rm).imm8(imm).osz(),
        Rm32Imm8(rm, imm) => opcode2(0xba).reg(4).rm32(rm).imm8(imm),
        Rm64Imm8(rm, imm) => opcode2(0xba).reg(4).rm64(rm).imm8(imm).rex_w(),
    },

    Btc {
        Rm16R16(rm, r) => opcode2(0xbb).rm16(rm).reg(r).osz(),
        Rm32R32(rm, r) => opcode2(0xbb).rm32(rm).reg(r),
        Rm64R64(rm, r) => opcode2(0xbb).rm64(rm).reg(r).rex_w(),

        Rm16Imm8(rm, imm) => opcode2(0xba).reg(7).rm16(rm).imm8(imm).osz(),
        Rm32Imm8(rm, imm) => opcode2(0xba).reg(7).rm32(rm).imm8(imm),
        Rm64Imm8(rm, imm) => opcode2(0xba).reg(7).rm64(rm).imm8(imm).rex_w(),
    },

    Btr {
        Rm16R16(rm, r) => opcode2(0xb3).rm16(rm).reg(r).osz(),
        Rm32R32(rm, r) => opcode2(0xb3).rm32(rm).reg(r),
        Rm64R64(rm, r) => opcode2(0xb3).rm64(rm).reg(r).rex_w(),

        Rm16Imm8(rm, imm) => opcode2(0xba).reg(6).rm16(rm).imm8(imm).osz(),
        Rm32Imm8(rm, imm) => opcode2(0xba).reg(6).rm32(rm).imm8(imm),
        Rm64Imm8(rm, imm) => opcode2(0xba).reg(6).rm64(rm).imm8(imm).rex_w(),
    },

    Bts {
        Rm16R16(rm, r) => opcode2(0xab).rm16(rm).reg(r).osz(),
        Rm32R32(rm, r) => opcode2(0xab).rm32(rm).reg(r),
        Rm64R64(rm, r) => opcode2(0xab).rm64(rm).reg(r).rex_w(),

        Rm16Imm8(rm, imm) => opcode2(0xba).reg(5).rm16(rm).imm8(imm).osz(),
        Rm32Imm8(rm, imm) => opcode2(0xba).reg(5).rm32(rm).imm8(imm),
        Rm64Imm8(rm, imm) => opcode2(0xba).reg(5).rm64(rm).imm8(imm).rex_w(),
    },

    Call {
        Rel32(rel) => opcode1(0xe8).disp32(rel.0),
        Rm64(rm)   => opcode1(0xff).reg(2).rm64(rm),
        M16x16(m)  => opcode1(0xff).reg(3).mem(m).osz(),
        M16x32(m)  => opcode1(0xff).reg(3).mem(m),
        M16x64(m)  => opcode1(0xff).reg(3).mem(m).rex_w(),
    },

    Cbw  { opcode1(0x98).osz() },
    Cwde { opcode1(0x98) },
    Cdqe { opcode1(0x98).rex_w() },

    Clac { opcode3(0x01, 0xca) },

    Clc { opcode1(0xf8) },

    Cld { opcode1(0xfc) },

    Clflush {
        M8(m) => opcode2(0xae).reg(7).mem(m),
    },

    Clflushopt {
        M8(m) => opcode2(0xae).reg(7).mem(m).osz(),
    },

    Cli { opcode1(0xfa) },

    Clts { opcode1(0x06) },

    Cmc { opcode1(0xf5) },

    Cmov {
        CcR16Rm16(cc, r, rm) => opcode2(0x40).cc(cc).reg(r).rm16(rm).osz(),
        CcR32Rm32(cc, r, rm) => opcode2(0x40).cc(cc).reg(r).rm32(rm),
        CcR64Rm64(cc, r, rm) => opcode2(0x40).cc(cc).reg(r).rm64(rm).rex_w(),
    },

    Cmp(0x38, 7),

    Cmps {
        B => opcode1(0xa6),
        W => opcode1(0xa7).osz(),
        D => opcode1(0xa7),
        Q => opcode1(0xa7).rex_w(),
    },

    Cmpxchg {
        Rm8lR8l(rm, r) => opcode2(0xb0).rm8l(rm).reg(r),
        Rm8R8(rm, r)   => opcode2(0xb0).rm8(rm).reg(r),
        Rm16R16(rm, r) => opcode2(0xb1).rm16(rm).reg(r).osz(),
        Rm32R32(rm, r) => opcode2(0xb1).rm32(rm).reg(r),
        Rm64R64(rm, r) => opcode2(0xb1).rm64(rm).reg(r).rex_w(),
    },

    Cmpxchg8b {
        M64(m) => opcode2(0xc7).reg(1).mem(m),
    },

    Cmpxchg16b {
        M128(m) => opcode2(0xc7).reg(1).mem(m).rex_w(),
    },

    Cpuid { opcode2(0xa2) },

    Crc32 {
        R32lRm8l(r, rm) => opcode3(0x38, 0xf0).repne().reg(r).rm8l(rm),
        R32Rm8(r, rm)   => opcode3(0x38, 0xf0).repne().reg(r).rm8(rm),
        R32Rm16(r, rm)  => opcode3(0x38, 0xf1).repne().reg(r).rm16(rm).osz(),
        R32Rm32(r, rm)  => opcode3(0x38, 0xf1).repne().reg(r).rm32(rm),
        R64Rm8(r, rm)   => opcode3(0x38, 0xf0).repne().reg(r).rm8(rm).rex_w(),
        R64Rm64(r, rm)  => opcode3(0x38, 0xf1).repne().reg(r).rm64(rm).rex_w(),
    },

    Cwd { opcode1(0x99).osz() },
    Cdq { opcode1(0x99) },
    Cqo { opcode1(0x99).rex_w() },

    Dec {
        Rm8l(rm) => opcode1(0xfe).reg(1).rm8l(rm),
        Rm8(rm)  => opcode1(0xfe).reg(1).rm8(rm),
        Rm16(rm) => opcode1(0xff).reg(1).rm16(rm).osz(),
        Rm32(rm) => opcode1(0xff).reg(1).rm32(rm),
        Rm64(rm) => opcode1(0xff).reg(1).rm64(rm).rex_w(),
    },

    Div {
        Rm8l(rm) => opcode1(0xf6).reg(6).rm8l(rm),
        Rm8(rm)  => opcode1(0xf6).reg(6).rm8(rm),
        Rm16(rm) => opcode1(0xf7).reg(6).rm16(rm).osz(),
        Rm32(rm) => opcode1(0xf7).reg(6).rm32(rm),
        Rm64(rm) => opcode1(0xf7).reg(6).rm64(rm).rex_w(),
    },

    F2xm1 { fopcode(0xf0) },

    Fabs { fopcode(0xe1) },

    Fadd {
        M32fp(m)  => opcode1(0xd8).reg(0).mem(m),
        M64fp(m)  => opcode1(0xdc).reg(0).mem(m),
        St0Sti(i) => opcode1(0xd8).reg(0).rm_reg(i),
        StiSt0(i) => opcode1(0xdc).reg(0).rm_reg(i),
    },
    Faddp {
        StiSt0(i) => opcode1(0xde).reg(0).rm_reg(i),
    },
    Fiadd {
        M32int(m) => opcode1(0xda).reg(0).mem(m),
        M16int(m) => opcode1(0xde).reg(0).mem(m),
    },

    Fbld {
        M80dec(m) => opcode1(0xdf).reg(4).mem(m),
    },

    Fbstp {
        M80bcd(m) => opcode1(0xdf).reg(6).mem(m),
    },

    Fchs { fopcode(0xe0) },

    Fclex { Instruction::new(Opcode::B3([0x9b, 0xdb, 0xe2])) },
    Fnclex { Instruction::new(Opcode::B2([0xdb, 0xe2])) },

    Fcmov {
        BSt0Sti(i)   => opcode1(0xda).reg(0).rm_reg(i),
        ESt0Sti(i)   => opcode1(0xda).reg(1).rm_reg(i),
        BeSt0Sti(i)  => opcode1(0xda).reg(2).rm_reg(i),
        USt0Sti(i)   => opcode1(0xda).reg(3).rm_reg(i),
        NbSt0Sti(i)  => opcode1(0xdb).reg(0).rm_reg(i),
        NeSt0Sti(i)  => opcode1(0xdb).reg(1).rm_reg(i),
        NbeSt0Sti(i) => opcode1(0xdb).reg(2).rm_reg(i),
        NuSt0Sti(i)  => opcode1(0xdb).reg(3).rm_reg(i),
    },

    Fcom {
        M32fp(m) => opcode1(0xd8).reg(2).mem(m),
        M64fp(m) => opcode1(0xdc).reg(2).mem(m),
        Sti(i)   => opcode1(0xd8).reg(2).rm_reg(i),
    },
    Fcomp {
        M32fp(m) => opcode1(0xd8).reg(3).mem(m),
        M64fp(m) => opcode1(0xdc).reg(3).mem(m),
        Sti(i)   => opcode1(0xd8).reg(3).rm_reg(i),
    },
    Fcompp { Instruction::new(Opcode::B2([0xde, 0xd9])) },

    Fcomi {
        St0Sti(i) => opcode1(0xdb).reg(6).rm_reg(i),
    },
    Fcomip {
        St0Sti(i) => opcode1(0xdf).reg(6).rm_reg(i),
    },
    Fucomi {
        St0Sti(i) => opcode1(0xdb).reg(5).rm_reg(i),
    },
    Fucomip {
        St0Sti(i) => opcode1(0xdf).reg(5).rm_reg(i),
    },

    Fcos { fopcode(0xff) },

    Fdecstp { fopcode(0xf6) },

    Fdiv {
        M32fp(m)  => opcode1(0xd8).reg(6).mem(m),
        M64fp(m)  => opcode1(0xdc).reg(6).mem(m),
        St0Sti(i) => opcode1(0xd8).reg(6).rm_reg(i),
        StiSt0(i) => opcode1(0xdc).reg(7).rm_reg(i),
    },
    Fdivp {
        StiSt0(i) => opcode1(0xde).reg(7).rm_reg(i),
    },
    Fidiv {
        M32int(m) => opcode1(0xda).reg(6).mem(m),
        M16int(m) => opcode1(0xde).reg(6).mem(m),
    },

    Fdivr {
        M32fp(m)  => opcode1(0xd8).reg(7).mem(m),
        M64fp(m)  => opcode1(0xdc).reg(7).mem(m),
        St0Sti(i) => opcode1(0xd8).reg(7).rm_reg(i),
        StiSt0(i) => opcode1(0xdc).reg(6).rm_reg(i),
    },
    Fdivrp {
        StiSt0(i) => opcode1(0xde).reg(6).rm_reg(i),
    },
    Fidivr {
        M32int(m) => opcode1(0xda).reg(7).mem(m),
        M16int(m) => opcode1(0xde).reg(7).mem(m),
    },

    Ffree {
        Sti(i) => opcode1(0xdd).reg(0).rm_reg(i),
    },

    Ficom {
        M16int(m) => opcode1(0xde).reg(2).mem(m),
        M32int(m) => opcode1(0xda).reg(2).mem(m),
    },
    Ficomp {
        M16int(m) => opcode1(0xde).reg(3).mem(m),
        M32int(m) => opcode1(0xda).reg(3).mem(m),
    },

    Fild {
        M16int(m) => opcode1(0xdf).reg(0).mem(m),
        M32int(m) => opcode1(0xdb).reg(0).mem(m),
        M64int(m) => opcode1(0xdf).reg(5).mem(m),
    },

    Fincstp { fopcode(0xf7) },

    Finit { Instruction::new(Opcode::B3([0x9b, 0xdb, 0xe3])) },
    Fninit { Instruction::new(Opcode::B2([0xdb, 0xe3])) },

    Fist {
        M16int(m) => opcode1(0xdf).reg(2).mem(m),
        M32int(m) => opcode1(0xdb).reg(2).mem(m),
    },
    Fistp {
        M16int(m) => opcode1(0xdf).reg(3).mem(m),
        M32int(m) => opcode1(0xdb).reg(3).mem(m),
        M64int(m) => opcode1(0xdf).reg(7).mem(m),
    },

    Fisttp {
        M16int(m) => opcode1(0xdf).reg(1).mem(m),
        M32int(m) => opcode1(0xdb).reg(1).mem(m),
        M64int(m) => opcode1(0xdd).reg(1).mem(m),
    },

    Fld {
        M32fp(m) => opcode1(0xd9).reg(0).mem(m),
        M64fp(m) => opcode1(0xdd).reg(0).mem(m),
        M80fp(m) => opcode1(0xdb).reg(5).mem(m),
        Sti(i)   => opcode1(0xd9).reg(0).rm_reg(i),
    },

    Fld1   { fopcode(0xe8) },
    Fldl2t { fopcode(0xe9) },
    Fldl2e { fopcode(0xea) },
    Fldpi  { fopcode(0xeb) },
    Fldlg2 { fopcode(0xec) },
    Fldln2 { fopcode(0xed) },
    Fldz   { fopcode(0xee) },

    Fldcw {
        M2byte(m) => opcode1(0xd9).reg(5).mem(m),
    },

    Fldenv {
        M28byte(m) => opcode1(0xd9).reg(4).mem(m),
    },

    Fmul {
        M32fp(m)  => opcode1(0xd8).reg(1).mem(m),
        M64fp(m)  => opcode1(0xdc).reg(1).mem(m),
        St0Sti(i) => opcode1(0xd8).reg(1).rm_reg(i),
        StiSt0(i) => opcode1(0xdc).reg(1).rm_reg(i),
    },
    Fmulp {
        StiSt0(i) => opcode1(0xde).reg(1).rm_reg(i),
    },
    Fimul {
        M32int(m) => opcode1(0xda).reg(1).mem(m),
        M16int(m) => opcode1(0xde).reg(1).mem(m),
    },

    Fnop { fopcode(0xd0) },

    Fpatan { fopcode(0xf3) },

    Fprem { fopcode(0xf8) },

    Fprem1 { fopcode(0xf5) },

    Fptan { fopcode(0xf2) },

    Frndint { fopcode(0xfc) },

    Frstor {
        M108byte(m) => opcode1(0xdd).reg(4).mem(m),
    },

    Fsave {
        M108byte(m) => Instruction::new(Opcode::B2([0x9b, 0xdd])).reg(6).mem(m),
    },
    Fnsave {
        M108byte(m) => opcode1(0xdd).reg(6).mem(m),
    },

    Fscale { fopcode(0xfd) },

    Fsin { fopcode(0xfe) },

    Fsincos { fopcode(0xfb) },

    Fsqrt { fopcode(0xfa) },

    Fst {
        M32fp(m) => opcode1(0xd9).reg(2).mem(m),
        M64fp(m) => opcode1(0xdd).reg(2).mem(m),
        Sti(i)   => opcode1(0xdd).reg(2).rm_reg(i),
    },
    Fstp {
        M32fp(m) => opcode1(0xd9).reg(3).mem(m),
        M64fp(m) => opcode1(0xdd).reg(3).mem(m),
        M80fp(m) => opcode1(0xdb).reg(7).mem(m),
        Sti(i)   => opcode1(0xdd).reg(3).rm_reg(i),
    },

    Fstcw {
        M2byte(m) => Instruction::new(Opcode::B2([0x9b, 0xd9])).reg(7).mem(m),
    },
    Fnstcw {
        M2byte(m) => opcode1(0xd9).reg(7).mem(m),
    },

    Fstenv {
        M28byte(m) => Instruction::new(Opcode::B2([0x9b, 0xd9])).reg(6).mem(m),
    },
    Fnstenv {
        M28byte(m) => opcode1(0xd9).reg(6).mem(m),
    },

    Fstsw {
        M2byte(m) => Instruction::new(Opcode::B2([0x9b, 0xdd])).reg(7).mem(m),
        Ax        => Instruction::new(Opcode::B3([0x9b, 0xdf, 0xe0])),
    },
    Fnstsw {
        M2byte(m) => opcode1(0xdd).reg(7).mem(m),
        Ax        => Instruction::new(Opcode::B2([0xdf, 0xe0])),
    },

    Fsub {
        M32fp(m)  => opcode1(0xd8).reg(4).mem(m),
        M64fp(m)  => opcode1(0xdc).reg(4).mem(m),
        St0Sti(i) => opcode1(0xd8).reg(4).rm_reg(i),
        StiSt0(i) => opcode1(0xdc).reg(5).rm_reg(i),
    },
    Fsubp {
        StiSt0(i) => opcode1(0xde).reg(5).rm_reg(i),
    },
    Fisub {
        M32int(m) => opcode1(0xda).reg(4).mem(m),
        M16int(m) => opcode1(0xde).reg(4).mem(m),
    },

    Fsubr {
        M32fp(m)  => opcode1(0xd8).reg(5).mem(m),
        M64fp(m)  => opcode1(0xdc).reg(5).mem(m),
        St0Sti(i) => opcode1(0xd8).reg(5).rm_reg(i),
        StiSt0(i) => opcode1(0xdc).reg(4).rm_reg(i),
    },
    Fsubrp {
        StiSt0(i) => opcode1(0xde).reg(4).rm_reg(i),
    },
    Fisubr {
        M32int(m) => opcode1(0xda).reg(5).mem(m),
        M16int(m) => opcode1(0xde).reg(5).mem(m),
    },

    Ftst { fopcode(0xe4) },

    Fucom {
        Sti(i) => opcode1(0xdd).reg(4).rm_reg(i),
    },
    Fucomp {
        Sti(i) => opcode1(0xdd).reg(5).rm_reg(i),
    },
    Fucompp { opcode1(0xda).reg(5).rm_reg(1) },

    Fxam { fopcode(0xe5) },

    Fxch {
        Sti(i) => opcode1(0xd9).reg(1).rm_reg(i),
    },

    Fxrstor {
        M512byte(m) => opcode2(0xae).reg(1).mem(m),
    },
    Fxrstor64 {
        M512byte(m) => opcode2(0xae).reg(1).mem(m).rex_w(),
    },

    Fxsave {
        M512byte(m) => opcode2(0xae).reg(0).mem(m),
    },
    Fxsave64 {
        M512byte(m) => opcode2(0xae).reg(0).mem(m),
    },

    Fxtract { fopcode(0xf4) },

    Fyl2x { fopcode(0xf1) },

    Fyl2xp1 { fopcode(0xf9) },

    Hlt { opcode1(0xf4) },

    Idiv {
        Rm8l(rm) => opcode1(0xf6).reg(7).rm8l(rm),
        Rm8(rm)  => opcode1(0xf6).reg(7).rm8(rm),
        Rm16(rm) => opcode1(0xf7).reg(7).rm16(rm).osz(),
        Rm32(rm) => opcode1(0xf7).reg(7).rm32(rm),
        Rm64(rm) => opcode1(0xf7).reg(7).rm64(rm).rex_w(),
    },

    Imul {
        Rm8l(rm) => opcode1(0xf6).reg(5).rm8l(rm),
        Rm8(rm)  => opcode1(0xf6).reg(5).rm8(rm),
        Rm16(rm) => opcode1(0xf7).reg(5).rm16(rm).osz(),
        Rm32(rm) => opcode1(0xf7).reg(5).rm32(rm),
        Rm64(rm) => opcode1(0xf7).reg(5).rm64(rm).rex_w(),

        R16Rm16(r, rm) => opcode2(0xaf).reg(r).rm16(rm).osz(),
        R32Rm32(r, rm) => opcode2(0xaf).reg(r).rm32(rm),
        R64Rm64(r, rm) => opcode2(0xaf).reg(r).rm64(rm).rex_w(),

        R16Rm16Imm8(r, rm, imm) => opcode1(0x6b).reg(r).rm16(rm).imm8(imm).osz(),
        R32Rm32Imm8(r, rm, imm) => opcode1(0x6b).reg(r).rm32(rm).imm8(imm),
        R64Rm64Imm8(r, rm, imm) => opcode1(0x6b).reg(r).rm64(rm).imm8(imm).rex_w(),

        R16Rm16Imm16(r, rm, imm) => opcode1(0x69).reg(r).rm16(rm).imm16(imm).osz(),
        R32Rm32Imm32(r, rm, imm) => opcode1(0x69).reg(r).rm32(rm).imm32(imm),
        R64Rm64Imm32(r, rm, imm) => opcode1(0x69).reg(r).rm64(rm).imm32(imm).rex_w(),
    },

    In {
        AlImm8(imm)  => opcode1(0xe4).imm8(imm),
        AxImm8(imm)  => opcode1(0xe5).imm8(imm).osz(),
        EaxImm8(imm) => opcode1(0xe5).imm8(imm),

        AlDx  => opcode1(0xec),
        AxDx  => opcode1(0xed).osz(),
        EaxDx => opcode1(0xed),
    },

    Inc {
        Rm8l(rm) => opcode1(0xfe).reg(0).rm8l(rm),
        Rm8(rm)  => opcode1(0xfe).reg(0).rm8(rm),
        Rm16(rm) => opcode1(0xff).reg(0).rm16(rm).osz(),
        Rm32(rm) => opcode1(0xff).reg(0).rm32(rm),
        Rm64(rm) => opcode1(0xff).reg(0).rm64(rm).rex_w(),
    },

    Ins {
        B => opcode1(0x6c),
        W => opcode1(0x6d).osz(),
        D => opcode1(0x6d),
    },

    Int3 { opcode1(0xcc) },
    Int {
        Imm8(imm) => { opcode1(0xcd).imm8(imm) },
    },
    Into { opcode1(0xce) },

    Invd { opcode2(0x08) },

    Invlpg {
        M(m) => opcode2(0x01).reg(7).mem(m),
    },

    Invpcid {
        R64M128(r, m) => opcode3(0x38, 0x82).osz().reg(r).mem(m),
    },

    Iret {
        D => opcode1(0xcf),
        Q => opcode1(0xcf).rex_w(),
    },

    J {
        CcRel8(cc, rel)  => opcode1(0x70).cc(cc).disp8(rel.0),
        RcxzRel8(rel)    => opcode1(0xe3).disp8(rel.0),
        CcRel32(cc, rel) => opcode2(0x80).cc(cc).disp32(rel.0),
    },

    Jmp {
        Rel8(rel)  => opcode1(0xeb).disp8(rel.0),
        Rel32(rel) => opcode1(0xe9).disp32(rel.0),
        Rm64(rm)   => opcode1(0xff).reg(4).rm64(rm),
        M16x16(m)  => opcode1(0xff).reg(5).mem(m).osz(),
        M16x32(m)  => opcode1(0xff).reg(5).mem(m),
        M16x64(m)  => opcode1(0xff).reg(5).mem(m).rex_w(),
    },

    Lss {
        R16M16x16(r, m) => opcode2(0xb2).reg(r).mem(m).osz(),
        R32M16x32(r, m) => opcode2(0xb2).reg(r).mem(m),
        R64M16x64(r, m) => opcode2(0xb2).reg(r).mem(m).rex_w(),
    },
    Lfs {
        R16M16x16(r, m) => opcode2(0xb4).reg(r).mem(m).osz(),
        R32M16x32(r, m) => opcode2(0xb4).reg(r).mem(m),
        R64M16x64(r, m) => opcode2(0xb4).reg(r).mem(m).rex_w(),
    },
    Lgs {
        R16M16x16(r, m) => opcode2(0xb5).reg(r).mem(m).osz(),
        R32M16x32(r, m) => opcode2(0xb5).reg(r).mem(m),
        R64M16x64(r, m) => opcode2(0xb5).reg(r).mem(m).rex_w(),
    },

    Lea {
        R16M(r, m) => opcode1(0x8d).reg(r).mem(m).osz(),
        R32M(r, m) => opcode1(0x8d).reg(r).mem(m),
        R64M(r, m) => opcode1(0x8d).reg(r).mem(m).rex_w(),
    },

    Leave { opcode1(0xc9) },

    Lfence { opcode3(0xae, 0xe8) },

    Lgdt {
        M16x64(m) => opcode2(0x01).reg(2).mem(m),
    },
    Lidt {
        M16x64(m) => opcode2(0x01).reg(3).mem(m),
    },

    Lldt {
        Rm16(rm) => opcode2(0x00).reg(2).rm16(rm),
    },

    Lmsw {
        Rm16(rm) => opcode2(0x01).reg(6).rm16(rm),
    },

    Lods {
        B => opcode1(0xac),
        W => opcode1(0xad).osz(),
        D => opcode1(0xad),
        Q => opcode1(0xad).rex_w(),
    },

    Loop {
        Rel8(rel)   => opcode1(0xe2).disp8(rel.0),
        ERel8(rel)  => opcode1(0xe1).disp8(rel.0),
        NeRel8(rel) => opcode1(0xe0).disp8(rel.0),
    },

    Ltr {
        Rm16(rm) => opcode2(0x00).reg(3).rm16(rm),
    },

    Lzcnt {
        R16Rm16(r, rm) => opcode2(0xbd).rep().reg(r).rm16(rm).osz(),
        R32Rm32(r, rm) => opcode2(0xbd).rep().reg(r).rm32(rm),
        R64Rm64(r, rm) => opcode2(0xbd).rep().reg(r).rm64(rm).rex_w(),
    },

    Mfence { opcode3(0xae, 0xf0) },

    Monitor { opcode3(0x01, 0xc8) },

    Mov {
        Rm8lR8l(rm, r) => opcode1(0x88).rm8l(rm).reg(r),
        Rm8R8(rm, r)   => opcode1(0x88).rm8(rm).reg(r),
        Rm16R16(rm, r) => opcode1(0x89).rm16(rm).reg(r).osz(),
        Rm32R32(rm, r) => opcode1(0x89).rm32(rm).reg(r),
        Rm64R64(rm, r) => opcode1(0x89).rm64(rm).reg(r).rex_w(),

        R8lRm8l(r, rm) => opcode1(0x8a).reg(r).rm8l(rm),
        R8Rm8(r, rm)   => opcode1(0x8a).reg(r).rm8(rm),
        R16Rm16(r, rm) => opcode1(0x8b).reg(r).rm16(rm).osz(),
        R32Rm32(r, rm) => opcode1(0x8b).reg(r).rm32(rm),
        R64Rm64(r, rm) => opcode1(0x8b).reg(r).rm64(rm).rex_w(),

        Rm16Sreg(rm, sreg) => opcode1(0x8c).rm16(rm).reg(sreg),
        Rm64Sreg(rm, sreg) => opcode1(0x8c).rm64(rm).reg(sreg).rex_w(),

        SregRm16(sreg, rm) => opcode1(0x8e).rm16(rm).reg(sreg),
        SregRm64(sreg, rm) => opcode1(0x8e).rm64(rm).reg(sreg).rex_w(),

        AlMoffs8(moffs)   => opcode1(0xa0).moffs(moffs),
        AxMoffs16(moffs)  => opcode1(0xa1).moffs(moffs).osz(),
        EaxMoffs32(moffs) => opcode1(0xa1).moffs(moffs),
        RaxMoffs64(moffs) => opcode1(0xa1).moffs(moffs).rex_w(),

        Moffs8Al(moffs)   => opcode1(0xa2).moffs(moffs),
        Moffs16Ax(moffs)  => opcode1(0xa3).moffs(moffs).osz(),
        Moffs32Eax(moffs) => opcode1(0xa3).moffs(moffs),
        Moffs64Rax(moffs) => opcode1(0xa3).moffs(moffs).rex_w(),

        R8lImm8(r, imm)  => opcode1(0xb0).plus(r).imm8(imm),
        R8Imm8(r, imm)   => opcode1(0xb0).plus(r).imm8(imm),
        R16Imm16(r, imm) => opcode1(0xb8).plus(r).imm16(imm).osz(),
        R32Imm32(r, imm) => opcode1(0xb8).plus(r).imm32(imm),
        R64Imm64(r, imm) => opcode1(0xb8).plus(r).imm64(imm).rex_w(),

        Rm8lImm8(rm, imm)  => opcode1(0xc6).reg(0).rm8l(rm).imm8(imm),
        Rm8Imm8(rm, imm)   => opcode1(0xc6).reg(0).rm8(rm).imm8(imm),
        Rm16Imm16(rm, imm) => opcode1(0xc7).reg(0).rm16(rm).imm16(imm).osz(),
        Rm32Imm32(rm, imm) => opcode1(0xc7).reg(0).rm32(rm).imm32(imm),
        Rm64Imm32(rm, imm) => opcode1(0xc7).reg(0).rm64(rm).imm32(imm).rex_w(),

        R64Cr(r, cr) => opcode2(0x20).rm_reg(r).reg(cr),
        CrR64(cr, r) => opcode2(0x22).reg(cr).rm_reg(r),

        R64Dr(r, dr) => opcode2(0x21).rm_reg(r).reg(dr),
        DrR64(dr, r) => opcode2(0x23).reg(dr).rm_reg(r),
    },
}
