//! Immediates, relative jumps and condition codes.

/// 8-bit immediate.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imm8(pub u8);

/// 16-bit immediate.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imm16(pub u16);

/// 32-bit immediate.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imm32(pub u32);

/// 64-bit immediate.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imm64(pub u64);

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
