#[cfg(test)]
mod tests {
    use rp2350_sim::cortex_m33::opcodes::*;
    use rp2350_sim::{RAM_START_ADDRESS, RP2350};

    #[test]
    fn dmb_sy() {
        // should correctly decode a `dmb sy` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, DmbT1Sy::opcode());
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000004);
    }
}
