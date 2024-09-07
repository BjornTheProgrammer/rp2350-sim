mod apsr;
mod instructions;
pub mod opcodes;
mod operation;
pub mod registers;
pub mod exception;

use std::collections::HashMap;

use crate::cortex_m33::apsr::Apsr;
use crate::cortex_m33::registers::{CortexM33Registers, Register, SpControl, SpControlOn};
pub use instructions::OpCode;

use self::exception::Exceptions;
use self::operation::get_bits;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Thread,
    Handler
}

pub struct Shpr {
    shpr2: u32,
    shpr3: u32,
}

impl Shpr {
    pub fn new() -> Self {
        Self {
            shpr2: 0,
            shpr3: 0
        }
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

pub struct Nvic {
    ipr0: u32,
    ipr1: u32,
    ipr2: u32,
    ipr3: u32,
    ipr4: u32,
    ipr5: u32,
    ipr6: u32,
    ipr7: u32,
}

impl Nvic {
    pub fn new() -> Self {
        Self {
            ipr0: 0,
            ipr1: 0,
            ipr2: 0,
            ipr3: 0,
            ipr4: 0,
            ipr5: 0,
            ipr6: 0,
            ipr7: 0,
        }
    }

    pub fn write(ipr_number: u8, value: u32) {
        assert!(ipr_number <= 7);
        match ipr_number {

            _ => unreachable!()
        }
    }
}

pub struct CortexM33<S: SpControl = SpControlOn> {
    pub registers: CortexM33Registers<S>,
    pub apsr: Apsr,
    pub mode: Mode,
    pub ipsr: u8,
    pub exceptions: Exceptions,
    pub shpr: Shpr,
    pub nvic: Nvic
}

impl<S: SpControl> CortexM33<S> {
    pub fn deafult_exceptions() {

    }

    pub fn new() -> Self {
        Self {
            registers: CortexM33Registers::new(),
            apsr: Apsr::new(),
            mode: Mode::Thread,
            ipsr: 0,
            exceptions: Exceptions::new(),
            shpr: Shpr::new(),
            nvic: Nvic::new()
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
            _ => {
                unreachable!()
            }
        }
    }
}
