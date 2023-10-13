pub struct IO {
    // Joypad
    joypad_input: u8,
    // Serial transfer
    serial_transfer: u16,
    // Timer and divider
    divider: u8,
    timer_counter: u8,
    timer_modulo: u8,
    timer_control: u8,
    // Audio
    audio_master_control: u8,
    sound_panning: u8,
    master_volume_and_vin_panning: u8,
    channel_1_sweep: u8,
    channel_1_length_timer_and_duty_cycle u8,
    channel_1_volume_and_envolope: u8,
    channel_1_period_low: u8,
    channel_1_period_high_and_control: u8,
    channel_2_length_timer_and_duty_cycle u8,
    channel_2_volume_and_envolope: u8,
    channel_2_period_low: u8,
    channel_2_period_high_and_control: u8,
    channel_3_dac_enable: u8,
    channel_3_length_timer: u8,
    channel_3_output_level: u8,
    channel_3_period_low: u8,
    channel_3_period_high_and_control: u8,
    channel_4_lenght_timer: u8,
    channel_4_volume_and_enveloper: u8,
    channel_4_frequency_and_randomness: u8,
    channel_4_control: u8,
    // Wave pattern
    wave_pattern_ram: Vec <u8>,
    // LCD Control
    lcd_contrl: u8,
    // LCD status registers
    lcd_y_coordinate: u8,
    lyc_compare: u8,
    lcd_status: u8,
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
            0x00 => {
                self.joypad_input
            },
            0x01 => {
                ((self.serial_transfer & 0xFF00) >> 2) as u8
            },
            0x02 => {
                (self.serial_transfer & 0x00FF) as u8
            },
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
            0x10 => {
                self.channel_1_sweep
            },
            0x11 => {
                self.channel_1_length_timer_and_duty_cycle
            },
            0x12 => {
                self.channel_1_volume_and_envolope
            },
            0x13 => {
                self.channel_1_period_low
            },
            0x14 => {
                self.channel_1_period_high_and_control
            },
            0x16 => {
                self.channel_2_length_timer_and_duty_cycle
            },
            0x17 => {
                self.channel_2_volume_and_envolope
            },
            0x18 => {
                self.channel_2_period_low
            },
            0x19 => {
                self.channel_2_period_high_and_control
            },
            0x1A => {
                self.channel_3_dac_enable
            },
            0x1B => {
                self.channel_3_length_timer
            },
            0x1C => {
                self.channel_3_output_level
            },
            0x1D => {
                self.channel_3_period_low
            },
            0x1E => {
                self.channel_3_period_high_and_control
            },
            0x20 => {
                self.channel_4_lenght_timer
            },
            0x21 => {
                self.channel_4_volume_and_enveloper
            },
            0x22 => {
                self.channel_4_frequency_and_randomness
            },
            0x23 => {
                self.channel_4_control
            },
            0x24 => {
                self.master_volume_and_vin_panning
            },
            0x25 => {
                self.sound_panning
            },
            0x26 => {
                self.audio_master_control
            },
            0x30 => {
                self.wave_pattern_ram[address - 0x30]
            },
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
            0x47 => {
                self.bg_palette_data
            },
            0x48 => {
                self.obp0
            },
            0x49 => {
                self.obp1
            },
            0x50 => {
                self.disable_boot_rom
            },
    }

    pub fn write(
        &mut self,
        address: u16,
        value: u8
    ) {
    }
}
