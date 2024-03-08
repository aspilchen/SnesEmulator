//! Snes Addressing Modes
//!
//! Dereference address to be used for read/write operations.
//! Each instruction has an address mode that determines where
//! to find an operand (if one is needed).
use std::ops::Add;

use crate::memory;
use crate::ricoh5a22::*;
use crate::snes::Snes;

pub enum AddressMode {
    AbsIndexIndir,
    AbsIndexX,
    AbsIndexY,
    AbsIndir,
    AbsLong,
    AbsLongIndex,
    Absolute,
    Direct,
    DPIndexIndir,
    DPIndexX,
    DPIndexY,
    DPIndir,
    DPIndirIndex,
    DPIndirIndexLong,
    DPIndirLong,
    ImmediateM,
    ImmediateX,
    PCRel,
}

pub fn deref_address(sys: &mut Snes, mode: &AddressMode) -> usize {
    use AddressMode::*;

    match mode {
        AbsIndexIndir => absolute_indexed_indirect(sys),
        AbsIndexX => absolute_indexed_x(sys),
        AbsIndexY => absolute_indexed_y(sys),
        AbsIndir => absolute_indirect(sys),
        AbsLong => absolute_long(sys),
        AbsLongIndex => absolute_long_indexed(sys),
        Absolute => absolute(sys),
        Direct => direct(sys),
        DPIndexIndir => direct_indexed_indirect(sys),
        DPIndexX => direct_indexed_x(sys),
        DPIndexY => direct_indexed_y(sys),
        DPIndir => direct_indirect(sys),
        DPIndirIndex => direct_indirect_indexed(sys),
        DPIndirIndexLong => direct_indirect_indexed_long(sys),
        DPIndirLong => direct_indirect_long(sys),
        ImmediateM => immediate_m(sys),
        ImmediateX => immediate_x(sys),
        PCRel => pc_relative(sys),
        _ => panic!("Address mode not implemented"),
    }
}

pub fn immediate_m(sys: &mut Snes) -> usize {
    let result = get_pc_address(&sys.cpu);

    let increment = if status_flags::is_set_m(&sys.cpu) {
        1
    } else {
        2
    };

    let new_pc = memory::move_pc(result, &sys.mem.cartridge_type, increment);
    set_pc_address(&mut sys.cpu, new_pc);

    return result;
}

pub fn immediate_x(sys: &mut Snes) -> usize {
    let result = get_pc_address(&sys.cpu);

    let increment = if status_flags::is_set_m(&sys.cpu) {
        1
    } else {
        2
    };

    let new_pc = memory::move_pc(result, &sys.mem.cartridge_type, increment);
    set_pc_address(&mut sys.cpu, new_pc);

    return result;
}

pub fn direct(sys: &mut Snes) -> usize {
    let cpu = &mut sys.cpu;
    let mem = &sys.mem;
    let value = memory::io::byte::fetch(mem, cpu);
    let direct_page = registers_16::get_d(cpu);
    let low_byte_mask = 0x00FF;
    let is_non_zero = (direct_page & low_byte_mask) != 0;
    if is_non_zero {
        let cycle_count = 1;
        add_cycles(cpu, cycle_count);
    }

    let bank = 0;
    let offset = direct_page.wrapping_add(value as u16);
    let result = memory::make_address(bank, offset);
    return result;
}

pub fn direct_indexed_x(sys: &mut Snes) -> usize {
    let address = direct(sys);
    let cpu = &mut sys.cpu;
    let index = if status_flags::is_set_x(cpu) {
        registers_8::get_x(cpu) as usize
    } else {
        registers_16::get_x(cpu) as usize
    };
    let result = address + index;
    return result;
}

pub fn direct_indexed_y(sys: &mut Snes) -> usize {
    let address = direct(sys);
    let cpu = &mut sys.cpu;
    let index = if status_flags::is_set_x(cpu) {
        registers_8::get_y(cpu) as usize
    } else {
        registers_16::get_y(cpu) as usize
    };
    let result = address + index;
    return result;
}

pub fn direct_indexed_indirect(sys: &mut Snes) -> usize {
    let address = direct_indexed_x(sys);
    let offset = memory::io::word::read(&sys.mem, address);
    let bank = registers_8::get_dbr(&sys.cpu);
    let result = memory::make_address(bank, offset);
    return result;
}

pub fn direct_indirect(sys: &mut Snes) -> usize {
    let address = direct(sys);
    let offset = memory::io::word::read(&sys.mem, address);
    let bank = registers_8::get_dbr(&sys.cpu);
    let result = memory::make_address(bank, offset);
    return result;
}

