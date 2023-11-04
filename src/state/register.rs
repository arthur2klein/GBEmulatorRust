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
            pub fn [< get_ $register1 $register2 >](&self) -> u16 {
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
            pub fn [< set_ $register1 $register2 >](&mut self, value: u16) {
                self.$register1 = ((value & 0xFF00) >> 8) as u8;
                self.$register2 = (value & 0xFF) as u8;
            }
        }
    }
}

#[derive(Debug)]
/// The registers used by the CPU to store values
pub struct Registers {
    /// 8 bit register A 
    pub a: u8,
    /// 8 bit register B
    pub b: u8,
    /// 8 bit register C
    pub c: u8,
    /// 8 bit register D
    pub d: u8,
    /// 8 bit register E
    pub e: u8,
    /// 8 bit register F
    pub f: u8,
    /// 8 bit register H
    pub h: u8,
    /// 8 bit register L
    pub l: u8,
    /// 16 bit Program Counter register
    pub pc: u16,
    /// 16 bit Stack Pointer register
    pub sp: u16,
}

impl Registers {
    /// Create the registers with their initial values
    ///
    /// # Returns
    /// 
    /// **Register**: New instance of Registers
    pub fn new() -> Self {
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
    pub fn get_hld(&mut self) -> u16 {
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
    pub fn get_hli(&mut self) -> u16 {
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
    pub fn get_carry(&self) -> bool {
        self.f & 0b00010000 != 0
    }

    /// Assigns the wanted value to the carry flag (aka C flag)
    ///
    /// The carry flag is generally set when the previous operation overflows
    ///
    /// # Arguments
    /// **value (bool)**: true iff you want the carry flag is to be set
    pub fn set_carry(&mut self, value: bool) {
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
    pub fn get_half(&self) -> bool {
        self.f & 0b00100000 != 0
    }

    /// Assigns the wanted value to the half carry flag (aka H flag)
    ///
    /// The half carry flag is generally set when the previous operation
    /// overflows considering only the first half of the operators
    ///
    /// # Arguments
    /// **value (bool)**: true iff you want the half carry flag is to be set
    pub fn set_half(&mut self, value: bool) {
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
    pub fn get_sub(&self) -> bool {
        self.f & 0b01000000 != 0
    }

    /// Assigns the wanted value to the substraction flag (aka N flag)
    ///
    /// The substraction flag is generally set when the previous operation is
    /// a substraction
    ///
    /// # Arguments
    /// **value (bool)**: true iff you want the substaction flag is to be set
    pub fn set_sub(&mut self, value: bool) {
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
    pub fn get_zero(&self) -> bool {
        self.f & 0b10000000 != 0
    }

    /// Assigns the wanted value to the zero flag (aka Z flag)
    ///
    /// The zero flag is generally set when the result of the previous
    /// operation is 0
    ///
    /// # Arguments
    /// **value (bool)**: true iff you want the zero flag is to be set
    pub fn set_zero(&mut self, value: bool) {
        if value {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
    }
}

