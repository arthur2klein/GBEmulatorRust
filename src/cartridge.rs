use std::fs::File;

pub struct Cartridge {
    rom: Vec<u8>,
    ram: Vec<u8>
}

impl Cartridge {
    pub fn new(
        file_path: &str
    ) -> Self {
        let file = File::open(file_path).expect("Cannot read the cartridge.");
        let mut rom: Vec<u8> = Vec::new();
        file.read_to_end(&mut rom);
        Self {
            rom,
            ram: vec![0;0x2000],
        }
    }

    pub fn read_rom(&self, adress: u16) -> u8 {
        rom[adress]
    }

    pub fn write_rom(
        &mut self,
        adress: u16,
        value: u8
    ) {
        rom[adress] = value;
    }

    pub fn read_ram(&self, adress: u16) -> u8 {
        ram[adress - 0xA000]
    }

    pub fn write_ram(
        &mut self,
        adress: u16,
        value: u8
    ) {
        ram[adress - 0xA000] = value;
    }
}
