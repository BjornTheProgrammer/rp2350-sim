use bilge::bitsize;
use bilge::prelude::*;
use bilge::FromBits;

#[bitsize(32)]
#[derive(FromBits, Clone, Copy, DebugBits)]
struct ApsrInternal {
    n: bool,
    z: bool,
    c: bool,
    v: bool,
    q: bool,
    /**
    This is reserved for other archetectures, and should never be used, after setting to 0.
    */
    bits0: u7,
    ge: u4,
    /**
    This is reserved for other archetectures, and should never be used, after setting to 0.
    */
    bits1: u16,
}

#[derive(Debug)]
pub struct Apsr {
    aspr: ApsrInternal,
}

impl Apsr {
    pub fn new() -> Self {
        let aspr = ApsrInternal::new(false, false, false, false, false, u7::new(0), u4::new(0), 0);
        Self { aspr }
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
        self.aspr.n()
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
        self.aspr.z()
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
        self.aspr.c()
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
        self.aspr.v()
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
        self.aspr.q()
    }

    /**
    Greater than or equal flags. When updated by parallel addition and subtraction instructions these bits record
    whether the result was greater than or equal to zero. SEL instructions use these bits to determine which
    register to select a particular byte from.
    */
    pub fn ge(&self) -> u4 {
        self.aspr.ge()
    }

    pub fn set_n(&mut self, value: bool) {
        self.aspr.set_n(value);
    }

    pub fn set_z(&mut self, value: bool) {
        self.aspr.set_z(value);
    }

    pub fn set_c(&mut self, value: bool) {
        self.aspr.set_c(value);
    }

    pub fn set_v(&mut self, value: bool) {
        self.aspr.set_v(value);
    }

    pub fn set_q(&mut self, value: bool) {
        self.aspr.set_q(value);
    }

    pub fn set_ge(&mut self, value: u4) {
        self.aspr.set_ge(value);
    }

    pub fn binary(&self) -> u32 {
        self.aspr.into()
    }

    pub fn set_from_u32(&mut self, value: u32) {
        self.aspr = ApsrInternal::from(value);
        self.aspr.set_bits0(u7::new(0));
        self.aspr.set_bits1(0);
    }
}
