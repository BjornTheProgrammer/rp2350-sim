use rp2350_sim::{SpControlOn, RP2350};

fn main() {
	let hex = include_str!("../../programs/blinky.hex");

	let mut mcu: RP2350 = RP2350::new();
	
	match mcu.load_hex(hex) {
		Ok(_) => (),
		Err(err) => {
			println!("failed to load hex with err: {:?}", err);
			return;
		},
	};

	mcu.cortex_m33_registers.pc.set(0x304);

	// println!("flash: {:?}", mcu.flash);

	mcu.execute_instruction();
}
