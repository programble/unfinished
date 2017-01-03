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
        Self::from(Vec::new())
    }

    /// Constructs a new, empty gap buffer with the specified capacity.
    ///
    /// The buffer will be able to hold exactly `capacity` elements without reallocating. If
    /// `capacity` is 0, the buffer will not allocate.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::from(Vec::with_capacity(capacity))
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

    /// Reserves capacity for at least `additional` more bytes to be inserted in the buffer. The
    /// buffer may reserve more space to avoid frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        // Move gap to end.
        let len = self.len();
        self.splice(len..len, &[]);

        self.buf.reserve(additional);
        let capacity = self.buf.capacity();
        self.buf.resize(capacity, 0);

        self.gap.end = self.buf.len();
    }

    fn splice(&mut self, _dest: Range<usize>, _src: &[u8]) {
        unimplemented!()
    }
}

impl Default for GapBuffer {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<u8>> for GapBuffer {
    #[inline]
    fn from(mut buf: Vec<u8>) -> Self {
        let len = buf.len();
        let capacity = buf.capacity();
        buf.resize(capacity, 0);
        GapBuffer {
            buf: buf,
            gap: len..capacity,
        }
    }
}

impl From<GapBuffer> for Vec<u8> {
    #[inline]
    fn from(mut buf: GapBuffer) -> Self {
        // Move gap to end.
        let len = buf.len();
        buf.splice(len..len, &[]);
        buf.buf.truncate(len);
        buf.buf
    }
}
