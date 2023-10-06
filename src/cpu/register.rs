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

impl std::convert::From<FlagsRegister> for u8  {
    fn from(flag: FlagsRegister) -> u8 {
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

