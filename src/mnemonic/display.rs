use mnemonic::instruction::*;
use mnemonic::operand::*;

use core::fmt::{Display, Formatter, Error};

macro_rules! impl_display_int {
    ($ty:ident, $fmt:expr) => {
        impl Display for $ty {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                write!(f, $fmt, self.0)
            }
        }
    }
}

macro_rules! impl_display_reg {
    ($ty:ident { $($var:ident => $str:expr,)+ }) => {
        impl Display for $ty {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                f.write_str(match *self { $($ty::$var => $str),+ })
            }
        }
    }
}

macro_rules! impl_display_rm {
    ($ty:ident, $r:ident, $mstr:expr, $m:ident) => {
        impl Display for $ty {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                match *self {
                    $ty::$r(r) => write!(f, "{}", r),
                    $ty::$m(m) => write!(f, concat!($mstr, " {}"), m),
                }
            }
        }
    }
}

macro_rules! impl_display_str {
    ($str:expr, $ty:ident) => {
        impl Display for $ty {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                f.write_str($str)
            }
        }
    }
}

macro_rules! impl_display_unary {
    ($str:expr, $ty:ident { $($var:ident),+ }) => {
        impl Display for $ty {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                match *self {
                    $($ty::$var(a) => write!(f, concat!($str, " {}"), a)),+
                }
            }
        }
    }
}

macro_rules! impl_display_binary {
    ($str:expr, $ty:ident { $($var:ident),+ }) => {
        impl Display for $ty {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                match *self {
                    $($ty::$var(a, b) => write!(f, concat!($str, " {}, {}"), a, b)),+
                }
            }
        }
    }
}

macro_rules! impl_display_arith {
    ($str:expr, $ty:ident) => {
        impl Display for $ty {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                match *self {
                    $ty::AlImm8(imm)        => write!(f, concat!($str, " al, {}"), imm),
                    $ty::AxImm16(imm)       => write!(f, concat!($str, " ax, {}"), imm),
                    $ty::EaxImm32(imm)      => write!(f, concat!($str, " eax, {}"), imm),
                    $ty::RaxImm32(imm)      => write!(f, concat!($str, " rax, {}"), imm),
                    $ty::Rm8lImm8(rm, imm)  => write!(f, concat!($str, " {}, {}"), rm, imm),
                    $ty::Rm8Imm8(rm, imm)   => write!(f, concat!($str, " {}, {}"), rm, imm),
                    $ty::Rm16Imm16(rm, imm) => write!(f, concat!($str, " {}, {}"), rm, imm),
                    $ty::Rm32Imm32(rm, imm) => write!(f, concat!($str, " {}, {}"), rm, imm),
                    $ty::Rm64Imm32(rm, imm) => write!(f, concat!($str, " {}, {}"), rm, imm),
                    $ty::Rm16Imm8(rm, imm)  => write!(f, concat!($str, " {}, {}"), rm, imm),
                    $ty::Rm32Imm8(rm, imm)  => write!(f, concat!($str, " {}, {}"), rm, imm),
                    $ty::Rm64Imm8(rm, imm)  => write!(f, concat!($str, " {}, {}"), rm, imm),
                    $ty::Rm8lR8l(rm, r)     => write!(f, concat!($str, " {}, {}"), rm, r),
                    $ty::Rm8R8(rm, r)       => write!(f, concat!($str, " {}, {}"), rm, r),
                    $ty::Rm16R16(rm, r)     => write!(f, concat!($str, " {}, {}"), rm, r),
                    $ty::Rm32R32(rm, r)     => write!(f, concat!($str, " {}, {}"), rm, r),
                    $ty::Rm64R64(rm, r)     => write!(f, concat!($str, " {}, {}"), rm, r),
                    $ty::R8lRm8l(r, rm)     => write!(f, concat!($str, " {}, {}"), r, rm),
                    $ty::R8Rm8(r, rm)       => write!(f, concat!($str, " {}, {}"), r, rm),
                    $ty::R16Rm16(r, rm)     => write!(f, concat!($str, " {}, {}"), r, rm),
                    $ty::R32Rm32(r, rm)     => write!(f, concat!($str, " {}, {}"), r, rm),
                    $ty::R64Rm64(r, rm)     => write!(f, concat!($str, " {}, {}"), r, rm),
                }
            }
        }
    }
}

