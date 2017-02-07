pub mod encoder;

use self::encoder::Encoder;

pub trait Encode {
    fn encode<E>(&self, encoder: &mut E) -> Result<(), E::Err> where E: Encoder;
}
