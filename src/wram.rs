pub struct WRAM {
    ram: Vec<u8>
}

impl WRAM {
    pub fn new() -> Self {
        Self {
            ram: vec![0x00; 0x2000]
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        self.ram[(address - 0xC000) as usize]
    }

    pub fn write(
        &mut self,
        address: u16,
        value: u8
    ) {
        self.ram[(address - 0xC000) as usize] = value;
    }
}