macro_rules! impl_display_farith {
    ($fstr:expr, $fpstr:expr, $fistr:expr, $fty:ident, $fpty:ident, $fity:ident) => {
        impl Display for $fty {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                match *self {
                    $fty::M32fp(m)  => write!(f, concat!($fstr, " dword {}"), m),
                    $fty::M64fp(m)  => write!(f, concat!($fstr, " qword {}"), m),
                    $fty::St0Sti(i) => write!(f, concat!($fstr, " st0, {}"), i),
                    $fty::StiSt0(i) => write!(f, concat!($fstr, " {}, st0"), i),
                }
            }
        }

        impl Display for $fpty {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                match *self {
                    $fpty::StiSt0(i) => write!(f, concat!($fpstr, " {}, st0"), i),
                }
            }
        }

        impl Display for $fity {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                match *self {
                    $fity::M32int(m) => write!(f, concat!($fistr, " dword {}"), m),
                    $fity::M16int(m) => write!(f, concat!($fistr, " word {}"), m),
                }
            }
        }
    }
}

impl_display_int!(Imm8, "{:#04x}");
impl_display_int!(Imm16, "{:#06x}");
impl_display_int!(Imm32, "{:#010x}");
impl_display_int!(Imm64, "{:#018x}");
impl_display_int!(Rel8, "{:#04x}");
impl_display_int!(Rel32, "{:#010x}");

impl_display_reg!(
    Cc {
        A   => "a",
        Ae  => "ae",
        B   => "b",
        Be  => "be",
        C   => "c",
        E   => "e",
        G   => "g",
        Ge  => "ge",
        L   => "l",
        Le  => "le",
        Na  => "na",
        Nae => "nae",
        Nb  => "nb",
        Nbe => "nbe",
        Nc  => "nc",
        Ne  => "ne",
        Ng  => "ng",
        Nge => "nge",
        Nl  => "nl",
        Nle => "nle",
        No  => "no",
        Np  => "np",
        Ns  => "ns",
        Nz  => "nz",
        O   => "o",
        P   => "p",
        Pe  => "pe",
        Po  => "po",
        S   => "s",
        Z   => "z",
    }
);

impl_display_reg!(
    R8l {
        Al => "al",
        Bl => "bl",
        Cl => "cl",
        Dl => "dl",
        Ah => "ah",
        Bh => "bh",
        Ch => "ch",
        Dh => "dh",
    }
);

impl_display_reg!(
    R8 {
        Al => "al",
        Bl => "bl",
        Cl => "cl",
        Dl => "dl",
        Sil => "sil",
        Dil => "dil",
        Bpl => "bpl",
        Spl => "spl",
        R8l => "r8l",
        R9l => "r9l",
        R10l => "r10l",
        R11l => "r11l",
        R12l => "r12l",
        R13l => "r13l",
        R14l => "r14l",
        R15l => "r15l",
    }
);

impl_display_reg!(
    R16 {
        Ax => "ax",
        Bx => "bx",
        Cx => "cx",
        Dx => "dx",
        Si => "si",
        Di => "di",
        Bp => "bp",
        Sp => "sp",
        R8w => "r8w",
        R9w => "r9w",
        R10w => "r10w",
        R11w => "r11w",
        R12w => "r12w",
        R13w => "r13w",
        R14w => "r14w",
        R15w => "r15w",
    }
);

impl_display_reg!(
    R32l {
        Eax => "eax",
        Ebx => "ebx",
        Ecx => "ecx",
        Edx => "edx",
        Esi => "esi",
        Edi => "edi",
        Ebp => "ebp",
        Esp => "esp",
    }
);

impl_display_reg!(
    R32 {
        Eax => "eax",
        Ebx => "ebx",
        Ecx => "ecx",
        Edx => "edx",
        Esi => "esi",
        Edi => "edi",
        Ebp => "ebp",
        Esp => "esp",
        R8d => "r8d",
        R9d => "r9d",
        R10d => "r10d",
        R11d => "r11d",
        R12d => "r12d",
        R13d => "r13d",
        R14d => "r14d",
        R15d => "r15d",
    }
);

impl_display_reg!(
    R64l {
        Rax => "rax",
        Rbx => "rbx",
        Rcx => "rcx",
        Rdx => "rdx",
        Rsi => "rsi",
        Rdi => "rdi",
        Rbp => "rbp",
        Rsp => "rsp",
    }
);

