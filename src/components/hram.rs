/// Contains the data of the High ram
pub struct Hram {
    ram: Vec<u8>
}

impl Hram {
    /// Create the HRAM with no data
    ///
    /// # Returns
    /// **HRAM**: New hram without any data
    pub fn new() -> Self {
        Self {
            ram: vec![0x00; 0x7F]
        }
    }

    /// Read a byte in the HRAM
    ///
    /// # Arguments
    /// **address (u16)**: Address of the byte to read
    ///
    /// # Returns
    /// **u8**: Value of the byte at the given address
    pub fn read(&self, address: u16) -> u8 {
        self.ram[(address - 0xFF80) as usize]
    }

    /// Modify a byte in the HRAM
    ///
    /// # Arguments
    /// **address (u16)**: Address of the byte to read
    /// **value (u8)**: New value of the byte at the given address
    pub fn write(
        &mut self,
        address: u16,
        value: u8
    ) {
        self.ram[(address - 0xFF80) as usize] = value;
    }
}
