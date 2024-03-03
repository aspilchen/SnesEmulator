//! Snes Addressing Modes
//!
//! Everything here will fetch and read as needed. These will change the [`sys`] in
//! a number of ways such as incrementing the [`sys::pc`], or modifying the [`sys::dbr`].
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

use crate::cartridge;
use crate::memory;
use crate::cpu::*;
use crate::snes::Snes;
use crate::cartridge::*;

/// Immediate
///
/// * Program counter = [`sys::pc`]
/// * Provides current program counter as address
/// * Increments program counter
///
/// # Address Formation
/// * `0` : [`sys::d`] + `byte`
///
/// # Note
/// Direct page bank is always zero
///
/// # Params
/// * `sys` - Current system sys
///
/// # Returns
/// [`usize`]
pub fn immediate16(sys: &mut Snes) -> usize {
    let result = get_pc_address(&sys.cpu);
    let _ = memory::io::word::fetch(&sys.mem, &mut sys.cpu);
    return result;
}

pub fn immediate8(sys: &mut Snes) -> usize {
    let result = get_pc_address(&sys.cpu);
    let _ = memory::io::byte::fetch(&sys.mem, &mut sys.cpu);
    return result;
}

/// Direct Page
///
/// Fetches the next byte and adds it with [`sys::d`]
///
/// # Address Formation
/// * `0` : [`sys::d`] + `byte`
///
/// # Note
/// Direct page bank is always zero
///
/// # Params
/// * `sys` - Current system sys
///
/// # Returns
/// [`usize`]
pub fn direct(sys: &mut Snes) -> usize {
    let cpu = &mut sys.cpu;
    let mem = &sys.mem;
    let byte =  memory::io::byte::fetch(mem, cpu);
    let direct_page = registers_16::get_d(cpu);
    
    let low_byte_mask = 0x00FF;
    let is_non_zero = (direct_page & low_byte_mask) != 0;
    if is_non_zero {
        let cycle_count = 1;
        add_cycles(cpu, cycle_count);
    }

    let bank = 0;
    let offset = direct_page.wrapping_add(byte as u16);
    let result = make_address(bank, offset);
    return result;
}

/// Todo
///
/// Takes the [`direct`] address and adds it with [`sys::x`].
///
/// # Address Formation
/// * bank = 0
/// * address = [`direct`] + [`sys::x`]
/// * result = bank : address
///
/// # Params
/// * `sys` - Current system sys
///
/// # Returns
/// [`usize`]
pub fn direct_indexed_x(sys: &mut Snes) -> usize {
    todo!();
    //     let mut address = direct(sys);
    //     let addr = address.get_address();
    //     let index = sys.x.get_16();
    //     address.set_address(addr + index);
    //     return address;
}

/// Todo
///
/// Takes the [`direct`] address and adds it with [`sys::y`].
///
/// # Address Formation
/// * bank = 0
/// * address = [`direct`] + [`sys::y`]
/// * result = bank : address
///
/// # Params
/// * `sys` - Current system sys
///
/// # Returns
/// [`usize`]
pub fn direct_indexed_y(sys: &mut Snes) -> usize {
    todo!();
    //     let mut address = direct(sys);
    //     let addr = address.get_address();
    //     let index = sys.y.get_16();
    //     address.set_address(addr + index);
    //     return address;
}

/// Todo
///
/// Reads two bytes from the [`direct_indexed_x`] and combines with [`sys::dbr`] to
/// form an address.
///
/// # Address Formation
/// * bank = [`sys::dbr`]
/// * address = read([`direct_indexed_x`])
/// * result = bank : address
///
/// # Params
/// * `sys` - Current system sys
///
/// # Returns
/// [`usize`]
pub fn direct_indexed_indirect(sys: &mut Snes) -> usize {
    todo!();
    //     let address = direct_indexed_x(sys);
    //     let addr = cpu::read_word(sys, &address);
    //     let bank = sys.dbr;
    //     let result = usize::from((bank, addr));
    //     return result;
}

/// Todo
pub fn direct_indirect(sys: &mut Snes) -> usize {
    todo!();
    //     let address = direct(sys);
    //     let address = cpu::read_word(sys, &address);
    //     let bank = sys.dbr;
    //     let result  = usize::from((bank, address));
    //     return result;
}

/// Todo
pub fn direct_indirect_long(sys: &mut Snes) -> usize {
    todo!();
    //     let address = direct(sys);
    //     let addr = cpu::read_word(sys, &address);
    //     let bank = cpu::read_byte(sys, &(address + 2));
    //     let result = usize::from((bank, addr));
    //     return result;
}

