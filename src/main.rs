use crate::cpu::Instruction::*;
use crate::cpu::CPU;

mod cpu;

fn main() {
	let mut cpu = CPU::create();

	cpu.reset();
	cpu.execute(LDA_IM);
	cpu.execute(LDA_ABSX);
}
