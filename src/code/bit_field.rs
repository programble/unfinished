use code::{Rex, Modrm, Sib, Instruction};

impl Default for Rex {
    #[inline]
    fn default() -> Self {
        Rex(0x40)
    }
}

impl Rex {
    #[inline]
    pub fn w(self) -> Self {
        Rex(self.0 | 0b1000)
    }

    #[inline]
    pub fn r(self) -> Self {
        Rex(self.0 | 0b0100)
    }

    #[inline]
    pub fn x(self) -> Self {
        Rex(self.0 | 0b0010)
    }

    #[inline]
    pub fn b(self) -> Self {
        Rex(self.0 | 0b0001)
    }
}

impl Modrm {
    #[inline]
    pub fn mode(self, mode: u8) -> Self {
        debug_assert_eq!(0, mode & !0b11);
        Modrm(self.0 & 0b00_111_111 | mode << 6)
    }

    #[inline]
    pub fn reg(self, reg: u8) -> Self {
        debug_assert_eq!(0, reg & !0b111);
        Modrm(self.0 & 0b11_000_111 | (reg << 3 & 0b00_111_000))
    }

    #[inline]
    pub fn rm(self, rm: u8) -> Self {
        debug_assert_eq!(0, rm & !0b111);
        Modrm(self.0 & 0b11_111_000 | (rm & 0b00_000_111))
    }
}

impl Sib {
    #[inline]
    pub fn scale(self, scale: u8) -> Self {
        debug_assert_eq!(0, scale & !0b11);
        Sib(self.0 & 0b00_111_111 | scale << 6)
    }

    #[inline]
    pub fn index(self, index: u8) -> Self {
        debug_assert_eq!(0, index & !0b111);
        Sib(self.0 & 0b11_000_111 | (index << 3 & 0b00_111_000))
    }

    #[inline]
    pub fn base(self, base: u8) -> Self {
        debug_assert_eq!(0, base & !0b111);
        Sib(self.0 & 0b11_111_000 | (base & 0b00_000_111))
    }
}

impl Instruction {
    #[inline]
    pub fn rex(mut self) -> Self {
        self.rex = Some(self.rex.unwrap_or_default());
        self
    }

    #[inline]
    pub fn rex_w(mut self) -> Self {
        self.rex = Some(self.rex.unwrap_or_default().w());
        self
    }

    #[inline]
    pub fn rex_r(mut self) -> Self {
        self.rex = Some(self.rex.unwrap_or_default().r());
        self
    }

    #[inline]
    pub fn rex_x(mut self) -> Self {
        self.rex = Some(self.rex.unwrap_or_default().x());
        self
    }

    #[inline]
    pub fn rex_b(mut self) -> Self {
        self.rex = Some(self.rex.unwrap_or_default().b());
        self
    }

    #[inline]
    pub fn modrm_mode(mut self, mode: u8) -> Self {
        self.modrm = Some(self.modrm.unwrap_or_default().mode(mode));
        self
    }

    #[inline]
    pub fn modrm_reg(mut self, reg: u8) -> Self {
        self.modrm = Some(self.modrm.unwrap_or_default().reg(reg));
        self
    }

    #[inline]
    pub fn modrm_rm(mut self, rm: u8) -> Self {
        self.modrm = Some(self.modrm.unwrap_or_default().rm(rm));
        self
    }

    #[inline]
    pub fn sib_scale(mut self, scale: u8) -> Self {
        self.sib = Some(self.sib.unwrap_or_default().scale(scale));
        self
    }

    #[inline]
    pub fn sib_index(mut self, index: u8) -> Self {
        self.sib = Some(self.sib.unwrap_or_default().index(index));
        self
    }

    #[inline]
    pub fn sib_base(mut self, base: u8) -> Self {
        self.sib = Some(self.sib.unwrap_or_default().base(base));
        self
    }
}
