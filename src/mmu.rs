pub struct MMU {
    /* Register that controls the interrupts that are considered
     to be enabled and should be triggered. */
    ie: u8,
    /* The cartridge that is currently loaded into the system,
    going to be used to access ROM and external RAM banks.*/
    cartridge: Cartridge,
    /* Reference to the GPU that is going to be used both for VRAM
    reading/writing and to forward some of the access operations. */
    gpu: GPU,
    /* Working RAM of the system */
    wram: WRAM,
    /* High RAM of the system */
    hram: HRAM,
    /* I/0 Registers */
    io: IO,
    /* Is the gameboy in double speed mode */
    is_double_speed: bool
}

impl MMU {
    pub new() -> Self {
        Self {
            ie: 0x00,
            cartridge:, //TODO
            gpu: , //TODO
            wram: , //TODO
            hram: , //TODO
            io: , //TODO
            is_double_speed: false
        }
    }

    pub fn read_byte(
        &self,
        adress: u16
    ) -> u8 {
        // https://gbdev.io/pandocs/Memory_Map.html
        match adress {
            // 16 KiB ROM bank 00
            // From cartridge, usually a fixed bank
            0x0000..0x4000 => {
                self.cartridge.read_rom(adress)
            },
            // 16 KiB ROM Bank 01~NN
            // From cartridge, switchable bank via mapper (if any)
            0x4000..0x8000 => {
                self.cartridge.read_rom(adress)
            },
            // 8Kib Video RAM (VRAM)
            // In CGB mode, switchable bank 0/1
            0x8000..0xA000 => {
                self.gpu.read_ram(adress)
            },
            // 8 Kib External RAM
            // From cartridge, switchable bank if any
            0xA000..0xC000 => { 
                self.cartridge.read_ram(adress)
            },
            // 4 KiB Work RAM (WRAM)
            //
            0xC000..0xD000 => { 
                self.wram.read(adress)
            },
            // 4Kib Work RAM (WRAM)
            // In CGB mode, switchable bank 1~7
            0xD000..0xE000 => {
                self.wram.read(adress)
            },
            // Mirror of C000~DDFF (ECHO RAM)
            // Nintendo says use of this area is prohibited
            0xE000..0xFE00 => {
                self.wram.read(adress - 0x2000)
            },
            // Object attribute Memory (OAM)
            //
            0xFE00..0xFEA0 => {
                self.gpu.read_oam(adress)
            },
            // Not Usable
            // Nintendo says use of this area is prohibited
            0xFEA0..0xFF00 => {
                panic!("Tried to access to a prohibited memory adress");
                0
            },
            // I/0 Registers
            //
            0xFF00..0xFF80 => {
                self.io.read(adress)
            }
            // High RAM (HRAM)
            //
            0xFF80..0xFFFF => {
                self.hram.read(adress)
            }
            // Interrubp Enable register
            //
            0xFFFF => {
                self.ie
            }
        }
    }

    pub fn write_byte(
        &mut self,
        adress: u16,
        value: u8
    ) {
        // https://gbdev.io/pandocs/Memory_Map.html
        match adress {
            // 16 KiB ROM bank 00
            // From cartridge, usually a fixed bank
            0x0000..0x4000 => {
                self.cartridge.write_rom(
                    adress,
                    value
                );
            },
            // 16 KiB ROM Bank 01~NN
            // From cartridge, switchable bank via mapper (if any)
            0x4000..0x8000 => {
                self.cartridge.write_rom(
                    adress,
                    value
                );
            },
            // 8Kib Video RAM (VRAM)
            // In CGB mode, switchable bank 0/1
            0x8000..0xA000 => {
                self.gpu.write_ram(
                    adress,
                    value
                );
            },
            // 8 Kib External RAM
            // From cartridge, switchable bank if any
            0xA000..0xC000 => { 
                self.cartridge.write_ram(
                    adress,
                    value
                );
            },
            // 4 KiB Work RAM (WRAM)
            //
            0xC000..0xD000 => { 
                self.wram.write(
                    adress,
                    value
                );
            },
            // 4Kib Work RAM (WRAM)
            // In CGB mode, switchable bank 1~7
            0xD000..0xE000 => {
                self.wram.write(
                    adress,
                    value
                );
            },
            // Mirror of C000~DDFF (ECHO RAM)
            // Nintendo says use of this area is prohibited
            0xE000..0xFE00 => {
                self.wram.write(
                    adress - 0x2000,
                    value
                );
            },
            // Object attribute Memory (OAM)
            //
            0xFE00..0xFEA0 => {
                self.gpu.write_oam(
                    adress,
                    value
                );
            },
            // Not Usable
            // Nintendo says use of this area is prohibited
            0xFEA0..0xFF00 => {
                panic!("Tried to access to a prohibited memory adress");
            },
            // I/0 Registers
            //
            0xFF00..0xFF80 => {
                self.io.write(
                    adress,
                    value
                );
            }
            // High RAM (HRAM)
            //
            0xFF80..0xFFFF => {
                self.hram.write(
                    adress,
                    value
                );
            }
            // Interrubp Enable register
            //
            0xFFFF => {
                self.ie = value;
            }
        }
    }

    pub fn read_word(
        &self,
        adress: u16
    ) -> u16 {
        (
            ((self.read_byte(adress) as u16) << 8) |
            (self.read_byte(adress + 1) as u16)
        )
    }

    pub fn write_word(
        &mut self,
        adress: u16,
        value: u16
    ) {
        self.write_byte(
            adress,
            ((value & 0xFF00) >> 8) as u8
        );
        self.write_byte(
            adress,
            (value & 0x00FF) as u8
        );
    }

    pub fn update(&mut self, n_cycles: u32) {
        io.update(n_cycles);
    }

    pub fn receive_stop(&mut self) {
        self.is_double_speed = !self.double_speed;
    }
}
