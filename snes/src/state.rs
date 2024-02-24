use crate::memory;
use crate::multi_register::MultiRegister;
use crate::snes_address::SnesAddress;
use crate::status::Status;
use std::fmt;

pub struct State {
    pub dbr: u8,
    pub d: u16,
    pub cycles: u32,
    pub a: MultiRegister,
    pub x: MultiRegister,
    pub y: MultiRegister,
    pub s: SnesAddress,
    pub pc: SnesAddress,
    pub p: Status,
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
    pub fn get_pbr(&self) -> u8 {
        return self.pc.get_bank();
    }

    pub fn set_pbr(&mut self, value: u8) {
        self.pc.set_bank(value);
    }
}