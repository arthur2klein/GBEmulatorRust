// Le CPU de la Game Boy est un CPU a 8 bits, ce qui signifie que chacun de ses registres peut contenir 8 bits.

/* ---------------------------------------------------------------------------*/

//Definition des structs

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
  }

struct FlagRegister{
    carry: bool,
    half_carry: bool,
    zero: bool,
    sub: bool,
}

/* ---------------------------------------------------------------------------*/

//Implementation des registres normaux et flag

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

  fn get_de(&self) -> u16 {
    (self.d as u16) << 8
    | self.e as u16
  }

  fn set_de(&mut self, value: u16) {
    self.d = ((value & 0xFF00) >> 8) as u8;
    self.e = (value & 0xFF) as u8;
  }
}

// 7,6,5,4 correspondent au bit auquel sont attribues respectivement (zero, substract, half_carry et carry)

impl std::convert::From<FlagRegister> for u8  {
    fn from(flag: FlagRegister) -> u8 {
        (if flag.zero       { 1 } else { 0 }) << 7 |
        (if flag.subtract   { 1 } else { 0 }) << 6 |
        (if flag.half_carry { 1 } else { 0 }) << 5 |
        (if flag.carry      { 1 } else { 0 }) << 4
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> 7) & 0b1) != 0;
        let subtract = ((byte >> 6) & 0b1) != 0;
        let half_carry = ((byte >> 5) & 0b1) != 0;
        let carry = ((byte >> 4) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}
/* ---------------------------------------------------------------------------*/

//Instruction 

struct CPU {
    registers: Register,
    flags: FlagRegister,
}

enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTarget),
}
  
enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

enum ArithmeticTarget16 {
    BC,
    DE,
    HL,
}

//On execute les instructions en implemetant CPU

