pub struct HRAM {
    ram: Vec<u8>
}

impl HRAM {
    pub fn new() -> Self {
        Self {
            ram: vec![0x7F, 0x00]
        }
    }

    pub fn read(&self, adress: u16) -> u8 {
        ram[adress - 0xFF80]
    }

    pub fn write(
        &mut self,
        adress: u16,
        value: u8
    ) {
        ram[adress - 0xFF80] = value;
    }
}
