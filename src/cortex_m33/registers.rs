use std::marker::PhantomData;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CortexM33Registers {
    pub r0: Register,
    pub r1: Register,
    pub r2: Register,
    pub r3: Register,
    pub r4: Register,
    pub r5: Register,
    pub r6: Register,
    pub r7: Register,
    pub r8: Register,
    pub r9: Register,
    pub r10: Register,
    pub r11: Register,
    pub r12: Register,

    pub sp: Register,

    pub lr: Register,

    pub pc: Register,

    pub mode: SpMode,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SpMode {
    Main,
    Process
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
            mode: SpMode::Main
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
            sp: Register::SPRegister(13, Sp::new(0, 0)),
            lr: Register::LrRegister(14, 0),
            pc: Register::PcRegister(15, 0),
            mode: SpMode::Main
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Register {
    // First value is the register number.
    GeneralRegister(u16, u32),
    LrRegister(u16, u32),
    PcRegister(u16, u32),
    SPRegister(u16, Sp),
}

impl Register {
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
            _ => false,
        }
    }

    pub fn is_pc(&self) -> bool {
        match self {
            Register::PcRegister(_, _) => true,
            _ => false,
        }
    }

    pub fn is_sp(&self) -> bool {
        match self {
            Register::SPRegister(_, _) => true,
            _ => false,
        }
    }

    pub fn is_general_register(&self) -> bool {
        match self {
            Register::GeneralRegister(_, _) => true,
            _ => false,
        }
    }
}

