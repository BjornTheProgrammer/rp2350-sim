#[cfg(test)]
mod tests {
    use rp2350_sim::cortex_m33::opcodes::*;
    use rp2350_sim::cortex_m33::registers::Register;
    use rp2350_sim::{RAM_START_ADDRESS, RP2350};

    #[test]
    fn sub_sp_minus_immediate() {
        // should execute a `sub sp, 0x10` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.cortex_m33.memory.write_u16(RAM_START_ADDRESS, SubSpMinusImmediateT1::opcode(0x10));

        rp2350.cortex_m33.registers.sp.set(0x10000040);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.sp.get(), 0x10000030);
    }
}
