#[cfg(test)]
mod tests {
    use rp2350_sim::cortex_m33::registers::Register;
    use rp2350_sim::cortex_m33::opcodes::*;
    use rp2350_sim::{RAM_START_ADDRESS, RP2350};

    #[test]
    fn adcs() {
        // should execute `adcs r5, r4` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(
            RAM_START_ADDRESS,
            AdcT1::opcode(
                &rp2350.cortex_m33.registers.r5,
                &rp2350.cortex_m33.registers.r4,
            ),
        );
        rp2350.cortex_m33.registers.r4.set(55);
        rp2350.cortex_m33.registers.r5.set(66);
        rp2350.cortex_m33.apsr.set_c(true);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r5.get(), 122);
        assert_eq!(rp2350.cortex_m33.apsr.n(), false);
        assert_eq!(rp2350.cortex_m33.apsr.z(), false);
        assert_eq!(rp2350.cortex_m33.apsr.c(), false);
        assert_eq!(rp2350.cortex_m33.apsr.v(), false);
    }

    #[test]
    fn adcs_2() {
        // should execute `adcs r5, r4` instruction and set negative/overflow flags
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(
            RAM_START_ADDRESS,
            AdcT1::opcode(
                &rp2350.cortex_m33.registers.r5,
                &rp2350.cortex_m33.registers.r4,
            ),
        );
        rp2350.cortex_m33.registers.r4.set(0x7fffffff); // Max signed INT32
        rp2350.cortex_m33.registers.r5.set(0);
        rp2350.cortex_m33.apsr.set_c(true);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r5.get(), 0x80000000);
        assert_eq!(rp2350.cortex_m33.apsr.n(), true);
        assert_eq!(rp2350.cortex_m33.apsr.z(), false);
        assert_eq!(rp2350.cortex_m33.apsr.c(), false);
        assert_eq!(rp2350.cortex_m33.apsr.v(), true);
    }

    #[test]
    fn adcs_3() {
        // should not set the overflow flag when executing `adcs r3, r2` adding 0 to 0 with carry
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(
            RAM_START_ADDRESS,
            AdcT1::opcode(
                &rp2350.cortex_m33.registers.r3,
                &rp2350.cortex_m33.registers.r2,
            ),
        );
        rp2350.cortex_m33.registers.r2.set(0); // Max signed INT32
        rp2350.cortex_m33.registers.r3.set(0);
        rp2350.cortex_m33.apsr.set_c(true);
        rp2350.cortex_m33.apsr.set_z(true);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r3.get(), 1);
        assert_eq!(rp2350.cortex_m33.apsr.n(), false);
        assert_eq!(rp2350.cortex_m33.apsr.z(), false);
        assert_eq!(rp2350.cortex_m33.apsr.c(), false);
        assert_eq!(rp2350.cortex_m33.apsr.v(), false);
    }

    #[test]
    fn adcs_4() {
        // should set the zero, carry and overflow flag when executing `adcs r0, r0` adding 0x80000000 to 0x80000000
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(
            RAM_START_ADDRESS,
            AdcT1::opcode(
                &rp2350.cortex_m33.registers.r0,
                &rp2350.cortex_m33.registers.r0,
            ),
        );
        rp2350.cortex_m33.registers.r0.set(0x80000000); // Max signed INT32
        rp2350.cortex_m33.apsr.set_c(false);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r0.get(), 0);
        assert_eq!(rp2350.cortex_m33.apsr.n(), false);
        assert_eq!(rp2350.cortex_m33.apsr.z(), true);
        assert_eq!(rp2350.cortex_m33.apsr.c(), true);
        assert_eq!(rp2350.cortex_m33.apsr.v(), true);
    }
}
