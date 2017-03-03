use code::{Encode, Instruction};
use mnemonic::instruction::*;

fn opcode1(a: u8) -> Instruction {
    Instruction::opcode1(a)
}

fn opcode2(b: u8) -> Instruction {
    Instruction::opcode2(b)
}

fn opcode3(b: u8, c: u8) -> Instruction {
    Instruction::opcode3(b, c)
}

macro_rules! impl_encode_arith {
    ($mnemonic:ident, $opcode:expr, $reg:expr) => {
        impl Encode for $mnemonic {
            fn encode(&self) -> Instruction {
                use self::$mnemonic::*;
                match *self {
                    Rm8LR8L(rm, r) => opcode1($opcode).rm8l(rm).reg(r),
                    Rm8R8(rm, r)   => opcode1($opcode).rm8(rm).reg(r),
                    Rm16R16(rm, r) => opcode1($opcode + 1).rm16(rm).reg(r).oper16(),
                    Rm32R32(rm, r) => opcode1($opcode + 1).rm32(rm).reg(r),
                    Rm64R64(rm, r) => opcode1($opcode + 1).rm64(rm).reg(r).rex_w(),

                    R8LRm8L(r, rm) => opcode1($opcode + 2).reg(r).rm8l(rm),
                    R8Rm8(r, rm)   => opcode1($opcode + 2).reg(r).rm8(rm),
                    R16Rm16(r, rm) => opcode1($opcode + 3).reg(r).rm16(rm).oper16(),
                    R32Rm32(r, rm) => opcode1($opcode + 3).reg(r).rm32(rm),
                    R64Rm64(r, rm) => opcode1($opcode + 3).reg(r).rm64(rm).rex_w(),

                    AlImm8(imm)   => opcode1($opcode + 4).imm8(imm),
                    AxImm16(imm)  => opcode1($opcode + 5).imm16(imm).oper16(),
                    EaxImm32(imm) => opcode1($opcode + 5).imm32(imm),
                    RaxImm32(imm) => opcode1($opcode + 5).imm32(imm).rex_w(),

                    Rm8LImm8(rm, imm)  => opcode1(0x80).reg($reg).rm8l(rm).imm8(imm),
                    Rm8Imm8(rm, imm)   => opcode1(0x80).reg($reg).rm8(rm).imm8(imm),
                    Rm16Imm16(rm, imm) => opcode1(0x81).reg($reg).rm16(rm).imm16(imm).oper16(),
                    Rm32Imm32(rm, imm) => opcode1(0x81).reg($reg).rm32(rm).imm32(imm),
                    Rm64Imm32(rm, imm) => opcode1(0x81).reg($reg).rm64(rm).imm32(imm).rex_w(),

                    Rm16Imm8(rm, imm) => opcode1(0x83).reg($reg).rm16(rm).imm8(imm).oper16(),
                    Rm32Imm8(rm, imm) => opcode1(0x83).reg($reg).rm32(rm).imm8(imm),
                    Rm64Imm8(rm, imm) => opcode1(0x83).reg($reg).rm64(rm).imm8(imm).rex_w(),
                }
            }
        }
    }
}

impl_encode_arith!(Adc, 0x10, 2);

impl Encode for Adcx {
    fn encode(&self) -> Instruction {
        use self::Adcx::*;
        match *self {
            R32Rm32(r, rm) => opcode3(0x38, 0xf6).oper16().reg(r).rm32(rm),
            R64Rm64(r, rm) => opcode3(0x38, 0xf6).oper16().reg(r).rm64(rm).rex_w(),
        }
    }
}

impl_encode_arith!(Add, 0x00, 0);

impl Encode for Adox {
    fn encode(&self) -> Instruction {
        use self::Adox::*;
        match *self {
            R32Rm32(r, rm) => opcode3(0x38, 0xf6).rep().reg(r).rm32(rm),
            R64Rm64(r, rm) => opcode3(0x38, 0xf6).rep().reg(r).rm64(rm).rex_w(),
        }
    }
}

