//! Controls memory IO.
//!
//! The SNES uses memory maps to access RAM/ROM etc. A lot of the mapping is determined by the cartridge,
//! and there are a number of different mapping structures depending on which cartridge is used.
//! This will eventually serve as an interface to the memory map, and therefore all memory IO should
//! go through here.
//!
//! # Todo
//! * Implement memory mapping

use crate::snes_address::SnesAddress;
use crate::state::State;

pub type Address = SnesAddress;
pub type BaseAddress = u16;
pub type Word = u16;
pub type Byte = u8;

pub const ONE_BYTE: u16 = 1;
pub const BITS_IN_BYTE: u16 = 8;
pub const BYTES_IN_WORD: u16 = 2;
pub const BITS_IN_WORD: u16 = BYTES_IN_WORD * BITS_IN_BYTE;
pub const BYTES_IN_LONG: u16 = 3;
pub const MEMORY_MAX: usize = 0xFFFFFF;

/// Load a binary file into memory
///
/// Copies the data from `binary` into [`State::mem`]
///
/// # Params
/// * `state` - Current system state
/// * `binary` - Vector containing program binary data
pub fn load(state: &mut State, binary: &Vec<u8>) {
    state.mem[0..binary.len()].copy_from_slice(&binary[..]);
}

/// Memory write
pub fn put_word(state: &mut State, address: &Address, value: u16) {
    let addr = usize::from(address);
    let result = u16::to_le_bytes(value);
    state.mem[addr..addr + 2].copy_from_slice(&result);
}

/// Memory write
pub fn put_byte(state: &mut State, address: &Address, value: u8) {
    let addr = usize::from(address);
    state.mem[addr] = value;
}

/// Memory read
pub fn get_word(state: &State, address: &Address) -> Word {
    let addr = usize::from(address);
    let bytes = [state.mem[addr], state.mem[addr + 1]];
    let result = u16::from_le_bytes(bytes);
    return result;
}

/// Memory read
pub fn get_byte(state: &State, address: &Address) -> Byte {
    let addr = usize::from(address);
    return state.mem[addr];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_put_word() {
        let mut state = State::default();

        {
            let address = SnesAddress::from((0, 50));
            let value = 0x25f9;
            put_word(&mut state, &address, value);
            let result = u16::from_le_bytes([state.mem[50], state.mem[51]]);
            assert_eq!(result, value);
        }

        {
            let address = SnesAddress::from((1, 50));
            let value = 0x25f9;
            put_word(&mut state, &address, value);
            let addr = usize::from(address);
            let result = u16::from_le_bytes([state.mem[addr], state.mem[addr + 1]]);
            assert_eq!(result, value);
        }
    }

    #[test]
    fn test_put_byte() {
        let mut state = State::default();

        {
            let address = SnesAddress::from((0, 50));
            let value = 0x25;
            put_byte(&mut state, &address, value);
            let result = state.mem[50];
            assert_eq!(result, value);
        }

        {
            let address = SnesAddress::from((1, 50));
            let value = 0xf9;
            put_byte(&mut state, &address, value);
            let addr = usize::from(address);
            let result = state.mem[addr];
            assert_eq!(result, value);
        }
    }

    #[test]
    fn test_get_word() {
        let mut state = State::default();

        {
            let addr: usize = 20;
            let value: u16 = 0x25f9;
            let address = SnesAddress::from(addr);
            put_word(&mut state, &address, value);
            let result = get_word(&state, &address);
            assert_eq!(result, value);
        }

        {
            let addr: usize = 0x102030;
            let value: u16 = 0x2AB;
            let address = SnesAddress::from(addr);
            put_word(&mut state, &address, value);
            let result = get_word(&state, &address);
            assert_eq!(result, value);
        }
    }

    #[test]
    fn test_get_byte() {
        let mut state = State::default();

        {
            let addr: usize = 20;
            let value: u8 = 0x25;
            let address = SnesAddress::from(addr);
            put_byte(&mut state, &address, value);
            let result = get_byte(&state, &address);
            assert_eq!(result, value);
        }

        {
            let addr: usize = 0x102030;
            let value: u16 = 0x25f9;
            let address = SnesAddress::from(addr);
            put_word(&mut state, &address, value);
            let result = get_byte(&state, &address);
            assert_eq!(result, 0xf9);
        }
    }
}
