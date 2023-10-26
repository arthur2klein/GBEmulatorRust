use crate::screen::KeyState;

/// Contains the memory of the gameboy used to store time and user input
pub struct IO {
    /// State of the buttons of the joypad
    /// unused/unused/are ssba buttons disabled/are movement buttons disabled/
    /// start and down/select and up/b and left/a and right
    joypad_input: u8,
    /// Serial transfer (should not be used)
    serial_transfer: u16,
    /// Divider
    /// Mesure the time
    divider: u8,
    /// Number of cpu cycle elapsed to determin the timers
    cpu_cycle: u16,
    /// Timer counting the time
    timer_counter: u8,
    /// Value at which the timer will be reset after overflow
    timer_modulo: u8,
    /// Controls the timer speed
    /// unused*5/enabled/clock select*2
    timer_control: u8,
    /// Set to non zero to diasable boot ROM
    disable_boot_rom: u8,
    /// Is a joypad interruption waiting to be handled by the CPU?
    pub pending_joypad_interruption: bool,
    /// Is a timer interruption waiting to be handled by the CPU?
    pub pending_timer_interruption: bool,
    /// Other information of IO not used in this project (eg audio)
    other: Vec<u8>,
    /// Is the divider disabled
    is_stopped: bool,
}

impl IO {
    /// Initialize the IO with data set to 0
    ///
    /// # Returns
    /// **IO**: New data with all attributes set to zero
    pub fn new() -> Self {
        Self {
            joypad_input: 0x00,
            serial_transfer: 0x0000,
            divider: 0x00,
            cpu_cycle: 0x0000,
            timer_counter: 0x00,
            timer_modulo: 0x00,
            timer_control: 0x00,
            disable_boot_rom: 0x00,
            pending_joypad_interruption: false,
            pending_timer_interruption: false,
            other: vec![0x00; 256],
            is_stopped: false,
        }
    }
    
