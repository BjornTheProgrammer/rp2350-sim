#[cfg(test)]
mod tests {
    use rp2350_sim::cortex_m33::registers;
    use rp2350_sim::{RAM_START_ADDRESS, RP2350};
    use rp2350_sim::cortex_m33::{self, registers::Register};

    struct Registers<'a, T: registers::SpControl> {
        register_numbers: &'a [Register<T>]
    }

    impl<T: cortex_m33::registers::SpControl> Registers<'_, T> {
        fn binary(&self) -> u16 {
            let mut binary = 0;
            for number in self.register_numbers.iter() {
                binary = binary | (1 << number.number());
            }

            return binary;
        }
    }

    // Implementing `From<&'a [u8]>` for `Registers<'a>`
    impl<'a, T: cortex_m33::registers::SpControl> From<&'a [Register<T>]> for Registers<'a, T> {
        fn from(array: &'a [Register<T>]) -> Self {
            Registers {
                register_numbers: array,
            }
        }
    }

    // Implementing `Into<&'a [u8]>` for `Registers<'a>`
    impl<'a, T: cortex_m33::registers::SpControl> Into<&'a [Register<T>]> for Registers<'a, T> {
        fn into(self) -> &'a [Register<T>] {
            self.register_numbers
        }
    }

    struct Push;
    impl Push {
        fn opcode<T: cortex_m33::registers::SpControl>(push_to_lr: bool, registers: Registers<T>) -> u16 {
            return (0b1011010 << 9) | ((push_to_lr as u16) << 8) | registers.binary();
        }
    }

    struct Adcs;
    impl Adcs {
        fn opcode<T: cortex_m33::registers::SpControl, U: cortex_m33::registers::SpControl>(rdn: Register<T>, rm: Register<U>) -> u16 {
            return (0b0100000101 << 6) | ((rm.number() & 7) << 3) | (rdn.number() & 7);
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
        fn opcode<T: cortex_m33::registers::SpControl>(rd: Register<T>, imm8: u16) -> u16 {
            return (0b10101 << 11) | ((rd.number() & 7) << 8) | ((imm8 >> 2) & 0xff);
        }
    }

    struct AddsT1;
    impl AddsT1 {
        fn opcode<T: cortex_m33::registers::SpControl, U: cortex_m33::registers::SpControl>(rd: Register<T>, rn: Register<U>,  imm3: u16) -> u16 {
            return (0b0001110 << 9) | ((imm3 & 0x7) << 6) | ((rn.number() & 7) << 3) | (rd.number() & 7);
        }
    }

    struct AddsT2;
    impl AddsT2 {
        fn opcode<T: cortex_m33::registers::SpControl>(rdn: Register<T>,  imm8: u16) -> u16 {
            return (0b00110 << 11) | ((rdn.number() & 7) << 8) | (imm8 & 0xff);
        }
    }

    struct AddsRegister;
    impl AddsRegister {
        fn opcode<T: cortex_m33::registers::SpControl, U: cortex_m33::registers::SpControl, V: cortex_m33::registers::SpControl>(rd: Register<T>, rn: Register<U>, rm: Register<V>) -> u16 {
            return (0b0001100 << 9) | ((rm.number() & 0x7) << 6) | ((rn.number() & 7) << 3) | (rd.number() & 7);
        }
    }

    struct AddRegister;
    impl AddRegister {
        fn opcode<T: cortex_m33::registers::SpControl, U: cortex_m33::registers::SpControl>(rdn: Register<T>, rm: Register<U>) -> u16 {
            return (0b01000100 << 8) | ((rdn.number() & 0x8) << 4) | ((rm.number() & 0xf) << 3) | (rdn.number() & 0x7);
        }
    }

    struct Adr;
    impl Adr {
        fn opcode<T: cortex_m33::registers::SpControl>(rd: Register<T>, imm8: u16) -> u16 {
            return (0b10100 << 11) | ((rd.number() & 7) << 8) | ((imm8 >> 2) & 0xff);
        }
    }

    struct AndsT1;
    impl AndsT1 {
        fn opcode<T: cortex_m33::registers::SpControl, U: cortex_m33::registers::SpControl>(rn: Register<T>, rm: Register<U>) -> u16 {
            return (0b0100000000 << 6) | ((rm.number() & 7) << 3) | (rn.number() & 0x7);
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
        fn opcode<T: cortex_m33::registers::SpControl, U: cortex_m33::registers::SpControl>(rd: Register<T>, rm: Register<U>) -> u16 {
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
        fn opcode<T: cortex_m33::registers::SpControl>(rn: Register<T>, registers: Registers<T>) -> u16 {
            return (0b11001 << 11) | ((rn.number() & 0x7) << 8) | (registers.binary() & 0xff);
        }
    }

    struct Rev;
    impl Rev {
        fn opcode<T: cortex_m33::registers::SpControl, U: cortex_m33::registers::SpControl>(rd: Register<T>, rn: Register<U>) -> u16 {
            return (0b1011101000 << 6) | ((rn.number() & 0x7) << 3) | (rd.number() & 0x7);
        }
    }

    struct Rev16;
    impl Rev16 {
        fn opcode<T: cortex_m33::registers::SpControl, U: cortex_m33::registers::SpControl>(rd: Register<T>, rn: Register<U>) -> u16 {
            return (0b1011101001 << 6) | ((rn.number() & 0x7) << 3) | (rd.number() & 0x7);
        }
    }

    struct Stmia;
    impl Stmia {
        fn opcode<T: cortex_m33::registers::SpControl>(rn: Register<T>, registers: Registers<T>) -> u16 {
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
        fn opcode<T: cortex_m33::registers::SpControl, U: cortex_m33::registers::SpControl>(rd: Register<T>, rm: Register<U>) -> u16 {
            return (0b1011001011 << 6) | ((rm.number() & 7) << 3) | (rd.number() & 7);
        }
    }

    struct Uxth;
    impl Uxth {
        fn opcode<T: cortex_m33::registers::SpControl, U: cortex_m33::registers::SpControl>(rd: Register<T>, rm: Register<U>) -> u16 {
            return (0b1011001010 << 6) | ((rm.number() & 7) << 3) | (rd.number() & 7);
        }
    }

    struct Yield;
    impl Yield {
        fn opcode() -> u16 {
            return 0b1011111100010000;
        }
    }

    struct Bl;
    impl Bl {
        fn opcode(imm: i32) -> u32 {
            let imm11 = (imm >> 1) & 0x7ff;
            let imm10 = (imm >> 12) & 0x3ff;
            let s = if imm < 0 { 1 } else { 0 };
            let j2 = 1 - (((imm >> 22) & 0x1) ^ s);
            let j1 = 1 - (((imm >> 23) & 0x1) ^ s);
            let opcode = (0b1101 << 28) | (j1 << 29) | (j2 << 27) | (imm11 << 16) | (0b11110 << 11) | (s << 10) | imm10;
            return opcode as u32 >> 0;
        }
    }

    struct Blx;
    impl Blx {
        fn opcode<T: cortex_m33::registers::SpControl>(rm: Register<T>) -> u16 {
            return (0b010001111 << 7) | (rm.number() << 3);
        }
    }

    #[test]
    fn adcs() {
        // should execute `adcs r5, r4` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, Adcs::opcode(rp2350.cortex_m33.registers.r5, rp2350.cortex_m33.registers.r4));
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

        rp2350.write_to_address(RAM_START_ADDRESS, Adcs::opcode(rp2350.cortex_m33.registers.r5, rp2350.cortex_m33.registers.r4));
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

        rp2350.write_to_address(RAM_START_ADDRESS, Adcs::opcode(rp2350.cortex_m33.registers.r3, rp2350.cortex_m33.registers.r2));
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

        rp2350.write_to_address(RAM_START_ADDRESS, Adcs::opcode(rp2350.cortex_m33.registers.r0, rp2350.cortex_m33.registers.r0));
        rp2350.cortex_m33.registers.r0.set(0x80000000); // Max signed INT32
        rp2350.cortex_m33.apsr.set_c(false);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r0.get(), 0);
        assert_eq!(rp2350.cortex_m33.apsr.n(), false);
        assert_eq!(rp2350.cortex_m33.apsr.z(), true);
        assert_eq!(rp2350.cortex_m33.apsr.c(), true);
        assert_eq!(rp2350.cortex_m33.apsr.v(), true);
    }

    #[test]
    fn add_sp_plus_immediate() {
        // should execute a `add sp, 0x10` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);
        rp2350.cortex_m33.registers.sp.set(0x10000040);

        rp2350.write_to_address(RAM_START_ADDRESS, AddSpPlusImmediate::opcode(0x10));
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.sp.get(), 0x10000050)
    }

    #[test]
    fn add_register_sp_plus_immediate() {
        // should execute a `add r1, sp, #4` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);
        rp2350.cortex_m33.registers.sp.set(0x54);

        rp2350.write_to_address(RAM_START_ADDRESS, AddRegisterSpPlusImmediate::opcode(rp2350.cortex_m33.registers.r1, 0x10));
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

        rp2350.write_to_address(RAM_START_ADDRESS, AddsT1::opcode(rp2350.cortex_m33.registers.r1, rp2350.cortex_m33.registers.r2, 3));
        rp2350.cortex_m33.registers.r2.set(2);
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r1.get(), 5);
        assert_eq!(rp2350.cortex_m33.apsr.n(), false);
        assert_eq!(rp2350.cortex_m33.apsr.z(), false);
        assert_eq!(rp2350.cortex_m33.apsr.c(), false);
        assert_eq!(rp2350.cortex_m33.apsr.v(), false);
    }

    #[test]
    fn adds_t2() {
        // should execute `adds r1, #1` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, AddsT2::opcode(rp2350.cortex_m33.registers.r1, 1));
        rp2350.cortex_m33.registers.r1.set(0xffffffff);
        rp2350.execute_instruction();

        println!("opcode: {:#x}", AddsT2::opcode(rp2350.cortex_m33.registers.r1, 1));
        println!("apsr: {:?}", rp2350.cortex_m33.apsr);

        assert_eq!(rp2350.cortex_m33.registers.r1.get(), 0);
        assert_eq!(rp2350.cortex_m33.apsr.n(), false);
        assert_eq!(rp2350.cortex_m33.apsr.z(), true);
        assert_eq!(rp2350.cortex_m33.apsr.c(), true);
        assert_eq!(rp2350.cortex_m33.apsr.v(), false);
    }

    #[test]
    fn adds_register() {
        // should execute `adds r1, r2, r7` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, AddsRegister::opcode(rp2350.cortex_m33.registers.r1, rp2350.cortex_m33.registers.r2, rp2350.cortex_m33.registers.r7));
        rp2350.cortex_m33.registers.r2.set(2);
        rp2350.cortex_m33.registers.r7.set(27);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r1.get(), 29);
        assert_eq!(rp2350.cortex_m33.apsr.n(), false);
        assert_eq!(rp2350.cortex_m33.apsr.z(), false);
        assert_eq!(rp2350.cortex_m33.apsr.c(), false);
        assert_eq!(rp2350.cortex_m33.apsr.v(), false);
    }

    #[test]
    fn adds_register_2() {
        // should execute `adds r4, r4, r2` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, AddsRegister::opcode(rp2350.cortex_m33.registers.r4, rp2350.cortex_m33.registers.r4, rp2350.cortex_m33.registers.r2));
        rp2350.cortex_m33.registers.r2.set(0x74bc8000);
        rp2350.cortex_m33.registers.r4.set(0x43740000);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r4.get(), 0xb8308000);
        assert_eq!(rp2350.cortex_m33.apsr.n(), true);
        assert_eq!(rp2350.cortex_m33.apsr.z(), false);
        assert_eq!(rp2350.cortex_m33.apsr.c(), false);
        assert_eq!(rp2350.cortex_m33.apsr.v(), true);
    }

    #[test]
    fn adds_register_3() {
        // should execute `adds r1, r1, r1` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, AddsRegister::opcode(rp2350.cortex_m33.registers.r1, rp2350.cortex_m33.registers.r1, rp2350.cortex_m33.registers.r1));
        rp2350.cortex_m33.registers.r1.set(0xbf8d1424);
        rp2350.cortex_m33.apsr.set_c(true);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r1.get(), 0x7f1a2848);
        assert_eq!(rp2350.cortex_m33.apsr.n(), false);
        assert_eq!(rp2350.cortex_m33.apsr.z(), false);
        assert_eq!(rp2350.cortex_m33.apsr.c(), true);
        assert_eq!(rp2350.cortex_m33.apsr.v(), true);
    }

    #[test]
    fn add_register() {
        // should execute `add r1, ip` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, AddRegister::opcode(rp2350.cortex_m33.registers.r1, rp2350.cortex_m33.registers.r12));
        rp2350.cortex_m33.registers.r1.set(66);
        rp2350.cortex_m33.registers.r12.set(44);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r1.get(), 110);
    }

    #[test]
    fn adr() {
        // should execute `adr r4, #0x50` instruction and set the overflow flag correctly
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, Adr::opcode(rp2350.cortex_m33.registers.r4, 0x50));
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r4.get(), 0x20000054);
    }

    #[test]
    fn ands_t1() {
        // should execute `ands r5, r0` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, AndsT1::opcode(rp2350.cortex_m33.registers.r5, rp2350.cortex_m33.registers.r0));
        rp2350.cortex_m33.registers.r5.set(0xffff0000);
        rp2350.cortex_m33.registers.r0.set(0xf00fffff);
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r5.get(), 0xf00f0000);
        assert_eq!(rp2350.cortex_m33.apsr.n(), true);
        assert_eq!(rp2350.cortex_m33.apsr.z(), false);
    }

    #[test]
    fn bt2() {
        // should execute a `b.n .-20` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS + 9 * 2);

        rp2350.write_to_address(RAM_START_ADDRESS + 9 * 2, BT2::opcode(0xfec));
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000002);
    }

    #[test]
    fn bl() {
        // should execute `bl 0x34` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, Bl::opcode(0x34));

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000038);
        assert_eq!(rp2350.cortex_m33.registers.lr.get(), 0x20000005);
    }

    #[test]
    fn bl2() {
        // should execute `bl -0x10` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, Bl::opcode(-0x10));

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000004 - 0x10);
        assert_eq!(rp2350.cortex_m33.registers.lr.get(), 0x20000005);
    }

    #[test]
    fn bl3() {
        // should execute `bl -3242` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, Bl::opcode(-3242));

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000004 - 3242);
        assert_eq!(rp2350.cortex_m33.registers.lr.get(), 0x20000005);
    }

    #[test]
    fn blx() {
        // should execute `blx r3` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);
        rp2350.cortex_m33.registers.r3.set(0x20000201);

        rp2350.write_to_address(RAM_START_ADDRESS, Blx::opcode(rp2350.cortex_m33.registers.r3));

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000200);
        assert_eq!(rp2350.cortex_m33.registers.lr.get(), 0x20000003);
    }

    #[test]
    fn dmb_sy() {
        // should correctly decode a `dmb sy` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, DmbSy::opcode());
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000004);
    }

    #[test]
    fn dsb_sy() {
        // should correctly decode a `dsb sy` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, DsbSy::opcode());
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000004);
    }

    #[test]
    fn isb_sy() {
        // should correctly decode a `isb sy` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, IsbSy::opcode());
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000004);
    }

    #[test]
    fn ldmia() {
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        let registers: &[Register<_>] = &[
            rp2350.cortex_m33.registers.r1,
            rp2350.cortex_m33.registers.r2
        ];

        let opcode = Ldmia::opcode(rp2350.cortex_m33.registers.r0, registers.into());
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

    #[test]
    fn mov() {
        // should execute a `mov r3, r8` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, Mov::opcode(rp2350.cortex_m33.registers.r3, rp2350.cortex_m33.registers.r8));
        rp2350.cortex_m33.registers.r8.set(55);
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r3.get(), 55);
    }

    #[test]
    fn mov_pc() {
        // should execute a `mov r3, pc` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, Mov::opcode(rp2350.cortex_m33.registers.r3, rp2350.cortex_m33.registers.pc));
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r3.get(), 0x20000004);
    }

    #[test]
    fn push_instruction() {
        // should execute a `push {r4, r5, r6, lr}` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);
        rp2350.cortex_m33.registers.sp.set(RAM_START_ADDRESS + 0x100);
        
        let registers: &[Register<_>] = &[
            rp2350.cortex_m33.registers.r4,
            rp2350.cortex_m33.registers.r5,
            rp2350.cortex_m33.registers.r6
        ];
        let binary = Push::opcode(true, registers.into());

        rp2350.write_to_address(RAM_START_ADDRESS, binary);

        rp2350.cortex_m33.registers.r4.set(0x40);
        rp2350.cortex_m33.registers.r5.set(0x50);
        rp2350.cortex_m33.registers.r6.set(0x60);
        rp2350.cortex_m33.registers.lr.set(0x42);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.sp.get(), RAM_START_ADDRESS + 0xf0);
        assert_eq!(rp2350.sram[0xf0], 0x40);
        assert_eq!(rp2350.sram[0xf4], 0x50);
        assert_eq!(rp2350.sram[0xf8], 0x60);
        assert_eq!(rp2350.sram[0xfc], 0x42);
    }

    #[test]
    fn rev() {
        // should execute a `rev r3, r1` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);
        
        rp2350.write_to_address(RAM_START_ADDRESS, Rev::opcode(rp2350.cortex_m33.registers.r2, rp2350.cortex_m33.registers.r3));
        
        rp2350.cortex_m33.registers.r3.set(0x11223344);
        
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r2.get(), 0x44332211);
    }

    #[test]
    fn rev16() {
        // should execute a `rev16 r0, r5` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);
        
        rp2350.write_to_address(RAM_START_ADDRESS, Rev16::opcode(rp2350.cortex_m33.registers.r0, rp2350.cortex_m33.registers.r5));
        
        rp2350.cortex_m33.registers.r5.set(0x11223344);
        
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r0.get(), 0x22114433);
    }

    #[test]
    fn stmia() {
        // should execute a `stmia r0!, {r1, r2}` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        let registers: &[Register<_>] = &[
            rp2350.cortex_m33.registers.r1,
            rp2350.cortex_m33.registers.r2,
        ];
        
        rp2350.write_to_address(RAM_START_ADDRESS, Stmia::opcode(rp2350.cortex_m33.registers.r0, registers.into()));
        
        rp2350.cortex_m33.registers.r0.set(0x20000010);
        rp2350.cortex_m33.registers.r1.set(0xf00df00d);
        rp2350.cortex_m33.registers.r2.set(0x4242);
        
        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000002);
        assert_eq!(rp2350.cortex_m33.registers.r0.get(), 0x20000018);

        assert_eq!(rp2350.read_u32_from_address(0x20000010), 0xf00df00d);
        assert_eq!(rp2350.read_u32_from_address(0x20000014), 0x4242);
    }

    #[test]
    fn sub_sp_minus_immediate() {
        // should execute a `sub sp, 0x10` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, SubSpMinusImmediate::opcode(0x10));
        
        rp2350.cortex_m33.registers.sp.set(0x10000040);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.sp.get(), 0x10000030);
    }

    #[test]
    fn uxtb() {
        // should execute an `uxtb r5, r3` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, Uxtb::opcode(rp2350.cortex_m33.registers.r5, rp2350.cortex_m33.registers.r3));
        
        rp2350.cortex_m33.registers.r3.set(0x12345678);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r5.get(), 0x78);
    }

    #[test]
    fn uxth() {
        // should execute an `uxtb r3, r1` instruction
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, Uxth::opcode(rp2350.cortex_m33.registers.r3, rp2350.cortex_m33.registers.r1));
        
        rp2350.cortex_m33.registers.r1.set(0x12345678);

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.r3.get(), 0x5678);
    }

    #[test]
    fn r#yield() {
        let mut rp2350: RP2350 = RP2350::new();
        rp2350.cortex_m33.registers.pc.set(RAM_START_ADDRESS);

        rp2350.write_to_address(RAM_START_ADDRESS, Yield::opcode());

        rp2350.execute_instruction();

        assert_eq!(rp2350.cortex_m33.registers.pc.get(), 0x20000002);
    }
}
















