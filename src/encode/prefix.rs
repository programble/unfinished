use super::Encode;

use mnemonic::prefix::{Lock, Rep, Repne};
use output::Output;

pub const LOCK: u8 = 0xf0;
pub const REPNE: u8 = 0xf2;
pub const REP: u8 = 0xf3;

pub const CS: u8 = 0x2e;
pub const SS: u8 = 0x36;
pub const DS: u8 = 0x3e;
pub const ES: u8 = 0x26;
pub const FS: u8 = 0x64;
pub const GS: u8 = 0x65;

pub const OPERAND_SIZE: u8 = 0x66;

pub const ADDRESS_SIZE: u8 = 0x67;

impl<I> Encode for Lock<I> where I: Encode {
    fn encode<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        out.write_u8(LOCK)?;
        self.0.encode(out)
    }
}

impl<I> Encode for Repne<I> where I: Encode {
    fn encode<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        out.write_u8(REPNE)?;
        self.0.encode(out)
    }
}

impl<I> Encode for Rep<I> where I: Encode {
    fn encode<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        out.write_u8(REP)?;
        self.0.encode(out)
    }
}
