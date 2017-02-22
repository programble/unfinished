use code::{Rex, Instruction};

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
}
