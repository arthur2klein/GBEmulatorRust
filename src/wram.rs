/// Contains the data of the Working ram
pub struct WRAM {
    ram: Vec<u8>
}

impl WRAM {
    /// Create the WRAM with no data
    ///
    /// # Returns
    /// **WRAM**: New wram without any data
    pub fn new() -> Self {
        Self {
            ram: vec![0x00; 0x2000]
        }
    }

    /// Read a byte in the WRAM
    ///
    /// # Arguments
    /// **address (u16)**: Address of the byte to read
    ///
    /// # Returns
    /// **u8**: Value of the byte at the given address
    pub fn read(&self, address: u16) -> u8 {
        self.ram[(address - 0xC000) as usize]
    }

    /// Modify a byte in the WRAM
    ///
    /// # Arguments
    /// **address (u16)**: Address of the byte to read
    /// **value (u8)**: New value of the byte at the given address
    pub fn write(
        &mut self,
        address: u16,
        value: u8
    ) {
        self.ram[(address - 0xC000) as usize] = value;
    }
}
