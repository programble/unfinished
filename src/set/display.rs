use super::*;

use core::fmt::{Display, Formatter, Error};

macro_rules! impl_display {
    ($($ty:ident $tt:tt,)+) => {
        $(impl_display!($ty $tt);)+
    };

    ($ty:ident { $($pat:pat => $fmt:expr,)+ }) => {
        impl Display for $ty {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                use self::$ty::*;
                match *self {
                    $($pat => f.write_fmt($fmt),)+
                }
            }
        }
    };

    ($ty:ident($str:expr)) => {
        impl Display for $ty {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                f.write_str($str)
            }
        }
    };

    ($ty:ident($str:expr, 1, { $($var:ident),+ })) => {
        impl_display! {
            $ty {
                $($var(a) => format_args!(concat!($str, " {}"), a),)+
            }
        }
    };

    ($ty:ident($str:expr, 2, { $($var:ident),+ })) => {
        impl_display! {
            $ty {
                $($var(a, b) => format_args!(concat!($str, " {}, {}"), a, b),)+
            }
        }
    };

    ($ty:ident($str:expr, a)) => {
        impl_display! {
            $ty {
                AlImm8(imm)        => format_args!(concat!($str, " al, {}"), imm),
                AxImm16(imm)       => format_args!(concat!($str, " ax, {}"), imm),
                EaxImm32(imm)      => format_args!(concat!($str, " eax, {}"), imm),
                RaxImm32(imm)      => format_args!(concat!($str, " rax, {}"), imm),
                Rm8lImm8(rm, imm)  => format_args!(concat!($str, " {}, {}"), rm, imm),
                Rm8Imm8(rm, imm)   => format_args!(concat!($str, " {}, {}"), rm, imm),
                Rm16Imm16(rm, imm) => format_args!(concat!($str, " {}, {}"), rm, imm),
                Rm32Imm32(rm, imm) => format_args!(concat!($str, " {}, {}"), rm, imm),
                Rm64Imm32(rm, imm) => format_args!(concat!($str, " {}, {}"), rm, imm),
                Rm16Imm8(rm, imm)  => format_args!(concat!($str, " {}, {}"), rm, imm),
                Rm32Imm8(rm, imm)  => format_args!(concat!($str, " {}, {}"), rm, imm),
                Rm64Imm8(rm, imm)  => format_args!(concat!($str, " {}, {}"), rm, imm),
                Rm8lR8l(rm, r)     => format_args!(concat!($str, " {}, {}"), rm, r),
                Rm8R8(rm, r)       => format_args!(concat!($str, " {}, {}"), rm, r),
                Rm16R16(rm, r)     => format_args!(concat!($str, " {}, {}"), rm, r),
                Rm32R32(rm, r)     => format_args!(concat!($str, " {}, {}"), rm, r),
                Rm64R64(rm, r)     => format_args!(concat!($str, " {}, {}"), rm, r),
                R8lRm8l(r, rm)     => format_args!(concat!($str, " {}, {}"), r, rm),
                R8Rm8(r, rm)       => format_args!(concat!($str, " {}, {}"), r, rm),
                R16Rm16(r, rm)     => format_args!(concat!($str, " {}, {}"), r, rm),
                R32Rm32(r, rm)     => format_args!(concat!($str, " {}, {}"), r, rm),
                R64Rm64(r, rm)     => format_args!(concat!($str, " {}, {}"), r, rm),
            }
        }
    };

    ($ty:ident($str:expr, f)) => {
        impl_display! {
            $ty {
                M32fp(m)  => format_args!(concat!($str, " dword {}"), m),
                M64fp(m)  => format_args!(concat!($str, " qword {}"), m),
                St0Sti(i) => format_args!(concat!($str, " st0, {}"), i),
                StiSt0(i) => format_args!(concat!($str, " {}, st0"), i),
            }
        }
    };

    ($ty:ident($str:expr, fp)) => {
        impl_display! {
            $ty {
                StiSt0(i) => format_args!(concat!($str, " {}, st0"), i),
            }
        }
    };

    ($ty:ident($str:expr, fi)) => {
        impl_display! {
            $ty {
                M32int(m) => format_args!(concat!($str, " dword {}"), m),
                M16int(m) => format_args!(concat!($str, " word {}"), m),
            }
        }
    };
}