impl_encode_arith!(And, 0x20, 4);

impl Encode for Bsf {
    fn encode(&self) -> Instruction {
        use self::Bsf::*;
        match *self {
            R16Rm16(r, rm) => opcode2(0xbc).reg(r).rm16(rm).oper16(),
            R32Rm32(r, rm) => opcode2(0xbc).reg(r).rm32(rm),
            R64Rm64(r, rm) => opcode2(0xbc).reg(r).rm64(rm).rex_w(),
        }
    }
}

impl Encode for Bsr {
    fn encode(&self) -> Instruction {
        use self::Bsr::*;
        match *self {
            R16Rm16(r, rm) => opcode2(0xbd).reg(r).rm16(rm).oper16(),
            R32Rm32(r, rm) => opcode2(0xbd).reg(r).rm32(rm),
            R64Rm64(r, rm) => opcode2(0xbd).reg(r).rm64(rm).rex_w(),
        }
    }
}

impl Encode for Bswap {
    fn encode(&self) -> Instruction {
        use self::Bswap::*;
        match *self {
            R32(r) => opcode2(0xc8).plus(r),
            R64(r) => opcode2(0xc8).plus(r).rex_w(),
        }
    }
}

impl Encode for Bt {
    fn encode(&self) -> Instruction {
        use self::Bt::*;
        match *self {
            Rm16R16(rm, r) => opcode2(0xa3).rm16(rm).reg(r).oper16(),
            Rm32R32(rm, r) => opcode2(0xa3).rm32(rm).reg(r),
            Rm64R64(rm, r) => opcode2(0xa3).rm64(rm).reg(r).rex_w(),

            Rm16Imm8(rm, imm) => opcode2(0xba).reg(4).rm16(rm).imm8(imm).oper16(),
            Rm32Imm8(rm, imm) => opcode2(0xba).reg(4).rm32(rm).imm8(imm),
            Rm64Imm8(rm, imm) => opcode2(0xba).reg(4).rm64(rm).imm8(imm).rex_w(),
        }
    }
}

impl Encode for Btc {
    fn encode(&self) -> Instruction {
        use self::Btc::*;
        match *self {
            Rm16R16(rm, r) => opcode2(0xbb).rm16(rm).reg(r).oper16(),
            Rm32R32(rm, r) => opcode2(0xbb).rm32(rm).reg(r),
            Rm64R64(rm, r) => opcode2(0xbb).rm64(rm).reg(r).rex_w(),

            Rm16Imm8(rm, imm) => opcode2(0xba).reg(7).rm16(rm).imm8(imm).oper16(),
            Rm32Imm8(rm, imm) => opcode2(0xba).reg(7).rm32(rm).imm8(imm),
            Rm64Imm8(rm, imm) => opcode2(0xba).reg(7).rm64(rm).imm8(imm).rex_w(),
        }
    }
}

impl Encode for Btr {
    fn encode(&self) -> Instruction {
        use self::Btr::*;
        match *self {
            Rm16R16(rm, r) => opcode2(0xb3).rm16(rm).reg(r).oper16(),
            Rm32R32(rm, r) => opcode2(0xb3).rm32(rm).reg(r),
            Rm64R64(rm, r) => opcode2(0xb3).rm64(rm).reg(r).rex_w(),

            Rm16Imm8(rm, imm) => opcode2(0xba).reg(6).rm16(rm).imm8(imm).oper16(),
            Rm32Imm8(rm, imm) => opcode2(0xba).reg(6).rm32(rm).imm8(imm),
            Rm64Imm8(rm, imm) => opcode2(0xba).reg(6).rm64(rm).imm8(imm).rex_w(),
        }
    }
}