pub fn direct_indirect_long(sys: &mut Snes) -> usize {
    let address = direct(sys);
    let mut bytes = [0; std::mem::size_of::<usize>()];
    let mem = &sys.mem;
    bytes[0] = memory::io::byte::read(mem, address);
    bytes[1] = memory::io::byte::read(mem, address + 1);
    bytes[2] = memory::io::byte::read(mem, address + 2);
    let result = usize::from_le_bytes(bytes);
    return result;
}

pub fn direct_indirect_indexed(sys: &mut Snes) -> usize {
    let address = direct_indirect(sys);
    let index = if status_flags::is_set_x(&sys.cpu) {
        registers_8::get_y(&sys.cpu) as usize
    } else {
        registers_16::get_y(&sys.cpu) as usize
    };
    let result = address + index;
    return result;
}

pub fn direct_indirect_indexed_long(sys: &mut Snes) -> usize {
    let address = direct_indirect_long(sys);
    let index = if status_flags::is_set_x(&sys.cpu) {
        registers_8::get_y(&sys.cpu) as usize
    } else {
        registers_16::get_y(&sys.cpu) as usize
    };
    let result = address + index as usize;
    return result;
}

pub fn absolute(sys: &mut Snes) -> usize {
    let mem = &sys.mem;
    let cpu = &mut sys.cpu;
    let bank = registers_8::get_dbr(cpu);
    let offset = memory::io::word::fetch(mem, cpu);
    let result = memory::make_address(bank, offset);
    return result;
}

pub fn absolute_indexed_x(sys: &mut Snes) -> usize {
    let address = absolute(sys);
    let index = if status_flags::is_set_x(&sys.cpu) {
        registers_8::get_x(&sys.cpu) as usize
    } else {
        registers_16::get_x(&sys.cpu) as usize
    };
    let result = address + index;
    return result;
}

pub fn absolute_indexed_y(sys: &mut Snes) -> usize {
    let address = absolute(sys);
    let index = if status_flags::is_set_x(&sys.cpu) {
        registers_8::get_x(&sys.cpu) as usize
    } else {
        registers_16::get_x(&sys.cpu) as usize
    };
    let result = address + index;
    return result;
}

pub fn absolute_long(sys: &mut Snes) -> usize {
    let mem = &mut sys.mem;
    let cpu = &mut sys.cpu;
    let mut bytes = [0; std::mem::size_of::<usize>()];
    bytes[0] = memory::io::byte::fetch(mem, cpu);
    bytes[1] = memory::io::byte::fetch(mem, cpu);
    bytes[2] = memory::io::byte::fetch(mem, cpu);
    let result = usize::from_le_bytes(bytes);
    return result;
}

pub fn absolute_long_indexed(sys: &mut Snes) -> usize {
    let address = absolute_long(sys);
    let index = if status_flags::is_set_x(&sys.cpu) {
        registers_8::get_x(&sys.cpu) as usize
    } else {
        registers_16::get_x(&sys.cpu) as usize
    };
    let result = address + index;
    return result;
}

pub fn absolute_indirect(sys: &mut Snes) -> usize {
    let address = absolute(sys);
    let offset = memory::io::word::read(&sys.mem, address);
    let bank = registers_8::get_dbr(&sys.cpu);
    let result = memory::make_address(bank, offset);
    return result;
}

pub fn absolute_indexed_indirect(sys: &mut Snes) -> usize {
    let address = absolute_indexed_x(sys);
    let offset = memory::io::word::read(&sys.mem, address);
    let bank = registers_8::get_dbr(&sys.cpu);
    let result = memory::make_address(bank, offset);
    return result;
}

pub fn pc_relative(sys: &mut Snes) -> usize {
    let cpu = &mut sys.cpu;
    let mem = &sys.mem;
    let offset = memory::io::byte::fetch(mem, cpu) as i8;
    let curr_pc = get_pc_address(cpu);
    let result = curr_pc.wrapping_add_signed(offset as isize);
    return result;
}

pub fn pc_relative_long(sys: &mut Snes) -> usize {
    let offset = memory::io::word::fetch(&sys.mem, &mut sys.cpu);
    let curr_pc = get_pc_address(&sys.cpu);
    let result = curr_pc.wrapping_add_signed(offset as isize);
    return result;
}

pub fn stack_relative(sys: &mut Snes) -> usize {
    let cpu = &mut sys.cpu;
    let mem = &sys.mem;
    let offset = memory::io::byte::fetch(mem, cpu);
    let curr_s = registers_16::get_s(cpu) as usize;
    let result = curr_s.wrapping_add_signed(offset as isize);
    return result;
}
