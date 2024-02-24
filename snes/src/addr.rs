//! # Info
//!
//! Everything here will fetch and read as needed. These will change the [`State`] in
//! a number of ways such as incrementing the [`State::pc`], or modifying the [`State::dbr`].
//!
//! The SNES has a 24 bit address space but only 16 bit registers.
//! Values are combined using a number of address modes, registers
//! and indirection to access the full 24 bit range.
//!
//! For now I have commented out the more complex address modes and left the simple
//! modes implemented. Just so I can progress on other parts of the system.
//! These will be worked on slowly as I find more information.
//!
//! These are difficult to implement correctly. Clear documentation on these has been
//! difficult to find. Handling things like indexing overflow will be dealth with
//! over time.
//!
//! Testing the complex modes needs a lot of set up and precision. I am slowly working
//! on this, but it's hard to know if the tests are correct.
//!
//! # Extra Notes
//!
//! SNES addresses will be shown as `bank` : `address` where `bank` is the high byte, and `address`
//! is the two low bytes.

use crate::cpu;
use crate::memory::Address;
use crate::snes_address::SnesAddress;
use crate::state::State;

/// Immediate
///
/// * Program counter = [`State::pc`]
/// * Provides current program counter as address
/// * Increments program counter
///
/// # Address Formation
/// * `0` : [`State::d`] + `byte`
///
/// # Note
/// Direct page bank is always zero
///
/// # Params
/// * `state` - Current system state
///
/// # Returns
/// [`SnesAddress`]
pub fn immediate16(state: &mut State) -> SnesAddress {
    let result = state.pc;
    cpu::increment_pc(state, 2);
    return result;
}

/// Direct Page
///
/// Fetches the next byte and adds it with [`State::d`]
///
/// # Address Formation
/// * `0` : [`State::d`] + `byte`
///
/// # Note
/// Direct page bank is always zero
///
/// # Params
/// * `state` - Current system state
///
/// # Returns
/// [`SnesAddress`]
pub fn direct(state: &mut State) -> SnesAddress {
    let address = cpu::fetch_byte(state);
    let d = state.d;
    let address = d.wrapping_add(address as u16);
    let result = SnesAddress::from((0, address));
    return result;
}

/// Todo
///
/// Takes the [`direct`] address and adds it with [`State::x`].
///
/// # Address Formation
/// * bank = 0
/// * address = [`direct`] + [`State::x`]
/// * result = bank : address
///
/// # Params
/// * `state` - Current system state
///
/// # Returns
/// [`SnesAddress`]
pub fn direct_indexed_x(state: &mut State) -> SnesAddress {
    todo!();
    //     let mut address = direct(state);
    //     let addr = address.get_address();
    //     let index = state.x.get_16();
    //     address.set_address(addr + index);
    //     return address;
}

/// Todo
///
/// Takes the [`direct`] address and adds it with [`State::y`].
///
/// # Address Formation
/// * bank = 0
/// * address = [`direct`] + [`State::y`]
/// * result = bank : address
///
/// # Params
/// * `state` - Current system state
///
/// # Returns
/// [`SnesAddress`]
pub fn direct_indexed_y(state: &mut State) -> SnesAddress {
    todo!();
    //     let mut address = direct(state);
    //     let addr = address.get_address();
    //     let index = state.y.get_16();
    //     address.set_address(addr + index);
    //     return address;
}

/// Todo
///
/// Reads two bytes from the [`direct_indexed_x`] and combines with [`State::dbr`] to
/// form an address.
///
/// # Address Formation
/// * bank = [`State::dbr`]
/// * address = read([`direct_indexed_x`])
/// * result = bank : address
///
/// # Params
/// * `state` - Current system state
///
/// # Returns
/// [`SnesAddress`]
pub fn direct_indexed_indirect(state: &mut State) -> SnesAddress {
    todo!();
    //     let address = direct_indexed_x(state);
    //     let addr = cpu::read_word(state, &address);
    //     let bank = state.dbr;
    //     let result = SnesAddress::from((bank, addr));
    //     return result;
}

/// Todo
pub fn direct_indirect(state: &mut State) -> SnesAddress {
    todo!();
    //     let address = direct(state);
    //     let address = cpu::read_word(state, &address);
    //     let bank = state.dbr;
    //     let result  = SnesAddress::from((bank, address));
    //     return result;
}

