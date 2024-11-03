#[cfg(test)]
mod tests {
    use rp2350_sim::cortex_m33::opcodes::*;
    use rp2350_sim::cortex_m33::registers::Register;

    use rp2350_sim::{registers, RAM_START_ADDRESS, RP2350};

    #[test]
    fn push() {
        // should execute a `push {r4, r5, r6, lr}` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);
        rp2350
            .cortex_m33
            .registers
            .sp
            .set(RAM_START_ADDRESS + 0x100);

        let registers = registers![
            rp2350.cortex_m33.registers.r4,
            rp2350.cortex_m33.registers.r5,
            rp2350.cortex_m33.registers.r6
        ];
        let binary = PushT1::opcode(true, registers.into());

        rp2350.write_to_address(RAM_START_ADDRESS, binary);

        rp2350.cortex_m33.registers.r4.set(0x40);
        rp2350.cortex_m33.registers.r5.set(0x50);
        rp2350.cortex_m33.registers.r6.set(0x60);
        rp2350.cortex_m33.registers.lr.set(0x42);

        rp2350.execute_instruction();

        assert_eq!(
            rp2350.cortex_m33.registers.sp.get(),
            RAM_START_ADDRESS + 0xf0
        );
        assert_eq!(rp2350.sram[0xf0], 0x40);
        assert_eq!(rp2350.sram[0xf4], 0x50);
        assert_eq!(rp2350.sram[0xf8], 0x60);
        assert_eq!(rp2350.sram[0xfc], 0x42);
    }
}
