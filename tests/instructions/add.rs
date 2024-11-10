#[cfg(test)]
mod tests {
    use rp2350_sim::cortex_m33::opcodes::*;
    use rp2350_sim::cortex_m33::registers::Register;
    use rp2350_sim::{RAM_START_ADDRESS, RP2350};

    #[test]
    fn add_sp_plus_immediate() {
        // should execute a `add sp, 0x10` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);
        rp2350.cortex_m33.registers.sp.set(0x10000040);

        rp2350.cortex_m33.memory.write_u16(RAM_START_ADDRESS, AddSpPlusImmediateT2::opcode(0x10));
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.sp.get(), 0x10000050)
    }

    #[test]
    fn add_register_sp_plus_immediate() {
        // should execute a `add r1, sp, #4` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);
        rp2350.cortex_m33.registers.sp.set(0x54);

        rp2350.cortex_m33.memory.write_u16(
            RAM_START_ADDRESS,
            AddSpPlusImmediateT1::opcode(&rp2350.cortex_m33.registers.r1, 0x10),
        );
        rp2350.cortex_m33.registers.r1.set(0);
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.sp.get(), 0x54);
        assert_eq!(rp2350.cortex_m33.registers.r1.get(), 0x64);
    }

    #[test]
    fn adds_t1() {
        // should execute `adds r1, r2, #3` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.cortex_m33.memory.write_u16(
            RAM_START_ADDRESS,
            AddsT1::opcode(
                &rp2350.cortex_m33.registers.r1,
                &rp2350.cortex_m33.registers.r2,
                3,
            ),
        );
        rp2350.cortex_m33.registers.r2.set(2);
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r1.get(), 5);
        assert_eq!(rp2350.cortex_m33.xpsr.apsr.n(), false);
        assert_eq!(rp2350.cortex_m33.xpsr.apsr.z(), false);
        assert_eq!(rp2350.cortex_m33.xpsr.apsr.c(), false);
        assert_eq!(rp2350.cortex_m33.xpsr.apsr.v(), false);
    }

    #[test]
    fn adds_t2() {
        // should execute `adds r1, #1` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.cortex_m33.memory.write_u16(
            RAM_START_ADDRESS,
            AddsT2::opcode(&rp2350.cortex_m33.registers.r1, 1),
        );
        rp2350.cortex_m33.registers.r1.set(0xffffffff);
        rp2350.execute_instruction();

        println!(
            "opcode: {:#x}",
            AddsT2::opcode(&rp2350.cortex_m33.registers.r1, 1)
        );
        println!("apsr: {:?}", rp2350.cortex_m33.xpsr.apsr);

        assert_eq!(rp2350.cortex_m33.registers.r1.get(), 0);
        assert_eq!(rp2350.cortex_m33.xpsr.apsr.n(), false);
        assert_eq!(rp2350.cortex_m33.xpsr.apsr.z(), true);
        assert_eq!(rp2350.cortex_m33.xpsr.apsr.c(), true);
        assert_eq!(rp2350.cortex_m33.xpsr.apsr.v(), false);
    }

    #[test]
    fn adds_register() {
        // should execute `adds r1, r2, r7` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.cortex_m33.memory.write_u16(
            RAM_START_ADDRESS,
            AddsRegisterT1::opcode(
                &rp2350.cortex_m33.registers.r1,
                &rp2350.cortex_m33.registers.r2,
                &rp2350.cortex_m33.registers.r7,
            ),
        );
        rp2350.cortex_m33.registers.r2.set(2);
        rp2350.cortex_m33.registers.r7.set(27);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r1.get(), 29);
        assert_eq!(rp2350.cortex_m33.xpsr.apsr.n(), false);
        assert_eq!(rp2350.cortex_m33.xpsr.apsr.z(), false);
        assert_eq!(rp2350.cortex_m33.xpsr.apsr.c(), false);
        assert_eq!(rp2350.cortex_m33.xpsr.apsr.v(), false);
    }

    #[test]
    fn adds_register_2() {
        // should execute `adds r4, r4, r2` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.cortex_m33.memory.write_u16(
            RAM_START_ADDRESS,
            AddsRegisterT1::opcode(
                &rp2350.cortex_m33.registers.r4,
                &rp2350.cortex_m33.registers.r4,
                &rp2350.cortex_m33.registers.r2,
            ),
        );
        rp2350.cortex_m33.registers.r2.set(0x74bc8000);
        rp2350.cortex_m33.registers.r4.set(0x43740000);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r4.get(), 0xb8308000);
        assert_eq!(rp2350.cortex_m33.xpsr.apsr.n(), true);
        assert_eq!(rp2350.cortex_m33.xpsr.apsr.z(), false);
        assert_eq!(rp2350.cortex_m33.xpsr.apsr.c(), false);
        assert_eq!(rp2350.cortex_m33.xpsr.apsr.v(), true);
    }

    #[test]
    fn adds_register_3() {
        // should execute `adds r1, r1, r1` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.cortex_m33.memory.write_u16(
            RAM_START_ADDRESS,
            AddsRegisterT1::opcode(
                &rp2350.cortex_m33.registers.r1,
                &rp2350.cortex_m33.registers.r1,
                &rp2350.cortex_m33.registers.r1,
            ),
        );
        rp2350.cortex_m33.registers.r1.set(0xbf8d1424);
        rp2350.cortex_m33.xpsr.apsr.set_c(true);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r1.get(), 0x7f1a2848);
        assert_eq!(rp2350.cortex_m33.xpsr.apsr.n(), false);
        assert_eq!(rp2350.cortex_m33.xpsr.apsr.z(), false);
        assert_eq!(rp2350.cortex_m33.xpsr.apsr.c(), true);
        assert_eq!(rp2350.cortex_m33.xpsr.apsr.v(), true);
    }

    #[test]
    fn add_register() {
        // should execute `add r1, ip` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.cortex_m33.memory.write_u16(
            RAM_START_ADDRESS,
            AddRegisterT2::opcode(
                &rp2350.cortex_m33.registers.r1,
                &rp2350.cortex_m33.registers.r12,
            ),
        );
        rp2350.cortex_m33.registers.r1.set(66);
        rp2350.cortex_m33.registers.r12.set(44);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r1.get(), 110);
    }
}