/// Todo
pub fn direct_indirect_long(state: &mut State) -> SnesAddress {
    todo!();
    //     let address = direct(state);
    //     let addr = cpu::read_word(state, &address);
    //     let bank = cpu::read_byte(state, &(address + 2));
    //     let result = SnesAddress::from((bank, addr));
    //     return result;
}

/// Todo
pub fn direct_indirect_indexed(state: &mut State) -> SnesAddress {
    todo!();
    //     let mut address = direct_indirect(state);
    //     let addr = address.get_address();
    //     let index = state.y.get_16();
    //     address.set_address(addr + index);
    //     return address;
}

// pub fn direct_indirect_indexed_long(state: &mut state::State) -> usize {
//     let address = direct_indirect_long(state);
//     let index = state.get_y16();
//     let result = address + index as usize;
//     return result;
// }

/// Absolute
///
/// Fetches the next two program bytes and combines them with [`State::dbr`] to
/// form an address.
///
/// # Address Formation
/// * bank = [`State::dbr`]
/// * address = [`cpu::fetch_word`]
/// * result = bank : address
///
/// # Params
/// * `state` - Current system state
///
/// # Returns
/// [`SnesAddress`]
pub fn absolute(state: &mut State) -> SnesAddress {
    let address = cpu::fetch_word(state);
    let bank = state.dbr;
    let result = SnesAddress::from((bank, address));
    return result;
}

/// Todo
pub fn absolute_indexed_x(state: &mut State) -> SnesAddress {
    todo!();
    //     let mut address = absolute(state);
    //     let index = state.x.get_16();
    //     let result = address + index as usize;
    //     return result;
}

/// Todo
pub fn absolute_indexed_y(state: &mut State) -> SnesAddress {
    todo!();
    //     let address = absolute(state);
    //     let index = state.get_y16();
    //     let result = address + index as usize;
    //     return result;
}

/// Todo
pub fn absolute_long(state: &mut State) -> SnesAddress {
    todo!();
    //     let mut bytes = [0; mem::size_of::<usize>()];
    //     bytes[0] = cpu::fetch_byte(state);
    //     bytes[1] = cpu::fetch_byte(state);
    //     bytes[2] = cpu::fetch_byte(state);
    //     let result = usize::from_le_bytes(bytes);
    //     return result;
}

/// Todo
pub fn absolute_long_indexed(state: &mut State) -> SnesAddress {
    todo!();
    //     let address = absolute_long(state);
    //     let index = state.get_x16();
    //     let result = address + index as usize;
    //     return result;
}

/// Todo
pub fn absolute_indirect(state: &mut State) -> SnesAddress {
    todo!();
    //     let address = absolute(state);
    //     let address = cpu::read_word(state, address);
    //     let result = combine_dbr(state, address);
    //     return result;
}

/// Todo
pub fn absolute_indexed_indirect(state: &mut State) -> SnesAddress {
    todo!();
    //     let
    //     let x = u16::from_le_bytes(state.x);
    //     let abs = cpu::fetch_word(state) + x;
    //     let result = state.mem.get_word(abs as usize);
    //     return result as usize;
}

/// Program counter relative
///
/// Forms a new address by fetching the next byte, and using it as
/// a signed offset from the program counter.
///
/// # Address Formation
/// * bank = [`State::get_pbr`]
/// * address = ([`cpu::fetch_byte`] as [i8]) + [`State::pc`][`SnesAddress::get_address`]
/// * result = bank : address
///
/// # Params
/// * `state` - Current system state
///
/// # Returns
/// [`SnesAddress`]
pub fn pc_relative(state: &mut State) -> SnesAddress {
    let offset = cpu::fetch_byte(state);
    let mut result = state.pc;
    let address = result.get_address().wrapping_add_signed(offset as i16);
    result.set_address(address);
    return result;
}

/// Todo
pub fn pc_relative_long(state: &mut State) -> SnesAddress {
    todo!();
    //     let pc = state.pc;
    //     let offset = cpu::fetch_word(state);
    //     let result = pc.wrapping_add_signed(offset as i16);
    //     return result as usize;
}

