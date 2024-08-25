use core::ops::Range;
use bilge::prelude::*;

pub fn thing(value: u16, ) {}

pub fn add_with_carry(x: u32, y: u32, carry_in: bool) -> (u32, bool, bool) {
	let overflow_x_and_y = x.overflowing_add(y);
	let unsigned_sum = (carry_in as u32).overflowing_add(overflow_x_and_y.0);
	let carry_out = overflow_x_and_y.1 || unsigned_sum.1;

	let overflow_x_and_y = (x as i32).overflowing_add(y as i32);
	let signed_sum = (carry_in as i32).overflowing_add(overflow_x_and_y.0);
	let overflow = overflow_x_and_y.1 || signed_sum.1;

	(unsigned_sum.0, carry_out, overflow)
}

// 0b0100000000111110, 0..3  -  0b110
// 0b0100000000111110, 3..6  -  0b111
pub fn get_bits<V: num_traits::PrimInt + num_traits::ToBytes>(value: V, range: Range<usize>) -> V {
	let size = value.to_be_bytes().as_ref().len() * 8;
	
	let shifted = value >> range.start.into();
	let mask: V = (V::one() << (range.end - range.start)) - V::one();
	shifted & mask
}

pub fn is_zero_bit(x: u32) -> bool {
	x == 0
}

pub fn get_bit<V: num_traits::PrimInt + derive_more::Debug + derive_more::Binary>(value: V, bit: usize) -> bool {
	assert!(bit < 32);

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

pub fn asr_c(value: u32, amount: usize) -> (u32, bool) {
	assert!(amount > 0);

	let value = value as i32;
	let carry_out = (value & (1 << (amount - 1))) != 0;
	let result = (value >> amount) as u32;

	(result, carry_out)
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
	Ror
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
		let imm5 = if imm5 == 0b00000 {
			32
		} else {
			imm5
		};
		(SRType::Lsl, imm5)
	} else if imm_type == u2::new(0b10) {
		let imm5 = if imm5 == 0b00000 {
			32
		} else {
			imm5
		};
		(SRType::Asr, imm5)
	} else {
		if imm5 == 0b00000 {
			(SRType::Rrx, 1)
		} else {
			(SRType::Ror, imm5)
		}
	}
}
