pub struct WRAM {
    ram: Vec<u8>
}

impl WRAM {
    pub fn new() -> Self {
        Self {
            ram: vec![0x2000, 0x00]
        }
    }

    pub fn read(&self, adress: u16) -> u8 {
        self.ram[adress - 0xC000]
    }

    pub fn write(
        &mut self,
        adress: u16,
        value: u8
    ) {
        self.ram[adress - 0xC000] = value;
    }
}
