use crate::cartridge::Cartridge;
use crate::gpu::GPU;
use crate::wram::WRAM;
use crate::hram::HRAM;
use crate::io::IO;

pub struct MMU<'a> {
    /* Interrupt flag: unused/unused/unused/joypad/serial/timer/lcd/vblank */
    interrupt_flag: u8,
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
    io: IO<'a>,
    /* Is the gameboy in double speed mode */
    is_double_speed: bool
}

impl MMU<'_> {
    pub fn new(cartridge_path: &str) -> Self {
        let gpu = GPU::new();
        Self {
            interrupt_flag: 0x00,
            ie: 0x00,
            cartridge: Cartridge::new(cartridge_path),
            gpu,
            wram: WRAM::new(),
            hram: HRAM::new(),
            io: IO::new(gpu),
            is_double_speed: false
        }
    }

    pub fn read_byte(
        &self,
        adress: u16
    ) -> u8 {
        // https://gbdev.io/pandocs/Memory_Map.html
        match adress {
            0xFF0F => {
                self.interrupt_flag
            },
            // 16 KiB ROM bank 00
            // From cartridge, usually a fixed bank
            0x0000..=0x3FFF => {
                self.cartridge.read_rom(adress)
            },
            // 16 KiB ROM Bank 01~NN
            // From cartridge, switchable bank via mapper (if any)
            0x4000..=0x7FFF => {
                self.cartridge.read_rom(adress)
            },
            // 8Kib Video RAM (VRAM)
            // In CGB mode, switchable bank 0/1
            0x8000..=0x9FFF => {
                self.gpu.read_ram(adress)
            },
            // 8 Kib External RAM
            // From cartridge, switchable bank if any
            0xA000..=0xBFFF => { 
                self.cartridge.read_ram(adress)
            },
            // 4 KiB Work RAM (WRAM)
            //
            0xC000..=0xCFFF => { 
                self.wram.read(adress)
            },
            // 4Kib Work RAM (WRAM)
            // In CGB mode, switchable bank 1~7
            0xD000..=0xDFFF => {
                self.wram.read(adress)
            },
            // Mirror of C000~DDFF (ECHO RAM)
            // Nintendo says use of this area is prohibited
            0xE000..=0xFDFF => {
                self.wram.read(adress - 0x2000)
            },
            // Object attribute Memory (OAM)
            //
            0xFE00..=0xFE9F => {
                self.gpu.read_oam(adress)
            },
            // Not Usable
            // Nintendo says use of this area is prohibited
            0xFEA0..=0xFEFF => {
                panic!("Tried to access to a prohibited memory adress");
                0
            },
            // I/0 Registers
            //
            0xFF00..=0xFF7F => {
                self.io.read(adress)
            }
            // High RAM (HRAM)
            //
            0xFF80..=0xFFFE => {
                self.hram.read(adress)
            }
            // Interrupt Enable register
            // unused/unused/unused/joypad/serial/timer/lcd/vblank
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
            0xFF0F => {
                self.interrupt_flag = value;
            },
            // 16 KiB ROM bank 00
            // From cartridge, usually a fixed bank
            0x0000..=0x3FFF => {
                self.cartridge.write_rom(
                    adress,
                    value
                );
            },
            // 16 KiB ROM Bank 01~NN
            // From cartridge, switchable bank via mapper (if any)
            0x4000..=0x7FFF => {
                self.cartridge.write_rom(
                    adress,
                    value
                );
            },
            // 8Kib Video RAM (VRAM)
            // In CGB mode, switchable bank 0/1
            0x8000..=0x9FFF => {
                self.gpu.write_ram(
                    adress,
                    value
                );
            },
            // 8 Kib External RAM
            // From cartridge, switchable bank if any
            0xA000..=0xBFFF => { 
                self.cartridge.write_ram(
                    adress,
                    value
                );
            },
            // 4 KiB Work RAM (WRAM)
            //
            0xC000..=0xCFFF => { 
                self.wram.write(
                    adress,
                    value
                );
            },
            // 4Kib Work RAM (WRAM)
            // In CGB mode, switchable bank 1~7
            0xD000..=0xDFFF => {
                self.wram.write(
                    adress,
                    value
                );
            },
            // Mirror of C000~DDFF (ECHO RAM)
            // Nintendo says use of this area is prohibited
            0xE000..=0xFDFF => {
                self.wram.write(
                    adress - 0x2000,
                    value
                );
            },
            // Object attribute Memory (OAM)
            //
            0xFE00..=0xFE9F => {
                self.gpu.write_oam(
                    adress,
                    value
                );
            },
            // Not Usable
            // Nintendo says use of this area is prohibited
            0xFEA0..=0xFEFF => {
                panic!("Tried to access to a prohibited memory adress");
            },
            // I/0 Registers
            //
            0xFF00..=0xFF7F => {
                self.io.write(
                    adress,
                    value
                );
            }
            // High RAM (HRAM)
            //
            0xFF80..=0xFFFE => {
                self.hram.write(
                    adress,
                    value
                );
            }
            // Interrupt Enable register
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
        ((self.read_byte(adress) as u16) << 8) |
            (self.read_byte(adress + 1) as u16)
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

    pub fn update(
        &mut self,
        n_cycles: u32,
    ) {
        self.io.update(n_cycles);
        self.gpu.update(n_cycles);
        // INT 0x60
        if self.io.pending_joypad_interruption {
            self.interrupt_flag |= 0x10;
            self.io.pending_joypad_interruption = false;
        }
        // INT 0x50
        if self.io.pending_timer_interruption {
            self.interrupt_flag |= 0x04;
            self.io.pending_timer_interruption = false;
        }
        // INT 0x48
        if self.gpu.pending_stat_interrupt {
            self.interrupt_flag |= 0x02;
            self.gpu.pending_stat_interrupt  = false;
        }
        // INT 0x40
        if self.gpu.pending_vblank_interrupt {
            self.interrupt_flag |= 0x01;
            self.gpu.pending_stat_interrupt  = false;
        }
    }

    pub fn receive_stop(&mut self) {
        self.is_double_speed = !self.double_speed;
    }
}
