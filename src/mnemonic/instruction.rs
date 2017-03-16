use mnemonic::operand::*;

macro_rules! declare {
    ($($ty:ident $tt:tt,)+) => {
        $(declare!($ty $tt);)+
    };

    ($ty:ident()) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $ty;
    };

    ($ty:ident { $($tt:tt)+ }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $ty {
            $($tt)+
        }
    };
}

declare! {
    Adc {
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
    },

    Adcx {
        R32Rm32(R32, Rm32),
        R64Rm64(R64, Rm64),
    },

    Add {
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
    },

    // ADDPD
    // ADDPS
    // ADDSD
    // ADDSS
    // ADDSUBPD
    // ADDSUBPS

    Adox {
        R32Rm32(R32, Rm32),
        R64Rm64(R64, Rm64),
    },

    // AESDEC
    // AESDECLAST
    // AESENC
    // AESENCLAST
    // AESIMC
    // AESKEYGENASSIST

    And {
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
    },

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

    Bsf {
        R16Rm16(R16, Rm16),
        R32Rm32(R32, Rm32),
        R64Rm64(R64, Rm64),
    },

    Bsr {
        R16Rm16(R16, Rm16),
        R32Rm32(R32, Rm32),
        R64Rm64(R64, Rm64),
    },

    Bswap {
        R32(R32),
        R64(R64),
    },

    Bt {
        Rm16R16(Rm16, R16),
        Rm32R32(Rm32, R32),
        Rm64R64(Rm64, R64),

        Rm16Imm8(Rm16, Imm8),
        Rm32Imm8(Rm32, Imm8),
        Rm64Imm8(Rm64, Imm8),
    },

    Btc {
        Rm16R16(Rm16, R16),
        Rm32R32(Rm32, R32),
        Rm64R64(Rm64, R64),

        Rm16Imm8(Rm16, Imm8),
        Rm32Imm8(Rm32, Imm8),
        Rm64Imm8(Rm64, Imm8),
    },

    Btr {
        Rm16R16(Rm16, R16),
        Rm32R32(Rm32, R32),
        Rm64R64(Rm64, R64),

        Rm16Imm8(Rm16, Imm8),
        Rm32Imm8(Rm32, Imm8),
        Rm64Imm8(Rm64, Imm8),
    },

    Bts {
        Rm16R16(Rm16, R16),
        Rm32R32(Rm32, R32),
        Rm64R64(Rm64, R64),

        Rm16Imm8(Rm16, Imm8),
        Rm32Imm8(Rm32, Imm8),
        Rm64Imm8(Rm64, Imm8),
    },

    // BZHI

    Call {
        Rel32(Rel32),
        Rm64(Rm64),
        M16x16(Mem),
        M16x32(Mem),
        M16x64(Mem),
    },

    Cbw(),
    Cwde(),
    Cdqe(),

    Clac(),

    Clc(),

    Cld(),

    Clflush {
        M8(Mem),
    },

    Clflushopt {
        M8(Mem),
    },

    Cli(),

    Clts(),

    Cmc(),

    Cmov {
        CcR16Rm16(Cc, R16, Rm16),
        CcR32Rm32(Cc, R32, Rm32),
        CcR64Rm64(Cc, R64, Rm64),
    },

    Cmp {
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
    },

    // CMPPD
    // CMPPS

    Cmps {
        B,
        W,
        D,
        Q,
    },

    // CMPSD
    // CMPSS

    Cmpxchg {
        Rm8lR8l(Rm8l, R8l),
        Rm8R8(Rm8, R8),
        Rm16R16(Rm16, R16),
        Rm32R32(Rm32, R32),
        Rm64R64(Rm64, R64),
    },

    Cmpxchg8b {
        M64(Mem),
    },
    Cmpxchg16b {
        M128(Mem),
    },

    // COMISD
    // COMISS

    Cpuid(),

    Crc32 {
        R32lRm8l(R32l, Rm8l),
        R32Rm8(R32, Rm8),
        R32Rm16(R32, Rm16),
        R32Rm32(R32, Rm32),
        R64Rm8(R64, Rm8),
        R64Rm64(R64, Rm64),
    },

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

    Cwd(),
    Cdq(),
    Cqo(),

    Dec {
        Rm8l(Rm8l),
        Rm8(Rm8),
        Rm16(Rm16),
        Rm32(Rm32),
        Rm64(Rm64),
    },

    Div {
        Rm8l(Rm8l),
        Rm8(Rm8),
        Rm16(Rm16),
        Rm32(Rm32),
        Rm64(Rm64),
    },

    // DIVPD
    // DIVPS
    // DIVSD
    // DIVSS
    // DPPD
    // DPPS
    // EMMS
    // ENTER
    // EXTRACTPS

    F2xm1(),

    Fabs(),

    Fadd {
        M32fp(Mem),
        M64fp(Mem),
        St0Sti(Sti),
        StiSt0(Sti),
    },
    Faddp {
        StiSt0(Sti),
    },
    Fiadd {
        M32int(Mem),
        M16int(Mem),
    },

    Fbld {
        M80dec(Mem),
    },

    Fbstp {
        M80bcd(Mem),
    },

    Fchs(),

    Fclex(),

    Fnclex(),

    Fcmov {
        BSt0Sti(Sti),
        ESt0Sti(Sti),
        BeSt0Sti(Sti),
        USt0Sti(Sti),
        NbSt0Sti(Sti),
        NeSt0Sti(Sti),
        NbeSt0Sti(Sti),
        NuSt0Sti(Sti),
    },

    Fcom {
        M32fp(Mem),
        M64fp(Mem),
        Sti(Sti),
    },
    Fcomp {
        M32fp(Mem),
        M64fp(Mem),
        Sti(Sti),
    },
    Fcompp(),

    Fcomi {
        St0Sti(Sti),
    },
    Fcomip {
        St0Sti(Sti),
    },
    Fucomi {
        St0Sti(Sti),
    },
    Fucomip {
        St0Sti(Sti),
    },

    Fcos(),

    Fdecstp(),

    Fdiv {
        M32fp(Mem),
        M64fp(Mem),
        St0Sti(Sti),
        StiSt0(Sti),
    },
    Fdivp {
        StiSt0(Sti),
    },
    Fidiv {
        M32int(Mem),
        M16int(Mem),
    },

    Fdivr {
        M32fp(Mem),
        M64fp(Mem),
        St0Sti(Sti),
        StiSt0(Sti),
    },
    Fdivrp {
        StiSt0(Sti),
    },
    Fidivr {
        M32int(Mem),
        M16int(Mem),
    },

    Ffree {
        Sti(Sti),
    },

    Ficom {
        M16int(Mem),
        M32int(Mem),
    },
    Ficomp {
        M16int(Mem),
        M32int(Mem),
    },

    Fild {
        M16int(Mem),
        M32int(Mem),
        M64int(Mem),
    },

    Fincstp(),

    Finit(),
    Fninit(),

    Fist {
        M16int(Mem),
        M32int(Mem),
    },
    Fistp {
        M16int(Mem),
        M32int(Mem),
        M64int(Mem),
    },

    Fisttp {
        M16int(Mem),
        M32int(Mem),
        M64int(Mem),
    },

    Fld {
        M32fp(Mem),
        M64fp(Mem),
        M80fp(Mem),
        Sti(Sti),
    },

    Fld1(),
    Fldl2t(),
    Fldl2e(),
    Fldpi(),
    Fldlg2(),
    Fldln2(),
    Fldz(),

    Fldcw {
        M2byte(Mem),
    },

    Fldenv {
        M28byte(Mem),
    },

    Fmul {
        M32fp(Mem),
        M64fp(Mem),
        St0Sti(Sti),
        StiSt0(Sti),
    },
    Fmulp {
        StiSt0(Sti),
    },
    Fimul {
        M32int(Mem),
        M16int(Mem),
    },

    Fnop(),

    Fpatan(),

    Fprem(),

    Fprem1(),

    Fptan(),

    Frndint(),

    Frstor {
        M108byte(Mem),
    },

    Fsave {
        M108byte(Mem),
    },
    Fnsave {
        M108byte(Mem),
    },

    Fscale(),

    Fsin(),

    Fsincos(),

    Fsqrt(),

    Fst {
        M32fp(Mem),
        M64fp(Mem),
        Sti(Sti),
    },
    Fstp {
        M32fp(Mem),
        M64fp(Mem),
        M80fp(Mem),
        Sti(Sti),
    },

    Fstcw {
        M2byte(Mem),
    },
    Fnstcw {
        M2byte(Mem),
    },

    Fstenv {
        M28byte(Mem),
    },
    Fnstenv {
        M28byte(Mem),
    },

    Fstsw {
        M2byte(Mem),
        Ax,
    },
    Fnstsw {
        M2byte(Mem),
        Ax,
    },

    Fsub {
        M32fp(Mem),
        M64fp(Mem),
        St0Sti(Sti),
        StiSt0(Sti),
    },
    Fsubp {
        StiSt0(Sti),
    },
    Fisub {
        M32int(Mem),
        M16int(Mem),
    },

    Fsubr {
        M32fp(Mem),
        M64fp(Mem),
        St0Sti(Sti),
        StiSt0(Sti),
    },
    Fsubrp {
        StiSt0(Sti),
    },
    Fisubr {
        M32int(Mem),
        M16int(Mem),
    },

    Ftst(),

    Fucom {
        Sti(Sti),
    },
    Fucomp {
        Sti(Sti),
    },
    Fucompp(),

    Fxam(),

    Fxch {
        Sti(Sti),
    },

    Fxrstor {
        M512byte(Mem),
    },
    Fxrstor64 {
        M512byte(Mem),
    },

    Fxsave {
        M512byte(Mem),
    },
    Fxsave64 {
        M512byte(Mem),
    },

    Fxtract(),

    Fyl2x(),

    Fyl2xp1(),

    // HADDPD
    // HADDPS

    Hlt(),

    // HSUBPD
    // HSUBPS

    Idiv {
        Rm8l(Rm8l),
        Rm8(Rm8),
        Rm16(Rm16),
        Rm32(Rm32),
        Rm64(Rm64),
    },

    Imul {
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
    },

    In {
        AlImm8(Imm8),
        AxImm8(Imm8),
        EaxImm8(Imm8),

        AlDx,
        AxDx,
        EaxDx,
    },

    Inc {
        Rm8l(Rm8l),
        Rm8(Rm8),
        Rm16(Rm16),
        Rm32(Rm32),
        Rm64(Rm64),
    },

    Ins {
        B,
        W,
        D,
    },

    // INSERTPS

    Int3(),
    Int {
        Imm8(Imm8),
    },
    Into(),

    Invd(),

    Invlpg {
        M(Mem),
    },

    Invpcid {
        R64M128(R64, Mem),
    },

    Iret {
        D,
        Q,
    },

    J {
        CcRel8(Cc, Rel8),
        RcxzRel8(Rel8),
        CcRel32(Cc, Rel32),
    },

    Jmp {
        Rel8(Rel8),
        Rel32(Rel32),
        Rm64(Rm64),
        M16x16(Mem),
        M16x32(Mem),
        M16x64(Mem),
    },

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

    Lss {
        R16M16x16(R16, Mem),
        R32M16x32(R32, Mem),
        R64M16x64(R64, Mem),
    },
    Lfs {
        R16M16x16(R16, Mem),
        R32M16x32(R32, Mem),
        R64M16x64(R64, Mem),
    },
    Lgs {
        R16M16x16(R16, Mem),
        R32M16x32(R32, Mem),
        R64M16x64(R64, Mem),
    },

    Lea {
        R16M(R16, Mem),
        R32M(R32, Mem),
        R64M(R64, Mem),
    },

    Leave(),

    Lfence(),

    Lgdt {
        M16x64(Mem),
    },
    Lidt {
        M16x64(Mem),
    },

    Lldt {
        Rm16(Rm16),
    },
}
