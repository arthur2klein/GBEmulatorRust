use std::time::{Duration, SystemTime};
use std::thread::sleep;
use crate::mmu::MMU;

/// This macro creates accessors for the 16 bit register obtained by combining
/// the two given 8 bits register
#[allow(unused_macros)]
macro_rules! form_16_bits_register {
    ($register1: ident, $register2: ident) => {
        paste::item! {
            #[doc = concat!(
                "Returns the value of the 16 bit register ",
                stringify!($register1),
                stringify!($register2),
                " obtained by reading the bits of ",
                stringify!($register1),
                " and then those of ",
                stringify!($register2)
            )]
            /// # Returns
            /// **u16**: Value of the 16 bit register.
            ///
            /// # Examples
            /// ``` rust
            /// let mut new_registers = Registers::new();
            #[doc = concat!(
                "new_registers.",
                stringify!($register1),
                "= 0x12;"
            )]
            #[doc = concat!(
                "new_registers.",
                stringify!($register2),
                "= 0x34;"
            )]
            #[doc = concat!(
                "assert_eq!(",
                "new_registers.get_",
                stringify!($register1),
                stringify!($register2),
                "(), 0x1234);"
            )]
            /// ```
            fn [< get_ $register1 $register2 >](&self) -> u16 {
                (self.$register1 as u16) << 8
                   | self.$register2 as u16
            }

            #[doc = concat!(
                "Modify the value of the 16 bit register ",
                stringify!($register1),
                stringify!($register2),
                " obtained by reading the bits of ",
                stringify!($register1),
                " and then those of ",
                stringify!($register2)
            )]
            /// # Arguments
            /// **value (u16)**: New value of the 16 bit register.
            ///
            /// # Examples
            /// ``` rust
            /// let mut new_registers = Registers::new();
            #[doc = concat!(
                "new_registers.set_",
                stringify!($register1),
                stringify!($register2),
                "(0x1234);"
            )]
            #[doc = concat!(
                "assert_eq!(new_registers.",
                stringify!($register1),
                ", 0x12);"
            )]
            #[doc = concat!(
                "assert_eq!(new_registers.",
                stringify!($register2),
                ", 0x34);"
            )]
            /// ```
            fn [< set_ $register1 $register2 >](&mut self, value: u16) {
                self.$register1 = ((value & 0xFF00) >> 8) as u8;
                self.$register2 = (value & 0xFF) as u8;
            }
        }
    }
}

/// The registers used by the CPU to store values
struct Registers {
    /// 8 bit register A 
    a: u8,
    /// 8 bit register B
    b: u8,
    /// 8 bit register C
    c: u8,
    /// 8 bit register D
    d: u8,
    /// 8 bit register E
    e: u8,
    /// 8 bit register F
    f: u8,
    /// 8 bit register H
    h: u8,
    /// 8 bit register L
    l: u8,
    /// 16 bit Program Counter register
    pc: u16,
    /// 16 bit Stack Pointer register
    sp: u16,
}

impl Registers {
    /// Create the registers with their initial values
    ///
    /// # Returns
    /// 
    /// **Register**: New instance of Registers
    ///
    /// # Examples
    /// ``` rust
    /// let mut new_registers = Registers::new();
    /// ```
    fn new() -> Self {
        Registers {
            a: 0x01,
            b: 0xB0,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            f: 0x00,
            h: 0x01,
            l: 0x4D,
            pc: 0x0100,
            sp: 0xFFFE,
        }
    }

    form_16_bits_register!(a, f);
    form_16_bits_register!(b, c);
    form_16_bits_register!(d, e);
    form_16_bits_register!(h, l);

    /// Returns the current value of the 16 bit register HL and decrement it
    ///
    /// The value of HL is obtained by reading the bits of H and then those of
    /// L
    ///
    /// # Returns
    /// **u16**: Current value of the 16 bit register.
    ///
    /// # Examples
    /// ``` rust
    /// let mut new_registers = Registers::new();
    /// new_registers.h = 0x12;
    /// new_registers.l = 0x34;
    /// // The current value is still returned
    /// assert_eq!(new_registers.get_hld(), 0x1234);
    /// // But it is changed for the next evaluation
    /// assert_eq!(new_registers.get_hl(), 0x1233);
    /// ```
    fn get_hld(&mut self) -> u16 {
        let res = self.get_hl();
        self.set_hl(res - 1);
        res
    }

    /// Returns the current value of the 16 bit register HL and increment it
    ///
    /// The value of HL is obtained by reading the bits of H and then those of
    /// L
    ///
    /// # Returns
    /// **u16**: Current value of the 16 bit register.
    ///
    /// # Examples
    /// ``` rust
    /// let mut new_registers = Registers::new();
    /// new_registers.h = 0x12;
    /// new_registers.l = 0x34;
    /// // The current value is still returned
    /// assert_eq!(new_registers.get_hli(), 0x1234);
    /// // But it is changed for the next evaluation
    /// assert_eq!(new_registers.get_hl(), 0x1235);
    /// ```
    fn get_hli(&mut self) -> u16 {
        let res = self.get_hl();
        self.set_hl(res + 1);
        res
    }

    /// Returns the value of the carry flag (aka C flag)
    ///
    /// The carry flag is generally set when the previous operation overflows
    ///
    /// # Returns
    /// **bool**: true iff the carry flag is set
    ///
    /// # Examples
    /// ``` rust
    /// let mut new_registers = Registers::new();
    /// // Sets the carry flag
    /// new_registers.f = 0x10;
    /// assert!(new_registers.get_carry());
    /// ```
    fn get_carry(&self) -> bool {
        self.f & 0b00010000 != 0
    }