impl_display_reg!(
    R64 {
        Rax => "rax",
        Rbx => "rbx",
        Rcx => "rcx",
        Rdx => "rdx",
        Rsi => "rsi",
        Rdi => "rdi",
        Rbp => "rbp",
        Rsp => "rsp",
        R8 => "r8",
        R9 => "r9",
        R10 => "r10",
        R11 => "r11",
        R12 => "r12",
        R13 => "r13",
        R14 => "r14",
        R15 => "r15",
    }
);

impl_display_reg!(
    Sti {
        St0 => "st0",
        St1 => "st1",
        St2 => "st2",
        St3 => "st3",
        St4 => "st4",
        St5 => "st5",
        St6 => "st6",
        St7 => "st7",
    }
);

impl_display_reg!(
    Cr {
        Cr0 => "cr0",
        Cr2 => "cr2",
        Cr3 => "cr3",
        Cr4 => "cr4",
        Cr8 => "cr8",
    }
);

impl_display_reg!(
    Dr {
        Dr0 => "dr0",
        Dr1 => "dr1",
        Dr2 => "dr2",
        Dr3 => "dr3",
        Dr4 => "dr4",
        Dr5 => "dr5",
        Dr6 => "dr6",
        Dr7 => "dr7",
    }
);

impl_display_reg!(
    Index32l {
        Eax => "eax",
        Ebx => "ebx",
        Ecx => "ecx",
        Edx => "edx",
        Esi => "esi",
        Edi => "edi",
        Ebp => "ebp",
    }
);

impl_display_reg!(
    Index32 {
        Eax => "eax",
        Ebx => "ebx",
        Ecx => "ecx",
        Edx => "edx",
        Esi => "esi",
        Edi => "edi",
        Ebp => "ebp",
        R8d => "r8d",
        R9d => "r9d",
        R10d => "r10d",
        R11d => "r11d",
        R12d => "r12d",
        R13d => "r13d",
        R14d => "r14d",
        R15d => "r15d",
    }
);

impl_display_reg!(
    Index64l {
        Rax => "rax",
        Rbx => "rbx",
        Rcx => "rcx",
        Rdx => "rdx",
        Rsi => "rsi",
        Rdi => "rdi",
        Rbp => "rbp",
    }
);

impl_display_reg!(
    Index64 {
        Rax => "rax",
        Rbx => "rbx",
        Rcx => "rcx",
        Rdx => "rdx",
        Rsi => "rsi",
        Rdi => "rdi",
        Rbp => "rbp",
        R8 => "r8",
        R9 => "r9",
        R10 => "r10",
        R11 => "r11",
        R12 => "r12",
        R13 => "r13",
        R14 => "r14",
        R15 => "r15",
    }
);

impl_display_reg!(
    Scale {
        X1 => "1",
        X2 => "2",
        X4 => "4",
        X8 => "8",
    }
);

impl Display for Disp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Disp::Disp8(disp) => write!(f, "{:#04x}", disp),
            Disp::Disp32(disp) => write!(f, "{:#010x}", disp),
        }
    }
}

impl<Base, Index> Display for Offset<Base, Index>
where Base: Display + Copy, Index: Display + Copy {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Offset::Base(b)                   => write!(f, "{}", b),
            Offset::BaseDisp(b, d)            => write!(f, "{} + {}", b, d),
            Offset::BaseIndex(b, i, s)        => write!(f, "{} + {} * {}", b, i, s),
            Offset::BaseIndexDisp(b, i, s, d) => write!(f, "{} + {} * {} + {}", b, i, s, d),
            Offset::IndexDisp(i, s, d)        => write!(f, "{} * {} + {:#010x}", i, s, d),
            Offset::Disp(d)                   => write!(f, "{:#010x}", d),
            Offset::RipDisp(d)                => write!(f, "rip + {:#010x}", d),
        }
    }
}

impl_display_reg!(
    Sreg {
        Cs => "cs",
        Ds => "ds",
        Ss => "ss",
        Es => "es",
        Fs => "fs",
        Gs => "gs",
    }
);