impl_display! {
    Adc("adc", a),
    Adcx("adcx", 2, { R32Rm32, R64Rm64 }),
    Add("add", a),
    Adox("adox", 2, { R32Rm32, R64Rm64 }),
    And("and", a),
    Bsf("bsf", 2, { R16Rm16, R32Rm32, R64Rm64 }),
    Bsr("bsj", 2, { R16Rm16, R32Rm32, R64Rm64 }),
    Bswap("bswap", 1, { R32, R64 }),
    Bt("bt", 2, { Rm16R16, Rm32R32, Rm64R64, Rm16Imm8, Rm32Imm8, Rm64Imm8 }),
    Btc("btc", 2, { Rm16R16, Rm32R32, Rm64R64, Rm16Imm8, Rm32Imm8, Rm64Imm8 }),
    Btr("btr", 2, { Rm16R16, Rm32R32, Rm64R64, Rm16Imm8, Rm32Imm8, Rm64Imm8 }),
    Bts("bts", 2, { Rm16R16, Rm32R32, Rm64R64, Rm16Imm8, Rm32Imm8, Rm64Imm8 }),

    Call {
        Rel32(rel) => format_args!("call rel {}", rel),
        Rm64(rm)   => format_args!("call near {}", rm),
        M16x16(m)  => format_args!("call far word {}", m),
        M16x32(m)  => format_args!("call far dword {}", m),
        M16x64(m)  => format_args!("call far qword {}", m),
    },

    Cbw("cbw"),
    Cwde("cwde"),
    Cdqe("cdqe"),
    Clac("clac"),
    Clc("clc"),
    Cld("cld"),
    Clflush("clflush", 1, { M8 }),
    Clflushopt("clflushopt", 1, { M8 }),
    Cli("cli"),
    Clts("clts"),
    Cmc("cmc"),

    Cmov {
        CcR16Rm16(cc, r, rm) => format_args!("cmov{} {}, {}", cc, r, rm),
        CcR32Rm32(cc, r, rm) => format_args!("cmov{} {}, {}", cc, r, rm),
        CcR64Rm64(cc, r, rm) => format_args!("cmov{} {}, {}", cc, r, rm),
    },

    Cmp("cmp", a),

    Cmps {
        B => format_args!("cmpsb"),
        W => format_args!("cmpsw"),
        D => format_args!("cmpsd"),
        Q => format_args!("cmpsq"),
    },

    Cmpxchg("cmpxchg", 2, { Rm8lR8l, Rm8R8, Rm16R16, Rm32R32, Rm64R64 }),
    Cmpxchg8b("cmpxchg8b", 1, { M64 }),
    Cmpxchg16b("cmpxchg16b", 1, { M128 }),
    Cpuid("cpuid"),
    Crc32("crc32", 2, { R32lRm8l, R32Rm8, R32Rm16, R32Rm32, R64Rm8, R64Rm64 }),
    Cwd("cwd"),
    Cdq("cdq"),
    Cqo("cqo"),
    Dec("dec", 1, { Rm8l, Rm8, Rm16, Rm32, Rm64 }),
    Div("div", 1, { Rm8l, Rm8, Rm16, Rm32, Rm64 }),
    F2xm1("f2xm1"),
    Fabs("fabs"),
    Fadd("fadd", f),
    Faddp("faddp", fp),
    Fiadd("fiadd", fi),
    Fbld("fbld", 1, { M80dec }),
    Fbstp("fbstp", 1, { M80bcd }),
    Fchs("fchs"),
    Fclex("fclex"),
    Fnclex("fnclex"),

    Fcmov {
        BSt0Sti(i)   => format_args!("fcmovb st0, {}", i),
        ESt0Sti(i)   => format_args!("fcmove st0, {}", i),
        BeSt0Sti(i)  => format_args!("fcmovbe st0, {}", i),
        USt0Sti(i)   => format_args!("fcmovu st0, {}", i),
        NbSt0Sti(i)  => format_args!("fcmovnb st0, {}", i),
        NeSt0Sti(i)  => format_args!("fcmovne st0, {}", i),
        NbeSt0Sti(i) => format_args!("fcmovnbe st0, {}", i),
        NuSt0Sti(i)  => format_args!("fcmovnu st0, {}", i),
    },

    Fcom {
        M32fp(m) => format_args!("fcom dword {}", m),
        M64fp(m) => format_args!("fcom qword {}", m),
        Sti(i)   => format_args!("fcom {}", i),
    },

    Fcomp {
        M32fp(m) => format_args!("fcomp dword {}", m),
        M64fp(m) => format_args!("fcomp qword {}", m),
        Sti(i)   => format_args!("fcomp {}", i),
    },

    Fcompp("fcompp"),
    Fcomi("fcomi st0,", 1, { St0Sti }),
    Fcomip("fcomip st0,", 1, { St0Sti }),
    Fucomi("fucomi st0,", 1, { St0Sti }),
    Fucomip("fucomip st0,", 1, { St0Sti }),
    Fcos("fcos"),
    Fdecstp("fdecstp"),

    Fdiv("fdiv", f),
    Fdivp("fdivp", fp),
    Fidiv("fidiv", fi),
    Fdivr("fdivr", f),
    Fdivrp("fdivrp", fp),
    Fidivr("fidivr", fi),
    Ffree("ffree", 1, { Sti }),

    Ficom {
        M16int(m) => format_args!("ficom word {}", m),
        M32int(m) => format_args!("ficom dword {}", m),
    },

    Ficomp {
        M16int(m) => format_args!("ficomp word {}", m),
        M32int(m) => format_args!("ficomp dword {}", m),
    },

    Fild {
        M16int(m) => format_args!("fild word {}", m),
        M32int(m) => format_args!("fild dword {}", m),
        M64int(m) => format_args!("fild qword {}", m),
    },

    Fincstp("fincstp"),
    Finit("finit"),
    Fninit("fninit"),

    Fist {
        M16int(m) => format_args!("fist word {}", m),
        M32int(m) => format_args!("fist dword {}", m),
    },

    Fistp {
        M16int(m) => format_args!("fistp word {}", m),
        M32int(m) => format_args!("fistp dword {}", m),
        M64int(m) => format_args!("fistp qword {}", m),
    },

    Fisttp {
        M16int(m) => format_args!("fisttp word {}", m),
        M32int(m) => format_args!("fisttp dword {}", m),
        M64int(m) => format_args!("fisttp qword {}", m),
    },

    Fld {
        M32fp(m) => format_args!("fld dword {}", m),
        M64fp(m) => format_args!("fld qword {}", m),
        M80fp(m) => format_args!("fld tword {}", m),
        Sti(i)   => format_args!("fld {}", i),
    },

    Fld1("fld1"),
    Fldl2t("fldl2t"),
    Fldl2e("fldl2e"),
    Fldpi("fldpi"),
    Fldlg2("fldlg2"),
    Fldln2("fldln2"),
    Fldz("fldz"),
    Fldcw("fldcw", 1, { M2byte }),
    Fldenv("fldenv", 1, { M28byte }),
    Fmul("fmul", f),
    Fmulp("fmulp", fp),
    Fimul("fimul", fi),
    Fnop("fnop"),
    Fpatan("fpatan"),
    Fprem("fprem"),
    Fprem1("fprem1"),
    Fptan("fptan"),
    Frndint("frndint"),
    Frstor("frstor", 1, { M108byte }),
    Fsave("fsave", 1, { M108byte }),
    Fnsave("fnsave", 1, { M108byte }),
    Fscale("fscale"),
    Fsin("fsin"),
    Fsincos("fsincos"),
    Fsqrt("fsqrt"),

    Fst {
        M32fp(m) => format_args!("fst dword {}", m),
        M64fp(m) => format_args!("fst qword {}", m),
        Sti(i)   => format_args!("fst {}", i),
    },

    Fstp {
        M32fp(m) => format_args!("fstp dword {}", m),
        M64fp(m) => format_args!("fstp qword {}", m),
        M80fp(m) => format_args!("fstp tword {}", m),
        Sti(i)   => format_args!("fstp {}", i),
    },

    Fstcw("fstcw", 1, { M2byte }),
    Fnstcw("fnstcw", 1, { M2byte }),
    Fstenv("fstenv", 1, { M28byte }),
    Fnstenv("fnstenv", 1, { M28byte }),

    Fstsw {
        M2byte(m) => format_args!("fstsw {}", m),
        Ax        => format_args!("fstsw ax"),
    },

    Fnstsw {
        M2byte(m) => format_args!("fnstsw {}", m),
        Ax        => format_args!("fnstsw ax"),
    },

    Fsub("fsub", f),
    Fsubp("fsubp", fp),
    Fisub("fisub", fi),
    Fsubr("fsubr", f),
    Fsubrp("fsubrp", fp),
    Fisubr("fisubr", fi),
    Ftst("ftst"),
    Fucom("fucom", 1, { Sti }),
    Fucomp("fucomp", 1, { Sti }),
    Fucompp("fucompp"),
    Fxam("fxam"),
    Fxch("fxch", 1, { Sti }),
    Fxrstor("fxrstor", 1, { M512byte }),
    Fxrstor64("fxrstor64", 1, { M512byte }),
    Fxsave("fxsave", 1, { M512byte }),
    Fxsave64("fxsave64", 1, { M512byte }),
    Fxtract("fxtract"),
    Fyl2x("fyl2x"),
    Fyl2xp1("fyl2xp1"),
    Hlt("hlt"),
    Idiv("idiv", 1, { Rm8l, Rm8, Rm16, Rm32, Rm64 }),

    Imul {
        Rm8l(rm) => format_args!("imul {}", rm),
        Rm8(rm)  => format_args!("imul {}", rm),
        Rm16(rm) => format_args!("imul {}", rm),
        Rm32(rm) => format_args!("imul {}", rm),
        Rm64(rm) => format_args!("imul {}", rm),

        R16Rm16(r, rm) => format_args!("imul {}, {}", r, rm),
        R32Rm32(r, rm) => format_args!("imul {}, {}", r, rm),
        R64Rm64(r, rm) => format_args!("imul {}, {}", r, rm),

        R16Rm16Imm8(r, rm, imm) => format_args!("imul {}, {}, {}", r, rm, imm),
        R32Rm32Imm8(r, rm, imm) => format_args!("imul {}, {}, {}", r, rm, imm),
        R64Rm64Imm8(r, rm, imm) => format_args!("imul {}, {}, {}", r, rm, imm),

        R16Rm16Imm16(r, rm, imm) => format_args!("imul {}, {}, {}", r, rm, imm),
        R32Rm32Imm32(r, rm, imm) => format_args!("imul {}, {}, {}", r, rm, imm),
        R64Rm64Imm32(r, rm, imm) => format_args!("imul {}, {}, {}", r, rm, imm),
    },

    In {
        AlImm8(imm)  => format_args!("in al, {}", imm),
        AxImm8(imm)  => format_args!("in ax, {}", imm),
        EaxImm8(imm) => format_args!("in eax, {}", imm),

        AlDx  => format_args!("in al, dx"),
        AxDx  => format_args!("in ax, dx"),
        EaxDx => format_args!("in eax, dx"),
    },

    Inc("inc", 1, { Rm8l, Rm8, Rm16, Rm32, Rm64 }),

    Ins {
        B => format_args!("insb"),
        W => format_args!("insw"),
        D => format_args!("insd"),
    },
    Int3("int 3"),
    Int("int", 1, { Imm8 }),
    Into("into"),
    Invd("invd"),
    Invlpg("invlpg", 1, { M }),
    Invpcid("invpcid", 2, { R64M128 }),

    Iret {
        D => format_args!("iretd"),
        Q => format_args!("iretq"),
    },

    J {
        CcRel8(cc, rel)  => format_args!("j{} short {}", cc, rel),
        RcxzRel8(rel)    => format_args!("jrcxz short {}", rel),
        CcRel32(cc, rel) => format_args!("j{} near {}", cc, rel),
    },

    Jmp {
        Rel8(rel)  => format_args!("jmp short {}", rel),
        Rel32(rel) => format_args!("jmp rel {}", rel),
        Rm64(rm)   => format_args!("jmp near {}", rm),
        M16x16(m)  => format_args!("jmp far word {}", m),
        M16x32(m)  => format_args!("jmp far dword {}", m),
        M16x64(m)  => format_args!("jmp far qword {}", m),
    },

    Lahf("lahf"),
    Lss("lss", 2, { R16M16x16, R32M16x32, R64M16x64 }),
    Lfs("lfs", 2, { R16M16x16, R32M16x32, R64M16x64 }),
    Lgs("lgs", 2, { R16M16x16, R32M16x32, R64M16x64 }),
    Lea("lea", 2, { R16M, R32M, R64M }),
    Leave("leave"),
    Lfence("lfence"),
    Lgdt("lgdt", 1, { M16x64 }),
    Lidt("lidt", 1, { M16x64 }),
    Lldt("lldt", 1, { Rm16 }),
    Lmsw("lmsw", 1, { Rm16 }),

    Lods {
        B => format_args!("lodsb"),
        W => format_args!("lodsw"),
        D => format_args!("lodsd"),
        Q => format_args!("lodsq"),
    },

    Loop {
        Rel8(rel)   => format_args!("loop {}", rel),
        ERel8(rel)  => format_args!("loope {}", rel),
        NeRel8(rel) => format_args!("loopne {}", rel),
    },

    Ltr("ltr", 1, { Rm16 }),
    Lzcnt("lzcnt", 2, { R16Rm16, R32Rm32, R64Rm64 }),
    Mfence("mfence"),
    Monitor("monitor"),

    Mov {
        Rm8lR8l(rm, r) => format_args!("mov {}, {}", rm, r),
        Rm8R8(rm, r)   => format_args!("mov {}, {}", rm, r),
        Rm16R16(rm, r) => format_args!("mov {}, {}", rm, r),
        Rm32R32(rm, r) => format_args!("mov {}, {}", rm, r),
        Rm64R64(rm, r) => format_args!("mov {}, {}", rm, r),

        R8lRm8l(r, rm) => format_args!("mov {}, {}", r, rm),
        R8Rm8(r, rm)   => format_args!("mov {}, {}", r, rm),
        R16Rm16(r, rm) => format_args!("mov {}, {}", r, rm),
        R32Rm32(r, rm) => format_args!("mov {}, {}", r, rm),
        R64Rm64(r, rm) => format_args!("mov {}, {}", r, rm),

        Rm16Sreg(rm, sreg) => format_args!("mov {}, {}", rm, sreg),
        Rm64Sreg(rm, sreg) => format_args!("mov {}, {}", rm, sreg),

        SregRm16(sreg, rm) => format_args!("mov {}, {}", sreg, rm),
        SregRm64(sreg, rm) => format_args!("mov {}, {}", sreg, rm),

        AlMoffs8(moffs)   => format_args!("mov al, {}", moffs),
        AxMoffs16(moffs)  => format_args!("mov ax, {}", moffs),
        EaxMoffs32(moffs) => format_args!("mov eax, {}", moffs),
        RaxMoffs64(moffs) => format_args!("mov rax, {}", moffs),

        Moffs8Al(moffs)   => format_args!("mov {}, al", moffs),
        Moffs16Ax(moffs)  => format_args!("mov {}, ax", moffs),
        Moffs32Eax(moffs) => format_args!("mov {}, eax", moffs),
        Moffs64Rax(moffs) => format_args!("mov {}, rax", moffs),

        R8lImm8(r, imm)  => format_args!("mov {}, {}", r, imm),
        R8Imm8(r, imm)   => format_args!("mov {}, {}", r, imm),
        R16Imm16(r, imm) => format_args!("mov {}, {}", r, imm),
        R32Imm32(r, imm) => format_args!("mov {}, {}", r, imm),
        R64Imm64(r, imm) => format_args!("mov {}, {}", r, imm),

        Rm8lImm8(rm, imm)  => format_args!("mov {}, {}", rm, imm),
        Rm8Imm8(rm, imm)   => format_args!("mov {}, {}", rm, imm),
        Rm16Imm16(rm, imm) => format_args!("mov {}, {}", rm, imm),
        Rm32Imm32(rm, imm) => format_args!("mov {}, {}", rm, imm),
        Rm64Imm32(rm, imm) => format_args!("mov {}, {}", rm, imm),

        R64Cr(r, cr) => format_args!("mov {}, {}", r, cr),
        CrR64(cr, r) => format_args!("mov {}, {}", cr, r),

        R64Dr(r, dr) => format_args!("mov {}, {}", r, dr),
        DrR64(dr, r) => format_args!("mov {}, {}", dr, r),
    },

    Movbe("movbe", 2, { R16M16, R32M32, R64M64, M16R16, M32R32, M64R64 }),

    Movs {
        B => format_args!("movsb"),
        W => format_args!("movsw"),
        D => format_args!("movsd"),
        Q => format_args!("movsq"),
    },

    Movsx("movsx", 2, { R16lRm8l, R16Rm8, R32lRm8l, R32Rm8, R64Rm8, R32Rm16, R64Rm16, R64Rm32 }),
    Movzx("movzx", 2, { R16lRm8l, R16Rm8, R32lRm8l, R32Rm8, R64Rm8, R32Rm16, R64Rm16 }),
    Mul("mul", 1, { Rm8l, Rm8, Rm16, Rm32, Rm64 }),
    Mwait("mwait"),
    Neg("neg", 1, { Rm8l, Rm8, Rm16, Rm32, Rm64 }),

    Nop {
        Ax  => format_args!("nop ax"),
        Eax => format_args!("nop eax"),

        Rm16(rm) => format_args!("nop {}", rm),
        Rm32(rm) => format_args!("nop {}", rm),
    },

    Not("not", 1, { Rm8l, Rm8, Rm16, Rm32, Rm64 }),
    Or("or", a),

    Out {
        Imm8Al(imm)  => format_args!("out {}, al", imm),
        Imm8Ax(imm)  => format_args!("out {}, ax", imm),
        Imm8Eax(imm) => format_args!("out {}, eax", imm),

        DxAl  => format_args!("out dx, al"),
        DxAx  => format_args!("out dx, ax"),
        DxEax => format_args!("out dx, eax"),
    },

    Outs {
        B => format_args!("outsb"),
        W => format_args!("outsw"),
        D => format_args!("outsd"),
    },

    Pause("pause"),

    Pop {
        Rm16(rm) => format_args!("pop {}", rm),
        Rm64(rm) => format_args!("pop {}", rm),

        R16(r) => format_args!("pop {}", r),
        R64(r) => format_args!("pop {}", r),

        Fs16 => format_args!("pop word fs"),
        Fs64 => format_args!("pop qword fs"),
        Gs16 => format_args!("pop word gs"),
        Gs64 => format_args!("pop qword gs"),
    },

    Popcnt("popcnt", 2, { R16Rm16, R32Rm32, R64Rm64 }),

    Popf {
        W => format_args!("popf"),
        Q => format_args!("popfq"),
    },

    Prefetch {
        T0M8(m)  => format_args!("prefetcht0 {}", m),
        T1M8(m)  => format_args!("prefetcht1 {}", m),
        T2M8(m)  => format_args!("prefetcht2 {}", m),
        NtaM8(m) => format_args!("prefetchnta {}", m),
    },

    Prefetchw("prefetchw", 1, { M8 }),
    Prefetchwt1("prefetchwt1", 1, { M8 }),
    Ptwrite("ptwrite", 1, { Rm32, Rm64 }),

    Push {
        Rm16(rm) => format_args!("push {}", rm),
        Rm64(rm) => format_args!("push {}", rm),

        R16(r) => format_args!("push {}", r),
        R64(r) => format_args!("push {}", r),

        Imm8(imm)  => format_args!("push {}", imm),
        Imm16(imm) => format_args!("push {}", imm),
        Imm32(imm) => format_args!("push {}", imm),

        Fs16 => format_args!("push word fs"),
        Fs64 => format_args!("push qword fs"),
        Gs16 => format_args!("push word gs"),
        Gs64 => format_args!("push qword gs"),
    },

    Pushf {
        W => format_args!("pushf"),
        Q => format_args!("pushfq"),
    },

    Rcl {
        Rm8l(rm)          => format_args!("rcl {}, 1", rm),
        Rm8(rm)           => format_args!("rcl {}, 1", rm),
        Rm8lCl(rm)        => format_args!("rcl {}, cl", rm),
        Rm8Cl(rm)         => format_args!("rcl {}, cl", rm),
        Rm8lImm8(rm, imm) => format_args!("rcl {}, {}", rm, imm),
        Rm8Imm8(rm, imm)  => format_args!("rcl {}, {}", rm, imm),

        Rm16(rm)          => format_args!("rcl {}, 1", rm),
        Rm16Cl(rm)        => format_args!("rcl {}, cl", rm),
        Rm16Imm8(rm, imm) => format_args!("rcl {}, {}", rm, imm),

        Rm32(rm)          => format_args!("rcl {}, 1", rm),
        Rm64(rm)          => format_args!("rcl {}, 1", rm),
        Rm32Cl(rm)        => format_args!("rcl {}, cl", rm),
        Rm64Cl(rm)        => format_args!("rcl {}, cl", rm),
        Rm32Imm8(rm, imm) => format_args!("rcl {}, {}", rm, imm),
        Rm64Imm8(rm, imm) => format_args!("rcl {}, {}", rm, imm),
    },

    Rcr {
        Rm8l(rm)          => format_args!("rcr {}, 1", rm),
        Rm8(rm)           => format_args!("rcr {}, 1", rm),
        Rm8lCl(rm)        => format_args!("rcr {}, cl", rm),
        Rm8Cl(rm)         => format_args!("rcr {}, cl", rm),
        Rm8lImm8(rm, imm) => format_args!("rcr {}, {}", rm, imm),
        Rm8Imm8(rm, imm)  => format_args!("rcr {}, {}", rm, imm),

        Rm16(rm)          => format_args!("rcr {}, 1", rm),
        Rm16Cl(rm)        => format_args!("rcr {}, cl", rm),
        Rm16Imm8(rm, imm) => format_args!("rcr {}, {}", rm, imm),

        Rm32(rm)          => format_args!("rcr {}, 1", rm),
        Rm64(rm)          => format_args!("rcr {}, 1", rm),
        Rm32Cl(rm)        => format_args!("rcr {}, cl", rm),
        Rm64Cl(rm)        => format_args!("rcr {}, cl", rm),
        Rm32Imm8(rm, imm) => format_args!("rcr {}, {}", rm, imm),
        Rm64Imm8(rm, imm) => format_args!("rcr {}, {}", rm, imm),
    },

    Rol {
        Rm8l(rm)          => format_args!("rol {}, 1", rm),
        Rm8(rm)           => format_args!("rol {}, 1", rm),
        Rm8lCl(rm)        => format_args!("rol {}, cl", rm),
        Rm8Cl(rm)         => format_args!("rol {}, cl", rm),
        Rm8lImm8(rm, imm) => format_args!("rol {}, {}", rm, imm),
        Rm8Imm8(rm, imm)  => format_args!("rol {}, {}", rm, imm),

        Rm16(rm)          => format_args!("rol {}, 1", rm),
        Rm16Cl(rm)        => format_args!("rol {}, cl", rm),
        Rm16Imm8(rm, imm) => format_args!("rol {}, {}", rm, imm),

        Rm32(rm)          => format_args!("rol {}, 1", rm),
        Rm64(rm)          => format_args!("rol {}, 1", rm),
        Rm32Cl(rm)        => format_args!("rol {}, cl", rm),
        Rm64Cl(rm)        => format_args!("rol {}, cl", rm),
        Rm32Imm8(rm, imm) => format_args!("rol {}, {}", rm, imm),
        Rm64Imm8(rm, imm) => format_args!("rol {}, {}", rm, imm),
    },

    Ror {
        Rm8l(rm)          => format_args!("ror {}, 1", rm),
        Rm8(rm)           => format_args!("ror {}, 1", rm),
        Rm8lCl(rm)        => format_args!("ror {}, cl", rm),
        Rm8Cl(rm)         => format_args!("ror {}, cl", rm),
        Rm8lImm8(rm, imm) => format_args!("ror {}, {}", rm, imm),
        Rm8Imm8(rm, imm)  => format_args!("ror {}, {}", rm, imm),

        Rm16(rm)          => format_args!("ror {}, 1", rm),
        Rm16Cl(rm)        => format_args!("ror {}, cl", rm),
        Rm16Imm8(rm, imm) => format_args!("ror {}, {}", rm, imm),

        Rm32(rm)          => format_args!("ror {}, 1", rm),
        Rm64(rm)          => format_args!("ror {}, 1", rm),
        Rm32Cl(rm)        => format_args!("ror {}, cl", rm),
        Rm64Cl(rm)        => format_args!("ror {}, cl", rm),
        Rm32Imm8(rm, imm) => format_args!("ror {}, {}", rm, imm),
        Rm64Imm8(rm, imm) => format_args!("ror {}, {}", rm, imm),
    },

    Rdfsbase("rdfsbase", 1, { R32, R64 }),
    Rdgsbase("rdgsbase", 1, { R32, R64 }),
    Rdmsr("rdmsr"),
    Rdpid("rdpid", 1, { R64 }),
    Rdpkru("rdpkru"),
    Rdpmc("rdpmc"),
    Rdrand("rdrand", 1, { R16, R32, R64 }),
    Rdseed("rdseed", 1, { R16, R32, R64 }),
    Rdtsc("rdtsc"),
    Rdtscp("rdtscp"),

    Ret {
        Near => format_args!("ret near"),
        Far  => format_args!("ret far"),

        NearImm16(imm) => format_args!("ret near {}", imm),
        FarImm16(imm)  => format_args!("ret far {}", imm),
    },

    Rsm("rsm"),
    Sahf("sahf"),

    Sal {
        Rm8l(rm)          => format_args!("sal {}, 1", rm),
        Rm8(rm)           => format_args!("sal {}, 1", rm),
        Rm8lCl(rm)        => format_args!("sal {}, cl", rm),
        Rm8Cl(rm)         => format_args!("sal {}, cl", rm),
        Rm8lImm8(rm, imm) => format_args!("sal {}, {}", rm, imm),
        Rm8Imm8(rm, imm)  => format_args!("sal {}, {}", rm, imm),

        Rm16(rm)          => format_args!("sal {}, 1", rm),
        Rm16Cl(rm)        => format_args!("sal {}, cl", rm),
        Rm16Imm8(rm, imm) => format_args!("sal {}, {}", rm, imm),

        Rm32(rm)          => format_args!("sal {}, 1", rm),
        Rm64(rm)          => format_args!("sal {}, 1", rm),
        Rm32Cl(rm)        => format_args!("sal {}, cl", rm),
        Rm64Cl(rm)        => format_args!("sal {}, cl", rm),
        Rm32Imm8(rm, imm) => format_args!("sal {}, {}", rm, imm),
        Rm64Imm8(rm, imm) => format_args!("sal {}, {}", rm, imm),
    },

    Sar {
        Rm8l(rm)          => format_args!("sar {}, 1", rm),
        Rm8(rm)           => format_args!("sar {}, 1", rm),
        Rm8lCl(rm)        => format_args!("sar {}, cl", rm),
        Rm8Cl(rm)         => format_args!("sar {}, cl", rm),
        Rm8lImm8(rm, imm) => format_args!("sar {}, {}", rm, imm),
        Rm8Imm8(rm, imm)  => format_args!("sar {}, {}", rm, imm),

        Rm16(rm)          => format_args!("sar {}, 1", rm),
        Rm16Cl(rm)        => format_args!("sar {}, cl", rm),
        Rm16Imm8(rm, imm) => format_args!("sar {}, {}", rm, imm),

        Rm32(rm)          => format_args!("sar {}, 1", rm),
        Rm64(rm)          => format_args!("sar {}, 1", rm),
        Rm32Cl(rm)        => format_args!("sar {}, cl", rm),
        Rm64Cl(rm)        => format_args!("sar {}, cl", rm),
        Rm32Imm8(rm, imm) => format_args!("sar {}, {}", rm, imm),
        Rm64Imm8(rm, imm) => format_args!("sar {}, {}", rm, imm),
    },

    Shl {
        Rm8l(rm)          => format_args!("shl {}, 1", rm),
        Rm8(rm)           => format_args!("shl {}, 1", rm),
        Rm8lCl(rm)        => format_args!("shl {}, cl", rm),
        Rm8Cl(rm)         => format_args!("shl {}, cl", rm),
        Rm8lImm8(rm, imm) => format_args!("shl {}, {}", rm, imm),
        Rm8Imm8(rm, imm)  => format_args!("shl {}, {}", rm, imm),

        Rm16(rm)          => format_args!("shl {}, 1", rm),
        Rm16Cl(rm)        => format_args!("shl {}, cl", rm),
        Rm16Imm8(rm, imm) => format_args!("shl {}, {}", rm, imm),

        Rm32(rm)          => format_args!("shl {}, 1", rm),
        Rm64(rm)          => format_args!("shl {}, 1", rm),
        Rm32Cl(rm)        => format_args!("shl {}, cl", rm),
        Rm64Cl(rm)        => format_args!("shl {}, cl", rm),
        Rm32Imm8(rm, imm) => format_args!("shl {}, {}", rm, imm),
        Rm64Imm8(rm, imm) => format_args!("shl {}, {}", rm, imm),
    },

    Shr {
        Rm8l(rm)          => format_args!("shr {}, 1", rm),
        Rm8(rm)           => format_args!("shr {}, 1", rm),
        Rm8lCl(rm)        => format_args!("shr {}, cl", rm),
        Rm8Cl(rm)         => format_args!("shr {}, cl", rm),
        Rm8lImm8(rm, imm) => format_args!("shr {}, {}", rm, imm),
        Rm8Imm8(rm, imm)  => format_args!("shr {}, {}", rm, imm),

        Rm16(rm)          => format_args!("shr {}, 1", rm),
        Rm16Cl(rm)        => format_args!("shr {}, cl", rm),
        Rm16Imm8(rm, imm) => format_args!("shr {}, {}", rm, imm),

        Rm32(rm)          => format_args!("shr {}, 1", rm),
        Rm64(rm)          => format_args!("shr {}, 1", rm),
        Rm32Cl(rm)        => format_args!("shr {}, cl", rm),
        Rm64Cl(rm)        => format_args!("shr {}, cl", rm),
        Rm32Imm8(rm, imm) => format_args!("shr {}, {}", rm, imm),
        Rm64Imm8(rm, imm) => format_args!("shr {}, {}", rm, imm),
    },

    Sbb("sbb", a),

    Scas {
        B => format_args!("scasb"),
        W => format_args!("scasw"),
        D => format_args!("scasd"),
        Q => format_args!("scasq"),
    },

    Set {
        CcRm8l(cc, rm) => format_args!("set{} {}", cc, rm),
        CcRm8(cc, rm)  => format_args!("set{} {}", cc, rm),
    },

    Sfence("sfence"),
    Sgdt("sgdt", 1, { M }),

    Shld {
        Rm16R16Imm8(rm, r, imm) => format_args!("shld {}, {}, {}", rm, r, imm),
        Rm16R16Cl(rm, r)        => format_args!("shld {}, {}, cl", rm, r),
        Rm32R32Imm8(rm, r, imm) => format_args!("shld {}, {}, {}", rm, r, imm),
        Rm64R64Imm8(rm, r, imm) => format_args!("shld {}, {}, {}", rm, r, imm),
        Rm32R32Cl(rm, r)        => format_args!("shld {}, {}, cl", rm, r),
        Rm64R64Cl(rm, r)        => format_args!("shld {}, {}, cl", rm, r),
    },

    Shrd {
        Rm16R16Imm8(rm, r, imm) => format_args!("shrd {}, {}, {}", rm, r, imm),
        Rm16R16Cl(rm, r)        => format_args!("shrd {}, {}, cl", rm, r),
        Rm32R32Imm8(rm, r, imm) => format_args!("shrd {}, {}, {}", rm, r, imm),
        Rm64R64Imm8(rm, r, imm) => format_args!("shrd {}, {}, {}", rm, r, imm),
        Rm32R32Cl(rm, r)        => format_args!("shrd {}, {}, cl", rm, r),
        Rm64R64Cl(rm, r)        => format_args!("shrd {}, {}, cl", rm, r),
    },

    Sidt("sidt", 1, { M }),
    Sldt("sldt", 1, { Rm16, Rm64 }),
    Smsw("smsw", 1, { Rm16, Rm32, Rm64 }),
}
