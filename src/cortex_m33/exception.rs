use super::CortexM33;

pub enum InterruptException {
	PendSV,
	SysTick,
	ExternalInterrupt(u8)
}

pub enum Exception {
	Reset,
	NMI,
	HardFault,
	SVCall,
	Interrupt(InterruptException)
}

impl Exception {
	pub fn number(&self) -> u8 {
		match self {
			Exception::Reset => 1,
			Exception::NMI => 2,
			Exception::HardFault => 3,
			Exception::SVCall => 11,
			Exception::Interrupt(interrupt) => match interrupt {
				InterruptException::PendSV => 14,
				InterruptException::SysTick => 15,
				InterruptException::ExternalInterrupt(val) => 16 + val,
			},
		}
	}
}

pub struct Exceptions {
	reset: bool,
	nmi: bool,
	hardfault: bool,
	svcall: bool
}

impl Exceptions {
	pub fn new() -> Self {
		Self {
			reset: false,
			nmi: false,
			hardfault: false,
			svcall: false
		}
	}

	pub fn priotiry(cortex: &CortexM33, n: u8) -> i8 {
		assert!(n >= 1 && n <= 48);

		let result = if n == Exception::Reset.number() {
			-3
		} else if n == Exception::NMI.number() {
			-2
		} else if n == Exception::HardFault.number() {
			-1
		} else if n == Exception::SVCall.number() {
			cortex.shpr.pri_11() as i8
		} else if n == Exception::Interrupt(InterruptException::PendSV).number() {
			cortex.shpr.pri_14() as i8
		} else if n == Exception::Interrupt(InterruptException::SysTick).number() {
			cortex.shpr.pri_15() as i8
		} else if n >= 16 {
			let r: u8 = (n - 16) / 4;
			let v = n % 4;
			
			0

		} else {
			4
		}

		match n {
			_ => unreachable!()
		}

		todo!()
	}
}