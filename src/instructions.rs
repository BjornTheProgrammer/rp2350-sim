use crate::registers;
use derive_more::{Add, From, Into};

use crate::rp2350::RP2350;

#[derive(PartialEq, From, Into, Add, Copy, Clone, Debug)]
pub struct OpCode(pub u16);

impl OpCode {
	pub fn execute<S: registers::SpControl>(&self, rp2350: &mut RP2350<S>) {
		Instruction::new(self).execute(rp2350);
	}
}

struct Instruction {
	opcode: OpCode,
	instruction: InstructionType
}

enum InstructionType {
	Push
}

impl Instruction {
	pub fn new(opcode: &OpCode) -> Self {
		let instruction = if opcode.0 >> 9 == 0b1011010 {
			InstructionType::Push
		} else {
			unimplemented!("Instruction not implemented, file a github issue.")
		};

		Self {
			opcode: *opcode,
			instruction
		}
	}

	pub fn execute<S: registers::SpControl>(&self, rp2350: &mut RP2350<S>) {
		match self.instruction {
			InstructionType::Push => {
				let mut bitcount = 0;
				for i in 0..=8 {
					if self.opcode.0 & (1 << i) > 0 {
						bitcount += 1;
					}
				}

				let mut address = rp2350.cortex_m33_registers.sp.get() - 4 * bitcount;

				for i in 0..=7 {
					if self.opcode.0 & (1 << i) > 0 {
						let register = match i {
							0 => rp2350.cortex_m33_registers.r0.get(),
							1 => rp2350.cortex_m33_registers.r1.get(),
							2 => rp2350.cortex_m33_registers.r2.get(),
							3 => rp2350.cortex_m33_registers.r3.get(),
							4 => rp2350.cortex_m33_registers.r4.get(),
							5 => rp2350.cortex_m33_registers.r5.get(),
							6 => rp2350.cortex_m33_registers.r6.get(),
							7 => rp2350.cortex_m33_registers.r7.get(),
							_ => { unreachable!() }
						};

						rp2350.write_to_address(address, register as u8);
						address += 4;
					}
				}


				if self.opcode.0 & (1 << 8) > 0 {
					rp2350.write_to_address(address, rp2350.cortex_m33_registers.lr.get() as u8);
				}

				let current_sp = rp2350.cortex_m33_registers.sp.get();
				rp2350.cortex_m33_registers.sp.set(current_sp - 4 * bitcount);
			}
		}
	}
}
