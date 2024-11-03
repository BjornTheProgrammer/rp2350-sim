use std::{collections::HashMap, ops::{Index, IndexMut}};
use crate::cortex_m33::operation::get_bits;

use super::CortexM33;

#[derive(Debug, Clone, Copy)]
pub enum InterruptException {
    PendSV,
    SysTick,
    ExternalInterrupt(u8)
}

#[derive(Debug, Clone, Copy)]
pub enum Exception {
    Reset,
    NMI,
    HardFault,
    SVCall,
    Interrupt(InterruptException)
}

impl Exception {
    pub fn number(&self) -> u8 {
        match self {
            Exception::Reset => 1,
            Exception::NMI => 2,
            Exception::HardFault => 3,
            Exception::SVCall => 11,
            Exception::Interrupt(interrupt) => match interrupt {
                InterruptException::PendSV => 14,
                InterruptException::SysTick => 15,
                InterruptException::ExternalInterrupt(val) => 16 + val,
            },
        }
    }
}

pub struct Exceptions {
    pub active: HashMap<u8, Exception>
}

impl Exceptions {
    pub fn new() -> Self {
        Self {
            active: HashMap::new()
        }
    }

    pub fn priority(cortex: &CortexM33, n: u8) -> i8 {
        assert!(n >= 1 && n <= 48);

        let result = if n == Exception::Reset.number() {
            -3
        } else if n == Exception::NMI.number() {
            -2
        } else if n == Exception::HardFault.number() {
            -1
        } else if n == Exception::SVCall.number() {
            cortex.shpr.pri_11() as i8
        } else if n == Exception::Interrupt(InterruptException::PendSV).number() {
            cortex.shpr.pri_14() as i8
        } else if n == Exception::Interrupt(InterruptException::SysTick).number() {
            cortex.shpr.pri_15() as i8
        } else if n >= 16 {
            let r = ((n - 16) / 4) as usize;
            let v = n as usize % 4;
            let v = v * 8;

            let bits = get_bits(cortex.nvic[r], (v + 6)..=(v + 7)) as i8;

            bits
        } else {
            4
        };

        assert!(-3 <= result && result <= 4);

        result
    }
}
