//! System Memory
//!
//! The SNES uses memory maps to access RAM/ROM etc. A lot of the mapping is determined by the cartridge,
//! and there are a number of different mapping structures depending on which cartridge is used.
//! This will eventually serve as an interface to the memory map, and therefore all memory IO should
//! go through here.
//!
//! # Todo
//! * Implement memory mapping
use crate::cartridge;

pub struct Memory {
    pub data: Vec<u8>,
    pub cartridge_type: cartridge::CartridgeType,
    pub rom_size: usize,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            data: vec![],
            cartridge_type: cartridge::CartridgeType::None,
            rom_size: 0,
        }
    }
}

/// Memory I/O
pub mod io {

    /// 16 bit operations
    pub mod word {
        use crate::ricoh5a22::{self, Cpu};

        use super::super::*;

        /// Read using address mapping
        pub fn read(mem: &Memory, address: usize) -> u16 {
            let address = map_address(address, &mem.cartridge_type);
            let result = read_nomap(mem, address);
            return result;
        }

        /// Write using address mapping
        pub fn write(mem: &mut Memory, address: usize, value: u16) {
            let address = map_address(address, &mem.cartridge_type);
            let bytes = value.to_le_bytes();
            mem.data[address..address + 2].copy_from_slice(&bytes);
        }

        /// Read without address mapping
        pub fn read_nomap(mem: &Memory, address: usize) -> u16 {
            let bytes = [mem.data[address], mem.data[address + 1]];
            let result = u16::from_le_bytes(bytes);
            return result;
        }

        pub fn fetch(mem: &Memory, cpu: &mut ricoh5a22::Cpu) -> u16 {
            let address = ricoh5a22::get_pc_address(cpu);
            let result = read(mem, address);
            let next_address = move_pc(address, &mem.cartridge_type, 2);
            ricoh5a22::set_pc_address(cpu, next_address);
            return result;
        }

        pub fn push(mem: &mut Memory, cpu: &mut Cpu, value: u16) {
            let address = ricoh5a22::registers_16::get_s(cpu) - 1;
            write(mem, address as usize, value);
            ricoh5a22::registers_16::set_s(cpu, address - 1);
        }

        pub fn pop(mem: &Memory, cpu: &mut Cpu) -> u16 {
            let address = ricoh5a22::registers_16::get_s(cpu) + 1;
            let result = read(mem, address as usize);
            ricoh5a22::registers_16::set_s(cpu, address + 1);
            return result;
        }
    }

    /// 8 bit operations
    pub mod byte {
        use super::super::*;
        use crate::ricoh5a22;

        /// Read using address mapping
        pub fn read(mem: &Memory, address: usize) -> u8 {
            let address = map_address(address, &mem.cartridge_type);
            let result = read_nomap(mem, address);
            return result;
        }

        /// Write using address mapping
        pub fn write(mem: &mut Memory, address: usize, value: u8) {
            let address = map_address(address, &mem.cartridge_type);
            mem.data[address] = value;
        }

        /// Read without address mapping
        pub fn read_nomap(mem: &Memory, address: usize) -> u8 {
            return mem.data[address];
        }

        pub fn fetch(mem: &Memory, cpu: &mut ricoh5a22::Cpu) -> u8 {
            let address = ricoh5a22::get_pc_address(cpu);
            let result = read(mem, address);
            let next_address = move_pc(address, &mem.cartridge_type, 1);
            ricoh5a22::set_pc_address(cpu, next_address);
            return result;
        }

        pub fn push(mem: &mut Memory, cpu: &mut ricoh5a22::Cpu, value: u8) {
            let address = ricoh5a22::registers_16::get_s(cpu);
            write(mem, address as usize, value);
            ricoh5a22::registers_16::set_s(cpu, address - 1);
        }

        pub fn pop(mem: &Memory, cpu: &mut ricoh5a22::Cpu) -> u8 {
            let address = ricoh5a22::registers_16::get_s(cpu) + 1;
            let result = read(mem, address as usize);
            ricoh5a22::registers_16::set_s(cpu, address);
            return result;
        }
    }
}

pub fn load_binary(mem: &mut Memory, bin: &Vec<u8>) {
    mem.data = cartridge::lo_rom::reserve_memory();
    mem.cartridge_type = cartridge::CartridgeType::LoRom;
    mem.rom_size = bin.len();
    mem.data[0..mem.rom_size].copy_from_slice(&bin[0..]);
}

pub fn reset_vector(mem: &Memory) -> u16 {
    match mem.cartridge_type {
        cartridge::CartridgeType::LoRom => {
            let address = cartridge::lo_rom::rom::inturrupt_vectors::emulation::RES;
            return io::word::read(mem, address);
        }
        _ => 0,
    }
}

pub fn map_address(address: usize, cartridge_type: &cartridge::CartridgeType) -> usize {
    match cartridge_type {
        cartridge::CartridgeType::LoRom => return cartridge::lo_rom::map_address(address),
        _ => return address,
    }
}

pub fn move_pc(address: usize, cartridge_type: &cartridge::CartridgeType, value: i16) -> usize {
    match cartridge_type {
        cartridge::CartridgeType::LoRom => {
            return cartridge::lo_rom::rom::wrapped_add(address, value)
        }
        _ => return address + value as usize,
    }
}

pub fn make_address(bank: u8, offset: u16) -> usize {
    let shifted_bank = (bank as usize) << 16;
    let offset = offset as usize;
    let result = shifted_bank + offset;
    return result;
}
