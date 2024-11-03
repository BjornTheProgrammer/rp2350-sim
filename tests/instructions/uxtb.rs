#[cfg(test)]
mod tests {
    use rp2350_sim::cortex_m33::registers::Register;
    use rp2350_sim::cortex_m33::opcodes::*;
    use rp2350_sim::{RAM_START_ADDRESS, RP2350};

    #[test]
    fn uxtb() {
        // should execute an `uxtb r5, r3` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(
            RAM_START_ADDRESS,
            UxtbT1::opcode(
                &rp2350.cortex_m33.registers.r5,
                &rp2350.cortex_m33.registers.r3,
            ),
        );

        rp2350.cortex_m33.registers.r3.set(0x12345678);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r5.get(), 0x78);
    }
}
