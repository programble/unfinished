use encode::prefix;
use mnemonic::operand::*;
use output::Output;

pub trait Prefix {
    fn encode_prefix2<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output;
    fn encode_prefix4<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output;
}

fn encode_sreg<O>(sreg: Sreg, out: &mut O) -> Result<(), O::Err> where O: Output {
    match sreg {
        Sreg::Cs => out.write_u8(prefix::CS),
        Sreg::Ds => out.write_u8(prefix::DS),
        Sreg::Ss => out.write_u8(prefix::SS),
        Sreg::Es => out.write_u8(prefix::ES),
        Sreg::Fs => out.write_u8(prefix::FS),
        Sreg::Gs => out.write_u8(prefix::GS),
    }
}

impl<Base32, Index32, Base64, Index64> Prefix for Memory<Base32, Index32, Base64, Index64> {
    fn encode_prefix2<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        match *self {
            Memory::Offset32(Some(sreg), _) | Memory::Offset64(Some(sreg), _) => {
                encode_sreg(sreg, out)
            },
            _ => Ok(()),
        }
    }

    fn encode_prefix4<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        match *self {
            Memory::Offset32(..) => out.write_u8(prefix::ADDRESS_SIZE),
            _ => Ok(()),
        }
    }
}

macro_rules! impl_prefix_rmx {
    ($rmx:ident, $mem:ident, $mex:ident) => {
        impl Prefix for $rmx {
            fn encode_prefix2<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
                match *self {
                    $rmx::$mem(ref mem) => mem.encode_prefix2(out),
                    $rmx::$mex(ref mex) => mex.encode_prefix2(out),
                    _ => Ok(()),
                }
            }

            fn encode_prefix4<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
                match *self {
                    $rmx::$mem(ref mem) => mem.encode_prefix4(out),
                    $rmx::$mex(ref mex) => mex.encode_prefix4(out),
                    _ => Ok(()),
                }
            }
        }
    }
}

macro_rules! impl_prefix_rmxrx {
    ($rmxrx:ident, $mem:ident, $mex:ident) => {
        impl Prefix for $rmxrx {
            fn encode_prefix2<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
                match *self {
                    $rmxrx::$mem(ref mem, _) => mem.encode_prefix2(out),
                    $rmxrx::$mex(ref mex, _) => mex.encode_prefix2(out),
                    _ => Ok(()),
                }
            }

            fn encode_prefix4<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
                match *self {
                    $rmxrx::$mem(ref mem, _) => mem.encode_prefix4(out),
                    $rmxrx::$mex(ref mex, _) => mex.encode_prefix4(out),
                    _ => Ok(()),
                }
            }
        }
    }
}

macro_rules! impl_prefix_rxrmx {
    ($rxrmx:ident, $mem:ident, $mex:ident) => {
        impl Prefix for $rxrmx {
            fn encode_prefix2<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
                match *self {
                    $rxrmx::$mem(_, ref mem) => mem.encode_prefix2(out),
                    $rxrmx::$mex(_, ref mex) => mex.encode_prefix2(out),
                    _ => Ok(()),
                }
            }

            fn encode_prefix4<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
                match *self {
                    $rxrmx::$mem(_, ref mem) => mem.encode_prefix4(out),
                    $rxrmx::$mex(_, ref mex) => mex.encode_prefix4(out),
                    _ => Ok(()),
                }
            }
        }
    }
}

impl_prefix_rmx!(Rm8, Mem8, Mex8);
impl_prefix_rmx!(Rm16, Mem16, Mex16);
impl_prefix_rmx!(Rm32, Mem32, Mex32);
impl_prefix_rmx!(Rm64, Mem64, Mex64);

impl_prefix_rmxrx!(Rm8R8, Mem8Reg8, Mex8Rex8);
impl_prefix_rmxrx!(Rm16R16, Mem16Reg16, Mex16Rex16);
impl_prefix_rmxrx!(Rm32R32, Mem32Reg32, Mex32Rex32);

impl_prefix_rxrmx!(R8Rm8, Reg8Mem8, Rex8Mex8);
impl_prefix_rxrmx!(R16Rm16, Reg16Mem16, Rex16Mex16);
impl_prefix_rxrmx!(R32Rm32, Reg32Mem32, Rex32Mex32);

impl Prefix for Rm64R64 {
    fn encode_prefix2<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        match *self {
            Rm64R64::Mex64Rex64(ref mex, _) => mex.encode_prefix2(out),
            _ => Ok(()),
        }
    }

    fn encode_prefix4<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        match *self {
            Rm64R64::Mex64Rex64(ref mex, _) => mex.encode_prefix4(out),
            _ => Ok(()),
        }
    }
}

impl Prefix for R64Rm64 {
    fn encode_prefix2<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        match *self {
            R64Rm64::Rex64Mex64(_, ref mex) => mex.encode_prefix2(out),
            _ => Ok(()),
        }
    }

    fn encode_prefix4<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        match *self {
            R64Rm64::Rex64Mex64(_, ref mex) => mex.encode_prefix4(out),
            _ => Ok(()),
        }
    }
}
