use std::fs::File;
use std::io::Read;

/// Contains the memory of a game cartridge
pub struct Cartridge {
    /// Rom of the cartridge containing its code
    rom: Vec<u8>,
    /// Ram of the cartridge containing the save
    ram: Vec<u8>,
}

impl Cartridge {
    /// Initialize the memory of the cartridge
    ///
    /// # Arguments
    /// **file_path (&str)**: Name of the file containing the game.
    ///
    /// # Returns
    /// **Cartridge**: New cartridge for the given game.
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

    /// Read a byte in the rom
    ///
    /// # Arguments
    /// **address (u16)**: Address of the byte
    ///
    /// # Returns
    /// **u8**: Byte of the rom at the given address
    pub fn read_rom(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    /// Change a byte in the rom
    ///
    /// # Arguments
    /// **address (u16)**: Address of the byte
    /// **value (u8)**: New value of the byte at the given address
    pub fn write_rom(
        &mut self,
        address: u16,
        value: u8
    ) {
        self.rom[address as usize] = value;
    }

    /// Read a byte in the ram of the cartridge
    ///
    /// # Arguments
    /// **address (u16)**: Address of the byte
    ///
    /// # Returns
    /// **u8**: Byte of the ram at the given address
    pub fn read_ram(&self, address: u16) -> u8 {
        self.ram[(address - 0xA000) as usize]
    }

    /// Change a byte in the ram of the cartridge
    ///
    /// # Arguments
    /// **address (u16)**: Address of the byte
    /// **value (u8)**: New value of the byte at the given address
    pub fn write_ram(
        &mut self,
        address: u16,
        value: u8
    ) {
        self.ram[(address - 0xA000) as usize] = value;
    }
}
