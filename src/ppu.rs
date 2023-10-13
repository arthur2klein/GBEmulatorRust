struct Tile_object {
    y_position: u8,
    x_positino: u8,
    tile_index: u8,
    flags: u8
}

impl Tile_object {
    fn new() -> Self {
        Self {
            0,
            0,
            0,
            0
        }
    }
    
    fn get_priority(&self) -> bool {
        self.flags & 0x80 == 0x80
    }

    fn get_y_flip(&self) -> bool {
        self.flags & 0x40 == 0x40
    }

    fn get_x_flip(&self) -> bool {
        self.flags & 0x20 == 0x20
    }

    fn get_dmg_palette(&self) -> bool {
        self.flags & 0x10 == 0x10
    }

    fn get_bank(&self) -> bool {
        self.flags & 0x08 == 0x08
    }

    fn get_cgb_palette(&self) -> u8 {
        self.flags & 0x07
    }
}

pub class GPU {
    ram: Vec<u8>,
    object_attribute: Vec<Tile_object>
}

impl GPU {
    pub fn new() -> Self {
        Self {
            ram: vec![0; 0x2000],
            object_attribues: vec![40; Tile_object::new()]
        }
    }

    pub fn read_ram(&self, adress: u16) -> u8 {
        self.ram[adress - 0x8000]
    }
    
    pub fn write_ram(
        &mut self,
        adress: u16,
        value: u8
    ) {
        self.ram[adress - 0x8000] = value;
    }

    pub fn read_oam(&self, adress: u16) -> u8 {
       let entry = (adress - 0xFE00) / 4;
       let byte = (adress - 0xFE00) mod 4;
       match byte {
           0 => {
               object_attribute[entry].y_position
           },
           1 => {
               object_attribute[entry].x_position
           },
           2 => {
               object_attribute[entry].tile_index
           },
           3 => {
               object_attribute[entry].flags
           }
       }
    }

    pub fn write_oam(
        &mut self,
        adress: u16,
        value: u8
    ) {
       let entry = (adress - 0xFE00) / 4;
       let byte = (adress - 0xFE00) mod 4;
       match byte {
           0 => {
               object_attribute[entry].y_position = value;
           },
           1 => {
               object_attribute[entry].x_position = value;
           },
           2 => {
               object_attribute[entry].tile_index = value;
           },
           3 => {
               object_attribute[entry].flags = value;
           }
       }
    }
}
