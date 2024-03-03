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
    data: Vec<u8>,
    cartridge_type: cartridge::CartridgeType,
    rom_size: usize,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            data: vec!(),
            cartridge_type: cartridge::CartridgeType::None,
            rom_size: 0,
        }
    }
}

/// Memory I/O
pub mod io {

    /// 16 bit operations
    pub mod word {
        use crate::cpu;

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
            let bytes = value.to_be_bytes();
            mem.data[address..address + 2].copy_from_slice(&bytes);
        }

        /// Read without address mapping
        pub fn read_nomap(mem: &Memory, address: usize) -> u16 {
            let bytes = [mem.data[address], mem.data[address + 1]];
            let result = u16::from_le_bytes(bytes);
            return result;
        }

        pub fn fetch(mem: &Memory, cpu: &mut cpu::Cpu) -> u16 {
            let address = cpu::get_pc_address(cpu);
            let result = read_nomap(mem, address);
            let next_address = address + 2;
            cpu::set_pc_address(cpu, next_address);
            return result;
        }
    }

    /// 8 bit operations
    pub mod byte {
        use super::super::*;
        use crate::cpu;

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

        pub fn fetch(mem: &Memory, cpu: &mut cpu::Cpu) -> u8 {
            let address = cpu::get_pc_address(cpu);
            let result = read_nomap(mem, address);
            let next_address = address + 2;
            cpu::set_pc_address(cpu, next_address);
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


pub fn map_address(address: usize, cartridge_type: &cartridge::CartridgeType) -> usize {
    match cartridge_type {
        cartridge::CartridgeType::LoRom => return cartridge::lo_rom::map_address(address),
        _ => return 0,
    }
}
