use crate::screen::Screen;
use crate::screen::KeyState;

#[derive(Clone)]
struct TileObject {
    y_position: u8,
    x_position: u8,
    tile_index: u8,
    flags: u8
}

impl TileObject {
    fn new() -> Self {
        Self {
            y_position: 0x00,
            x_position: 0x00,
            tile_index: 0x00,
            flags: 0x00,
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
}

pub struct GPU {
    ram: Vec<u8>,
    object_attribute: Vec<TileObject>,
    lcd_control: u8,
    lcd_status: u8,
    background_viewport_y: u8,
    background_viewport_x: u8,
    lcd_y_coordinate: u8,
    window_y_position: u8,
    window_x_position_plus_sept: u8,
    lyc_compare: u8,
    /// gray shades (2 bit each) corresponding to the color ids
    bg_palette_data: u8,
    obp0: u8,
    obp1: u8,
    pub pending_stat_interrupt: bool,
    pub pending_vblank_interrupt: bool,
    screen: Screen,
    cpu_cycle: u16,
}

impl GPU {
    /// Create a new GPU
    ///
    /// # Returns
    /// **GPU**: New GPU
    pub fn new() -> Self {
        Self {
            ram: vec![0; 0x2000],
            object_attribute: vec![TileObject::new(); 40],
            lcd_control: 0,
            lcd_status: 0,
            background_viewport_y: 0,
            background_viewport_x: 0,
            lcd_y_coordinate: 0,
            window_y_position: 0,
            window_x_position_plus_sept: 0,
            lyc_compare: 0,
            bg_palette_data: 0,
            obp0: 0,
            obp1: 0,
            pending_stat_interrupt: false,
            pending_vblank_interrupt: false,
            screen: Screen::new(),
            cpu_cycle: 0,
        }
    }

    /// Transmit the Key State
    ///
    /// Returns informations about what key is down
    ///
    /// # Returns
    /// **KeyState**: Are key pressed for the screen?
    pub fn transmit_key(&self) -> &KeyState {
        &self.screen.key_state
    }

    /// Read a value in the given address of the LCD memory are
    ///
    /// # Arguments
    /// **address (u16)**: Address to read
    ///
    /// # Returns
    /// **u8**: Value read at this address
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
                self.window_y_position
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
            _ => {
                panic!("Wrong address in lcd");
            }
        }
    }