impl<B32, I32, B64, I64> Display for Mem<B32, I32, B64, I64>
where Offset<B32, I32>: Display, Offset<B64, I64>: Display {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Mem::Offset32(None, ref offset)       => write!(f, "[{}]", offset),
            Mem::Offset32(Some(sreg), ref offset) => write!(f, "[{}:{}]", sreg, offset),
            Mem::Offset64(None, ref offset)       => write!(f, "[{}]", offset),
            Mem::Offset64(Some(sreg), ref offset) => write!(f, "[{}:{}]", sreg, offset),
        }
    }
}

impl Display for Moffs {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Moffs::Moffset32(None, offset)       => write!(f, "[dword {:#010x}]", offset),
            Moffs::Moffset32(Some(sreg), offset) => write!(f, "[{}:dword {:#010x}]", sreg, offset),
            Moffs::Moffset64(None, offset)       => write!(f, "[qword {:#018x}]", offset),
            Moffs::Moffset64(Some(sreg), offset) => write!(f, "[{}:qword {:#018x}]", sreg, offset),
        }
    }
}

impl_display_rm!(Rm8l, R8l, "byte", M8l);
impl_display_rm!(Rm8, R8, "byte", M8);
impl_display_rm!(Rm16, R16, "word", M16);
impl_display_rm!(Rm32, R32, "dword", M32);
impl_display_rm!(Rm64, R64, "qword", M64);

impl_display_arith!("adc", Adc);
impl_display_binary!("adcx", Adcx { R32Rm32, R64Rm64 });
impl_display_arith!("add", Add);
impl_display_binary!("adox", Adox { R32Rm32, R64Rm64 });
impl_display_arith!("and", And);
impl_display_binary!("bsf", Bsf { R16Rm16, R32Rm32, R64Rm64 });
impl_display_binary!("bsr", Bsr { R16Rm16, R32Rm32, R64Rm64 });
impl_display_unary!("bswap", Bswap { R32, R64 });
impl_display_binary!("bt", Bt { Rm16R16, Rm32R32, Rm64R64, Rm16Imm8, Rm32Imm8, Rm64Imm8 });
impl_display_binary!("btc", Btc { Rm16R16, Rm32R32, Rm64R64, Rm16Imm8, Rm32Imm8, Rm64Imm8 });
impl_display_binary!("btr", Btr { Rm16R16, Rm32R32, Rm64R64, Rm16Imm8, Rm32Imm8, Rm64Imm8 });
impl_display_binary!("bts", Bts { Rm16R16, Rm32R32, Rm64R64, Rm16Imm8, Rm32Imm8, Rm64Imm8 });

impl Display for Call {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Call::Rel32(rel) => write!(f, "call rel {}", rel),
            Call::Rm64(rm)   => write!(f, "call near {}", rm),
            Call::M16x16(m)  => write!(f, "call far word {}", m),
            Call::M16x32(m)  => write!(f, "call far dword {}", m),
            Call::M16x64(m)  => write!(f, "call far qword {}", m),
        }
    }
}

impl_display_str!("cbw", Cbw);
impl_display_str!("cwde", Cwde);
impl_display_str!("cdqe", Cdqe);
impl_display_str!("clac", Clac);
impl_display_str!("clc", Clc);
impl_display_str!("cld", Cld);
impl_display_unary!("clflush", Clflush { M8 });
impl_display_unary!("clflushopt", Clflushopt { M8 });
impl_display_str!("cli", Cli);
impl_display_str!("clts", Clts);
impl_display_str!("cmc", Cmc);

impl Display for Cmov {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Cmov::CcR16Rm16(cc, r, rm) => write!(f, "cmov{} {}, {}", cc, r, rm),
            Cmov::CcR32Rm32(cc, r, rm) => write!(f, "cmov{} {}, {}", cc, r, rm),
            Cmov::CcR64Rm64(cc, r, rm) => write!(f, "cmov{} {}, {}", cc, r, rm),
        }
    }
}

