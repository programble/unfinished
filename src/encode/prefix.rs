use super::Encode;
use mnemonic::prefix::{Lock, Rep, Repne};
use output::Output;

const LOCK: u8 = 0xf0;
const REPNE: u8 = 0xf2;
const REP: u8 = 0xf3;

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
