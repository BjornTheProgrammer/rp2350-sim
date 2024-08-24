use crate::registers;
use crate::CortexM33Registers;
use crate::OpCode;
use crate::Register;
use crate::SpControlOn;
use anyhow::{Context, Result};

const KB_OF_RAM: usize = 520;
const MB_OF_FLASH: usize = 4;

const KB: usize = 1024;
const MB: usize = KB * 1024;


pub const FLASH_START_ADDRESS: u32 = 0x10000000;
pub const FLASH_END_ADDRESS: u32 = 0x14000000;
pub const RAM_START_ADDRESS: u32 = 0x20000000;
pub const APB_START_ADDRESS: u32 = 0x40000000;
pub const DPRAM_START_ADDRESS: u32 = 0x50100000;
pub const SIO_START_ADDRESS: u32 = 0xd0000000;

/*
Bus Segment						Base Address
ROM 							0x00000000 // Boot
XIP 							0x10000000 // Flash memory
SRAM 							0x20000000
APB Peripherals 				0x40000000
AHB Peripherals 				0x50000000
Core-local Peripherals (SIO) 	0xd0000000
Cortex-M33 private registers 	0xe0000000
*/
pub struct RP2350<S: registers::SpControl = SpControlOn> {
	// SRAM is partitioned into 10 banks that act like one
	pub sram: [u8; KB_OF_RAM * KB],

	// Has to be on the heap, absolutely blows up the stack
	pub flash: Box<[u8; MB_OF_FLASH * MB]>,

	pub cortex_m33_registers: CortexM33Registers<S>,
}

impl<S: registers::SpControl> RP2350<S> {
	pub fn new() -> Self {
		RP2350 {
			sram: [0; KB_OF_RAM * 1024],
			flash: Box::new([0xff; MB_OF_FLASH * 1024 * 1024]),
			cortex_m33_registers: CortexM33Registers::new(),
		}
	}

	pub fn load_hex(&mut self, source: &str) -> Result<()> {
		for line in source.split('\n') {
			if line.get(..=0) == Some(":") && line.get(7..9) == Some("00") {
				let bytes = line.get(1..3).context("Failed to get bytes from line")?;
				let bytes = usize::from_str_radix(bytes, 16)?;

				let addr = line.get(3..7).context("Failed to get address from line")?;
				let addr = usize::from_str_radix(addr, 16)?;

				for i in 0..bytes {
					let start = 9 + i * 2;
					let value = line.get(start..(start + 2)).context("Failed to get value from bytes")?;
					let value = u8::from_str_radix(value, 16)?;

					self.flash[addr + i] = value;
				}
			}
		}

		Ok(())
	}

	pub fn get_opcode(&self) -> OpCode {
		self.read_from_address(self.cortex_m33_registers.pc.get())
	}

	pub fn execute_instruction(&mut self) {
		let opcode = self.get_opcode();
		opcode.execute(self);
	}

	pub fn read_u32_from_address(&self, address: u32) -> u32 {
		let first_byte = self.read_from_address(address).code;
		let second_byte = self.read_from_address(address + 2).code;

		let result: u32 = ((second_byte as u32) << 16) | (first_byte as u32);

		result
	}

	pub fn read_from_address(&self, address: u32) -> OpCode {
		match address {
			FLASH_START_ADDRESS..RAM_START_ADDRESS => {
				let flash_address = address - FLASH_START_ADDRESS;
				let first_byte = self.flash[flash_address as usize];
				let second_byte = self.flash[flash_address as usize + 1];

				let opcode: u16 = ((first_byte as u16) << 8) | (second_byte as u16);
				
				OpCode {
					code: opcode,
					address
				}
			},
			RAM_START_ADDRESS..APB_START_ADDRESS => {
				let ram_address = address - RAM_START_ADDRESS;
				let first_byte = self.sram[ram_address as usize];
				let second_byte = self.sram[ram_address as usize + 1];

				let opcode: u16 = ((first_byte as u16) << 8) | (second_byte as u16);
				
				OpCode {
					code: opcode,
					address
				}
			},
			_ => unimplemented!("File a github issue and this will get implmeented")
		}
	}

	pub fn write_to_address<V: num_traits::ToBytes + derive_more::LowerHex>(&mut self, address: u32, value: V) {
		match address {
			0x00000000..FLASH_START_ADDRESS => {
				panic!("What the fuck are you doing? ROM is readonly.")
			},
			FLASH_START_ADDRESS..RAM_START_ADDRESS => {
				panic!("What the fuck are you doing? Flash/XIP is readonly.")
			}
			RAM_START_ADDRESS..APB_START_ADDRESS => {
				let bytes = value.to_be_bytes();
				let bytes = bytes.as_ref();

				for (i, byte) in bytes.chunks(2).rev().enumerate() {
					match byte.get(0) {
						Some(val) => {
							self.sram[address as usize - 0x20000000 as usize + i * 2] = *val;
						},
						None => ()
					}

					match byte.get(1) {
						Some(val) => {
							self.sram[address as usize - 0x20000000 as usize + i * 2 + 1] = *val;
						},
						None => ()
					}
				}
			}
			_ => {
				unimplemented!("File a github issue and this will get implmeented")
			}
		}
	}

	pub fn get_register_from_number(&mut self, i: u16) -> &mut Register<S> {
		match i {
			0 => &mut self.cortex_m33_registers.r0,
			1 => &mut self.cortex_m33_registers.r1,
			2 => &mut self.cortex_m33_registers.r2,
			3 => &mut self.cortex_m33_registers.r3,
			4 => &mut self.cortex_m33_registers.r4,
			5 => &mut self.cortex_m33_registers.r5,
			6 => &mut self.cortex_m33_registers.r6,
			7 => &mut self.cortex_m33_registers.r7,
			8 => &mut self.cortex_m33_registers.r8,
			9 => &mut self.cortex_m33_registers.r9,
			10 => &mut self.cortex_m33_registers.r10,
			11 => &mut self.cortex_m33_registers.r11,
			12 => &mut self.cortex_m33_registers.r12,
			13 => &mut self.cortex_m33_registers.sp,
			14 => &mut self.cortex_m33_registers.lr,
			15 => &mut self.cortex_m33_registers.pc,
			_ => { unreachable!() }
		}
	}
}




































