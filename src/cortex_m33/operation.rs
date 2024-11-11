use bilge::prelude::*;
use core::ops::Range;
use std::ops::{Bound, RangeBounds};

use crate::cortex_m33::control::SpSel;

use super::{
    apsr::{Apsr, Ipsr}, control::NPriv, exception::Exceptions, registers::{PcRegister, Register}, CortexM33, Mode
};

pub fn add_with_carry(x: u32, y: u32, carry_in: bool) -> (u32, bool, bool) {
    let overflow_x_and_y = x.overflowing_add(y);
    let unsigned_sum = (carry_in as u32).overflowing_add(overflow_x_and_y.0);
    let carry_out = overflow_x_and_y.1 || unsigned_sum.1;

    let overflow_x_and_y = (x as i32).overflowing_add(y as i32);
    let signed_sum = (carry_in as i32).overflowing_add(overflow_x_and_y.0);
    let overflow = overflow_x_and_y.1 || signed_sum.1;

    (unsigned_sum.0, carry_out, overflow)
}

pub fn unwind_bounds<V: num_traits::PrimInt, R: RangeBounds<usize>>(
    value: V,
    range: R,
) -> (usize, usize) {
    let start: Bound<&usize> = range.start_bound();
    let start = match start {
        Bound::Included(&start) => start,
        Bound::Excluded(start) => start
            .checked_add(1)
            .unwrap_or_else(|| panic!("attempted to index slice from after maximum usize")),
        Bound::Unbounded => 0,
    };

    let end: Bound<&usize> = range.end_bound();
    let end = match end {
        Bound::Included(end) => end
            .checked_add(1)
            .unwrap_or_else(|| panic!("attempted to index slice up to maximum usize")),
        Bound::Excluded(&end) => end,
        Bound::Unbounded => get_size_of_number(value),
    };

    (start, end)
}

// 0b0100000000111110, 0..3  -  0b110
// 0b0100000000111110, 3..6  -  0b111
pub fn get_bits<V: num_traits::PrimInt, R: RangeBounds<usize>>(value: V, range: R) -> V {
    let (start, end) = unwind_bounds(value, range);

    let shifted = value >> start;
    let mask: V = (V::one() << (end - start)) - V::one();
    shifted & mask
}

pub fn set_bit<V: num_traits::PrimInt>(value: &mut V, bit_index: usize, bit: bool) {
    assert!(bit_index < get_size_of_number(*value));

    if bit {
        *value = *value | (V::one() << bit_index);
    } else {
        *value = *value & !(V::one() << bit_index);
    }
}

pub fn set_bits<V: num_traits::PrimInt>(
    value: &mut V,
    range: Range<usize>,
    bits: u32,
    bits_len: usize,
) {
    let range = range;
    let start = range.start;
    for i in range {
        set_bit(value, i, get_bit(bits, (i - start) % bits_len));
    }
}

pub fn is_zero_bit(x: u32) -> bool {
    x == 0
}

/// Indexs from 0.
pub fn get_bit<V: num_traits::PrimInt>(value: V, bit: usize) -> bool {
    assert!(bit < get_size_of_number(value));

    let mask = V::one() << bit.into();
    if (mask & value) > V::zero() {
        true
    } else {
        false
    }
}

pub fn lsl_c(value: u32, amount: usize) -> (u32, bool) {
    assert!(amount > 0);
    let value = value as u64;
    let value = value << amount;
    let carry_out = get_bit(value, 32);
    let result = value as u32;

    (result, carry_out)
}

pub fn lsr_c(value: u32, amount: usize) -> (u32, bool) {
    assert!(amount > 0);

    let value = value as u64;
    let result = get_bits(value, amount..amount + 32) as u32;
    let carry_out = get_bit(value, amount - 1);

    (result, carry_out)
}

#[macro_export]
macro_rules! unpredictable {
	() => {
        {
    		println!("Unpredictable behavior occured!");
    		println!("What does this mean? It means according to the documentation...");
    		println!("Means the behavior cannot be relied on. UNPREDICTABLE behavior must not represent a security hole. UNPREDICTABLE behavior must not hang the processor, or any parts of the system. UNPREDICTABLE behavior must not be documented or promoted as having a defined effect.");
        }
	};
}

pub fn in_it_block() -> bool {
    return false;
}