    /// Write the given value in the given address of the LCD memory area
    ///
    /// # Arguments
    /// **address (u16)**: Address to write to
    /// **value (u8)**: Value to write at this address
    pub fn write_lcd(&mut self, address: u16, value: u8) {
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
                self.window_y_position = value;
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
            _ => {
                panic!("Wrong address in lcd");
            }
        }
    }

    /// Read a value in the given address of the VRAM
    ///
    /// # Arguments
    /// **address (u16)**: Address to read
    ///
    /// # Returns
    /// **u8**: Value read at this address
    pub fn read_ram(&self, address: u16) -> u8 {
        self.ram[(address - 0x8000) as usize]
    }
    
    /// Write the given value in the given address of the VRAM
    ///
    /// # Arguments
    /// **address (u16)**: Address to write to
    /// **value (u8)**: Value to write at this address
    pub fn write_ram(
        &mut self,
        address: u16,
        value: u8
    ) {
        self.ram[(address - 0x8000) as usize] = value;
    }

    /// Read a value in the given address of the OAM
    ///
    /// # Arguments
    /// **address (u16)**: Address to read
    ///
    /// # Returns
    /// **u8**: Value read at this address
    pub fn read_oam(&self, address: u16) -> u8 {
       let entry = (address & 0x00FF) >> 2;
       let byte = (address & 0x00FF) & 0x0003;
       match byte {
           0 => {
               self.object_attribute[entry as usize].y_position
           },
           1 => {
               self.object_attribute[entry as usize].x_position
           },
           2 => {
               self.object_attribute[entry as usize].tile_index
           },
           3 => {
               self.object_attribute[entry as usize].flags
           },
           _ => {
               panic!("Wrong attribute in oam");
           }
       }
    }

    /// Write the given value in the given address of the OAM
    ///
    /// # Arguments
    /// **address (u16)**: Address to write to
    /// **value (u8)**: Value to write at this address
    pub fn write_oam(
        &mut self,
        address: u16,
        value: u8
    ) {
       let entry = (address & 0x00FF) >> 2;
       let byte = (address & 0x00FF) & 0x0003;
       match byte {
           0 => {
               self.object_attribute[entry as usize].y_position = value;
           },
           1 => {
               self.object_attribute[entry as usize].x_position = value;
           },
           2 => {
               self.object_attribute[entry as usize].tile_index = value;
           },
           3 => {
               self.object_attribute[entry as usize].flags = value;
           },
           _ => {
               panic!("Wrong attribute in oam");
           }
       }
    }

    /// Returns true iff the screen should be displayed
    ///
    /// # Returns
    /// **bool**: True iff the screen should be displayed
    fn is_enabled(&self) -> bool {
        self.lcd_control & 0x80 == 0x80
    }

    /// Returns the window tile map beginning address
    ///
    /// # Retuns
    /// **u16**: Beginning of the window tile map area
    fn window_tile_map(&self) -> u16 {
        if self.lcd_control & 0x40 == 0x40 {
            0x9800
        } else {
            0x9C00
        }
    }

    /// Returns true iff the window should be drawn on the screen
    ///
    /// # Returns 
    /// **bool**: True iff the window should be drawn on the screen
    fn should_draw_window(&self) -> bool {
        self.lcd_control & 0x20 == 0x20
    }

    /// Returns the beginning address of the tile map area
    ///
    /// # Returns
    /// **u16**: Beginning address of the tile map area
    fn background_tile_map(&self) -> u16 {
        if self.lcd_control & 0x08 == 0x08 {
            0x9800
        } else {
            0x9C00
        }
    }

    /// Returns the height of the objects
    ///
    /// # Returns
    /// **u8**: Height of the objects (16 or 8)
    fn obj_size(&self) -> u8 {
        if self.lcd_control & 0x04 == 0x04 {
            16
        } else {
            8
        }
    }

    /// Returns true if the objects should be drawn on the screen
    ///
    /// # Retuns 
    /// **bool**: True if the objects should be drans on the screen
    fn should_draw_objects(&self) -> bool {
        self.lcd_control & 0x02 == 0x02
    }

    /// Returns true if the window and background should be drawn on the screen
    ///
    /// # Retuns
    /// **bool**: True if the window and background should be drawn on the
    /// screen
    fn should_draw_window_and_background(&self) -> bool {
        self.lcd_control & 0x01 == 0x01
    }

    /// Sends a VBlank interruption
    ///
    /// Specify that a VBlank interruption is pending for the MMU to indicate it
    /// This means that the PPU is wainting for the next frame
    fn send_vblank_interrupt(&mut self) {
        self.pending_vblank_interrupt = true;
    }

    /// Sends a STAT interruption
    ///
    /// Specify that a STAT interruption is pending for the MMU to indicate it
    fn send_stat_interrupt(&mut self) {
        self.pending_stat_interrupt = true;
    }

    /// Checks if lyc == ly
    ///
    /// The gameboy compare constantly the values of the addresses of LCY Y
    /// Compare and LCD Y coordinate, and sends an interruption when they are
    /// equal
    fn lyc_equal_ly(&mut self) {
        self.lcd_status |= 0x040;
        if self.lcd_status & 0x40 == 0x40 {
            self.send_stat_interrupt();
        }
    }

    /// Switches the PPU mode
    ///
    /// If the new mode is different for the previous one, change the mode
    /// indicated in LCD status, and if the interruptions are activated for
    /// this mode, send one.
    ///
    /// # Arguments
    /// **mode (u8)**: New PPU mode
    fn switch_mode_to(&mut self, mode: u8) {
        let old_mode = self.lcd_status & 0x03;
        if mode == old_mode {
            return;
        }
        self.lcd_status = (self.lcd_status & 0xFC) | (mode & 0x03);
        let mask = 1 << (mode + 3);
        let interruption_for_this_mode = self.lcd_status & mask == mask;
        if interruption_for_this_mode {
            self.send_stat_interrupt();
        }
    }

    pub fn update(&mut self, n_cycles: u16) {
        if (self.cpu_cycle & 0x3FFF + n_cycles) >= 0x4000 {
            self.draw_lines();
        }
        self.screen.update_key_press();
        self.cpu_cycle = self.cpu_cycle.wrapping_add(n_cycles);
    }

    /// Draws one frame
    ///
    /// One frame lasts 16.74 ms
    fn draw_lines(&mut self) {
        if !self.is_enabled() {
            return;
        }
        self.lcd_y_coordinate = 0;
        while self.lcd_y_coordinate < 154 {
            //let time = SystemTime::now();
            if self.lcd_y_coordinate == self.lyc_compare {
                self.lyc_equal_ly();
            }
            self.draw_line();
            self.lcd_y_coordinate += 1;
            //sleep(Duration::from_micros(16740) - time.elapsed.unwrap());
        }
        self.screen.update();
    }

    /// Draws a line on the screen
    ///
    /// Drawn the line which as y = lcd_y_coordinate
    fn draw_line(&mut self) {
        // 4 dots per CPU cycle (4.194 MHz)
        let ly = self.lcd_y_coordinate;
        if ly == 144 {
            self.send_vblank_interrupt();
        }
        if ly > 143 {
            self.switch_mode_to(1);
            // Mode 1
            // Vertical Black
            // Waiting until the next frame
            // 456 dots
            return;
        }
        self.switch_mode_to(2);
        // Mode 2
        // OAM Scan
        // Searching for OBJs which overlap this line
        // 80 dots
        let obj_in_line = self.objects_in_line(ly);
        self.switch_mode_to(3);
        // Mode 3
        // Drawing pixels
        // Sending pixels to the LCD
        // 172 dots (160 pixels wide)
        for x in 0..159 {
            let pixel = self.draw_pixel(x, ly, &obj_in_line);
            self.screen.receive_pixel(
                x,
                ly,
                pixel
            );
        }
        self.switch_mode_to(0);
        // Mode 0
        // Horizontal blank
        // Waiting for the end of the scanline
        // 204 dots
    }

    /// Returns the color id of a pixel in a tile
    ///
    /// # Arguments
    /// **tile_address (u16)**: Address of the tile
    /// **x_in_tile (u8)**: column in the tile
    /// **y_in_tile (u8)**: line in the tile
    ///
    /// # Returns
    /// **u8**: Color id of the pixel
    ///
    /// # Returns
    /// Color id of the given pixel in the given tile
    fn color_id_in_tile(
        &self,
        tile_address: u16,
        y_in_tile: u8,
        x_in_tile: u8,
    ) -> u8{
        let line_tile = tile_address + (y_in_tile * 16) as u16;
        (
            ((line_tile & (1 << (x_in_tile + 8))) >> (x_in_tile + 8)) +
            ((line_tile & (1 << (x_in_tile))) >> (x_in_tile + 7)) 
        ) as u8
    }

    /// Returns the color of a pixel of the background
    ///
    /// # Arguments
    /// **x (u8)**: x coordinate of the pixel on the screen
    /// **y (u8)**: y coordinate of the pixel on the screen
    ///
    /// # Returns
    /// **u8**: Color of the given pixel from the background
    fn color_background(&self, x: u8, y: u8) -> u8 {
        let y_in_map = self.background_viewport_y + y;
        let x_in_map = self.background_viewport_x + x;
        let tile_index: u16 = x_in_map as u16 / 8 + (y_in_map as u16 / 8) * 256;
        let tile_address = self.background_tile_map() + (tile_index * 2) as u16; 
        let x_in_tile = x_in_map % 8;
        let y_in_tile = y_in_map % 8;
        let color_id = self.color_id_in_tile(
           tile_address,
           y_in_tile,
           x_in_tile
        );
        (self.bg_palette_data >> (color_id * 2)) & 0x03
    }

    /// Returns the color of a pixel of the window
    ///
    /// # Arguments
    /// **x (u8)**: x coordinate of the pixel on the screen
    /// **y (u8)**: y coordinate of the pixel on the screen
    ///
    /// # Returns
    /// **u8**: Color of the given pixel from the window or 4 if the pixel is
    /// out of the window
    fn color_window(&self, x: u8, y: u8) -> u8 {
        let y_in_map = self.window_y_position + y;
        let x_in_map = self.window_x_position_plus_sept + x;
        if (y_in_map >= 143) || (x_in_map >= 166) {
            return 4;
        }
        let tile_index = x_in_map as u16 / 8 + (y_in_map as u16 / 8) * 256;
        let tile_address = self.window_tile_map() + (tile_index * 2); 
        let x_in_tile = x_in_map % 8;
        let y_in_tile = y_in_map % 8;
        let color_id = self.color_id_in_tile(
           tile_address,
           y_in_tile,
           x_in_tile
        );
        (self.bg_palette_data >> (color_id * 2)) & 0x03
    }

    /// Returns the color of the pixel on the screen
    ///
    /// Checks whether an object, the window or the background should be
    /// displayed at this pixel and sends it to the lcd
    ///
    /// # Arguments
    /// **x (u8)**: X coordinate of the pixel
    /// **y (u8)**: Y coordinate of the pixel
    /// **obj_in_line (Vec<u32>)**: Indices of the objects in this line
    ///
    /// # Retuns
    /// **u8**: Color of the given pixel
    fn draw_pixel(
        &mut self,
        x: u8,
        y: u8,
        obj_in_line: &Vec<u32>
    ) -> u8 {
        let color_from_background = self.color_background(x, y);
        let color_from_window = self.color_window(x, y);
        let mut has_priority: bool = false;
        let mut x_position: u8 = 0xFF;
        let mut color_from_obj: u8 = 0;
        let mut is_transparent: bool = true;
        for i in obj_in_line.iter() {
            let object = &self.object_attribute[
                obj_in_line[*i as usize] as usize
            ];
            if !(
                object.x_position <= x &&
                object.x_position + 8 > x
            ) {
                continue;
            }
            let tile_for_obj = 0x8000 + (16 * object.tile_index) as u16;
            let color_id = self.color_id_in_tile(
                tile_for_obj,
                if object.get_y_flip() {
                    (y - object.y_position) % 8
                } else {
                    (15 - (y + object.y_position)) % 8
                },
                if object.get_x_flip() {
                    x - object.x_position
                } else {
                    7 - (x + object.x_position)
                },
            );
            if color_id == 0 {
                continue;
            }
            is_transparent = false;
            let current_has_priority = object.get_priority();
            if has_priority && !current_has_priority {
                continue;
            }
            if x_position < object.x_position {
                continue;
            }
            has_priority = current_has_priority;
            x_position = object.x_position;
            color_from_obj = (if object.get_dmg_palette() {
                self.obp1
            } else {
                self.obp0
            } >> (2 * color_id)) & 0x3;
        }
        if !is_transparent && self.should_draw_objects() {
            color_from_obj
        } else if self.should_draw_window_and_background() {
            if self.should_draw_window() && color_from_window != 4 {
                color_from_window
            } else {
                color_from_background
            }
        } else {
            0x00
        }
    }

    /// Mode 2 of drawing a line
    ///
    /// During 80 dots, the ppu search up to 10 valid objects intersecting the
    /// current y coordinate
    ///
    /// # Arguments
    /// **y (u8)**: Current y coordinate (as found at 0xFF40)
    ///
    /// # Returns
    /// **Vec<u32>**: Collections of the indices of objects found in the
    /// current line
    fn objects_in_line(&self, y: u8) -> Vec<u32> {
        let mut res: Vec<u32> = vec![];
        let obj_size = self.obj_size();
        for i in 0..40 {
            let y_position = self.object_attribute[i].y_position - 16;
            if y_position <= y && y_position + obj_size > y {
                res.push(i as u32);
                if res.len() == 10 {
                    return res;
                }
            }
        }
        res
    }
}