/// stack pointer relative
///
/// Forms a new address by fetching the next byte, and using it as
/// a signed offset from the stack pointer.
///
/// # Address Formation
/// * bank = [`State::s`][`SnesAddress::get_bank`]
/// * address = ([`cpu::fetch_byte`] as [i8]) + [`State::s`][`SnesAddress::get_address`]
/// * result = bank : address
///
/// # Params
/// * `state` - Current system state
///
/// # Returns
/// [`SnesAddress`]
pub fn stack_relative(state: &mut State) -> SnesAddress {
    let mut result = state.s;
    let offset = cpu::fetch_byte(state);
    let address = result.get_address().wrapping_add_signed(offset as i16);
    result.set_address(address);
    return result;
}

#[cfg(test)]
mod tests {
    use crate::memory;

    use super::*;

    #[test]
    fn test_immediate16() {
        let mut state = State::default();
        let expected = state.pc;
        let result = immediate16(&mut state);
        assert_eq!(result, expected);
        assert_eq!(state.pc.get_bank(), 0);
        assert_eq!(state.pc.get_address(), 2);
    }

    // Test direct page when D 0
    #[test]
    fn dp0() {
        let mut state = State::default();
        let expected = state.pc;
        let result = direct(&mut state);
        assert_eq!(result, expected);
        assert_eq!(state.pc.get_bank(), 0);
        assert_eq!(state.pc.get_address(), 1);
    }

    // Test direct page when D not 0
    #[test]
    fn dp1() {
        let mut state = State::default();
        state.mem[1] = 20;
        state.d = 20;
        state.dbr = 20;

        {
            let expected = SnesAddress::from((0, 20));
            let result = direct(&mut state);
            assert_eq!(result, expected);
        }

        {
            let expected = SnesAddress::from((0, 40));
            let result = direct(&mut state);
            assert_eq!(result, expected);
        }
    }

    // #[test]
    // fn dp_idx_x() {
    //     let mut state = State::default();
    //     state.mem[2] = 30;

    //     {
    //         let expected = SnesAddress::from((0, 0));
    //         let result = direct_indexed_x(&mut state);
    //         assert_eq!(expected, result);
    //     }

    //     {
    //         state.x.set_16(20);
    //         let expected = SnesAddress::from((0, 20));
    //         let result = direct_indexed_x(&mut state);
    //         assert_eq!(expected, result);
    //     }

    //     {
    //         let expected = SnesAddress::from((0, 50));
    //         let result = direct_indexed_x(&mut state);
    //         assert_eq!(expected, result);
    //     }
    // }

    // #[test]
    // fn dp_idx_y() {
    //     let mut state = State::default();
    //     state.mem[2] = 30;

    //     {
    //         let expected = SnesAddress::from((0, 0));
    //         let result = direct_indexed_x(&mut state);
    //         assert_eq!(expected, result);
    //     }

    //     {
    //         state.y.set_16(20);
    //         let expected = SnesAddress::from((0, 20));
    //         let result = direct_indexed_y(&mut state);
    //         assert_eq!(expected, result);
    //     }

    //     {
    //         let expected = SnesAddress::from((0, 50));
    //         let result = direct_indexed_y(&mut state);
    //         assert_eq!(expected, result);
    //     }
    // }

    // #[test]
    // fn dp_idx_indir() {
    //     let mut state = State::default();

    //     {
    //         state.mem[0] = 30;
    //         state.mem[30] = 10;
    //         let expected = SnesAddress::from((0, 10));
    //         let result = direct_indexed_indirect(&mut state);
    //         assert_eq!(expected, result);
    //     }

    //     {
    //         state.mem[40] = 7;
    //         state.x.set_16(40);
    //         let expected = SnesAddress::from((0, 7));
    //         let result = direct_indexed_indirect(&mut state);
    //         assert_eq!(expected, result);
    //     }

    //     {
    //         state.mem[2] = 50;
    //         state.mem[100] = 0x20;
    //         state.mem[101] = 0x67;
    //         state.dbr = 100;
    //         state.x.set_16(50);
    //         let expected = SnesAddress::from((100, 0x6720));
    //         let result = direct_indexed_indirect(&mut state);
    //         assert_eq!(expected, result);
    //     }
    // }
}
