use std::marker::PhantomData;

#[derive(PartialEq, Copy, Clone)]
pub struct CortexM33Registers<S: SpControl> {
	pub r0: Register<S>,
	pub r1: Register<S>,
	pub r2: Register<S>,
	pub r3: Register<S>,
	pub r4: Register<S>,
	pub r5: Register<S>,
	pub r6: Register<S>,
	pub r7: Register<S>,
	pub r8: Register<S>,
	pub r9: Register<S>,
	pub r10: Register<S>,
	pub r11: Register<S>,
	pub r12: Register<S>,

	pub sp: Register<S>,

	pub lr: Register<S>,

	pub pc: Register<S>
}

impl<S: SpControl> CortexM33Registers<S> {
	pub fn new() -> Self {
		Self {
			r0: Register::R0(0),
			r1: Register::R1(0),
			r2: Register::R2(0),
			r3: Register::R3(0),
			r4: Register::R4(0),
			r5: Register::R5(0),
			r6: Register::R6(0),
			r7: Register::R7(0),
			r8: Register::R8(0),
			r9: Register::R9(0),
			r10: Register::R10(0),
			r11: Register::R11(0),
			r12: Register::R12(0),
			sp: Register::SP(SP::new(MSP(0), PSP(0))),
			lr: Register::LR(0),
			pc: Register::PC(0)
		}
	}
}

#[derive(PartialEq, Copy, Clone)]
pub enum Register<S: SpControl> {
	R0(u32),
	R1(u32),
	R2(u32),
	R3(u32),
	R4(u32),
	R5(u32),
	R6(u32),
	R7(u32),
	R8(u32),
	R9(u32),
	R10(u32),
	R11(u32),
	R12(u32),
	SP(SP<S>),
	LR(u32),
	PC(u32)
}

impl<S: SpControl> Register<S> {
	pub fn get(&self) -> u32 {
		match self {
			Register::R0(val) => *val,
			Register::R1(val) => *val,
			Register::R2(val) => *val,
			Register::R3(val) => *val,
			Register::R4(val) => *val,
			Register::R5(val) => *val,
			Register::R6(val) => *val,
			Register::R7(val) => *val,
			Register::R8(val) => *val,
			Register::R9(val) => *val,
			Register::R10(val) => *val,
			Register::R11(val) => *val,
			Register::R12(val) => *val,
			Register::SP(val) => val.get(),
			Register::LR(val) => *val,
			Register::PC(val) => *val,
		}
	}

	pub fn set(&mut self, value: u32) {
		match self {
			Register::R0(val) => *val = value,
			Register::R1(val) => *val = value,
			Register::R2(val) => *val = value,
			Register::R3(val) => *val = value,
			Register::R4(val) => *val = value,
			Register::R5(val) => *val = value,
			Register::R6(val) => *val = value,
			Register::R7(val) => *val = value,
			Register::R8(val) => *val = value,
			Register::R9(val) => *val = value,
			Register::R10(val) => *val = value,
			Register::R11(val) => *val = value,
			Register::R12(val) => *val = value,
			Register::SP(val) => val.set(value),
			Register::LR(val) => *val = value,
			Register::PC(val) => *val = value
		};
	}
}


#[derive(PartialEq, Copy, Clone)]
pub struct MSP (pub u32);
#[derive(PartialEq, Copy, Clone)]
pub struct PSP (pub u32);

// Define the SpControl trait with associated methods for get and set
pub trait SpControl {
    fn get(sp: &SP<Self>) -> u32 where Self: Sized;
    fn set(sp: &mut SP<Self>, value: u32) where Self: Sized;
}

#[derive(PartialEq, Copy, Clone)]
// Define the struct for the Stack Pointer Control
pub struct SP<S: SpControl> {
    msp: MSP,
    psp: PSP,
    _state: PhantomData<S>,
}

impl<S: SpControl> SP<S> {
    pub fn new(msp: MSP, psp: PSP) -> Self {
        Self {
            msp,
            psp,
            _state: PhantomData,
        }
    }

    // Generic method for get
    pub fn get(&self) -> u32 {
        S::get(self)
    }

    // Generic method for set
    pub fn set(&mut self, value: u32) {
        S::set(self, value)
    }
}

#[derive(PartialEq, Copy, Clone)]
pub struct SpControlOn;
#[derive(PartialEq, Copy, Clone)]
pub struct SpControlOff;

// Implement the SpControl trait for SpControlOn
impl SpControl for SpControlOn {
    fn get(sp: &SP<Self>) -> u32 {
        sp.msp.0
    }

    fn set(sp: &mut SP<Self>, value: u32) {
        sp.msp.0 = value;
    }
}

// Implement the SpControl trait for SpControlOff
impl SpControl for SpControlOff {
    fn get(sp: &SP<Self>) -> u32 {
        sp.psp.0
    }

    fn set(sp: &mut SP<Self>, value: u32) {
        sp.psp.0 = value;
    }
}