impl_display_arith!("cmp", Cmp);
impl_display_reg!(
    Cmps {
        B => "cmpsb",
        W => "cmpsw",
        D => "cmpsd",
        Q => "cmpsq",
    }
);
impl_display_binary!("cmpxchg", Cmpxchg { Rm8lR8l, Rm8R8, Rm16R16, Rm32R32, Rm64R64 });
impl_display_unary!("cmpxchg8b", Cmpxchg8b { M64 });
impl_display_unary!("cmpxchg16b", Cmpxchg16b { M128 });
impl_display_str!("cpuid", Cpuid);
impl_display_binary!("crc32", Crc32 { R32lRm8l, R32Rm8, R32Rm16, R32Rm32, R64Rm8, R64Rm64 });
impl_display_str!("cwd", Cwd);
impl_display_str!("cdq", Cdq);
impl_display_str!("cqo", Cqo);
impl_display_unary!("dec", Dec { Rm8l, Rm8, Rm16, Rm32, Rm64 });
impl_display_unary!("div", Div { Rm8l, Rm8, Rm16, Rm32, Rm64 });
impl_display_str!("f2xm1", F2xm1);
impl_display_str!("fabs", Fabs);
impl_display_farith!("fadd", "faddp", "fiadd", Fadd, Faddp, Fiadd);
impl_display_unary!("fbld", Fbld { M80dec });
impl_display_unary!("fbstp", Fbstp { M80bcd });
impl_display_str!("fchs", Fchs);
impl_display_str!("fclex", Fclex);
impl_display_str!("fnclex", Fnclex);

impl Display for Fcmov {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Fcmov::BSt0Sti(i)   => write!(f, "fcmovb st0, {}", i),
            Fcmov::ESt0Sti(i)   => write!(f, "fcmove st0, {}", i),
            Fcmov::BeSt0Sti(i)  => write!(f, "fcmovbe st0, {}", i),
            Fcmov::USt0Sti(i)   => write!(f, "fcmovu st0, {}", i),
            Fcmov::NbSt0Sti(i)  => write!(f, "fcmovnb st0, {}", i),
            Fcmov::NeSt0Sti(i)  => write!(f, "fcmovne st0, {}", i),
            Fcmov::NbeSt0Sti(i) => write!(f, "fcmovnbe st0, {}", i),
            Fcmov::NuSt0Sti(i)  => write!(f, "fcmovnu st0, {}", i),
        }
    }
}

impl Display for Fcom {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Fcom::M32fp(m) => write!(f, "fcom dword {}", m),
            Fcom::M64fp(m) => write!(f, "fcom qword {}", m),
            Fcom::Sti(i)   => write!(f, "fcom {}", i),
        }
    }
}

impl Display for Fcomp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Fcomp::M32fp(m) => write!(f, "fcomp dword {}", m),
            Fcomp::M64fp(m) => write!(f, "fcomp qword {}", m),
            Fcomp::Sti(i)   => write!(f, "fcomp {}", i),
        }
    }
}

impl_display_str!("fcompp", Fcompp);
impl_display_unary!("fcomi st0,", Fcomi { St0Sti });
impl_display_unary!("fcomip st0,", Fcomip { St0Sti });
impl_display_unary!("fucomi st0,", Fucomi { St0Sti });
impl_display_unary!("fucomip st0,", Fucomip { St0Sti });
impl_display_str!("fcos", Fcos);
impl_display_str!("fdecstp", Fdecstp);
impl_display_farith!("fdiv", "fdivp", "fidiv", Fdiv, Fdivp, Fidiv);
impl_display_farith!("fdivr", "fdivrp", "fidivr", Fdivr, Fdivrp, Fidivr);
impl_display_unary!("ffree", Ffree { Sti });

impl Display for Ficom {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Ficom::M16int(m) => write!(f, "ficom word {}", m),
            Ficom::M32int(m) => write!(f, "ficom dword {}", m),
        }
    }
}

impl Display for Ficomp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Ficomp::M16int(m) => write!(f, "ficomp word {}", m),
            Ficomp::M32int(m) => write!(f, "ficomp dword {}", m),
        }
    }
}

impl Display for Fild {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Fild::M16int(m) => write!(f, "fild word {}", m),
            Fild::M32int(m) => write!(f, "fild dword {}", m),
            Fild::M64int(m) => write!(f, "fild qword {}", m),
        }
    }
}

impl_display_str!("fincstp", Fincstp);
impl_display_str!("finit", Finit);
impl_display_str!("fninit", Fninit);

impl Display for Fist {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Fist::M16int(m) => write!(f, "fist word {}", m),
            Fist::M32int(m) => write!(f, "fist dword {}", m),
        }
    }
}

impl Display for Fistp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Fistp::M16int(m) => write!(f, "fistp word {}", m),
            Fistp::M32int(m) => write!(f, "fistp dword {}", m),
            Fistp::M64int(m) => write!(f, "fistp qword {}", m),
        }
    }
}

