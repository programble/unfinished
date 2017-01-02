use std::ops::Range;

/// Gap buffer...
#[derive(Debug)] // TODO
pub struct GapBuffer {
    buf: Vec<u8>,
    gap: Range<usize>,
}

impl GapBuffer {
    /// Constructs a new, empty gap buffer.
    ///
    /// The buffer will not allocate until bytes are inserted.
    pub fn new() -> Self {
        GapBuffer {
            buf: Vec::new(),
            gap: 0..0,
        }
    }

    /// Constructs a new, empty gap buffer with the specified capacity.
    ///
    /// The buffer will be able to hold exactly `capacity` elements without reallocating. If
    /// `capacity` is 0, the buffer will not allocate.
    pub fn with_capacity(capacity: usize) -> Self {
        GapBuffer {
            buf: vec![0; capacity], // Calls Vec::with_capacity.
            gap: 0..capacity,
        }
    }
}

impl Default for GapBuffer {
    fn default() -> Self {
        Self::new()
    }
}
