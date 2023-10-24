pub struct HRAM {
    ram: Vec<u8>
}

impl HRAM {
    pub fn new() -> Self {
        Self {
            ram: vec![0x00; 0x7F]
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.ram[(address - 0xFF80) as usize]
    }

    pub fn write(
        &mut self,
        address: u16,
        value: u8
    ) {
        self.ram[(address - 0xFF80) as usize] = value;
    }
}
