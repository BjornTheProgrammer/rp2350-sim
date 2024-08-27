use crate::cortex_m33::registers;
use crate::cortex_m33::Register;

pub struct Registers<'a, T: registers::SpControl> {
    register_numbers: &'a [Register<T>],
}

impl<T: registers::SpControl> Registers<'_, T> {
    pub fn binary(&self) -> u16 {
        let mut binary = 0;
        for number in self.register_numbers.iter() {
            binary = binary | (1 << number.number());
        }

        return binary;
    }
}

// Implementing `From<&'a [u8]>` for `Registers<'a>`
impl<'a, T: registers::SpControl> From<&'a [Register<T>]> for Registers<'a, T> {
    fn from(array: &'a [Register<T>]) -> Self {
        Registers {
            register_numbers: array,
        }
    }
}

// Implementing `Into<&'a [u8]>` for `Registers<'a>`
impl<'a, T: registers::SpControl> Into<&'a [Register<T>]> for Registers<'a, T> {
    fn into(self) -> &'a [Register<T>] {
        self.register_numbers
    }
}

pub struct PushT1;
impl PushT1 {
    pub fn opcode<T: registers::SpControl>(push_to_lr: bool, registers: Registers<T>) -> u16 {
        return (0b1011010 << 9) | ((push_to_lr as u16) << 8) | registers.binary();
    }
}

pub struct AdcT1;
impl AdcT1 {
    pub fn opcode<T: registers::SpControl, U: registers::SpControl>(
        rdn: Register<T>,
        rm: Register<U>,
    ) -> u16 {
        return (0b0100000101 << 6) | ((rm.number() & 7) << 3) | (rdn.number() & 7);
    }
}

pub struct AddSpPlusImmediateT2;
impl AddSpPlusImmediateT2 {
    pub fn opcode(imm: u16) -> u16 {
        return (0b101100000 << 7) | ((imm >> 2) & 0x7f);
    }
}

pub struct AddSpPlusImmediateT1;
impl AddSpPlusImmediateT1 {
    // rd is the register number
    pub fn opcode<T: registers::SpControl>(rd: Register<T>, imm8: u16) -> u16 {
        return (0b10101 << 11) | ((rd.number() & 7) << 8) | ((imm8 >> 2) & 0xff);
    }
}

pub struct AddsT1;
impl AddsT1 {
    pub fn opcode<T: registers::SpControl, U: registers::SpControl>(
        rd: Register<T>,
        rn: Register<U>,
        imm3: u16,
    ) -> u16 {
        return (0b0001110 << 9)
            | ((imm3 & 0x7) << 6)
            | ((rn.number() & 7) << 3)
            | (rd.number() & 7);
    }
}

pub struct AddsT2;
impl AddsT2 {
    pub fn opcode<T: registers::SpControl>(rdn: Register<T>, imm8: u16) -> u16 {
        return (0b00110 << 11) | ((rdn.number() & 7) << 8) | (imm8 & 0xff);
    }
}

pub struct AddsRegisterT1;
impl AddsRegisterT1 {
    pub fn opcode<T: registers::SpControl, U: registers::SpControl, V: registers::SpControl>(
        rd: Register<T>,
        rn: Register<U>,
        rm: Register<V>,
    ) -> u16 {
        return (0b0001100 << 9)
            | ((rm.number() & 0x7) << 6)
            | ((rn.number() & 7) << 3)
            | (rd.number() & 7);
    }
}

pub struct AddRegisterT2;
impl AddRegisterT2 {
    pub fn opcode<T: registers::SpControl, U: registers::SpControl>(
        rdn: Register<T>,
        rm: Register<U>,
    ) -> u16 {
        return (0b01000100 << 8)
            | ((rdn.number() & 0x8) << 4)
            | ((rm.number() & 0xf) << 3)
            | (rdn.number() & 0x7);
    }
}

pub struct AdrT1;
impl AdrT1 {
    pub fn opcode<T: registers::SpControl>(rd: Register<T>, imm8: u16) -> u16 {
        return (0b10100 << 11) | ((rd.number() & 7) << 8) | ((imm8 >> 2) & 0xff);
    }
}

pub struct AndRegisterT1;
impl AndRegisterT1 {
    pub fn opcode<T: registers::SpControl, U: registers::SpControl>(
        rn: Register<T>,
        rm: Register<U>,
    ) -> u16 {
        return (0b0100000000 << 6) | ((rm.number() & 7) << 3) | (rn.number() & 0x7);
    }
}

pub struct BT2;
impl BT2 {
    pub fn opcode(imm11: u16) -> u16 {
        return (0b11100 << 11) | ((imm11 >> 1) & 0x7ff);
    }
}

pub struct BT1;
impl BT1 {
    pub fn opcode(cond: u16, imm8: u16) -> u16 {
        return (0b1101 << 12) | ((cond & 0xf) << 8) | ((imm8 >> 1) & 0x1ff);
    }
}

