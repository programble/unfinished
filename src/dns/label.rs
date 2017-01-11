use std::io::Error as IoError;

use byteorder::{ReadBytesExt, WriteBytesExt};

use s11n::{Deserialize, Serialize};
use super::Error;

/// Domain name label.
///
/// Cannot exceed 63 data bytes.
#[derive(Clone)]
pub struct Label {
    data: Vec<u8>,
}

impl Label {
    /// Returns the number of data bytes.
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl Serialize for Label {
    fn serialize<W: WriteBytesExt>(&self, write: &mut W) -> Result<(), IoError> {
        write.write_u8(self.data.len() as u8)?;
        write.write_all(&self.data)
    }

    fn serialized_len(&self) -> usize {
        1 + self.data.len()
    }
}

impl Deserialize for Label {
    type Err = Error;

    fn deserialize<R: ReadBytesExt>(read: &mut R) -> Result<Self, Error> {
        let len = read.read_u8()? as usize;
        if len > 63 {
            Err(Error::LabelLen(len))
        } else {
            let mut label = Label { data: vec![0; len] };
            read.read_exact(&mut label.data)?;
            Ok(label)
        }
    }
}

#[cfg(test)]
mod tests {
    use s11n::{Deserialize, Serialize};
    use super::Label;

    #[test]
    fn label_ok_null() {
        let input = &[0];
        let output = Label::from_bytes(input).unwrap().to_bytes();
        assert_eq!(input, &output[..]);
    }

    #[test]
    fn label_ok() {
        let input: &mut [u8] = &mut [0; 64];
        input[0] = 63;
        let output = Label::from_bytes(input).unwrap().to_bytes();
        assert_eq!(input, &output[..]);
    }

    #[test]
    #[should_panic = "LabelLen"]
    fn label_err() {
        let input = &mut [0; 65];
        input[0] = 64;
        Label::from_bytes(input).unwrap();
    }
}