    /// Read a byte of the IO memory
    ///
    /// # Arguments
    /// **address (u16)**: Address of the byte to read
    ///
    /// # Returns
    /// **u8**: Value read at the given address of the IO memory
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
            // Set to non zero to diasable boot ROM
            0x50 => {
                self.disable_boot_rom
            },
            _ => {
                self.other[(address & 0x00FF) as usize]
            }
        }
    }

    /// Change a byte in the IO memory
    ///
    /// # Arguments
    /// **address (u16)**: Address of the byte to change
    /// **value (u8)**: New value at the given address of the IO memory
    pub fn write(
        &mut self,
        address: u16,
        value: u8
    ) {
        match (address & 0x00FF) as u8 {
            // Joypad
            0x00 => {
                // The lower nible is read-only
                self.joypad_input =
                    (self.joypad_input & 0x0F) |
                    (value & 0xF0)
                ;
            },
            // Serial transfer (should not be used)
            0x01 => {
                self.serial_transfer = 
                    (value as u16) << 8 |
                    (self.serial_transfer & 0x00FF)
                ;
            },
            0x02 => {
                self.serial_transfer = 
                    (self.serial_transfer & 0xFF00) |
                    value as u16
                ;
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
            // Set to non zero to diasable boot ROM
            0x50 => {
                self.disable_boot_rom = value;
            },
            _ => {
                self.other[(address & 0x00FF) as usize] = value;
            }
        }
    }

    /// Update the joypad byte with the given informations
    ///
    /// # Arguments
    /// **keys (&KeyState)**: Contains information about what key is being
    /// pushed
    fn listen_for_buttons(&mut self, keys: &KeyState) {
        // Was a button already being pushed
        let was_pushed = self.joypad_input & 0x0F == 0x0F;
        let joypad_input_ssba =
            if keys.is_start_pressed {
                0x00
            } else {
                0x08
            } |
            if keys.is_select_pressed {
                0x00
            } else {
                0x04
            } |
            if keys.is_b_pressed {
                0x00
            } else {
                0x02
            } |
            if keys.is_a_pressed {
                0x00
            } else {
                0x01
            }
        ;
        let joypad_movement =
            if keys.is_down_pressed {
                0x00
            } else {
                0x08
            } |
            if keys.is_up_pressed {
                0x00
            } else {
                0x04
            } |
            if keys.is_left_pressed {
                0x00
            } else {
                0x02
            } |
            if keys.is_right_pressed {
                0x00
            } else {
                0x01
            }
        ;
        self.joypad_input |= 0x0F;
        // If the movements keys are used
        if self.joypad_input & 0x10 == 0x00 {
            self.joypad_input &= joypad_movement;
        }
        // If the SSBA keys are being used
        if self.joypad_input & 0x20 == 0x00 {
            self.joypad_input &= joypad_input_ssba;
        }
        // Is a button currently being pushed
        let is_pushed = self.joypad_input & 0x0F == 0x0F;
        if !was_pushed && is_pushed {
            self.send_joypad_interrupt();
        }
    }

    /// Updates the IO memory
    /// 
    /// # Arguments
    /// **n_ticks (u32)**: Number of cpu cycles since the last update
    /// **keys (&KeyState)**: Contains information about what key is being
    /// pushed
    pub fn update(
        &mut self,
        n_ticks: u32,
        keys: &KeyState
    ) {
        self.listen_for_buttons(keys);
        if !self.is_stopped {
            // The clock frequency of the CPU is 4194304 Hz
            // The divider increment frequency is  16384 Hz (every 256 cycle)
            let increment_divider = ((
                ((self.cpu_cycle & 0x00FF).wrapping_add(
                    (n_ticks & 0xFFFF) as u16
                )) & 0xFF00
            ) >> 8) as u8;
            self.divider = self.divider.wrapping_add(increment_divider);
        }
        // The timer is incremented at the clock frequency specified by the TAC
        // register (0xFF07)
        if self.timer_control & 0x20 == 0x20 {
            let increment_timer = match self.timer_control & 0x03 {
                // Frequency: 4096 Hz (1024 cycles)
                0 => {
                    ((
                        (self.cpu_cycle & 0x03FF).wrapping_add(n_ticks as u16)
                    ) & 0xFC00) >> 10
                },
                // Frequency: 262144 Hz (16 cycles)
                1 => {
                    ((
                        (self.cpu_cycle & 0x000F).wrapping_add(n_ticks as u16)
                    ) & 0xFFF0) >> 4
                },
                // Frequency: 65536 Hz (64 cycles)
                2 => {
                    ((
                        (self.cpu_cycle & 0x003F).wrapping_add(n_ticks as u16)
                    ) & 0xFFC0) >> 6
                },
                // Frequency: 16384 Hz (256 cycles)
                3 => {
                    ((
                        (self.cpu_cycle & 0x00FF).wrapping_add(n_ticks as u16)
                    ) & 0xFF00) >> 8
                },
                _ => {
                    panic!("Invalid increment");
                }
            };
            let (timer_counter, did_overflow) = self.timer_counter
                .overflowing_add(increment_timer as u8);
            self.timer_counter = timer_counter;
            // When the value exceeds 0xFF, it is reet to the value specified in
            // TMA (0xFF06) and an interrupt is requested.
            if did_overflow {
                self.timer_counter = self.timer_counter.wrapping_add(
                    self.timer_modulo
                );
                self.send_timer_interrupt();
            }
        }
        self.cpu_cycle = self.cpu_cycle.wrapping_add(n_ticks as u16);
    }

    /// Reacts to a stop instruction
    ///
    /// Re-initialize and stop the divider
    pub fn receive_stop(&mut self) {
        self.divider = 0;
        self.is_stopped = !self.is_stopped;
    }

    /// Signify that a joypad interruption is waiting to be handled by the cpu
    fn send_joypad_interrupt(&mut self) {
        // INT 0x60
        self.pending_joypad_interruption = true;
    }

    /// Signify that a timer interruption is waiting to be handled by the cpu
    fn send_timer_interrupt(&mut self) {
        // INT 0x50
        self.pending_timer_interruption = true;
    }
}
