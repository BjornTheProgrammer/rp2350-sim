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
			r0: Register::GeneralRegister(0, 0),
			r1: Register::GeneralRegister(1, 0),
			r2: Register::GeneralRegister(2, 0),
			r3: Register::GeneralRegister(3, 0),
			r4: Register::GeneralRegister(4, 0),
			r5: Register::GeneralRegister(5, 0),
			r6: Register::GeneralRegister(6, 0),
			r7: Register::GeneralRegister(7, 0),
			r8: Register::GeneralRegister(8, 0),
			r9: Register::GeneralRegister(9, 0),
			r10: Register::GeneralRegister(10, 0),
			r11: Register::GeneralRegister(11, 0),
			r12: Register::GeneralRegister(12, 0),
			sp: Register::SPRegister(13, SP::new(MSP(0), PSP(0))),
			lr: Register::LrRegister(14, 0),
			pc: Register::PcRegister(15, 0)
		}
	}
}

#[derive(PartialEq, Copy, Clone)]
pub enum Register<S: SpControl> {
	// First value is the register number.
	GeneralRegister(u16, u32),
	LrRegister(u16, u32),
	PcRegister(u16, u32),
	SPRegister(u16, SP<S>),
}

impl<S: SpControl> Register<S> {
	pub fn get(&self) -> u32 {
		match self {
			Register::GeneralRegister(_, val) => *val,
			Register::LrRegister(_, val) => *val,
			Register::PcRegister(_, val) => *val,
			Register::SPRegister(_, val) => val.get(),
		}
	}

	pub fn set(&mut self, value: u32) {
		match self {
			Register::GeneralRegister(_, val) => *val = value,
			Register::LrRegister(_, val) => *val = value,
			Register::PcRegister(_, val) => *val = value,
			Register::SPRegister(_, val) => val.set(value),
		};
	}

	pub fn number(&self) -> u16 {
		match self {
			Register::GeneralRegister(val, _) => *val,
			Register::LrRegister(val, _) => *val,
			Register::PcRegister(val, _) => *val,
			Register::SPRegister(val, _) => *val,
		}
	}

	pub fn is_lr(&self) -> bool {
		match self {
			Register::LrRegister(_, _) => true,
			_ => false
		}
	}

	pub fn is_pc(&self) -> bool {
		match self {
			Register::PcRegister(_, _) => true,
			_ => false
		}
	}

	pub fn is_sp(&self) -> bool {
		match self {
			Register::SPRegister(_, _) => true,
			_ => false
		}
	}

	pub fn is_general_register(&self) -> bool {
		match self {
			Register::GeneralRegister(_, _) => true,
			_ => false
		}
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
