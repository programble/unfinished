use mnemonic::operand::{
    Cc,
    R8l, R8, R16, R32l, R32, R64l, R64, Sti, Cr, Dr,
    Index32l, Index32, Index64l, Index64,
    Sreg,
};

pub trait Register: Copy {
    fn code(self) -> u8;

    fn rex_code(self) -> (bool, u8) {
        if 0 != self.code() & 0b1000 {
            (true, self.code() & 0b0111)
        } else {
            (false, self.code())
        }
    }

    fn force_rex(self) -> bool {
        false
    }
}

impl Register for u8 {
    fn code(self) -> u8 {
        self
    }
}

impl Register for Cc {
    fn code(self) -> u8 {
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

impl Register for R8l {
    fn code(self) -> u8 {
        match self {
            R8l::Al => 0,
            R8l::Cl => 1,
            R8l::Dl => 2,
            R8l::Bl => 3,
            R8l::Ah => 4,
            R8l::Ch => 5,
            R8l::Dh => 6,
            R8l::Bh => 7,
        }
    }
}

impl Register for R8 {
    fn code(self) -> u8 {
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
    fn code(self) -> u8 {
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

impl Register for R32l {
    fn code(self) -> u8 {
        match self {
            R32l::Eax => 0,
            R32l::Ecx => 1,
            R32l::Edx => 2,
            R32l::Ebx => 3,
            R32l::Esp => 4,
            R32l::Ebp => 5,
            R32l::Esi => 6,
            R32l::Edi => 7,
        }
    }
}

impl Register for R32 {
    fn code(self) -> u8 {
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

impl Register for R64l {
    fn code(self) -> u8 {
        match self {
            R64l::Rax => 0,
            R64l::Rcx => 1,
            R64l::Rdx => 2,
            R64l::Rbx => 3,
            R64l::Rsp => 4,
            R64l::Rbp => 5,
            R64l::Rsi => 6,
            R64l::Rdi => 7,
        }
    }
}

impl Register for R64 {
    fn code(self) -> u8 {
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

impl Register for Sti {
    fn code(self) -> u8 {
        match self {
            Sti::St0 => 0,
            Sti::St1 => 1,
            Sti::St2 => 2,
            Sti::St3 => 3,
            Sti::St4 => 4,
            Sti::St5 => 5,
            Sti::St6 => 6,
            Sti::St7 => 7,
        }
    }
}

impl Register for Cr {
    fn code(self) -> u8 {
        match self {
            Cr::Cr0 => 0,
            Cr::Cr2 => 2,
            Cr::Cr3 => 3,
            Cr::Cr4 => 4,
            Cr::Cr8 => 8,
        }
    }
}

impl Register for Dr {
    fn code(self) -> u8 {
        match self {
            Dr::Dr0 => 0,
            Dr::Dr1 => 1,
            Dr::Dr2 => 2,
            Dr::Dr3 => 3,
            Dr::Dr4 => 4,
            Dr::Dr5 => 5,
            Dr::Dr6 => 6,
            Dr::Dr7 => 7,
        }
    }
}

impl Register for Index32l {
    fn code(self) -> u8 {
        match self {
            Index32l::Eax => 0,
            Index32l::Ecx => 1,
            Index32l::Edx => 2,
            Index32l::Ebx => 3,
            Index32l::Ebp => 5,
            Index32l::Esi => 6,
            Index32l::Edi => 7,
        }
    }
}

impl Register for Index32 {
    fn code(self) -> u8 {
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

impl Register for Index64l {
    fn code(self) -> u8 {
        match self {
            Index64l::Rax => 0,
            Index64l::Rcx => 1,
            Index64l::Rdx => 2,
            Index64l::Rbx => 3,
            Index64l::Rbp => 5,
            Index64l::Rsi => 6,
            Index64l::Rdi => 7,
        }
    }
}

impl Register for Index64 {
    fn code(self) -> u8 {
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

impl Register for Sreg {
    fn code(self) -> u8 {
        match self {
            Sreg::Es => 0,
            Sreg::Cs => 1,
            Sreg::Ss => 2,
            Sreg::Ds => 3,
            Sreg::Fs => 4,
            Sreg::Gs => 5,
        }
    }
}
