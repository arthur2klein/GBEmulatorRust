struct Tile_object {
    y_position: u8,
    x_position: u8,
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
    object_attribute: Vec<Tile_object>,
    lcd_control: u8,
    lcd_status: u8,
    background_viewport_y: u8,
    background_viewport_x: u8,
    lcd_y_coordinate: u8,
    windoy_y_position: u8,
    window_x_position_plus_sept: u8,
    lyc_compare: u8,
    /// gray shades (2 bit each) corresponding to the color ids
    bg_palette_data: u8,
    obp0: u8,
    obp1: u8,
}

impl GPU {
    pub fn new() -> Self {
        Self {
            ram: vec![0; 0x2000],
            object_attribues: vec![40; Tile_object::new()]
        }
    }

    pub fn read_lcd(&self, address: u16) -> u8 {
        match address {
            // LCD
            0x40 => {
                self.lcd_control
            },
            0x41 => {
                self.lcd_status
            },
            0x42 => {
                self.background_viewport_y
            },
            0x43 => {
                self.background_viewport_x
            },
            0x44 => {
                self.lcd_y_coordinate
            },
            0x4A => {
                self.windoy_y_position
            },
            0x4B => {
                self.window_x_position_plus_sept
            },
            0x45 => {
                self.lyc_compare
            },
            // Palettes
            0x47 => {
                self.bg_palette_data
            },
            0x48 => {
                self.obp0
            },
            0x49 => {
                self.obp1
            },
        }
    }

    pub fn write_lcd(&self, address: u16, value: u8) {
        match address {
            // LCD
            0x40 => {
                self.lcd_control = value;
            },
            0x41 => {
                self.lcd_status = value;
            },
            0x42 => {
                self.background_viewport_y = value;
            },
            0x43 => {
                self.background_viewport_x = value;
            },
            0x44 => {
                self.lcd_y_coordinate = value;
            },
            0x4A => {
                self.windoy_y_position = value;
            },
            0x4B => {
                self.window_x_position_plus_sept = value;
            },
            0x45 => {
                self.lyc_compare = value;
            },
            // Palettes
            0x47 => {
                self.bg_palette_data = value;
            },
            0x48 => {
                self.obp0 = value;
            },
            0x49 => {
                self.obp1 = value;
            },
        }
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        self.ram[address - 0x8000]
    }
    
    pub fn write_ram(
        &mut self,
        address: u16,
        value: u8
    ) {
        self.ram[address - 0x8000] = value;
    }

    pub fn read_oam(&self, address: u16) -> u8 {
       let entry = (address & 0x00FF) >> 2;
       let byte = (address & 0x00FF) & 0x0003;
       match byte {
           0 => {
               self.object_attribute[entry].y_position
           },
           1 => {
               self.object_attribute[entry].x_position
           },
           2 => {
               self.object_attribute[entry].tile_index
           },
           3 => {
               self.object_attribute[entry].flags
           }
       }
    }

    fn draw(&mut self) {
        self.draw_background();
        self.draw_objects();
    }

    fn draw_background(&mut self) {
        // TODO
    }
    
    fn tile_from_index(&self, index: u8) -> Tile {
        if index > 127 {
            self.ram[0x0800 | (index & 0x7F as u16)]
        } else {
            if self.lcd_control & 0x10 {
                self.ram[0x1000 | (index & 0x7F as u16)]
            } else {
                self.ram[0x0000 | (index & 0x7F as u16)]
            }
        }
    }

    fn draw_object(&mut self, index: u8) {
        let y_position = self.object_attribute[index].y_position - 16;
        let x_position = self.object_attribute[index].x_position - 8;
        let tile_index = self.object_attribute[index].tile_index;
        let priority = self.object_attribute[index].get_priority();
        let y_flip = self.object_attribute[index].get_y_flip();
        let x_flip = self.object_attribute[index].get_x_flip();
        let dmg_palette = self.object_attribute[index].get_dmg_palette();
        let tile_address = self.tile_from_index(tile_index);
        let lines = ram[tile_address..tile_address + 16];
        for i in 0..8 {
            for j in 0..8 {
                let color_id = lines[2 * line + 1] * 2 + lines[2 * line];
                let color = if dmg_palette {
                    (self.obp1 >> (2 * color_id)) & 0x3
                } else {
                    (self.obp0 >> (2 * color_id)) & 0x3
                }
                if color != 0 {
                    self.draw_pixel(
                        y_position + if y_flip {i} else {8 - i},
                        x_position + if x_flip {j} else {8 - j},
                        color,
                        priority,
                    );
                }

            }
        }
    }

    fn draw_pixel(
        y_position,
        x_position,
        color,
        priority,
    ) {
        // TODO
    }

    fn draw_objects(&mut self) {
        for i in 0..40 {
            self.draw_object(i);
        }
    }

    pub fn write_oam(
        &mut self,
        address: u16,
        value: u8
    ) {
       let entry = (address & 0x00FF) >> 2;
       let byte = (address & 0x00FF) & 0x0003;
       match byte {
           0 => {
               self.object_attribute[entry].y_position = value;
           },
           1 => {
               self.object_attribute[entry].x_position = value;
           },
           2 => {
               self.object_attribute[entry].tile_index = value;
           },
           3 => {
               self.object_attribute[entry].flags = value;
           }
       }
    }

}
