pub struct Encoder<'a> {
    buf: &'a mut [u8],
    index: usize,
}

impl<'a> Encoder<'a> {
    #[inline]
    pub fn new(buf: &'a mut [u8]) -> Self {
        Encoder {
            buf: buf,
            index: 0,
        }
    }

    #[inline]
    pub fn write_u8(&mut self, x: u8) {
        self.buf[self.index] = x;
        self.index += 1;
    }

    #[inline]
    pub fn write_u16(&mut self, x: u16) {
        self.write_u8((x & 0xff) as u8);
        self.write_u8((x >> 8) as u8);
    }

    #[inline]
    pub fn write_u32(&mut self, x: u32) {
        self.write_u16((x & 0xffff) as u16);
        self.write_u16((x >> 16) as u16);
    }

    #[inline]
    pub fn write_u64(&mut self, x: u64) {
        self.write_u32((x & 0xffffffff) as u32);
        self.write_u32((x >> 32) as u32);
    }
}

#[cfg(test)]
mod tests {
    use super::Encoder;

    #[test]
    fn write_u8() {
        let mut buf = [0; 1];
        {
            let mut encoder = Encoder::new(&mut buf);
            encoder.write_u8(0x01);
        }
        assert_eq!([0x01], buf);
    }

    #[test]
    fn write_u16() {
        let mut buf = [0; 2];
        {
            let mut encoder = Encoder::new(&mut buf);
            encoder.write_u16(0x0201);
        }
        assert_eq!([0x01, 0x02], buf);
    }

    #[test]
    fn write_u32() {
        let mut buf = [0; 4];
        {
            let mut encoder = Encoder::new(&mut buf);
            encoder.write_u32(0x04030201);
        }
        assert_eq!([0x01, 0x02, 0x03, 0x04], buf);
    }

    #[test]
    fn write_u64() {
        let mut buf = [0; 8];
        {
            let mut encoder = Encoder::new(&mut buf);
            encoder.write_u64(0x0807060504030201);
        }
        assert_eq!([0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08], buf);
    }
}