impl Encode for Bts {
    fn encode(&self) -> Instruction {
        use self::Bts::*;
        match *self {
            Rm16R16(rm, r) => opcode2(0xab).rm16(rm).reg(r).oper16(),
            Rm32R32(rm, r) => opcode2(0xab).rm32(rm).reg(r),
            Rm64R64(rm, r) => opcode2(0xab).rm64(rm).reg(r).rex_w(),

            Rm16Imm8(rm, imm) => opcode2(0xba).reg(5).rm16(rm).imm8(imm).oper16(),
            Rm32Imm8(rm, imm) => opcode2(0xba).reg(5).rm32(rm).imm8(imm),
            Rm64Imm8(rm, imm) => opcode2(0xba).reg(5).rm64(rm).imm8(imm).rex_w(),
        }
    }
}

impl Encode for Call {
    fn encode(&self) -> Instruction {
        use self::Call::*;
        match *self {
            Rel32(rel) => opcode1(0xe8).disp32(rel.0),
            Rm64(rm)   => opcode1(0xff).reg(2).rm64(rm),
            M1616(m)   => opcode1(0xff).reg(3).mem(m).oper16(),
            M1632(m)   => opcode1(0xff).reg(3).mem(m),
            M1664(m)   => opcode1(0xff).reg(3).mem(m).rex_w(),
        }
    }
}

impl Encode for Cbw {
    fn encode(&self) -> Instruction {
        opcode1(0x98).oper16()
    }
}

impl Encode for Cwde {
    fn encode(&self) -> Instruction {
        opcode1(0x98)
    }
}

impl Encode for Cdqe {
    fn encode(&self) -> Instruction {
        opcode1(0x98).rex_w()
    }
}

impl Encode for Clac {
    fn encode(&self) -> Instruction {
        opcode3(0x01, 0xca)
    }
}

impl Encode for Clc {
    fn encode(&self) -> Instruction {
        opcode1(0xf8)
    }
}

impl Encode for Cld {
    fn encode(&self) -> Instruction {
        opcode1(0xfc)
    }
}

impl Encode for Clflush {
    fn encode(&self) -> Instruction {
        use self::Clflush::*;
        match *self {
            M8(m) => opcode2(0xae).reg(7).mem(m),
        }
    }
}

impl Encode for Clflushopt {
    fn encode(&self) -> Instruction {
        use self::Clflushopt::*;
        match *self {
            M8(m) => opcode2(0xae).reg(7).mem(m).oper16(),
        }
    }
}

impl Encode for Cli {
    fn encode(&self) -> Instruction {
        opcode1(0xfa)
    }
}

impl Encode for Clts {
    fn encode(&self) -> Instruction {
        opcode2(0x06)
    }
}

impl Encode for Cmc {
    fn encode(&self) -> Instruction {
        opcode1(0xf5)
    }
}

impl Encode for Cmov {
    fn encode(&self) -> Instruction {
        use self::Cmov::*;
        match *self {
            CcR16Rm16(cc, r, rm) => opcode2(0x40).cc(cc).reg(r).rm16(rm).oper16(),
            CcR32Rm32(cc, r, rm) => opcode2(0x40).cc(cc).reg(r).rm32(rm),
            CcR64Rm64(cc, r, rm) => opcode2(0x40).cc(cc).reg(r).rm64(rm).rex_w(),
        }
    }
}

impl_encode_arith!(Cmp, 0x38, 7);

impl Encode for Cmps {
    fn encode(&self) -> Instruction {
        use self::Cmps::*;
        match *self {
            B => opcode1(0xa6),
            W => opcode1(0xa7).oper16(),
            D => opcode1(0xa7),
            Q => opcode1(0xa7).rex_w(),
        }
    }
}

impl Encode for Cmpxchg {
    fn encode(&self) -> Instruction {
        use self::Cmpxchg::*;
        match *self {
            Rm8LR8L(rm, r) => opcode2(0xb0).rm8l(rm).reg(r),
            Rm8R8(rm, r)   => opcode2(0xb0).rm8(rm).reg(r),
            Rm16R16(rm, r) => opcode2(0xb1).rm16(rm).reg(r).oper16(),
            Rm32R32(rm, r) => opcode2(0xb1).rm32(rm).reg(r),
            Rm64R64(rm, r) => opcode2(0xb1).rm64(rm).reg(r).rex_w(),
        }
    }
}
