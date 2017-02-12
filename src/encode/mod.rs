mod instruction;
mod prefix;

use output::Output;

pub trait Encode {
    fn encode<O>(&self, out: &mut O) -> Result<(), O::Err> where O: Output;
}
