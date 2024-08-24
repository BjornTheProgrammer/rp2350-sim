use crate::{registers, rp2350, Register};
use crate::rp2350::RP2350;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct OpCode {
	pub code: u16,
	pub address: u32
}

impl OpCode {
	pub fn execute<S: registers::SpControl>(&self, rp2350: &mut RP2350<S>) {
		let op_code_2 = rp2350.read_from_address(self.address + 2);
		Instruction::new(self, &op_code_2).execute(rp2350);
	}
}

struct Instruction {
	opcode: OpCode,
	instruction: InstructionType
}

#[derive(Debug)]
enum InstructionType {
	Adcs,
	AddRegisterSpPlusImmediate,
	AddSpPlusImmediate,
	AddsT1,
	AddsT2,
	AddsRegister,
	AddRegister,
	Adr,
	AndsT2,
	AsrsImmediate,
	AsrsRegister,
	BWithCondition,
	B,
	Bics,
	Bkpt,
	Bl,
	Blx,
	Bx,
	CmnRegister,
	CmpImmediate,
	CmpRegister,
	CmpRegisterT2,
	CpsidI,
	CpsieI,
	DmbSy,
	DsbSy,
	Eors,
	IsbSy,
	Ldmia,
	LdrImmediate,
	LdrSpPlusImmediate,
	LdrLiteral,
	LdrRegister,
	LdrbImmediate,
	LdrbRegister,
	LdrhImmediate,
	LdrhRegister,
	Ldrsb,
	Ldrsh,
	LslsImmediate,
	LslsRegister,
	LsrsImmediate,
	LsrsRegister,
	Mov,
	Movs,
	Mrs,
	Msr,
	Muls,
	Mvns,
	OrrsT2,
	Pop,
	Push,
	Rev,
	Rev16,
	Revsh,
	Ror,
	Negs,
	Nop,
	SbcsT1,
	Sev,
	Stmia,
	StrImmediate,
	StrSpPlusImmediate,
	StrRegister,
	StrbImmediate,
	StrbRegister,
	StrhImmediate,
	StrhRegister,
	SubSpMinusImmediate,
	SubsT1,
	SubsT2,
	SubsRegister,
	Svc,
	Sxtb,
	Sxth,
	Tst,
	Udf,
	UdfT2,
	Uxtb,
	Uxth,
	Wfe,
	Wfi,
	Yield,
}

use InstructionType::*;

