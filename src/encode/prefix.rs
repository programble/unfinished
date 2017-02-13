use encode::Encode;
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

macro_rules! impl_encode {
    ($name:ident, $val:expr) => {
        impl<I> Encode for $name<I> where I: Encode {
            fn encode_prefix1<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
                out.write_u8($val)
            }
            fn encode_prefix2<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
                self.0.encode_prefix2(out)
            }
            fn encode_prefix3<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
                self.0.encode_prefix3(out)
            }
            fn encode_prefix4<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
                self.0.encode_prefix4(out)
            }
            fn encode_rex<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
                self.0.encode_rex(out)
            }
            fn encode_opcode<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
                self.0.encode_opcode(out)
            }
            fn encode_modrm<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
                self.0.encode_modrm(out)
            }
            fn encode_sib<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
                self.0.encode_sib(out)
            }
            fn encode_disp<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
                self.0.encode_disp(out)
            }
            fn encode_imm<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
                self.0.encode_imm(out)
            }
        }
    }
}

impl_encode!(Lock, LOCK);
impl_encode!(Repne, REPNE);
impl_encode!(Rep, REP);
