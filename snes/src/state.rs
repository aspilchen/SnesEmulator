//! System state
//!
//! Holds the current state of all registers and memory.
//! 

use crate::memory;
use crate::multi_register::MultiRegister;
use crate::snes_address::SnesAddress;
use crate::status::Status;
use std::fmt;

pub struct State {
    /// Data Bank Register
    ///
    /// Used as the bank byte for reading/writing
    pub dbr: u8,

    /// Direct Page Register
    ///
    /// Used for direct page addressing
    pub d: u16,

    /// Accumulator
    ///
    /// Used to perform  math and logic
    pub a: MultiRegister,

    /// Index Register
    ///
    /// Used for indexing addresses
    pub x: MultiRegister,

    /// Index Register
    ///
    /// Used for indexing addresses
    pub y: MultiRegister,

    /// Stack Pointer
    ///
    /// Address for top of stack
    pub s: SnesAddress,

    /// Program Counter
    ///
    /// Combines the DBR (Data Bank Register) with the
    /// 16 bit program counter to form the address.
    /// Used to fetch data and for control flow.
    pub pc: SnesAddress,

    /// Status Register
    ///
    /// Status bits used for comparison operations and to
    /// control CPU behaviour.
    pub p: Status,

    /// Emulation Flag
    ///
    /// Hidden flag to control emulation mode.
    /// When in emulation mode, system behaves like the NES.
    ///
    /// # Note
    /// As far as I can tell the SNES was meant to be backwards compatible
    /// with NES games, but this feature was never used. For now I will just
    /// be working with the native mode, and will not spend much time handling
    /// emulation features.
    pub e: bool,

    /// Cycle Count
    ///
    /// Counts cycles during execution. Will be used by a system clock
    /// to keep the system running at the correct speed.
    pub cycles: u32,
    pub mem: Vec<u8>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            dbr: 0,
            d: 0,
            cycles: 0,
            a: MultiRegister::default(),
            x: MultiRegister::default(),
            y: MultiRegister::default(),
            s: SnesAddress::default(),
            pc: SnesAddress::default(),
            p: Status::none(),
            e: false,
            mem: vec![0; memory::MEMORY_MAX],
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A = {} {:?}\n", self.a.get_16(), self.a)?;
        write!(f, "X = {} {:?}\n", self.x.get_16(), self.x)?;
        write!(f, "Y = {} {:?}\n", self.y.get_16(), self.y)?;
        write!(f, "P = {:08b}\n", self.p)?;
        write!(f, "DBR = {}\n", self.dbr)?;
        write!(
            f,
            "PBR:PC = {}:{}\n",
            self.pc.get_bank(),
            self.pc.get_address()
        )?;
        return write!(f, "cycles: {}\n", self.cycles);
    }
}

impl State {
    /// Get program bank
    ///
    /// Clear access to the PBR which is hidden in the [State::pc]
    pub fn get_pbr(&self) -> u8 {
        return self.pc.get_bank();
    }

    /// Set program bank
    ///
    /// Clear access to the PBR which is hidden in the [State::pc]
    pub fn set_pbr(&mut self, value: u8) {
        self.pc.set_bank(value);
    }
}
