#[cfg(test)]
mod tests {
    use assert_hex::assert_eq_hex;
    use rp2350_sim::cortex_m33::opcodes::*;
    use rp2350_sim::cortex_m33::registers::Register;
    use rp2350_sim::{RAM_START_ADDRESS, RP2350};

    #[test]
    fn bt1() {
        // should execute a `bne.n .-6` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350
            .cortex_m33
            .registers
            .pc
            .set(RAM_START_ADDRESS + 9 * 2);
        rp2350.cortex_m33.apsr.set_z(false);

        let opcode = BT1::opcode(1, 0x1f8);
        rp2350.cortex_m33.memory.write_u16(RAM_START_ADDRESS + 9 * 2, opcode);
        rp2350.execute_instruction();

        assert_eq_hex!(rp2350.cortex_m33.registers.pc.get(), 0x2000000e);
    }

    #[test]
    fn bt2() {
        // should execute a `b.n .-20` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350
            .cortex_m33
            .registers
            .pc
            .set(RAM_START_ADDRESS + 9 * 2);

        rp2350.cortex_m33.memory.write_u16(RAM_START_ADDRESS + 9 * 2, BT2::opcode(0xfec));
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000002);
    }
}
