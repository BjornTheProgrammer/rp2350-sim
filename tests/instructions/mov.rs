#[cfg(test)]
mod tests {
    use rp2350_sim::cortex_m33::opcodes::*;
    use rp2350_sim::cortex_m33::registers::Register;
    use rp2350_sim::{RAM_START_ADDRESS, RP2350};

    #[test]
    fn mov() {
        // should execute a `mov r3, r8` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.cortex_m33.memory.write_u16(
            RAM_START_ADDRESS,
            MovRegisterT1::opcode(
                &rp2350.cortex_m33.registers.r3,
                &rp2350.cortex_m33.registers.r8,
            ),
        );
        rp2350.cortex_m33.registers.r8.set(55);
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r3.get(), 55);
    }

    #[test]
    fn mov_pc() {
        // should execute a `mov r3, pc` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.cortex_m33.memory.write_u16(
            RAM_START_ADDRESS,
            MovRegisterT1::opcode(
                &rp2350.cortex_m33.registers.r3,
                &rp2350.cortex_m33.registers.pc,
            ),
        );
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r3.get(), 0x20000004);
    }
}