    /// Assigns the wanted value to the carry flag (aka C flag)
    ///
    /// The carry flag is generally set when the previous operation overflows
    ///
    /// # Arguments
    /// **value (bool)**: true iff you want the carry flag is to be set
    ///
    /// # Examples
    /// ``` rust
    /// let mut new_registers = Registers::new();
    /// new_registers.set_carry(true);
    /// // Only the carry flag has been set
    /// assert_eq!(new_registers.f, 0x10);
    /// ```
    fn set_carry(&mut self, value: bool) {
        if value {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
    }

    /// Returns the value of the half carry flag (aka H flag)
    ///
    /// The half carry flag is generally set when the previous operation
    /// overflows considering only the first half of the operators
    ///
    /// # Returns
    /// **bool**: true iff the half carry flag is set
    ///
    /// # Examples
    /// ``` rust
    /// let mut new_registers = Registers::new();
    /// // Sets the half carry flag
    /// new_registers.f = 0x20;
    /// assert!(new_registers.get_half());
    /// ```
    fn get_half(&self) -> bool {
        self.f & 0b00100000 != 0
    }

    /// Assigns the wanted value to the half carry flag (aka H flag)
    ///
    /// The half carry flag is generally set when the previous operation
    /// overflows considering only the first half of the operators
    ///
    /// # Arguments
    /// **value (bool)**: true iff you want the half carry flag is to be set
    ///
    /// # Examples
    /// ``` rust
    /// let mut new_registers = Registers::new();
    /// new_registers.set_half(true);
    /// // Only the half carry flag has been set
    /// assert_eq!(new_registers.f, 0x20);
    /// ```
    fn set_half(&mut self, value: bool) {
        if value {
            self.f |= 0b00100000;
        } else {
            self.f &= 0b11011111;
        }
    }

    /// Returns the value of the substraction flag (aka N flag)
    ///
    /// The substraction flag is generally set when the previous operation is
    /// a substraction
    ///
    /// # Returns
    /// **bool**: true iff the substraction flag is set
    ///
    /// # Examples
    /// ``` rust
    /// let mut new_registers = Registers::new();
    /// // Sets the substraction flag
    /// new_registers.f = 0x40;
    /// assert!(new_registers.get_sub());
    /// ```
    fn get_sub(&self) -> bool {
        self.f & 0b01000000 != 0
    }

    /// Assigns the wanted value to the substraction flag (aka N flag)
    ///
    /// The substraction flag is generally set when the previous operation is
    /// a substraction
    ///
    /// # Arguments
    /// **value (bool)**: true iff you want the substaction flag is to be set
    ///
    /// # Examples
    /// ``` rust
    /// let mut new_registers = Registers::new();
    /// new_registers.set_sub(true);
    /// // Only the substraction has been set
    /// assert_eq!(new_registers.f, 0x40);
    /// ```
    fn set_sub(&mut self, value: bool) {
        if value {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
    }

    /// Returns the value of the zero flag (aka Z flag)
    ///
    /// The zero flag is generally set when the result of the previous
    /// operation is 0
    ///
    /// # Returns
    /// **bool**: true iff the zero flag is set
    ///
    /// # Examples
    /// ``` rust
    /// let mut new_registers = Registers::new();
    /// // Sets the zero flag
    /// new_registers.f = 0x80;
    /// assert!(new_registers.get_zero());
    /// ```
    fn get_zero(&self) -> bool {
        self.f & 0b10000000 != 0
    }

    /// Assigns the wanted value to the zero flag (aka Z flag)
    ///
    /// The zero flag is generally set when the result of the previous
    /// operation is 0
    ///
    /// # Arguments
    /// **value (bool)**: true iff you want the zero flag is to be set
    ///
    /// # Examples
    /// ``` rust
    /// let mut new_registers = Registers::new();
    /// new_registers.set_zero(true);
    /// // Only the zero has been set
    /// assert_eq!(new_registers.f, 0x80);
    /// ```
    fn set_zero(&mut self, value: bool) {
        if value {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
    }
}

/// The CPU of the gameboy
pub struct CPU {
    /// The registers used by the CPU to store values
    registers: Registers,
    /// The memory management unit allows the CPU to communicate with the
    /// memory
    mmu: MMU,
    /// Stops the CPU until an interruption is pending
    is_halted: bool,
    /// Enable interruptions
    /// If 1, enable interruptions ; if 2, enable interruptions after next
    /// instruction
    ei: u32,
    /// Disable interrputions
    /// If 1, disable interruptions ; if 2, disable interruptons after next
    /// instruction
    di: u32,
    /// Should pending interrpution be managed?
    ime: bool,
    /// Has the user asked for the program to stop
    should_stop: bool,
}

impl CPU {
    /// Create the CPU of the gameboy
    ///
    /// # Returns
    /// 
    /// **CPU**: New instance of CPU
    ///
    /// # Examples
    /// ``` rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// ```
    pub fn new(cartridge_path: &str) -> Self {
        CPU{
            registers: Registers::new(),
            mmu: MMU::new(cartridge_path),
            is_halted: false,
            ei: 0,
            di: 0,
            ime: true,
            should_stop: false,
        }
    }

    /// Gets an immediate value as a byte in the instructions of the code
    ///
    /// # Retuns
    /// **u8**: Byte read in the code of the program
    ///
    /// # Examples
    /// ``` rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.mmu.write_byte(
    ///     new_cpu.registers.pc,
    ///     0x12
    /// );
    /// assert_eq!(new_cpu.fetchbyte(), 0x12);
    /// ```
    fn fetchbyte(&mut self) -> u8 {
        let res = self.mmu.read_byte(self.registers.pc);
        println!("pc = {:#04x}, res = {:#02x}", self.registers.pc, res);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        res
    }

    /// Gets an immediate value as a word in the instructions of the code
    ///
    /// # Retuns
    /// **u16**: Word read in the code of the program
    ///
    /// # Examples
    /// ``` rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.mmu.write_word(
    ///     new_cpu.registers.pc,
    ///     0x1234
    /// );
    /// assert_eq!(new_cpu.fetchword(), 0x1234);
    /// ```
    fn fetchword(&mut self) -> u16 {
        let res = self.mmu.read_word(self.registers.pc);
        println!("pc = {:#04x}, res = {:#04x}", self.registers.pc, res);
        self.registers.pc = self.registers.pc.wrapping_add(2);
        res
    }

    /// Sends a stop message to the MMU
    ///
    /// Switch the speed of the Memory Management Unit
    ///
    /// # Examples
    /// ``` rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// assert!(!new_cpu.mmu.is_double_speed);
    /// new_cpu.send_stop();
    /// assert!(new_cpu.mmu.is_double_speed);
    /// ```
    fn send_stop(&mut self) {
        self.mmu.receive_stop();
    }

    /// Stops the gameboy until an interruption is triggered
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// // Deactivate interruption
    /// new_cpu.ime = false;
    /// new_cpu.halt();
    /// // Now the CPU will only execute NOP
    /// ```
    fn halt(&mut self) {
        self.is_halted = true;
    }

    /// Pops a value from the stack
    ///
    /// # Returns
    /// **u16**: value popped from the stack
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.push(0x1234);
    /// assert_eq!(new_cpu.pop(), 0x1234);
    /// ```
    fn pop(&mut self) -> u16 {
        let res = self.mmu.read_word(
            self.registers.sp
        );
        self.registers.sp = self.registers.sp.wrapping_add(2);
        res
    }

    /// Pushes the given value in the stack
    ///
    /// # Arguments
    /// **value (u16)**: value to push in the stack
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.push(0x1234);
    /// assert_eq!(new_cpu.pop(), 0x1234);
    /// ```
    fn push(&mut self, value: u16) {
        self.registers.sp = self.registers.sp.wrapping_sub(2);
        self.mmu.write_word(
            self.registers.sp,
            value
        );
    }

    /// Jumps to the given value after pushing the value of the program counter
    /// in the stack
    ///
    /// # Arguments
    /// **value (u16)**: new value of the program counter
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.rst(0x0060);
    /// assert_eq!(new_cpu.registers.pc, 0x0060);
    /// ```
    fn rst(&mut self, value: u16) {
        self.push(self.registers.pc);
        self.registers.pc = value;
    }

    /// Jumps by the value of the next immediate in the program read as a real
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// let before = new_cpu.registers.get_pc();
    /// // Normally the adress should already be written in test.gb
    /// new_cpu.mmu.write_byte(
    ///     new_cpu.registers.pc + 1,
    ///     0x12
    /// );
    /// new_cpu.jr();
    /// assert_eq!(new_cpu.registers.pc - before, 0x12);
    /// ```
    fn jr(&mut self) {
        // Les conversions permettent d'assurer que fetchbyte est considéré
        // comme signé, mais pas pc, que l'opérations puissnet avoir lieu, et
        // que le résutat ait le bon format
        self.registers.pc = (
            (self.registers.pc as u32 as i32) +
            (self.fetchbyte() as i8 as i32)
        ) as u16;
    }

    /// Make the CPU work indefinitively
    ///
    /// # Examples
    /// ```rust
    /// let new_cpu = CPU::new("test.gb");
    /// // Let's hope the cartridge "test.gb" contains something
    /// new_cpu.run();
    /// ```
    pub fn run(&mut self) {
        while !self.should_stop {
            let time = SystemTime::now();
            let time_used = self.execute_step();
            // One cycle lasts 2385ns
            sleep(
                Duration::from_nanos((2385 * time_used) as u64).saturating_sub(
                    time.elapsed().unwrap()
                )
            );
        }
    }

    /// Reads an instruction and execute it from the normal table
    ///
    /// <https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html>
    ///
    /// # Returns
    /// **u32**: Number of cycles used for the step
    ///
    /// # Examples
    /// ```rust
    /// let new_cpu = CPU::new("test.gb");
    /// new_cpu.receive_op();
    /// ```
    fn receive_op(&mut self) -> u32 {
        println!("Execution of the operation at address {}/{}", self.registers.pc, 0x4000);
        assert!(self.registers.pc < 0x4000);
        let op = self.fetchbyte();
        match op {
            // NOP
            0x00 => {
                println!("NOP");
                4
            },
            // LD BC, d16
            0x01 => {
                println!("LD BC, d16");
                let word = self.fetchword();
                self.registers.set_bc(word);
                12
            },
            // LD (BC), A
            0x02 => {
                println!("LD (BC), A");
                self.mmu.write_byte(
                    self.registers.get_bc(),
                    self.registers.a
                );
                8
            },
            // INC BC
            0x03 => {
                println!("INC BC");
                self.registers.set_bc(
                    self.registers.get_bc().wrapping_add(1)
                );
                8
            },
            // INC B
            0x04 => {
                println!("INC B");
                self.registers.b = self.inc(self.registers.b);
                4
            },
            // DEC B
            0x05 => {
                println!("DEC B");
                self.registers.b = self.dec(self.registers.b);
                4
            },
            // LD B, d8
            0x06 => {
                println!("LD B, d8");
                self.registers.b = self.fetchbyte();
                8
            },
            // RLCA
            0x07 => {
                println!("RLCA");
                self.registers.a = self.rlc(self.registers.a);
                self.registers.set_zero(false);
                4
            },
            // LD (a16), SP
            0x08 => {
                println!("LD (a16), SP");
                let word = self.fetchword();
                self.mmu.write_word(
                    word,
                    self.registers.sp
                );
                20
            },
            // ADD HL, BC
            0x09 => {
                println!("ADD HL, BC");
                self.addhl(self.registers.get_bc());
                8
            },
            // LD A, (BC)
            0x0A => {
                println!("LD A, (BC)");
                self.registers.a = self.mmu.read_byte(self.registers.get_bc());
                8
            },
            // DEC BC
            0x0B => {
                println!("DEC BC");
                self.registers.set_bc(self.registers.get_bc().wrapping_sub(1));
                8
            },
            // INC C
            0x0C => {
                println!("INC C");
                self.registers.c = self.inc(self.registers.c);
                4
            },
            // DEC C
            0x0D => {
                println!("DEC C");
                self.registers.c = self.dec(self.registers.c);
                4
            },
            // LD C, d8
            0x0E => {
                println!("LD C, d8");
                self.registers.c = self.fetchbyte();
                8
            },
            // RRCA
            0x0F => {
                println!("RRCA");
                self.registers.a = self.rrc(self.registers.a);
                self.registers.set_zero(false);
                4
            },
            // STOP A
            0x10 => {
                println!("STOP A");
                self.send_stop();
                4
            },
            // LD DE, D16
            0x11 => {
                println!("LD DE, D16");
                let word = self.fetchword();
                self.registers.set_de(word);
                12
            },
            // LD (DE), A
            0x12 => {
                println!("LD (DE), A");
                self.mmu.write_byte(
                    self.registers.get_de(),
                    self.registers.a
                );
                8
            },
            // INC DE
            0x13 => {
                println!("INC DE");
                self.registers.set_de(
                    self.registers.get_de().wrapping_add(1)
                );
                8
            },
            // INC D
            0x14 => {
                println!("INC D");
                self.registers.d = self.inc(self.registers.d);
                4
            },
            // DEC D
            0x15 => {
                println!("DEC D");
                self.registers.d = self.dec(self.registers.d);
                4
            },
            // LD D, d8
            0x16 => {
                println!("LD D, d8");
                self.registers.d = self.fetchbyte();
                8
            },
            // RLA
            0x17 => {
                println!("RLA");
                self.registers.a = self.rl(self.registers.a);
                self.registers.set_zero(false);
                4
            },
            // JR r8
            0x18 => {
                println!("JR r8");
                self.jr();
                12
            },
            // ADD HL, DE
            0x19 => {
                println!("ADD HL, DE");
                self.addhl(
                    self.registers.get_de()
                );
                8
               },
            // LD A, (DE)
            0x1A => {
                println!("LD A, (DE)");
                self.registers.a = self.mmu.read_byte(
                    self.registers.get_de()
                );
                8
            },
            // DEC DE
            0x1B => {
                println!("DEC DE");
                self.registers.set_de(
                    self.registers.get_de().wrapping_sub(1)
                );
                8
            },
            // INC E
            0x1C => {
                println!("INC E");
                self.registers.e = self.inc(self.registers.e);
                4
            },
            // DEC E
            0x1D => {
                println!("DEC E");
                self.registers.e = self.dec(self.registers.e);
                4
            },
            // LD D, d8
            0x1E => {
                println!("LD D, d8");
                self.registers.d = self.fetchbyte();
                8
            },
            // RRA
            0x1F => {
                println!("RRA");
                self.registers.a = self.rr(self.registers.a);
                self.registers.set_zero(false);
                4
            },
            // JR NZ, r8
            0x20 => {
                println!("JR NZ, r8");
                if !self.registers.get_zero() {
                    self.jr();
                    12
                } else {
                    self.registers.pc += 1;
                    8
                }
            },
            // LD HL, d16
            0x21 => {
                println!("LD HL, d16");
                let word = self.fetchword();
                self.registers.set_hl(word);
                12
            },
            // LD (HL+), A
            0x22 => {
                println!("LD (HL+), A");
                self.mmu.write_byte(
                    self.registers.get_hli(),
                    self.registers.a
                );
                8
            },
            // INC HL
            0x23 => {
                println!("INC HL");
                self.registers.set_hl(
                    self.registers.get_hl().wrapping_add(1)
                );
                8
            },
            // INC H
            0x24 => {
                println!("INC H");
                self.registers.h = self.inc(self.registers.h);
                4
            },
            // DEC H
            0x25 => {
                println!("DEC H");
                self.registers.h = self.dec(self.registers.h);
                4
            },
            // LD H, d8
            0x26 => {
                println!("LD H, d8");
                self.registers.h = self.fetchbyte();
                8
            },
            // DAA
            0x27 => {
                println!("DAA");
                self.daa();
                4
            },
            // JR Z, r8
            0x28 => {
                println!("JR Z, r8");
                if self.registers.get_zero() {
                    self.jr();
                    12
                } else {
                    self.registers.pc += 1;
                    8
                }
            },
            // ADD HL, HL
            0x29 => {
                println!("ADD HL, HL");
                self.addhl(self.registers.get_hl());
                8
            },
            // LD A, (HL+)
            0x2A => {
                println!("LD A, (HL+)");
                self.mmu.write_byte(
                    self.registers.get_hli(),
                    self.registers.a
                );
                8
            },
            // DEC HL
            0x2B => {
                println!("DEC HL");
                self.registers.set_hl(
                    self.registers.get_hl().wrapping_sub(1)
                );
                8
            },
            // INC L
            0x2C => {
                println!("INC L");
                self.registers.l = self.inc(self.registers.l);
                4
            },
            // DEC L
            0x2D => {
                println!("DEC L");
                self.registers.l = self.dec(self.registers.l);
                4
            },
            // LD L, d8
            0x2E => {
                println!("LD L, d8");
                self.registers.l = self.fetchbyte();
                8
            },
            // CPL
            0x2F => {
                println!("CPL");
                self.registers.a = self.registers.a;
                self.registers.set_half(true);
                self.registers.set_sub(true);
                4
            },
            // JR NC, r8
            0x30 => {
                println!("JR NC, r8");
                if !self.registers.get_carry() {
                    self.jr();
                    12
                } else {
                    self.registers.pc += 1;
                    8
                }
            },
            // LD SP, d16
            0x31 => {
                println!("LD SP, d16");
                self.registers.sp = self.fetchword();
                12
            },
            // LD (HL-), A
            0x32 => {
                println!("LD (HL-), A");
                self.mmu.write_byte(
                    self.registers.get_hld(),
                    self.registers.a
                );
                8
            },
            // INC SP
            0x33 => {
                println!("INC SP");
                self.registers.sp = self.registers.sp.wrapping_add(1);
                8
            },
            // INC (HL)
            0x34 => {
                println!("INC (HL)");
                let value = self.inc(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                12
            },
            // DEC (HL)
            0x35 => {
                println!("DEC (HL)");
                let value = self.dec(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                12
            },
            // LD (HL), d8
            0x36 => {
                println!("LD (HL), d8");
                let word = self.fetchbyte();
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    word
                );
                12
            },
            // SCF
            0x37 => {
                println!("SCF");
                self.registers.set_carry(true);
                self.registers.set_half(false);
                self.registers.set_sub(true);
                4
            },
            // JR C, r8
            0x38 => {
                println!("JR C, r8");
                if self.registers.get_carry() {
                    self.jr();
                    12
                } else {
                    self.registers.pc += 1;
                    8
                }
            },
            // ADD HL, SP
            0x39 => {
                println!("ADD HL, SP");
                self.addhl(
                    self.registers.sp
                );
                8
            },
            // LD A, (HL-)
            0x3A => {
                println!("LD A, (HL-)");
                self.registers.a = self.mmu.read_byte(
                    self.registers.get_hld()
                );
                8
            },
            // DEC SP
            0x3B => {
                println!("DEC SP");
                self.registers.sp = self.registers.sp.wrapping_sub(1);
                8
            },
            // INC A
            0x3C => {
                println!("INC A");
                self.registers.a = self.inc(
                    self.registers.a
                );
                4
            },
            // DEC A
            0x3D => {
                println!("DEC A");
                self.registers.a = self.dec(
                    self.registers.a
                );
                4
            },
            // LD A, d8
            0x3E => {
                println!("LD A, d8");
                self.registers.a = self.fetchbyte();
                8
            },
            // CCF
            0x3F => {
                println!("CCF");
                self.registers.set_carry(
                    !self.registers.get_carry()
                );
                self.registers.set_half(
                    false
                );
                self.registers.set_sub(
                    false
                );
                4
            },
            // LD B, B
            0x40 => {
                println!("LD B, B");
                self.registers.b = self.registers.b;
                4
            },
            // LD B, C
            0x41 => {
                println!("LD B, C");
                self.registers.b = self.registers.c;
                4
            },
            // LD B, D
            0x42 => {
                println!("LD B, D");
                self.registers.b = self.registers.d;
                4
            },
            // LD B, E
            0x43 => {
                println!("LD B, E");
                self.registers.b = self.registers.e;
                4
            },
            // LD B, H
            0x44 => {
                println!("LD B, H");
                self.registers.b = self.registers.h;
                4
            },
            // LD B, L
            0x45 => {
                println!("LD B, L");
                self.registers.b = self.registers.l;
                4
            },
            // LD B, (HL)
            0x46 => {
                println!("LD B, (HL)");
                self.registers.b = self.mmu.read_byte(
                    self.registers.get_hl()
                );
                8
            },
            // LD B, A
            0x47 => {
                println!("LD B, A");
                self.registers.b = self.registers.a;
                4
            },
            // LD C, B
            0x48 => {
                println!("LD C, B");
                self.registers.c = self.registers.b;
                4
            },
            // LC C, C
            0x49 => {
                println!("LC C, C");
                self.registers.c = self.registers.c;
                4
            },
            // LC C, D
            0x4A => {
                println!("LC C, D");
                self.registers.c = self.registers.d;
                4
            },
            // LD C, E
            0x4B => {
                println!("LD C, E");
                self.registers.c = self.registers.e;
                4
            },
            // LD C, H
            0x4C => {
                println!("LD C, H");
                self.registers.c = self.registers.h;
                4
            },
            // LD C, L
            0x4D => {
                println!("LD C, L");
                self.registers.c = self.registers.l;
                4
            },
            // LD C, (HL)
            0x4E => {
                println!("LD C, (HL)");
                self.registers.c = self.mmu.read_byte(
                    self.registers.get_hl()
                );
                8
            },
            // LD C, A
            0x4F => {
                println!("LD C, A");
                self.registers.c = self.registers.a;
                4
            },
            // LD D, B
            0x50 => {
                println!("LD D, B");
                self.registers.d = self.registers.b;
                4
            },
            // LD D, C
            0x51 => {
                println!("LD D, C");
                self.registers.d = self.registers.c;
                4
            },
            // LD D, D
            0x52 => {
                println!("LD D, D");
                self.registers.d = self.registers.d;
                4
            },
            // LD D, E
            0x53 => {
                println!("LD D, E");
                self.registers.d = self.registers.e;
                4
            },
            // LD D, H
            0x54 => {
                println!("LD D, H");
                self.registers.d = self.registers.h;
                4
            },
            // LD D, L
            0x55 => {
                println!("LD D, L");
                self.registers.d = self.registers.l;
                4
            },
            // LD D, (HL)
            0x56 => {
                println!("LD D, (HL)");
                self.registers.d = self.mmu.read_byte(
                    self.registers.get_hl()
                );
                8
            },
            // LD D, A
            0x57 => {
                println!("LD D, A");
                self.registers.d = self.registers.a;
                4
            },
            // LD E, B
            0x58 => {
                println!("LD E, B");
                self.registers.e = self.registers.b;
                4
            },
            // LC E, C
            0x59 => {
                println!("LC E, C");
                self.registers.e = self.registers.c;
                4
            },
            // LC E, D
            0x5A => {
                println!("LC E, D");
                self.registers.e = self.registers.d;
                4
            },
            // LD E, E
            0x5B => {
                println!("LD E, E");
                self.registers.e = self.registers.e;
                4
            },
            // LD E, H
            0x5C => {
                println!("LD E, H");
                self.registers.e = self.registers.h;
                4
            },
            // LD E, L
            0x5D => {
                println!("LD E, L");
                self.registers.e = self.registers.l;
                4
            },
            // LD E, (HL)
            0x5E => {
                println!("LD E, (HL)");
                self.registers.e = self.mmu.read_byte(
                    self.registers.get_hl()
                );
                8
            },
            // LD E, A
            0x5F => {
                println!("LD E, A");
                self.registers.e = self.registers.a;
                4
            },
            // LD H, B
            0x60 => {
                println!("LD H, B");
                self.registers.h = self.registers.b;
                4
            },
            // LD H, C
            0x61 => {
                println!("LD H, C");
                self.registers.h = self.registers.c;
                4
            },
            // LD H, D
            0x62 => {
                println!("LD H, D");
                self.registers.h = self.registers.d;
                4
            },
            // LD H, E
            0x63 => {
                println!("LD H, E");
                self.registers.h = self.registers.e;
                4
            },
            // LD H, H
            0x64 => {
                println!("LD H, H");
                self.registers.h = self.registers.h;
                4
            },
            // LD H, L
            0x65 => {
                println!("LD H, L");
                self.registers.h = self.registers.l;
                4
            },
            // LD H, (HL)
            0x66 => {
                println!("LD H, (HL)");
                self.registers.h = self.mmu.read_byte(
                    self.registers.get_hl()
                );
                8
            },
            // LD H, A
            0x67 => {
                println!("LD H, A");
                self.registers.h = self.registers.a;
                4
            },
            // LD L, B
            0x68 => {
                println!("LD L, B");
                self.registers.l = self.registers.b;
                4
            },
            // LC L, C
            0x69 => {
                println!("LC L, C");
                self.registers.l = self.registers.c;
                4
            },
            // LC L, D
            0x6A => {
                println!("LC L, D");
                self.registers.l = self.registers.d;
                4
            },
            // LD L, E
            0x6B => {
                println!("LD L, E");
                self.registers.l = self.registers.e;
                4
            },
            // LD L, H
            0x6C => {
                println!("LD L, H");
                self.registers.l = self.registers.h;
                4
            },
            // LD L, L
            0x6D => {
                println!("LD L, L");
                self.registers.l = self.registers.l;
                4
            },
            // LD L, (HL)
            0x6E => {
                println!("LD L, (HL)");
                self.registers.l = self.mmu.read_byte(
                    self.registers.get_hl()
                );
                8
            },
            // LD L, A
            0x6F => {
                println!("LD L, A");
                self.registers.l = self.registers.a;
                4
            },
            // LD (HL), B
            0x70 => {
                println!("LD (HL), B");
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    self.registers.b
                );
                8
            },
            // LD (HL), C
            0x71 => {
                println!("LD (HL), C");
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    self.registers.c
                );
                8
            },
            // LD (HL), D
            0x72 => {
                println!("LD (HL), D");
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    self.registers.d
                );
                8
            },
            // LD (HL), E
            0x73 => {
                println!("LD (HL), E");
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    self.registers.e
                );
                8
            },
            // LD (HL), H
            0x74 => {
                println!("LD (HL), H");
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    self.registers.h
                );
                8
            },
            // LD (HL), L
            0x75 => {
                println!("LD (HL), L");
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    self.registers.l
                );
                8
            },
            // HALT
            0x76 => {
                println!("HALT");
                self.halt();
                4
            },
            // LD (HL), A
            0x77 => {
                println!("LD (HL), A");
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    self.registers.a
                );
                8
            },
            // LD A, B
            0x78 => {
                println!("LD A, B");
                self.registers.a = self.registers.b;
                4
            },
            // LC A, C
            0x79 => {
                println!("LC A, C");
                self.registers.a = self.registers.c;
                4
            },
            // LC A, D
            0x7A => {
                println!("LC A, D");
                self.registers.a = self.registers.d;
                4
            },
            // LD A, E
            0x7B => {
                println!("LD A, E");
                self.registers.a = self.registers.e;
                4
            },
            // LD A, H
            0x7C => {
                println!("LD A, H");
                self.registers.a = self.registers.h;
                4
            },
            // LD A, L
            0x7D => {
                println!("LD A, L");
                self.registers.a = self.registers.l;
                4
            },
            // LD A, (HL)
            0x7E => {
                println!("LD A, (HL)");
                self.registers.a = self.mmu.read_byte(
                    self.registers.get_hl()
                );
                8
            },
            // LD A, A
            0x7F => {
                println!("LD A, A");
                self.registers.a = self.registers.a;
                4
            },
            // ADD A, B
            0x80 => {
                println!("ADD A, B");
                self.add(
                    self.registers.b
                );
                4
            },
            // ADD A, C
            0x81 => {
                println!("ADD A, C");
                self.add(
                    self.registers.c
                );
                4
            },
            // ADD A, D
            0x82 => {
                println!("ADD A, D");
                self.add(
                    self.registers.d
                );
                4
            },
            // ADD A, E
            0x83 => {
                println!("ADD A, E");
                self.add(
                    self.registers.e
                );
                4
            },
            // ADD A, H
            0x84 => {
                println!("ADD A, H");
                self.add(
                    self.registers.h
                );
                4
            },
            // ADD A, L
            0x85 => {
                println!("ADD A, L");
                self.add(
                    self.registers.l
                );
                4
            },
            // ADD A, (HL)
            0x86 => {
                println!("ADD A, (HL)");
                self.add(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                8
            },
            // ADD A, A
            0x87 => {
                println!("ADD A, A");
                self.add(
                    self.registers.a
                );
                4
            },
            // ADC A, B
            0x88 => {
                println!("ADC A, B");
                self.adc(
                    self.registers.b
                );
                4
            },
            // ADC A, C
            0x89 => {
                println!("ADC A, C");
                self.adc(
                    self.registers.c
                );
                4
            },
            // ADC A, D
            0x8A => {
                println!("ADC A, D");
                self.adc(
                    self.registers.d
                );
                4
            },
            // ADC A, E
            0x8B => {
                println!("ADC A, E");
                self.adc(
                    self.registers.e
                );
                4
            },
            // ADC A, H
            0x8C => {
                println!("ADC A, H");
                self.adc(
                    self.registers.h
                );
                4
            },
            // ADC A, L
            0x8D => {
                println!("ADC A, L");
                self.adc(
                    self.registers.l
                );
                4
            },
            // ADC A, (HL)
            0x8E => {
                println!("ADC A, (HL)");
                self.adc(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                8
            },
            // ADC A, A
            0x8F => {
                println!("ADC A, A");
                self.adc(
                    self.registers.a
                );
                4
            },
            // SUB A, B
            0x90 => {
                println!("SUB A, B");
                self.sub(
                    self.registers.b
                );
                4
            },
            // SUB A, C
            0x91 => {
                println!("SUB A, C");
                self.sub(
                    self.registers.c
                );
                4
            },
            // SUB A, D
            0x92 => {
                println!("SUB A, D");
                self.sub(
                    self.registers.d
                );
                4
            },
            // SUB A, E
            0x93 => {
                println!("SUB A, E");
                self.sub(
                    self.registers.e
                );
                4
            },
            // SUB A, H
            0x94 => {
                println!("SUB A, H");
                self.sub(
                    self.registers.h
                );
                4
            },
            // SUB A, L
            0x95 => {
                println!("SUB A, L");
                self.sub(
                    self.registers.l
                );
                4
            },
            // SUB A, (HL)
            0x96 => {
                println!("SUB A, (HL)");
                self.sub(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                8
            },
            // SUB A, A
            0x97 => {
                println!("SUB A, A");
                self.sub(
                    self.registers.a
                );
                4
            },
            // SBC A, B
            0x98 => {
                println!("SBC A, B");
                self.sbc(
                    self.registers.b
                );
                4
            },
            // SBC A, C
            0x99 => {
                println!("SBC A, C");
                self.sbc(
                    self.registers.c
                );
                4
            },
            // SBC A, D
            0x9A => {
                println!("SBC A, D");
                self.sbc(
                    self.registers.d
                );
                4
            },
            // SBC A, E
            0x9B => {
                println!("SBC A, E");
                self.sbc(
                    self.registers.e
                );
                4
            },
            // SBC A, H
            0x9C => {
                println!("SBC A, H");
                self.sbc(
                    self.registers.h
                );
                4
            },
            // SBC A, L
            0x9D => {
                println!("SBC A, L");
                self.sbc(
                    self.registers.l
                );
                4
            },
            // SBC A, (HL)
            0x9E => {
                println!("SBC A, (HL)");
                self.sbc(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                8
            },
            // SBC A, A
            0x9F => {
                println!("SBC A, A");
                self.sbc(
                    self.registers.a
                );
                4
            },
            // AND A, B
            0xA0 => {
                println!("AND A, B");
                self.and(
                    self.registers.b
                );
                4
            },
            // AND A, C
            0xA1 => {
                println!("AND A, C");
                self.and(
                    self.registers.c
                );
                4
            },
            // AND A, D
            0xA2 => {
                println!("AND A, D");
                self.and(
                    self.registers.d
                );
                4
            },
            // AND A, E
            0xA3 => {
                println!("AND A, E");
                self.and(
                    self.registers.e
                );
                4
            },
            // AND A, H
            0xA4 => {
                println!("AND A, H");
                self.and(
                    self.registers.h
                );
                4
            },
            // AND A, L
            0xA5 => {
                println!("AND A, L");
                self.and(
                    self.registers.l
                );
                4
            },
            // AND A, (HL)
            0xA6 => {
                println!("AND A, (HL)");
                self.and(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                8
            },
            // AND A, A
            0xA7 => {
                println!("AND A, A");
                self.and(
                    self.registers.a
                );
                4
            },
            // XOR A, B
            0xA8 => {
                println!("XOR A, B");
                self.xor(
                    self.registers.b
                );
                4
            },
            // XOR A, C
            0xA9 => {
                println!("XOR A, C");
                self.xor(
                    self.registers.c
                );
                4
            },
            // XOR A, D
            0xAA => {
                println!("XOR A, D");
                self.xor(
                    self.registers.d
                );
                4
            },
            // XOR A, E
            0xAB => {
                println!("XOR A, E");
                self.xor(
                    self.registers.e
                );
                4
            },
            // XOR A, H
            0xAC => {
                println!("XOR A, H");
                self.xor(
                    self.registers.h
                );
                4
            },
            // XOR A, L
            0xAD => {
                println!("XOR A, L");
                self.xor(
                    self.registers.l
                );
                4
            },
            // XOR A, (HL)
            0xAE => {
                println!("XOR A, (HL)");
                self.xor(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                8
            },
            // XOR A, A
            0xAF => {
                println!("XOR A, A");
                self.xor(
                    self.registers.a
                );
                4
            },
            // OR A, B
            0xB0 => {
                println!("OR A, B");
                self.or(
                    self.registers.b
                );
                4
            },
            // OR A, C
            0xB1 => {
                println!("OR A, C");
                self.or(
                    self.registers.c
                );
                4
            },
            // OR A, D
            0xB2 => {
                println!("OR A, D");
                self.or(
                    self.registers.d
                );
                4
            },
            // OR A, E
            0xB3 => {
                println!("OR A, E");
                self.or(
                    self.registers.e
                );
                4
            },
            // OR A, H
            0xB4 => {
                println!("OR A, H");
                self.or(
                    self.registers.h
                );
                4
            },
            // OR A, L
            0xB5 => {
                println!("OR A, L");
                self.or(
                    self.registers.l
                );
                4
            },
            // OR A, (HL)
            0xB6 => {
                println!("OR A, (HL)");
                self.or(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                8
            },
            // OR A, A
            0xB7 => {
                println!("OR A, A");
                self.or(
                    self.registers.a
                );
                4
            },
            // CP A, B
            0xB8 => {
                println!("CP A, B");
                self.cp(
                    self.registers.b
                );
                4
            },
            // CP A, C
            0xB9 => {
                println!("CP A, C");
                self.cp(
                    self.registers.c
                );
                4
            },
            // CP A, D
            0xBA => {
                println!("CP A, D");
                self.cp(
                    self.registers.d
                );
                4
            },
            // CP A, E
            0xBB => {
                println!("CP A, E");
                self.cp(
                    self.registers.e
                );
                4
            },
            // CP A, H
            0xBC => {
                println!("CP A, H");
                self.cp(
                    self.registers.h
                );
                4
            },
            // CP A, L
            0xBD => {
                println!("CP A, L");
                self.cp(
                    self.registers.l
                );
                4
            },
            // CP A, (HL)
            0xBE => {
                println!("CP A, (HL)");
                self.cp(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                8
            },
            // CP A, A
            0xBF => {
                println!("CP A, A");
                self.cp(
                    self.registers.a
                );
                4
            },
            // RET NZ
            0xC0 => {
                println!("RET NZ");
                if !self.registers.get_zero() {
                    self.registers.pc = self.fetchword();
                    20
                } else {
                    8
                }
            },
            // POP BC
            0xC1 => {
                println!("POP BC");
                let value = self.pop();
                self.registers.set_bc(value);
                12
            },
            // JP NZ, a16
            0xC2 => {
                println!("JP NZ, a16");
                if !self.registers.get_zero() {
                    self.registers.pc = self.fetchword();
                    16
                } else {
                    self.registers.pc += 2;
                    12
                }
            },
            // JP a16
            0xC3 => {
                println!("JP a16");
                self.registers.pc = self.fetchword();
                16
            },
            // CALL NZ, a16
            0xC4 => {
                println!("CALL NZ, a16");
                if !self.registers.get_zero() {
                    self.push(
                        self.registers.pc + 2
                    );
                    self.registers.pc = self.fetchword();
                    24
                } else {
                    self.registers.pc += 2;
                    12
                }
            },
            // PUSH BC
            0xC5 => {
                println!("PUSH BC");
                self.push(
                    self.registers.get_bc()
                );
                16
            },
            // ADD A, d8
            0xC6 => {
                println!("ADD A, d8");
                let value = self.fetchbyte();
                self.add(value);
                8
            },
            // RST 00H
            0xC7 => {
                println!("RST 00H");
                self.rst(0x0000);
                16
            },
            // RET Z
            0xC8 => {
                println!("RET Z");
                if self.registers.get_zero() {
                    self.registers.pc = self.fetchword();
                    20
                } else {
                    8
                }
            },
            // RET
            0xC9 => {
                println!("RET");
                self.registers.pc = self.fetchword();
                16
            },
            // JP Z, a16
            0xCA => {
                println!("JP Z, a16");
                if self.registers.get_zero() {
                    self.registers.pc = self.fetchword();
                    16
                } else {
                    self.registers.pc += 2;
                    12
                }
            },
            // PREFIX CB
            0xCB => {
                println!("PREFIX CB");
                self.call_cb()
            },
            // CALL Z, a16
            0xCC => {
                println!("CALL Z, a16");
                if self.registers.get_zero() {
                    self.push(
                        self.registers.pc + 2
                    );
                    self.registers.pc = self.fetchword();
                    24
                } else {
                    self.registers.pc += 2;
                    12
                }
            },
            // CALL a16
            0xCD => {
                println!("CALL a16");
                self.push(
                    self.registers.pc + 2
                );
                self.registers.pc = self.fetchword();
                24
            },
            // ADC A, d8
            0xCE => {
                println!("ADC A, d8");
                let value = self.fetchbyte();
                self.adc(value);
                8
            },
            // RST 08H
            0xCF => {
                println!("RST 08H");
                self.rst(0x0080);
                16
            },
            // RET NC
            0xD0 => {
                println!("RET NC");
                if !self.registers.get_carry() {
                    self.registers.pc = self.fetchword();
                    20
                } else {
                    8
                }
            },
            // POP DE
            0xD1 => {
                println!("POP DE");
                let value = self.pop();
                self.registers.set_de(value);
                12
            },
            // JP NC, a16
            0xD2 => {
                println!("JP NC, a16");
                if !self.registers.get_carry() {
                    self.registers.pc = self.fetchword();
                    16
                } else {
                    self.registers.pc += 2;
                    12
                }
            },
            // CALL NC, a16
            0xD4 => {
                println!("CALL NC, a16");
                if !self.registers.get_carry() {
                    self.push(
                        self.registers.pc + 2
                    );
                    self.registers.pc = self.fetchword();
                    24
                } else {
                    self.registers.pc += 2;
                    12
                }
            },
            // PUSH DE
            0xD5 => {
                println!("PUSH DE");
                self.push(
                    self.registers.get_de()
                );
                16
            },
            // SUB d8
            0xD6 => {
                println!("SUB d8");
                let value = self.fetchbyte();
                self.sub(value);
                8
            },
            // RST 10H
            0xD7 => {
                println!("RST 10H");
                self.rst(0x0010);
                16
            },
            // RET C
            0xD8 => {
                println!("RET C");
                if self.registers.get_carry() {
                    self.registers.pc = self.fetchword();
                    20
                } else {
                    8
                }
            },
            // RETI
            0xD9 => {
                println!("RETI");
                self.registers.pc = self.pop();
                self.ei = 1;
                self.di = 0;
                16
            },
            // JP C, a16
            0xDA => {
                println!("JP C, a16");
                if self.registers.get_carry() {
                    self.registers.pc = self.fetchword();
                    16
                } else {
                    self.registers.pc += 2;
                    12
                }
            },
            // CALL C, a16
            0xDC => {
                println!("CALL C, a16");
                if self.registers.get_carry() {
                    self.push(
                        self.registers.pc + 2
                    );
                    self.registers.pc = self.fetchword();
                    24
                } else {
                    self.registers.pc += 2;
                    12
                }
            },
            // SBC A, d8
            0xDE => {
                println!("SBC A, d8");
                let value = self.fetchbyte();
                self.sbc(value);
                8
            },
            // RST 18H
            0xDF => {
                println!("RST 18H");
                self.rst(0x0018);
                16
            },
            // LDH (a8), A
            0xE0 => {
                println!("LDH (a8), A");
                let value = self.fetchbyte();
                self.mmu.write_byte(
                    0xFF00 | value as u16,
                    self.registers.a
                );
                12
            },
            // POP HL
            0xE1 => {
                println!("POP HL");
                let value = self.pop();
                self.registers.set_hl(value);
                12
            },
            // LDH (C), A
            0xE2 => {
                println!("LDH (C), A");
                self.mmu.write_byte(
                    0xFF00 | self.registers.c as u16,
                    self.registers.a
                );
                8
            },
            // PUSH HL
            0xE5 => {
                println!("PUSH HL");
                self.push(
                    self.registers.get_hl()
                );
                16
            },
            // AND d8
            0xE6 => {
                println!("AND d8");
                let value = self.fetchbyte();
                self.and(value);
                8
            },
            // RST 20H
            0xE7 => {
                println!("RST 20H");
                self.rst(0x0020);
                16
            },
            // ADD SP, r8
            0xE8 => {
                println!("ADD SP, r8");
                self.registers.sp = self.addr8(self.registers.sp);
                16
            },
            // JP (HL)
            0xE9 => {
                println!("JP (HL)");
                self.registers.pc = self.registers.get_hl();
                4
            },
            // LD (a16), A
            0xEA => {
                println!("LD (a16), A");
                let value = self.fetchword();
                self.mmu.write_byte(
                    value,
                    self.registers.a
                );
                16
            },
            // XOR d8
            0xEE => {
                println!("XOR d8");
                let value = self.fetchbyte();
                self.xor(value);
                8
            },
            // RST 28H
            0xEF => {
                println!("RST 28H");
                self.rst(0x0028);
                16
            },
            // LDH A, (a8)
            0xF0 => {
                println!("LDH A, (a8)");
                let value = self.fetchbyte();
                self.registers.a = self.mmu.read_byte(
                    0xFF00 | value as u16
                );
                12
            },
            // POP AF
            0xF1 => {
                println!("POP AF");
                let value = self.pop();
                self.registers.set_af(
                    value & 0xFFF0
                );
                12
            },
            // LD A, (C)
            0xF2 => {
                println!("LD A, (C)");
                self.registers.a = self.mmu.read_byte(
                    0xFF00 | self.registers.c as u16
                );
                8
            },
            // DI
            0xF3 => {
                println!("DI");
                self.di = 2;
                // Cancel any scheduled effects of the ei instruction
                self.ei = 0;
                4
            },
            // PUSH AF
            0xF5 => {
                println!("PUSH AF");
                self.push(
                    self.registers.get_af()
                );
                16
            },
            // OR d8
            0xF6 => {
                println!("OR d8");
                let value = self.fetchbyte();
                self.or(value);
                8
            },
            // RST 30H
            0xF7 => {
                println!("RST 30H");
                self.rst(0x0030);
                16
            },
            // LD HL, SP+r8
            0xF8 => {
                println!("LD HL, SP+r8");
                let value = self.addr8(self.registers.sp);
                self.registers.set_hl(value);
                12
            },
            // LD SP, HL
            0xF9 => {
                println!("LD SP, HL");
                self.registers.sp = self.registers.get_hl();
                8
            },
            // LD A, (a16)
            0xFA => {
                println!("LD A, (a16)");
                let value = self.fetchword();
                self.registers.a = self.mmu.read_byte(value);
                12
            },
            // EI
            0xFB => {
                println!("EI");
                self.ei = 2;
                4
            },
            // CP d8
            0xFE => {
                println!("CP d8");
                let value = self.fetchbyte();
                self.cp(value);
                8
            },
            // RST 38H
            0xFF => {
                println!("RST 38H");
                self.rst(0x0038);
                16
            },
            // Si code non trouvé
            _ => {
                println!("Si code non trouvé");
                panic!("OpCode not found");
            }
        }
    }

    /// Reads an instruction and execute it from the CB table
    ///
    /// <https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html>
    ///
    /// # Returns
    /// **u32**: Number of cycles used for the step
    ///
    /// # Examples
    /// ```rust
    /// let new_cpu = CPU::new("test.gb");
    /// new_cpu.call_cb();
    /// ```
    fn call_cb(&mut self) -> u32 {
        let op = self.fetchbyte();
        match op {
            // RLC B
            0x00 => {
                println!("RLC B");
                self.registers.b = self.rlc(
                    self.registers.b
                );
                8
            },
            // RLC C
            0x01 => {
                println!("RLC C");
                self.registers.c = self.rlc(
                    self.registers.c
                );
                8
            },
            // RLC D
            0x02 => {
                println!("RLC D");
                self.registers.d = self.rlc(
                    self.registers.d
                );
                8
            },
            // RLC E
            0x03 => {
                println!("RLC E");
                self.registers.e = self.rlc(
                    self.registers.e
                );
                8
            },
            // RLC H
            0x04 => {
                println!("RLC H");
                self.registers.h = self.rlc(
                    self.registers.h
                );
                8
            },
            // RLC L
            0x05 => {
                println!("RLC L");
                self.registers.l = self.rlc(
                    self.registers.l
                );
                8
            },
            // RLC (HL)
            0x06 => {
                println!("RLC (HL)");
                let value = self.rlc(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // RLC A
            0x07 => {
                println!("RLC A");
                self.registers.a = self.rlc(
                    self.registers.a
                );
                8
            },
            // RRC B
            0x08 => {
                println!("RRC B");
                self.registers.b = self.rrc(
                    self.registers.b
                );
                8
            },
            // RRC C
            0x09 => {
                println!("RRC C");
                self.registers.c = self.rrc(
                    self.registers.c
                );
                8
            },
            // RRC D
            0x0A => {
                println!("RRC D");
                self.registers.d = self.rrc(
                    self.registers.d
                );
                8
            },
            // RRC E
            0x0B => {
                println!("RRC E");
                self.registers.e = self.rrc(
                    self.registers.e
                );
                8
            },
            // RRC H
            0x0C => {
                println!("RRC H");
                self.registers.h = self.rrc(
                    self.registers.h
                );
                8
            },
            // RRC L
            0x0D => {
                println!("RRC L");
                self.registers.l = self.rrc(
                    self.registers.l
                );
                8
            },
            // RRC (HL)
            0x0E => {
                println!("RRC (HL)");
                let value = self.rrc(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // RRC A
            0x0F => {
                println!("RRC A");
                self.registers.a = self.rrc(
                    self.registers.a
                );
                8
            },
            // RL B
            0x10 => {
                println!("RL B");
                self.registers.b = self.rl(
                    self.registers.b
                );
                8
            },
            // RL C
            0x11 => {
                println!("RL C");
                self.registers.c = self.rl(
                    self.registers.c
                );
                8
            },
            // RL D
            0x12 => {
                println!("RL D");
                self.registers.d = self.rl(
                    self.registers.d
                );
                8
            },
            // RC E
            0x13 => {
                println!("RC E");
                self.registers.e = self.rl(
                    self.registers.e
                );
                8
            },
            // RC H
            0x14 => {
                println!("RC H");
                self.registers.h = self.rl(
                    self.registers.h
                );
                8
            },
            // RL L
            0x15 => {
                println!("RL L");
                self.registers.l = self.rl(
                    self.registers.l
                );
                8
            },
            // RL (HL)
            0x16 => {
                println!("RL (HL)");
                let value = self.rl(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // RL A
            0x17 => {
                println!("RL A");
                self.registers.a = self.rl(
                    self.registers.a
                );
                8
            },
            // RR B
            0x18 => {
                println!("RR B");
                self.registers.b = self.rr(
                    self.registers.b
                );
                8
            },
            // RR C
            0x19 => {
                println!("RR C");
                self.registers.c = self.rr(
                    self.registers.c
                );
                8
            },
            // RR D
            0x1A => {
                println!("RR D");
                self.registers.d = self.rr(
                    self.registers.d
                );
                8
            },
            // RR E
            0x1B => {
                println!("RR E");
                self.registers.e = self.rr(
                    self.registers.e
                );
                8
            },
            // RR H
            0x1C => {
                println!("RR H");
                self.registers.h = self.rr(
                    self.registers.h
                );
                8
            },
            // RR L
            0x1D => {
                println!("RR L");
                self.registers.l = self.rr(
                    self.registers.l
                );
                8
            },
            // RR (HL)
            0x1E => {
                println!("RR (HL)");
                let value = self.rr(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // RR A
            0x1F => {
                println!("RR A");
                self.registers.a = self.rr(
                    self.registers.a
                );
                8
            },
            // SLA B
            0x20 => {
                println!("SLA B");
                self.registers.b = self.sla(
                    self.registers.b
                );
                8
            },
            // SLA C
            0x21 => {
                println!("SLA C");
                self.registers.c = self.sla(
                    self.registers.c
                );
                8
            },
            // SLA D
            0x22 => {
                println!("SLA D");
                self.registers.d = self.sla(
                    self.registers.d
                );
                8
            },
            // RC E
            0x23 => {
                println!("RC E");
                self.registers.e = self.sla(
                    self.registers.e
                );
                8
            },
            // RC H
            0x24 => {
                println!("RC H");
                self.registers.h = self.sla(
                    self.registers.h
                );
                8
            },
            // SLA L
            0x25 => {
                println!("SLA L");
                self.registers.l = self.sla(
                    self.registers.l
                );
                8
            },
            // SLA (HL)
            0x26 => {
                println!("SLA (HL)");
                let value = self.sla(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // SLA A
            0x27 => {
                println!("SLA A");
                self.registers.a = self.sla(
                    self.registers.a
                );
                8
            },
            // SRA B
            0x28 => {
                println!("SRA B");
                self.registers.b = self.sra(
                    self.registers.b
                );
                8
            },
            // SRA C
            0x29 => {
                println!("SRA C");
                self.registers.c = self.sra(
                    self.registers.c
                );
                8
            },
            // SRA D
            0x2A => {
                println!("SRA D");
                self.registers.d = self.sra(
                    self.registers.d
                );
                8
            },
            // SRA E
            0x2B => {
                println!("SRA E");
                self.registers.e = self.sra(
                    self.registers.e
                );
                8
            },
            // SRA H
            0x2C => {
                println!("SRA H");
                self.registers.h = self.sra(
                    self.registers.h
                );
                8
            },
            // SRA L
            0x2D => {
                println!("SRA L");
                self.registers.l = self.sra(
                    self.registers.l
                );
                8
            },
            // SRA (HL)
            0x2E => {
                println!("SRA (HL)");
                let value = self.sra(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // SRA A
            0x2F => {
                println!("SRA A");
                self.registers.a = self.sra(
                    self.registers.a
                );
                8
            },
            // SWAP B
            0x30 => {
                println!("SWAP B");
                self.registers.b = self.swap(
                    self.registers.b
                );
                8
            },
            // SWAP C
            0x31 => {
                println!("SWAP C");
                self.registers.c = self.swap(
                    self.registers.c
                );
                8
            },
            // SWAP D
            0x32 => {
                println!("SWAP D");
                self.registers.d = self.swap(
                    self.registers.d
                );
                8
            },
            // RC E
            0x33 => {
                println!("RC E");
                self.registers.e = self.swap(
                    self.registers.e
                );
                8
            },
            // RC H
            0x34 => {
                println!("RC H");
                self.registers.h = self.swap(
                    self.registers.h
                );
                8
            },
            // SWAP L
            0x35 => {
                println!("SWAP L");
                self.registers.l = self.swap(
                    self.registers.l
                );
                8
            },
            // SWAP (HL)
            0x36 => {
                println!("SWAP (HL)");
                let value = self.swap(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // SWAP A
            0x37 => {
                println!("SWAP A");
                self.registers.a = self.swap(
                    self.registers.a
                );
                8
            },
            // SRL B
            0x38 => {
                println!("SRL B");
                self.registers.b = self.srl(
                    self.registers.b
                );
                8
            },
            // SRL C
            0x39 => {
                println!("SRL C");
                self.registers.c = self.srl(
                    self.registers.c
                );
                8
            },
            // SRL D
            0x3A => {
                println!("SRL D");
                self.registers.d = self.srl(
                    self.registers.d
                );
                8
            },
            // SRL E
            0x3B => {
                println!("SRL E");
                self.registers.e = self.srl(
                    self.registers.e
                );
                8
            },
            // SRL H
            0x3C => {
                println!("SRL H");
                self.registers.h = self.srl(
                    self.registers.h
                );
                8
            },
            // SRL L
            0x3D => {
                println!("SRL L");
                self.registers.l = self.srl(
                    self.registers.l
                );
                8
            },
            // SRL (HL)
            0x3E => {
                println!("SRL (HL)");
                let value = self.srl(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // SRL A
            0x3F => {
                println!("SRL A");
                self.registers.a = self.srl(
                    self.registers.a
                );
                8
            },
            // BIT 0, B
            0x40 => {
                println!("BIT 0, B");
                self.bit(0, self.registers.b);
                8
            },
            // BIT 0, C
            0x41 => {
                println!("BIT 0, C");
                self.bit(0, self.registers.c);
                8
            },
            // BIT 0, D
            0x42 => {
                println!("BIT 0, D");
                self.bit(0, self.registers.d);
                8
            },
            // BIT 0, E
            0x43 => {
                println!("BIT 0, E");
                self.bit(0, self.registers.e);
                8
            },
            // BIT 0, H
            0x44 => {
                println!("BIT 0, H");
                self.bit(0, self.registers.h);
                8
            },
            // BIT 0, L
            0x45 => {
                println!("BIT 0, L");
                self.bit(0, self.registers.l);
                8
            },
            // BIT 0, (HL)
            0x46 => {
                println!("BIT 0, (HL)");
                self.bit(
                    0,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                16
            },
            // BIT 0, A
            0x47 => {
                println!("BIT 0, A");
                self.bit(0, self.registers.a);
                8
            },
            // BIT 1, B
            0x48 => {
                println!("BIT 1, B");
                self.bit(1, self.registers.b);
                8
            },
            // BIT 1, C
            0x49 => {
                println!("BIT 1, C");
                self.bit(1, self.registers.c);
                8
            },
            // BIT 1, D
            0x4A => {
                println!("BIT 1, D");
                self.bit(1, self.registers.d);
                8
            },
            // BIT 1, E
            0x4B => {
                println!("BIT 1, E");
                self.bit(1, self.registers.e);
                8
            },
            // BIT 1, H
            0x4C => {
                println!("BIT 1, H");
                self.bit(1, self.registers.h);
                8
            },
            // BIT 1, L
            0x4D => {
                println!("BIT 1, L");
                self.bit(1, self.registers.l);
                8
            },
            // BIT 1, (HL)
            0x4E => {
                println!("BIT 1, (HL)");
                self.bit(
                    1,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                16
            },
            // BIT 1, A
            0x4F => {
                println!("BIT 1, A");
                self.bit(1, self.registers.a);
                8
            },
            // BIT 2, B
            0x50 => {
                println!("BIT 2, B");
                self.bit(2, self.registers.b);
                8
            },
            // BIT 2, C
            0x51 => {
                println!("BIT 2, C");
                self.bit(2, self.registers.c);
                8
            },
            // BIT 2, D
            0x52 => {
                println!("BIT 2, D");
                self.bit(2, self.registers.d);
                8
            },
            // BIT 2, E
            0x53 => {
                println!("BIT 2, E");
                self.bit(2, self.registers.e);
                8
            },
            // BIT 2, H
            0x54 => {
                println!("BIT 2, H");
                self.bit(2, self.registers.h);
                8
            },
            // BIT 2, L
            0x55 => {
                println!("BIT 2, L");
                self.bit(2, self.registers.l);
                8
            },
            // BIT 2, (HL)
            0x56 => {
                println!("BIT 2, (HL)");
                self.bit(
                    2,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                16
            },
            // BIT 2, A
            0x57 => {
                println!("BIT 2, A");
                self.bit(2, self.registers.a);
                8
            },
            // BIT 3, B
            0x58 => {
                println!("BIT 3, B");
                self.bit(3, self.registers.b);
                8
            },
            // BIT 3, C
            0x59 => {
                println!("BIT 3, C");
                self.bit(3, self.registers.c);
                8
            },
            // BIT 3, D
            0x5A => {
                println!("BIT 3, D");
                self.bit(3, self.registers.d);
                8
            },
            // BIT 3, E
            0x5B => {
                println!("BIT 3, E");
                self.bit(3, self.registers.e);
                8
            },
            // BIT 3, H
            0x5C => {
                println!("BIT 3, H");
                self.bit(3, self.registers.h);
                8
            },
            // BIT 3, L
            0x5D => {
                println!("BIT 3, L");
                self.bit(3, self.registers.l);
                8
            },
            // BIT 3, (HL)
            0x5E => {
                println!("BIT 3, (HL)");
                self.bit(
                    3,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                16
            },
            // BIT 3, A
            0x5F => {
                println!("BIT 3, A");
                self.bit(3, self.registers.a);
                8
            },
            // BIT 4, B
            0x60 => {
                println!("BIT 4, B");
                self.bit(4, self.registers.b);
                8
            },
            // BIT 4, C
            0x61 => {
                println!("BIT 4, C");
                self.bit(4, self.registers.c);
                8
            },
            // BIT 4, D
            0x62 => {
                println!("BIT 4, D");
                self.bit(4, self.registers.d);
                8
            },
            // BIT 4, E
            0x63 => {
                println!("BIT 4, E");
                self.bit(4, self.registers.e);
                8
            },
            // BIT 4, H
            0x64 => {
                println!("BIT 4, H");
                self.bit(4, self.registers.h);
                8
            },
            // BIT 4, L
            0x65 => {
                println!("BIT 4, L");
                self.bit(4, self.registers.l);
                8
            },
            // BIT 4, (HL)
            0x66 => {
                println!("BIT 4, (HL)");
                self.bit(
                    4,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                16
            },
            // BIT 4, A
            0x67 => {
                println!("BIT 4, A");
                self.bit(4, self.registers.a);
                8
            },
            // BIT 5, B
            0x68 => {
                println!("BIT 5, B");
                self.bit(5, self.registers.b);
                8
            },
            // BIT 5, C
            0x69 => {
                println!("BIT 5, C");
                self.bit(5, self.registers.c);
                8
            },
            // BIT 5, D
            0x6A => {
                println!("BIT 5, D");
                self.bit(5, self.registers.d);
                8
            },
            // BIT 5, E
            0x6B => {
                println!("BIT 5, E");
                self.bit(5, self.registers.e);
                8
            },
            // BIT 5, H
            0x6C => {
                println!("BIT 5, H");
                self.bit(5, self.registers.h);
                8
            },
            // BIT 5, L
            0x6D => {
                println!("BIT 5, L");
                self.bit(5, self.registers.l);
                8
            },
            // BIT 5, (HL)
            0x6E => {
                println!("BIT 5, (HL)");
                self.bit(
                    5,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                16
            },
            // BIT 5, A
            0x6F => {
                println!("BIT 5, A");
                self.bit(5, self.registers.a);
                8
            },
            // BIT 6, B
            0x70 => {
                println!("BIT 6, B");
                self.bit(6, self.registers.b);
                8
            },
            // BIT 6, C
            0x71 => {
                println!("BIT 6, C");
                self.bit(6, self.registers.c);
                8
            },
            // BIT 6, D
            0x72 => {
                println!("BIT 6, D");
                self.bit(6, self.registers.d);
                8
            },
            // BIT 6, E
            0x73 => {
                println!("BIT 6, E");
                self.bit(6, self.registers.e);
                8
            },
            // BIT 6, H
            0x74 => {
                println!("BIT 6, H");
                self.bit(6, self.registers.h);
                8
            },
            // BIT 6, L
            0x75 => {
                println!("BIT 6, L");
                self.bit(6, self.registers.l);
                8
            },
            // BIT 6, (HL)
            0x76 => {
                println!("BIT 6, (HL)");
                self.bit(
                    6,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                16
            },
            // BIT 6, A
            0x77 => {
                println!("BIT 6, A");
                self.bit(6, self.registers.a);
                8
            },
            // BIT 7, B
            0x78 => {
                println!("BIT 7, B");
                self.bit(7, self.registers.b);
                8
            },
            // BIT 7, C
            0x79 => {
                println!("BIT 7, C");
                self.bit(7, self.registers.c);
                8
            },
            // BIT 7, D
            0x7A => {
                println!("BIT 7, D");
                self.bit(7, self.registers.d);
                8
            },
            // BIT 7, E
            0x7B => {
                println!("BIT 7, E");
                self.bit(7, self.registers.e);
                8
            },
            // BIT 7, H
            0x7C => {
                println!("BIT 7, H");
                self.bit(7, self.registers.h);
                8
            },
            // BIT 7, L
            0x7D => {
                println!("BIT 7, L");
                self.bit(7, self.registers.l);
                8
            },
            // BIT 7, (HL)
            0x7E => {
                println!("BIT 7, (HL)");
                self.bit(
                    7,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                16
            },
            // BIT 7, A
            0x7F => {
                println!("BIT 7, A");
                self.bit(7, self.registers.a);
                8
            },
            // RES 0, B
            0x80 => {
                println!("RES 0, B");
                self.registers.b = self.res(0, self.registers.b);
                8
            },
            // RES 0, C
            0x81 => {
                println!("RES 0, C");
                self.registers.c = self.res(0, self.registers.c);
                8
            },
            // RES 0, D
            0x82 => {
                println!("RES 0, D");
                self.registers.d = self.res(0, self.registers.d);
                8
            },
            // RES 0, E
            0x83 => {
                println!("RES 0, E");
                self.registers.e = self.res(0, self.registers.e);
                8
            },
            // RES 0, H
            0x84 => {
                println!("RES 0, H");
                self.registers.e = self.res(0, self.registers.h);
                8
            },
            // RES 0, L
            0x85 => {
                println!("RES 0, L");
                self.registers.l = self.res(0, self.registers.l);
                8
            },
            // RES 0, (HL)
            0x86 => {
                println!("RES 0, (HL)");
                let value = self.res(
                    0,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // RES 0, A
            0x87 => {
                println!("RES 0, A");
                self.registers.a = self.res(0, self.registers.a);
                8
            },
            // RES 1, B
            0x88 => {
                println!("RES 1, B");
                self.registers.b = self.res(1, self.registers.b);
                8
            },
            // RES 1, C
            0x89 => {
                println!("RES 1, C");
                self.registers.c = self.res(1, self.registers.c);
                8
            },
            // RES 1, D
            0x8A => {
                println!("RES 1, D");
                self.registers.d = self.res(1, self.registers.d);
                8
            },
            // RES 1, E
            0x8B => {
                println!("RES 1, E");
                self.registers.e = self.res(1, self.registers.e);
                8
            },
            // RES 1, H
            0x8C => {
                println!("RES 1, H");
                self.registers.h = self.res(1, self.registers.h);
                8
            },
            // RES 1, L
            0x8D => {
                println!("RES 1, L");
                self.registers.l = self.res(1, self.registers.l);
                8
            },
            // RES 1, (HL)
            0x8E => {
                println!("RES 1, (HL)");
                let value = self.res(
                    1,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // RES 1, A
            0x8F => {
                println!("RES 1, A");
                self.registers.a = self.res(1, self.registers.a);
                8
            },
            // RES 2, B
            0x90 => {
                println!("RES 2, B");
                self.registers.b = self.res(2, self.registers.b);
                8
            },
            // RES 2, C
            0x91 => {
                println!("RES 2, C");
                self.registers.c = self.res(2, self.registers.c);
                8
            },
            // RES 2, D
            0x92 => {
                println!("RES 2, D");
                self.registers.d = self.res(2, self.registers.d);
                8
            },
            // RES 2, E
            0x93 => {
                println!("RES 2, E");
                self.registers.e = self.res(2, self.registers.e);
                8
            },
            // RES 2, H
            0x94 => {
                println!("RES 2, H");
                self.registers.h = self.res(2, self.registers.h);
                8
            },
            // RES 2, L
            0x95 => {
                println!("RES 2, L");
                self.registers.l = self.res(2, self.registers.l);
                8
            },
            // RES 2, (HL)
            0x96 => {
                println!("RES 2, (HL)");
                let value = self.res(
                    2,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // RES 2, A
            0x97 => {
                println!("RES 2, A");
                self.registers.a = self.res(2, self.registers.a);
                8
            },
            // RES 3, B
            0x98 => {
                println!("RES 3, B");
                self.registers.b = self.res(3, self.registers.b);
                8
            },
            // RES 3, C
            0x99 => {
                println!("RES 3, C");
                self.registers.c = self.res(3, self.registers.c);
                8
            },
            // RES 3, D
            0x9A => {
                println!("RES 3, D");
                self.registers.d = self.res(3, self.registers.d);
                8
            },
            // RES 3, E
            0x9B => {
                println!("RES 3, E");
                self.registers.e = self.res(3, self.registers.e);
                8
            },
            // RES 3, H
            0x9C => {
                println!("RES 3, H");
                self.registers.h = self.res(3, self.registers.h);
                8
            },
            // RES 3, L
            0x9D => {
                println!("RES 3, L");
                self.registers.l = self.res(3, self.registers.l);
                8
            },
            // RES 3, (HL)
            0x9E => {
                println!("RES 3, (HL)");
                let value = self.res(
                    3,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // RES 3, A
            0x9F => {
                println!("RES 3, A");
                self.registers.a = self.res(3, self.registers.a);
                8
            },
            // RES 4, B
            0xA0 => {
                println!("RES 4, B");
                self.registers.b = self.res(4, self.registers.b);
                8
            },
            // RES 4, C
            0xA1 => {
                println!("RES 4, C");
                self.registers.c = self.res(4, self.registers.c);
                8
            },
            // RES 4, D
            0xA2 => {
                println!("RES 4, D");
                self.registers.d = self.res(4, self.registers.d);
                8
            },
            // RES 4, E
            0xA3 => {
                println!("RES 4, E");
                self.registers.e = self.res(4, self.registers.e);
                8
            },
            // RES 4, H
            0xA4 => {
                println!("RES 4, H");
                self.registers.h = self.res(4, self.registers.h);
                8
            },
            // RES 4, L
            0xA5 => {
                println!("RES 4, L");
                self.registers.l = self.res(4, self.registers.l);
                8
            },
            // RES 4, (HL)
            0xA6 => {
                println!("RES 4, (HL)");
                let value = self.res(
                    4,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // RES 4, A
            0xA7 => {
                println!("RES 4, A");
                self.registers.a = self.res(4, self.registers.a);
                8
            },
            // RES 5, B
            0xA8 => {
                println!("RES 5, B");
                self.registers.b = self.res(5, self.registers.b);
                8
            },
            // RES 5, C
            0xA9 => {
                println!("RES 5, C");
                self.registers.c = self.res(5, self.registers.c);
                8
            },
            // RES 5, D
            0xAA => {
                println!("RES 5, D");
                self.registers.d = self.res(5, self.registers.d);
                8
            },
            // RES 5, E
            0xAB => {
                println!("RES 5, E");
                self.registers.e = self.res(5, self.registers.e);
                8
            },
            // RES 5, H
            0xAC => {
                println!("RES 5, H");
                self.registers.h = self.res(5, self.registers.h);
                8
            },
            // RES 5, L
            0xAD => {
                println!("RES 5, L");
                self.registers.l = self.res(5, self.registers.l);
                8
            },
            // RES 5, (HL)
            0xAE => {
                println!("RES 5, (HL)");
                let value = self.res(
                    5,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // RES 5, A
            0xAF => {
                println!("RES 5, A");
                self.registers.a = self.res(5, self.registers.a);
                8
            },
            // RES 6, B
            0xB0 => {
                println!("RES 6, B");
                self.registers.b = self.res(6, self.registers.b);
                8
            },
            // RES 6, C
            0xB1 => {
                println!("RES 6, C");
                self.registers.c = self.res(6, self.registers.c);
                8
            },
            // RES 6, D
            0xB2 => {
                println!("RES 6, D");
                self.registers.d = self.res(6, self.registers.d);
                8
            },
            // RES 6, E
            0xB3 => {
                println!("RES 6, E");
                self.registers.e = self.res(6, self.registers.e);
                8
            },
            // RES 6, H
            0xB4 => {
                println!("RES 6, H");
                self.registers.h = self.res(6, self.registers.h);
                8
            },
            // RES 6, L
            0xB5 => {
                println!("RES 6, L");
                self.registers.l = self.res(6, self.registers.l);
                8
            },
            // RES 6, (HL)
            0xB6 => {
                println!("RES 6, (HL)");
                let value = self.res(
                    6,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // RES 6, A
            0xB7 => {
                println!("RES 6, A");
                self.registers.a = self.res(6, self.registers.a);
                8
            },
            // RES 7, B
            0xB8 => {
                println!("RES 7, B");
                self.registers.b = self.res(7, self.registers.b);
                8
            },
            // RES 7, C
            0xB9 => {
                println!("RES 7, C");
                self.registers.c = self.res(7, self.registers.c);
                8
            },
            // RES 7, D
            0xBA => {
                println!("RES 7, D");
                self.registers.d = self.res(7, self.registers.d);
                8
            },
            // RES 7, E
            0xBB => {
                println!("RES 7, E");
                self.registers.e = self.res(7, self.registers.e);
                8
            },
            // RES 7, H
            0xBC => {
                println!("RES 7, H");
                self.registers.h = self.res(7, self.registers.h);
                8
            },
            // RES 7, L
            0xBD => {
                println!("RES 7, L");
                self.registers.l = self.res(7, self.registers.l);
                8
            },
            // RES 7, (HL)
            0xBE => {
                println!("RES 7, (HL)");
                let value = self.res(
                    7,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // RES 7, A
            0xBF => {
                println!("RES 7, A");
                self.registers.a = self.res(7, self.registers.a);
                8
            },
            // SET 0, B
            0xC0 => {
                println!("SET 0, B");
                self.registers.b = self.set(0, self.registers.b);
                8
            },
            // SET 0, C
            0xC1 => {
                println!("SET 0, C");
                self.registers.c = self.set(0, self.registers.c);
                8
            },
            // SET 0, D
            0xC2 => {
                println!("SET 0, D");
                self.registers.d = self.set(0, self.registers.d);
                8
            },
            // SET 0, E
            0xC3 => {
                println!("SET 0, E");
                self.registers.e = self.set(0, self.registers.e);
                8
            },
            // SET 0, H
            0xC4 => {
                println!("SET 0, H");
                self.registers.h = self.set(0, self.registers.h);
                8
            },
            // SET 0, L
            0xC5 => {
                println!("SET 0, L");
                self.registers.l = self.set(0, self.registers.l);
                8
            },
            // SET 0, (HL)
            0xC6 => {
                println!("SET 0, (HL)");
                let value = self.set(
                    0,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // SET 0, A
            0xC7 => {
                println!("SET 0, A");
                self.registers.a = self.set(0, self.registers.a);
                8
            },
            // SET 1, B
            0xC8 => {
                println!("SET 1, B");
                self.registers.b = self.set(1, self.registers.b);
                8
            },
            // SET 1, C
            0xC9 => {
                println!("SET 1, C");
                self.registers.c = self.set(1, self.registers.c);
                8
            },
            // SET 1, D
            0xCA => {
                println!("SET 1, D");
                self.registers.d = self.set(1, self.registers.d);
                8
            },
            // SET 1, E
            0xCB => {
                println!("SET 1, E");
                self.registers.e = self.set(1, self.registers.e);
                8
            },
            // SET 1, H
            0xCC => {
                println!("SET 1, H");
                self.registers.h = self.set(1, self.registers.h);
                8
            },
            // SET 1, L
            0xCD => {
                println!("SET 1, L");
                self.registers.l = self.set(1, self.registers.l);
                8
            },
            // SET 1, (HL)
            0xCE => {
                println!("SET 1, (HL)");
                let value = self.set(
                    1,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // SET 1, A
            0xCF => {
                println!("SET 1, A");
                self.registers.a = self.set(1, self.registers.a);
                8
            },
            // SET 2, B
            0xD0 => {
                println!("SET 2, B");
                self.registers.b = self.set(2, self.registers.b);
                8
            },
            // SET 2, C
            0xD1 => {
                println!("SET 2, C");
                self.registers.c = self.set(2, self.registers.c);
                8
            },
            // SET 2, D
            0xD2 => {
                println!("SET 2, D");
                self.registers.d = self.set(2, self.registers.d);
                8
            },
            // SET 2, E
            0xD3 => {
                println!("SET 2, E");
                self.registers.e = self.set(2, self.registers.e);
                8
            },
            // SET 2, H
            0xD4 => {
                println!("SET 2, H");
                self.registers.h = self.set(2, self.registers.h);
                8
            },
            // SET 2, L
            0xD5 => {
                println!("SET 2, L");
                self.registers.l = self.set(2, self.registers.l);
                8
            },
            // SET 2, (HL)
            0xD6 => {
                println!("SET 2, (HL)");
                let value = self.set(
                    2,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // SET 2, A
            0xD7 => {
                println!("SET 2, A");
                self.registers.a = self.set(2, self.registers.a);
                8
            },
            // SET 3, B
            0xD8 => {
                println!("SET 3, B");
                self.registers.b = self.set(3, self.registers.b);
                8
            },
            // SET 3, C
            0xD9 => {
                println!("SET 3, C");
                self.registers.c = self.set(3, self.registers.c);
                8
            },
            // SET 3, D
            0xDA => {
                println!("SET 3, D");
                self.registers.d = self.set(3, self.registers.d);
                8
            },
            // SET 3, E
            0xDB => {
                println!("SET 3, E");
                self.registers.e = self.set(3, self.registers.e);
                8
            },
            // SET 3, H
            0xDC => {
                println!("SET 3, H");
                self.registers.h = self.set(3, self.registers.h);
                8
            },
            // SET 3, L
            0xDD => {
                println!("SET 3, L");
                self.registers.l = self.set(3, self.registers.l);
                8
            },
            // SET 3, (HL)
            0xDE => {
                println!("SET 3, (HL)");
                let value = self.set(
                    3,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // SET 3, A
            0xDF => {
                println!("SET 3, A");
                self.registers.a = self.set(3, self.registers.a);
                8
            },
            // SET 4, B
            0xE0 => {
                println!("SET 4, B");
                self.registers.b = self.set(4, self.registers.b);
                8
            },
            // SET 4, C
            0xE1 => {
                println!("SET 4, C");
                self.registers.c = self.set(4, self.registers.c);
                8
            },
            // SET 4, D
            0xE2 => {
                println!("SET 4, D");
                self.registers.d = self.set(4, self.registers.d);
                8
            },
            // SET 4, E
            0xE3 => {
                println!("SET 4, E");
                self.registers.e = self.set(4, self.registers.e);
                8
            },
            // SET 4, H
            0xE4 => {
                println!("SET 4, H");
                self.registers.h = self.set(4, self.registers.h);
                8
            },
            // SET 4, L
            0xE5 => {
                println!("SET 4, L");
                self.registers.l = self.set(4, self.registers.l);
                8
            },
            // SET 4, (HL)
            0xE6 => {
                println!("SET 4, (HL)");
                let value = self.set(
                    4,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // SET 4, A
            0xE7 => {
                println!("SET 4, A");
                self.registers.a = self.set(4, self.registers.a);
                8
            },
            // SET 5, B
            0xE8 => {
                println!("SET 5, B");
                self.registers.b = self.set(5, self.registers.b);
                8
            },
            // SET 5, C
            0xE9 => {
                println!("SET 5, C");
                self.registers.c = self.set(5, self.registers.c);
                8
            },
            // SET 5, D
            0xEA => {
                println!("SET 5, D");
                self.registers.d = self.set(5, self.registers.d);
                8
            },
            // SET 5, E
            0xEB => {
                println!("SET 5, E");
                self.registers.e = self.set(5, self.registers.e);
                8
            },
            // SET 5, H
            0xEC => {
                println!("SET 5, H");
                self.registers.h = self.set(5, self.registers.h);
                8
            },
            // SET 5, L
            0xED => {
                println!("SET 5, L");
                self.registers.l = self.set(5, self.registers.l);
                8
            },
            // SET 5, (HL)
            0xEE => {
                println!("SET 5, (HL)");
                let value = self.set(
                    5,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // SET 5, A
            0xEF => {
                println!("SET 5, A");
                self.registers.a = self.set(5, self.registers.a);
                8
            },
            // SET 6, B
            0xF0 => {
                println!("SET 6, B");
                self.registers.b = self.set(6, self.registers.b);
                8
            },
            // SET 6, C
            0xF1 => {
                println!("SET 6, C");
                self.registers.c = self.set(6, self.registers.c);
                8
            },
            // SET 6, D
            0xF2 => {
                println!("SET 6, D");
                self.registers.d = self.set(6, self.registers.d);
                8
            },
            // SET 6, E
            0xF3 => {
                println!("SET 6, E");
                self.registers.e = self.set(6, self.registers.e);
                8
            },
            // SET 6, H
            0xF4 => {
                println!("SET 6, H");
                self.registers.h = self.set(6, self.registers.h);
                8
            },
            // SET 6, L
            0xF5 => {
                println!("SET 6, L");
                self.registers.l = self.set(6, self.registers.l);
                8
            },
            // SET 6, (HL)
            0xF6 => {
                println!("SET 6, (HL)");
                let value = self.set(
                    6,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // SET 6, A
            0xF7 => {
                println!("SET 6, A");
                self.registers.a = self.set(6, self.registers.a);
                8
            },
            // SET 7, B
            0xF8 => {
                println!("SET 7, B");
                self.registers.b = self.set(7, self.registers.b);
                8
            },
            // SET 7, C
            0xF9 => {
                println!("SET 7, C");
                self.registers.c = self.set(7, self.registers.c);
                8
            },
            // SET 7, D
            0xFA => {
                println!("SET 7, D");
                self.registers.d = self.set(7, self.registers.d);
                8
            },
            // SET 7, E
            0xFB => {
                println!("SET 7, E");
                self.registers.e = self.set(7, self.registers.e);
                8
            },
            // SET 7, H
            0xFC => {
                println!("SET 7, H");
                self.registers.h = self.set(7, self.registers.h);
                8
            },
            // SET 7, L
            0xFD => {
                println!("SET 7, L");
                self.registers.l = self.set(7, self.registers.l);
                8
            },
            // SET 7, (HL)
            0xFE => {
                println!("SET 7, (HL)");
                let value = self.set(
                    7,
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    value
                );
                16
            },
            // SET 7, A
            0xFF => {
                println!("SET 7, A");
                self.registers.a = self.set(7, self.registers.a);
                8
            },
        }
    }

    /// Updates the value of ime
    ///
    /// Activate interruption handing 1 instruction after ei.  
    /// Deactivate interruption handing 1 instruction after di.  
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.di = 2;
    /// // emi is not deactivated after one update
    /// new_cpu.update_ime();
    /// assert!(new_cpu.emi);
    /// // emi is deactivated after the second update
    /// new_cpu.update_ime();
    /// assert!(!new_cpu.emi);
    /// ```
    fn update_ime(&mut self) {
        match self.di {
            2 => {
                self.di = 1;
            },
            1 => {
                self.di = 0;
                self.ime = false;
            },
            _ => {}
        }
        match self.ei {
            2 => {
                self.ei = 1;
            },
            1 => {
                self.ei = 0;
                self.ime = true;
            },
            _ => {}
        }
    }

    /// Checks for interruption and handle them
    ///
    /// If the cpu wants to handle interruption(ime = 1), if the interrupt
    /// flag and the corresponding interrupt enable is set, the program counter
    /// is moved to the interruption handler.
    ///
    /// # Returns
    /// **u32**: Number of cycles used to handle interruptions (0 if not
    /// handled).
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// // Artificially create a joypad interruption
    /// new_cpu.mmu.interrupt_flag = 0x10;
    /// new_cpu.mmu.ie = 0x10;
    /// new_cpu.ime = true;
    /// assert_eq!(new_cpu.manage_interruptions(), 20);
    /// // A joypad interruption moves the program counter to the adress 0x0060
    /// assert_eq!(new_cpu.registers.get_pc(), 0x0060);
    fn manage_interruptions(&mut self) -> u32 {
        if self.ime {
            // if io.pending_joypad_interruption
            if 
                self.mmu.interrupt_flag & 0x10 == 0x10 &&
                self.mmu.ie & 0x10 == 0x10
            {
                self.mmu.interrupt_flag |= 0xEF;
                self.mmu.ie |= 0xEF;
                // 2 NOP + PUSH PC
                self.rst(0x0060);
                return 20;
            }
            // if io.pending_timer_interruption
            if 
                self.mmu.interrupt_flag & 0x04 == 0x04 &&
                self.mmu.ie & 0x04 == 0x04
            {
                self.mmu.interrupt_flag |= 0xFB;
                self.mmu.ie |= 0xFB;
                // 2 NOP + PUSH PC + LD PC 0x50
                self.rst(0x0050);
                return 20;
            }
            // if gpu.pending_stat_interrupt
            if 
                self.mmu.interrupt_flag & 0x02 == 0x02 &&
                self.mmu.ie & 0x02 == 0x02
            {
                self.mmu.interrupt_flag |= 0xFD;
                self.mmu.ie |= 0xFD;
                // 2 NOP + PUSH PC + LD PC 0x50
                self.rst(0x0048);
                return 20;
            }
            // if gpu.pending_vblank_interrupt
            if 
                self.mmu.interrupt_flag & 0x01 == 0x01 &&
                self.mmu.ie & 0x01 == 0x01
            {
                self.mmu.interrupt_flag |= 0xFE;
                self.mmu.ie |= 0xFE;
                // 2 NOP + PUSH PC + LD PC 0x50
                self.rst(0x0040);
                return 20;
            }
        }
        // If 0 is return, no interruptions should be called
        0
    }

    /// Execute one CPU step
    ///
    /// Update and mange interruption, and execute one instruction if no
    /// interruption are found
    ///
    /// # Returns
    /// **u32**: Number of CPU cycles used for the step
    pub fn execute_step(&mut self) -> u32 {
        self.update_ime();
        let time_interruption = self.manage_interruptions();
        if time_interruption != 0 {
            self.should_stop = self.mmu.update(time_interruption);
            return time_interruption;
        }
        if self.is_halted {
            self.should_stop = self.mmu.update(4);
            return 4;
        }
        let res = self.receive_op();
        self.should_stop = self.mmu.update(res);
        res
    }

    /// Returns the given value incremented
    ///
    /// Sets the Z flag iff the result is zero  
    /// Sets the H flag iff the lower nybble has a carry
    /// Always resets the N flag
    /// Does not affect the C flag
    ///
    /// # Arguments
    /// **value (u8)**: Value to increment
    ///
    /// # Returns
    /// **u8**: Parameter's value incremented
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU_new("test.gb");
    /// assert_eq!(new_cpu.dec(0x01), 0x02);
    /// ```
    fn inc(&mut self, value: u8) -> u8 {
        let res = value.wrapping_add(1);
        self.registers.set_zero(
            res == 0
        );
        // Est-ce que la première moitié overflow?
        self.registers.set_half(
            (value & 0x0F) == 0x0F
        );
        self.registers.set_sub(
            false
        );
        // Pas de maj du carry
        res
    }

    /// Returns the given value decremented
    ///
    /// Sets the Z flag iff the result is zero  
    /// Sets the H flag iff the lower nybble takes a borrow  
    /// Always sets the N flag
    /// Does not affect the C flag
    ///
    /// # Arguments
    /// **value (u8)**: Value to decrement
    ///
    /// # Returns
    /// **u8**: Parameter's value decremented
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU_new("test.gb");
    /// assert_eq!(new_cpu.dec(0x02), 0x01);
    /// ```
    fn dec(&mut self, value: u8) -> u8 {
        let res = value.wrapping_sub(1);
        self.registers.set_zero(
            res == 0
        );
        // Est-ce que la première moitié overflow?
        self.registers.set_half(
            (value & 0x0F) == 0x00
        );
        self.registers.set_sub(
            true
        );
        // Pas de maj du carry
        res
    }

    /// Add the value of the A register with the given value
    ///
    /// Always resets the N flag  
    /// Sets the C flag iff there is an overflow
    /// Sets the H falg iff there is a carry on bit 4  
    /// Sets the z flag iff the result is zero
    ///
    /// # Arguments
    /// **value (u8)**: Value added to A
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.registers.a = 0x12;
    /// new_cpu.add(0x34);
    /// // 0x12 + 0x34 = 0x46
    /// assert_eq!(new_cpu.registers.a, 0x46);
    /// ```
    fn add(&mut self, value: u8) {
        let (
            new_value,
            did_overflow
        ) = self.registers.a.overflowing_add(value);
        self.registers.set_zero(
            new_value == 0
        );
        self.registers.set_sub(
            false
        );
        self.registers.set_carry(
            did_overflow
        );
        // Est-ce que la première moitié overflow?
        self.registers.set_half(
            (self.registers.a & 0x0F) + (value + 0x0F) > 0x0F
        );
        self.registers.a = new_value;
    }

    /// Add the value of the A register with the given value and with the carry
    ///
    /// Always resets the N flag  
    /// Sets the C flag iff there is an overflow
    /// Sets the H falg iff there is a carry on bit 4  
    /// Sets the z flag iff the result is zero
    ///
    /// # Arguments
    /// **value (u8)**: Value added to A
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.registers.a = 0x12;
    /// new_cpu.registers.set_carry(true);
    /// new_cpu.adc(0x34);
    /// // 0x12 + 0x34 + 0x01 = 0x47
    /// assert_eq!(new_cpu.registers.a, 0x47);
    /// ```
    fn adc(&mut self, value: u8) {
        let carry_as_u8 = if self.registers.get_carry() {1u8} else {0u8};
        let (
            temp_value,
            did_overflow1
        ) = value.overflowing_add(carry_as_u8);
        let (
            new_value,
            did_overflow2
        ) = self.registers.a.overflowing_add(temp_value);
        self.registers.set_zero(
            new_value == 0
        );
        self.registers.set_sub(
            false
        );
        self.registers.set_carry(
            did_overflow1 || did_overflow2
        );
        // Est-ce que la première moitié overflow?
        self.registers.set_half(
            (self.registers.a & 0x0F) + (value + 0x0F) + carry_as_u8 > 0x0F
        );
        self.registers.a = new_value;
    }

    /// Add the value of the HL register with the given value and update HL.
    ///
    /// Always resets the N flag  
    /// Sets the C flag iff there is an overflow
    /// Sets the H falg iff there is a carry on bit 12  
    /// Does not affect the Z flag  
    ///
    /// # Arguments
    /// **value (u16)**: Value added to HL
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.registers.set_hl(0x3412);
    /// new_cpu.addhl(0x369C);
    /// // 0x369C + 0x3412 = 0x6AAE
    /// assert_eq!(new_cpu.registers.get_hl(), 0x6AAE);
    /// ```
    fn addhl(&mut self, value: u16) {
        let (
            new_value,
            did_overflow
        ) = self.registers.get_hl().overflowing_add(value);
        // Pas de maj du flag zero
        self.registers.set_sub(
            false
        );
        self.registers.set_carry(
            did_overflow
        );
        // Est-ce que la première moitié overflow?
        self.registers.set_half(
            (self.registers.get_hl() & 0x07FF) + (value + 0x07FF) > 0x07FF
        );
        self.registers.set_hl(new_value);
    }

    /// Add the value with an immediate word value read as a signed number.
    ///
    /// Always resets the Z flag  
    /// Always resets the N flag  
    /// Sets the C flag iff there is a carry on bit 8
    /// Sets the H falg iff there is a carry on bit 4
    ///
    /// # Arguments
    /// **value (u16)**: Value to add the immediate real to
    ///
    /// # Returns
    /// **u16**: Given value added with the real word written at the program
    /// counter.
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// // Normally the value will be read in the text session containing the
    /// // code, however, for testing purpose and to be able to write at the
    /// // adress of the programm counter, we will write in the stack and move
    /// // the program counter in the stack.
    /// let adress_imediate = new_cpu.registers.get_sp() - 1;
    /// new_cpu.push(0x12);
    /// new_cpu.push(0x34);
    /// new_cpu.set_pc(adress_immediate);
    /// // 0x369C + 0x3412 = 0x6AAE
    /// assert_eq!(new_cpu.addr8(0x369C), 0x6AAE);
    /// ```
    fn addr8(&mut self, value: u16) -> u16 {
        // i8 to have a sign value, i16 to keep the sign and have 16 bits, u16
        // to make the addition
        let fetched_value = self.fetchbyte() as i8 as i16 as u16;
        self.registers.set_zero(
            false
        );
        self.registers.set_sub(
            false
        );
        self.registers.set_carry(
            (value & 0x00FF) + (fetched_value & 0x00FF) > 0x00FF
        );
        self.registers.set_half(
            (value & 0x000F) + (fetched_value & 0x000F) > 0x000F
        );
        fetched_value.wrapping_add(value)
    }

    /// Substract the given value to the register A
    ///
    /// Sets the Z flag iff the result is 0  
    /// Always sets the N flag  
    /// Sets the C flag iff the result would be negative without wrapping  
    /// Sets the H falg iff the result of the substraction of the value made by
    /// the four rightest bits of A and those of the given value is negative
    ///
    /// # Arguments
    /// **value (u8)**: Value to substract to the register A
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.registers.a = 0x12;
    /// new_cpu.sub(0x03);
    /// // 0x12 - 0x03 = 0x0F
    /// assert_eq!(new_cpu.registers.a, 0x0F);
    /// ```
    fn sub(&mut self, value: u8) {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        self.registers.set_zero(
            new_value == 0
        );
        self.registers.set_sub(
            true
        );
        self.registers.set_carry(
            did_overflow
        );
        // Est-ce que la première moitié overflow?
        self.registers.set_half(
            (self.registers.a & 0x0F) < (value + 0x0F)
        );
        self.registers.a = new_value;
    }

    /// Substract the given value and the carry to the register A
    ///
    /// Sets the Z flag iff the result is 0  
    /// Always sets the N flag  
    /// Sets the C flag iff the result would be negative without wrapping  
    /// Sets the H falg iff the result of the substraction of the value made by
    /// the four rightest bits of A, those of the given value, and the value of
    /// the carry flag, is negative
    ///
    /// # Arguments
    /// **value (u8)**: Value to substract to the register A
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.registers.a = 0x12;
    /// new_cpu.registers.set_carry(true);
    /// new_cpu.sbc(0x03);
    /// // 0x12 - 0x03 - 0x01 = 0x0E
    /// assert_eq!(new_cpu.registers.a, 0x0E);
    /// ```
    fn sbc(&mut self, value: u8) {
        let carry_as_u8 = if self.registers.get_carry() {1u8} else {0u8};
        let (temp_value, did_overflow1) = self.registers.a.overflowing_sub(
            carry_as_u8
        );
        let (new_value, did_overflow2) = temp_value.overflowing_sub(value);
        self.registers.set_zero(
            new_value == 0
        );
        self.registers.set_sub(
            true
        );
        self.registers.set_carry(
            did_overflow1 || did_overflow2
        );
        // Est-ce que la première moitié overflow?
        self.registers.set_half(
            (self.registers.a & 0x0F) < (value + 0x0F) + carry_as_u8
        );
        self.registers.a = new_value;
    }

    /// Sets the flags depending of the result of A - arg
    ///
    /// Sets the Z flag iff the result is zero (aka A == arg)  
    /// Always sets the N flag  
    /// Sets the C flag iff the result is negative (aka A < arg)  
    /// Sets the H flag iff the four lower bits of A make a smaller value than
    /// the four lower bits of arg  
    ///
    /// # Arguments
    /// **value (u8)**: Value to compare A to
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.registers.a = 0x12;
    /// let res = new_cpu.cp(0x15);
    /// // 0x12 < 0x15
    /// assert!(new_cpu.registers.get_carry());
    /// ```
    fn cp(&mut self, value: u8) {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        self.registers.set_zero(
            new_value == 0
        );
        self.registers.set_sub(
            true
        );
        self.registers.set_carry(
            did_overflow
        );
        self.registers.set_half(
            (self.registers.a & 0x0F) < (value + 0x0F)
        );
    }

    /// And the given value with the register A and update the value of this
    /// register, bit by bit
    ///
    /// Sets the Z flag iff the result is 0  
    /// Always sets the H flag  
    /// Always resets the C flag  
    /// Always resets the N flag
    ///
    /// # Arguments
    /// **value (u8)**: Value to and A with
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.registers.a = 0b01110110;
    /// new_cpu.or(0b01001011);
    /// assert_eq!(new_cpu.registers.a, 0b01000010);
    /// ```
    fn and(&mut self, value: u8) {
        self.registers.a &= value;
        self.registers.set_zero(
            self.registers.a == 0
        );
        self.registers.set_half(
            true
        );
        self.registers.set_carry(
            false
        );
        self.registers.set_sub(
            false
        );
    }

    /// Or the given value with the register A and update the value of this
    /// register, bit by bit
    ///
    /// Sets the Z flag iff the result is 0  
    /// Always resets the H flag  
    /// Always resets the C flag  
    /// Always resets the N flag
    ///
    /// # Arguments
    /// **value (u8)**: Value to or A with
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.registers.a = 0b01110110;
    /// new_cpu.or(0b01001011);
    /// assert_eq!(new_cpu.registers.a, 0b01111111);
    /// ```
    fn or(&mut self, value: u8) {
        self.registers.a |= value;
        self.registers.set_zero(
            self.registers.a == 0
        );
        self.registers.set_half(
            false
        );
        self.registers.set_carry(
            false
        );
        self.registers.set_sub(
            false
        );
    }

    /// Xor the given value with the register A and update the value of this
    /// register, bit by bit
    ///
    /// Sets the Z flag iff the result is 0  
    /// Always resets the H flag  
    /// Always resets the C flag  
    /// Always resets the N flag
    ///
    /// # Arguments
    /// **value (u8)**: Value to xor A with
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.registers.a = 0b01110110;
    /// new_cpu.xor(0b01001011);
    /// assert_eq!(new_cpu.registers.a, 0b00001101);
    /// ```
    fn xor(&mut self, value: u8) {
        self.registers.a ^= value;
        self.registers.set_zero(
            self.registers.a == 0
        );
        self.registers.set_half(
            false
        );
        self.registers.set_carry(
            false
        );
        self.registers.set_sub(
            false
        );
    }

    /// Returns the given shifted one bit to the left and with the rightest bit
    /// set iff the carry is set
    ///
    /// Always resets the H flag  
    /// Always resets the N flag  
    /// Sets the Z flag iff the result is zero  
    /// Sets the carry flag iff the bit that overstep is set  
    ///
    /// # Arguments
    /// **value (u8)**: Value to shift
    ///
    /// # Returns
    /// **u8**: Value of the argument shifted one bit to the left and with the
    /// rightest bit set iff the carry is set
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.registers.a = 0b10010011;
    /// assert!(!new_cpu.registers.get_carry());
    /// let res = new_cpu.rlc(new_cpu.regiters.a);
    /// assert_eq!(res, 0b0010110);
    /// ```
    fn rl(&mut self, value: u8) -> u8 {
        let res = (value << 1) | (
            if self.registers.get_carry() {
                1
            } else {
                0
            }
        );
        self.registers.set_half(
            false
        );
        self.registers.set_sub(
            false
        );
        self.registers.set_zero(
            res == 0
        );
        // Est-ce que le bit qui "sort" est un 1?
        self.registers.set_carry(
            (value & 0x80) == 0x80
        );
        res
    }

    /// Returns the given shifted one bit to the left and with the rightest bit
    /// set iff the bit that overstep is set
    ///
    /// Always resets the H flag  
    /// Always resets the N flag  
    /// Sets the Z flag iff the result is zero  
    /// Sets the carry flag iff the bit that overstep is set  
    ///
    /// # Arguments
    /// **value (u8)**: Value to shift
    ///
    /// # Returns
    /// **u8**: Value of the argument shifted one bit to the left and with the
    /// rightest bit set iff the bit that overstep is set
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.registers.a = 0b10010011;
    /// let res = new_cpu.rlc(new_cpu.regiters.a);
    /// assert_eq!(res, 0b0010111);
    /// ```
    fn rlc(&mut self, value: u8) -> u8 {
        // left shift + bit qui sort placé à droite
        let res = (value << 1) | (
            if (value & 0x80) == 0x80 {
                1
            } else {
                0
            }
        );
        self.registers.set_half(
            false
        );
        self.registers.set_sub(
            false
        );
        self.registers.set_zero(
            res == 0
        );
        // Est-ce que le bit qui "sort" est un 1?
        self.registers.set_carry(
            (value & 0x80) == 0x80
        );
        res
    }

    /// Returns the given shifted one bit to the right and with the leftest bit
    /// set iff the carry is set
    ///
    /// Always resets the H flag  
    /// Always resets the N flag  
    /// Sets the Z flag iff the result is zero  
    /// Sets the carry flag iff the bit that overstep is set  
    ///
    /// # Arguments
    /// **value (u8)**: Value to shift
    ///
    /// # Returns
    /// **u8**: Value of the argument shifted one bit to the right and with the
    /// leftest bit set iff the carry is set
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.registers.a = 0b10010011;
    /// // The carry flag is initially reset
    /// assert!(!new_cpu.registers.get_carry());
    /// let res = new_cpu.rrc(new_cpu.regiters.a);
    /// assert_eq!(res, 0b01001001);
    /// ```
    fn rr(&mut self, value: u8) -> u8 {
        // right shift + carry flag placé à gauche
        let res = (value >> 1) | (
            if self.registers.get_carry() {
                0x80
            } else {
                0
            }
        );
        self.registers.set_half(
            false
        );
        self.registers.set_sub(
            false
        );
        self.registers.set_zero(
            res == 0
        );
        // Est-ce que le bit qui "sort" est un 1?
        self.registers.set_carry(
            (value & 0x01) == 0x01
        );
        res
    }

    /// Returns the given shifted one bit to the right and with the leftest bit
    /// set iff the bit that overstep is set
    ///
    /// Always resets the H flag  
    /// Always resets the N flag  
    /// Sets the Z flag iff the result is zero  
    /// Sets the carry flag iff the bit that overstep is set  
    ///
    /// # Arguments
    /// **value (u8)**: Value to shift
    ///
    /// # Returns
    /// **u8**: Value of the argument shifted one bit to the right and with the
    /// leftest bit set iff the bit that overstep is set
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.registers.a = 0b10010011;
    /// let res = new_cpu.rrc(new_cpu.regiters.a);
    /// assert_eq!(res, 0b11001001);
    /// ```
    fn rrc(&mut self, value: u8) -> u8 {
        let res = (value >> 1) | (
            if (value & 0x01) == 0x01 {
                0x80
            } else {
                0
            }
        );
        self.registers.set_half(
            false
        );
        self.registers.set_sub(
            false
        );
        self.registers.set_zero(
            res == 0
        );
        // Est-ce que le bit qui "sort" est un 1?
        self.registers.set_carry(
            (value & 0x01) == 0x01
        );
        res
    }

    /// Returns the given shifted one bit to the left and with the rightest bit
    /// unchanged
    ///
    /// Always resets the H flag  
    /// Always resets the N flag  
    /// Sets the Z flag iff the result is zero  
    /// Sets the carry flag iff the bit that overstep is set  
    ///
    /// # Arguments
    /// **value (u8)**: Value to shift
    ///
    /// # Returns
    /// **u8**: Value of the argument shifted one bit to the left and with the
    /// rightest bit unchanged
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.registers.a = 0b10010011;
    /// let res = new_cpu.sla(new_cpu.regiters.a);
    /// assert_eq!(res, 0b00100111);
    /// ```
    fn sla(&mut self, value: u8) -> u8 {
        let result = value << 1;
        self.registers.set_half(
            false
        );
        self.registers.set_sub(
            false
        );
        self.registers.set_zero(
            result == 0
        );
        self.registers.set_carry(
            value & 0x80 == 0x80
        );
        result
    }

    /// Returns the given shifted one bit to the right and with the leftest bit
    /// unchanged
    ///
    /// Always resets the H flag  
    /// Always resets the N flag  
    /// Sets the Z flag iff the result is zero  
    /// Sets the carry flag iff the bit that overstep is set  
    ///
    /// # Arguments
    /// **value (u8)**: Value to shift
    ///
    /// # Returns
    /// **u8**: Value of the argument shifted one bit to the right and with the
    /// leftest bit unchanged
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.registers.a = 0b10010010;
    /// let res = new_cpu.sra(new_cpu.regiters.a);
    /// assert_eq!(res, 0b11001001);
    /// ```
    fn sra(&mut self, value: u8) -> u8 {
        let result = value >> 1 | (value & 0x80);
        self.registers.set_half(
            false
        );
        self.registers.set_sub(
            false
        );
        self.registers.set_zero(
            result == 0
        );
        self.registers.set_carry(
            value & 0x01 == 0x01
        );
        result
    }

    /// Returns the given shifted one bit to the right and with the leftest bit
    /// reset
    ///
    /// Always resets the H flag  
    /// Always resets the N flag  
    /// Sets the Z flag iff the result is zero  
    /// Sets the carry flag iff the bit that overstep is set  
    ///
    /// # Arguments
    /// **value (u8)**: Value to shift
    ///
    /// # Returns
    /// **u8**: Value of the argument shifted one bit to the right and with the
    /// leftest bit reset
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.registers.a = 0b10010010;
    /// let res = new_cpu.srl(new_cpu.regiters.a);
    /// assert_eq!(res, 0b01001001);
    /// ```
    fn srl(&mut self, value: u8) -> u8 {
        let result = value >> 1;
        self.registers.set_half(
            false
        );
        self.registers.set_sub(
            false
        );
        self.registers.set_zero(
            result == 0
        );
        self.registers.set_carry(
            value & 0x01 == 0x01
        );
        result
    }

    /// Returns the given value with the four lower and the four upper bits
    /// swapped.
    ///
    /// Sets the Z bit iff the result is zero  
    /// Always resets the C flag  
    /// Always resets the N flag  
    /// Always resets the H flag  
    ///
    /// # Arguments
    /// **value (u8)**: Operand to modify
    ///
    /// # Returns
    /// **u8**: Value of the argument with the four lower and the four upper
    /// bits swapped.
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.registers.a = 0x12;
    /// let res = new_cpu.swap(new_cpu.regiters.a);
    /// assert_eq!(res, 0x21);
    /// ```
    fn swap(&mut self, value: u8) -> u8 {
        let result = 
            (value & 0xF0) >> 4 |
            (value & 0x0F) << 4
        ;
        self.registers.set_zero(
            result == 0
        );
        self.registers.set_carry(
            false
        );
        self.registers.set_sub(
            false
        );
        self.registers.set_half(
            false
        );
        result
    }

    /// Sets the zero flag depending of the value of the given bit of the
    /// second argument
    ///
    /// Sets the Z flag iff the bit-th bit of value is 0b0  
    /// Always resets the N flag  
    /// Always sets the H flag  
    /// Does not affect the C flag  
    ///
    /// # Arguments
    /// **bit (u32)**: Position of the bit to test  
    /// **value (u8)**: Value to test
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// assert!(new_cpu.get_zero());
    /// new_cpu.registers.a = 0x02;
    /// new_cpu.bit(2, new_cpu.regiters.a);
    /// assert!(!new_cpu.get_zero());
    /// ```
    fn bit(&mut self, bit: u32, value: u8) {
        self.registers.set_zero(
            value & ((1 << bit) as u8) == 0
        );
        self.registers.set_sub(
            false
        );
        self.registers.set_half(
            true
        );
        // Carry not affected
    }

    /// Returns the given value but the bit whose position is given in argument
    /// is reset
    ///
    /// # Arguments
    /// **bit (u32)**: Position of the bit to reset in the value (0 being the
    /// LSB)  
    /// **value (u8)**: Value that will be modified  
    ///
    /// # Returns
    /// **u8**: Initial value with the given bit reset
    ///
    /// # Examples
    /// ``` rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.registers.a = 0x04;
    /// let res = new_cpu.res(2, new_cpu.registers.a);
    /// assert_eq!(res, 0x00);
    /// ```
    fn res(&mut self, bit: u32, value: u8) -> u8 {
        value & !((1 << bit) as u8)
    }

    /// Returns the given value but the bit whose position is given in argument
    /// is set
    ///
    /// # Arguments
    /// **bit (u32)**: Position of the bit to set in the value (0 being the
    /// LSB)  
    /// **value (u8)**: Value that will be modified
    ///
    /// # Returns
    /// **u8**: Initial value with the given bit set
    ///
    /// # Examples
    /// ``` rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// // Initially the A registers contains the value 0
    /// assert_eq!(new_cpu.registers.a, 0x00);
    /// let res = new_cpu.set(2, new_cpu.registers.a);
    /// assert_eq!(res, 0x04);
    /// ```
    fn set(&mut self, bit: u32, value: u8) -> u8 {
        value | ((1 << bit) as u8)
    }

    /// Decimal adjust the register A
    ///
    /// Adjust the value of A to obtain a correct Binary Coded Decimal (BCD)
    /// meaning that each byte has a value between 0 and 9.  
    /// Its value is adjusted to make the previous operation appear as if
    /// it was done with two decimal numbers.  
    /// Substracts 6 to the upper or lower nybble depending of some criteria:  
    /// - was the previous operation a substraction (N flag) -> correction by
    /// addition or substraction,  
    /// - did an overflow occur (C flag) -> adjust the upper nybble,  
    /// - did an overflow occur on the first nible (H flab) -> adjust the lower
    /// nybble,  
    /// - is one of the nybble's value greater than 9 (addition only) ->
    /// correction of this nybble.
    ///
    /// Sets the Z flag iff the result is zero  
    /// Does not affect the N flag  
    /// Always resets the H falg  
    /// Sets the carry flag iff the correction create an overflow  
    ///
    /// # Examples
    /// ```rust
    /// let mut new_cpu = CPU::new("test.gb");
    /// new_cpu.registers.a = 0x92;
    /// new_cpu.registers.b = 0x36;
    /// new_cpu.add(new_cpu.registers.b);
    /// // The two operand are correct BCD but the resutl is not
    /// assert_eq!(new_cpu.registers.a, 0xC8);
    /// new_cpu.daa();
    /// assert_eq!(new_cpu.registers.a, 0x28);
    /// ```
    fn daa(&mut self) {
        let mut a = self.registers.a;
        self.registers.set_carry(false);
        if self.registers.get_sub() {
            if self.registers.get_carry() {
                a = a.wrapping_sub(0x60);
                self.registers.set_carry(true);
            }
            if self.registers.get_half() {
                a = a.wrapping_sub(0x06);
            }
        } else {
            if self.registers.get_carry() || a > 0x99 {
                self.registers.set_carry(true);
                a = a.wrapping_add(0x60);
            }
            if self.registers.get_half() || (a & 0x0F) > 0x09 {
                a = a.wrapping_add(0x06);
            }
        }
        self.registers.set_half(false);
        self.registers.set_zero(a == 0);
        self.registers.a = a;
    }
}
