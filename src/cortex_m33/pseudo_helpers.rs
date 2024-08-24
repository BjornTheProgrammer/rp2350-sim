// All of these are translations of the pseudo code from https://developer.arm.com/documentation/ddi0419/latest/ to rust

pub fn sint_u32(x: u32) -> i32 {
	(x as i32 * -1i32) as i32
}

pub fn add_with_carry(x: u32, y: u32, carry_in: bool) -> (u32, bool, bool) {
	let overflow_x_and_y = x.overflowing_add(y);
	let unsigned_sum = (carry_in as u32).overflowing_add(overflow_x_and_y.0);
	let carry_out = overflow_x_and_y.1 || unsigned_sum.1;

	let overflow_x_and_y = (x as i32).overflowing_add(y as i32);
	let signed_sum = (carry_in as i32).overflowing_add(overflow_x_and_y.0);
	let overflow = overflow_x_and_y.1 || signed_sum.1;

	(unsigned_sum.0, carry_out, overflow)
}

pub fn is_zero_bit(x: u32) -> bool {
	x == 0
}

pub fn get_bit<V: num_traits::PrimInt>(value: V, bit: u8) -> bool {
	let mask = V::one() << bit.into();
	if (mask & value) > V::zero() {
		true
	} else {
		false
	}
}