/// Todo
pub fn direct_indirect_indexed(sys: &mut Snes) -> usize {
    todo!();
    //     let mut address = direct_indirect(sys);
    //     let addr = address.get_address();
    //     let index = sys.y.get_16();
    //     address.set_address(addr + index);
    //     return address;
}

// pub fn direct_indirect_indexed_long(sys: &mut Snes::sys) -> usize {
//     let address = direct_indirect_long(sys);
//     let index = sys.get_y16();
//     let result = address + index as usize;
//     return result;
// }

/// Absolute
///
/// Fetches the next two program bytes and combines them with [`sys::dbr`] to
/// form an address.
///
/// # Address Formation
/// * bank = [`sys::dbr`]
/// * address = [`cpu::fetch_word`]
/// * result = bank : address
///
/// # Params
/// * `sys` - Current system sys
///
/// # Returns
/// [`usize`]
pub fn absolute(sys: &mut Snes) -> usize {
    let mem = &sys.mem;
    let cpu = &mut sys.cpu;
    let bank = registers_8::get_dbr(cpu);
    let offset = memory::io::word::fetch(mem, cpu);
    let result = cartridge::make_address(bank, offset);
    return result;
}

/// Todo
pub fn absolute_indexed_x(sys: &mut Snes) -> usize {
    todo!();
    //     let mut address = absolute(sys);
    //     let index = sys.x.get_16();
    //     let result = address + index as usize;
    //     return result;
}

/// Todo
pub fn absolute_indexed_y(sys: &mut Snes) -> usize {
    todo!();
    //     let address = absolute(sys);
    //     let index = sys.get_y16();
    //     let result = address + index as usize;
    //     return result;
}

/// Todo
pub fn absolute_long(sys: &mut Snes) -> usize {
    todo!();
    //     let mut bytes = [0; mem::size_of::<usize>()];
    //     bytes[0] = cpu::fetch_byte(sys);
    //     bytes[1] = cpu::fetch_byte(sys);
    //     bytes[2] = cpu::fetch_byte(sys);
    //     let result = usize::from_le_bytes(bytes);
    //     return result;
}

/// Todo
pub fn absolute_long_indexed(sys: &mut Snes) -> usize {
    todo!();
    //     let address = absolute_long(sys);
    //     let index = sys.get_x16();
    //     let result = address + index as usize;
    //     return result;
}

/// Todo
pub fn absolute_indirect(sys: &mut Snes) -> usize {
    todo!();
    //     let address = absolute(sys);
    //     let address = cpu::read_word(sys, address);
    //     let result = combine_dbr(sys, address);
    //     return result;
}

/// Todo
pub fn absolute_indexed_indirect(sys: &mut Snes) -> usize {
    todo!();
    //     let
    //     let x = u16::from_le_bytes(sys.x);
    //     let abs = cpu::fetch_word(sys) + x;
    //     let result = sys.mem.get_word(abs as usize);
    //     return result as usize;
}

/// Program counter relative
///
/// Forms a new address by fetching the next byte, and using it as
/// a signed offset from the program counter.
///
/// # Address Formation
/// * bank = [`sys::get_pbr`]
/// * address = ([`cpu::fetch_byte`] as [i8]) + [`sys::pc`][`usize::get_address`]
/// * result = bank : address
///
/// # Params
/// * `sys` - Current system sys
///
/// # Returns
/// [`usize`]
pub fn pc_relative(sys: &mut Snes) -> usize {
    todo!();
    // let byte = mmap.fetch(sys);
    // let bank = 0;
    // let offset = sys.cpu.pc.wrapping_add_signed(byte as i16);
    // let result = make_address(bank, offset);
    // return result;
}

/// Todo
pub fn pc_relative_long(sys: &mut Snes) -> usize {
    todo!();
    //     let pc = sys.pc;
    //     let offset = cpu::fetch_word(sys);
    //     let result = pc.wrapping_add_signed(offset as i16);
    //     return result as usize;
}

/// stack pointer relative
///
/// Forms a new address by fetching the next byte, and using it as
/// a signed offset from the stack pointer.
///
/// # Address Formation
/// * bank = [`sys::s`][`usize::get_bank`]
/// * address = ([`cpu::fetch_byte`] as [i8]) + [`sys::s`][`usize::get_address`]
/// * result = bank : address
///
/// # Params
/// * `sys` - Current system sys
///
/// # Returns
/// [`usize`]
pub fn stack_relative(sys: &mut Snes) -> usize {
    todo!();
    // let byte = mmap.fetch(sys);
    // let bank = 0;
    // let offset = sys.cpu.s.wrapping_add_signed(byte as i16);
    // let result = make_address(bank, offset);
    // return result;
}