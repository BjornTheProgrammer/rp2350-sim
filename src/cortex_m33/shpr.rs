use crate::cortex_m33::operation::get_bits;

pub struct Shpr {
    shpr2: u32,
    shpr3: u32,
}

impl Shpr {
    pub fn new() -> Self {
        Self { shpr2: 0, shpr3: 0 }
    }

    pub fn pri_11(&self) -> u8 {
        get_bits(self.shpr2, 30..=31) as u8
    }

    pub fn pri_15(&self) -> u8 {
        get_bits(self.shpr3, 30..=31) as u8
    }

    pub fn pri_14(&self) -> u8 {
        get_bits(self.shpr3, 22..=23) as u8
    }
}
