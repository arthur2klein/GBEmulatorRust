use crate::components::io::IO;
use crate::components::hram::HRAM;
use crate::components::wram::WRAM;
use crate::components::gpu::GPU;
use crate::components::cartridge::Cartridge;

/// Memory management unit of the GameBoy
pub struct MMU {
    /// Interrupt flag: unused/unused/unused/joypad/serial/timer/lcd/vblank
    pub interrupt_flag: u8,
    /// Register that controls the interrupts that are considered to be
    /// enabled and should be triggered
    pub ie: u8,
    /// The cartridge that is currently loaded into the system, going to be
    /// used to access ROM and external RAM banks.
    cartridge: Cartridge,
    /// Reference to the GPU that is going to be used both for VRAM
    /// reading/writing and to forward some of the access operations.
    gpu: GPU,
    /// Working RAM of the system
    wram: WRAM,
    /// High RAM of the system
    hram: HRAM,
    /// I/0 Registers
    io: IO,
    /// Is the gameboy in double speed mode
    is_double_speed: bool
}

impl MMU {
    /// Create a new Memory management unit
    ///
    /// # Arguments
    /// **cartridge_path (&str)**: Path of the file containing the ROM of the
    /// game
    ///
    /// # Returns
    /// **MMU**: New Memory Management Unit
    pub fn new(cartridge_path: &str) -> Self {
        Self {
            interrupt_flag: 0x00,
            ie: 0x00,
            cartridge: Cartridge::new(cartridge_path),
            gpu: GPU::new(),
            wram: WRAM::new(),
            hram: HRAM::new(),
            io: IO::new(),
            is_double_speed: false
        }
    }

    /// Read a byte in the memory of the GameBoy
    ///
    /// # Arguments
    /// **address (u16)**: Address of the byte to read
    ///
    /// # Returns
    /// **u8**: Value read at this address
    pub fn read_byte(
        &self,
        address: u16
    ) -> u8 {
        // https://gbdev.io/pandocs/Memory_Map.html
        match address {
            0xFF0F => {
                self.interrupt_flag
            },
            // LCD
            0x40..=0x4F => {
                self.gpu.read_lcd(address)
            },
            // 16 KiB ROM bank 00
            // From cartridge, usually a fixed bank
            0x0000..=0x3FFF => {
                self.cartridge.read_rom(address)
            },
            // 16 KiB ROM Bank 01~NN
            // From cartridge, switchable bank via mapper (if any)
            0x4000..=0x7FFF => {
                self.cartridge.read_rom(address)
            },
            // 8Kib Video RAM (VRAM)
            // In CGB mode, switchable bank 0/1
            0x8000..=0x9FFF => {
                self.gpu.read_ram(address)
            },
            // 8 Kib External RAM
            // From cartridge, switchable bank if any
            0xA000..=0xBFFF => { 
                self.cartridge.read_ram(address)
            },
            // 4 KiB Work RAM (WRAM)
            //
            0xC000..=0xCFFF => { 
                self.wram.read(address)
            },
            // 4Kib Work RAM (WRAM)
            // In CGB mode, switchable bank 1~7
            0xD000..=0xDFFF => {
                self.wram.read(address)
            },
            // Mirror of C000~DDFF (ECHO RAM)
            // Nintendo says use of this area is prohibited
            0xE000..=0xFDFF => {
                self.wram.read(address - 0x2000)
            },
            // Object attribute Memory (OAM)
            //
            0xFE00..=0xFE9F => {
                self.gpu.read_oam(address)
            },
            // Not Usable
            // Nintendo says use of this area is prohibited
            0xFEA0..=0xFEFF => {
                panic!("Tried to access to a prohibited memory address");
            },
            // I/0 Registers
            //
            0xFF00..=0xFF7F => {
                self.io.read(address)
            }
            // High RAM (HRAM)
            //
            0xFF80..=0xFFFE => {
                self.hram.read(address)
            }
            // Interrupt Enable register
            // unused/unused/unused/joypad/serial/timer/lcd/vblank
            0xFFFF => {
                self.ie
            }
        }
    }

