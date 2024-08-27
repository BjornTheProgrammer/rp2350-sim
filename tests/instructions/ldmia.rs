#[cfg(test)]
mod tests {
    use assert_hex::assert_eq_hex;
    use rp2350_sim::cortex_m33::opcodes::*;
    use rp2350_sim::cortex_m33::registers::Register;
    use rp2350_sim::{RAM_START_ADDRESS, RP2350};

    #[test]
    fn ldmia() {
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        let registers: &[Register<_>] = &[
            rp2350.cortex_m33.registers.r1,
            rp2350.cortex_m33.registers.r2,
        ];

        let opcode = LdmiaT1::opcode(rp2350.cortex_m33.registers.r0, registers.into());
        rp2350.write_to_address(RAM_START_ADDRESS, opcode);
        rp2350.cortex_m33.registers.r0.set(0x20000010);

        rp2350.write_to_address(0x20000010, 0xf00df00d as u32);
        rp2350.write_to_address(0x20000014, 0x4242 as u16);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000002);
        assert_eq!(rp2350.cortex_m33.registers.r0.get(), 0x20000018);
        assert_eq!(rp2350.cortex_m33.registers.r1.get(), 0xf00df00d);
        assert_eq!(rp2350.cortex_m33.registers.r2.get(), 0x4242);
    }
}
