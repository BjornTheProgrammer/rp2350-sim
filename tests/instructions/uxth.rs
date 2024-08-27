#[cfg(test)]
mod tests {
    use rp2350_sim::cortex_m33::opcodes::*;
    use rp2350_sim::{RAM_START_ADDRESS, RP2350};

    #[test]
    fn uxth() {
        // should execute an `uxtb r3, r1` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(
            RAM_START_ADDRESS,
            UxthT1::opcode(
                rp2350.cortex_m33.registers.r3,
                rp2350.cortex_m33.registers.r1,
            ),
        );

        rp2350.cortex_m33.registers.r1.set(0x12345678);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r3.get(), 0x5678);
    }
}
