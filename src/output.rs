pub trait Output {
    type Err;

    fn write_u8(&mut self, x: u8) -> Result<(), Self::Err>;

    #[inline]
    fn write_u16(&mut self, x: u16) -> Result<(), Self::Err> {
        self.write_u8((x & 0xff) as u8)?;
        self.write_u8((x >> 8) as u8)
    }

    #[inline]
    fn write_u32(&mut self, x: u32) -> Result<(), Self::Err> {
        self.write_u16((x & 0xffff) as u16)?;
        self.write_u16((x >> 16) as u16)
    }

    #[inline]
    fn write_u64(&mut self, x: u64) -> Result<(), Self::Err> {
        self.write_u32((x & 0xffffffff) as u32)?;
        self.write_u32((x >> 32) as u32)
    }
}

pub struct SliceOutput<'a> {
    slice: &'a mut [u8],
    index: usize,
}

impl<'a> SliceOutput<'a> {
    pub fn new(slice: &'a mut [u8]) -> Self {
        SliceOutput {
            slice: slice,
            index: 0,
        }
    }
}

impl<'a> Output for SliceOutput<'a> {
    // Wants to be !.
    type Err = ();

    #[inline]
    fn write_u8(&mut self, x: u8) -> Result<(), ()> {
        self.slice[self.index] = x;
        self.index += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Output, SliceOutput};

    #[test]
    fn write_u8() {
        let mut buf = [0; 1];
        {
            let mut out = SliceOutput::new(&mut buf);
            out.write_u8(0x01).unwrap();
        }
        assert_eq!([0x01], buf);
    }

    #[test]
    fn write_u16() {
        let mut buf = [0; 2];
        {
            let mut out = SliceOutput::new(&mut buf);
            out.write_u16(0x0201).unwrap();
        }
        assert_eq!([0x01, 0x02], buf);
    }

    #[test]
    fn write_u32() {
        let mut buf = [0; 4];
        {
            let mut out = SliceOutput::new(&mut buf);
            out.write_u32(0x04030201).unwrap();
        }
        assert_eq!([0x01, 0x02, 0x03, 0x04], buf);
    }

    #[test]
    fn write_u64() {
        let mut buf = [0; 8];
        {
            let mut out = SliceOutput::new(&mut buf);
            out.write_u64(0x0807060504030201).unwrap();
        }
        assert_eq!([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08], buf);
    }
}
