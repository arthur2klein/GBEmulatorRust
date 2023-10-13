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
    wave_pattern_ram: u16,
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
    // VRAM Bank
    vram_bank: u8,
    // Set to non zero to diasable boot ROM
    disable_boot_rom: u8,
    // VRAM DMA
    hdma1: u8,
    hdma1: u8,
    hdma3: u8,
    hdma4: u8,
    hdma5: u8a,
    // LCD Color palettes
    background_color_palette_specification: u8,
    background_color_palette_data: u8,
    obj_color_palette_specification: u8,
    obj_color_palette_data: u8,
    // WRAM Bank Select
    wram_bank_select: u8

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

    pub fn read(&self, adress: u16) -> u8 {
        match (adress & 0x00FF) as u8 {
            0x00 => {
                joypad_input
            },
            0x01 => {
                ((serial_transfer & 0xFF00) >> 2) as u8
            },
            0x02 => {
                (serial_transfer & 0x00FF) as u8
            },
            0x04 => {
                divider
            },
            0x05 => {
                timer_counter
            },
            0x06 => {
                timer_modulo
            },
            0x07 => {
                timer_control
            },
            0x10 => {
                channel_1_sweep
            },
            0x11 => {
                channel_1_length_timer_and_duty_cycle
            },
            0x12 => {
                channel_1_volume_and_envolope
            },
            0x13 => {
                channel_1_period_low
            },
            0x14 => {
                channel_1_period_high_and_control
            },
            0x16 => {
                channel_2_length_timer_and_duty_cycle
            },
            0x24 => {
                master_volume_and_vin_panning
            },
            0x25 => {
                sound_panning
            },
            0x26 => {
                audio_master_control
            },
            
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
            wave_pattern_ram: u16,
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
            // VRAM Bank
            vram_bank: u8,
            // Set to non zero to diasable boot ROM
            disable_boot_rom: u8,
            // VRAM DMA
            hdma1: u8,
            hdma1: u8,
            hdma3: u8,
            hdma4: u8,
            hdma5: u8a,
            // LCD Color palettes
            background_color_palette_specification: u8,
            background_color_palette_data: u8,
            obj_color_palette_specification: u8,
            obj_color_palette_data: u8,
            // WRAM Bank Select
            wram_bank_select: u8
        }
    }

    pub fn write(
        &mut self,
        adress: u16,
        value: u8
    ) {
    }
}
