use code::{Encode, Instruction};
use mnemonic::instruction::*;

impl Encode for Adc {
    fn encode(&self) -> Instruction {
        use self::Adc::*;
        match *self {
            AlImm8(imm)   => Instruction::opcode1(0x14).imm8(imm),
            AxImm16(imm)  => Instruction::opcode1(0x15).imm16(imm).oper16(),
            EaxImm32(imm) => Instruction::opcode1(0x15).imm32(imm),
            RaxImm32(imm) => Instruction::opcode1(0x15).imm32(imm).rex_w(),

            Rm8LImm8(rm, imm)  => Instruction::opcode1(0x80).reg(2).rm8l(rm).imm8(imm),
            Rm8Imm8(rm, imm)   => Instruction::opcode1(0x80).reg(2).rm8(rm).imm8(imm),
            Rm16Imm16(rm, imm) => Instruction::opcode1(0x81).reg(2).rm16(rm).imm16(imm).oper16(),
            Rm32Imm32(rm, imm) => Instruction::opcode1(0x81).reg(2).rm32(rm).imm32(imm),
            Rm64Imm32(rm, imm) => Instruction::opcode1(0x81).reg(2).rm64(rm).imm32(imm).rex_w(),

            Rm16Imm8(rm, imm) => Instruction::opcode1(0x83).reg(2).rm16(rm).imm8(imm).oper16(),
            Rm32Imm8(rm, imm) => Instruction::opcode1(0x83).reg(2).rm32(rm).imm8(imm),
            Rm64Imm8(rm, imm) => Instruction::opcode1(0x83).reg(2).rm64(rm).imm8(imm).rex_w(),

            Rm8LR8L(rm, r) => Instruction::opcode1(0x10).rm8l(rm).reg(r),
            Rm8R8(rm, r)   => Instruction::opcode1(0x10).rm8(rm).reg(r),
            Rm16R16(rm, r) => Instruction::opcode1(0x11).rm16(rm).reg(r).oper16(),
            Rm32R32(rm, r) => Instruction::opcode1(0x11).rm32(rm).reg(r),
            Rm64R64(rm, r) => Instruction::opcode1(0x11).rm64(rm).reg(r).rex_w(),

            R8LRm8L(r, rm) => Instruction::opcode1(0x12).reg(r).rm8l(rm),
            R8Rm8(r, rm)   => Instruction::opcode1(0x12).reg(r).rm8(rm),
            R16Rm16(r, rm) => Instruction::opcode1(0x13).reg(r).rm16(rm).oper16(),
            R32Rm32(r, rm) => Instruction::opcode1(0x13).reg(r).rm32(rm),
            R64Rm64(r, rm) => Instruction::opcode1(0x13).reg(r).rm64(rm).rex_w(),
        }
    }
}

impl Encode for Adcx {
    fn encode(&self) -> Instruction {
        use self::Adcx::*;
        match *self {
            R32Rm32(r, rm) => Instruction::opcode3(0x38, 0xf6).oper16().reg(r).rm32(rm),
            R64Rm64(r, rm) => Instruction::opcode3(0x38, 0xf6).oper16().reg(r).rm64(rm).rex_w(),
        }
    }
}

