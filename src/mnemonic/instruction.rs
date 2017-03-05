use mnemonic::operand::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Adc {
    AlImm8(Imm8),
    AxImm16(Imm16),
    EaxImm32(Imm32),
    RaxImm32(Imm32),

    Rm8LImm8(Rm8L, Imm8),
    Rm8Imm8(Rm8, Imm8),
    Rm16Imm16(Rm16, Imm16),
    Rm32Imm32(Rm32, Imm32),
    Rm64Imm32(Rm64, Imm32),

    Rm16Imm8(Rm16, Imm8),
    Rm32Imm8(Rm32, Imm8),
    Rm64Imm8(Rm64, Imm8),

    Rm8LR8L(Rm8L, R8L),
    Rm8R8(Rm8, R8),
    Rm16R16(Rm16, R16),
    Rm32R32(Rm32, R32),
    Rm64R64(Rm64, R64),

    R8LRm8L(R8L, Rm8L),
    R8Rm8(R8, Rm8),
    R16Rm16(R16, Rm16),
    R32Rm32(R32, Rm32),
    R64Rm64(R64, Rm64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Adcx {
    R32Rm32(R32, Rm32),
    R64Rm64(R64, Rm64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Add {
    AlImm8(Imm8),
    AxImm16(Imm16),
    EaxImm32(Imm32),
    RaxImm32(Imm32),

    Rm8LImm8(Rm8L, Imm8),
    Rm8Imm8(Rm8, Imm8),
    Rm16Imm16(Rm16, Imm16),
    Rm32Imm32(Rm32, Imm32),
    Rm64Imm32(Rm64, Imm32),

    Rm16Imm8(Rm16, Imm8),
    Rm32Imm8(Rm32, Imm8),
    Rm64Imm8(Rm64, Imm8),

    Rm8LR8L(Rm8L, R8L),
    Rm8R8(Rm8, R8),
    Rm16R16(Rm16, R16),
    Rm32R32(Rm32, R32),
    Rm64R64(Rm64, R64),

    R8LRm8L(R8L, Rm8L),
    R8Rm8(R8, Rm8),
    R16Rm16(R16, Rm16),
    R32Rm32(R32, Rm32),
    R64Rm64(R64, Rm64),
}

// ADDPD
// ADDPS
// ADDSD
// ADDSS
// ADDSUBPD
// ADDSUBPS

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Adox {
    R32Rm32(R32, Rm32),
    R64Rm64(R64, Rm64),
}

// AESDEC
// AESDECLAST
// AESENC
// AESENCLAST
// AESIMC
// AESKEYGENASSIST

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum And {
    AlImm8(Imm8),
    AxImm16(Imm16),
    EaxImm32(Imm32),
    RaxImm32(Imm32),

    Rm8LImm8(Rm8L, Imm8),
    Rm8Imm8(Rm8, Imm8),
    Rm16Imm16(Rm16, Imm16),
    Rm32Imm32(Rm32, Imm32),
    Rm64Imm32(Rm64, Imm32),

    Rm16Imm8(Rm16, Imm8),
    Rm32Imm8(Rm32, Imm8),
    Rm64Imm8(Rm64, Imm8),

    Rm8LR8L(Rm8L, R8L),
    Rm8R8(Rm8, R8),
    Rm16R16(Rm16, R16),
    Rm32R32(Rm32, R32),
    Rm64R64(Rm64, R64),

    R8LRm8L(R8L, Rm8L),
    R8Rm8(R8, Rm8),
    R16Rm16(R16, Rm16),
    R32Rm32(R32, Rm32),
    R64Rm64(R64, Rm64),
}

// ANDN
// ANDPD
// ANDPS
// ANDNPD
// ANDNPS
// BLENDPD
// BEXTR
// BLENDPS
// BLENDVPD
// BLENDVPS
// BLSI
// BLSMSK
// BLSR
// BNDCL
// BNDCU/BNDCN
// BNDLDX
// BNDMK
// BNDMOV
// BNDSTX

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Bsf {
    R16Rm16(R16, Rm16),
    R32Rm32(R32, Rm32),
    R64Rm64(R64, Rm64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Bsr {
    R16Rm16(R16, Rm16),
    R32Rm32(R32, Rm32),
    R64Rm64(R64, Rm64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Bswap {
    R32(R32),
    R64(R64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Bt {
    Rm16R16(Rm16, R16),
    Rm32R32(Rm32, R32),
    Rm64R64(Rm64, R64),

    Rm16Imm8(Rm16, Imm8),
    Rm32Imm8(Rm32, Imm8),
    Rm64Imm8(Rm64, Imm8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Btc {
    Rm16R16(Rm16, R16),
    Rm32R32(Rm32, R32),
    Rm64R64(Rm64, R64),

    Rm16Imm8(Rm16, Imm8),
    Rm32Imm8(Rm32, Imm8),
    Rm64Imm8(Rm64, Imm8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Btr {
    Rm16R16(Rm16, R16),
    Rm32R32(Rm32, R32),
    Rm64R64(Rm64, R64),

    Rm16Imm8(Rm16, Imm8),
    Rm32Imm8(Rm32, Imm8),
    Rm64Imm8(Rm64, Imm8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Bts {
    Rm16R16(Rm16, R16),
    Rm32R32(Rm32, R32),
    Rm64R64(Rm64, R64),

    Rm16Imm8(Rm16, Imm8),
    Rm32Imm8(Rm32, Imm8),
    Rm64Imm8(Rm64, Imm8),
}

// BZHI

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Call {
    Rel32(Rel32),
    Rm64(Rm64),
    M1616(Mem),
    M1632(Mem),
    M1664(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cbw;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cwde;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cdqe;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Clac;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Clc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cld;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Clflush {
    M8(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Clflushopt {
    M8(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cli;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Clts;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cmc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cmov {
    CcR16Rm16(Cc, R16, Rm16),
    CcR32Rm32(Cc, R32, Rm32),
    CcR64Rm64(Cc, R64, Rm64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cmp {
    AlImm8(Imm8),
    AxImm16(Imm16),
    EaxImm32(Imm32),
    RaxImm32(Imm32),

    Rm8LImm8(Rm8L, Imm8),
    Rm8Imm8(Rm8, Imm8),
    Rm16Imm16(Rm16, Imm16),
    Rm32Imm32(Rm32, Imm32),
    Rm64Imm32(Rm64, Imm32),

    Rm16Imm8(Rm16, Imm8),
    Rm32Imm8(Rm32, Imm8),
    Rm64Imm8(Rm64, Imm8),

    Rm8LR8L(Rm8L, R8L),
    Rm8R8(Rm8, R8),
    Rm16R16(Rm16, R16),
    Rm32R32(Rm32, R32),
    Rm64R64(Rm64, R64),

    R8LRm8L(R8L, Rm8L),
    R8Rm8(R8, Rm8),
    R16Rm16(R16, Rm16),
    R32Rm32(R32, Rm32),
    R64Rm64(R64, Rm64),
}

// CMPPD
// CMPPS

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cmps {
    B,
    W,
    D,
    Q,
}

// CMPSD
// CMPSS

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cmpxchg {
    Rm8LR8L(Rm8L, R8L),
    Rm8R8(Rm8, R8),
    Rm16R16(Rm16, R16),
    Rm32R32(Rm32, R32),
    Rm64R64(Rm64, R64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cmpxchg8b {
    M64(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cmpxchg16b {
    M128(Mem),
}

// COMISD
// COMISS

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cpuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Crc32 {
    R32LRm8L(R32L, Rm8L),
    R32Rm8(R32, Rm8),
    R32Rm16(R32, Rm16),
    R32Rm32(R32, Rm32),
    R64Rm8(R64, Rm8),
    R64Rm64(R64, Rm64),
}

// CVTDQ2PD
// CVTDQ2PS
// CVTPD2DQ
// CVTPD2PI
// CVTPD2PS
// CVTPI2PD
// CVTPI2PS
// CVTPS2DQ
// CVTPS2PD
// CVTPS2PI
// CVTSD2SI
// CVTSD2SS
// CVTSI2SD
// CVTSI2SS
// CVTSS2SD
// CVTSS2SI
// CVTTPD2DQ
// CVTTPD2PI
// CVTTPS2DQ
// CVTTPS2PI
// CVTTSD2SI
// CVTTSS2SI

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cwd;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cdq;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cqo;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dec {
    Rm8L(Rm8L),
    Rm8(Rm8),
    Rm16(Rm16),
    Rm32(Rm32),
    Rm64(Rm64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Div {
    Rm8L(Rm8L),
    Rm8(Rm8),
    Rm16(Rm16),
    Rm32(Rm32),
    Rm64(Rm64),
}

// DIVPD
// DIVPS
// DIVSD
// DIVSS
// DPPD
// DPPS
// EMMS
// ENTER
// EXTRACTPS

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct F2xm1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fabs;
