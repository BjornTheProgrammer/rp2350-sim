use crate::cortex_m33::operation::{
    add_with_carry, branch_write_pc, condition_passed, decode_imm_shift, in_it_block,
    last_in_it_block, sign_extend, SignExtended,
};
use crate::cortex_m33::operation::{get_bit, get_bits, is_zero_bit, shift_c, SRType};
use crate::cortex_m33::registers::Register;
use crate::unpredictable;
use bilge::prelude::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct OpCode {
    pub code: u16,
    pub address: u32,
}

impl OpCode {
    pub fn from_address(cortex: &CortexM33, address: u32) -> Self {
        let code = cortex.memory.read_u16(address);
        Self {
            address,
            code,
        }
    }

    pub fn execute(&self, cortex: &mut CortexM33) {
        let op_code_2 = Self::from_address(cortex, self.address + 2);
        Instruction::new(self, &op_code_2).execute(cortex);
    }
}

struct Instruction {
    opcode: OpCode,
    opcode_2: OpCode,
    instruction: InstructionType,
}

#[derive(Debug)]
enum InstructionType {
    AdcT1,
    AddSpPlusImmediateT1,
    AddSpPlusImmediateT2,
    AddsT1,
    AddsT2,
    AddRegisterT1,
    AddRegisterT2,
    AdrT1,
    AndRegisterT1,
    AsrImmediateT1,
    AsrRegisterT1,
    BT1,
    BT2,
    BicRegisterT1,
    BkptT1,
    BlT1,
    BlxT1,
    BxT1,
    CmnRegisterT1,
    CmpImmediateT1,
    CmpRegisterT1,
    CmpRegisterT2,
    CpsT1Id,
    CpsT1Ie,
    DmbT1Sy,
    DsbT1Sy,
    EorRegisterT1,
    IsbT1Sy,
    LdmiaT1,
    LdrImmediateT1,
    LdrImmediateT2,
    LdrLiteralT1,
    LdrRegisterT1,
    LdrbImmediateT1,
    LdrbRegisterT1,
    LdrhImmediateT1,
    LdrhRegisterT1,
    LdrsbRegisterT1,
    LdrshRegisterT1,
    LslImmediateT1,
    LslRegisterT1,
    LsrImmediateT1,
    LsrRegisterT1,
    MovRegisterT1,
    MovImmediateT1,
    MrsT1,
    MsrT1,
    MulT1,
    MvnT1,
    OrrRegisterT1,
    PopT1,
    PushT1,
    RevT1,
    Rev16T1,
    RevshT1,
    RorRegisterT1,
    RsbImmediateT1,
    NopT1,
    SbcRegisterT1,
    SevT1,
    StmiaT1,
    StrImmediateT1,
    StrImmediateT2,
    StrRegisterT1,
    StrbImmediateT1,
    StrbRegisterT1,
    StrhImmediateT1,
    StrhRegisterT1,
    SubSpMinusImmediateT1,
    SubT1,
    SubT2,
    SubRegisterT1,
    SvcT1,
    SxtbT1,
    SxthT1,
    TstRegisterT1,
    UdfT1,
    UdfT2,
    UxtbT1,
    UxthT1,
    WfeT1,
    WfiT1,
    YieldT1,
}

use InstructionType::*;

use super::apsr::Apsr;
use super::CortexM33;

