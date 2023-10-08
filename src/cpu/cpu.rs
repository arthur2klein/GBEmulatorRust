// Le CPU de la Game Boy est un CPU a 8 bits, ce qui signifie que chacun de ses registres peut contenir 8 bits.

/* ---------------------------------------------------------------------------*/

//Definition des structs

struct CPU {
    registers: Register,
    flags: FlagRegister,
    // to do in a separated file
    mmu: MMU,
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

struct SpecialRegisters {
    d16: u16,
    d8: u8,
    r8: u8,

}

/* ---------------------------------------------------------------------------*/

//Implementation des registres

impl Registers {
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

    fn send_stop(&self) {
        self.mmu.switch_speed();
    }

    // Jump
    fn jr(&mut self) {
        // Les conversions permettent d'assurer que fetchbyte est considéré comme signé, mais pas
        // pc, que l'opérations puissnet avoir lieu, et que le résutat ait le bon format
        self.registers.pc = (
            (self.registers.pc as u32 as i32) +
            (self.fetchbyte() as i8 as i32)
        ) as u16;
    }

    // Lecture des OpCodes

    // Returns the length in bytes
    fn receiveOp(&mut self, op: u8) -> u32 {
        match op {
            // NOP
            0x00 => {
                1
            },
            // LD BC, d16
            0x01 => {
                self.registers.set_bc(
                    self.fetchword()
                );
                3
            },
            // LD (BC), A
            0x02 => {
                self.mmu.write_byte(
                    self.registers.get_bc(),
                    self.registers.a
                    );
                1
            },
            // INC BC
            0x03 => {
                self.registers.set_bc(
                    self.registers.get_bc().wrapping_add(1)
                    );
                1
            },
            // INC B
            0x04 => {
                self.registers.b = self.inc(self.registers.b);
                1
            },
            // DEC B
            0x05 => {
                self.registers.b = self.dec(self.registers.b);
                1
            },
            // LD B, d8
            0x06 => {
                self.registers.b = self.fetchbyte();
                2
            },
            // RLCA
            0x07 => {
                self.registers.a = self.rlc(self.registers.a);
                self.registers.set_zero(false);
                1
            },
            // LD (a16), SP
            0x08 => {
                self.mmu.write_word(
                    self.fetchword(),
                    self.registers.sp
                );
                3
            },
            // ADD HL, BC
            0x09 => {
                self.addhl(self.registers.get_bc());
                1
            },
            // LD A, (BC)
            0x0A => {
                self.registers.a = self.mmu.read_byte(self.registers.get_bc());
                1
            },
            // DEC BC
            0x0B => {
                self.registers.set_bc(self.registers.get_bc().wrapping_sub(1));
                1
            },
            // INC C
            0x0C => {
                self.registers.c = self.inc(self.registers.c);
                1
            },
            // DEC C
            0x0D => {
                self.registers.c = self.dec(self.registers.c);
                1
            },
            // LD C, d8
            0x0E => {
                self.registers.c = self.fetchbyte();
                2
            },
            // RRCA
            0x0F => {
                self.registers.a = self.rrc(self.registers.a);
                self.registers.set_zero(false);
                1
            },
            // STOP A
            0x10 => {
                self.send_stop();
                2
            },
            // LD DE, D16
            0x11 => {
                self.registers.set_de(
                    self.fetchword()
                );
                3
            },
            // LD (DE), A
            0x12 => {
                self.mmu.write_byte(
                    self.registers.get_de(),
                    self.registers.a
                );
                2
            },
            // INC DE
            0x13 => {
                self.registers.set_de(
                    self.registers.get_de().wrapping_add(1)
                );
                1
            },
            // INC D
            0x14 => {
                self.registers.d = self.inc(self.registers.d);
                1
            },
            // DEC D
            0x15 => {
                self.registers.d = self.dec(self.registers.d);
                1
            ),
            // LD D, d8
            0x16 => {
                self.registers.d = self.fetchbyte();
                2
            },
            // RLA
            0x17 => {
                self.registers.a = self.rl(self.registers.a);
                self.registers.set_zero(false);
                1
            },
            // JR r8
            0x18 => {
                self.jr();
                2
            },
            // ADD HL, DE
            0x19 => {
                self.registers.addhl(
                    self.registers.get_de()
                );
                1
            },
            // LD A, (DE)
            0x1A => {
                self.registers.a = self.mmu.read_byte(
                    self.registers.get_de()
                );
                1
            },
            // DEC DE
            0x1B => {
                self.registers.set_de(
                    self.registers.get_de().wrapping_sub(1)
                );
                1
            },
            // INC E
            0x1C => {
                self.registers.e = self.inc(self.registers.e);
                1
            },
            // DEC E
            0x1D => {
                self.registers.e = self.dec(self.registers.e);
                1
            },
            // LD D, d8
            0x1E => {
                self.registers.d = self.fetchbyte();
                2
            },
            // RRA
            0x1F => {
                self.registers.a = self.rr(self.registers.a);
                self.registers.set_zero(false);
                1
            },
            // JR NZ, r8
            // LD HL, d16
            // LD (HL+), A
            // INC HL
            // INC H
            // DEC H
            // LD H, d8
            // DAA
            // JR Z, r8
            // ADD HL, HL
            // LD A, (HL+)
            // DEC HL
            // INC L
            // DEC L
            // LD L, d8
            // CPL
        }
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
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
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
        let (temp_value, did_overflow1) = value.overflowing_add(1);
        let (new_value, did_overflow2) = self.registers.a.overflowing_add(temp_value);
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
        self.registers.set_helf(
            (self.registers.a & 0x0F) + (value + 0x0F) + 1 > 0x0F
        );
        self.registers.a = new_value;
    }

    fn addhl(&mut self, value: u16) -> u16 {
        let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
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

}
