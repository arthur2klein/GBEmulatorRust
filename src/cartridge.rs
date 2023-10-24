use std::fs::File;
use std::io::Read;

pub struct Cartridge {
    rom: Vec<u8>,
    ram: Vec<u8>
}

impl Cartridge {
    pub fn new(
        file_path: &str
    ) -> Self {
        let mut file = File::open(file_path)
            .expect("Cannot read the cartridge.");
        let mut rom: Vec<u8> = Vec::new();
        file.read_to_end(&mut rom).unwrap();
        Self {
            rom,
            ram: vec![0; 0x2000],
        }
    }

    pub fn read_rom(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    pub fn write_rom(
        &mut self,
        address: u16,
        value: u8
    ) {
        self.rom[address as usize] = value;
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        self.ram[(address - 0xA000) as usize]
    }

    pub fn write_ram(
        &mut self,
        address: u16,
        value: u8
    ) {
        self.ram[(address - 0xA000) as usize] = value;
    }
}
