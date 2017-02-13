mod instruction;
mod operand;
mod prefix;

use output::Output;

#[allow(unused_variables)]
pub trait Encode {
    fn encode_prefix1<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        Ok(())
    }

    fn encode_prefix2<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        Ok(())
    }

    fn encode_prefix3<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        Ok(())
    }

    fn encode_prefix4<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        Ok(())
    }

    fn encode_rex<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        Ok(())
    }

    fn encode_opcode<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output;

    fn encode_modrm<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        Ok(())
    }

    fn encode_sib<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        Ok(())
    }

    fn encode_disp<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        Ok(())
    }

    fn encode_imm<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        Ok(())
    }

    fn encode<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output {
        self.encode_prefix1(out)?;
        self.encode_prefix2(out)?;
        self.encode_prefix3(out)?;
        self.encode_prefix4(out)?;
        self.encode_rex(out)?;
        self.encode_opcode(out)?;
        self.encode_modrm(out)?;
        self.encode_sib(out)?;
        self.encode_disp(out)?;
        self.encode_imm(out)
    }
}