pub fn condition_passed(apsr: &Apsr, cond: u16) -> bool {
    println!("looking at condition");
    println!("cond as came in: {:#b}", cond);
    let cond = get_bits(cond, 0..4);
    println!("cond: {:#b}", cond);
    let mut result = match get_bits(cond, 0..3) {
        0b000 => {
            println!("case 1");
            apsr.z() == true
        }
        0b001 => {
            println!("case 2");
            apsr.c() == true
        }
        0b010 => {
            println!("case 3");
            apsr.n() == true
        }
        0b011 => {
            println!("case 4");
            apsr.v() == true
        }
        0b100 => {
            println!("case 5");
            apsr.c() == true && apsr.z() == false
        }
        0b101 => {
            println!("case 6");
            apsr.n() == apsr.v()
        }
        0b110 => {
            println!("case 7");
            apsr.n() == apsr.v() && apsr.z() == false
        }
        0b111 => {
            println!("case 8");
            true
        }
        _ => unreachable!(),
    };

    println!("result: {result}");

    if get_bit(cond, 0) == true && cond != 0b1111 {
        result = !result;
    }

    println!("final result: {result}");

    result
}

pub fn asr_c(value: u32, shift: usize) -> (u32, bool) {
    println!("value: {:#x}", value);
    println!("value: {:?}", shift);
    assert!(shift > 0);

    let extended = match sign_extend(value, 31, 32 + shift) {
        SignExtended::U64(val) => val as u128,
        SignExtended::U128(val) => val,
        _ => unreachable!(),
    };

    let result = get_bits(extended, shift..(shift + 32)) as u32;
    let carry = get_bit(extended, shift - 1);

    (result, carry)
}

pub fn get_lsb<N: num_traits::PrimInt>(n: N) -> N {
    n & N::one()
}

pub fn get_size_of_number<N: num_traits::PrimInt>(_n: N) -> usize {
    std::mem::size_of::<N>() * 8
}

pub fn get_msb<N: num_traits::PrimInt>(n: N) -> bool {
    let shift = get_size_of_number(n) - 1;
    if (n >> shift) & N::one() == N::zero() {
        false
    } else {
        true
    }
}

pub fn last_in_it_block() -> bool {
    false
}

pub fn is_ones<N: num_traits::PrimInt, R: RangeBounds<usize>>(value: N, range: R) -> bool {
    let (start, end) = unwind_bounds(value, range);

    for i in start..end {
        if get_bit(value, i) == false {
            return false;
        }
    }

    true
}

pub fn exception_active_bit_count(exceptions: &Exceptions) -> usize {
    exceptions.active.len()
}

pub fn exception_return(cortex: &mut CortexM33, ipsr: u8, current_mode: Mode, exc_return: u32) {
    assert_eq!(current_mode, Mode::Handler);
    if !is_ones(get_bits(exc_return, 4..=27), 0..23) {
        unpredictable!();
    }

    let returning_exception_number = get_bits(ipsr, 0..=5);
    let nested_activation = exception_active_bit_count(&cortex.exceptions);

    if cortex
        .exceptions
        .active
        .contains_key(&returning_exception_number) {
        unpredictable!();
    }

    match get_bits(exc_return, 0..=3) {
        0b0001 => {
            if nested_activation == 1 {
                unpredictable!();
            }

            let frameptr = cortex.registers.sp.get_msp();
            let current_mode = Mode::Handler;
            cortex.control.spsel = SpSel::SpMain;
        },
        0b1001 => {
            if nested_activation != 1 {
                unpredictable!();
            }

            let frameptr = cortex.registers.sp.get_msp();
            let current_mode = Mode::Thread;
            cortex.control.spsel = SpSel::SpMain;
        },
        0b1101 => {
            if nested_activation != 1 {
                unpredictable!();
            }

            let frameptr = cortex.registers.sp.get_psp();
            let current_mode = Mode::Thread;
            cortex.control.spsel = SpSel::SpProcess;
        },
        _ => unpredictable!(),
    }


}

pub fn deactivate(cortex: &mut CortexM33, returning_exception_number: u8) {
    cortex.exceptions.active.remove(&returning_exception_number);
}

pub fn popstack(cortex: &mut CortexM33, frameptr: u32, exc_return: u32) {
    cortex.registers[0] = cortex.memory.read_u32(frameptr);
    cortex.registers[1] = cortex.memory.read_u32(frameptr + 0x04);
    cortex.registers[2] = cortex.memory.read_u32(frameptr + 0x08);
    cortex.registers[3] = cortex.memory.read_u32(frameptr + 0x0C);
    cortex.registers[12] = cortex.memory.read_u32(frameptr + 0x10);
    cortex.registers.lr.set(frameptr + 0x14);
    cortex.registers.pc.set(frameptr + 0x18);
    let psr = frameptr + 0x1C;

    if get_bit(psr, 0) == true { unpredictable!() }
    branch_to(cortex, cortex.registers.pc.get());

    let sp_mask = (get_bit(psr, 9) as u32) << 2;
    match get_bits(exc_return, 0..=3) {
        0b0001 | 0b1001 => {
            let msp = cortex.registers.sp.get_msp() + 0x20;
            cortex.registers.sp.set_msp(msp | sp_mask);
        },
        0b1101 => {
            let psp = cortex.registers.sp.get_psp() + 0x20;
            cortex.registers.sp.set_msp(psp | sp_mask);
        },
        _ => unpredictable!(),
    }

    let mut aspr_values = Apsr::new();
    aspr_values.set_from_u32(psr);

    cortex.xpsr.apsr.set_n(aspr_values.n());
    cortex.xpsr.apsr.set_z(aspr_values.z());
    cortex.xpsr.apsr.set_c(aspr_values.c());
    cortex.xpsr.apsr.set_v(aspr_values.v());

    let force_thread = cortex.mode == Mode::Thread && cortex.control.npriv == NPriv::ThreadModeUnprivileged;

    // let Ipsr

}

