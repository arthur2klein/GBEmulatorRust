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
    // Serial transfer (should not be used)
    serial_transfer: u16,
    // Timer and divider
    divider: u8,
    cpu_cycle: u16,
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
    // Interruptions
    pub pending_joypad_interruption: bool,
    pub pending_timer_interruption: bool,
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
            // Serial transfer (should not be used)
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
                // The lower nible is read-only
                self.joypad_input = (
                    (self.joypad_input & 0x0F) |
                    (value & 0xF0)
                );
            },
            // Serial transfer (should not be used)
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
            // Writing any value to it will set it to 0.
            0x04 => {
                self.divider = 0x00;
            },
            0x05 => {
                self.timer_counter = value;
            },
            0x06 => {
                self.timer_modulo = value;
            },
            0x07 => {
                self.timer_control = value;
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
        // The clock frequency of the CPU is 4194304 Hz
        // The divider increment frequency is  16384 Hz (every 256 cycle)
        let increment_divider = (
            ((self.cpu_cycle & 0x00FF).wrapping_add(n_ticks)) & 0xFF00
        ) >> 8;
        self.divider.wrapping_add(increment_divider);
        // The timer is incremented at the clock frequency specified by the TAC
        // register (0xFF07)
        if self.timer_control & 0x20 == 0x20 {
            let increment_timer = match self.timer_control & 0x03 {
                // Frequency: 4096 Hz (1024 cycles)
                0 => {
                    ((
                        (self.cpu_cycle & 0x03FF).wrapping_add(n_ticks)
                    ) & 0xFC00) >> 10
                },
                // Frequency: 262144 Hz (16 cycles)
                1 => {
                    ((
                        (self.cpu_cycle & 0x000F).wrapping_add(n_ticks)
                    ) & 0xFFF0) >> 4
                },
                // Frequency: 65536 Hz (64 cycles)
                2 => {
                    ((
                        (self.cpu_cycle & 0x003F).wrapping_add(n_ticks)
                    ) & 0xFFC0) >> 6
                },
                // Frequency: 16384 Hz (256 cycles)
                3 => {
                    ((
                        (self.cpu_cycle & 0x00FF).wrapping_add(n_ticks)
                    ) & 0xFF00) >> 8
                },
            }
            let did_overflow, self.timer_counter = (
                self.timer_counter.overflowing_add(increment_timer)
            );
            // When the value exceeds 0xFF, it is reet to the value specified in
            // TMA (0xFF06) and an interrupt is requested.
            if did_overflow {
                self.timer_counter = self.timer_counter.wrapping_add(
                    self.timer_modulo
                );
                self.send_timer_interrupt();
            }
        }

        self.cpu_cycle.wrapping_add(n_ticks);
    }

    pub fn receive_stop(&mut self) {
        self.divider = 0;
    }

    ///////////////////////////////////////////////////////////////////////////
    // Interruptions
    ///////////////////////////////////////////////////////////////////////////
    fn send_joypad_interrupt(&mut self) {
        // INT 0x60
        self.pending_joypad_interruption = true;
    }

    fn send_timer_interrupt(&mut self) {
        // INT 0x50
        self.pending_timer_interruption = true;
    }
}
