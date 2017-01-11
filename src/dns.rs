//! Domain name system.

use std::io::Error as IoError;

use byteorder::{ReadBytesExt, WriteBytesExt};

use s11n::{Deserialize, Serialize};

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
            if name.labels.last().unwrap().data.len() == 0 {
                return Ok(name)
            }
        }
    }
}

/// Domain name label.
///
/// Cannot exceed 63 data bytes.
#[derive(Clone)]
pub struct Label {
    data: Vec<u8>,
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

/// DNS error.
#[derive(Debug)]
pub enum Error {
    /// IO error.
    Io(IoError),

    /// Label length exceeds 63 bytes.
    LabelLen(usize),

    /// Name length exceeds 255 bytes.
    NameLen(usize),
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::Io(err)
    }
}

#[cfg(test)]
mod tests {
    use s11n::{Deserialize, Serialize};

    use super::{Label, Name};

    #[test]
    fn label_null() {
        let input = &[0];
        let output = Label::from_bytes(input).unwrap().to_bytes();
        assert_eq!(input, &output[..]);
    }

    #[test]
    fn label() {
        let input: &mut [u8] = &mut [0; 64];
        input[0] = 63;
        let output = Label::from_bytes(input).unwrap().to_bytes();
        assert_eq!(input, &output[..]);
    }

    #[test]
    #[should_panic = "LabelLen"]
    fn label_exceed() {
        let input = &mut [0; 65];
        input[0] = 64;
        Label::from_bytes(input).unwrap();
    }

    #[test]
    fn name() {
        let input = &[5, 1, 2, 3, 4, 5, 3, 1, 2, 3, 0];
        let output = Name::from_bytes(input).unwrap().to_bytes();
        assert_eq!(input, &output[..]);
    }

    #[test]
    #[should_panic = "NameLen"]
    fn name_exceeds() {
        let input = &mut [0; 256];
        input[0] = 63;
        input[64] = 63;
        input[128] = 63;
        input[192] = 63;
        Name::from_bytes(input).unwrap();
    }
}
