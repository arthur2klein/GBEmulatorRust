use crate::components::screen::Screen;
use crate::state::key_state::KeyState;
use crate::state::tile_object::TileObject;

/// Represents the GPU or PPU of the GameBoy
pub struct Gpu {
    /// VRAM of the GPU
    ram: Vec<u8>,
    /// OAM of the GPU
    /// Contains informations about the objects drawn on screen
    object_attribute: Vec<TileObject>,
    /// Contains information about the display
    /// is enabled/window tile map/window enabled/background tiles/
    /// background tile map/object size/object enabled/background enabled
    lcd_control: u8,
    /// Contains information about the display
    /// unused/interruption if lyc==ly/interruption if mode2/
    /// interruption if mode1/interruption if mode 0/lyc==ly/gpu mode/gpu mode
    lcd_status: u8,
    /// y coordinate of the top left corner of the background
    background_viewport_y: u8,
    /// x coordinate of the top left corner of the background
    background_viewport_x: u8,
    /// y coordinate of the line being drawn
    lcd_y_coordinate: u8,
    /// y coordinate of the top left corner of the window
    window_y_position: u8,
    /// x coordinate of the top left corner of the window plus sept
    window_x_position_plus_sept: u8,
    /// Value compared with lcd_y_coordinate to allow interruption
    lyc_compare: u8,
    /// gray shades (2 bit each) corresponding to the color ids for the
    /// background
    bg_palette_data: u8,
    /// gray shades (2 bit each) corresponding to the color ids for object
    /// using this palette
    obp0: u8,
    /// gray shades (2 bit each) corresponding to the color ids for object
    /// using this palette
    obp1: u8,
    /// Is a stat interrpution waiting to be handled by the cpu?
    pub pending_stat_interrupt: bool,
    /// Is a vblank interrpution waiting to be handled by the cpu?
    pub pending_vblank_interrupt: bool,
    /// Screen used to display the information of the gpu
    screen: Screen,
    /// Current cpu cycle to know when to refresh the screen
    cpu_cycle: u16,
}

impl Gpu {
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
    