impl Encode for Add {
    fn encode(&self) -> Instruction {
        use self::Add::*;
        match *self {
            AlImm8(imm)   => Instruction::opcode1(0x04).imm8(imm),
            AxImm16(imm)  => Instruction::opcode1(0x05).imm16(imm).oper16(),
            EaxImm32(imm) => Instruction::opcode1(0x05).imm32(imm),
            RaxImm32(imm) => Instruction::opcode1(0x05).imm32(imm).rex_w(),

            Rm8LImm8(rm, imm)  => Instruction::opcode1(0x80).reg(0).rm8l(rm).imm8(imm),
            Rm8Imm8(rm, imm)   => Instruction::opcode1(0x80).reg(0).rm8(rm).imm8(imm),
            Rm16Imm16(rm, imm) => Instruction::opcode1(0x81).reg(0).rm16(rm).imm16(imm).oper16(),
            Rm32Imm32(rm, imm) => Instruction::opcode1(0x81).reg(0).rm32(rm).imm32(imm),
            Rm64Imm32(rm, imm) => Instruction::opcode1(0x81).reg(0).rm64(rm).imm32(imm).rex_w(),

            Rm16Imm8(rm, imm) => Instruction::opcode1(0x83).reg(0).rm16(rm).imm8(imm).oper16(),
            Rm32Imm8(rm, imm) => Instruction::opcode1(0x83).reg(0).rm32(rm).imm8(imm),
            Rm64Imm8(rm, imm) => Instruction::opcode1(0x83).reg(0).rm64(rm).imm8(imm).rex_w(),

            Rm8LR8L(rm, r) => Instruction::opcode1(0x00).rm8l(rm).reg(r),
            Rm8R8(rm, r)   => Instruction::opcode1(0x00).rm8(rm).reg(r),
            Rm16R16(rm, r) => Instruction::opcode1(0x01).rm16(rm).reg(r).oper16(),
            Rm32R32(rm, r) => Instruction::opcode1(0x01).rm32(rm).reg(r),
            Rm64R64(rm, r) => Instruction::opcode1(0x01).rm64(rm).reg(r).rex_w(),

            R8LRm8L(r, rm) => Instruction::opcode1(0x02).reg(r).rm8l(rm),
            R8Rm8(r, rm)   => Instruction::opcode1(0x02).reg(r).rm8(rm),
            R16Rm16(r, rm) => Instruction::opcode1(0x03).reg(r).rm16(rm).oper16(),
            R32Rm32(r, rm) => Instruction::opcode1(0x03).reg(r).rm32(rm),
            R64Rm64(r, rm) => Instruction::opcode1(0x03).reg(r).rm64(rm).rex_w(),
        }
    }
}

impl Encode for Adox {
    fn encode(&self) -> Instruction {
        use self::Adox::*;
        match *self {
            R32Rm32(r, rm) => Instruction::opcode3(0x38, 0xf6).rep().reg(r).rm32(rm),
            R64Rm64(r, rm) => Instruction::opcode3(0x38, 0xf6).rep().reg(r).rm64(rm).rex_w(),
        }
    }
}

impl Encode for And {
    fn encode(&self) -> Instruction {
        use self::And::*;
        match *self {
            AlImm8(imm)   => Instruction::opcode1(0x24).imm8(imm),
            AxImm16(imm)  => Instruction::opcode1(0x25).imm16(imm).oper16(),
            EaxImm32(imm) => Instruction::opcode1(0x25).imm32(imm),
            RaxImm32(imm) => Instruction::opcode1(0x25).imm32(imm).rex_w(),

            Rm8LImm8(rm, imm)  => Instruction::opcode1(0x80).reg(4).rm8l(rm).imm8(imm),
            Rm8Imm8(rm, imm)   => Instruction::opcode1(0x80).reg(4).rm8(rm).imm8(imm),
            Rm16Imm16(rm, imm) => Instruction::opcode1(0x81).reg(4).rm16(rm).imm16(imm).oper16(),
            Rm32Imm32(rm, imm) => Instruction::opcode1(0x81).reg(4).rm32(rm).imm32(imm),
            Rm64Imm32(rm, imm) => Instruction::opcode1(0x81).reg(4).rm64(rm).imm32(imm).rex_w(),

            Rm16Imm8(rm, imm) => Instruction::opcode1(0x83).reg(4).rm16(rm).imm8(imm).oper16(),
            Rm32Imm8(rm, imm) => Instruction::opcode1(0x83).reg(4).rm32(rm).imm8(imm),
            Rm64Imm8(rm, imm) => Instruction::opcode1(0x83).reg(4).rm64(rm).imm8(imm).rex_w(),

            Rm8LR8L(rm, r) => Instruction::opcode1(0x20).rm8l(rm).reg(r),
            Rm8R8(rm, r)   => Instruction::opcode1(0x20).rm8(rm).reg(r),
            Rm16R16(rm, r) => Instruction::opcode1(0x21).rm16(rm).reg(r).oper16(),
            Rm32R32(rm, r) => Instruction::opcode1(0x21).rm32(rm).reg(r),
            Rm64R64(rm, r) => Instruction::opcode1(0x21).rm64(rm).reg(r).rex_w(),

            R8LRm8L(r, rm) => Instruction::opcode1(0x22).reg(r).rm8l(rm),
            R8Rm8(r, rm)   => Instruction::opcode1(0x22).reg(r).rm8(rm),
            R16Rm16(r, rm) => Instruction::opcode1(0x23).reg(r).rm16(rm).oper16(),
            R32Rm32(r, rm) => Instruction::opcode1(0x23).reg(r).rm32(rm),
            R64Rm64(r, rm) => Instruction::opcode1(0x23).reg(r).rm64(rm).rex_w(),
        }
    }
}

