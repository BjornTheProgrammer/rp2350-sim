use std::any::Any;

use crate::cortex_m33::registers::Register;
use crate::cortex_m33::{CortexM33, OpCode};
use crate::MemoryInterface;
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
pub struct RP2350Memory {
    // SRAM is partitioned into 10 banks that act like one
    pub sram: [u8; KB_OF_RAM * KB],

    // Has to be on the heap, absolutely blows up the stack
    pub flash: Box<[u8; MB_OF_FLASH * MB]>,
}

impl RP2350Memory {
    pub fn new() -> Self {
        Self {
            sram: [0; KB_OF_RAM * 1024],
            // The box has to be defined like this, cuz for whatever reason this causes stack overflows still in tests. https://github.com/rust-lang/rust/issues/53827
            flash: vec![0xff; MB_OF_FLASH * 1024 * 1024]
                .into_boxed_slice()
                .try_into()
                .unwrap(),
        }
    }
}

pub struct RP2350 {
    pub cortex_m33: CortexM33,
}

impl<'a> MemoryInterface<u32> for RP2350Memory {
    fn read(&self, address: u32) -> u8 {
        match address {
            FLASH_START_ADDRESS..RAM_START_ADDRESS => {
                let flash_address = address - FLASH_START_ADDRESS;
                self.flash[flash_address as usize]
            }
            RAM_START_ADDRESS..APB_START_ADDRESS => {
                let ram_address = address - RAM_START_ADDRESS;
                self.sram[ram_address as usize]
            }
            _ => unimplemented!("File a github issue and this will get implmented"),
        }
    }

    fn write(&mut self, address: u32, value: u8) {
        match address {
            0x00000000..FLASH_START_ADDRESS => {
                panic!("What the fuck are you doing? ROM is readonly.")
            }
            FLASH_START_ADDRESS..RAM_START_ADDRESS => {
                panic!("What the fuck are you doing? Flash/XIP is readonly.")
            }
            RAM_START_ADDRESS..APB_START_ADDRESS => {
                self.sram[address as usize - 0x20000000 as usize] = value;
            }
            _ => {
                unimplemented!("File a github issue and this will get implmeented")
            }
        }   
    }

    fn as_any(&self) -> &(dyn Any + 'static) {
        self
    }
}


impl RP2350 {
    pub fn new() -> Self {
        let memory = RP2350Memory::new();
        let cortex = CortexM33::new(Box::new(memory));
        RP2350 {
            cortex_m33: cortex
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
                    let value = line
                        .get(start..(start + 2))
                        .context("Failed to get value from bytes")?;
                    let value = u8::from_str_radix(value, 16)?;

                    self.cortex_m33.memory.write((addr + i) as u32, value);
                }
            }
        }

        Ok(())
    }

    pub fn get_opcode(&self) -> OpCode {
        let address = self.cortex_m33.registers.pc.get();
        OpCode::from_address(&self.cortex_m33, address)
    }

    pub fn execute_instruction(&mut self) {
        let opcode = self.get_opcode();
        opcode.execute(&mut self.cortex_m33);
    }
}