impl Instruction {
    pub fn new(opcode: &OpCode, opcode_2: &OpCode) -> Self {
        let instruction = if opcode.code >> 6 == 0b0100000101 {
            AdcT1
        } else if opcode.code >> 11 == 0b10101 {
            AddSpPlusImmediateT1
        } else if opcode.code >> 7 == 0b101100000 {
            AddSpPlusImmediateT2
        } else if opcode.code >> 9 == 0b0001110 {
            AddsT1
        } else if opcode.code >> 11 == 0b00110 {
            AddsT2
        } else if opcode.code >> 9 == 0b0001100 {
            AddRegisterT1
        } else if opcode.code >> 8 == 0b01000100 {
            AddRegisterT2
        } else if opcode.code >> 11 == 0b10100 {
            AdrT1
        } else if opcode.code >> 6 == 0b0100000000 {
            AndRegisterT1
        } else if opcode.code >> 11 == 0b00010 {
            AsrImmediateT1
        } else if opcode.code >> 6 == 0b0100000100 {
            AsrRegisterT1
        } else if opcode.code >> 12 == 0b1101 && ((opcode.code >> 9) & 0x7) != 0b111 {
            BT1
        } else if opcode.code >> 11 == 0b11100 {
            BT2
        } else if opcode.code >> 6 == 0b0100001110 {
            BicRegisterT1
        } else if opcode.code >> 8 == 0b10111110 {
            BkptT1
        } else if opcode.code >> 11 == 0b11110
            && opcode_2.code >> 14 == 0b11
            && ((opcode_2.code >> 12) & 0x1) == 1
        {
            BlT1
        } else if opcode.code >> 7 == 0b010001111 && (opcode.code & 0x7) == 0 {
            BlxT1
        } else if opcode.code >> 7 == 0b010001110 && (opcode.code & 0x7) == 0 {
            BxT1
        } else if opcode.code >> 6 == 0b0100001011 {
            CmnRegisterT1
        } else if opcode.code >> 11 == 0b00101 {
            CmpImmediateT1
        } else if opcode.code >> 6 == 0b0100001010 {
            CmpRegisterT1
        } else if opcode.code >> 8 == 0b01000101 {
            CmpRegisterT2
        } else if opcode.code == 0xb672 {
            CpsT1Id
        } else if opcode.code == 0xb662 {
            CpsT1Ie
        } else if opcode.code == 0xf3bf && (opcode_2.code & 0xfff0) == 0x8f50 {
            DmbT1Sy
        } else if opcode.code == 0xf3bf && (opcode_2.code & 0xfff0) == 0x8f40 {
            DsbT1Sy
        } else if opcode.code >> 6 == 0b0100000001 {
            EorRegisterT1
        } else if opcode.code == 0xf3bf && (opcode_2.code & 0xfff0) == 0x8f60 {
            IsbT1Sy
        } else if opcode.code >> 11 == 0b11001 {
            LdmiaT1
        } else if opcode.code >> 11 == 0b01101 {
            LdrImmediateT1
        } else if opcode.code >> 11 == 0b10011 {
            LdrImmediateT2
        } else if opcode.code >> 11 == 0b01001 {
            LdrLiteralT1
        } else if opcode.code >> 9 == 0b0101100 {
            LdrRegisterT1
        } else if opcode.code >> 11 == 0b01111 {
            LdrbImmediateT1
        } else if opcode.code >> 9 == 0b0101110 {
            LdrbRegisterT1
        } else if opcode.code >> 11 == 0b10001 {
            LdrhImmediateT1
        } else if opcode.code >> 9 == 0b0101101 {
            LdrhRegisterT1
        } else if opcode.code >> 9 == 0b0101011 {
            LdrsbRegisterT1
        } else if opcode.code >> 9 == 0b0101111 {
            LdrshRegisterT1
        } else if opcode.code >> 11 == 0b00000 {
            LslImmediateT1
        } else if opcode.code >> 6 == 0b0100000010 {
            LslRegisterT1
        } else if opcode.code >> 11 == 0b00001 {
            LsrImmediateT1
        } else if opcode.code >> 6 == 0b0100000011 {
            LsrRegisterT1
        } else if opcode.code >> 8 == 0b01000110 {
            MovRegisterT1
        } else if opcode.code >> 11 == 0b00100 {
            MovImmediateT1
        } else if opcode.code == 0b1111001111101111 && opcode_2.code >> 12 == 0b1000 {
            MrsT1
        } else if opcode.code >> 4 == 0b111100111000 && opcode_2.code >> 8 == 0b10001000 {
            MsrT1
        } else if opcode.code >> 6 == 0b0100001101 {
            MulT1
        } else if opcode.code >> 6 == 0b0100001111 {
            MvnT1
        } else if opcode.code >> 6 == 0b0100001100 {
            OrrRegisterT1
        } else if opcode.code >> 9 == 0b1011110 {
            PopT1
        } else if opcode.code >> 9 == 0b1011010 {
            PushT1
        } else if opcode.code >> 6 == 0b1011101000 {
            RevT1
        } else if opcode.code >> 6 == 0b1011101001 {
            Rev16T1
        } else if opcode.code >> 6 == 0b1011101011 {
            RevshT1
        } else if opcode.code >> 6 == 0b0100000111 {
            RorRegisterT1
        } else if opcode.code >> 6 == 0b0100001001 {
            RsbImmediateT1
        } else if opcode.code == 0b1011111100000000 {
            NopT1
        } else if opcode.code >> 6 == 0b0100000110 {
            SbcRegisterT1
        } else if opcode.code == 0b1011111101000000 {
            SevT1
        } else if opcode.code >> 11 == 0b11000 {
            StmiaT1
        } else if opcode.code >> 11 == 0b01100 {
            StrImmediateT1
        } else if opcode.code >> 11 == 0b10010 {
            StrImmediateT2
        } else if opcode.code >> 9 == 0b0101000 {
            StrRegisterT1
        } else if opcode.code >> 11 == 0b01110 {
            StrbImmediateT1
        } else if opcode.code >> 9 == 0b0101010 {
            StrbRegisterT1
        } else if opcode.code >> 11 == 0b10000 {
            StrhImmediateT1
        } else if opcode.code >> 9 == 0b0101001 {
            StrhRegisterT1
        } else if opcode.code >> 7 == 0b101100001 {
            SubSpMinusImmediateT1
        } else if opcode.code >> 9 == 0b0001111 {
            SubT1
        } else if opcode.code >> 11 == 0b00111 {
            SubT2
        } else if opcode.code >> 9 == 0b0001101 {
            SubRegisterT1
        } else if opcode.code >> 8 == 0b11011111 {
            SvcT1
        } else if opcode.code >> 6 == 0b1011001001 {
            SxtbT1
        } else if opcode.code >> 6 == 0b1011001000 {
            SxthT1
        } else if opcode.code >> 6 == 0b0100001000 {
            TstRegisterT1
        } else if opcode.code >> 8 == 0b11011110 {
            UdfT1
        } else if opcode.code >> 4 == 0b111101111111 && opcode_2.code >> 12 == 0b1010 {
            UdfT2
        } else if opcode.code >> 6 == 0b1011001011 {
            UxtbT1
        } else if opcode.code >> 6 == 0b1011001010 {
            UxthT1
        } else if opcode.code == 0b1011111100100000 {
            WfeT1
        } else if opcode.code == 0b1011111100110000 {
            WfiT1
        } else if opcode.code == 0b1011111100010000 {
            YieldT1
        } else {
            println!("opcode.code: {:#x}", opcode.code);
            unimplemented!("Instruction not implemented, file a github issue.");
        };

        Self {
            opcode: *opcode,
            opcode_2: *opcode_2,
            instruction,
        }
    }

