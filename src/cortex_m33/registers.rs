use std::ops::{Index, IndexMut};

pub trait Register {
    fn get(&self) -> u32;
    fn set(&mut self, value: u32);
    fn number(&self) -> u16;

    fn is_lr(&self) -> bool {
        false
    }

    fn is_pc(&self) -> bool {
        false
    }

    fn is_sp(&self) -> bool {
        false
    }

    fn is_general_register(&self) -> bool {
        false
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct GeneralRegister(u16, u32);
impl Register for GeneralRegister {
    fn get(&self) -> u32 {
        self.1
    }

    fn set(&mut self, value: u32) {
        self.1 = value;
    }

    fn number(&self) -> u16 {
        self.0
    }

    fn is_general_register(&self) -> bool {
        true
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SPRegister(u16, Sp);
impl SPRegister {
    pub fn get_msp(&self) -> u32 {
        self.1.msp
    }

    pub fn get_psp(&self) -> u32 {
        self.1.psp
    }

    pub fn set_mode(&mut self, mode: SpMode) {
        self.1.mode = mode;
    }

    pub fn get_mode(&mut self) -> SpMode {
        self.1.mode
    }
}

impl Register for SPRegister {
    fn get(&self) -> u32 {
        match self.1.mode {
            SpMode::Main => self.1.msp,
            SpMode::Process => self.1.psp,
        }
    }

    fn set(&mut self, value: u32) {
        match self.1.mode {
            SpMode::Main => self.1.msp = value,
            SpMode::Process => self.1.psp = value,
        }
    }

    fn number(&self) -> u16 {
        self.0
    }

    fn is_sp(&self) -> bool {
        true
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct LrRegister(u16, u32);
impl Register for LrRegister {
    fn get(&self) -> u32 {
        self.1
    }

    fn set(&mut self, value: u32) {
        self.1 = value;
    }

    fn number(&self) -> u16 {
        self.0
    }

    fn is_lr(&self) -> bool {
        true
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PcRegister(u16, u32);
impl Register for PcRegister {
    fn get(&self) -> u32 {
        self.1
    }

    fn set(&mut self, value: u32) {
        self.1 = value;
    }

    fn number(&self) -> u16 {
        self.0
    }

    fn is_pc(&self) -> bool {
        true
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CortexM33Registers {
    pub r0: GeneralRegister,
    pub r1: GeneralRegister,
    pub r2: GeneralRegister,
    pub r3: GeneralRegister,
    pub r4: GeneralRegister,
    pub r5: GeneralRegister,
    pub r6: GeneralRegister,
    pub r7: GeneralRegister,
    pub r8: GeneralRegister,
    pub r9: GeneralRegister,
    pub r10: GeneralRegister,
    pub r11: GeneralRegister,
    pub r12: GeneralRegister,

    pub sp: SPRegister,

    pub lr: LrRegister,

    pub pc: PcRegister,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SpMode {
    Main,
    Process,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Sp {
    pub msp: u32,
    pub psp: u32,
    pub mode: SpMode,
}

impl Sp {
    pub fn new(msp: u32, psp: u32) -> Self {
        Self {
            msp,
            psp,
            mode: SpMode::Main,
        }
    }

    pub fn change_mode(&mut self) {
        match self.mode {
            SpMode::Main => self.mode = SpMode::Process,
            SpMode::Process => self.mode = SpMode::Main,
        }
    }

    pub fn get(&self) -> u32 {
        match self.mode {
            SpMode::Main => self.msp,
            SpMode::Process => self.psp,
        }
    }

    pub fn set(&mut self, value: u32) {
        match self.mode {
            SpMode::Main => self.msp = value,
            SpMode::Process => self.psp = value,
        }
    }
}

impl CortexM33Registers {
    pub fn new() -> Self {
        Self {
            r0: GeneralRegister(0, 0),
            r1: GeneralRegister(1, 0),
            r2: GeneralRegister(2, 0),
            r3: GeneralRegister(3, 0),
            r4: GeneralRegister(4, 0),
            r5: GeneralRegister(5, 0),
            r6: GeneralRegister(6, 0),
            r7: GeneralRegister(7, 0),
            r8: GeneralRegister(8, 0),
            r9: GeneralRegister(9, 0),
            r10: GeneralRegister(10, 0),
            r11: GeneralRegister(11, 0),
            r12: GeneralRegister(12, 0),
            sp: SPRegister(13, Sp::new(0, 0)),
            lr: LrRegister(14, 0),
            pc: PcRegister(15, 0),
        }
    }
}

impl Index<usize> for CortexM33Registers {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r0.1,
            1 => &self.r1.1,
            2 => &self.r2.1,
            3 => &self.r3.1,
            4 => &self.r4.1,
            5 => &self.r5.1,
            6 => &self.r6.1,
            7 => &self.r7.1,
            8 => &self.r8.1,
            9 => &self.r9.1,
            10 => &self.r10.1,
            11 => &self.r11.1,
            12 => &self.r12.1,
            _ => panic!("Specified register of '{}' does not exist", index),
        }
    }
}

impl IndexMut<usize> for CortexM33Registers {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r0.1,
            1 => &mut self.r1.1,
            2 => &mut self.r2.1,
            3 => &mut self.r3.1,
            4 => &mut self.r4.1,
            5 => &mut self.r5.1,
            6 => &mut self.r6.1,
            7 => &mut self.r7.1,
            8 => &mut self.r8.1,
            9 => &mut self.r9.1,
            10 => &mut self.r10.1,
            11 => &mut self.r11.1,
            12 => &mut self.r12.1,
            _ => panic!("Specified register of '{}' does not exist", index),
        }
    }
}
