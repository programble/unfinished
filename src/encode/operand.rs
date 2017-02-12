use encode::prefix;
use mnemonic::operand::*;
use output::Output;

pub trait Prefix {
    fn encode_prefix<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output;
}

impl Prefix for Sreg {
    fn encode_prefix<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        use self::Sreg::*;
        match *self {
            Cs => out.write_u8(prefix::CS),
            Ds => out.write_u8(prefix::DS),
            Ss => out.write_u8(prefix::SS),
            Es => out.write_u8(prefix::ES),
            Fs => out.write_u8(prefix::FS),
            Gs => out.write_u8(prefix::GS),
        }
    }
}

impl<Base32, Index32, Base64, Index64> Prefix for Memory<Base32, Index32, Base64, Index64> {
    fn encode_prefix<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        if let Memory::Offset32(..) = *self {
            out.write_u8(prefix::ADDRESS_SIZE)?;
        }
        match *self {
            Memory::Offset32(Some(sreg), _) | Memory::Offset64(Some(sreg), _) => {
                sreg.encode_prefix(out)
            },
            _ => Ok(()),
        }
    }
}

macro_rules! impl_prefix_rmx {
    ($rm:ident, $mem:ident, $mex:ident) => {
        impl Prefix for $rm {
            fn encode_prefix<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
                match *self {
                    $rm::$mem(ref mem) => mem.encode_prefix(out),
                    $rm::$mex(ref mex) => mex.encode_prefix(out),
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

macro_rules! impl_prefix_rmxrx {
    ($rmr:ident, $mem:ident, $mex:ident) => {
        impl Prefix for $rmr {
            fn encode_prefix<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
                match *self {
                    $rmr::$mem(ref mem, _) => mem.encode_prefix(out),
                    $rmr::$mex(ref mex, _) => mex.encode_prefix(out),
                    _ => Ok(()),
                }
            }
        }
    }
}

impl_prefix_rmxrx!(Rm8R8, Mem8Reg8, Mex8Rex8);
impl_prefix_rmxrx!(Rm16R16, Mem16Reg16, Mex16Rex16);
impl_prefix_rmxrx!(Rm32R32, Mem32Reg32, Mex32Rex32);

impl Prefix for Rm64R64 {
    fn encode_prefix<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        match *self {
            Rm64R64::Mex64Rex64(ref mex, _) => mex.encode_prefix(out),
            _ => Ok(()),
        }
    }
}

macro_rules! impl_prefix_rxrmx {
    ($rmr:ident, $mem:ident, $mex:ident) => {
        impl Prefix for $rmr {
            fn encode_prefix<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
                match *self {
                    $rmr::$mem(_, ref mem) => mem.encode_prefix(out),
                    $rmr::$mex(_, ref mex) => mex.encode_prefix(out),
                    _ => Ok(()),
                }
            }
        }
    }
}

impl_prefix_rxrmx!(R8Rm8, Reg8Mem8, Rex8Mex8);
impl_prefix_rxrmx!(R16Rm16, Reg16Mem16, Rex16Mex16);
impl_prefix_rxrmx!(R32Rm32, Reg32Mem32, Rex32Mex32);

impl Prefix for R64Rm64 {
    fn encode_prefix<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        match *self {
            R64Rm64::Rex64Mex64(_, ref mex) => mex.encode_prefix(out),
            _ => Ok(()),
        }
    }
}
