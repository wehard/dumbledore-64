// 6502/6510

const NEGATIVE_FLAG_BIT: u8 = 1 << 7;
const OVERFLOW_FLAG_BIT: u8 = 1 << 6;
const UNUSED_FLAG_BIT: u8 = 1 << 5;
const BREAK_FLAG_BIT: u8 = 1 << 4;
const INTERRUPT_DISABLE_FLAG_BIT: u8 = 1 << 2;
const ZERO_FLAG_BIT: u8 = 1 << 0;

pub enum Instruction {
	//LDA
	LDA_IM = 0xA9,
	LDA_ZP = 0xA5,
	LDA_ZPX = 0xB5,
	LDA_ABS = 0xAD,
	LDA_ABSX = 0xBD,
	LDA_ABSY = 0xB9,
	LDA_INDX = 0xA1,
	LDA_INDY = 0xB1,
	//LDX
	LDX_IM = 0xA2,
	LDX_ZP = 0xA6,
	LDX_ZPY = 0xB6,
	LDX_ABS = 0xAE,
	LDX_ABSY = 0xBE,
	//LDY
	LDY_IM = 0xA0,
	LDY_ZP = 0xA4,
	LDY_ZPX = 0xB4,
	LDY_ABS = 0xAC,
	LDY_ABSX = 0xBC,
}

pub struct CPU {
	pc: u8, // program counter
	sp: u8, // stack pointer
	a: u8,  // a register
	x: u8,  // x register
	y: u8,  // y register
	ps: u8, // proc status
}

impl CPU {
	pub fn create() -> CPU {
		CPU {
			pc: 0,
			sp: 0,
			a: 0,
			x: 0,
			y: 0,
			ps: 0,
		}
	}
	pub fn reset(&mut self) {
		println!("Reset CPU");
		self.pc = 0;
		self.sp = 0;
		self.a = 0;
		self.x = 0;
		self.y = 0;
		self.ps = 0;
	}

	pub fn execute(&self, instruction: Instruction) {
		match instruction {
			Instruction::LDA_IM => {
				println!("Load accumulator immediate");
			}
			_ => {
				println!("Instruction not handled!");
			}
		}
	}
}