impl Instruction {
	pub fn new(opcode: &OpCode, opcode_2: &OpCode) -> Self {
		let instruction = if opcode.code >> 6 == 0b0100000101 {
			Adcs
		} else if opcode.code >> 11 == 0b10101 {
			AddRegisterSpPlusImmediate
		} else if opcode.code >> 7 == 0b101100000 {
			AddSpPlusImmediate
		} else if opcode.code >> 9 == 0b0001110 {
			AddsT1
		} else if opcode.code >> 11 == 0b00110 {
			AddsT2
		} else if opcode.code >> 9 == 0b0001100 {
			AddsRegister
		} else if opcode.code >> 8 == 0b01000100 {
			AddRegister
		} else if opcode.code >> 11 == 0b10100 {
			Adr
		} else if opcode.code >> 6 == 0b0100000000 {
			AndsT2
		} else if opcode.code >> 11 == 0b00010 {
			AsrsImmediate
		} else if opcode.code >> 6 == 0b0100000100 {
			AsrsRegister
		} else if opcode.code >> 12 == 0b1101 && ((opcode.code >> 9) & 0x7) != 0b111 {
			BWithCondition
		} else if opcode.code >> 11 == 0b11100 {
			B
		} else if opcode.code >> 6 == 0b0100001110 {
			Bics
		} else if opcode.code >> 8 == 0b10111110 {
			Bkpt
		} else if opcode.code >> 11 == 0b11110 && opcode_2.code >> 14 == 0b11 && ((opcode_2.code >> 12) & 0x1) == 1 {
			Bl
		} else if opcode.code >> 7 == 0b010001111 && (opcode.code & 0x7) == 0 {
			Blx
		} else if opcode.code >> 7 == 0b010001110 && (opcode.code & 0x7) == 0 {
			Bx
		} else if opcode.code >> 6 == 0b0100001011 {
			CmnRegister
		} else if opcode.code >> 11 == 0b00101 {
			CmpImmediate
		} else if opcode.code >> 6 == 0b0100001010 {
			CmpRegister
		} else if opcode.code >> 8 == 0b01000101 {
			CmpRegisterT2
		} else if opcode.code == 0xb672 {
			CpsidI
		} else if opcode.code == 0xb662 {
			CpsieI
		} else if opcode.code == 0xf3bf && (opcode_2.code & 0xfff0) == 0x8f50 {
			DmbSy
		} else if opcode.code == 0xf3bf && (opcode_2.code & 0xfff0) == 0x8f40 {
			DsbSy
		} else if opcode.code >> 6 == 0b0100000001 {
			Eors
		} else if opcode.code == 0xf3bf && (opcode_2.code & 0xfff0) == 0x8f60 {
			IsbSy
		} else if opcode.code >> 11 == 0b11001 {
			Ldmia
		} else if opcode.code >> 11 == 0b01101 {
			LdrImmediate
		} else if opcode.code >> 11 == 0b10011 {
			LdrSpPlusImmediate
		} else if opcode.code >> 11 == 0b01001 {
			LdrLiteral
		} else if opcode.code >> 9 == 0b0101100 {
			LdrRegister
		} else if opcode.code >> 11 == 0b01111 {
			LdrbImmediate
		} else if opcode.code >> 9 == 0b0101110 {
			LdrbRegister
		} else if opcode.code >> 11 == 0b10001 {
			LdrhImmediate
		} else if opcode.code >> 9 == 0b0101101 {
			LdrhRegister
		} else if opcode.code >> 9 == 0b0101011 {
			Ldrsb
		} else if opcode.code >> 9 == 0b0101111 {
			Ldrsh
		} else if opcode.code >> 11 == 0b00000 {
			LslsImmediate
		} else if opcode.code >> 6 == 0b0100000010 {
			LslsRegister
		} else if opcode.code >> 11 == 0b00001 {
			LsrsImmediate
		} else if opcode.code >> 6 == 0b0100000011 {
			LsrsRegister
		} else if opcode.code >> 8 == 0b01000110 {
			Mov
		} else if opcode.code >> 11 == 0b00100 {
			Movs
		} else if opcode.code == 0b1111001111101111 && opcode_2.code >> 12 == 0b1000 {
			Mrs
		} else if opcode.code >> 4 == 0b111100111000 && opcode_2.code >> 8 == 0b10001000 {
			Msr
		} else if opcode.code >> 6 == 0b0100001101 {
			Muls
		} else if opcode.code >> 6 == 0b0100001111 {
			Mvns
		} else if opcode.code >> 6 == 0b0100001100 {
			OrrsT2
		} else if opcode.code >> 9 == 0b1011110 {
			Pop
		} else if opcode.code >> 9 == 0b1011010 {
			Push
		} else if opcode.code >> 6 == 0b1011101000 {
			Rev
		} else if opcode.code >> 6 == 0b1011101001 {
			Rev16
		} else if opcode.code >> 6 == 0b1011101011 {
			Revsh
		} else if opcode.code >> 6 == 0b0100000111 {
			Ror
		} else if opcode.code >> 6 == 0b0100001001 {
			Negs
		} else if opcode.code == 0b1011111100000000 {
			Nop
		} else if opcode.code >> 6 == 0b0100000110 {
			SbcsT1
		} else if opcode.code == 0b1011111101000000 {
			Sev
		} else if opcode.code >> 11 == 0b11000 {
			Stmia
		} else if opcode.code >> 11 == 0b01100 {
			StrImmediate
		} else if opcode.code >> 11 == 0b10010 {
			StrSpPlusImmediate
		} else if opcode.code >> 9 == 0b0101000 {
			StrRegister
		} else if opcode.code >> 11 == 0b01110 {
			StrbImmediate
		} else if opcode.code >> 9 == 0b0101010 {
			StrbRegister
		} else if opcode.code >> 11 == 0b10000 {
			StrhImmediate
		} else if opcode.code >> 9 == 0b0101001 {
			StrhRegister
		} else if opcode.code >> 7 == 0b101100001 {
			SubSpMinusImmediate
		} else if opcode.code >> 9 == 0b0001111 {
			SubsT1
		} else if opcode.code >> 11 == 0b00111 {
			SubsT2
		} else if opcode.code >> 9 == 0b0001101 {
			SubsRegister
		} else if opcode.code >> 8 == 0b11011111 {
			Svc
		} else if opcode.code >> 6 == 0b1011001001 {
			Sxtb
		} else if opcode.code >> 6 == 0b1011001000 {
			Sxth
		} else if opcode.code >> 6 == 0b0100001000 {
			Tst
		} else if opcode.code >> 8 == 0b11011110 {
			Udf
		} else if opcode.code >> 4 == 0b111101111111 && opcode_2.code >> 12 == 0b1010 {
			UdfT2
		} else if opcode.code >> 6 == 0b1011001011 {
			Uxtb
		} else if opcode.code >> 6 == 0b1011001010 {
			Uxth
		} else if opcode.code == 0b1011111100100000 {
			Wfe
		} else if opcode.code == 0b1011111100110000 {
			Wfi
		} else if opcode.code == 0b1011111100010000 {
			Yield
		} else {
			unimplemented!("Instruction not implemented, file a github issue.");
		};

		Self {
			opcode: *opcode,
			instruction
		}
	}

