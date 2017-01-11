use std::io::Error as IoError;

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
