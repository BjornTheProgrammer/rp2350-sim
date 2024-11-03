#[cfg(test)]
mod tests {
    use rp2350_sim::cortex_m33::registers::Register;
    use rp2350_sim::cortex_m33::opcodes::*;
    use rp2350_sim::{RAM_START_ADDRESS, RP2350};

    #[test]
    fn rev16() {
        // should execute a `rev16 r0, r5` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(
            RAM_START_ADDRESS,
            Rev16T1::opcode(
                &rp2350.cortex_m33.registers.r0,
                &rp2350.cortex_m33.registers.r5,
            ),
        );

        rp2350.cortex_m33.registers.r5.set(0x11223344);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r0.get(), 0x22114433);
    }
}
