//! Used to work with 3 byte addresses.
//!
//! The SNES uses a 24 bit address space but only has 16 bit registers.
//! Addresses are formed by combining values and registers in different ways.
//!
//! This provides an interface to make things a little easier.

use std::ops::Add;

/// Three byte address
///
/// All addresses are created by combining a bank byte with two
/// address bytes. Often any address operations require bitwise logic.
/// This provides an interface to make life easier.
#[derive(Clone, Copy, Debug)]
pub struct SnesAddress {
    bank: u8,
    addr: u16,
}

impl Default for SnesAddress {
    /// Default constructor
    ///
    /// # Returns
    /// [SnesAddress] with bank and addres both 0
    fn default() -> Self {
        Self { bank: 0, addr: 0 }
    }
}

impl From<(u8, u16)> for SnesAddress {
    /// Init bank and address
    ///
    /// Initializes the address with desired values.
    ///
    /// # Params
    /// * `value` - Tuple with address in the form of (bank: u8, address: u16)
    fn from(value: (u8, u16)) -> Self {
        Self {
            bank: value.0,
            addr: value.1,
        }
    }
}

impl From<usize> for SnesAddress {
    /// Init whole address
    ///
    /// Initializes the address with desired values.
    ///
    /// # Params
    /// * `value` - A single value address as a `usize`
    fn from(value: usize) -> Self {
        let bytes = value.to_le_bytes();
        Self {
            bank: bytes[2],
            addr: u16::from_le_bytes([bytes[0], bytes[1]]),
        }
    }
}

impl From<&SnesAddress> for usize {
    /// Cast to usize
    ///
    /// Uses the data to form a usize for memory indexing
    ///
    /// # Params
    /// * `value` - &[SnesAddress]
    ///
    /// # Returns
    /// [usize]
    fn from(value: &SnesAddress) -> Self {
        let bank = value.get_bank();
        let address = value.get_address();
        let two_bytes = 16;
        let shifted_bank = (bank as usize) << two_bytes;
        let result = shifted_bank + (address as usize);
        return result;
    }
}

impl From<SnesAddress> for usize {
    /// Cast to usize
    ///
    /// Uses the data to form a usize for memory indexing
    ///
    /// # Params
    /// * `value` - [SnesAddress]
    ///
    /// # Returns
    /// [usize]
    fn from(value: SnesAddress) -> Self {
        let bank = value.get_bank();
        let address = value.get_address();
        let two_bytes = 16;
        let shifted_bank = (bank as usize) << two_bytes;
        let result = shifted_bank + (address as usize);
        return result;
    }
}

impl Add<u16> for SnesAddress {
    type Output = Self;
    fn add(self, rhs: u16) -> Self {
        Self {
            bank: self.bank,
            addr: self.addr + rhs,
        }
    }
}

impl PartialEq<SnesAddress> for SnesAddress {
    fn eq(&self, other: &SnesAddress) -> bool {
        return (self.bank == other.bank) && (self.addr == other.addr);
    }
}

impl SnesAddress {
    /// Get address bank (high byte)
    pub fn get_bank(&self) -> u8 {
        return self.bank;
    }

    /// Get address offset (low bytes)
    pub fn get_address(&self) -> u16 {
        return self.addr;
    }

    pub fn set_bank(&mut self, bank: u8) {
        self.bank = bank;
    }

    pub fn set_address(&mut self, address: u16) {
        self.addr = address;
    }
}
