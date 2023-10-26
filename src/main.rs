use std::fs;
use std::io;
use gb_emulator_rust::cpu::CPU;

const CARTRIDGES_FOLDER_NAME: &str = "cartridges";

fn chose_cartridge() -> String {
    for (i, path) in fs::read_dir(CARTRIDGES_FOLDER_NAME)
        .unwrap()
        .enumerate() {
        println!(
            "Chose {} for {}",
            i,
            path.unwrap().path().display()
        );
    }
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read the answer");
    format!(
        "{}",
        fs::read_dir(CARTRIDGES_FOLDER_NAME)
            .unwrap()
            .nth(
                answer
                    .trim()
                    .parse()
                    .expect("Integer not found")
            ).expect("Wrong value")
            .unwrap()
            .path()
            .display()
    )
}

fn main() {
    let cartridge_name = chose_cartridge();
    let mut cpu = CPU::new(&cartridge_name);
    cpu.run();
}