impl Display for Fisttp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Fisttp::M16int(m) => write!(f, "fisttp word {}", m),
            Fisttp::M32int(m) => write!(f, "fisttp dword {}", m),
            Fisttp::M64int(m) => write!(f, "fisttp qword {}", m),
        }
    }
}

impl Display for Fld {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Fld::M32fp(m) => write!(f, "fld dword {}", m),
            Fld::M64fp(m) => write!(f, "fld qword {}", m),
            Fld::M80fp(m) => write!(f, "fld tword {}", m),
            Fld::Sti(i)   => write!(f, "fld {}", i),
        }
    }
}

impl_display_str!("fld1", Fld1);
impl_display_str!("fldl2t", Fldl2t);
impl_display_str!("fldl2e", Fldl2e);
impl_display_str!("fldpi", Fldpi);
impl_display_str!("fldlg2", Fldlg2);
impl_display_str!("fldln2", Fldln2);
impl_display_str!("fldz", Fldz);
impl_display_unary!("fldcw", Fldcw { M2byte });
impl_display_unary!("fldenv", Fldenv { M28byte });
impl_display_farith!("fmul", "fmulp", "fimul", Fmul, Fmulp, Fimul);
impl_display_str!("fnop", Fnop);
impl_display_str!("fpatan", Fpatan);
impl_display_str!("fprem", Fprem);
impl_display_str!("fprem1", Fprem1);
impl_display_str!("fptan", Fptan);
impl_display_str!("frndint", Frndint);
impl_display_unary!("frstor", Frstor { M108byte });
impl_display_unary!("fsave", Fsave { M108byte });
impl_display_unary!("fnsave", Fnsave { M108byte });
impl_display_str!("fscale", Fscale);
impl_display_str!("fsin", Fsin);
impl_display_str!("fsincos", Fsincos);
impl_display_str!("fsqrt", Fsqrt);

impl Display for Fst {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Fst::M32fp(m) => write!(f, "fst dword {}", m),
            Fst::M64fp(m) => write!(f, "fst qword {}", m),
            Fst::Sti(i)   => write!(f, "fst {}", i),
        }
    }
}

impl Display for Fstp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Fstp::M32fp(m) => write!(f, "fstp dword {}", m),
            Fstp::M64fp(m) => write!(f, "fstp qword {}", m),
            Fstp::M80fp(m) => write!(f, "fstp tword {}", m),
            Fstp::Sti(i)   => write!(f, "fstp {}", i),
        }
    }
}

impl_display_unary!("fstcw", Fstcw { M2byte });
impl_display_unary!("fnstcw", Fnstcw { M2byte });
impl_display_unary!("fstenv", Fstenv { M28byte });
impl_display_unary!("fnstenv", Fnstenv { M28byte });

impl Display for Fstsw {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Fstsw::M2byte(m) => write!(f, "fstsw {}", m),
            Fstsw::Ax        => write!(f, "fstsw ax"),
        }
    }
}

impl Display for Fnstsw {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Fnstsw::M2byte(m) => write!(f, "fnstsw {}", m),
            Fnstsw::Ax        => write!(f, "fnstsw ax"),
        }
    }
}

impl_display_farith!("fsub", "fsubp", "fisub", Fsub, Fsubp, Fisub);
impl_display_farith!("fsubr", "fsubrp", "fisubr", Fsubr, Fsubrp, Fisubr);
impl_display_str!("ftst", Ftst);
impl_display_unary!("fucom", Fucom { Sti });
impl_display_unary!("fucomp", Fucomp { Sti });
impl_display_str!("fucompp", Fucompp);
impl_display_str!("fxam", Fxam);
impl_display_unary!("fxch", Fxch { Sti });
impl_display_unary!("fxrstor", Fxrstor { M512byte });
impl_display_unary!("fxrstor64", Fxrstor64 { M512byte });
impl_display_unary!("fxsave", Fxsave { M512byte });
impl_display_unary!("fxsave64", Fxsave64 { M512byte });
impl_display_str!("fxtract", Fxtract);
impl_display_str!("fyl2x", Fyl2x);
impl_display_str!("fyl2xp1", Fyl2xp1);
impl_display_str!("hlt", Hlt);
impl_display_unary!("idiv", Idiv { Rm8l, Rm8, Rm16, Rm32, Rm64 });

