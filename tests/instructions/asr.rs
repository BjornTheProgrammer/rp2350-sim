#[cfg(test)]
mod tests {
    use rp2350_sim::cortex_m33::opcodes::*;
    use rp2350_sim::cortex_m33::registers::Register;
    use rp2350_sim::{RAM_START_ADDRESS, RP2350};

    #[test]
    fn asr_immediate_t1() {
        // should execute an `asrs r3, r2, #31` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(
            RAM_START_ADDRESS,
            AsrImmediateT1::opcode(
                &rp2350.cortex_m33.registers.r3,
                &rp2350.cortex_m33.registers.r2,
                31,
            ),
        );
        rp2350.cortex_m33.registers.r2.set(0x80000000);
        rp2350.cortex_m33.apsr.set_c(true);
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r3.get(), 0xffffffff);
        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000002);
        assert_eq!(rp2350.cortex_m33.apsr.n(), true);
        assert_eq!(rp2350.cortex_m33.apsr.z(), false);
        assert_eq!(rp2350.cortex_m33.apsr.c(), false);
    }

    #[test]
    fn asr_immediate_t1_2() {
        // should correctly update the carry flags when executing `asrs r3, r2, #32` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(
            RAM_START_ADDRESS,
            AsrImmediateT1::opcode(
                &rp2350.cortex_m33.registers.r3,
                &rp2350.cortex_m33.registers.r2,
                0,
            ),
        );
        rp2350.cortex_m33.registers.r2.set(0x80000000);
        rp2350.cortex_m33.apsr.set_c(false);
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r3.get(), 0xffffffff);
        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000002);
        assert_eq!(rp2350.cortex_m33.apsr.n(), true);
        assert_eq!(rp2350.cortex_m33.apsr.z(), false);
        assert_eq!(rp2350.cortex_m33.apsr.c(), true);
    }

    #[test]
    fn asr_register_t1() {
        // should execute an `asrs r3, r4` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(
            RAM_START_ADDRESS,
            AsrRegisterT1::opcode(
                &rp2350.cortex_m33.registers.r3,
                &rp2350.cortex_m33.registers.r4,
            ),
        );
        rp2350.cortex_m33.registers.r3.set(0x80000040);
        rp2350.cortex_m33.registers.r4.set(0xff500007);
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r3.get(), 0xff000000);
        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000002);
        assert_eq!(rp2350.cortex_m33.apsr.n(), true);
        assert_eq!(rp2350.cortex_m33.apsr.z(), false);
        assert_eq!(rp2350.cortex_m33.apsr.c(), true);
    }

    #[test]
    fn asr_register_t1_2() {
        // should execute an `asrs r3, r4` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(
            RAM_START_ADDRESS,
            AsrRegisterT1::opcode(
                &rp2350.cortex_m33.registers.r3,
                &rp2350.cortex_m33.registers.r4,
            ),
        );
        rp2350.cortex_m33.registers.r3.set(0x40000040);
        rp2350.cortex_m33.registers.r4.set(50);
        rp2350.cortex_m33.apsr.set_c(true);
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r3.get(), 0);
        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000002);
        assert_eq!(rp2350.cortex_m33.apsr.n(), false);
        assert_eq!(rp2350.cortex_m33.apsr.z(), true);
        assert_eq!(rp2350.cortex_m33.apsr.c(), false);
    }

    #[test]
    fn asr_register_t1_3() {
        // should execute an `asrs r3, r4` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(
            RAM_START_ADDRESS,
            AsrRegisterT1::opcode(
                &rp2350.cortex_m33.registers.r3,
                &rp2350.cortex_m33.registers.r4,
            ),
        );
        rp2350.cortex_m33.registers.r3.set(0x40000040);
        rp2350.cortex_m33.registers.r4.set(31);
        rp2350.cortex_m33.apsr.set_c(true);
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r3.get(), 0);
        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000002);
        assert_eq!(rp2350.cortex_m33.apsr.n(), false);
        assert_eq!(rp2350.cortex_m33.apsr.z(), true);
        assert_eq!(rp2350.cortex_m33.apsr.c(), true);
    }

    #[test]
    fn asr_register_t1_4() {
        // should execute an `asrs r3, r4` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(
            RAM_START_ADDRESS,
            AsrRegisterT1::opcode(
                &rp2350.cortex_m33.registers.r3,
                &rp2350.cortex_m33.registers.r4,
            ),
        );
        rp2350.cortex_m33.registers.r3.set(0x80000040);
        rp2350.cortex_m33.registers.r4.set(50);
        rp2350.cortex_m33.apsr.set_c(true);
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r3.get(), 0xffffffff);
        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000002);
        assert_eq!(rp2350.cortex_m33.apsr.n(), true);
        assert_eq!(rp2350.cortex_m33.apsr.z(), false);
        assert_eq!(rp2350.cortex_m33.apsr.c(), true);
    }

    #[test]
    fn asr_register_t1_5() {
        // should execute an `asrs r3, r4` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(
            RAM_START_ADDRESS,
            AsrRegisterT1::opcode(
                &rp2350.cortex_m33.registers.r3,
                &rp2350.cortex_m33.registers.r4,
            ),
        );
        rp2350.cortex_m33.registers.r3.set(0x80000040);
        rp2350.cortex_m33.registers.r4.set(0);
        rp2350.cortex_m33.apsr.set_c(true);
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r3.get(), 0x80000040);
        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000002);
        assert_eq!(rp2350.cortex_m33.apsr.n(), true);
        assert_eq!(rp2350.cortex_m33.apsr.z(), false);
        assert_eq!(rp2350.cortex_m33.apsr.c(), true);
    }
}
