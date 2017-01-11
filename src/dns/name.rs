use std::io::Error as IoError;

use byteorder::{ReadBytesExt, WriteBytesExt};

use s11n::{Deserialize, Serialize};
use super::{Error, Label};

/// Domain name.
///
/// Cannot exceed 255 bytes in serialized length.
#[derive(Clone)]
pub struct Name {
    labels: Vec<Label>,
}

impl Serialize for Name {
    fn serialize<W: WriteBytesExt>(&self, write: &mut W) -> Result<(), IoError> {
        for label in &self.labels {
            label.serialize(write)?;
        }
        Ok(())
    }

    fn serialized_len(&self) -> usize {
        self.labels.iter().map(Serialize::serialized_len).sum()
    }
}

impl Deserialize for Name {
    type Err = Error;

    fn deserialize<R: ReadBytesExt>(read: &mut R) -> Result<Self, Error> {
        let mut name = Name { labels: Vec::new() };
        loop {
            name.labels.push(Label::deserialize(read)?);
            if name.serialized_len() > 255 {
                return Err(Error::NameLen(name.serialized_len()));
            }
            if name.labels.last().unwrap().len() == 0 {
                return Ok(name)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use s11n::{Deserialize, Serialize};
    use super::Name;

    #[test]
    fn name_ok() {
        let input = &[5, 1, 2, 3, 4, 5, 3, 1, 2, 3, 0];
        let output = Name::from_bytes(input).unwrap().to_bytes();
        assert_eq!(input, &output[..]);
    }

    #[test]
    #[should_panic = "NameLen"]
    fn name_err() {
        let input = &mut [0; 256];
        input[0] = 63;
        input[64] = 63;
        input[128] = 63;
        input[192] = 63;
        Name::from_bytes(input).unwrap();
    }
}
