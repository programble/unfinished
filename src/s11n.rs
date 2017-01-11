//! Serialization (s11n) and deserialization.

use std::io::Error as IoError;

use byteorder::{ReadBytesExt, WriteBytesExt};

/// Serialize to a `Write`.
pub trait Serialize {
    /// Perform the serialization.
    fn serialize<W: WriteBytesExt>(&self, write: &mut W) -> Result<(), IoError>;

    /// Returns the length in bytes of the serialization.
    fn serialized_len(&self) -> usize;

    /// Serialize to a `Vec<u8>`.
    fn to_bytes(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(self.serialized_len());
        self.serialize(&mut vec).unwrap();
        vec
    }
}

/// Deserialize from a `Read`.
pub trait Deserialize: Sized {
    /// The type returned in the event of a deserialization error.
    type Err;

    /// Perform the deserialization.
    fn deserialize<R: ReadBytesExt>(read: &mut R) -> Result<Self, Self::Err>;
}
