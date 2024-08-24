mod registers;
mod instructions;

pub use registers::{SpControl, SpControlOn, SpControlOff, Register, CortexM33Registers};
pub use instructions::OpCode;

pub struct CortexM33<S: SpControl = SpControlOn> {
	pub registers: CortexM33Registers<S>,
}

impl<S: SpControl> CortexM33<S> {
	pub fn new() -> Self {
		Self {
			registers: CortexM33Registers::new()
		}
	}

	pub fn get_register_from_number(&mut self, i: u16) -> &mut Register<S> {
		match i {
			0 => &mut self.registers.r0,
			1 => &mut self.registers.r1,
			2 => &mut self.registers.r2,
			3 => &mut self.registers.r3,
			4 => &mut self.registers.r4,
			5 => &mut self.registers.r5,
			6 => &mut self.registers.r6,
			7 => &mut self.registers.r7,
			8 => &mut self.registers.r8,
			9 => &mut self.registers.r9,
			10 => &mut self.registers.r10,
			11 => &mut self.registers.r11,
			12 => &mut self.registers.r12,
			13 => &mut self.registers.sp,
			14 => &mut self.registers.lr,
			15 => &mut self.registers.pc,
			_ => { unreachable!() }
		}
	}
}
