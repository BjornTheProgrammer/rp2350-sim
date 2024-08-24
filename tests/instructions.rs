#[cfg(test)]
mod tests {
    use rp2350_sim::{Register, RAM_START_ADDRESS, RP2350};

    struct Registers<'a, T: rp2350_sim::SpControl> {
        register_numbers: &'a [Register<T>]
    }

    impl<T: rp2350_sim::SpControl> Registers<'_, T> {
        fn binary(&self) -> u16 {
            let mut binary = 0;
            for number in self.register_numbers.iter() {
                binary = binary | (1 << number.number());
            }

            return binary;
        }
    }

    // Implementing `From<&'a [u8]>` for `Registers<'a>`
    impl<'a, T: rp2350_sim::SpControl> From<&'a [Register<T>]> for Registers<'a, T> {
        fn from(array: &'a [Register<T>]) -> Self {
            Registers {
                register_numbers: array,
            }
        }
    }

    // Implementing `Into<&'a [u8]>` for `Registers<'a>`
    impl<'a, T: rp2350_sim::SpControl> Into<&'a [Register<T>]> for Registers<'a, T> {
        fn into(self) -> &'a [Register<T>] {
            self.register_numbers
        }
    }

    struct Push;
    impl Push {
        fn opcode<T: rp2350_sim::SpControl>(push_to_lr: bool, registers: Registers<T>) -> u16 {
            return (0b1011010 << 9) | ((push_to_lr as u16) << 8) | registers.binary();
        }
    }

    struct AddSpPlusImmediate;
    impl AddSpPlusImmediate {
        fn opcode(imm: u16) -> u16 {
            return (0b101100000 << 7) | ((imm >> 2) & 0x7f);
        }
    }

    struct AddRegisterSpPlusImmediate;
    impl AddRegisterSpPlusImmediate {
        // rd is the register number
        fn opcode<T: rp2350_sim::SpControl>(rd: Register<T>, imm8: u16) -> u16 {
            return (0b10101 << 11) | ((rd.number() & 7) << 8) | ((imm8 >> 2) & 0xff);
        }
    }

    struct AddRegister;
    impl AddRegister {
        fn opcode<T: rp2350_sim::SpControl, U: rp2350_sim::SpControl>(rdn: Register<T>, rm: Register<U>) -> u16 {
            return (0b01000100 << 8) | ((rdn.number() & 0x8) << 4) | ((rm.number() & 0xf) << 3) | (rdn.number() & 0x7);
        }
    }

    struct Adr;
    impl Adr {
        fn opcode<T: rp2350_sim::SpControl>(rd: Register<T>, imm8: u16) -> u16 {
            return (0b10100 << 11) | ((rd.number() & 7) << 8) | ((imm8 >> 2) & 0xff);
        }
    }

    struct BT2;
    impl BT2 {
        fn opcode(imm11: u16) -> u16 {
            return (0b11100 << 11) | ((imm11 >> 1) & 0x7ff);
        }
    }

    struct DmbSy;
    impl DmbSy {
        fn opcode() -> u32 {
            return 0x8f50f3bf;
        }
    }

    struct DsbSy;
    impl DsbSy {
        fn opcode() -> u32 {
            return 0x8f4ff3bf;
        }
    }

    struct IsbSy;
    impl IsbSy {
        fn opcode() -> u32 {
            return 0x8f6ff3bf;
        }
    }

    struct Mov;
    impl Mov {
        fn opcode<T: rp2350_sim::SpControl, U: rp2350_sim::SpControl>(rd: Register<T>, rm: Register<U>) -> u16 {
            let bit = if rd.number() & 0x8 > 0 {
                1
            } else {
                0
            };

            return (0b01000110 << 8) | (bit << 7) | (rm.number() << 3) | (rd.number() & 0x7);
        }
    }

    struct Ldmia;
    impl Ldmia {
        fn opcode<T: rp2350_sim::SpControl>(rn: Register<T>, registers: Registers<T>) -> u16 {
            return (0b11001 << 11) | ((rn.number() & 0x7) << 8) | (registers.binary() & 0xff);
        }
    }

    struct Rev;
    impl Rev {
        fn opcode<T: rp2350_sim::SpControl, U: rp2350_sim::SpControl>(rd: Register<T>, rn: Register<U>) -> u16 {
            return (0b1011101000 << 6) | ((rn.number() & 0x7) << 3) | (rd.number() & 0x7);
        }
    }

    struct Rev16;
    impl Rev16 {
        fn opcode<T: rp2350_sim::SpControl, U: rp2350_sim::SpControl>(rd: Register<T>, rn: Register<U>) -> u16 {
            return (0b1011101001 << 6) | ((rn.number() & 0x7) << 3) | (rd.number() & 0x7);
        }
    }

    struct Stmia;
    impl Stmia {
        fn opcode<T: rp2350_sim::SpControl>(rn: Register<T>, registers: Registers<T>) -> u16 {
            return (0b11000 << 11) | ((rn.number() & 0x7) << 8) | (registers.binary() & 0xff);
        }
    }

    struct SubSpMinusImmediate;
    impl SubSpMinusImmediate {
        fn opcode(imm: u16) -> u16 {
            return (0b101100001 << 7) | ((imm >> 2) & 0x7f);
        }
    }

    struct Uxtb;
    impl Uxtb {
        fn opcode<T: rp2350_sim::SpControl, U: rp2350_sim::SpControl>(rd: Register<T>, rm: Register<U>) -> u16 {
            return (0b1011001011 << 6) | ((rm.number() & 7) << 3) | (rd.number() & 7);
        }
    }

    struct Uxth;
    impl Uxth {
        fn opcode<T: rp2350_sim::SpControl, U: rp2350_sim::SpControl>(rd: Register<T>, rm: Register<U>) -> u16 {
            return (0b1011001010 << 6) | ((rm.number() & 7) << 3) | (rd.number() & 7);
        }
    }

    struct Yield;
    impl Yield {
        fn opcode() -> u16 {
            return 0b1011111100010000;
        }
    }

    #[test]
    fn add_sp_plus_immediate() {
        // should execute a `add sp, 0x10` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33_registers.pc.set(RAM_START_ADDRESS);
        rp2350.cortex_m33_registers.sp.set(0x10000040);

        rp2350.write_to_address(RAM_START_ADDRESS, AddSpPlusImmediate::opcode(0x10));
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33_registers.sp.get(), 0x10000050)
    }

    #[test]
    fn add_register_sp_plus_immediate() {
        // should execute a `add r1, sp, #4` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33_registers.pc.set(RAM_START_ADDRESS);
        rp2350.cortex_m33_registers.sp.set(0x54);

        rp2350.write_to_address(RAM_START_ADDRESS, AddRegisterSpPlusImmediate::opcode(rp2350.cortex_m33_registers.r1, 0x10));
        rp2350.cortex_m33_registers.r1.set(0);
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33_registers.sp.get(), 0x54);
        assert_eq!(rp2350.cortex_m33_registers.r1.get(), 0x64);
    }

    #[test]
    fn add_register() {
        // should execute `add r1, ip` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33_registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, AddRegister::opcode(rp2350.cortex_m33_registers.r1, rp2350.cortex_m33_registers.r12));
        rp2350.cortex_m33_registers.r1.set(66);
        rp2350.cortex_m33_registers.r12.set(44);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33_registers.r1.get(), 110);
    }

    #[test]
    fn adr() {
        // should execute `adr r4, #0x50` instruction and set the overflow flag correctly
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33_registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, Adr::opcode(rp2350.cortex_m33_registers.r4, 0x50));
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33_registers.r4.get(), 0x20000054);
    }

    #[test]
    fn bt2() {
        // should execute a `b.n .-20` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33_registers.pc.set(RAM_START_ADDRESS + 9 * 2);

        rp2350.write_to_address(RAM_START_ADDRESS + 9 * 2, BT2::opcode(0xfec));
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33_registers.pc.get(), 0x20000002);
    }

    #[test]
    fn dmb_sy() {
        // should correctly decode a `dmb sy` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33_registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, DmbSy::opcode());
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33_registers.pc.get(), 0x20000004);
    }

    #[test]
    fn dsb_sy() {
        // should correctly decode a `dsb sy` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33_registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, DsbSy::opcode());
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33_registers.pc.get(), 0x20000004);
    }

    #[test]
    fn isb_sy() {
        // should correctly decode a `isb sy` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33_registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, IsbSy::opcode());
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33_registers.pc.get(), 0x20000004);
    }

    #[test]
    fn ldmia() {
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33_registers.pc.set(RAM_START_ADDRESS);

        let registers: &[Register<_>] = &[
            rp2350.cortex_m33_registers.r1,
            rp2350.cortex_m33_registers.r2
        ];

        let opcode = Ldmia::opcode(rp2350.cortex_m33_registers.r0, registers.into());
        rp2350.write_to_address(RAM_START_ADDRESS, opcode);
        rp2350.cortex_m33_registers.r0.set(0x20000010);

        rp2350.write_to_address(0x20000010, 0xf00df00d as u32);
        rp2350.write_to_address(0x20000014, 0x4242 as u16);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33_registers.pc.get(), 0x20000002);
        assert_eq!(rp2350.cortex_m33_registers.r0.get(), 0x20000018);
        assert_eq!(rp2350.cortex_m33_registers.r1.get(), 0xf00df00d);
        assert_eq!(rp2350.cortex_m33_registers.r2.get(), 0x4242);
    }

    #[test]
    fn mov() {
        // should execute a `mov r3, r8` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33_registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, Mov::opcode(rp2350.cortex_m33_registers.r3, rp2350.cortex_m33_registers.r8));
        rp2350.cortex_m33_registers.r8.set(55);
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33_registers.r3.get(), 55);
    }

    #[test]
    fn mov_pc() {
        // should execute a `mov r3, pc` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33_registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, Mov::opcode(rp2350.cortex_m33_registers.r3, rp2350.cortex_m33_registers.pc));
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33_registers.r3.get(), 0x20000004);
    }

    #[test]
    fn push_instruction() {
        // should execute a `push {r4, r5, r6, lr}` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33_registers.pc.set(RAM_START_ADDRESS);
        rp2350.cortex_m33_registers.sp.set(RAM_START_ADDRESS + 0x100);
        
        let registers: &[Register<_>] = &[
            rp2350.cortex_m33_registers.r4,
            rp2350.cortex_m33_registers.r5,
            rp2350.cortex_m33_registers.r6
        ];
        let binary = Push::opcode(true, registers.into());

        rp2350.write_to_address(RAM_START_ADDRESS, binary);

        rp2350.cortex_m33_registers.r4.set(0x40);
        rp2350.cortex_m33_registers.r5.set(0x50);
        rp2350.cortex_m33_registers.r6.set(0x60);
        rp2350.cortex_m33_registers.lr.set(0x42);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33_registers.sp.get(), RAM_START_ADDRESS + 0xf0);
        assert_eq!(rp2350.sram[0xf0], 0x40);
        assert_eq!(rp2350.sram[0xf4], 0x50);
        assert_eq!(rp2350.sram[0xf8], 0x60);
        assert_eq!(rp2350.sram[0xfc], 0x42);
    }

    #[test]
    fn rev() {
        // should execute a `rev r3, r1` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33_registers.pc.set(RAM_START_ADDRESS);
        
        rp2350.write_to_address(RAM_START_ADDRESS, Rev::opcode(rp2350.cortex_m33_registers.r2, rp2350.cortex_m33_registers.r3));
        
        rp2350.cortex_m33_registers.r3.set(0x11223344);
        
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33_registers.r2.get(), 0x44332211);
    }

    #[test]
    fn rev16() {
        // should execute a `rev16 r0, r5` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33_registers.pc.set(RAM_START_ADDRESS);
        
        rp2350.write_to_address(RAM_START_ADDRESS, Rev16::opcode(rp2350.cortex_m33_registers.r0, rp2350.cortex_m33_registers.r5));
        
        rp2350.cortex_m33_registers.r5.set(0x11223344);
        
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33_registers.r0.get(), 0x22114433);
    }

    #[test]
    fn stmia() {
        // should execute a `stmia r0!, {r1, r2}` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33_registers.pc.set(RAM_START_ADDRESS);

        let registers: &[Register<_>] = &[
            rp2350.cortex_m33_registers.r1,
            rp2350.cortex_m33_registers.r2,
        ];
        
        rp2350.write_to_address(RAM_START_ADDRESS, Stmia::opcode(rp2350.cortex_m33_registers.r0, registers.into()));
        
        rp2350.cortex_m33_registers.r0.set(0x20000010);
        rp2350.cortex_m33_registers.r1.set(0xf00df00d);
        rp2350.cortex_m33_registers.r2.set(0x4242);
        
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33_registers.pc.get(), 0x20000002);
        assert_eq!(rp2350.cortex_m33_registers.r0.get(), 0x20000018);

        assert_eq!(rp2350.read_u32_from_address(0x20000010), 0xf00df00d);
        assert_eq!(rp2350.read_u32_from_address(0x20000014), 0x4242);
    }

    #[test]
    fn sub_sp_minus_immediate() {
        // should execute a `sub sp, 0x10` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33_registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, SubSpMinusImmediate::opcode(0x10));
        
        rp2350.cortex_m33_registers.sp.set(0x10000040);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33_registers.sp.get(), 0x10000030);
    }

    #[test]
    fn uxtb() {
        // should execute an `uxtb r5, r3` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33_registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, Uxtb::opcode(rp2350.cortex_m33_registers.r5, rp2350.cortex_m33_registers.r3));
        
        rp2350.cortex_m33_registers.r3.set(0x12345678);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33_registers.r5.get(), 0x78);
    }

    #[test]
    fn uxth() {
        // should execute an `uxtb r3, r1` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33_registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, Uxth::opcode(rp2350.cortex_m33_registers.r3, rp2350.cortex_m33_registers.r1));
        
        rp2350.cortex_m33_registers.r1.set(0x12345678);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33_registers.r3.get(), 0x5678);
    }

    #[test]
    fn r#yield() {
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33_registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, Yield::opcode());

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33_registers.pc.get(), 0x20000002);
    }
}
















