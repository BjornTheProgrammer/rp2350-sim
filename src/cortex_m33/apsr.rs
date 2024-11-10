use bitmatch::*;

#[derive(Debug, Default)]
pub struct Xpsr {
    pub apsr: Apsr,
    pub ipsr: Ipsr,
    pub epsr: Epsr,
}

impl Xpsr {
    pub fn new() -> Self {
        Xpsr::default()
    }

    pub fn set_from_u32(&mut self, value: u32) {
        self.apsr.set_from_u32(value);
        self.ipsr.set_from_u32(value);
        self.epsr.set_from_u32(value);
    }

    pub fn into_u32(&self) -> u32 {
        self.apsr.into_u32() | self.ipsr.into_u32() | self.epsr.into_u32()
    }
}

#[derive(Debug, Default)]
pub struct Epsr {
    ici_0: u8,
    t: bool,
    b: bool,
    ici_1: u8,
}

impl Epsr {
    pub fn new() -> Self {
        Epsr::default()
    }

    pub fn ici(&self) -> u8 {
        todo!();
    }

    pub fn set_t(&mut self, value: bool) {
        self.t = value;
    }

    pub fn t(&self) -> bool {
        self.t
    }

    pub fn set_b(&mut self, value: bool) {
        self.b = value;
    }

    pub fn b(&self) -> bool {
        self.b
    }

    #[bitmatch]
    pub fn set_from_u32(&mut self, value: u32) {
        #[bitmatch]
        let "????_?uut_??b?_????_??ll_lll?_????_????" = value;
        self.ici_0 = u as u8;
        self.ici_1 = l as u8;
        self.set_b(b != 0);
        self.set_t(t != 0);
    }

    #[bitmatch]
    pub fn into_u32(&self) -> u32 {
        let l: u32 = self.ici_0.into();
        let u: u32 = self.ici_1.into();
        let b = self.b as u32;
        let t = self.t as u32;
        bitpack!("0000_0uut_00b0_0000_00ll_lll0_0000_0000")
    }
}

#[derive(Debug, Default)]
pub struct Ipsr(u8);

impl Ipsr {
    #[bitmatch]
    pub fn set_from_u32(&mut self, value: u32) {
        #[bitmatch]
        let "????_????_????_????_????_????_vvvv_vvvv" = value;

        self.0 = v as u8;
    }

    #[bitmatch]
    pub fn into_u32(&self) -> u32 {
        self.0 as u32
    }
}

#[derive(Debug, Default)]
pub struct Apsr {
    n: bool,
    z: bool,
    c: bool,
    v: bool,
    q: bool,
    ge: u8
}

impl Apsr {
    pub fn new() -> Self {
        Apsr::default()
    }

    /**
    Negative condition flag. When updated by a flag setting instruction this bit indicates whether the result of the
    operation when treated as a two’s complement signed integer is negative. \
    \
    The possible values of this bit are:
    * 0: result is positive or zero.
    * 1: result is negative
    */
    pub fn n(&self) -> bool {
        self.n
    }

    /**
    Zero condition flag. When updated by a flag setting instruction this bit indicates whether the result of the
    operation was zero. \
    \
    The possible values of this bit are:
    * 0: result is nonzero.
    * 1: result is zero
    */
    pub fn z(&self) -> bool {
        self.z
    }

    /**
    Carry condition flag. When updated by a flag setting instruction this bit indicates whether the operation
    resulted in an unsigned overflow or whether the last bit shifted out of the result was set. \
    \
    The possible values of this bit are:
    * 0: No carry occurred, or last bit shifted was clear.
    * 1: Carry occurred, or last bit shifted was set
    */
    pub fn c(&self) -> bool {
        self.c
    }

    /**
    Overflow condition flag. When updated by a flag setting instruction this bit indicates whether a signed
    overflow occurred. \
    \
    The possible values of this bit are:
    * 0: Signed overflow did not occur.
    * 1: Signed overflow occurred.
    */
    pub fn v(&self) -> bool {
        self.v
    }

    /**
    Sticky saturation flag. When updated by certain instructions this bit indicates either that an overflow occurred
    or that the result was saturated. This bit is cumulative and can only be cleared to zero by software. \
    \
    The possible values of this bit are:
    * 0: Saturation or overflow has not occurred since bit was last cleared.
    * 1: Saturation or overflow has occurred since bit was last cleared.
    */
    pub fn q(&self) -> bool {
        self.q
    }

    /**
    Greater than or equal flags. When updated by parallel addition and subtraction instructions these bits record
    whether the result was greater than or equal to zero. SEL instructions use these bits to determine which
    register to select a particular byte from.
    */
    pub fn ge(&self) -> u8 {
        self.ge
    }

    pub fn set_n(&mut self, value: bool) {
        self.n = value
    }

    pub fn set_z(&mut self, value: bool) {
        self.z = value
    }

    pub fn set_c(&mut self, value: bool) {
        self.c = value
    }

    pub fn set_v(&mut self, value: bool) {
        self.v = value
    }

    pub fn set_q(&mut self, value: bool) {
        self.q = value
    }

    pub fn set_ge(&mut self, value: u8) {
        self.ge = value
    }

    #[bitmatch]
    pub fn set_from_u32(&mut self, value: u32) {
        #[bitmatch]
        let "nzcv_q???_????_gggg_????_????_????_????" = value;
        self.set_n(n != 0);
        self.set_z(z != 0);
        self.set_c(c != 0);
        self.set_v(v != 0);
        self.set_q(q != 0);

        self.set_ge(g as u8);
    }

    #[bitmatch]
    pub fn into_u32(&self) -> u32 {
        let n = self.n() as u32;
        let z = self.z() as u32;
        let c = self.c() as u32;
        let v = self.v() as u32;
        let q = self.q() as u32;

        let g: u32 = self.ge().into();

        bitpack!("nzcv_q000_0000_gggg_0000_0000_0000_0000")
    }
}