    /// Returns the beginning of the region of the memory that links the value
    /// of the tile map with the addresses of the bytes
    ///
    /// # Returns
    /// Beginning of the region of the memory that links the value of the tile
    /// map with the addresses of the bytes
    fn bg_and_window_tile_data_area(&self) -> u16 {
        if self.lcd_control & 0x10 == 0x10 {
            0x8000
        } else {
            0x9000
        }
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

    /// Updates the screen and search for key presses
    ///
    /// # Arguments
    /// **n_cycles (u16)**: Number of cpu cycles since last update
    ///
    /// # Returns
    /// **bool**: true iff the Escape key was pressed
    pub fn update(&mut self, n_cycles: u16) -> bool {
        if ((self.cpu_cycle & 0x3FFF) + n_cycles) >= 0x4000 {
            self.draw_lines();
        }
        self.cpu_cycle = self.cpu_cycle.wrapping_add(n_cycles);
        self.screen.update_key_press()
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
        let high_byte = self.ram[
            (tile_address + y_in_tile as u16 * 2 + 1) as usize
        ];
        let low_byte = self.ram[
            (tile_address + y_in_tile as u16 * 2) as usize
        ];
        
        (((high_byte >> (7 - x_in_tile)) & 0x01) << 1) |
        ((low_byte >> (7 - x_in_tile)) & 0x01)
        
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
        // Position of the tile when reading line by line from left to right
        let tile_in_map =
            (x_in_map >> 3) as usize +
            (y_in_map >> 3) as usize * 32;
        // This position is the index in the background tile map which is a
        // list of byte identifying each tile of the background.
        let tile_index = self.ram[
            self.background_tile_map() as usize +
            tile_in_map
        ]; 
        // The id found above correspond to one of the tile of the background
        // and window tile data (each tile is 16 bytes)
        let tile_address = if tile_index < 128 {
            self.bg_and_window_tile_data_area() + ((tile_index as u16) << 4)
        } else {
            0x8800 + ((tile_index as u16) << 4)
        };
        // The color is is then found for this tile for the correct pixel. Each
        // tile is 8x8 pixels.
        let color_id = self.color_id_in_tile(
           tile_address,
           y_in_map & 0x07,
           x_in_map & 0x07
        );
        // This color id is a color of the palette of the background
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
        let (y_in_map, did_overflow_y) =
            self.window_y_position.overflowing_add(y);
        let (x_in_map_plus_sept, did_overflow_x) =
            self.window_x_position_plus_sept.overflowing_add(x);
        // If the window is outside of the screen, 4 is returned
        if x_in_map_plus_sept < 7 ||
           did_overflow_y ||
           did_overflow_x ||
           x_in_map_plus_sept > 167 ||
           y_in_map > 144 {
            return 4;
        }
        let x_in_map = x_in_map_plus_sept - 7;
        // Position of the tile when reading line by line from left to right
        let tile_in_map =
            (x_in_map >> 3) as usize +
            (y_in_map >> 3) as usize * 32;
        // This position is the index in the window tile map which is a
        // list of byte identifying each tile of the window.
        let tile_index = self.ram[
            self.window_tile_map() as usize +
            tile_in_map
        ]; 
        // The id found above correspond to one of the tile of the background
        // and window tile data (each tile is 16 bytes)
        let tile_address = if tile_index < 128 {
            self.bg_and_window_tile_data_area() + ((tile_index as u16) << 4)
        } else {
            0x8800 + ((tile_index as u16) << 4)
        };
        // The color is is then found for this tile for the correct pixel. Each
        // tile is 8x8 pixels.
        let color_id = self.color_id_in_tile(
           tile_address,
           y_in_map & 0x07,
           x_in_map & 0x07
        );
        // This color id is a color of the palette of the background
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
    /// **obj_in_line (`&Vec<u32>`)**: Indices of the objects in this line
    ///
    /// # Retuns
    /// **u8**: Color of the given pixel
    fn draw_pixel(
        &mut self,
        x: u8,
        y: u8,
        obj_in_line: &[u32]
    ) -> u8 {
        // Color of the background for this pixel
        let color_from_background = self.color_background(x, y);
        // Color of the window for this pixel
        let color_from_window = self.color_window(x, y);
        // Color of the objects for this pixel
        let mut has_priority: bool = false;
        let mut x_position: u8 = 0xFF;
        let mut color_from_obj: u8 = 0;
        let mut is_transparent: bool = true;
        // Comparison of the objects
        for i in obj_in_line.iter() {
            let object = &self.object_attribute[
                obj_in_line[*i as usize] as usize
            ];
            // If the object does not contain this pixel
            if !(
                object.x_position <= x &&
                object.x_position + 8 > x
            ) {
                continue;
            }
            // The tile_index is the index in the object tile data where each
            // tile is 16 bytes
            let tile_for_obj = 0x8000 + ((object.tile_index as u16) << 4);
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
            // If the pixel is pixel for this object
            if color_id == 0 {
                continue;
            }
            // If not, we are certain to have found a non-transparent pixel
            is_transparent = false;
            let current_has_priority = object.get_priority();
            // If we have already found an object with higher priority
            if has_priority && !current_has_priority {
                continue;
            }
            // Object with a smaller x position have a higher priority
            if x_position < object.x_position {
                continue;
            }
            // If the loop iteration reach this point, the object is on top
            has_priority = current_has_priority;
            x_position = object.x_position;
            color_from_obj = (if object.get_dmg_palette() {
                self.obp1
            } else {
                self.obp0
            } >> (2 * color_id)) & 0x3;
        }
        // We apply the rules to know what is on front
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
    /// **`Vec<u32>`**: Collections of the indices of objects found in the
    /// current line
    fn objects_in_line(&self, y: u8) -> Vec<u32> {
        let mut res: Vec<u32> = vec![];
        let obj_size = self.obj_size();
        for i in 0..40 {
            let y_position = self
                .object_attribute[i]
                .y_position
                .wrapping_sub(16);
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
