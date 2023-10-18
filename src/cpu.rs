// Le CPU de la Game Boy est un CPU a 8 bits, ce qui signifie que chacun de ses 
// registres peut contenir 8 bits.

/* ---------------------------------------------------------------------------*/

//Definition des structs

pub struct CPU {
    registers: Register,
    flags: FlagRegister,
    // to do in a separated file
    mmu: MMU,
    is_halted: bool,
    // If 1, enable interrupts ; if 2, enable interrupts after next instruction
    ei: u32,
    // If 2, disable interrputs after next instruction
    di: u32,
    emi: bool,
}


struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
}

/* ---------------------------------------------------------------------------*/

//Implementation des registres

impl Registers {

    fn new() -> Self {
        Registers {
            a: 0x01,
            b: 0xB0,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            pc: 0x0100,
            sp: 0xFFFE,
        }
    }

    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8
            | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn get_af(&self) -> u16 {
        (self.a as u16) << 8
            | self.f as u16
    }

    fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = (value & 0xFF) as u8;
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8
            | self.l as u16
    }

    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    fn get_hld(&self) -> u16 {
        let res = self.get_hl();
        self.set_hl(res - 1);
        res
    }

    fn get_hli(&mut self, value: u16) {
        let res = self.get_hl();
        self.set_hl(res + 1);
        res
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8
            | self.e as u16
    }

    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    // C flag
    fn get_carry(&self) -> bool {
        self.f & 0b00010000 != 0
    }

    fn set_carry(&self, value: bool) {
        if value {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
    }

    // H flag
    fn get_half(&self) -> bool {
        self.f & 0b00100000 != 0
    }

    fn set_half(&self, value: bool) {
        if value {
            self.f |= 0b00100000;
        } else {
            self.f &= 0b11011111;
        }
    }

    // N flag
    fn get_sub(&self) -> bool {
        self.f & 0b01000000 != 0
    }

    fn set_sub(&self, value: bool) {
        if value {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
    }

    // Z flag
    fn get_zero(&self) -> bool {
        self.f & 0b10000000 != 0
    }

    fn set_zero(&self, value: bool) {
        if value {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
    }
}

/* ---------------------------------------------------------------------------*/

//On execute les instructions en implemetant CPU

impl CPU {

    pub fn new() -> Self {
        CPU{
            registers: Register::new(),
            mmu: MMU::new(),
            is_halted: false,
            ei: 0,
            di: 0,
            emi: true,
        }
    }

    // Commuication avec mmu
    // MMU functions read_byte, read_word, write_byte, write_word, switch_speed
    fn fetchbyte(&mut self) -> u8 {
        let res = self.mmu.read_byte(self.reg.pc);
        self.reg.pc = self.registers.pc.wrapping_add(1);
        res
    }

    fn fetchword(&mut self) -> u8 {
        let res = self.mmu.read_word(self.reg.pc);
        self.reg.pc = self.registers.pc.wrapping_add(2);
        res
    }

    fn send_stop(&mut self) {
        self.mmu.receive_stop();
    }

    fn halt(&mut self) {
        self.is_halted = true;
    }

    fn pop(&mut self) -> u16 {
        let res = self.mmu.read_word(
            self.registers.sp
        );
        self.registers.sp = self.registers.sp.wrapping_add(2);
        res
    }

    fn push(&mut self, value: u16) {
        self.registers.sp = self.registers.sp.wrapping_sub(2);
        self.mmu.write_word(
            self.registers.sp,
            value
        );
    }

    fn rst(&mut self, value: u16) {
        self.push(self.registers.pc);
        self.registers.pc = value;
    }

    fn jr(&mut self) {
        // Les conversions permettent d'assurer que fetchbyte est considéré
        // comme signé, mais pas pc, que l'opérations puissnet avoir lieu, et
        // que le résutat ait le bon format
        self.registers.pc = (
            (self.registers.pc as u32 as i32) +
            (self.fetchbyte() as i8 as i32)
        ) as u16;
    }

    // Lecture des OpCodes

    // Returns the duration in cycles
    fn receiveOp(&mut self) -> u32 {
        let op = self.fetchbyte();
        match op {
            // NOP
            0x00 => {
                4
            },
            // LD BC, d16
            0x01 => {
                self.registers.set_bc(
                    self.fetchword()
                );
                12
            },
            // LD (BC), A
            0x02 => {
                self.mmu.write_byte(
                    self.registers.get_bc(),
                    self.registers.a
                );
                8
            },
            // INC BC
            0x03 => {
                self.registers.set_bc(
                    self.registers.get_bc().wrapping_add(1)
                );
                8
            },
            // INC B
            0x04 => {
                self.registers.b = self.inc(self.registers.b);
                4
            },
            // DEC B
            0x05 => {
                self.registers.b = self.dec(self.registers.b);
                4
            },
            // LD B, d8
            0x06 => {
                self.registers.b = self.fetchbyte();
                8
            },
            // RLCA
            0x07 => {
                self.registers.a = self.rlc(self.registers.a);
                self.registers.set_zero(false);
                4
            },
            // LD (a16), SP
            0x08 => {
                self.mmu.write_word(
                    self.fetchword(),
                    self.registers.sp
                );
                20
            },
            // ADD HL, BC
            0x09 => {
                self.addhl(self.registers.get_bc());
                8
            },
            // LD A, (BC)
            0x0A => {
                self.registers.a = self.mmu.read_byte(self.registers.get_bc());
                8
            },
            // DEC BC
            0x0B => {
                self.registers.set_bc(self.registers.get_bc().wrapping_sub(1));
                8
            },
            // INC C
            0x0C => {
                self.registers.c = self.inc(self.registers.c);
                4
            },
            // DEC C
            0x0D => {
                self.registers.c = self.dec(self.registers.c);
                4
            },
            // LD C, d8
            0x0E => {
                self.registers.c = self.fetchbyte();
                8
            },
            // RRCA
            0x0F => {
                self.registers.a = self.rrc(self.registers.a);
                self.registers.set_zero(false);
                4
            },
            // STOP A
            0x10 => {
                self.send_stop();
                4
            },
            // LD DE, D16
            0x11 => {
                self.registers.set_de(
                    self.fetchword()
                );
                12
            },
            // LD (DE), A
            0x12 => {
                self.mmu.write_byte(
                    self.registers.get_de(),
                    self.registers.a
                );
                8
            },
            // INC DE
            0x13 => {
                self.registers.set_de(
                    self.registers.get_de().wrapping_add(1)
                );
                8
            },
            // INC D
            0x14 => {
                self.registers.d = self.inc(self.registers.d);
                4
            },
            // DEC D
            0x15 => {
                self.registers.d = self.dec(self.registers.d);
                4
            ),
            // LD D, d8
            0x16 => {
                self.registers.d = self.fetchbyte();
                8
            },
            // RLA
            0x17 => {
                self.registers.a = self.rl(self.registers.a);
                self.registers.set_zero(false);
                4
            },
            // JR r8
            0x18 => {
                self.jr();
                12
            },
            // ADD HL, DE
            0x19 => {
                self.registers.addhl(
                    self.registers.get_de()
                );
                8
               },
            // LD A, (DE)
            0x1A => {
                self.registers.a = self.mmu.read_byte(
                    self.registers.get_de()
                );
                8
            },
            // DEC DE
            0x1B => {
                self.registers.set_de(
                    self.registers.get_de().wrapping_sub(1)
                );
                8
            },
            // INC E
            0x1C => {
                self.registers.e = self.inc(self.registers.e);
                4
            },
            // DEC E
            0x1D => {
                self.registers.e = self.dec(self.registers.e);
                4
            },
            // LD D, d8
            0x1E => {
                self.registers.d = self.fetchbyte();
                8
            },
            // RRA
            0x1F => {
                self.registers.a = self.rr(self.registers.a);
                self.registers.set_zero(false);
                4
            },
            // JR NZ, r8
            0x20 => {
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
                self.registers.set_hl(
                    self.fetchword();
                );
                12
            },
            // LD (HL+), A
            0x22 => {
                self.mmu.write_byte(
                    self.registers.get_hli(),
                    self.registers.a
                );
                8
            },
            // INC HL
            0x23 => {
                self.registers.set_hl(
                    self.registers.get_hl().wrapping_add(1)
                );
                8
            },
            // INC H
            0x24 => {
                self.registers.h = self.inc(self.registers.h);
                4
            },
            // DEC H
            0x25 => {
                self.registers.h = self.dec(self.registers.h);
                4
            },
            // LD H, d8
            0x26 => {
                self.registers.h = self.fetchbyte();
                8
            },
            // DAA
            0x27 => {
                self.daa();
                4
            },
            // JR Z, r8
            0x28 => {
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
                self.addhl(self.registers.get_hl());
                8
            },
            // LD A, (HL+)
            0x2A => {
                self.mmu.write_byte(
                    self.registers.a,
                    self.registers.get_hli()
                );
                8
            },
            // DEC HL
            0x2B => {
                self.registers.set_hl(
                    self.registers.get_hl().wrapping_sub(1)
                );
                8
            },
            // INC L
            0x2C => {
                self.registers.l = self.inc(self.registers.l);
                4
            },
            // DEC L
            0x2D => {
                self.registers.l = self.dec(self.registers.l);
                4
            },
            // LD L, d8
            0x2E => {
                self.registers.l = self.fetchbyte();
                8
            },
            // CPL
            0x2F => {
                self.registers.a = self.registers.a;
                self.registers.set_half(true);
                self.registers.set_sub(true);
                4
            },
            // JR NC, r8
            0x30 => {
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
                self.registers.sp = self.fetchword();
                12
            },
            // LD (HL-), A
            0x32 => {
                self.mmu.write_byte(
                    self.registers.get_hld(),
                    self.registers.a
                );
                8
            },
            // INC SP
            0x33 => {
                self.registers.sp = self.registers.sp.wrapping_add(1);
                8
            },
            // INC (HL)
            0x34 => {
                let value = self.mmu.read_byte(
                    self.registers.get_hl()
                )
                self.mmu.write_byte(
                    value,
                    self.inc(
                        value
                    )
                );
                12
            },
            // DEC (HL)
            0x35 => {
                let value = self.mmu.read_byte(
                    self.registers.get_hl()
                )
                self.mmu.write_byte(
                    value,
                    self.dec(
                        value
                    )
                );
                12
            },
            // LD (HL), d8
            0x36 => {
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    self.fetchbyte()
                );
                12
            },
            // SCF
            0x37 => {
                self.registers.set_carry(true);
                self.registers.set_half(false);
                self.registers.set_sub(true);
                4
            },
            // JR C, r8
            0x38 => {
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
                self.addhl(
                    self.registers.sp
                );
                8
            },
            // LD A, (HL-)
            0x3A => {
                self.mmu.write_byte(
                    self.registers.a,
                    self.registers.get_hld()
                );
                8
            },
            // DEC SP
            0x3B => {
                self.registers.sp = self.registers.sp.wrapping_sub(1);
                8
            },
            // INC A
            0x3C => {
                self.registers.a = self.inc(
                    self.regisers.a
                );
                4
            },
            // DEC A
            0x3D => {
                self.registers.a = self.dec(
                    self.regisers.a
                );
                4
            },
            // LD A, d8
            0x3E => {
                self.registers.a = self.fetchbyte();
                8
            },
            // CCF
            0x3F => {
                self.registers.set_carry(
                    !self.registers.get_carry()
                );
                self.registers.set_half(
                    false
                );
                self.registers.get_sub(
                    false
                );
            },
            // LD B, B
            0x40 => {
                self.registers.b = self.registers.b;
                4
            },
            // LD B, C
            0x41 => {
                self.registers.b = self.registers.c;
                4
            },
            // LD B, D
            0x42 => {
                self.registers.b = self.registers.d;
                4
            },
            // LD B, E
            0x43 => {
                self.registers.b = self.registers.e;
                4
            },
            // LD B, H
            0x44 => {
                self.registers.b = self.registers.h;
                4
            },
            // LD B, L
            0x45 => {
                self.registers.b = self.registers.l;
                4
            },
            // LD B, (HL)
            0x46 => {
                self.registers.b = self.mmu.read_byte(
                    self.registers.get_hl()
                );
                8
            },
            // LD B, A
            0x47 => {
                self.registers.b = self.registers.a;
                4
            },
            // LD C, B
            0x48 => {
                self.registers.c = self.registers.b;
                4
            },
            // LC C, C
            0x49 => {
                self.registers.c = self.registers.c;
                4
            },
            // LC C, D
            0x4A => {
                self.registers.c = self.registers.d;
                4
            },
            // LD C, E
            0x4B => {
                self.registers.c = self.registers.e;
                4
            },
            // LD C, H
            0x4C => {
                self.registers.c = self.registers.h;
                4
            },
            // LD C, L
            0x4D => {
                self.registers.c = self.registers.l;
                4
            },
            // LD C, (HL)
            0x4E => {
                self.registers.c = self.mmu.read_byte(
                    self.registers.get_hl()
                );
                8
            },
            // LD C, A
            0x4F => {
                self.registers.c = self.registers.a;
                4
            },
            // LD D, B
            0x50 => {
                self.registers.d = self.registers.b;
                4
            },
            // LD D, C
            0x51 => {
                self.registers.d = self.registers.c;
                4
            },
            // LD D, D
            0x52 => {
                self.registers.d = self.registers.d;
                4
            },
            // LD D, E
            0x53 => {
                self.registers.d = self.registers.e;
                4
            },
            // LD D, H
            0x54 => {
                self.registers.d = self.registers.h;
                4
            },
            // LD D, L
            0x55 => {
                self.registers.d = self.registers.l;
                4
            },
            // LD D, (HL)
            0x56 => {
                self.registers.d = self.mmu.read_byte(
                    self.registers.get_hl()
                );
                8
            },
            // LD D, A
            0x57 => {
                self.registers.d = self.registers.a;
                4
            },
            // LD E, B
            0x58 => {
                self.registers.e = self.registers.b;
                4
            },
            // LC E, C
            0x59 => {
                self.registers.e = self.registers.c;
                4
            },
            // LC E, D
            0x5A => {
                self.registers.e = self.registers.d;
                4
            },
            // LD E, E
            0x5B => {
                self.registers.e = self.registers.e;
                4
            },
            // LD E, H
            0x5C => {
                self.registers.e = self.registers.h;
                4
            },
            // LD E, L
            0x5D => {
                self.registers.e = self.registers.l;
                4
            },
            // LD E, (HL)
            0x5E => {
                self.registers.e = self.mmu.read_byte(
                    self.registers.get_hl()
                );
                8
            },
            // LD E, A
            0x5F => {
                self.registers.e = self.registers.a;
                4
            },
            // LD H, B
            0x60 => {
                self.registers.h = self.registers.b;
                4
            },
            // LD H, C
            0x61 => {
                self.registers.h = self.registers.c;
                4
            },
            // LD H, D
            0x62 => {
                self.registers.h = self.registers.d;
                4
            },
            // LD H, E
            0x63 => {
                self.registers.h = self.registers.e;
                4
            },
            // LD H, H
            0x64 => {
                self.registers.h = self.registers.h;
                4
            },
            // LD H, L
            0x65 => {
                self.registers.h = self.registers.l;
                4
            },
            // LD H, (HL)
            0x66 => {
                self.registers.h = self.mmu.read_byte(
                    self.registers.get_hl()
                );
                8
            },
            // LD H, A
            0x67 => {
                self.registers.h = self.registers.a;
                4
            },
            // LD L, B
            0x68 => {
                self.registers.l = self.registers.b;
                4
            },
            // LC L, C
            0x69 => {
                self.registers.l = self.registers.c;
                4
            },
            // LC L, D
            0x6A => {
                self.registers.l = self.registers.d;
                4
            },
            // LD L, E
            0x6B => {
                self.registers.l = self.registers.e;
                4
            },
            // LD L, H
            0x6C => {
                self.registers.l = self.registers.h;
                4
            },
            // LD L, L
            0x6D => {
                self.registers.l = self.registers.l;
                4
            },
            // LD L, (HL)
            0x6E => {
                self.registers.l = self.mmu.read_byte(
                    self.registers.get_hl()
                );
                8
            },
            // LD L, A
            0x6F => {
                self.registers.l = self.registers.a;
                4
            },
            // LD (HL), B
            0x70 => {
                self.mmu.write_byte(
                    registers.get_hl(),
                    self.registers.b
                );
                8
            },
            // LD (HL), C
            0x71 => {
                self.mmu.write_byte(
                    registers.get_hl(),
                    self.registers.c
                );
                8
            },
            // LD (HL), D
            0x72 => {
                self.mmu.write_byte(
                    registers.get_hl(),
                    self.registers.d
                );
                8
            },
            // LD (HL), E
            0x73 => {
                self.mmu.write_byte(
                    registers.get_hl(),
                    self.registers.e
                );
                8
            },
            // LD (HL), H
            0x74 => {
                self.mmu.write_byte(
                    registers.get_hl(),
                    self.registers.h
                );
                8
            },
            // LD (HL), L
            0x75 => {
                self.mmu.write_byte(
                    registers.get_hl(),
                    self.registers.l
                );
                8
            },
            // HALT
            0x76 => {
                self.halt();
                4
            },
            // LD (HL), A
            0x77 => {
                self.mmu.write_byte(
                    registers.get_hl(),
                    self.registers.a
                );
                8
            },
            // LD A, B
            0x78 => {
                self.registers.a = self.registers.b;
                4
            },
            // LC A, C
            0x79 => {
                self.registers.a = self.registers.c;
                4
            },
            // LC A, D
            0x7A => {
                self.registers.a = self.registers.d;
                4
            },
            // LD A, E
            0x7B => {
                self.registers.a = self.registers.e;
                4
            },
            // LD A, H
            0x7C => {
                self.registers.a = self.registers.h;
                4
            },
            // LD A, L
            0x7D => {
                self.registers.a = self.registers.l;
                4
            },
            // LD A, (HL)
            0x7E => {
                self.registers.a = self.mmu.read_byte(
                    self.registers.get_hl()
                );
                8
            },
            // LD A, A
            0x7F => {
                self.registers.a = self.registers.a;
                4
            },
            // ADD A, B
            0x80 => {
                self.add(
                    self.registers.b
                );
                4
            },
            // ADD A, C
            0x81 => {
                self.add(
                    self.registers.c
                );
                4
            },
            // ADD A, D
            0x82 => {
                self.add(
                    self.registers.d
                );
                4
            },
            // ADD A, E
            0x83 => {
                self.add(
                    self.registers.e
                );
                4
            },
            // ADD A, H
            0x84 => {
                self.add(
                    self.registers.h
                );
                4
            },
            // ADD A, L
            0x85 => {
                self.add(
                    self.registers.l
                );
                4
            },
            // ADD A, (HL)
            0x86 => {
                self.add(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                8
            },
            // ADD A, A
            0x87 => {
                self.add(
                    self.registers.a
                );
                4
            },
            // ADC A, B
            0x88 => {
                self.adc(
                    self.registers.b
                );
                4
            },
            // ADC A, C
            0x89 => {
                self.adc(
                    self.registers.c
                );
                4
            },
            // ADC A, D
            0x8A => {
                self.adc(
                    self.registers.d
                );
                4
            },
            // ADC A, E
            0x8B => {
                self.adc(
                    self.registers.e
                );
                4
            },
            // ADC A, H
            0x8C => {
                self.adc(
                    self.registers.h
                );
                4
            },
            // ADC A, L
            0x8D => {
                self.adc(
                    self.registers.l
                );
                4
            },
            // ADC A, (HL)
            0x8E => {
                self.adc(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                8
            },
            // ADC A, A
            0x8F => {
                self.adc(
                    self.registers.a
                );
                4
            },
            // SUB A, B
            0x90 => {
                self.sub(
                    self.registers.b
                );
                4
            },
            // SUB A, C
            0x91 => {
                self.sub(
                    self.registers.c
                );
                4
            },
            // SUB A, D
            0x92 => {
                self.sub(
                    self.registers.d
                );
                4
            },
            // SUB A, E
            0x93 => {
                self.sub(
                    self.registers.e
                );
                4
            },
            // SUB A, H
            0x94 => {
                self.sub(
                    self.registers.h
                );
                4
            },
            // SUB A, L
            0x95 => {
                self.sub(
                    self.registers.l
                );
                4
            },
            // SUB A, (HL)
            0x96 => {
                self.sub(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                8
            },
            // SUB A, A
            0x97 => {
                self.sub(
                    self.registers.a
                );
                4
            },
            // SBC A, B
            0x98 => {
                self.sbc(
                    self.registers.b
                );
                4
            },
            // SBC A, C
            0x99 => {
                self.sbc(
                    self.registers.c
                );
                4
            },
            // SBC A, D
            0x9A => {
                self.sbc(
                    self.registers.d
                );
                4
            },
            // SBC A, E
            0x9B => {
                self.sbc(
                    self.registers.e
                );
                4
            },
            // SBC A, H
            0x9C => {
                self.sbc(
                    self.registers.h
                );
                4
            },
            // SBC A, L
            0x9D => {
                self.sbc(
                    self.registers.l
                );
                4
            },
            // SBC A, (HL)
            0x9E => {
                self.sbc(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                8
            },
            // SBC A, A
            0x9F => {
                self.sbc(
                    self.registers.a
                );
                4
            },
            // AND A, B
            0xA0 => {
                self.and(
                    self.registers.b
                );
                4
            },
            // AND A, C
            0xA1 => {
                self.and(
                    self.registers.c
                );
                4
            },
            // AND A, D
            0xA2 => {
                self.and(
                    self.registers.d
                );
                4
            },
            // AND A, E
            0xA3 => {
                self.and(
                    self.registers.e
                );
                4
            },
            // AND A, H
            0xA4 => {
                self.and(
                    self.registers.h
                );
                4
            },
            // AND A, L
            0xA5 => {
                self.and(
                    self.registers.l
                );
                4
            },
            // AND A, (HL)
            0xA6 => {
                self.and(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                8
            },
            // AND A, A
            0xA7 => {
                self.and(
                    self.registers.a
                );
                4
            },
            // XOR A, B
            0xA8 => {
                self.xor(
                    self.registers.b
                );
                4
            },
            // XOR A, C
            0xA9 => {
                self.xor(
                    self.registers.c
                );
                4
            },
            // XOR A, D
            0xAA => {
                self.xor(
                    self.registers.d
                );
                4
            },
            // XOR A, E
            0xAB => {
                self.xor(
                    self.registers.e
                );
                4
            },
            // XOR A, H
            0xAC => {
                self.xor(
                    self.registers.h
                );
                4
            },
            // XOR A, L
            0xAD => {
                self.xor(
                    self.registers.l
                );
                4
            },
            // XOR A, (HL)
            0xAE => {
                self.xor(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                8
            },
            // XOR A, A
            0xAF => {
                self.xor(
                    self.registers.a
                );
                4
            },
            // OR A, B
            0xB0 => {
                self.or(
                    self.registers.b
                );
                4
            },
            // OR A, C
            0xB1 => {
                self.or(
                    self.registers.c
                );
                4
            },
            // OR A, D
            0xB2 => {
                self.or(
                    self.registers.d
                );
                4
            },
            // OR A, E
            0xB3 => {
                self.or(
                    self.registers.e
                );
                4
            },
            // OR A, H
            0xB4 => {
                self.or(
                    self.registers.h
                );
                4
            },
            // OR A, L
            0xB5 => {
                self.or(
                    self.registers.l
                );
                4
            },
            // OR A, (HL)
            0xB6 => {
                self.or(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                8
            },
            // OR A, A
            0xB7 => {
                self.or(
                    self.registers.a
                );
                4
            },
            // CP A, B
            0xB8 => {
                self.cp(
                    self.registers.b
                );
                4
            },
            // CP A, C
            0xB9 => {
                self.cp(
                    self.registers.c
                );
                4
            },
            // CP A, D
            0xBA => {
                self.cp(
                    self.registers.d
                );
                4
            },
            // CP A, E
            0xBB => {
                self.cp(
                    self.registers.e
                );
                4
            },
            // CP A, H
            0xBC => {
                self.cp(
                    self.registers.h
                );
                4
            },
            // CP A, L
            0xBD => {
                self.cp(
                    self.registers.l
                );
                4
            },
            // CP A, (HL)
            0xBE => {
                self.cp(
                    self.mmu.read_byte(
                        self.registers.get_hl()
                    )
                );
                8
            },
            // CP A, A
            0xBF => {
                self.cp(
                    self.registers.a
                );
                4
            },
            // RET NZ
            0xC0 => {
                if !self.registers.get_zero() {
                    self.registers.pc = self.fetchword;
                    20
                } else {
                    8
                }
            },
            // POP BC
            0xC1 => {
                self.registers.set_bc(
                    self.pop()
                );
                12
            },
            // JP NZ, a16
            0xC2 => {
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
                self.registers.pc = self.fetchword();
                16
            },
            // CALL NZ, a16
            0xC4 => {
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
                self.push(
                    self.registers.get_bc()
                );
                16
            },
            // ADD A, d8
            0xC6 => {
                self.add(
                    self.fetchbyte()
                );
                8
            },
            // RST 00H
            0xC7 => {
                self.rst(0x0000);
                16
            },
            // RET Z
            0xC8 => {
                if self.registers.get_zero() {
                    self.registers.pc = self.fetchword;
                    20
                } else {
                    8
                }
            },
            // RET
            0xC9 => {
                self.registers.pc = self.fetchword;
                16
            },
            // JP Z, a16
            0xCA => {
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
                self.call_cb(self.fetchbyte())
            },
            // CALL Z, a16
            0xCC => {
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
                self.push(
                    self.registers.pc + 2
                );
                self.registers.pc = self.fetchword();
                24
            },
            // ADC A, d8
            0xCE => {
                self.adc(
                    self.fetchbyte()
                );
                8
            },
            // RST 08H
            0xCF => {
                self.rst(0x0080);
                16
            },
            // RET NC
            0xD0 => {
                if !self.registers.get_carry() {
                    self.registers.pc = self.fetchword;
                    20
                } else {
                    8
                }
            },
            // POP DE
            0xD1 => {
                self.registers.set_de(
                    self.pop()
                );
                12
            },
            // JP NC, a16
            0xD2 => {
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
                self.push(
                    self.registers.get_de()
                );
                16
            },
            // SUB d8
            0xD6 => {
                self.sub(
                    self.fetchbyte()
                );
                8
            },
            // RST 10H
            0xD7 => {
                self.rst(0x0010);
                16
            },
            // RET C
            0xD8 => {
                if self.registers.get_carry() {
                    self.registers.pc = self.fetchword;
                    20
                } else {
                    8
                }
            },
            // RETI
            0xD9 => {
                self.registers.pc = self.pop();
                self.ei = 1;
                self.di = 0;
                16
            },
            // JP C, a16
            0xDA => {
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
                self.sbc(
                    self.fetchbyte()
                );
                8
            },
            // RST 18H
            0xDF => {
                self.rst(0x0018);
                16
            },
            // LDH (a8), A
            0xE0 => {
                self.mmu.write_byte(
                    0xFF00 | self.fetchbyte() as u16,
                    self.registers.a
                );
                12
            },
            // POP HL
            0xE1 => {
                self.registers.set_hl(
                    self.pop()
                );
                12
            },
            // LDH (C), A
            0xE2 => {
                self.mmu.write_byte(
                    0xFF00 | self.registers.c as u16,
                    self.registers.a
                );
                8
            },
            // PUSH HL
            0xE5 => {
                self.push(
                    self.registers.get_hl()
                );
                16
            },
            // AND d8
            0xE6 => {
                self.and(
                    self.fetchbyte()
                );
                8
            },
            // RST 20H
            0xE7 => {
                self.rst(0x0020);
                16
            },
            // ADD SP, r8
            0xE8 => {
                self.registers.sp = self.addr8(SP);
                16
            },
            // JP (HL)
            0xE9 => {
                self.registers.pc = self.registers.get_hl();
                4
            },
            // LD (a16), A
            0xEA => {
                self.mmu.write_byte(
                    self.fetchword(),
                    self.registers.a
                );
                16
            },
            // XOR d8
            0xEE => {
                self.xor(
                    self.fetchbyte()
                );
                8
            },
            // RST 28H
            0xEF => {
                self.rst(0x0028);
                16
            },
            // LDH A, (a8)
            0xF0 => {
                self.registers.a = self.mmu.read_byte(
                    0xFF00 | self.fetchbyte() as u16
                );
                12
            },
            // POP AF
            0xF1 => {
                self.registers.set_af(
                    self.pop() & 0xFFF0
                );
                12
            },
            // LD A, (C)
            0xF2 => {
                self.registers.a = self.mmu.read_byte(
                    0xFF00 | self.registers.c as u16
                );
                8
            },
            // DI
            0xF3 => {
                self.di = 2;
                // Cancel any scheduled effects of the ei instruction
                self.ei = 0;
                4
            },
            // PUSH AF
            0xF5 => {
                self.push(
                    self.registers.get_af()
                );
                16
            },
            // OR d8
            0xF6 => {
                self.or(
                    self.fetchbyte()
                );
                8
            },
            // RST 30H
            0xF7 => {
                self.rst(0x0030);
                16
            },
            // LD HL, SP+r8
            0xF8 => {
                self.registers.set_hl(
                    self.addr8(self.registers.sp)
                );
                12
            },
            // LD SP, HL
            0xF9 => {
                self.registers.sp = self.registers.get_hl();
                8
            },
            // LD A, (a16)
            0xFA => {
                self.registers.a = self.mmu.read_byte(
                    self.fetchword()
                );
                12
            },
            // EI
            0xFB => {
                self.ei = 2;
                4
            },
            // CP d8
            0xFE => {
                self.cp(
                    self.fetchbyte()
                );
                8
            },
            // RST 38H
            0xFF => {
                self.rst(0x0038);
                16
            },
            // Si code non trouvé
            _ => {
                panic!("OpCode not found");
                0
            }
        }
    }

    fn call_cb(&mut self) -> u32 {
        let op = self.fetchbyte();
        match op {
            // RLC B
            0x00 => {
                self.registers.b = self.rlc(
                    self.registers.b
                );
                8
            },
            // RLC C
            0x01 => {
                self.registers.c = self.rlc(
                    self.registers.c
                );
                8
            },
            // RLC D
            0x02 => {
                self.registers.d = self.rlc(
                    self.registers.d
                );
                8
            },
            // RLC E
            0x03 => {
                self.registers.e = self.rlc(
                    self.registers.e
                );
                8
            },
            // RLC H
            0x04 => {
                self.registers.h = self.rlc(
                    self.registers.h
                );
                8
            },
            // RLC L
            0x05 => {
                self.registers.l = self.rlc(
                    self.registers.l
                );
                8
            },
            // RLC (HL)
            0x06 => {
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    self.rlc(
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // RLC A
            0x07 => {
                self.registers.a = self.rlc(
                    self.registers.a
                );
                8
            },
            // RRC B
            0x08 => {
                self.registers.b = self.rrc(
                    self.registers.b
                );
                8
            },
            // RRC C
            0x09 => {
                self.registers.c = self.rrc(
                    self.registers.c
                );
                8
            },
            // RRC D
            0x0A => {
                self.registers.d = self.rrc(
                    self.registers.d
                );
                8
            },
            // RRC E
            0x0B => {
                self.registers.e = self.rrc(
                    self.registers.e
                );
                8
            },
            // RRC H
            0x0C => {
                self.registers.h = self.rrc(
                    self.registers.h
                );
                8
            },
            // RRC L
            0x0D => {
                self.registers.l = self.rrc(
                    self.registers.l
                );
                8
            },
            // RRC (HL)
            0x0E => {
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    self.rrc(
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // RRC A
            0x0F => {
                self.registers.a = self.rrc(
                    self.registers.a
                );
                8
            },
            // RL B
            0x10 => {
                self.registers.b = self.rl(
                    self.registers.b
                );
                8
            },
            // RL C
            0x11 => {
                self.registers.c = self.rl(
                    self.registers.c
                );
                8
            },
            // RL D
            0x12 => {
                self.registers.d = self.rl(
                    self.registers.d
                );
                8
            },
            // RC E
            0x13 => {
                self.registers.e = self.rl(
                    self.registers.e
                );
                8
            },
            // RC H
            0x14 => {
                self.registers.h = self.rl(
                    self.registers.h
                );
                8
            },
            // RL L
            0x15 => {
                self.registers.l = self.rl(
                    self.registers.l
                );
                8
            },
            // RL (HL)
            0x16 => {
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    self.rl(
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // RL A
            0x17 => {
                self.registers.a = self.rl(
                    self.registers.a
                );
                8
            },
            // RR B
            0x18 => {
                self.registers.b = self.rr(
                    self.registers.b
                );
                8
            },
            // RR C
            0x19 => {
                self.registers.c = self.rr(
                    self.registers.c
                );
                8
            },
            // RR D
            0x1A => {
                self.registers.d = self.rr(
                    self.registers.d
                );
                8
            },
            // RR E
            0x1B => {
                self.registers.e = self.rr(
                    self.registers.e
                );
                8
            },
            // RR H
            0x1C => {
                self.registers.h = self.rr(
                    self.registers.h
                );
                8
            },
            // RR L
            0x1D => {
                self.registers.l = self.rr(
                    self.registers.l
                );
                8
            },
            // RR (HL)
            0x1E => {
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    self.rr(
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // RR A
            0x1F => {
                self.registers.a = self.rr(
                    self.registers.a
                );
                8
            },
            // SLA B
            0x20 => {
                self.registers.b = self.sla(
                    self.registers.b
                );
                8
            },
            // SLA C
            0x21 => {
                self.registers.c = self.sla(
                    self.registers.c
                );
                8
            },
            // SLA D
            0x22 => {
                self.registers.d = self.sla(
                    self.registers.d
                );
                8
            },
            // RC E
            0x23 => {
                self.registers.e = self.sla(
                    self.registers.e
                );
                8
            },
            // RC H
            0x24 => {
                self.registers.h = self.sla(
                    self.registers.h
                );
                8
            },
            // SLA L
            0x25 => {
                self.registers.l = self.sla(
                    self.registers.l
                );
                8
            },
            // SLA (HL)
            0x26 => {
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    self.sla(
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // SLA A
            0x27 => {
                self.registers.a = self.sla(
                    self.registers.a
                );
                8
            },
            // SRA B
            0x28 => {
                self.registers.b = self.sra
                    self.registers.b
                );
                8
            },
            // SRA C
            0x29 => {
                self.registers.c = self.sra
                    self.registers.c
                );
                8
            },
            // SRA D
            0x2A => {
                self.registers.d = self.sra
                    self.registers.d
                );
                8
            },
            // SRA E
            0x2B => {
                self.registers.e = self.sra
                    self.registers.e
                );
                8
            },
            // SRA H
            0x2C => {
                self.registers.h = self.sra
                    self.registers.h
                );
                8
            },
            // SRA L
            0x2D => {
                self.registers.l = self.sra
                    self.registers.l
                );
                8
            },
            // SRA (HL)
            0x2E => {
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    self.sra(
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // SRA A
            0x2F => {
                self.registers.a = self.sra
                    self.registers.a
                );
                8
            },
            // SWAP B
            0x30 => {
                self.registers.b = self.swap(
                    self.registers.b
                );
                8
            },
            // SWAP C
            0x31 => {
                self.registers.c = self.swap(
                    self.registers.c
                );
                8
            },
            // SWAP D
            0x32 => {
                self.registers.d = self.swap(
                    self.registers.d
                );
                8
            },
            // RC E
            0x33 => {
                self.registers.e = self.swap(
                    self.registers.e
                );
                8
            },
            // RC H
            0x34 => {
                self.registers.h = self.swap(
                    self.registers.h
                );
                8
            },
            // SWAP L
            0x35 => {
                self.registers.l = self.swap(
                    self.registers.l
                );
                8
            },
            // SWAP (HL)
            0x36 => {
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    self.swap(
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // SWAP A
            0x37 => {
                self.registers.a = self.swap(
                    self.registers.a
                );
                8
            },
            // SRL B
            0x38 => {
                self.registers.b = self.srl
                    self.registers.b
                );
                8
            },
            // SRL C
            0x39 => {
                self.registers.c = self.srl
                    self.registers.c
                );
                8
            },
            // SRL D
            0x3A => {
                self.registers.d = self.srl
                    self.registers.d
                );
                8
            },
            // SRL E
            0x3B => {
                self.registers.e = self.srl
                    self.registers.e
                );
                8
            },
            // SRL H
            0x3C => {
                self.registers.h = self.srl
                    self.registers.h
                );
                8
            },
            // SRL L
            0x3D => {
                self.registers.l = self.srl
                    self.registers.l
                );
                8
            },
            // SRL (HL)
            0x3E => {
                self.mmu.write_byte(
                    self.registers.get_hl(),
                    self.srl(
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // SRL A
            0x3F => {
                self.registers.a = self.srl
                    self.registers.a
                );
                8
            },
            // BIT 0, B
            0x40 => {
                self.bit(0, self.registers.b);
                8
            },
            // BIT 0, C
            0x41 => {
                self.bit(0, self.registers.c);
                8
            },
            // BIT 0, D
            0x42 => {
                self.bit(0, self.registers.d);
                8
            },
            // BIT 0, E
            0x43 => {
                self.bit(0, self.registers.e);
                8
            },
            // BIT 0, H
            0x44 => {
                self.bit(0, self.registers.h);
                8
            },
            // BIT 0, L
            0x45 => {
                self.bit(0, self.registers.l);
                8
            },
            // BIT 0, (HL)
            0x46 => {
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
                self.bit(0, self.registers.a);
                8
            },
            // BIT 1, B
            0x48 => {
                self.bit(1, self.registers.b);
                8
            },
            // BIT 1, C
            0x49 => {
                self.bit(1, self.registers.c);
                8
            },
            // BIT 1, D
            0x4A => {
                self.bit(1, self.registers.d);
                8
            },
            // BIT 1, E
            0x4B => {
                self.bit(1, self.registers.e);
                8
            },
            // BIT 1, H
            0x4C => {
                self.bit(1, self.registers.h);
                8
            },
            // BIT 1, L
            0x4D => {
                self.bit(1, self.registers.l);
                8
            },
            // BIT 1, (HL)
            0x4E => {
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
                self.bit(1, self.registers.a);
                8
            },
            // BIT 2, B
            0x50 => {
                self.bit(2, self.registers.b);
                8
            },
            // BIT 2, C
            0x51 => {
                self.bit(2, self.registers.c);
                8
            },
            // BIT 2, D
            0x52 => {
                self.bit(2, self.registers.d);
                8
            },
            // BIT 2, E
            0x53 => {
                self.bit(2, self.registers.e);
                8
            },
            // BIT 2, H
            0x54 => {
                self.bit(2, self.registers.h);
                8
            },
            // BIT 2, L
            0x55 => {
                self.bit(2, self.registers.l);
                8
            },
            // BIT 2, (HL)
            0x56 => {
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
                self.bit(2, self.registers.a);
                8
            },
            // BIT 3, B
            0x58 => {
                self.bit(3, self.registers.b);
                8
            },
            // BIT 3, C
            0x59 => {
                self.bit(3, self.registers.c);
                8
            },
            // BIT 3, D
            0x5A => {
                self.bit(3, self.registers.d);
                8
            },
            // BIT 3, E
            0x5B => {
                self.bit(3, self.registers.e);
                8
            },
            // BIT 3, H
            0x5C => {
                self.bit(3, self.registers.h);
                8
            },
            // BIT 3, L
            0x5D => {
                self.bit(3, self.registers.l);
                8
            },
            // BIT 3, (HL)
            0x5E => {
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
                self.bit(3, self.registers.a);
                8
            },
            // BIT 4, B
            0x60 => {
                self.bit(4, self.registers.b);
                8
            },
            // BIT 4, C
            0x61 => {
                self.bit(4, self.registers.c);
                8
            },
            // BIT 4, D
            0x62 => {
                self.bit(4, self.registers.d);
                8
            },
            // BIT 4, E
            0x63 => {
                self.bit(4, self.registers.e);
                8
            },
            // BIT 4, H
            0x64 => {
                self.bit(4, self.registers.h);
                8
            },
            // BIT 4, L
            0x65 => {
                self.bit(4, self.registers.l);
                8
            },
            // BIT 4, (HL)
            0x66 => {
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
                self.bit(4, self.registers.a);
                8
            },
            // BIT 5, B
            0x68 => {
                self.bit(5, self.registers.b);
                8
            },
            // BIT 5, C
            0x69 => {
                self.bit(5, self.registers.c);
                8
            },
            // BIT 5, D
            0x6A => {
                self.bit(5, self.registers.d);
                8
            },
            // BIT 5, E
            0x6B => {
                self.bit(5, self.registers.e);
                8
            },
            // BIT 5, H
            0x6C => {
                self.bit(5, self.registers.h);
                8
            },
            // BIT 5, L
            0x6D => {
                self.bit(5, self.registers.l);
                8
            },
            // BIT 5, (HL)
            0x6E => {
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
                self.bit(5, self.registers.a);
                8
            },
            // BIT 6, B
            0x70 => {
                self.bit(6, self.registers.b);
                8
            },
            // BIT 6, C
            0x71 => {
                self.bit(6, self.registers.c);
                8
            },
            // BIT 6, D
            0x72 => {
                self.bit(6, self.registers.d);
                8
            },
            // BIT 6, E
            0x73 => {
                self.bit(6, self.registers.e);
                8
            },
            // BIT 6, H
            0x74 => {
                self.bit(6, self.registers.h);
                8
            },
            // BIT 6, L
            0x75 => {
                self.bit(6, self.registers.l);
                8
            },
            // BIT 6, (HL)
            0x76 => {
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
                self.bit(6, self.registers.a);
                8
            },
            // BIT 7, B
            0x78 => {
                self.bit(7, self.registers.b);
                8
            },
            // BIT 7, C
            0x79 => {
                self.bit(7, self.registers.c);
                8
            },
            // BIT 7, D
            0x7A => {
                self.bit(7, self.registers.d);
                8
            },
            // BIT 7, E
            0x7B => {
                self.bit(7, self.registers.e);
                8
            },
            // BIT 7, H
            0x7C => {
                self.bit(7, self.registers.h);
                8
            },
            // BIT 7, L
            0x7D => {
                self.bit(7, self.registers.l);
                8
            },
            // BIT 7, (HL)
            0x7E => {
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
                self.bit(7, self.registers.a);
                8
            },
            // RES 0, B
            0x80 => {
                self.registers.b = self.res(0, self.registers.b);
                8
            },
            // RES 0, C
            0x81 => {
                self.registers.c = self.res(0, self.registers.c);
                8
            },
            // RES 0, D
            0x82 => {
                self.registers.d = self.res(0, self.registers.d);
                8
            },
            // RES 0, E
            0x83 => {
                self.registers.e = self.res(0, self.registers.e);
                8
            },
            // RES 0, H
            0x84 => {
                self.registers.e = self.res(0, self.registers.h);
                8
            },
            // RES 0, L
            0x85 => {
                self.registers.l = self.res(0, self.registers.l);
                8
            },
            // RES 0, (HL)
            0x86 => {
                self.registers.set_hl(
                    self.res(
                        0,
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // RES 0, A
            0x87 => {
                self.registers.a = self.res(0, self.registers.a);
                8
            },
            // RES 1, B
            0x88 => {
                self.registers.b = self.res(1, self.registers.b);
                8
            },
            // RES 1, C
            0x89 => {
                self.registers.c = self.res(1, self.registers.c);
                8
            },
            // RES 1, D
            0x8A => {
                self.registers.d = self.res(1, self.registers.d);
                8
            },
            // RES 1, E
            0x8B => {
                self.registers.e = self.res(1, self.registers.e);
                8
            },
            // RES 1, H
            0x8C => {
                self.registers.h = self.res(1, self.registers.h);
                8
            },
            // RES 1, L
            0x8D => {
                self.registers.l = self.res(1, self.registers.l);
                8
            },
            // RES 1, (HL)
            0x8E => {
                self.registers.set_hl(
                    self.res(
                        1,
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // RES 1, A
            0x8F => {
                self.registers.a = self.res(1, self.registers.a);
                8
            },
            // RES 2, B
            0x90 => {
                self.registers.b = self.res(2, self.registers.b);
                8
            },
            // RES 2, C
            0x91 => {
                self.registers.c = self.res(2, self.registers.c);
                8
            },
            // RES 2, D
            0x92 => {
                self.registers.d = self.res(2, self.registers.d);
                8
            },
            // RES 2, E
            0x93 => {
                self.registers.e = self.res(2, self.registers.e);
                8
            },
            // RES 2, H
            0x94 => {
                self.registers.h = self.res(2, self.registers.h);
                8
            },
            // RES 2, L
            0x95 => {
                self.registers.l = self.res(2, self.registers.l);
                8
            },
            // RES 2, (HL)
            0x96 => {
                self.registers.set_hl(
                    self.res(
                        2,
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // RES 2, A
            0x97 => {
                self.registers.a = self.res(2, self.registers.a);
                8
            },
            // RES 3, B
            0x98 => {
                self.registers.b = self.res(3, self.registers.b);
                8
            },
            // RES 3, C
            0x99 => {
                self.registers.c = self.res(3, self.registers.c);
                8
            },
            // RES 3, D
            0x9A => {
                self.registers.d = self.res(3, self.registers.d);
                8
            },
            // RES 3, E
            0x9B => {
                self.registers.e = self.res(3, self.registers.e);
                8
            },
            // RES 3, H
            0x9C => {
                self.registers.h = self.res(3, self.registers.h);
                8
            },
            // RES 3, L
            0x9D => {
                self.registers.l = self.res(3, self.registers.l);
                8
            },
            // RES 3, (HL)
            0x9E => {
                self.registers.set_hl(
                    self.res(
                        3,
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // RES 3, A
            0x9F => {
                self.registers.a = self.res(3, self.registers.a);
                8
            },
            // RES 4, B
            0xA0 => {
                self.registers.b = self.res(4, self.registers.b);
                8
            },
            // RES 4, C
            0xA1 => {
                self.registers.c = self.res(4, self.registers.c);
                8
            },
            // RES 4, D
            0xA2 => {
                self.registers.d = self.res(4, self.registers.d);
                8
            },
            // RES 4, E
            0xA3 => {
                self.registers.e = self.res(4, self.registers.e);
                8
            },
            // RES 4, H
            0xA4 => {
                self.registers.h = self.res(4, self.registers.h);
                8
            },
            // RES 4, L
            0xA5 => {
                self.registers.l = self.res(4, self.registers.l);
                8
            },
            // RES 4, (HL)
            0xA6 => {
                self.registers.set_hl(
                    self.res(
                        4,
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // RES 4, A
            0xA7 => {
                self.registers.a = self.res(4, self.registers.a);
                8
            },
            // RES 5, B
            0xA8 => {
                self.registers.b = self.res(5, self.registers.b);
                8
            },
            // RES 5, C
            0xA9 => {
                self.registers.c = self.res(5, self.registers.c);
                8
            },
            // RES 5, D
            0xAA => {
                self.registers.d = self.res(5, self.registers.d);
                8
            },
            // RES 5, E
            0xAB => {
                self.registers.e = self.res(5, self.registers.e);
                8
            },
            // RES 5, H
            0xAC => {
                self.registers.h = self.res(5, self.registers.h);
                8
            },
            // RES 5, L
            0xAD => {
                self.registers.l = self.res(5, self.registers.l);
                8
            },
            // RES 5, (HL)
            0xAE => {
                self.registers.set_hl(
                    self.res(
                        5,
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // RES 5, A
            0xAF => {
                self.registers.a = self.res(5, self.registers.a);
                8
            },
            // RES 6, B
            0xB0 => {
                self.registers.b = self.res(6, self.registers.b);
                8
            },
            // RES 6, C
            0xB1 => {
                self.registers.c = self.res(6, self.registers.c);
                8
            },
            // RES 6, D
            0xB2 => {
                self.registers.d = self.res(6, self.registers.d);
                8
            },
            // RES 6, E
            0xB3 => {
                self.registers.e = self.res(6, self.registers.e);
                8
            },
            // RES 6, H
            0xB4 => {
                self.registers.h = self.res(6, self.registers.h);
                8
            },
            // RES 6, L
            0xB5 => {
                self.registers.l = self.res(6, self.registers.l);
                8
            },
            // RES 6, (HL)
            0xB6 => {
                self.registers.set_hl(
                    self.res(
                        6,
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // RES 6, A
            0xB7 => {
                self.registers.a = self.res(6, self.registers.a);
                8
            },
            // RES 7, B
            0xB8 => {
                self.registers.b = self.res(7, self.registers.b);
                8
            },
            // RES 7, C
            0xB9 => {
                self.registers.c = self.res(7, self.registers.c);
                8
            },
            // RES 7, D
            0xBA => {
                self.registers.d = self.res(7, self.registers.d);
                8
            },
            // RES 7, E
            0xBB => {
                self.registers.e = self.res(7, self.registers.e);
                8
            },
            // RES 7, H
            0xBC => {
                self.registers.h = self.res(7, self.registers.h);
                8
            },
            // RES 7, L
            0xBD => {
                self.registers.l = self.res(7, self.registers.l);
                8
            },
            // RES 7, (HL)
            0xBE => {
                self.registers.set_hl(
                    self.res(
                        7,
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // RES 7, A
            0xBF => {
                self.registers.a = self.res(7, self.registers.a);
                8
            },
            // SET 0, B
            0xC0 => {
                self.registers.b = self.set(0, self.registers.b);
                8
            },
            // SET 0, C
            0xC1 => {
                self.registers.c = self.set(0, self.registers.c);
                8
            },
            // SET 0, D
            0xC2 => {
                self.registers.d = self.set(0, self.registers.d);
                8
            },
            // SET 0, E
            0xC3 => {
                self.registers.e = self.set(0, self.registers.e);
                8
            },
            // SET 0, H
            0xC4 => {
                self.registers.h = self.set(0, self.registers.h);
                8
            },
            // SET 0, L
            0xC5 => {
                self.registers.l = self.set(0, self.registers.l);
                8
            },
            // SET 0, (HL)
            0xC6 => {
                self.registers.set_hl(
                    self.set(
                        0,
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // SET 0, A
            0xC7 => {
                self.registers.a = self.set(0, self.registers.a);
                8
            },
            // SET 1, B
            0xC8 => {
                self.registers.b = self.set(1, self.registers.b);
                8
            },
            // SET 1, C
            0xC9 => {
                self.registers.c = self.set(1, self.registers.c);
                8
            },
            // SET 1, D
            0xCA => {
                self.registers.d = self.set(1, self.registers.d);
                8
            },
            // SET 1, E
            0xCB => {
                self.registers.e = self.set(1, self.registers.e);
                8
            },
            // SET 1, H
            0xCC => {
                self.registers.h = self.set(1, self.registers.h);
                8
            },
            // SET 1, L
            0xCD => {
                self.registers.l = self.set(1, self.registers.l);
                8
            },
            // SET 1, (HL)
            0xCE => {
                self.registers.set_hl(
                    self.set(
                        1,
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // SET 1, A
            0xCF => {
                self.registers.a = self.set(1, self.registers.a);
                8
            },
            // SET 2, B
            0xD0 => {
                self.registers.b = self.set(2, self.registers.b);
                8
            },
            // SET 2, C
            0xD1 => {
                self.registers.c = self.set(2, self.registers.c);
                8
            },
            // SET 2, D
            0xD2 => {
                self.registers.d = self.set(2, self.registers.d);
                8
            },
            // SET 2, E
            0xD3 => {
                self.registers.e = self.set(2, self.registers.e);
                8
            },
            // SET 2, H
            0xD4 => {
                self.registers.h = self.set(2, self.registers.h);
                8
            },
            // SET 2, L
            0xD5 => {
                self.registers.l = self.set(2, self.registers.l);
                8
            },
            // SET 2, (HL)
            0xD6 => {
                self.registers.set_hl(
                    self.set(
                        2,
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // SET 2, A
            0xD7 => {
                self.registers.a = self.set(2, self.registers.a);
                8
            },
            // SET 3, B
            0xD8 => {
                self.registers.b = self.set(3, self.registers.b);
                8
            },
            // SET 3, C
            0xD9 => {
                self.registers.c = self.set(3, self.registers.c);
                8
            },
            // SET 3, D
            0xDA => {
                self.registers.d = self.set(3, self.registers.d);
                8
            },
            // SET 3, E
            0xDB => {
                self.registers.e = self.set(3, self.registers.e);
                8
            },
            // SET 3, H
            0xDC => {
                self.registers.h = self.set(3, self.registers.h);
                8
            },
            // SET 3, L
            0xDD => {
                self.registers.l = self.set(3, self.registers.l);
                8
            },
            // SET 3, (HL)
            0xDE => {
                self.registers.set_hl(
                    self.set(
                        3,
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // SET 3, A
            0xDF => {
                self.registers.a = self.set(3, self.registers.a);
                8
            },
            // SET 4, B
            0xE0 => {
                self.registers.b = self.set(4, self.registers.b);
                8
            },
            // SET 4, C
            0xE1 => {
                self.registers.c = self.set(4, self.registers.c);
                8
            },
            // SET 4, D
            0xE2 => {
                self.registers.d = self.set(4, self.registers.d);
                8
            },
            // SET 4, E
            0xE3 => {
                self.registers.e = self.set(4, self.registers.e);
                8
            },
            // SET 4, H
            0xE4 => {
                self.registers.h = self.set(4, self.registers.h);
                8
            },
            // SET 4, L
            0xE5 => {
                self.registers.l = self.set(4, self.registers.l);
                8
            },
            // SET 4, (HL)
            0xE6 => {
                self.registers.set_hl(
                    self.set(
                        4,
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // SET 4, A
            0xE7 => {
                self.registers.a = self.set(4, self.registers.a);
                8
            },
            // SET 5, B
            0xE8 => {
                self.registers.b = self.set(5, self.registers.b);
                8
            },
            // SET 5, C
            0xE9 => {
                self.registers.c = self.set(5, self.registers.c);
                8
            },
            // SET 5, D
            0xEA => {
                self.registers.d = self.set(5, self.registers.d);
                8
            },
            // SET 5, E
            0xEB => {
                self.registers.e = self.set(5, self.registers.e);
                8
            },
            // SET 5, H
            0xEC => {
                self.registers.h = self.set(5, self.registers.h);
                8
            },
            // SET 5, L
            0xED => {
                self.registers.l = self.set(5, self.registers.l);
                8
            },
            // SET 5, (HL)
            0xEE => {
                self.registers.set_hl(
                    self.set(
                        5,
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // SET 5, A
            0xEF => {
                self.registers.a = self.set(5, self.registers.a);
                8
            },
            // SET 6, B
            0xF0 => {
                self.registers.b = self.set(6, self.registers.b);
                8
            },
            // SET 6, C
            0xF1 => {
                self.registers.c = self.set(6, self.registers.c);
                8
            },
            // SET 6, D
            0xF2 => {
                self.registers.d = self.set(6, self.registers.d);
                8
            },
            // SET 6, E
            0xF3 => {
                self.registers.e = self.set(6, self.registers.e);
                8
            },
            // SET 6, H
            0xF4 => {
                self.registers.h = self.set(6, self.registers.h);
                8
            },
            // SET 6, L
            0xF5 => {
                self.registers.l = self.set(6, self.registers.l);
                8
            },
            // SET 6, (HL)
            0xF6 => {
                self.registers.set_hl(
                    self.set(
                        6,
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // SET 6, A
            0xF7 => {
                self.registers.a = self.set(6, self.registers.a);
                8
            },
            // SET 7, B
            0xF8 => {
                self.registers.b = self.set(7, self.registers.b);
                8
            },
            // SET 7, C
            0xF9 => {
                self.registers.c = self.set(7, self.registers.c);
                8
            },
            // SET 7, D
            0xFA => {
                self.registers.d = self.set(7, self.registers.d);
                8
            },
            // SET 7, E
            0xFB => {
                self.registers.e = self.set(7, self.registers.e);
                8
            },
            // SET 7, H
            0xFC => {
                self.registers.h = self.set(7, self.registers.h);
                8
            },
            // SET 7, L
            0xFD => {
                self.registers.l = self.set(7, self.registers.l);
                8
            },
            // SET 7, (HL)
            0xFE => {
                self.registers.set_hl(
                    self.set(
                        7,
                        self.mmu.read_byte(
                            self.registers.get_hl()
                        )
                    )
                );
                16
            },
            // SET 7, A
            0xFF => {
                self.registers.a = self.set(7, self.registers.a);
                8
            },
            // Si code non trouvé
            _ => {
                panic!("OpCode not found");
                0
            }
        }
    }

    fn update_emi(&mut self) {
        match self.di {
            2 => {
                self.di = 1;
            },
            1 => {
                self.di = 0;
                self.emi = false;
            }
        }
        match self.ei {
            2 => {
                self.ei = 1;
            },
            1 => {
                self.ei = 0;
                self.emi = true;
            }
        }
    }

    fn manage_interruptions(&mut self) -> u32 {
        if self.ime {
            // if io.pending_joypad_interruption
            if (
                mmu.interrupt_flag & 0x10 == 0x10 &&
                mmu.ie & 0x10 == 0x10
            ) {
                mmu.interrupt_flag |= 0xEF;
                mmu.ie |= 0xEF;
                // 2 NOP + PUSH PC
                self.rst(0x0060);
                return 20;
            }
            // if io.pending_timer_interruption
            if (
                mmu.interrupt_flag & 0x04 == 0x04 &&
                mmu.ie & 0x04 == 0x04
            ) {
                mmu.interrupt_flag |= 0xF7;
                mmu.ie |= 0xF7;
                // 2 NOP + PUSH PC + LD PC 0x50
                self.rst(0x0050);
                return 20;
            }
            // TODO: Interrputions related to graphics
        }
        // If 0 is return, no interruptions should be called
        0
    }

    pub fn execute_step(&mut self) -> u32 {
        self.update_emi();
        let time_interruption = self.manage_interruptions();
        if time_interruption != 0 {
            return time_interruption;
        }
        if self.is_halted {
            return 4;
        }
        self.receiveOp()
    }

    // Fonctions arithmétiques
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
            false
        );
        // Pas de maj du carry
        res
    }

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

    fn adc(&mut self, value: u8) {
        let (
            temp_value,
            did_overflow1
        ) = value.overflowing_add(1);
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
            (self.registers.a & 0x0F) + (value + 0x0F) + 1 > 0x0F
        );
        self.registers.a = new_value;
    }

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

    fn addr8(&mut self, value: u16) -> u16 {
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

    fn sbc(&mut self, value: u8) {
        let (temp_value, did_overflow1) = self.registers.a.overflowing_sub(1);
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
            (self.registers.a & 0x0F) < (value + 0x0F) + 1
        );
        self.registers.a = new_value;
    }

    fn cp(&mut self, value: u8) {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        self.flags.zero = new_value == 0;
        self.flags.subtract = true;
        self.flags.carry = did_overflow;
    }

    fn rl(&mut self, value: u8) -> u8 {
        // left shift + carry flag placé à droite
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

    fn rrc(&mut self, value: u8) -> u8 {
        // right shift + bit qui sort placé à gauche
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

    fn daa(&mut self) {
        // d'après https://github.com/mvdnes/rboy/blob/master/src/cpu.rs#L793
        let mut a = self.registers.a;
        let mut adjust = if self.registers.get_carry() {
            0x60
        } else {
            0x00
        };
        if self.registers.get_half() {
            adjust |= 0x06;
        };
        if !self.registers.get_sub() {
            if a & 0x0F > 0x09 { adjust |= 0x06; };
            if a > 0x99 { adjust |= 0x60; };
            a = a.wrapping_add(adjust);
        } else {
            a = a.wrapping_sub(adjust);
        }

        self.registers.set_carry(adjust >= 0x60);
        self.registers.set_half(false);
        self.registers.set_zero(a == 0);
        self.registers.a = a;
    }

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

    fn sra(&mut self, value: u8) -> u8 {
        // MSB doesn't change
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

    fn srl(&mut self, value: u8) -> u8 {
        // MSB set to zero
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

    fn swap(&mut self, value: u8) -> u8 {
        let result = (
            (value & 0xF0) >> 4 |
            (value & 0x0F) << 4
        );
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

    fn bit(&mut self, bit: u32, value: u8) {
        self.registers.set_zero(
            value & ((1 << bit) as u8) == 0;
        );
        self.registers.set_sub(
            false
        );
        self.registers.set_halr(
            true
        );
        // Carry not affected
    }

    fn res(&mut self, bit: u32, value: u8) {
        value & !((1 << bit) as u8)
    }

    fn set(&mut self, bit: u32, value: u8) {
        value | ((1 << bit) as u8)
    }