impl Display for Imul {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Imul::Rm8l(rm) => write!(f, "imul {}", rm),
            Imul::Rm8(rm)  => write!(f, "imul {}", rm),
            Imul::Rm16(rm) => write!(f, "imul {}", rm),
            Imul::Rm32(rm) => write!(f, "imul {}", rm),
            Imul::Rm64(rm) => write!(f, "imul {}", rm),

            Imul::R16Rm16(r, rm) => write!(f, "imul {}, {}", r, rm),
            Imul::R32Rm32(r, rm) => write!(f, "imul {}, {}", r, rm),
            Imul::R64Rm64(r, rm) => write!(f, "imul {}, {}", r, rm),

            Imul::R16Rm16Imm8(r, rm, imm) => write!(f, "imul {}, {}, {}", r, rm, imm),
            Imul::R32Rm32Imm8(r, rm, imm) => write!(f, "imul {}, {}, {}", r, rm, imm),
            Imul::R64Rm64Imm8(r, rm, imm) => write!(f, "imul {}, {}, {}", r, rm, imm),

            Imul::R16Rm16Imm16(r, rm, imm) => write!(f, "imul {}, {}, {}", r, rm, imm),
            Imul::R32Rm32Imm32(r, rm, imm) => write!(f, "imul {}, {}, {}", r, rm, imm),
            Imul::R64Rm64Imm32(r, rm, imm) => write!(f, "imul {}, {}, {}", r, rm, imm),
        }
    }
}

impl Display for In {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            In::AlImm8(imm)  => write!(f, "in al, {}", imm),
            In::AxImm8(imm)  => write!(f, "in ax, {}", imm),
            In::EaxImm8(imm) => write!(f, "in eax, {}", imm),

            In::AlDx  => write!(f, "in al, dx"),
            In::AxDx  => write!(f, "in ax, dx"),
            In::EaxDx => write!(f, "in eax, dx"),
        }
    }
}

impl_display_unary!("inc", Inc { Rm8l, Rm8, Rm16, Rm32, Rm64 });
impl_display_reg!(
    Ins {
        B => "insb",
        W => "insw",
        D => "insd",
    }
);
impl_display_str!("int 3", Int3);
impl_display_unary!("int", Int { Imm8 });
impl_display_str!("into", Into);
impl_display_str!("invd", Invd);
impl_display_unary!("invlpg", Invlpg { M });
impl_display_binary!("invpcid", Invpcid { R64M128 });
impl_display_reg!(
    Iret {
        D => "iretd",
        Q => "iretq",
    }
);

impl Display for J {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            J::CcRel8(cc, rel)  => write!(f, "j{} short {}", cc, rel),
            J::RcxzRel8(rel)    => write!(f, "jrcxz short {}", rel),
            J::CcRel32(cc, rel) => write!(f, "j{} near {}", cc, rel),
        }
    }
}

impl Display for Jmp {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Jmp::Rel8(rel)  => write!(f, "jmp short {}", rel),
            Jmp::Rel32(rel) => write!(f, "jmp rel {}", rel),
            Jmp::Rm64(rm)   => write!(f, "jmp near {}", rm),
            Jmp::M16x16(m)  => write!(f, "jmp far word {}", m),
            Jmp::M16x32(m)  => write!(f, "jmp far dword {}", m),
            Jmp::M16x64(m)  => write!(f, "jmp far qword {}", m),
        }
    }
}

impl_display_binary!("lss", Lss { R16M16x16, R32M16x32, R64M16x64 });
impl_display_binary!("lfs", Lfs { R16M16x16, R32M16x32, R64M16x64 });
impl_display_binary!("lgs", Lgs { R16M16x16, R32M16x32, R64M16x64 });
impl_display_binary!("lea", Lea { R16M, R32M, R64M });
impl_display_str!("leave", Leave);
impl_display_str!("lfence", Lfence);
impl_display_unary!("lgdt", Lgdt { M16x64 });
impl_display_unary!("lidt", Lidt { M16x64 });
impl_display_unary!("lldt", Lldt { Rm16 });
impl_display_unary!("lmsw", Lmsw { Rm16 });
impl_display_reg!(
    Lods {
        B => "lodsb",
        W => "lodsw",
        D => "lodsd",
        Q => "lodsq",
    }
);