impl CPU {
  fn execute(&mut self, instruction: Instruction) {
    match instruction {
      Instruction::ADD(target) => {
        match target {
          ArithmeticTarget::A => {
            let value = self.registers.a;
            let new_value = self.add(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::B => {
            let value = self.registers.b;
            let new_value = self.add(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::C => {
            let value = self.registers.c;
            let new_value = self.add(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::D => {
            let value = self.registers.d;
            let new_value = self.add(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::E => {
            let value = self.registers.e;
            let new_value = self.add(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::H => {
            let value = self.registers.h;
            let new_value = self.add(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::L => {
            let value = self.registers.l;
            let new_value = self.add(value);
            self.registers.a = new_value;
          }
        }
      }
      Instruction::ADDHL(target) => {
        match target {
          ArithmeticTarget::BC => {
            let value = self.registers.get_bc();
            let new_value = self.addhl(value);
            self.registers.set_hl(new_value);
          }
          ArithmeticTarget::DE => {
            let value = self.registers.get_de();
            let new_value = self.addhl(value);
            self.registers.set_hl(new_value);
          }
          ArithmeticTarget::HL => {
            let value = self.registers.get_hl();
            let new_value = self.addhl(value);
            self.registers.set_hl(new_value);
          }
        }
      }
      Instruction::ADC(target) => {
        match target {
          ArithmeticTarget::A => {
            let value = self.registers.a;
            let new_value = self.adc(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::B => {
            let value = self.registers.b;
            let new_value = self.adc(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::C => {
            let value = self.registers.c;
            let new_value = self.adc(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::D => {
            let value = self.registers.d;
            let new_value = self.adc(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::E => {
            let value = self.registers.e;
            let new_value = self.adc(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::H => {
            let value = self.registers.h;
            let new_value = self.adc(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::L => {
            let value = self.registers.l;
            let new_value = self.adc(value);
            self.registers.a = new_value;
          }
        }
      }
      Instruction::SUB(target) => {
        match target {
          ArithmeticTarget::A => {
            let value = self.registers.a;
            let new_value = self.sub(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::B => {
            let value = self.registers.b;
            let new_value = self.sub(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::C => {
            let value = self.registers.c;
            let new_value = self.sub(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::D => {
            let value = self.registers.d;
            let new_value = self.sub(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::E => {
            let value = self.registers.e;
            let new_value = self.sub(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::H => {
            let value = self.registers.h;
            let new_value = self.sub(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::L => {
            let value = self.registers.l;
            let new_value = self.sub(value);
            self.registers.a = new_value;
          }
        }
      }
      Instruction::SBC(target) => {
        match target {
          ArithmeticTarget::A => {
            let value = self.registers.a;
            let new_value = self.sbc(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::B => {
            let value = self.registers.b;
            let new_value = self.sbc(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::C => {
            let value = self.registers.c;
            let new_value = self.sbc(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::D => {
            let value = self.registers.d;
            let new_value = self.sbc(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::E => {
            let value = self.registers.e;
            let new_value = self.sbc(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::H => {
            let value = self.registers.h;
            let new_value = self.sbc(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::L => {
            let value = self.registers.l;
            let new_value = self.sbc(value);
            self.registers.a = new_value;
          }
        }
      }
      Instruction::AND(target) => {
        match target {
          ArithmeticTarget::A => {
            let value = self.registers.a;
            let new_value = self.and(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::B => {
            let value = self.registers.b;
            let new_value = self.and(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::C => {
            let value = self.registers.c;
            let new_value = self.and(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::D => {
            let value = self.registers.d;
            let new_value = self.and(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::E => {
            let value = self.registers.e;
            let new_value = self.and(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::H => {
            let value = self.registers.h;
            let new_value = self.and(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::L => {
            let value = self.registers.l;
            let new_value = self.and(value);
            self.registers.a = new_value;
          }
        }
      }
      Instruction::OR(target) => {
        match target {
          ArithmeticTarget::A => {
            let value = self.registers.a;
            let new_value = self.or(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::B => {
            let value = self.registers.b;
            let new_value = self.or(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::C => {
            let value = self.registers.c;
            let new_value = self.or(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::D => {
            let value = self.registers.d;
            let new_value = self.or(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::E => {
            let value = self.registers.e;
            let new_value = self.or(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::H => {
            let value = self.registers.h;
            let new_value = self.or(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::L => {
            let value = self.registers.l;
            let new_value = self.or(value);
            self.registers.a = new_value;
          }
        }
      }
      Instruction::XOR(target) => {
        match target {
          ArithmeticTarget::A => {
            let value = self.registers.a;
            let new_value = self.xor(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::B => {
            let value = self.registers.b;
            let new_value = self.xor(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::C => {
            let value = self.registers.c;
            let new_value = self.xor(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::D => {
            let value = self.registers.d;
            let new_value = self.xor(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::E => {
            let value = self.registers.e;
            let new_value = self.xor(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::H => {
            let value = self.registers.h;
            let new_value = self.xor(value);
            self.registers.a = new_value;
          }
          ArithmeticTarget::L => {
            let value = self.registers.l;
            let new_value = self.xor(value);
            self.registers.a = new_value;
          }
        }
      }
      Instruction::CMP(target) => {
        match target {
          ArithmeticTarget::A => {
            let value = self.registers.a;
            let new_value = self.sub(value);
          }
          ArithmeticTarget::B => {
            let value = self.registers.b;
            let new_value = self.sub(value);
          }
          ArithmeticTarget::C => {
            let value = self.registers.c;
            let new_value = self.sub(value);
          }
          ArithmeticTarget::D => {
            let value = self.registers.d;
            let new_value = self.sub(value);
          }
          ArithmeticTarget::E => {
            let value = self.registers.e;
            let new_value = self.sub(value);
          }
          ArithmeticTarget::H => {
            let value = self.registers.h;
            let new_value = self.sub(value);
          }
          ArithmeticTarget::L => {
            let value = self.registers.l;
            let new_value = self.sub(value);
          }
        }
      }
      _ => { /* TODO: support more instructions */ }
    }
  }
  fn add(&mut self, value: u8) -> u8 {
    let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
    self.flags.zero = new_value == 0;
    self.flags.subtract = false;
    self.flags.carry = did_overflow;
    new_value
  }
  fn addhl(&mut self, value: u16) -> u16 {
    let (new_value, did_overflow) = self.registers.get_hl().overflowing_add(value);
    self.flags.zero = new_value == 0;
    self.flags.subtract = false;
    self.flags.carry = did_overflow;
    new_value
  }
  fn adc(&mut self, value: u8) -> u8 {
    let (temp_value, did_overflow1) = value.overflowing_add(1);
    let (new_value, did_overflow2) = self.registers.a.overflowing_add(temp_value);
    self.flags.zero = new_value == 0;
    self.flags.subtract = false;
    self.flags.carry = did_overflow1 || did_overflow2;
    new_value
  }
  fn sub(&mut self, value: u8) -> u8 {
    let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
    self.flags.zero = new_value == 0;
    self.flags.subtract = true;
    self.flags.carry = did_overflow;
    new_value
  }
  fn sbc(&mut self, value: u8) -> u8 {
    let (temp_value, did_overflow1) = value.overflowing_sub(1);
    let (new_value, did_overflow2) = self.registers.a.overflowing_sub(temp_value);
    self.flags.zero = new_value == 0;
    self.flags.subtract = true;
    self.flags.carry = did_overflow1 || did_overflow2;
    new_value
  }
  fn and(&mut self, value: u8) -> u8 {
    let new_value = a & value;
    self.flags.zero = new_value == 0;
    self.flags.subtract = false;
    self.flags.carry = false;
    self.flags.half_carry = true;
    new_value
  }
  fn or(&mut self, value: u8) -> u8 {
    let new_value = a | value;
    self.flags.zero = new_value == 0;
    self.flags.subtract = false;
    self.flags.carry = false;
    self.flags.half_carry = false;
    new_value
  }
  fn xor(&mut self, value: u8) -> u8 {
    let new_value = a ^ value;
    self.flags.zero = new_value == 0;
    self.flags.subtract = false;
    self.flags.carry = false;
    self.flags.half_carry = false;
    new_value
  }
}
