#[cfg(test)]
mod tests {
    use rp2350_sim::cortex_m33::opcodes::*;
    use rp2350_sim::cortex_m33::registers::Register;
    use rp2350_sim::{RAM_START_ADDRESS, RP2350};

    #[test]
    fn adr() {
        // should execute `adr r4, #0x50` instruction and set the overflow flag correctly
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(
            RAM_START_ADDRESS,
            AdrT1::opcode(&rp2350.cortex_m33.registers.r4, 0x50),
        );
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r4.get(), 0x20000054);
    }
}