pub fn branch_to(cortex: &mut CortexM33, address: u32) {
    cortex.registers.pc.set(address);
}

pub fn bx_write_pc(pc: &mut PcRegister, current_mode: Mode, address: u32) {
    if current_mode == Mode::Handler && get_bits(address, 28..=31) == 0b1111 {
        // exception_return()
    }
}

#[derive(Debug)]
pub enum SignExtended {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

pub fn branch_write_pc(pc: &mut PcRegister, address: u32) {
    pc.set(address & 0xFFFFFFFE);
}


/// Will assign every bit to the left of start_size to the most significant bit.
pub fn sign_extend<
    V: num_traits::PrimInt
        + num_traits::AsPrimitive<u128>
        + num_traits::AsPrimitive<u64>
        + num_traits::AsPrimitive<u32>
        + num_traits::AsPrimitive<u16>
        + num_traits::AsPrimitive<u8>,
>(
    value: V,
    topbit: usize,
    len: usize,
) -> SignExtended {
    let msb = match get_bit(value, topbit) {
        true => 1,
        false => 0,
    };

    let mut value = match len {
        0..=8 => SignExtended::U8(value.as_()),
        9..=16 => SignExtended::U16(value.as_()),
        17..=32 => SignExtended::U32(value.as_()),
        33..=64 => SignExtended::U64(value.as_()),
        65..=128 => SignExtended::U128(value.as_()),
        _ => SignExtended::U128(value.as_()),
    };

    match &mut value {
        SignExtended::U8(ref mut value) => set_bits(value, topbit..len, msb, 1),
        SignExtended::U16(ref mut value) => set_bits(value, topbit..len, msb, 1),
        SignExtended::U32(ref mut value) => set_bits(value, topbit..len, msb, 1),
        SignExtended::U64(ref mut value) => set_bits(value, topbit..len, msb, 1),
        SignExtended::U128(ref mut value) => set_bits(value, topbit..len, msb, 1),
    }

    value
}

pub fn ror_c(value: u32, amount: usize) -> (u32, bool) {
    assert!(amount > 0);

    let value = (value as u64) << 32 >> (amount as u64);
    let result: u32 = (value as u32) | (value >> 32) as u32;
    let carry_out = (result & (1 << 31)) != 0;

    (result, carry_out)
}

pub fn rrx_c(value: u32, carry_in: bool) -> (u32, bool) {
    let carry_out = get_bit(value, 0);
    let result = value >> 1 + (carry_in as u32) << 31;

    (result, carry_out)
}

#[derive(PartialEq, Clone, Copy)]
pub enum SRType {
    Lsl,
    Lsr,
    Asr,
    Rrx,
    Ror,
}

pub fn shift_c(value: u32, sr_type: SRType, amount: u16, carry_in: bool) -> (u32, bool) {
    assert!(!(sr_type == SRType::Rrx && amount != 1));
    if amount == 0 {
        return (value, carry_in);
    }

    let amount = amount as usize;

    match sr_type {
        SRType::Lsl => lsl_c(value, amount),
        SRType::Lsr => lsr_c(value, amount),
        SRType::Asr => asr_c(value, amount),
        SRType::Rrx => rrx_c(value, carry_in),
        SRType::Ror => ror_c(value, amount),
    }
}

pub fn decode_imm_shift(imm_type: u2, imm5: u16) -> (SRType, u16) {
    if imm_type == u2::new(0b00) {
        (SRType::Lsl, imm5)
    } else if imm_type == u2::new(0b01) {
        let imm5 = if imm5 == 0b00000 { 32 } else { imm5 };
        (SRType::Lsl, imm5)
    } else if imm_type == u2::new(0b10) {
        let imm5 = if imm5 == 0b00000 { 32 } else { imm5 };
        (SRType::Asr, imm5)
    } else {
        if imm5 == 0b00000 {
            (SRType::Rrx, 1)
        } else {
            (SRType::Ror, imm5)
        }
    }
}