    /// Change a byte in the memory of the GameBoy
    ///
    /// # Arguments
    /// **address (u16)**: Address of the byte to modifiy
    /// **value (u8)**: New value to put at this address
    pub fn write_byte(
        &mut self,
        address: u16,
        value: u8
    ) {
        // https://gbdev.io/pandocs/Memory_Map.html
        match address {
            0xFF0F => {
                self.interrupt_flag = value;
            },
            // LCD
            0x40..=0x4F => {
                self.gpu.write_lcd(address, value);
            },
            // 16 KiB ROM bank 00
            // From cartridge, usually a fixed bank
            0x0000..=0x3FFF => {
                self.cartridge.write_rom(
                    address,
                    value
                );
            },
            // 16 KiB ROM Bank 01~NN
            // From cartridge, switchable bank via mapper (if any)
            0x4000..=0x7FFF => {
                self.cartridge.write_rom(
                    address,
                    value
                );
            },
            // 8Kib Video RAM (VRAM)
            // In CGB mode, switchable bank 0/1
            0x8000..=0x9FFF => {
                self.gpu.write_ram(
                    address,
                    value
                );
            },
            // 8 Kib External RAM
            // From cartridge, switchable bank if any
            0xA000..=0xBFFF => { 
                self.cartridge.write_ram(
                    address,
                    value
                );
            },
            // 4 KiB Work RAM (WRAM)
            //
            0xC000..=0xCFFF => { 
                self.wram.write(
                    address,
                    value
                );
            },
            // 4Kib Work RAM (WRAM)
            // In CGB mode, switchable bank 1~7
            0xD000..=0xDFFF => {
                self.wram.write(
                    address,
                    value
                );
            },
            // Mirror of C000~DDFF (ECHO RAM)
            // Nintendo says use of this area is prohibited
            0xE000..=0xFDFF => {
                self.wram.write(
                    address - 0x2000,
                    value
                );
            },
            // Object attribute Memory (OAM)
            //
            0xFE00..=0xFE9F => {
                self.gpu.write_oam(
                    address,
                    value
                );
            },
            // Not Usable
            // Nintendo says use of this area is prohibited
            0xFEA0..=0xFEFF => {
                panic!("Tried to access to a prohibited memory address");
            },
            // I/0 Registers
            //
            0xFF00..=0xFF7F => {
                self.io.write(
                    address,
                    value
                );
            }
            // High RAM (HRAM)
            //
            0xFF80..=0xFFFE => {
                self.hram.write(
                    address,
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

    /// Read a word in the memory of the GameBoy
    ///
    /// # Arguments
    /// **address (u16)**: Address of the word to read
    ///
    /// # Returns
    /// **u16**: Value read at this address
    pub fn read_word(
        &self,
        address: u16
    ) -> u16 {
        ((self.read_byte(address + 1) as u16) << 8) |
            (self.read_byte(address) as u16)
    }

    /// Change a word in the memory of the GameBoy
    ///
    /// # Arguments
    /// **address (u16)**: Address of the word to modifiy
    /// **value (u16)**: New value to put at this address
    pub fn write_word(
        &mut self,
        address: u16,
        value: u16
    ) {
        self.write_byte(
            address + 1,
            ((value & 0xFF00) >> 8) as u8
        );
        self.write_byte(
            address,
            (value & 0x00FF) as u8
        );
    }

    /// Updates the memory
    ///
    /// # Arguments
    /// **n_cycyles (u32)**: Number of cpu cycles since the last update
    ///
    /// # Returns
    /// **bool**: True iff the escape key is pressed
    pub fn update(
        &mut self,
        n_cycles: u32,
    ) -> bool {
        self.io.update(
            n_cycles,
            self.gpu.transmit_key()
        );
        let res = self.gpu.update(n_cycles as u16);
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
        res
    }

    /// React to a stop from the cpu
    ///
    /// Change the cpu cycle and transmit the stop to the memory zone that use
    /// it
    pub fn receive_stop(&mut self) {
        self.is_double_speed = !self.is_double_speed;
        self.io.receive_stop();
    }

    /// Function called when the MMU is no longer needed
    pub fn close(&self) {
        self.cartridge.close();
    }
}
