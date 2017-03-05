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
                    $ty::Rm8LImm8(rm, imm)  => write!(f, concat!($str, " {}, {}"), rm, imm),
                    $ty::Rm8Imm8(rm, imm)   => write!(f, concat!($str, " {}, {}"), rm, imm),
                    $ty::Rm16Imm16(rm, imm) => write!(f, concat!($str, " {}, {}"), rm, imm),
                    $ty::Rm32Imm32(rm, imm) => write!(f, concat!($str, " {}, {}"), rm, imm),
                    $ty::Rm64Imm32(rm, imm) => write!(f, concat!($str, " {}, {}"), rm, imm),
                    $ty::Rm16Imm8(rm, imm)  => write!(f, concat!($str, " {}, {}"), rm, imm),
                    $ty::Rm32Imm8(rm, imm)  => write!(f, concat!($str, " {}, {}"), rm, imm),
                    $ty::Rm64Imm8(rm, imm)  => write!(f, concat!($str, " {}, {}"), rm, imm),
                    $ty::Rm8LR8L(rm, r)     => write!(f, concat!($str, " {}, {}"), rm, r),
                    $ty::Rm8R8(rm, r)       => write!(f, concat!($str, " {}, {}"), rm, r),
                    $ty::Rm16R16(rm, r)     => write!(f, concat!($str, " {}, {}"), rm, r),
                    $ty::Rm32R32(rm, r)     => write!(f, concat!($str, " {}, {}"), rm, r),
                    $ty::Rm64R64(rm, r)     => write!(f, concat!($str, " {}, {}"), rm, r),
                    $ty::R8LRm8L(r, rm)     => write!(f, concat!($str, " {}, {}"), r, rm),
                    $ty::R8Rm8(r, rm)       => write!(f, concat!($str, " {}, {}"), r, rm),
                    $ty::R16Rm16(r, rm)     => write!(f, concat!($str, " {}, {}"), r, rm),
                    $ty::R32Rm32(r, rm)     => write!(f, concat!($str, " {}, {}"), r, rm),
                    $ty::R64Rm64(r, rm)     => write!(f, concat!($str, " {}, {}"), r, rm),
                }
            }
        }
    }
}

impl_display_int!(Imm8, "{:#04x}");
impl_display_int!(Imm16, "{:#06x}");
impl_display_int!(Imm32, "{:#010x}");
impl_display_int!(Imm64, "{:#018x}");
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
    R8L {
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
    R32L {
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
    R64L {
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
    Index32L {
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
    Index64L {
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
            Offset::Index(i, s)               => write!(f, "{} * {}", i, s),
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

impl_display_rm!(Rm8L, R8L, "byte", M8L);
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
            Call::M1616(m)   => write!(f, "call far {}", m),
            Call::M1632(m)   => write!(f, "call far {}", m),
            Call::M1664(m)   => write!(f, "call far {}", m),
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
impl_display_binary!("cmpxchg", Cmpxchg { Rm8LR8L, Rm8R8, Rm16R16, Rm32R32, Rm64R64 });
impl_display_str!("cpuid", Cpuid);
impl_display_binary!("crc32", Crc32 { R32LRm8L, R32Rm8, R32Rm16, R32Rm32, R64Rm8, R64Rm64 });
