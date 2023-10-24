use gb_emulator_rust::cpu::CPU;

fn main() {
    let mut cpu = CPU::new("Pokemon Red.gb");
    cpu.run();
}
