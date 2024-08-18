#[cfg(test)]
mod tests {
    use rp2350_sim::{RAM_START_ADDRESS, RP2350};

    #[test]
    fn push_instruction() {
    	// let hex = include_str!("../programs/blinky.hex");
        // rp2350.load_hex(hex).expect("Failed to load hex");

    	let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33_registers.pc.set(RAM_START_ADDRESS);
    	rp2350.cortex_m33_registers.sp.set(RAM_START_ADDRESS + 0x100);
        //                                                                              r4         r5         r6
        rp2350.write_to_address(RAM_START_ADDRESS, (0b1011010 << 9) | ((1) << 8) | (1 << 4) | (1 << 5) | (1 << 6) as u16);
        rp2350.cortex_m33_registers.r4.set(0x40);
        rp2350.cortex_m33_registers.r5.set(0x50);
        rp2350.cortex_m33_registers.r6.set(0x60);
        rp2350.cortex_m33_registers.lr.set(0x42);

        println!("pc: {:#x}", rp2350.cortex_m33_registers.pc.get());
        println!("sp: {:#x}", rp2350.cortex_m33_registers.sp.get());
        println!("r4: {:#x}", rp2350.cortex_m33_registers.r4.get());
        println!("r5: {:#x}", rp2350.cortex_m33_registers.r5.get());
        println!("r6: {:#x}", rp2350.cortex_m33_registers.r6.get());
        println!("lr: {:#x}", rp2350.cortex_m33_registers.lr.get());

        // println!("sram: {:?}", rp2350.sram);

        rp2350.execute_instruction();

        println!("sram: {:#x}", rp2350.sram[0xf0]);
        println!("sram: {:#x}", rp2350.sram[0xf4]);
        println!("sram: {:#x}", rp2350.sram[0xf8]);
        println!("sram: {:#x}", rp2350.sram[0xfc]);

    	assert_eq!(rp2350.cortex_m33_registers.sp.get(), RAM_START_ADDRESS + 0xf0);
    	assert_eq!(rp2350.sram[0xf0], 0x40);
        assert_eq!(rp2350.sram[0xf4], 0x50);
        assert_eq!(rp2350.sram[0xf8], 0x60);
        assert_eq!(rp2350.sram[0xfc], 0x42);
    }
}