pub struct DmbT1Sy;
impl DmbT1Sy {
    pub fn opcode() -> u32 {
        return 0x8f50f3bf;
    }
}

pub struct DsbT1Sy;
impl DsbT1Sy {
    pub fn opcode() -> u32 {
        return 0x8f4ff3bf;
    }
}

pub struct IsbT1Sy;
impl IsbT1Sy {
    pub fn opcode() -> u32 {
        return 0x8f6ff3bf;
    }
}

pub struct MovRegisterT1;
impl MovRegisterT1 {
    pub fn opcode<T: registers::SpControl, U: registers::SpControl>(
        rd: Register<T>,
        rm: Register<U>,
    ) -> u16 {
        let bit = if rd.number() & 0x8 > 0 { 1 } else { 0 };

        return (0b01000110 << 8) | (bit << 7) | (rm.number() << 3) | (rd.number() & 0x7);
    }
}

pub struct LdmiaT1;
impl LdmiaT1 {
    pub fn opcode<T: registers::SpControl>(rn: Register<T>, registers: Registers<T>) -> u16 {
        return (0b11001 << 11) | ((rn.number() & 0x7) << 8) | (registers.binary() & 0xff);
    }
}

pub struct RevT1;
impl RevT1 {
    pub fn opcode<T: registers::SpControl, U: registers::SpControl>(
        rd: Register<T>,
        rn: Register<U>,
    ) -> u16 {
        return (0b1011101000 << 6) | ((rn.number() & 0x7) << 3) | (rd.number() & 0x7);
    }
}

pub struct Rev16T1;
impl Rev16T1 {
    pub fn opcode<T: registers::SpControl, U: registers::SpControl>(
        rd: Register<T>,
        rn: Register<U>,
    ) -> u16 {
        return (0b1011101001 << 6) | ((rn.number() & 0x7) << 3) | (rd.number() & 0x7);
    }
}

pub struct StmiaT1;
impl StmiaT1 {
    pub fn opcode<T: registers::SpControl>(rn: Register<T>, registers: Registers<T>) -> u16 {
        return (0b11000 << 11) | ((rn.number() & 0x7) << 8) | (registers.binary() & 0xff);
    }
}

pub struct SubSpMinusImmediateT1;
impl SubSpMinusImmediateT1 {
    pub fn opcode(imm: u16) -> u16 {
        return (0b101100001 << 7) | ((imm >> 2) & 0x7f);
    }
}

pub struct UxtbT1;
impl UxtbT1 {
    pub fn opcode<T: registers::SpControl, U: registers::SpControl>(
        rd: Register<T>,
        rm: Register<U>,
    ) -> u16 {
        return (0b1011001011 << 6) | ((rm.number() & 7) << 3) | (rd.number() & 7);
    }
}

pub struct UxthT1;
impl UxthT1 {
    pub fn opcode<T: registers::SpControl, U: registers::SpControl>(
        rd: Register<T>,
        rm: Register<U>,
    ) -> u16 {
        return (0b1011001010 << 6) | ((rm.number() & 7) << 3) | (rd.number() & 7);
    }
}

pub struct YieldT1;
impl YieldT1 {
    pub fn opcode() -> u16 {
        return 0b1011111100010000;
    }
}

pub struct BlT1;
impl BlT1 {
    pub fn opcode(imm: i32) -> u32 {
        let imm11 = (imm >> 1) & 0x7ff;
        let imm10 = (imm >> 12) & 0x3ff;
        let s = if imm < 0 { 1 } else { 0 };
        let j2 = 1 - (((imm >> 22) & 0x1) ^ s);
        let j1 = 1 - (((imm >> 23) & 0x1) ^ s);
        let opcode = (0b1101 << 28)
            | (j1 << 29)
            | (j2 << 27)
            | (imm11 << 16)
            | (0b11110 << 11)
            | (s << 10)
            | imm10;
        return opcode as u32 >> 0;
    }
}

pub struct BlxT1;
impl BlxT1 {
    pub fn opcode<T: registers::SpControl>(rm: Register<T>) -> u16 {
        return (0b010001111 << 7) | (rm.number() << 3);
    }
}

pub struct AsrImmediateT1;
impl AsrImmediateT1 {
    pub fn opcode<T: registers::SpControl, U: registers::SpControl>(
        rd: Register<T>,
        rm: Register<U>,
        imm5: u16,
    ) -> u16 {
        return (0b00010 << 11)
            | ((imm5 & 0x1f) << 6)
            | ((rm.number() & 0x7) << 3)
            | (rd.number() & 0x7);
    }
}

pub struct AsrRegisterT1;
impl AsrRegisterT1 {
    pub fn opcode<T: registers::SpControl, U: registers::SpControl>(
        rdn: Register<T>,
        rm: Register<U>,
    ) -> u16 {
        return (0b0100000100 << 6)
            | ((rm.number() & 0x7) << 3)
            | ((rm.number() & 0x7) << 3)
            | (rdn.number() & 0x7);
    }
}
