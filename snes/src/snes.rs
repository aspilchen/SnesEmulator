//! SNES System
//! 
//! Provides the UI for the SNES.

use crate::ricoh5a22;
use crate::instructions as inst;
use crate::memory::{self, Memory};

/// This will hold all the hardware structs for a complete SNES.
pub struct Snes {
    pub cpu: ricoh5a22::Cpu,
    pub mem: Memory,
}

impl Default for Snes {
    fn default() -> Self {
        Self {
            cpu: ricoh5a22::Cpu::default(),
            mem: Memory::default(),
        }
    }
}

pub fn load_binary(sys: &mut Snes, bin: &Vec<u8>) {
    memory::load_binary(&mut sys.mem, bin);
    reset(sys);
}

pub fn reset(sys: &mut Snes) {
    let address = memory::reset_vector(&sys.mem);
    ricoh5a22::set_pc_address(&mut sys.cpu, address as usize);
}

pub fn run(sys: &mut Snes) {
    todo!();
}

pub fn run_for(sys: &mut Snes, cycles: i32) {
    while sys.cpu.cycles < cycles {
        let op = memory::io::byte::fetch(&sys.mem, &mut sys.cpu);
        inst::execute(sys, op);
    }
}

pub fn fetch_exec(sys: &mut Snes) {
    let op = memory::io::byte::fetch(&sys.mem, &mut sys.cpu);
    inst::execute(sys, op);
}
