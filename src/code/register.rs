use mnemonic::operand::{
    Reg8,
    Rex8,
    Reg16,
    Rex16,
    Reg32,
    Rex32,
    Reg64,
    Rex64,
    IndexReg32,
    IndexRex32,
    IndexReg64,
    IndexRex64,
};

pub trait Register: Copy {
    fn index(self) -> u8;

    #[inline]
    fn rex_index(self) -> (bool, u8) {
        if 0 != self.index() & 0b1000 {
            (true, self.index() & 0b0111)
        } else {
            (false, self.index())
        }
    }

    #[inline]
    fn force_rex(self) -> bool {
        false
    }
}

impl Register for u8 {
    #[inline]
    fn index(self) -> u8 {
        self
    }
}

impl Register for Reg8 {
    #[inline]
    fn index(self) -> u8 {
        match self {
            Reg8::Al => 0,
            Reg8::Cl => 1,
            Reg8::Dl => 2,
            Reg8::Bl => 3,
            Reg8::Ah => 4,
            Reg8::Ch => 5,
            Reg8::Dh => 6,
            Reg8::Bh => 7,
        }
    }
}

impl Register for Rex8 {
    #[inline]
    fn index(self) -> u8 {
        match self {
            Rex8::Al => 0,
            Rex8::Cl => 1,
            Rex8::Dl => 2,
            Rex8::Bl => 3,
            Rex8::Spl => 4,
            Rex8::Bpl => 5,
            Rex8::Sil => 6,
            Rex8::Dil => 7,
            Rex8::R8l => 8,
            Rex8::R9l => 9,
            Rex8::R10l => 10,
            Rex8::R11l => 11,
            Rex8::R12l => 12,
            Rex8::R13l => 13,
            Rex8::R14l => 14,
            Rex8::R15l => 15,
        }
    }

    #[inline]
    fn force_rex(self) -> bool {
        match self {
            Rex8::Spl | Rex8::Bpl | Rex8::Sil | Rex8::Dil => true,
            _ => false,
        }
    }
}

impl Register for Reg16 {
    #[inline]
    fn index(self) -> u8 {
        match self {
            Reg16::Ax => 0,
            Reg16::Cx => 1,
            Reg16::Dx => 2,
            Reg16::Bx => 3,
            Reg16::Sp => 4,
            Reg16::Bp => 5,
            Reg16::Si => 6,
            Reg16::Di => 7,
        }
    }
}

impl Register for Rex16 {
    #[inline]
    fn index(self) -> u8 {
        match self {
            Rex16::Ax => 0,
            Rex16::Cx => 1,
            Rex16::Dx => 2,
            Rex16::Bx => 3,
            Rex16::Sp => 4,
            Rex16::Bp => 5,
            Rex16::Si => 6,
            Rex16::Di => 7,
            Rex16::R8w => 8,
            Rex16::R9w => 9,
            Rex16::R10w => 10,
            Rex16::R11w => 11,
            Rex16::R12w => 12,
            Rex16::R13w => 13,
            Rex16::R14w => 14,
            Rex16::R15w => 15,
        }
    }
}

impl Register for Reg32 {
    #[inline]
    fn index(self) -> u8 {
        match self {
            Reg32::Eax => 0,
            Reg32::Ecx => 1,
            Reg32::Edx => 2,
            Reg32::Ebx => 3,
            Reg32::Esp => 4,
            Reg32::Ebp => 5,
            Reg32::Esi => 6,
            Reg32::Edi => 7,
        }
    }
}

impl Register for Rex32 {
    #[inline]
    fn index(self) -> u8 {
        match self {
            Rex32::Eax => 0,
            Rex32::Ecx => 1,
            Rex32::Edx => 2,
            Rex32::Ebx => 3,
            Rex32::Esp => 4,
            Rex32::Ebp => 5,
            Rex32::Esi => 6,
            Rex32::Edi => 7,
            Rex32::R8d => 8,
            Rex32::R9d => 9,
            Rex32::R10d => 10,
            Rex32::R11d => 11,
            Rex32::R12d => 12,
            Rex32::R13d => 13,
            Rex32::R14d => 14,
            Rex32::R15d => 15,
        }
    }
}

impl Register for Reg64 {
    #[inline]
    fn index(self) -> u8 {
        match self {
            Reg64::Rax => 0,
            Reg64::Rcx => 1,
            Reg64::Rdx => 2,
            Reg64::Rbx => 3,
            Reg64::Rsp => 4,
            Reg64::Rbp => 5,
            Reg64::Rsi => 6,
            Reg64::Rdi => 7,
        }
    }
}

impl Register for Rex64 {
    #[inline]
    fn index(self) -> u8 {
        match self {
            Rex64::Rax => 0,
            Rex64::Rcx => 1,
            Rex64::Rdx => 2,
            Rex64::Rbx => 3,
            Rex64::Rsp => 4,
            Rex64::Rbp => 5,
            Rex64::Rsi => 6,
            Rex64::Rdi => 7,
            Rex64::R8 => 8,
            Rex64::R9 => 9,
            Rex64::R10 => 10,
            Rex64::R11 => 11,
            Rex64::R12 => 12,
            Rex64::R13 => 13,
            Rex64::R14 => 14,
            Rex64::R15 => 15,
        }
    }
}

impl Register for IndexReg32 {
    #[inline]
    fn index(self) -> u8 {
        match self {
            IndexReg32::Eax => 0,
            IndexReg32::Ecx => 1,
            IndexReg32::Edx => 2,
            IndexReg32::Ebx => 3,
            IndexReg32::Ebp => 5,
            IndexReg32::Esi => 6,
            IndexReg32::Edi => 7,
        }
    }
}

impl Register for IndexRex32 {
    #[inline]
    fn index(self) -> u8 {
        match self {
            IndexRex32::Eax => 0,
            IndexRex32::Ecx => 1,
            IndexRex32::Edx => 2,
            IndexRex32::Ebx => 3,
            IndexRex32::Ebp => 5,
            IndexRex32::Esi => 6,
            IndexRex32::Edi => 7,
            IndexRex32::R8d => 8,
            IndexRex32::R9d => 9,
            IndexRex32::R10d => 10,
            IndexRex32::R11d => 11,
            IndexRex32::R12d => 12,
            IndexRex32::R13d => 13,
            IndexRex32::R14d => 14,
            IndexRex32::R15d => 15,
        }
    }
}

impl Register for IndexReg64 {
    #[inline]
    fn index(self) -> u8 {
        match self {
            IndexReg64::Rax => 0,
            IndexReg64::Rcx => 1,
            IndexReg64::Rdx => 2,
            IndexReg64::Rbx => 3,
            IndexReg64::Rbp => 5,
            IndexReg64::Rsi => 6,
            IndexReg64::Rdi => 7,
        }
    }
}

impl Register for IndexRex64 {
    #[inline]
    fn index(self) -> u8 {
        match self {
            IndexRex64::Rax => 0,
            IndexRex64::Rcx => 1,
            IndexRex64::Rdx => 2,
            IndexRex64::Rbx => 3,
            IndexRex64::Rbp => 5,
            IndexRex64::Rsi => 6,
            IndexRex64::Rdi => 7,
            IndexRex64::R8 => 8,
            IndexRex64::R9 => 9,
            IndexRex64::R10 => 10,
            IndexRex64::R11 => 11,
            IndexRex64::R12 => 12,
            IndexRex64::R13 => 13,
            IndexRex64::R14 => 14,
            IndexRex64::R15 => 15,
        }
    }
}