    pub fn execute(&self, cortex_m33: &mut CortexM33) {
        println!("Instruction: {:?}", self.instruction);
        let opcode_pc = cortex_m33.registers.pc.get() & !1;
        let opcode = self.opcode.code;
        let opcode_2 = self.opcode_2;

        cortex_m33
            .registers
            .pc
            .set(cortex_m33.registers.pc.get() + 2);

        match self.instruction {
            AdcT1 => {
                let rm = (opcode >> 3) & 0x7;
                let rdn = opcode & 0x7;

                let rm_value = cortex_m33.get_register_from_number(rm).get();
                let rdn_value = cortex_m33.get_register_from_number(rdn).get()
                    + cortex_m33.xpsr.apsr.c() as u32;
                let result = add_instruction_update_flags(
                    &mut cortex_m33.xpsr.apsr,
                    rm_value,
                    rdn_value,
                    false,
                );
                cortex_m33.get_register_from_number(rdn).set(result);
            }
            AddSpPlusImmediateT1 => {
                let imm8: u32 = opcode as u32 & 0xff;
                let rd = (opcode >> 8) & 0x7;

                let sp = cortex_m33.registers.sp.get();
                cortex_m33
                    .get_register_from_number(rd)
                    .set(sp + (imm8 << 2))
            }
            AddSpPlusImmediateT2 => {
                let imm32 = (opcode as u32 & 0x7f) << 2;
                cortex_m33
                    .registers
                    .sp
                    .set(cortex_m33.registers.sp.get() + imm32);
            }
            AddsT1 => {
                let imm3 = (opcode >> 6) & 0x7;
                let rn = (opcode >> 3) & 0x7;
                let rd = opcode & 0x7;

                let rn_value = cortex_m33.get_register_from_number(rn).get();
                let result = add_instruction_update_flags(
                    &mut cortex_m33.xpsr.apsr,
                    rn_value,
                    imm3 as u32,
                    false,
                );
                cortex_m33.get_register_from_number(rd).set(result);
            }
            AddsT2 => {
                let imm8 = opcode & 0xff;
                let rdn = (opcode >> 8) & 0x7;

                let rdn_value = cortex_m33.get_register_from_number(rdn).get();
                let result = add_instruction_update_flags(
                    &mut cortex_m33.xpsr.apsr,
                    rdn_value,
                    imm8 as u32,
                    false,
                );
                cortex_m33.get_register_from_number(rdn).set(result);
            }
            AddRegisterT1 => {
                let rm = (opcode >> 6) & 0x7;
                let rn = (opcode >> 3) & 0x7;
                let rd = opcode & 0x7;

                let rn_value = cortex_m33.get_register_from_number(rn).get();
                let rm_value = cortex_m33.get_register_from_number(rm).get();
                let result = add_instruction_update_flags(
                    &mut cortex_m33.xpsr.apsr,
                    rn_value,
                    rm_value,
                    false,
                );
                cortex_m33.get_register_from_number(rd).set(result);
            }
            AddRegisterT2 => {
                let rdn = ((opcode & 0x80) >> 4) | (opcode & 0x7);
                let left_value = {
                    let rdn = cortex_m33.get_register_from_number(rdn);
                    if rdn.is_pc() {
                        cortex_m33.registers.pc.get() + 2
                    } else {
                        rdn.get()
                    }
                };

                let rm = (opcode >> 3) & 0xf;
                let right_value = cortex_m33.get_register_from_number(rm).get();

                let result = left_value + right_value;

                let rdn = cortex_m33.get_register_from_number(rdn);

                if !rdn.is_sp() && !rdn.is_pc() {
                    rdn.set(result);
                } else if rdn.is_pc() {
                    rdn.set(result & !0x1)
                } else if rdn.is_sp() {
                    rdn.set(result & !0x3)
                }
            }
            AdrT1 => {
                let imm8 = opcode as u32 & 0xff;
                let rd = (opcode >> 8) & 0x7;

                cortex_m33
                    .get_register_from_number(rd)
                    .set((opcode_pc & 0xfffffffc) + 4 + (imm8 << 2));
            }
            AndRegisterT1 => {
                let rdn = get_bits(opcode, 0..3);
                let rm = get_bits(opcode, 3..6);
                let rm_value = cortex_m33.get_register_from_number(rm).get();
                let (shifted, carry) =
                    shift_c(rm_value, SRType::Lsl, 0, cortex_m33.xpsr.apsr.c());

                let rdn = cortex_m33.get_register_from_number(rdn);
                let result = rdn.get() & shifted;
                rdn.set(result);

                cortex_m33.xpsr.apsr.set_n(get_bit(result, 31));
                cortex_m33.xpsr.apsr.set_z(is_zero_bit(result));
                cortex_m33.xpsr.apsr.set_c(carry);
            }
            AsrImmediateT1 => {
                let rd = get_bits(opcode, 0..3);
                let rm = get_bits(opcode, 3..6);
                let imm5 = get_bits(opcode, 6..11);

                let (_, shift_n) = decode_imm_shift(u2::new(0b10), imm5);

                let rm = cortex_m33.get_register_from_number(rm).get();

                let (result, carry) = shift_c(rm, SRType::Asr, shift_n, cortex_m33.xpsr.apsr.c());
                cortex_m33.get_register_from_number(rd).set(result);

                cortex_m33.xpsr.apsr.set_n(get_bit(result, 31));
                cortex_m33.xpsr.apsr.set_z(is_zero_bit(result));
                cortex_m33.xpsr.apsr.set_c(carry);
            }
            AsrRegisterT1 => {
                let rdn = get_bits(opcode, 0..=2);
                let rm = get_bits(opcode, 3..=5);

                let rm = cortex_m33.get_register_from_number(rm).get();

                let setflags = !in_it_block();

                let shift_n = get_bits(rm as u16, 0..=7);
                let (result, carry) = shift_c(
                    cortex_m33.get_register_from_number(rdn).get(),
                    SRType::Asr,
                    shift_n,
                    cortex_m33.xpsr.apsr.c(),
                );
                cortex_m33.get_register_from_number(rdn).set(result);

                if setflags {
                    cortex_m33.xpsr.apsr.set_n(get_bit(result, 31));
                    cortex_m33.xpsr.apsr.set_z(is_zero_bit(result));
                    cortex_m33.xpsr.apsr.set_c(carry);
                }
            }
            BT1 => {
                let cond = get_bits(opcode, 8..=11);
                let imm8 = get_bits(opcode, 0..=7) << 1;

                if in_it_block() {
                    unpredictable!();
                };

                let imm32 = match sign_extend(imm8, 8, 32) {
                    SignExtended::U32(val) => val,
                    _ => unreachable!(),
                } as i32;

                if condition_passed(&cortex_m33.xpsr.apsr, cond) {
                    let pc_value = cortex_m33.registers.pc.get();
                    branch_write_pc(
                        &mut cortex_m33.registers.pc,
                        (pc_value as i32 + imm32) as u32,
                    );
                }

                cortex_m33
                    .registers
                    .pc
                    .set(cortex_m33.registers.pc.get() + 2);
            }
            BT2 => {
                let opcode = opcode as i32;
                let mut imm11 = (opcode & 0x7ff) << 1;
                if imm11 & (1 << 11) > 0 {
                    imm11 = (imm11 & 0x7ff) - 0x800;
                }

                let pc_value = cortex_m33.registers.pc.get() as i32;
                let value = pc_value + imm11 + 2;

                cortex_m33.registers.pc.set(value as u32);
            }
            BicRegisterT1 => {
                let rdn = get_bits(opcode, 0..=2);
                let rm = get_bits(opcode, 3..=5);

                let rm = cortex_m33.get_register_from_number(rm).get();

                let setflags = !in_it_block();
                let shift_t = SRType::Lsl;
                let shift_n = 0;

                let (shifted, carry) = shift_c(rm, shift_t, shift_n, cortex_m33.xpsr.apsr.c());

                let rdn = cortex_m33.get_register_from_number(rdn);
                let result = rdn.get() & !shifted;
                rdn.set(result);

                if setflags {
                    cortex_m33.xpsr.apsr.set_n(get_bit(result, 31));
                    cortex_m33.xpsr.apsr.set_z(is_zero_bit(result));
                    cortex_m33.xpsr.apsr.set_c(carry);
                }
            }
            BkptT1 => {
                let imm8 = get_bits(opcode, 0..=7);
                let _imm32 = imm8 as u32;

                todo!();
            }
            BlT1 => {
                let opcode = opcode as i32;
                let opcode_2 = opcode_2.code as i32;

                let imm11 = opcode_2 & 0x7ff;
                let j2 = (opcode_2 >> 11) & 0x1;
                let j1 = (opcode_2 >> 13) & 0x1;
                let imm10 = opcode & 0x3ff;
                let s = (opcode >> 10) & 0x1;
                let i1 = 1 - (s ^ j1);
                let i2 = 1 - (s ^ j2);

                let s = if s > 0 { 0b11111111 } else { 0 };

                let imm32: i32 =
                    (s << 24) | ((i1 << 23) | (i2 << 22) | (imm10 << 12) | (imm11 << 1));
                cortex_m33
                    .registers
                    .lr
                    .set(cortex_m33.registers.pc.get() + 2 | 0x1);

                let pc_value = cortex_m33.registers.pc.get() as i32 + 2 + imm32;
                cortex_m33.registers.pc.set(pc_value as u32);
            }
            BlxT1 => {
                let rm = (opcode >> 3) & 0xf;
                cortex_m33
                    .registers
                    .lr
                    .set(cortex_m33.registers.pc.get() | 0x1);
                let rm_value = cortex_m33.get_register_from_number(rm).get();
                cortex_m33.registers.pc.set(rm_value & !1);
            }
            BxT1 => {
                let rm = get_bits(opcode, 3..=6);
                if in_it_block() && !last_in_it_block() {
                    unpredictable!();
                }

                let rm_value = cortex_m33.get_register_from_number(rm).get();
                if rm_value == 15 {
                    unpredictable!();
                }

                todo!();
            }
            CmnRegisterT1 => {
                todo!();
            }
            CmpImmediateT1 => {
                todo!();
            }
            CmpRegisterT1 => {
                todo!();
            }
            CmpRegisterT2 => {
                todo!();
            }
            CpsT1Id => {
                todo!();
            }
            CpsT1Ie => {
                todo!();
            }
            DmbT1Sy => {
                cortex_m33
                    .registers
                    .pc
                    .set(cortex_m33.registers.pc.get() + 2);
            }
            DsbT1Sy => {
                cortex_m33
                    .registers
                    .pc
                    .set(cortex_m33.registers.pc.get() + 2);
            }
            EorRegisterT1 => {}
            IsbT1Sy => {
                cortex_m33
                    .registers
                    .pc
                    .set(cortex_m33.registers.pc.get() + 2);
            }
            LdmiaT1 => {
                let rn = (opcode >> 8) & 0x7;
                let registers = opcode & 0xff;
                let mut address = cortex_m33.get_register_from_number(rn).get();
                for i in 0..8 {
                    if registers & (1 << i) > 0 {
                        let address_value = cortex_m33.memory.read_u32(address);
                        cortex_m33
                            .get_register_from_number(i)
                            .set(address_value);
                        address += 4;
                    }
                }

                // Write back
                if !(registers & (1 << rn) > 0) {
                    cortex_m33.get_register_from_number(rn).set(address);
                }
            }
            LdrImmediateT1 => {
                todo!();
            }
            LdrImmediateT2 => {
                todo!();
            }
            LdrLiteralT1 => {
                todo!();
            }
            LdrRegisterT1 => {
                todo!();
            }
            LdrbImmediateT1 => {
                todo!();
            }
            LdrbRegisterT1 => {
                todo!();
            }
            LdrhImmediateT1 => {
                todo!();
            }
            LdrhRegisterT1 => {
                todo!();
            }
            LdrsbRegisterT1 => {
                todo!();
            }
            LdrshRegisterT1 => {
                todo!();
            }
            LslImmediateT1 => {
                todo!();
            }
            LslRegisterT1 => {
                todo!();
            }
            LsrImmediateT1 => {
                todo!();
            }
            LsrRegisterT1 => {
                todo!();
            }
            MovRegisterT1 => {
                let rm = (opcode >> 3) & 0xf;
                let rd = ((opcode >> 4) & 0x8) | (opcode & 0x7);

                let rm = cortex_m33.get_register_from_number(rm);
                let mut value = if rm.is_pc() {
                    cortex_m33.registers.pc.get() + 2
                } else {
                    rm.get()
                };

                let rd = cortex_m33.get_register_from_number(rd);

                if rd.is_pc() {
                    value &= !1;
                } else if rd.is_sp() {
                    value &= !3;
                }

                rd.set(value);
            }
            MovImmediateT1 => {
                todo!();
            }
            MrsT1 => {
                todo!();
            }
            MsrT1 => {
                todo!();
            }
            MulT1 => {
                todo!();
            }
            MvnT1 => {
                todo!();
            }
            OrrRegisterT1 => {
                todo!();
            }
            PopT1 => {
                todo!();
            }
            PushT1 => {
                let mut bitcount = 0;
                for i in 0..=8 {
                    if self.opcode.code & (1 << i) > 0 {
                        bitcount += 1;
                    }
                }

                let mut address = cortex_m33.registers.sp.get() - 4 * bitcount;

                for i in 0..=7 {
                    if self.opcode.code & (1 << i) > 0 {
                        let register = cortex_m33.get_register_from_number(i).get();

                        cortex_m33.memory.write(address, register as u8);
                        address += 4;
                    }
                }

                if self.opcode.code & (1 << 8) > 0 {
                    cortex_m33.memory.write(address, cortex_m33.registers.lr.get() as u8);
                }

                let current_sp = cortex_m33.registers.sp.get();
                cortex_m33
                    .registers
                    .sp
                    .set(current_sp - 4 * bitcount);
            }
            RevT1 => {
                let rm = (opcode >> 3) & 0x7;
                let rd = opcode & 0x7;
                let input = cortex_m33.get_register_from_number(rm).get();
                cortex_m33.get_register_from_number(rd).set(
                    ((input & 0xff) << 24)
                        | (((input >> 8) & 0xff) << 16)
                        | (((input >> 16) & 0xff) << 8)
                        | ((input >> 24) & 0xff),
                )
            }
            Rev16T1 => {
                let rm = (opcode >> 3) & 0x7;
                let rd = opcode & 0x7;
                let input = cortex_m33.get_register_from_number(rm).get();
                cortex_m33.get_register_from_number(rd).set(
                    (((input >> 16) & 0xff) << 24)
                        | (((input >> 24) & 0xff) << 16)
                        | ((input & 0xff) << 8)
                        | ((input >> 8) & 0xff),
                )
            }
            RevshT1 => {
                todo!();
            }
            RorRegisterT1 => {
                todo!();
            }
            RsbImmediateT1 => {
                todo!();
            }
            NopT1 => {
                // Do nothing
            }
            SbcRegisterT1 => {}
            SevT1 => {}
            StmiaT1 => {
                let rn = (opcode >> 8) & 0x7;
                let registers = opcode & 0xff;
                let mut address = cortex_m33.get_register_from_number(rn).get();
                for i in 0..8 {
                    if registers & (1 << i) > 0 {
                        let register_value = cortex_m33.get_register_from_number(i).get();
                        // This is probably not correct, gotta check this again later
                        cortex_m33.memory.write_u32(address, register_value);
                        address += 4;
                    }
                }
                // Write back
                if !(registers & (1 << rn) > 0) {
                    cortex_m33.get_register_from_number(rn).set(address);
                }
            }
            StrImmediateT1 => {
                todo!();
            }
            StrImmediateT2 => {
                todo!();
            }
            StrRegisterT1 => {
                todo!();
            }
            StrbImmediateT1 => {
                todo!();
            }
            StrbRegisterT1 => {
                todo!();
            }
            StrhImmediateT1 => {
                todo!();
            }
            StrhRegisterT1 => {
                todo!();
            }
            SubSpMinusImmediateT1 => {
                let imm32 = (opcode & 0x7f) << 2;
                cortex_m33
                    .registers
                    .sp
                    .set(cortex_m33.registers.sp.get() - imm32 as u32);
            }
            SubT1 => {
                todo!();
            }
            SubT2 => {
                todo!();
            }
            SubRegisterT1 => {
                todo!();
            }
            SvcT1 => {
                todo!();
            }
            SxtbT1 => {
                todo!();
            }
            SxthT1 => {
                todo!();
            }
            TstRegisterT1 => {
                todo!();
            }
            UdfT1 => {
                todo!();
            }
            UdfT2 => {
                todo!();
            }
            UxtbT1 => {
                let rm = (opcode >> 3) & 0x7;
                let rd = opcode & 0x7;
                let value = cortex_m33.get_register_from_number(rm).get() & 0xff;
                cortex_m33.get_register_from_number(rd).set(value);
            }
            UxthT1 => {
                let rm = (opcode >> 3) & 0x7;
                let rd = opcode & 0x7;
                let value = cortex_m33.get_register_from_number(rm).get() & 0xffff;
                cortex_m33.get_register_from_number(rd).set(value);
            }
            WfeT1 => {
                todo!();
            }
            WfiT1 => {
                todo!();
            }
            YieldT1 => {
                // Do nothing, wait for an event
            }
        }
    }
}

fn add_instruction_update_flags(apsr: &mut Apsr, x: u32, y: u32, carry_in: bool) -> u32 {
    let (result, carry, overflow) = add_with_carry(x, y, carry_in);

    apsr.set_n(get_bit(result, 31));
    apsr.set_z(is_zero_bit(result));
    apsr.set_c(carry);
    apsr.set_v(overflow);

    result
}
