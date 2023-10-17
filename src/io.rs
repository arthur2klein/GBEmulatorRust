// The SsBA buttons are select button and are read from bits 3 to 0 when bit 5
// is 0.
// The movement buttons read from bits 3 to 0 when bit 4 is 0.
pub enum Joypad_button {
    // SsBA buttons
    START,
    SELECT,
    B,
    A,
    // Movement buttons
    DOWN,
    UP,
    LEFT,
    RIGHT,
}

pub struct IO {
    // Joypad
    joypad_input: u8,
    joypad_input_ssba: u8,
    joypad_input_movement: u8,
    joypad_pending_interruption: bool
    // Serial transfer
    serial_transfer: u16,
    // Timer and divider
    divider: u8,
    timer_counter: u8,
    timer_modulo: u8,
    timer_control: u8,
    // LCD Control
    lcd_contrl: u8,
    // LCD status registers
    lcd_status: u8,
    lcd_y_coordinate: u8,
    lyc_compare: u8,
    // LCD Position and scrolling
    background_viewport_y: u8,
    background_viewport_x: u8,
    windoy_y_position: u8,
    window_x_position_plus_sept: u8,
    // Palettes
    bg_palette_data: u8,
    obp0: u8,
    obp1: u8,
    // Set to non zero to diasable boot ROM
    disable_boot_rom: u8,
}

impl IO {
    pub fn new() -> Self {
        Self {
        }
    }
    
    fn get_joypad_input(&self) -> u8 {
        self.read(0);
    }

    fn get_serial_transfer(&self) -> u16 {
        self.read(1) as u16 << 8 | self.read(2)
    }

    fn get_divider(&self) -> u8 {
        
    }

    pub fn read(&self, address: u16) -> u8 {
        match (address & 0x00FF) as u8 {
            // Joypad
            0x00 => {
                self.joypad_input
            },
            // Serial transfer
            0x01 => {
                ((self.serial_transfer & 0xFF00) >> 8) as u8
            },
            0x02 => {
                (self.serial_transfer & 0x00FF) as u8
            },
            // Timer and divider
            0x04 => {
                self.divider
            },
            0x05 => {
                self.timer_counter
            },
            0x06 => {
                self.timer_modulo
            },
            0x07 => {
                self.timer_control
            },
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
            // Set to non zero to diasable boot ROM
            0x50 => {
                self.disable_boot_rom
            },
    }

    pub fn write(
        &mut self,
        address: u16,
        value: u8
    ) {
        match (address & 0x00FF) as u8 {
            // Joypad
            0x00 => {
                self.joypad_input = value;
            },
            // Serial transfer
            0x01 => {
                self.serial_transfer = (
                    (value as u16) << 8 |
                    self.serial_transfer 0x00FF
                );
            },
            0x02 => {
                self.serial_transfer = (
                    self.serial_transfer 0xFF00 |
                    value as u16
                );
            },
            // Timer and divider
            0x04 => {
                self.divider
            },
            0x05 => {
                self.timer_counter
            },
            0x06 => {
                self.timer_modulo
            },
            0x07 => {
                self.timer_control
            },
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
            // Set to non zero to diasable boot ROM
            0x50 => {
                self.disable_boot_rom
            },
    }

    pub fn press_button(&mut self, button: Joypad_button) {
        // Was a button already being pushed
        let was_pushed = self.joypat_input & 0x0F == 0x0F;
        match button {
            Joypad_button::START => {
                self.joypad_input_ssba &= 0xF7;
            },
            Joypad_button::SELECT => {
                self.joypad_input_ssba &= 0xFB;
            },
            Joypad_button::B => {
                self.joypad_input_ssba &= 0xFD;
            },
            Joypad_button::A => {
                self.joypad_input_ssba &= 0xFE;
            },
            Joypad_button::DOWN => {
                self.joypad_input_movement &= 0xF7;
            },
            Joypad_button::UP => {
                self.joypad_input_movement &= 0xFB;
            },
            Joypad_button::LEFT => {
                self.joypad_input_movement &= 0xFD;
            },
            Joypad_button::RIGHT => {
                self.joypad_input_movement &= 0xFE;
            }
        }
        self.joypad_input |= 0x0F;
        // If the movements keys are used
        if self.joypad_input & 0x10 == 0x00 {
            self.joypad_input &= self.joypad_input_movement;
        }
        // If the SSBA keys are being used
        if self.joypad_input & 0x20 == 0x00 {
            self.joypad_input &= self.joypad_input_ssba;
        }
        if !was_pushed {
            self.send_joypad_interrupt();
        }
    }

    fn send_joypad_interrupt(&mut self) {
        self.pendingInterruption = true;
    }

    pub fn release_button(&mut self, button: Button) {
        match button {
            Joypad_button::START => {
                self.joypad_input_ssba |= 0x08;
            },
            Joypad_button::SELECT => {
                self.joypad_input_ssba |= 0x04;
            },
            Joypad_button::B => {
                self.joypad_input_ssba |= 0x02;
            },
            Joypad_button::A => {
                self.joypad_input_ssba |= 0x01;
            },
            Joypad_button::DOWN => {
                self.joypad_input_movement |= 0x08;
            },
            Joypad_button::UP => {
                self.joypad_input_movement |= 0x04;
            },
            Joypad_button::LEFT => {
                self.joypad_input_movement |= 0x02;
            },
            Joypad_button::RIGHT => {
                self.joypad_input_movement |= 0x01;
            }
        }
        self.joypad_input |= 0x0F;
        // If the movements keys are used
        if self.joypad_input & 0x10 == 0x00 {
            self.joypad_input &= self.joypad_input_movement;
        }
        // If the SSBA keys are being used
        if self.joypad_input & 0x20 == 0x00 {
            self.joypad_input &= self.joypad_input_ssba;
        }
    }

    pub fn update(
        &mut self,
        n_ticks: u32
    ) {
    }
}
