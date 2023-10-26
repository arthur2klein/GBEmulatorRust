use std::fs;
use std::io;
use gb_emulator_rust::cpu::CPU;

/// Name of the foler where the cartridge will be searched
const CARTRIDGES_FOLDER_NAME: &str = "cartridges";

/// Allow the user to chose one of the file of the cartridge folder
///
/// # Returns
/// **String**: Name of the chosen file
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

/// Emulate a GameBoy DMG
fn main() {
    let cartridge_name = chose_cartridge();
    let mut cpu = CPU::new(&cartridge_name);
    cpu.run();
}
