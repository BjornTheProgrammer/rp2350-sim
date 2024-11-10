#[cfg(test)]
mod tests {
    use rp2350_sim::cortex_m33::opcodes::*;
    use rp2350_sim::cortex_m33::registers::Register;
    use rp2350_sim::{RAM_START_ADDRESS, RP2350};

    #[test]
    fn bl() {
        // should execute `bl 0x34` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.cortex_m33.memory.write_u32(RAM_START_ADDRESS, BlT1::opcode(0x34));

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000038);
        assert_eq!(rp2350.cortex_m33.registers.lr.get(), 0x20000005);
    }

    #[test]
    fn bl2() {
        // should execute `bl -0x10` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.cortex_m33.memory.write_u32(RAM_START_ADDRESS, BlT1::opcode(-0x10));

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000004 - 0x10);
        assert_eq!(rp2350.cortex_m33.registers.lr.get(), 0x20000005);
    }

    #[test]
    fn bl3() {
        // should execute `bl -3242` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.cortex_m33.memory.write_u32(RAM_START_ADDRESS, BlT1::opcode(-3242));

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000004 - 3242);
        assert_eq!(rp2350.cortex_m33.registers.lr.get(), 0x20000005);
    }
}
