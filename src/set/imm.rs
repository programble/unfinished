use core::fmt::{Display, Formatter, Error};

/// 8-bit immediate.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imm8(pub i8);

/// 16-bit immediate.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imm16(pub i16);

/// 32-bit immediate.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imm32(pub i32);

/// 64-bit immediate.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imm64(pub i64);

/// 8-bit relative address.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rel8(pub i8);

/// 32-bit relative address.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rel32(pub i32);

macro_rules! declare_cc {
    ($($cc:ident $doc:tt,)+) => {
        /// Condition codes.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Cc {
            $(
                #[doc=$doc]
                $cc,
            )+
        }
    }
}

declare_cc! {
    A   "Above.",
    Ae  "Above or equal.",
    B   "Below.",
    Be  "Below or equal.",
    C   "Carry.",
    E   "Equal.",
    G   "Greater.",
    Ge  "Greater or equal.",
    L   "Less.",
    Le  "Less or equal.",
    Na  "Not above.",
    Nae "Not above or equal.",
    Nb  "Not below.",
    Nbe "Not below or equal.",
    Nc  "Not carry.",
    Ne  "Not equal.",
    Ng  "Not greater.",
    Nge "Not greater or equal.",
    Nl  "Not less.",
    Nle "Not less or equal.",
    No  "Not overflow.",
    Np  "Not parity.",
    Ns  "Not sign.",
    Nz  "Not zero.",
    O   "Overflow.",
    P   "Parity.",
    Pe  "Parity even.",
    Po  "Party odd.",
    S   "Sign.",
    Z   "Zero.",
}

impl Display for Imm8 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#04x}", self.0)
    }
}

impl Display for Imm16 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#06x}", self.0)
    }
}

impl Display for Imm32 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#010x}", self.0)
    }
}

impl Display for Imm64 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#018x}", self.0)
    }
}

impl Display for Rel8 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#04x}", self.0)
    }
}

impl Display for Rel32 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:#010x}", self.0)
    }
}

impl Display for Cc {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Cc::A   => f.write_str("a"),
            Cc::Ae  => f.write_str("ae"),
            Cc::B   => f.write_str("b"),
            Cc::Be  => f.write_str("be"),
            Cc::C   => f.write_str("c"),
            Cc::E   => f.write_str("e"),
            Cc::G   => f.write_str("g"),
            Cc::Ge  => f.write_str("ge"),
            Cc::L   => f.write_str("l"),
            Cc::Le  => f.write_str("le"),
            Cc::Na  => f.write_str("na"),
            Cc::Nae => f.write_str("nae"),
            Cc::Nb  => f.write_str("nb"),
            Cc::Nbe => f.write_str("nbe"),
            Cc::Nc  => f.write_str("nc"),
            Cc::Ne  => f.write_str("ne"),
            Cc::Ng  => f.write_str("ng"),
            Cc::Nge => f.write_str("nge"),
            Cc::Nl  => f.write_str("nl"),
            Cc::Nle => f.write_str("nle"),
            Cc::No  => f.write_str("no"),
            Cc::Np  => f.write_str("np"),
            Cc::Ns  => f.write_str("ns"),
            Cc::Nz  => f.write_str("nz"),
            Cc::O   => f.write_str("o"),
            Cc::P   => f.write_str("p"),
            Cc::Pe  => f.write_str("pe"),
            Cc::Po  => f.write_str("po"),
            Cc::S   => f.write_str("s"),
            Cc::Z   => f.write_str("z"),
        }
    }
}