impl Encode for Bsf {
    fn encode(&self) -> Instruction {
        use self::Bsf::*;
        match *self {
            R16Rm16(r, rm) => Instruction::opcode2(0xbc).reg(r).rm16(rm).oper16(),
            R32Rm32(r, rm) => Instruction::opcode2(0xbc).reg(r).rm32(rm),
            R64Rm64(r, rm) => Instruction::opcode2(0xbc).reg(r).rm64(rm).rex_w(),
        }
    }
}

impl Encode for Bsr {
    fn encode(&self) -> Instruction {
        use self::Bsr::*;
        match *self {
            R16Rm16(r, rm) => Instruction::opcode2(0xbd).reg(r).rm16(rm).oper16(),
            R32Rm32(r, rm) => Instruction::opcode2(0xbd).reg(r).rm32(rm),
            R64Rm64(r, rm) => Instruction::opcode2(0xbd).reg(r).rm64(rm).rex_w(),
        }
    }
}

impl Encode for Bswap {
    fn encode(&self) -> Instruction {
        use self::Bswap::*;
        match *self {
            R32(r) => Instruction::opcode2(0xc8).plus(r),
            R64(r) => Instruction::opcode2(0xc8).plus(r).rex_w(),
        }
    }
}

impl Encode for Bt {
    fn encode(&self) -> Instruction {
        use self::Bt::*;
        match *self {
            Rm16R16(rm, r) => Instruction::opcode2(0xa3).rm16(rm).reg(r).oper16(),
            Rm32R32(rm, r) => Instruction::opcode2(0xa3).rm32(rm).reg(r),
            Rm64R64(rm, r) => Instruction::opcode2(0xa3).rm64(rm).reg(r).rex_w(),

            Rm16Imm8(rm, imm) => Instruction::opcode2(0xba).reg(4).rm16(rm).imm8(imm).oper16(),
            Rm32Imm8(rm, imm) => Instruction::opcode2(0xba).reg(4).rm32(rm).imm8(imm),
            Rm64Imm8(rm, imm) => Instruction::opcode2(0xba).reg(4).rm64(rm).imm8(imm).rex_w(),
        }
    }
}

impl Encode for Btc {
    fn encode(&self) -> Instruction {
        use self::Btc::*;
        match *self {
            Rm16R16(rm, r) => Instruction::opcode2(0xbb).rm16(rm).reg(r).oper16(),
            Rm32R32(rm, r) => Instruction::opcode2(0xbb).rm32(rm).reg(r),
            Rm64R64(rm, r) => Instruction::opcode2(0xbb).rm64(rm).reg(r).rex_w(),

            Rm16Imm8(rm, imm) => Instruction::opcode2(0xba).reg(7).rm16(rm).imm8(imm).oper16(),
            Rm32Imm8(rm, imm) => Instruction::opcode2(0xba).reg(7).rm32(rm).imm8(imm),
            Rm64Imm8(rm, imm) => Instruction::opcode2(0xba).reg(7).rm64(rm).imm8(imm).rex_w(),
        }
    }
}

