use std::fs::{File, metadata, create_dir};
use std::io::Read;
use std::io::Write;

/// Contains the memory of a game cartridge
pub struct Cartridge {
    /// Rom of the cartridge containing its code
    rom: Vec<u8>,
    /// Ram of the cartridge containing the save
    ram: Vec<u8>,
    /// Path of the save file
    save_file: String,
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
        Self::check_folder_save();
        let mut file = File::open(file_path)
            .expect("Cannot read the cartridge.");
        let mut rom: Vec<u8> = Vec::new();
        file.read_to_end(&mut rom).unwrap();
        let game_name = file_path.rsplit_once('/').unwrap().1;
        let save_file = format!("save/{}.save", game_name);
        Self {
            rom,
            ram: Self::ram_from_save(&save_file),
            save_file,
        }
    }

    /// Checks that the folder save exists
    /// Create a folder save if none exists
    fn check_folder_save() {
        if metadata("save/").is_err() {
            create_dir("save").unwrap();
        }
    }

    /// Create the ram using an existing save file
    ///
    /// If no save file is found, an empy ram will be created.
    ///
    /// # Arguments
    /// **save_name (&str)**: Path of the save file
    ///
    /// # Returns
    /// **`Vec<u8>`**: Ram of the cartridge
    fn ram_from_save(save_name: &str) -> Vec<u8> {
        match File::open(save_name) {
            Ok(mut file) => {
                let mut res: Vec<u8> = Vec::new();
                file.read_to_end(&mut res).unwrap();
                res
            },
            Err(_) => {
                vec![0x00; 0x2000]
            }
        }
    }

    /// Function called when the cartridge is no longer needed
    ///
    /// Save the state of the ram
    pub fn close(&self) {
        self.save();
    }

    /// Save the current state of the ram
    ///
    /// The file will be either truncated or created
    fn save(&self) {
        let mut file = File::create(&self.save_file).unwrap();
        file.write_all(&self.ram).unwrap();
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
