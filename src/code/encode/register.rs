use mnemonic::operand::{
    Cc,
    R8L, R8, R16, R32L, R32, R64L, R64,
    Index32L, Index32, Index64L, Index64,
};

pub trait Register: Copy {
    fn index(self) -> u8;

    fn rex_index(self) -> (bool, u8) {
        if 0 != self.index() & 0b1000 {
            (true, self.index() & 0b0111)
        } else {
            (false, self.index())
        }
    }

    fn force_rex(self) -> bool {
        false
    }
}

impl Register for u8 {
    fn index(self) -> u8 {
        self
    }
}

impl Register for Cc {
    fn index(self) -> u8 {
        match self {
            Cc::O   => 0x0,
            Cc::No  => 0x1,
            Cc::B   => 0x2,
            Cc::C   => 0x2,
            Cc::Nae => 0x2,
            Cc::Ae  => 0x3,
            Cc::Nb  => 0x3,
            Cc::Nc  => 0x3,
            Cc::E   => 0x4,
            Cc::Z   => 0x4,
            Cc::Ne  => 0x5,
            Cc::Nz  => 0x5,
            Cc::Be  => 0x6,
            Cc::Na  => 0x6,
            Cc::A   => 0x7,
            Cc::Nbe => 0x7,
            Cc::S   => 0x8,
            Cc::Ns  => 0x9,
            Cc::P   => 0xa,
            Cc::Pe  => 0xa,
            Cc::Np  => 0xb,
            Cc::Po  => 0xb,
            Cc::L   => 0xc,
            Cc::Nge => 0xc,
            Cc::Ge  => 0xd,
            Cc::Nl  => 0xd,
            Cc::Le  => 0xe,
            Cc::Ng  => 0xe,
            Cc::G   => 0xf,
            Cc::Nle => 0xf,
        }
    }
}

impl Register for R8L {
    fn index(self) -> u8 {
        match self {
            R8L::Al => 0,
            R8L::Cl => 1,
            R8L::Dl => 2,
            R8L::Bl => 3,
            R8L::Ah => 4,
            R8L::Ch => 5,
            R8L::Dh => 6,
            R8L::Bh => 7,
        }
    }
}

impl Register for R8 {
    fn index(self) -> u8 {
        match self {
            R8::Al => 0,
            R8::Cl => 1,
            R8::Dl => 2,
            R8::Bl => 3,
            R8::Spl => 4,
            R8::Bpl => 5,
            R8::Sil => 6,
            R8::Dil => 7,
            R8::R8l => 8,
            R8::R9l => 9,
            R8::R10l => 10,
            R8::R11l => 11,
            R8::R12l => 12,
            R8::R13l => 13,
            R8::R14l => 14,
            R8::R15l => 15,
        }
    }

    fn force_rex(self) -> bool {
        match self {
            R8::Spl | R8::Bpl | R8::Sil | R8::Dil => true,
            _ => false,
        }
    }
}

impl Register for R16 {
    fn index(self) -> u8 {
        match self {
            R16::Ax => 0,
            R16::Cx => 1,
            R16::Dx => 2,
            R16::Bx => 3,
            R16::Sp => 4,
            R16::Bp => 5,
            R16::Si => 6,
            R16::Di => 7,
            R16::R8w => 8,
            R16::R9w => 9,
            R16::R10w => 10,
            R16::R11w => 11,
            R16::R12w => 12,
            R16::R13w => 13,
            R16::R14w => 14,
            R16::R15w => 15,
        }
    }
}

impl Register for R32L {
    fn index(self) -> u8 {
        match self {
            R32L::Eax => 0,
            R32L::Ecx => 1,
            R32L::Edx => 2,
            R32L::Ebx => 3,
            R32L::Esp => 4,
            R32L::Ebp => 5,
            R32L::Esi => 6,
            R32L::Edi => 7,
        }
    }
}

impl Register for R32 {
    fn index(self) -> u8 {
        match self {
            R32::Eax => 0,
            R32::Ecx => 1,
            R32::Edx => 2,
            R32::Ebx => 3,
            R32::Esp => 4,
            R32::Ebp => 5,
            R32::Esi => 6,
            R32::Edi => 7,
            R32::R8d => 8,
            R32::R9d => 9,
            R32::R10d => 10,
            R32::R11d => 11,
            R32::R12d => 12,
            R32::R13d => 13,
            R32::R14d => 14,
            R32::R15d => 15,
        }
    }
}

impl Register for R64L {
    fn index(self) -> u8 {
        match self {
            R64L::Rax => 0,
            R64L::Rcx => 1,
            R64L::Rdx => 2,
            R64L::Rbx => 3,
            R64L::Rsp => 4,
            R64L::Rbp => 5,
            R64L::Rsi => 6,
            R64L::Rdi => 7,
        }
    }
}

impl Register for R64 {
    fn index(self) -> u8 {
        match self {
            R64::Rax => 0,
            R64::Rcx => 1,
            R64::Rdx => 2,
            R64::Rbx => 3,
            R64::Rsp => 4,
            R64::Rbp => 5,
            R64::Rsi => 6,
            R64::Rdi => 7,
            R64::R8 => 8,
            R64::R9 => 9,
            R64::R10 => 10,
            R64::R11 => 11,
            R64::R12 => 12,
            R64::R13 => 13,
            R64::R14 => 14,
            R64::R15 => 15,
        }
    }
}

impl Register for Index32L {
    fn index(self) -> u8 {
        match self {
            Index32L::Eax => 0,
            Index32L::Ecx => 1,
            Index32L::Edx => 2,
            Index32L::Ebx => 3,
            Index32L::Ebp => 5,
            Index32L::Esi => 6,
            Index32L::Edi => 7,
        }
    }
}

impl Register for Index32 {
    fn index(self) -> u8 {
        match self {
            Index32::Eax => 0,
            Index32::Ecx => 1,
            Index32::Edx => 2,
            Index32::Ebx => 3,
            Index32::Ebp => 5,
            Index32::Esi => 6,
            Index32::Edi => 7,
            Index32::R8d => 8,
            Index32::R9d => 9,
            Index32::R10d => 10,
            Index32::R11d => 11,
            Index32::R12d => 12,
            Index32::R13d => 13,
            Index32::R14d => 14,
            Index32::R15d => 15,
        }
    }
}

impl Register for Index64L {
    fn index(self) -> u8 {
        match self {
            Index64L::Rax => 0,
            Index64L::Rcx => 1,
            Index64L::Rdx => 2,
            Index64L::Rbx => 3,
            Index64L::Rbp => 5,
            Index64L::Rsi => 6,
            Index64L::Rdi => 7,
        }
    }
}

impl Register for Index64 {
    fn index(self) -> u8 {
        match self {
            Index64::Rax => 0,
            Index64::Rcx => 1,
            Index64::Rdx => 2,
            Index64::Rbx => 3,
            Index64::Rbp => 5,
            Index64::Rsi => 6,
            Index64::Rdi => 7,
            Index64::R8 => 8,
            Index64::R9 => 9,
            Index64::R10 => 10,
            Index64::R11 => 11,
            Index64::R12 => 12,
            Index64::R13 => 13,
            Index64::R14 => 14,
            Index64::R15 => 15,
        }
    }
}