	pub fn execute<S: registers::SpControl>(&self, rp2350: &mut RP2350<S>) {
		println!("Instruction: {:?}", self.instruction);
		let opcode_pc = rp2350.cortex_m33_registers.pc.get() & !1;
		let opcode = self.opcode.code;
		let opcode_2 = rp2350.read_from_address(self.opcode.address + 2);

		rp2350.cortex_m33_registers.pc.set(rp2350.cortex_m33_registers.pc.get() + 2);

		match self.instruction {
			Adcs => {},
			AddRegisterSpPlusImmediate => {
				let imm8: u32 = opcode as u32 & 0xff;
				let rd = (opcode >> 8) & 0x7;

				let sp = rp2350.cortex_m33_registers.sp.get();
				rp2350.get_register_from_number(rd).set(sp + (imm8 << 2))
			},
			AddSpPlusImmediate => {
				let imm32 = (opcode as u32 & 0x7f) << 2;
				rp2350.cortex_m33_registers.sp.set(rp2350.cortex_m33_registers.sp.get() + imm32);
			},
			AddsT1 => {
				let opcode = opcode as u32;

				let _imm3 = (opcode >> 6) & 0x7;
				let _rn = (opcode >> 3) & 0x7;
				let _rd = opcode & 0x7;

				todo!();
			},
			AddsT2 => {
				todo!();
			},
			AddsRegister => {
				todo!();
			},
			AddRegister => {
				let rdn = ((opcode & 0x80) >> 4) | (opcode & 0x7);
				let left_value = {
					let rdn = rp2350.get_register_from_number(rdn);
					if rdn.is_pc() { rp2350.cortex_m33_registers.pc.get() + 2 } else { rdn.get() }
				};

				let rm = (opcode >> 3) & 0xf;
				let right_value = rp2350.get_register_from_number(rm).get();

				let result = left_value + right_value;

				let rdn = rp2350.get_register_from_number(rdn);

				if !rdn.is_sp() && !rdn.is_pc() {
					rdn.set(result);
				} else if rdn.is_pc() {
					rdn.set(result & !0x1)
				} else if rdn.is_sp() {
					rdn.set(result & !0x3)
				}
			},
			Adr => {
				let imm8 = opcode as u32 & 0xff;
				let rd = (opcode >> 8) & 0x7;

				rp2350.get_register_from_number(rd).set((opcode_pc & 0xfffffffc) + 4 + (imm8 << 2));
			},
			AndsT2 => {
				todo!();
			},
			AsrsImmediate => {
				todo!();
			},
			AsrsRegister => {
				todo!();
			},
			BWithCondition => {
				todo!();
			},
			B => {
				let opcode = opcode as i32;
				let mut imm11 = (opcode & 0x7ff) << 1;
				if imm11 & (1 << 11) > 0 {
					imm11 = (imm11 & 0x7ff) - 0x800;
				}

				let pc_value = rp2350.cortex_m33_registers.pc.get() as i32;
				let value = pc_value + imm11 + 2;
				
				rp2350.cortex_m33_registers.pc.set(value as u32);
			},
			Bics => {
				todo!();
			},
			Bkpt => {
				todo!();
			},
			Bl => {
				let opcode = opcode as i32;
				let opcode_2 = opcode_2.code as i32;

				let imm11 = opcode_2 & 0x7ff;
				let j2 = (opcode_2 >> 11) & 0x1;
				let j1 = (opcode_2 >> 13) & 0x1;
				let imm10 = opcode & 0x3ff;
				let s = (opcode >> 10) & 0x1;
				let i1 = 1 - (s ^ j1);
				let i2 = 1 - (s ^ j2);

				let s = if s > 0 {
					0b11111111
				} else {
					0
				};

				let imm32: i32 = (s << 24) | ((i1 << 23) | (i2 << 22) | (imm10 << 12) | (imm11 << 1));
				rp2350.cortex_m33_registers.lr.set(rp2350.cortex_m33_registers.pc.get() + 2 | 0x1);

				let pc_value = rp2350.cortex_m33_registers.pc.get() as i32 + 2 + imm32;
				rp2350.cortex_m33_registers.pc.set(pc_value as u32);
			},
			Blx => {
				let rm = (opcode >> 3) & 0xf;
				rp2350.cortex_m33_registers.lr.set(rp2350.cortex_m33_registers.pc.get() | 0x1);
				let rm_value = rp2350.get_register_from_number(rm).get();
				rp2350.cortex_m33_registers.pc.set(rm_value & !1);
			},
			Bx => {
				todo!();
			},
			CmnRegister => {
				todo!();
			},
			CmpImmediate => {
				todo!();
			},
			CmpRegister => {
				todo!();
			},
			CmpRegisterT2 => {
				todo!();
			},
			CpsidI => {
				todo!();
			},
			CpsieI => {
				todo!();
			},
			DmbSy => {
				rp2350.cortex_m33_registers.pc.set(rp2350.cortex_m33_registers.pc.get() + 2);
			},
			DsbSy => {
				rp2350.cortex_m33_registers.pc.set(rp2350.cortex_m33_registers.pc.get() + 2);
			},
			Eors => {},
			IsbSy => {
				rp2350.cortex_m33_registers.pc.set(rp2350.cortex_m33_registers.pc.get() + 2);
			},
			Ldmia => {
				let rn = (opcode >> 8) & 0x7;
				let registers = opcode & 0xff;
				let mut address = rp2350.get_register_from_number(rn).get();
				for i in 0..8 {
					if registers & (1 << i) > 0 {
						let address_value = rp2350.read_u32_from_address(address);
						rp2350.get_register_from_number(i).set(address_value);
						address += 4;
					}
				}

				// Write back
				if !(registers & (1 << rn) > 0) {
					rp2350.get_register_from_number(rn).set(address);
				}
			},
			LdrImmediate => {
				todo!();
			},
			LdrSpPlusImmediate => {
				todo!();
			},
			LdrLiteral => {
				todo!();
			},
			LdrRegister => {
				todo!();
			},
			LdrbImmediate => {
				todo!();
			},
			LdrbRegister => {
				todo!();
			},
			LdrhImmediate => {
				todo!();
			},
			LdrhRegister => {
				todo!();
			},
			Ldrsb => {
				todo!();
			},
			Ldrsh => {
				todo!();
			},
			LslsImmediate => {
				todo!();
			},
			LslsRegister => {
				todo!();
			},
			LsrsImmediate => {
				todo!();
			},
			LsrsRegister => {
				todo!();
			},
			Mov => {
				let rm = (opcode >> 3) & 0xf;
				let rd = ((opcode >> 4) & 0x8) | (opcode & 0x7);

				let rm = rp2350.get_register_from_number(rm);
				let mut value = if rm.is_pc() { rp2350.cortex_m33_registers.pc.get() + 2 } else { rm.get() };

				let rd = rp2350.get_register_from_number(rd);

				if rd.is_pc() {
					value &= !1;
				} else if rd.is_sp() {
					value &= !3;
				}

				rd.set(value);
			},
			Movs => {
				todo!();
			},
			Mrs => {
				todo!();
			},
			Msr => {
				todo!();
			},
			Muls => {
				todo!();
			},
			Mvns => {
				todo!();
			},
			OrrsT2 => {
				todo!();
			},
			Pop => {
				todo!();
			},
			Push => {
				let mut bitcount = 0;
				for i in 0..=8 {
					if self.opcode.code & (1 << i) > 0 {
						bitcount += 1;
					}
				}

				let mut address = rp2350.cortex_m33_registers.sp.get() - 4 * bitcount;

				for i in 0..=7 {
					if self.opcode.code & (1 << i) > 0 {
						let register = rp2350.get_register_from_number(i).get();

						rp2350.write_to_address(address, register as u8);
						address += 4;
					}
				}


				if self.opcode.code & (1 << 8) > 0 {
					rp2350.write_to_address(address, rp2350.cortex_m33_registers.lr.get() as u8);
				}

				let current_sp = rp2350.cortex_m33_registers.sp.get();
				rp2350.cortex_m33_registers.sp.set(current_sp - 4 * bitcount);
			},
			Rev => {
				let rm = (opcode >> 3) & 0x7;
				let rd = opcode & 0x7;
				let input = rp2350.get_register_from_number(rm).get();
				rp2350.get_register_from_number(rd).set(
					((input & 0xff) << 24) |
					(((input >> 8) & 0xff) << 16) |
					(((input >> 16) & 0xff) << 8) |
					((input >> 24) & 0xff)
				)
			},
			Rev16 => {
				let rm = (opcode >> 3) & 0x7;
				let rd = opcode & 0x7;
				let input = rp2350.get_register_from_number(rm).get();
				rp2350.get_register_from_number(rd).set(
					(((input >> 16) & 0xff) << 24) |
					(((input >> 24) & 0xff) << 16) |
					((input & 0xff) << 8) |
					((input >> 8) & 0xff)
				)
			},
			Revsh => {
				todo!();
			},
			Ror => {
				todo!();
			},
			Negs => {
				todo!();
			},
			Nop => {
				// Do nothing
			},
			SbcsT1 => {},
			Sev => {},
			Stmia => {
				let rn = (opcode >> 8) & 0x7;
				let registers = opcode & 0xff;
				let mut address = rp2350.get_register_from_number(rn).get();
				for i in 0..8 {
					if registers & (1 << i) > 0 {
						let register_value = rp2350.get_register_from_number(i).get();
						rp2350.write_to_address(address, register_value);
						address += 4;
					}
				}
				// Write back
				if !(registers & (1 << rn) > 0) {
					rp2350.get_register_from_number(rn).set(address);
				}
			},
			StrImmediate => {
				todo!();
			},
			StrSpPlusImmediate => {
				todo!();
			},
			StrRegister => {
				todo!();
			},
			StrbImmediate => {
				todo!();
			},
			StrbRegister => {
				todo!();
			},
			StrhImmediate => {
				todo!();
			},
			StrhRegister => {
				todo!();
			},
			SubSpMinusImmediate => {
				let imm32 = (opcode & 0x7f) << 2;
				rp2350.cortex_m33_registers.sp.set(rp2350.cortex_m33_registers.sp.get() - imm32 as u32);
			},
			SubsT1 => {
				todo!();
			},
			SubsT2 => {
				todo!();
			},
			SubsRegister => {
				todo!();
			},
			Svc => {
				todo!();
			},
			Sxtb => {
				todo!();
			},
			Sxth => {
				todo!();
			},
			Tst => {
				todo!();
			},
			Udf => {
				todo!();
			},
			UdfT2 => {
				todo!();
			},
			Uxtb => {
				let rm = (opcode >> 3) & 0x7;
				let rd = opcode & 0x7;
				let value = rp2350.get_register_from_number(rm).get() & 0xff;
				rp2350.get_register_from_number(rd).set(value);
			},
			Uxth => {
				let rm = (opcode >> 3) & 0x7;
				let rd = opcode & 0x7;
				let value = rp2350.get_register_from_number(rm).get() & 0xffff;
				rp2350.get_register_from_number(rd).set(value);
			},
			Wfe => {
				todo!();
			},
			Wfi => {
				todo!();
			},
			Yield => {
				// Do nothing, wait for an event
			},
		}
	}
}
