use mnemonic::operand::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Adc {
    AlImm8(Imm8),
    AxImm16(Imm16),
    EaxImm32(Imm32),
    RaxImm32(Imm32),

    Rm8lImm8(Rm8l, Imm8),
    Rm8Imm8(Rm8, Imm8),
    Rm16Imm16(Rm16, Imm16),
    Rm32Imm32(Rm32, Imm32),
    Rm64Imm32(Rm64, Imm32),

    Rm16Imm8(Rm16, Imm8),
    Rm32Imm8(Rm32, Imm8),
    Rm64Imm8(Rm64, Imm8),

    Rm8lR8l(Rm8l, R8l),
    Rm8R8(Rm8, R8),
    Rm16R16(Rm16, R16),
    Rm32R32(Rm32, R32),
    Rm64R64(Rm64, R64),

    R8lRm8l(R8l, Rm8l),
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

    Rm8lImm8(Rm8l, Imm8),
    Rm8Imm8(Rm8, Imm8),
    Rm16Imm16(Rm16, Imm16),
    Rm32Imm32(Rm32, Imm32),
    Rm64Imm32(Rm64, Imm32),

    Rm16Imm8(Rm16, Imm8),
    Rm32Imm8(Rm32, Imm8),
    Rm64Imm8(Rm64, Imm8),

    Rm8lR8l(Rm8l, R8l),
    Rm8R8(Rm8, R8),
    Rm16R16(Rm16, R16),
    Rm32R32(Rm32, R32),
    Rm64R64(Rm64, R64),

    R8lRm8l(R8l, Rm8l),
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

    Rm8lImm8(Rm8l, Imm8),
    Rm8Imm8(Rm8, Imm8),
    Rm16Imm16(Rm16, Imm16),
    Rm32Imm32(Rm32, Imm32),
    Rm64Imm32(Rm64, Imm32),

    Rm16Imm8(Rm16, Imm8),
    Rm32Imm8(Rm32, Imm8),
    Rm64Imm8(Rm64, Imm8),

    Rm8lR8l(Rm8l, R8l),
    Rm8R8(Rm8, R8),
    Rm16R16(Rm16, R16),
    Rm32R32(Rm32, R32),
    Rm64R64(Rm64, R64),

    R8lRm8l(R8l, Rm8l),
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

    Rm8lImm8(Rm8l, Imm8),
    Rm8Imm8(Rm8, Imm8),
    Rm16Imm16(Rm16, Imm16),
    Rm32Imm32(Rm32, Imm32),
    Rm64Imm32(Rm64, Imm32),

    Rm16Imm8(Rm16, Imm8),
    Rm32Imm8(Rm32, Imm8),
    Rm64Imm8(Rm64, Imm8),

    Rm8lR8l(Rm8l, R8l),
    Rm8R8(Rm8, R8),
    Rm16R16(Rm16, R16),
    Rm32R32(Rm32, R32),
    Rm64R64(Rm64, R64),

    R8lRm8l(R8l, Rm8l),
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
    Rm8lR8l(Rm8l, R8l),
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
    R32lRm8l(R32l, Rm8l),
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
    Rm8l(Rm8l),
    Rm8(Rm8),
    Rm16(Rm16),
    Rm32(Rm32),
    Rm64(Rm64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Div {
    Rm8l(Rm8l),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fadd {
    M32fp(Mem),
    M64fp(Mem),
    St0Sti(Sti),
    StiSt0(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Faddp {
    StiSt0(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fiadd {
    M32int(Mem),
    M16int(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fbld {
    M80dec(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fbstp {
    M80bcd(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fchs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fclex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fnclex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fcmov {
    BSt0Sti(Sti),
    ESt0Sti(Sti),
    BeSt0Sti(Sti),
    USt0Sti(Sti),
    NbSt0Sti(Sti),
    NeSt0Sti(Sti),
    NbeSt0Sti(Sti),
    NuSt0Sti(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fcom {
    M32fp(Mem),
    M64fp(Mem),
    Sti(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fcomp {
    M32fp(Mem),
    M64fp(Mem),
    Sti(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fcompp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fcomi {
    St0Sti(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fcomip {
    St0Sti(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fucomi {
    St0Sti(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fucomip {
    St0Sti(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fcos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fdecstp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fdiv {
    M32fp(Mem),
    M64fp(Mem),
    St0Sti(Sti),
    StiSt0(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fdivp {
    StiSt0(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fidiv {
    M32int(Mem),
    M16int(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fdivr {
    M32fp(Mem),
    M64fp(Mem),
    St0Sti(Sti),
    StiSt0(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fdivrp {
    StiSt0(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fidivr {
    M32int(Mem),
    M16int(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ffree {
    Sti(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ficom {
    M16int(Mem),
    M32int(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ficomp {
    M16int(Mem),
    M32int(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fild {
    M16int(Mem),
    M32int(Mem),
    M64int(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fincstp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Finit;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fninit;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fist {
    M16int(Mem),
    M32int(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fistp {
    M16int(Mem),
    M32int(Mem),
    M64int(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fisttp {
    M16int(Mem),
    M32int(Mem),
    M64int(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fld {
    M32fp(Mem),
    M64fp(Mem),
    M80fp(Mem),
    Sti(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fld1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fldl2t;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fldl2e;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fldpi;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fldlg2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fldln2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fldz;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fldcw {
    M2byte(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fldenv {
    M28byte(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fmul {
    M32fp(Mem),
    M64fp(Mem),
    St0Sti(Sti),
    StiSt0(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fmulp {
    StiSt0(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fimul {
    M32int(Mem),
    M16int(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fnop;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fpatan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fprem;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fprem1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fptan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Frndint;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Frstor {
    M108byte(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fsave {
    M108byte(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fnsave {
    M108byte(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fscale;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fsin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fsincos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fsqrt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fst {
    M32fp(Mem),
    M64fp(Mem),
    Sti(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fstp {
    M32fp(Mem),
    M64fp(Mem),
    M80fp(Mem),
    Sti(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fstcw {
    M2byte(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fnstcw {
    M2byte(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fstenv {
    M28byte(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fnstenv {
    M28byte(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fstsw {
    M2byte(Mem),
    Ax,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fnstsw {
    M2byte(Mem),
    Ax,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fsub {
    M32fp(Mem),
    M64fp(Mem),
    St0Sti(Sti),
    StiSt0(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fsubp {
    StiSt0(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fisub {
    M32int(Mem),
    M16int(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fsubr {
    M32fp(Mem),
    M64fp(Mem),
    St0Sti(Sti),
    StiSt0(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fsubrp {
    StiSt0(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fisubr {
    M32int(Mem),
    M16int(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ftst;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fucom {
    Sti(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fucomp {
    Sti(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fucompp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fxam;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fxch {
    Sti(Sti),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fxrstor {
    M512byte(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fxrstor64 {
    M512byte(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fxsave {
    M512byte(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fxsave64 {
    M512byte(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fxtract;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fyl2x;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fyl2xp1;

// HADDPD
// HADDPS

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hlt;

// HSUBPD
// HSUBPS

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Idiv {
    Rm8l(Rm8l),
    Rm8(Rm8),
    Rm16(Rm16),
    Rm32(Rm32),
    Rm64(Rm64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Imul {
    Rm8l(Rm8l),
    Rm8(Rm8),
    Rm16(Rm16),
    Rm32(Rm32),
    Rm64(Rm64),

    R16Rm16(R16, Rm16),
    R32Rm32(R32, Rm32),
    R64Rm64(R64, Rm64),

    R16Rm16Imm8(R16, Rm16, Imm8),
    R32Rm32Imm8(R32, Rm32, Imm8),
    R64Rm64Imm8(R64, Rm64, Imm8),

    R16Rm16Imm16(R16, Rm16, Imm16),
    R32Rm32Imm32(R32, Rm32, Imm32),
    R64Rm64Imm32(R64, Rm64, Imm32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum In {
    AlImm8(Imm8),
    AxImm8(Imm8),
    EaxImm8(Imm8),

    AlDx,
    AxDx,
    EaxDx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Inc {
    Rm8l(Rm8l),
    Rm8(Rm8),
    Rm16(Rm16),
    Rm32(Rm32),
    Rm64(Rm64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ins {
    B,
    W,
    D,
}

// INSERTPS

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Int3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Int {
    Imm8(Imm8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Into;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Invd;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Invlpg {
    M(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Invpcid {
    R64M128(R64, Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Iret {
    D,
    Q,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum J {
    CcRel8(Cc, Rel8),
    RcxzRel8(Rel8),
    CcRel32(Cc, Rel32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Jmp {
    Rel8(Rel8),
    Rel32(Rel32),
    Rm64(Rm64),
    M1616(Mem),
    M1632(Mem),
    M1664(Mem),
}

// KADD
// KAND
// KANDN
// KMOV
// KNOT
// KOR
// KORTEST
// KSHIFTL
// KSHIFTR
// KTEST
// KUNPCK
// KXNOR
// KXOR
// LAR
// LDDQU
// LDMXCSR

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lss {
    R16M1616(R16, Mem),
    R32M1632(R32, Mem),
    R64M1664(R64, Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lfs {
    R16M1616(R16, Mem),
    R32M1632(R32, Mem),
    R64M1664(R64, Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lgs {
    R16M1616(R16, Mem),
    R32M1632(R32, Mem),
    R64M1664(R64, Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lea {
    R16M(R16, Mem),
    R32M(R32, Mem),
    R64M(R64, Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Leave;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lfence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lgdt {
    M1664(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lidt {
    M1664(Mem),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lldt {
    Rm16(Rm16),
}