impl Encode for Btr {
    fn encode(&self) -> Instruction {
        use self::Btr::*;
        match *self {
            Rm16R16(rm, r) => Instruction::opcode2(0xb3).rm16(rm).reg(r).oper16(),
            Rm32R32(rm, r) => Instruction::opcode2(0xb3).rm32(rm).reg(r),
            Rm64R64(rm, r) => Instruction::opcode2(0xb3).rm64(rm).reg(r).rex_w(),

            Rm16Imm8(rm, imm) => Instruction::opcode2(0xba).reg(6).rm16(rm).imm8(imm).oper16(),
            Rm32Imm8(rm, imm) => Instruction::opcode2(0xba).reg(6).rm32(rm).imm8(imm),
            Rm64Imm8(rm, imm) => Instruction::opcode2(0xba).reg(6).rm64(rm).imm8(imm).rex_w(),
        }
    }
}

impl Encode for Bts {
    fn encode(&self) -> Instruction {
        use self::Bts::*;
        match *self {
            Rm16R16(rm, r) => Instruction::opcode2(0xab).rm16(rm).reg(r).oper16(),
            Rm32R32(rm, r) => Instruction::opcode2(0xab).rm32(rm).reg(r),
            Rm64R64(rm, r) => Instruction::opcode2(0xab).rm64(rm).reg(r).rex_w(),

            Rm16Imm8(rm, imm) => Instruction::opcode2(0xba).reg(5).rm16(rm).imm8(imm).oper16(),
            Rm32Imm8(rm, imm) => Instruction::opcode2(0xba).reg(5).rm32(rm).imm8(imm),
            Rm64Imm8(rm, imm) => Instruction::opcode2(0xba).reg(5).rm64(rm).imm8(imm).rex_w(),
        }
    }
}

impl Encode for Call {
    fn encode(&self) -> Instruction {
        use self::Call::*;
        match *self {
            Rel32(rel) => Instruction::opcode1(0xe8).disp32(rel.0),
            Rm64(rm)   => Instruction::opcode1(0xff).reg(2).rm64(rm),
            M1616(m)   => Instruction::opcode1(0xff).reg(3).mem(m).oper16(),
            M1632(m)   => Instruction::opcode1(0xff).reg(3).mem(m),
            M1664(m)   => Instruction::opcode1(0xff).reg(3).mem(m).rex_w(),
        }
    }
}

impl Encode for Cbw {
    fn encode(&self) -> Instruction {
        Instruction::opcode1(0x98).oper16()
    }
}

impl Encode for Cwde {
    fn encode(&self) -> Instruction {
        Instruction::opcode1(0x98)
    }
}

impl Encode for Cdqe {
    fn encode(&self) -> Instruction {
        Instruction::opcode1(0x98).rex_w()
    }
}

impl Encode for Clac {
    fn encode(&self) -> Instruction {
        Instruction::opcode3(0x01, 0xca)
    }
}

impl Encode for Clc {
    fn encode(&self) -> Instruction {
        Instruction::opcode1(0xf8)
    }
}

impl Encode for Cld {
    fn encode(&self) -> Instruction {
        Instruction::opcode1(0xfc)
    }
}

impl Encode for Clflush {
    fn encode(&self) -> Instruction {
        use self::Clflush::*;
        match *self {
            M8(m) => Instruction::opcode2(0xae).reg(7).mem(m),
        }
    }
}

impl Encode for Clflushopt {
    fn encode(&self) -> Instruction {
        use self::Clflushopt::*;
        match *self {
            M8(m) => Instruction::opcode2(0xae).reg(7).mem(m).oper16(),
        }
    }
}

impl Encode for Cli {
    fn encode(&self) -> Instruction {
        Instruction::opcode1(0xfa)
    }
}

impl Encode for Clts {
    fn encode(&self) -> Instruction {
        Instruction::opcode2(0x06)
    }
}

impl Encode for Cmc {
    fn encode(&self) -> Instruction {
        Instruction::opcode1(0xf5)
    }
}
