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
    #[inline]
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
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        let mut buf = Vec::with_capacity(capacity);
        buf.resize(capacity, 0);
        GapBuffer {
            buf: buf,
            gap: 0..capacity,
        }
    }

    /// Returns the number of bytes the buffer can hold without reallocating.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    /// Returns the number of bytes in the buffer.
    #[inline]
    pub fn len(&self) -> usize {
        self.capacity() - (self.gap.end - self.gap.start)
    }

    /// Returns `true` if the buffer contains no bytes.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for GapBuffer {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