impl Display for Loop {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Loop::Rel8(rel)   => write!(f, "loop {}", rel),
            Loop::ERel8(rel)  => write!(f, "loope {}", rel),
            Loop::NeRel8(rel) => write!(f, "loopne {}", rel),
        }
    }
}

impl_display_unary!("ltr", Ltr { Rm16 });
impl_display_binary!("lzcnt", Lzcnt { R16Rm16, R32Rm32, R64Rm64 });
impl_display_str!("mfence", Mfence);
impl_display_str!("monitor", Monitor);

impl Display for Mov {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Mov::Rm8lR8l(rm, r) => write!(f, "mov {}, {}", rm, r),
            Mov::Rm8R8(rm, r)   => write!(f, "mov {}, {}", rm, r),
            Mov::Rm16R16(rm, r) => write!(f, "mov {}, {}", rm, r),
            Mov::Rm32R32(rm, r) => write!(f, "mov {}, {}", rm, r),
            Mov::Rm64R64(rm, r) => write!(f, "mov {}, {}", rm, r),

            Mov::R8lRm8l(r, rm) => write!(f, "mov {}, {}", r, rm),
            Mov::R8Rm8(r, rm)   => write!(f, "mov {}, {}", r, rm),
            Mov::R16Rm16(r, rm) => write!(f, "mov {}, {}", r, rm),
            Mov::R32Rm32(r, rm) => write!(f, "mov {}, {}", r, rm),
            Mov::R64Rm64(r, rm) => write!(f, "mov {}, {}", r, rm),

            Mov::Rm16Sreg(rm, sreg) => write!(f, "mov {}, {}", rm, sreg),
            Mov::Rm64Sreg(rm, sreg) => write!(f, "mov {}, {}", rm, sreg),

            Mov::SregRm16(sreg, rm) => write!(f, "mov {}, {}", sreg, rm),
            Mov::SregRm64(sreg, rm) => write!(f, "mov {}, {}", sreg, rm),

            Mov::AlMoffs8(moffs)   => write!(f, "mov al, {}", moffs),
            Mov::AxMoffs16(moffs)  => write!(f, "mov ax, {}", moffs),
            Mov::EaxMoffs32(moffs) => write!(f, "mov eax, {}", moffs),
            Mov::RaxMoffs64(moffs) => write!(f, "mov rax, {}", moffs),

            Mov::Moffs8Al(moffs)   => write!(f, "mov {}, al", moffs),
            Mov::Moffs16Ax(moffs)  => write!(f, "mov {}, ax", moffs),
            Mov::Moffs32Eax(moffs) => write!(f, "mov {}, eax", moffs),
            Mov::Moffs64Rax(moffs) => write!(f, "mov {}, rax", moffs),

            Mov::R8lImm8(r, imm)  => write!(f, "mov {}, {}", r, imm),
            Mov::R8Imm8(r, imm)   => write!(f, "mov {}, {}", r, imm),
            Mov::R16Imm16(r, imm) => write!(f, "mov {}, {}", r, imm),
            Mov::R32Imm32(r, imm) => write!(f, "mov {}, {}", r, imm),
            Mov::R64Imm64(r, imm) => write!(f, "mov {}, {}", r, imm),

            Mov::Rm8lImm8(rm, imm)  => write!(f, "mov {}, {}", rm, imm),
            Mov::Rm8Imm8(rm, imm)   => write!(f, "mov {}, {}", rm, imm),
            Mov::Rm16Imm16(rm, imm) => write!(f, "mov {}, {}", rm, imm),
            Mov::Rm32Imm32(rm, imm) => write!(f, "mov {}, {}", rm, imm),
            Mov::Rm64Imm32(rm, imm) => write!(f, "mov {}, {}", rm, imm),

            Mov::R64Cr(r, cr) => write!(f, "mov {}, {}", r, cr),
            Mov::CrR64(cr, r) => write!(f, "mov {}, {}", cr, r),

            Mov::R64Dr(r, dr) => write!(f, "mov {}, {}", r, dr),
            Mov::DrR64(dr, r) => write!(f, "mov {}, {}", dr, r),
        }
    }
}

impl_display_binary!("movbe", Movbe { R16M16, R32M32, R64M64, M16R16, M32R32, M64R64 });
