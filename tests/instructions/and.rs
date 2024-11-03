#[cfg(test)]
mod tests {
    use rp2350_sim::cortex_m33::registers::Register;
    use rp2350_sim::cortex_m33::opcodes::*;
    use rp2350_sim::{RAM_START_ADDRESS, RP2350};

    #[test]
    fn ands_t1() {
        // should execute `ands r5, r0` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(
            RAM_START_ADDRESS,
            AndRegisterT1::opcode(
                &rp2350.cortex_m33.registers.r5,
                &rp2350.cortex_m33.registers.r0,
            ),
        );
        rp2350.cortex_m33.registers.r5.set(0xffff0000);
        rp2350.cortex_m33.registers.r0.set(0xf00fffff);
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r5.get(), 0xf00f0000);
        assert_eq!(rp2350.cortex_m33.apsr.n(), true);
        assert_eq!(rp2350.cortex_m33.apsr.z(), false);
    }
}
