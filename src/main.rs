use crate::cpu::CPU;

fn main() {
    let cpu = CPU::new("../Pokemon Red.gb");
    cpu.run();
}
