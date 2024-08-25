use core::ops::Range;
use bilge::prelude::*;

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
pub fn get_bits<V: num_traits::PrimInt>(value: V, range: Range<usize>) -> V {
	let shifted = value >> range.start.into();
	let mask: V = (V::one() << (range.end - range.start)) - V::one();
	shifted & mask
}

pub fn set_bit<V: num_traits::PrimInt + derive_more::Debug>(value: &mut V, bit_index: usize, bit: bool) {
	assert!(bit_index < get_size_of_number(*value));

	if bit {
		*value = *value | (V::one() << bit_index);
	} else {
		*value = *value & !(V::one() << bit_index);
	}
}

pub fn set_bits<V: num_traits::PrimInt + derive_more::Debug>(value: &mut V, range: Range<usize>, bits: u32, bits_len: usize) {
	let range = range;
	let start = range.start;
	for i in range {
		set_bit(value, i, get_bit(bits, (i - start) % bits_len));
	}
}

pub fn is_zero_bit(x: u32) -> bool {
	x == 0
}

pub fn get_bit<V: num_traits::PrimInt>(value: V, bit: usize) -> bool {
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



// macro_rules! sign_extend {
// 	($number:expr, $shift:expr) => {
// 		let size = get_size_of_number($number);
		
// 		sign_extend!($number, $shift, size);
// 	};

// 	($number:expr, $shift:expr, $size:tt) => {
// 		use paste::paste;
// 		paste! {
// 			let extended = match sign_extend($number, $size + $shift) {
// 				[<S i g n E x t e n d e d>]::[<U 8>](val) => val,
// 				_ => unreachable!(),
// 			};
// 		}
// 	};
// }

pub fn asr_c(value: u32, shift: usize) -> (u32, bool) {
	assert!(shift > 0);
	// sign_extend!(value, shift);

	let extended = match sign_extend(value, 32 + shift) {
		SignExtended::U64(val) => val,
		_ => unreachable!(),
	};

	let result = get_bits(extended, shift..shift + 32) as u32;
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
	if (n >> shift) & N::one() == N::zero() { false } else { true }
}

#[derive(Debug)]
pub enum SignExtended {
	U8(u8),
	U16(u16),
	U32(u32),
	U64(u64),
	U128(u128),
}

// Will assign every bit to the left of start_size to the most significant bit.
pub fn sign_extend<
	V: num_traits::PrimInt
	+ num_traits::AsPrimitive<u128>
	+ num_traits::AsPrimitive<u64>
	+ num_traits::AsPrimitive<u32>
	+ num_traits::AsPrimitive<u16>
	+ num_traits::AsPrimitive<u8> + derive_more::Debug
>(value: V, len: usize) -> SignExtended {
	let msb = match get_msb(value) {
		true => 1,
		false => 0,
	};

	let size = get_size_of_number(value);
	let mut value = match len {
		0..=8 => SignExtended::U8(value.as_()),
		9..=16 => SignExtended::U16(value.as_()),
		17..=32 => SignExtended::U32(value.as_()),
		33..=64 => SignExtended::U64(value.as_()),
		65..=128 => SignExtended::U128(value.as_()),
		_ => SignExtended::U128(value.as_())
	};
	
	match &mut value {
		SignExtended::U8(ref mut value) => set_bits(value, size..len, msb, 1),
		SignExtended::U16(ref mut value) => set_bits(value, size..len, msb, 1),
		SignExtended::U32(ref mut value) => set_bits(value, size..len, msb, 1),
		SignExtended::U64(ref mut value) => set_bits(value, size..len, msb, 1),
		SignExtended::U128(ref mut value) => set_bits(value, size..len, msb, 1),
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
