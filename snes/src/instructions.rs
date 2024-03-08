//! opcode implementation
//!
//! Due to the many address modes, there are many repeating instructions that use
//! a different op code for each address mode. Furthurmore, each instruction can operate
//! on 8 or 16 bit values depending on status flag settings.
//!
//! I used some Python scripts to automate most of the repetative instructions.
//! Groups of repeated instructions will generally have a single helper function
//! holding the real logic. The generated code will typically acquire an
//! address using the respective addressing mode, determine the operand size, and
//! pass the information to the helper function.
//!
//! Single instructions, or instructions that follow a different pattern, are implemented manually.

use crate::address_mode;
use crate::address_mode::AddressMode;
use crate::cartridge::lo_rom::rom::wrapped_add;
use crate::memory;
use crate::ricoh5a22::*; // as cpu;
use crate::snes::Snes;

/// Helpers for 16 bit operations
mod helpers_16 {
    use super::*;
    use memory::io::word::*;
    use ops_16 as op;
    use registers_16 as reg;

    pub fn adc(sys: &mut Snes, address: usize) {
        let mem = &sys.mem;
        let cpu = &mut sys.cpu;
        let rhs = read(mem, address);
        op::adc(cpu, rhs);
        add_cycles(cpu, 1);
    }

    pub fn sbc(sys: &mut Snes, address: usize) {
        let mem = &sys.mem;
        let cpu = &mut sys.cpu;
        let rhs = read(mem, address);
        op::sbc(cpu, rhs);
        add_cycles(cpu, 1);
    }

    pub fn and(sys: &mut Snes, address: usize) {
        let mem = &sys.mem;
        let cpu = &mut sys.cpu;
        let rhs = read(mem, address);
        op::and(cpu, rhs);
        add_cycles(cpu, 1);
    }

    pub fn eor(sys: &mut Snes, address: usize) {
        let mem = &sys.mem;
        let cpu = &mut sys.cpu;
        let rhs = read(mem, address);
        op::eor(cpu, rhs);
        add_cycles(cpu, 1);
    }

    pub fn ora(sys: &mut Snes, address: usize) {
        let mem = &sys.mem;
        let cpu = &mut sys.cpu;
        let rhs = read(mem, address);
        op::ora(cpu, rhs);
        add_cycles(cpu, 1);
    }

    pub fn tsb(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let rhs = read(mem, address);
        let result = op::tsb(cpu, rhs);
        write(&mut sys.mem, address, result);
        add_cycles(cpu, 1);
    }

    pub fn trb(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let rhs = read(mem, address);
        let result = op::trb(cpu, rhs);
        write(&mut sys.mem, address, result);
        add_cycles(cpu, 1);
    }

    pub fn asl(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = read(mem, address);
        let result = op::asl(cpu, value);
        write(&mut sys.mem, address, result);
        add_cycles(cpu, 1);
    }

    pub fn asla(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_a(cpu);
        let result = op::asl(cpu, value);
        reg::set_a(cpu, result);
        add_cycles(cpu, 1);
    }

    pub fn lsr(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = read(mem, address);
        let result = op::lsr(cpu, value);
        write(mem, address, result);
        add_cycles(cpu, 1);
    }

    pub fn lsra(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_a(cpu);
        let result = op::lsr(cpu, value);
        reg::set_a(cpu, result);
        add_cycles(cpu, 1);
    }

    pub fn rol(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = read(mem, address);
        let result = op::rol(cpu, value);
        write(mem, address, result);
        add_cycles(cpu, 1);
    }

    pub fn rola(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_a(cpu);
        let result = op::rol(cpu, value);
        reg::set_a(cpu, result);
        add_cycles(cpu, 1);
    }

    pub fn ror(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = read(mem, address);
        let result = op::ror(cpu, value);
        write(mem, address, result);
        add_cycles(cpu, 1);
    }

    pub fn rora(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_a(cpu);
        let result = op::ror(cpu, value);
        reg::set_a(cpu, result);
        add_cycles(cpu, 1);
    }

    pub fn bit(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let rhs = read(mem, address);
        op::bit(cpu, rhs);
        add_cycles(cpu, 1);
    }

    pub fn cmp(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let lhs = reg::get_a(cpu);
        let rhs = read(mem, address);
        op::compare(cpu, lhs, rhs);
        add_cycles(cpu, 1);
    }

    pub fn cpx(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let lhs = reg::get_x(cpu);
        let rhs = read(mem, address);
        op::compare(cpu, lhs, rhs);
        add_cycles(cpu, 1);
    }

    pub fn cpy(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let lhs = reg::get_y(cpu);
        let rhs = read(mem, address);
        op::compare(cpu, lhs, rhs);
        add_cycles(cpu, 1);
    }

    pub fn dea(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_a(cpu);
        let result = op::dec(cpu, value);
        reg::set_a(cpu, result);
        add_cycles(cpu, 1);
    }

    pub fn dec(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = read(mem, address);
        let result = op::dec(cpu, value);
        write(mem, address, result);
        add_cycles(cpu, 1);
    }

    pub fn dex(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_x(cpu);
        let result = op::dec(cpu, value);
        reg::set_x(cpu, result);
        add_cycles(cpu, 1);
    }

    pub fn dey(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_y(cpu);
        let result = op::dec(cpu, value);
        reg::set_y(cpu, result);
        add_cycles(cpu, 1);
    }

    pub fn ina(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_a(cpu);
        let result = op::inc(cpu, value);
        reg::set_a(cpu, result);
        add_cycles(cpu, 1);
    }

    pub fn inc(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = read(mem, address);
        let result = op::inc(cpu, value);
        write(mem, address, result);
        add_cycles(cpu, 1);
    }

    pub fn inx(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_x(cpu);
        let result = op::inc(cpu, value);
        reg::set_x(cpu, result);
        add_cycles(cpu, 1);
    }

    pub fn iny(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_y(cpu);
        let result = op::inc(cpu, value);
        reg::set_y(cpu, result);
        add_cycles(cpu, 1);
    }

    pub fn lda(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = read(mem, address);
        op::lda(cpu, value);
        add_cycles(cpu, 1);
    }

    pub fn ldx(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = read(mem, address);
        op::ldx(cpu, value);
        add_cycles(cpu, 1);
    }

    pub fn ldy(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = read(mem, address);
        op::ldy(cpu, value);
        add_cycles(cpu, 1);
    }

    pub fn sta(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = reg::get_a(cpu);
        write(mem, address, value);
        add_cycles(cpu, 1);
    }

    pub fn sty(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = reg::get_y(cpu);
        write(mem, address, value);
        add_cycles(cpu, 1);
    }

    pub fn stx(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = reg::get_x(cpu);
        write(mem, address, value);
        add_cycles(cpu, 1);
    }

    pub fn stz(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = 0;
        write(mem, address, value);
        add_cycles(cpu, 1);
    }
}

/// Helpers for 8 bit operations
mod helpers_8 {
    use super::*;
    use memory::io::byte::*;
    use ops_8 as op;
    use registers_8 as reg;

    pub fn adc(sys: &mut Snes, address: usize) {
        let mem = &sys.mem;
        let cpu = &mut sys.cpu;
        let rhs = read(mem, address);
        op::adc(cpu, rhs);
    }

    pub fn sbc(sys: &mut Snes, address: usize) {
        let mem = &sys.mem;
        let cpu = &mut sys.cpu;
        let rhs = read(mem, address);
        op::sbc(cpu, rhs);
    }

    pub fn and(sys: &mut Snes, address: usize) {
        let mem = &sys.mem;
        let cpu = &mut sys.cpu;
        let rhs = read(mem, address);
        op::and(cpu, rhs);
    }

    pub fn eor(sys: &mut Snes, address: usize) {
        let mem = &sys.mem;
        let cpu = &mut sys.cpu;
        let rhs = read(mem, address);
        op::eor(cpu, rhs);
    }

    pub fn ora(sys: &mut Snes, address: usize) {
        let mem = &sys.mem;
        let cpu = &mut sys.cpu;
        let rhs = read(mem, address);
        op::ora(cpu, rhs);
    }

    pub fn tsb(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let rhs = read(mem, address);
        let result = op::tsb(cpu, rhs);
        write(&mut sys.mem, address, result);
    }

    pub fn trb(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let rhs = read(mem, address);
        let result = op::trb(cpu, rhs);
        write(&mut sys.mem, address, result);
    }

    pub fn asl(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = read(mem, address);
        let result = op::asl(cpu, value);
        write(&mut sys.mem, address, result);
    }

    pub fn asla(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_a(cpu);
        let result = op::asl(cpu, value);
        reg::set_a(cpu, result);
    }

    pub fn lsr(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = read(mem, address);
        let result = op::lsr(cpu, value);
        write(mem, address, result);
    }

    pub fn lsra(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_a(cpu);
        let result = op::lsr(cpu, value);
        reg::set_a(cpu, result);
    }

    pub fn rol(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = read(mem, address);
        let result = op::rol(cpu, value);
        write(mem, address, result);
    }

    pub fn rola(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_a(cpu);
        let result = op::rol(cpu, value);
        reg::set_a(cpu, result);
    }

    pub fn ror(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = read(mem, address);
        let result = op::ror(cpu, value);
        write(mem, address, result);
    }

    pub fn rora(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_a(cpu);
        let result = op::ror(cpu, value);
        reg::set_a(cpu, result);
    }

    pub fn bit(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let rhs = read(mem, address);
        op::bit(cpu, rhs);
    }

    pub fn cmp(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let lhs = reg::get_a(cpu);
        let rhs = read(mem, address);
        op::compare(cpu, lhs, rhs);
    }

    pub fn cpx(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let lhs = reg::get_x(cpu);
        let rhs = read(mem, address);
        op::compare(cpu, lhs, rhs);
    }

    pub fn cpy(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let lhs = reg::get_y(cpu);
        let rhs = read(mem, address);
        op::compare(cpu, lhs, rhs);
    }

    pub fn dea(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_a(cpu);
        let result = op::dec(cpu, value);
        reg::set_a(cpu, result);
    }

    pub fn dec(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = read(mem, address);
        let result = op::dec(cpu, value);
        write(mem, address, result);
    }

    pub fn dex(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_x(cpu);
        let result = op::dec(cpu, value);
        reg::set_x(cpu, result);
    }

    pub fn dey(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_y(cpu);
        let result = op::dec(cpu, value);
        reg::set_y(cpu, result);
    }

    pub fn ina(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_a(cpu);
        let result = op::inc(cpu, value);
        reg::set_a(cpu, result);
    }

    pub fn inc(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = read(mem, address);
        let result = op::inc(cpu, value);
        write(mem, address, result);
    }

    pub fn inx(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_x(cpu);
        let result = op::inc(cpu, value);
        reg::set_x(cpu, result);
    }

    pub fn iny(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_y(cpu);
        let result = op::inc(cpu, value);
        reg::set_y(cpu, result);
    }

    pub fn lda(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = read(mem, address);
        op::lda(cpu, value);
    }

    pub fn ldx(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = read(mem, address);
        op::ldx(cpu, value);
    }

    pub fn ldy(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = read(mem, address);
        op::ldy(cpu, value);
    }

    pub fn sta(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = reg::get_a(cpu);
        write(mem, address, value);
    }

    pub fn sty(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = reg::get_y(cpu);
        write(mem, address, value);
    }

    pub fn stx(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let cpu = &mut sys.cpu;
        let value = reg::get_x(cpu);
        write(mem, address, value);
    }

    pub fn stz(sys: &mut Snes, address: usize) {
        let mem = &mut sys.mem;
        let value = 0;
        write(mem, address, value);
    }
}

//================================================================================//
//                      Not Generic, implement manually                           //
//================================================================================//

pub fn jmp(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    let cpu = &mut sys.cpu;
    set_pc_address(cpu, address);
}

pub fn brk(sys: &mut Snes) {
    sys.cpu.pc += 1;
    // todo!();
}

pub fn cop(sys: &mut Snes) {
    todo!();
}

pub fn stp(sys: &mut Snes) {
    todo!();
}

pub fn wai(sys: &mut Snes) {
    todo!();
}

pub fn rti(sys: &mut Snes) {
    todo!();
}

pub fn bra(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    let cpu = &mut sys.cpu;
    set_pc_address(cpu, address);
}

pub fn jsr(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let mem = &mut sys.mem;
    let cpu = &mut sys.cpu;
    let new_addr = memory::io::word::fetch(mem, cpu);
    let curr_addr = registers_16::get_pc(cpu);
    memory::io::word::push(mem, cpu, curr_addr);
    registers_16::set_pc(cpu, new_addr);
}

pub fn mvn(sys: &mut Snes) {
    let mem = &mut sys.mem;
    let cpu = &mut sys.cpu;
    let dest_bank = memory::io::byte::fetch(mem, cpu);
    let dest_offset = registers_16::get_y(cpu);
    let src_bank = memory::io::byte::fetch(mem, cpu);
    let src_offset = registers_16::get_x(cpu);
    let n_bytes = (registers_16::get_a(cpu) as usize) + 1;
    let src_address = memory::make_address(src_bank, src_offset);
    let dest_address = memory::make_address(dest_bank, dest_offset);

    for byte in 0..n_bytes {
        let src = src_address - byte;
        let dest = dest_address - byte;
        let value = memory::io::byte::read(mem, src);
        memory::io::byte::write(mem, dest, value);
    }

    let subtract = registers_16::get_a(cpu);
    let new_x = src_offset - subtract - 1;
    let new_y = dest_offset - subtract - 1;
    let new_a = 0xFFFF;
    registers_16::set_a(cpu, new_a);
    registers_16::set_x(cpu, new_x);
    registers_16::set_y(cpu, new_y);
}

pub fn mvp(sys: &mut Snes) {
    let mem = &mut sys.mem;
    let cpu = &mut sys.cpu;
    let dest_bank = memory::io::byte::fetch(mem, cpu);
    let dest_offset = registers_16::get_y(cpu);
    let src_bank = memory::io::byte::fetch(mem, cpu);
    let src_offset = registers_16::get_x(cpu);
    let n_bytes = (registers_16::get_a(cpu) as usize) + 1;
    let src_address = memory::make_address(src_bank, src_offset);
    let dest_address = memory::make_address(dest_bank, dest_offset);

    for byte in 0..n_bytes {
        let src = src_address - byte;
        let dest = dest_address - byte;
        let value = memory::io::byte::read(mem, src);
        memory::io::byte::write(mem, dest, value);
    }

    let subtract = registers_16::get_a(cpu);
    let new_x = src_offset - subtract - 1;
    let new_y = dest_offset - subtract - 1;
    let new_a = 0xFFFF;
    registers_16::set_a(cpu, new_a);
    registers_16::set_x(cpu, new_x);
    registers_16::set_y(cpu, new_y);
}

pub fn nop(sys: &mut Snes) {

    // todo!();
}

pub fn pea(sys: &mut Snes) {
    let mem = &mut sys.mem;
    let cpu = &mut sys.cpu;
    let value = memory::io::word::fetch(mem, cpu);
    memory::io::word::push(mem, cpu, value);
}

pub fn pei(sys: &mut Snes) {
    let address = address_mode::direct(sys);
    let mem = &mut sys.mem;
    let cpu = &mut sys.cpu;
    let value = memory::io::word::read(mem, address);
    memory::io::word::push(mem, cpu, value);
}

pub fn per(sys: &mut Snes) {
    let mem = &mut sys.mem;
    let cpu = &mut sys.cpu;
    let value = memory::io::word::fetch(mem, cpu);
    let pc = registers_16::get_pc(cpu);
    let result = pc.wrapping_add(value);
    memory::io::word::push(mem, cpu, result);
}

pub fn rep(sys: &mut Snes) {
    let mem = &sys.mem;
    let cpu = &mut sys.cpu;
    let value = memory::io::byte::fetch(mem, cpu);
    status_flags::clear_status_bits(cpu, value);
}

pub fn rtl(sys: &mut Snes) {
    let mem = &sys.mem;
    let cpu = &mut sys.cpu;
    let offset = memory::io::word::pop(mem, cpu);
    let bank = memory::io::byte::pop(mem, cpu);
    registers_16::set_pc(cpu, offset);
    registers_8::set_pbr(cpu, bank);
}

pub fn rts(sys: &mut Snes) {
    let mem = &sys.mem;
    let cpu = &mut sys.cpu;
    let offset = memory::io::word::pop(mem, cpu);
    registers_16::set_pc(cpu, offset);
}

pub fn sep(sys: &mut Snes) {
    let mem = &sys.mem;
    let cpu = &mut sys.cpu;
    let value = memory::io::byte::fetch(mem, cpu);
    status_flags::set_status_bits(cpu, value);
}

pub fn tcd(sys: &mut Snes) {
    let cpu = &mut sys.cpu;
    let value = registers_16::get_a(cpu);
    registers_16::set_d(cpu, value);
}

pub fn tcs(sys: &mut Snes) {
    let cpu = &mut sys.cpu;
    let value = registers_16::get_a(cpu);
    registers_16::set_s(cpu, value);
}

pub fn tdc(sys: &mut Snes) {
    let cpu = &mut sys.cpu;
    let value = registers_16::get_d(cpu);
    registers_16::set_a(cpu, value);
}

pub fn tsc(sys: &mut Snes) {
    let cpu = &mut sys.cpu;
    let value = registers_16::get_s(cpu);
    registers_16::set_a(cpu, value);
}

pub fn tsx(sys: &mut Snes) {
    let cpu = &mut sys.cpu;
    let value = registers_16::get_s(cpu);
    if status_flags::is_set_x(cpu) {
        registers_8::set_x(cpu, value as u8);
    } else {
        registers_16::set_x(cpu, value);
    }
}

pub fn txs(sys: &mut Snes) {
    let cpu = &mut sys.cpu;
    if status_flags::is_set_x(cpu) {
        let value = registers_8::get_x(cpu);
        registers_16::set_s(cpu, value as u16);
    } else {
        let value = registers_16::get_x(cpu);
        registers_16::set_s(cpu, value);
    }
}

pub fn xba(sys: &mut Snes) {
    let cpu = &mut sys.cpu;
    let value = registers_16::get_a(cpu);
    let bytes = value.to_le_bytes();
    let new_value = u16::from_le_bytes([bytes[1], bytes[0]]);
    registers_16::set_a(cpu, new_value);
}

pub fn xce(sys: &mut Snes) {
    let cpu = &mut sys.cpu;
    let emulation = status_flags::is_set_e(cpu);
    let carry = status_flags::is_set_c(cpu);
    status_flags::set_e(cpu, carry);
    status_flags::set_c(cpu, emulation);
}

//================================================================================//
//                               Auto Generated                                   //
//================================================================================//

pub fn execute(sys: &mut Snes, op_code: u8) {
    match op_code {
        0x61 => adc(sys, &address_mode::AddressMode::DPIndexIndir),
        0x65 => adc(sys, &address_mode::AddressMode::Direct),
        0x67 => adc(sys, &address_mode::AddressMode::DPIndirLong),
        0x69 => adc(sys, &address_mode::AddressMode::ImmediateM),
        0x6D => adc(sys, &address_mode::AddressMode::Absolute),
        0x6F => adc(sys, &address_mode::AddressMode::AbsLong),
        0x71 => adc(sys, &address_mode::AddressMode::DPIndirIndex),
        0x72 => adc(sys, &address_mode::AddressMode::DPIndir),
        0x75 => adc(sys, &address_mode::AddressMode::DPIndexX),
        0x79 => adc(sys, &address_mode::AddressMode::AbsIndexY),
        0x7D => adc(sys, &address_mode::AddressMode::AbsIndexX),
        0x7F => adc(sys, &address_mode::AddressMode::AbsLongIndex),
        0x21 => and(sys, &address_mode::AddressMode::DPIndexIndir),
        0x25 => and(sys, &address_mode::AddressMode::Direct),
        0x27 => and(sys, &address_mode::AddressMode::DPIndirLong),
        0x29 => and(sys, &address_mode::AddressMode::ImmediateM),
        0x2D => and(sys, &address_mode::AddressMode::Absolute),
        0x2F => and(sys, &address_mode::AddressMode::AbsLong),
        0x31 => and(sys, &address_mode::AddressMode::DPIndirIndex),
        0x32 => and(sys, &address_mode::AddressMode::DPIndir),
        0x35 => and(sys, &address_mode::AddressMode::DPIndexX),
        0x39 => and(sys, &address_mode::AddressMode::AbsIndexY),
        0x3D => and(sys, &address_mode::AddressMode::AbsIndexX),
        0x3F => and(sys, &address_mode::AddressMode::AbsLongIndex),
        0x06 => asl(sys, &address_mode::AddressMode::Direct),
        0x0A => asla(sys),
        0x0E => asl(sys, &address_mode::AddressMode::Absolute),
        0x16 => asl(sys, &address_mode::AddressMode::DPIndexX),
        0x1E => asl(sys, &address_mode::AddressMode::AbsIndexX),
        0x90 => bcc(sys, &address_mode::AddressMode::PCRel),
        0xB0 => bcs(sys, &address_mode::AddressMode::PCRel),
        0xF0 => beq(sys, &address_mode::AddressMode::PCRel),
        0x24 => bit(sys, &address_mode::AddressMode::Direct),
        0x2C => bit(sys, &address_mode::AddressMode::Absolute),
        0x34 => bit(sys, &address_mode::AddressMode::DPIndexX),
        0x3C => bit(sys, &address_mode::AddressMode::AbsIndexX),
        0x89 => bit(sys, &address_mode::AddressMode::ImmediateM),
        0x30 => bmi(sys, &address_mode::AddressMode::PCRel),
        0xD0 => bne(sys, &address_mode::AddressMode::PCRel),
        0x10 => bpl(sys, &address_mode::AddressMode::PCRel),
        0x80 => bra(sys, &address_mode::AddressMode::PCRel),
        0x50 => bvc(sys, &address_mode::AddressMode::PCRel),
        0x70 => bvs(sys, &address_mode::AddressMode::PCRel),
        0x18 => clc(sys),
        0xD8 => cld(sys),
        0x58 => cli(sys),
        0xB8 => clv(sys),
        0xC1 => cmp(sys, &address_mode::AddressMode::DPIndexIndir),
        0xC5 => cmp(sys, &address_mode::AddressMode::Direct),
        0xC7 => cmp(sys, &address_mode::AddressMode::DPIndirLong),
        0xC9 => cmp(sys, &address_mode::AddressMode::ImmediateM),
        0xCD => cmp(sys, &address_mode::AddressMode::Absolute),
        0xCF => cmp(sys, &address_mode::AddressMode::AbsLong),
        0xD1 => cmp(sys, &address_mode::AddressMode::DPIndirIndex),
        0xD2 => cmp(sys, &address_mode::AddressMode::DPIndir),
        0xD5 => cmp(sys, &address_mode::AddressMode::DPIndexX),
        0xD9 => cmp(sys, &address_mode::AddressMode::AbsIndexY),
        0xDD => cmp(sys, &address_mode::AddressMode::AbsIndexX),
        0xDF => cmp(sys, &address_mode::AddressMode::AbsLongIndex),
        0xE0 => cpx(sys, &address_mode::AddressMode::ImmediateX),
        0xE4 => cpx(sys, &address_mode::AddressMode::Direct),
        0xEC => cpx(sys, &address_mode::AddressMode::Absolute),
        0xC0 => cpy(sys, &address_mode::AddressMode::ImmediateX),
        0xC4 => cpy(sys, &address_mode::AddressMode::Direct),
        0xCC => cpy(sys, &address_mode::AddressMode::Absolute),
        0x3A => dea(sys),
        0xC6 => dec(sys, &address_mode::AddressMode::Direct),
        0xCE => dec(sys, &address_mode::AddressMode::Absolute),
        0xD6 => dec(sys, &address_mode::AddressMode::DPIndexX),
        0xDE => dec(sys, &address_mode::AddressMode::AbsIndexX),
        0xCA => dex(sys),
        0x88 => dey(sys),
        0x41 => eor(sys, &address_mode::AddressMode::DPIndexIndir),
        0x45 => eor(sys, &address_mode::AddressMode::Direct),
        0x47 => eor(sys, &address_mode::AddressMode::DPIndirLong),
        0x49 => eor(sys, &address_mode::AddressMode::ImmediateM),
        0x4D => eor(sys, &address_mode::AddressMode::Absolute),
        0x4F => eor(sys, &address_mode::AddressMode::AbsLong),
        0x51 => eor(sys, &address_mode::AddressMode::DPIndirIndex),
        0x52 => eor(sys, &address_mode::AddressMode::DPIndir),
        0x55 => eor(sys, &address_mode::AddressMode::DPIndexX),
        0x59 => eor(sys, &address_mode::AddressMode::AbsIndexY),
        0x5D => eor(sys, &address_mode::AddressMode::AbsIndexX),
        0x5F => eor(sys, &address_mode::AddressMode::AbsLongIndex),
        0x1A => ina(sys),
        0xE6 => inc(sys, &address_mode::AddressMode::Direct),
        0xEE => inc(sys, &address_mode::AddressMode::Absolute),
        0xF6 => inc(sys, &address_mode::AddressMode::DPIndexX),
        0xFE => inc(sys, &address_mode::AddressMode::AbsIndexX),
        0xE8 => inx(sys),
        0xC8 => iny(sys),
        0x4C => jmp(sys, &address_mode::AddressMode::Absolute),
        0x5C => jmp(sys, &address_mode::AddressMode::AbsLong),
        0x20 => jsr(sys, &address_mode::AddressMode::Absolute),
        0x22 => jsr(sys, &address_mode::AddressMode::AbsLong),
        0xA1 => lda(sys, &address_mode::AddressMode::DPIndexIndir),
        0xA5 => lda(sys, &address_mode::AddressMode::Direct),
        0xA7 => lda(sys, &address_mode::AddressMode::DPIndirLong),
        0xA9 => lda(sys, &address_mode::AddressMode::ImmediateM),
        0xAD => lda(sys, &address_mode::AddressMode::Absolute),
        0xAF => lda(sys, &address_mode::AddressMode::AbsLong),
        0xB1 => lda(sys, &address_mode::AddressMode::DPIndirIndex),
        0xB2 => lda(sys, &address_mode::AddressMode::DPIndir),
        0xB5 => lda(sys, &address_mode::AddressMode::DPIndexX),
        0xB9 => lda(sys, &address_mode::AddressMode::AbsIndexY),
        0xBD => lda(sys, &address_mode::AddressMode::AbsIndexX),
        0xBF => lda(sys, &address_mode::AddressMode::AbsLongIndex),
        0xA2 => ldx(sys, &address_mode::AddressMode::ImmediateX),
        0xA6 => ldx(sys, &address_mode::AddressMode::Direct),
        0xAE => ldx(sys, &address_mode::AddressMode::Absolute),
        0xB6 => ldx(sys, &address_mode::AddressMode::DPIndexY),
        0xBE => ldx(sys, &address_mode::AddressMode::AbsIndexY),
        0xA0 => ldy(sys, &address_mode::AddressMode::ImmediateX),
        0xA4 => ldy(sys, &address_mode::AddressMode::Direct),
        0xAC => ldy(sys, &address_mode::AddressMode::Absolute),
        0xB4 => ldy(sys, &address_mode::AddressMode::DPIndexX),
        0xBC => ldy(sys, &address_mode::AddressMode::AbsIndexX),
        0x46 => lsr(sys, &address_mode::AddressMode::Direct),
        0x4A => lsra(sys),
        0x4E => lsr(sys, &address_mode::AddressMode::Absolute),
        0x56 => lsr(sys, &address_mode::AddressMode::DPIndexX),
        0x5E => lsr(sys, &address_mode::AddressMode::AbsIndexX),
        0xEA => nop(sys),
        0x01 => ora(sys, &address_mode::AddressMode::DPIndexIndir),
        0x05 => ora(sys, &address_mode::AddressMode::Direct),
        0x07 => ora(sys, &address_mode::AddressMode::DPIndirLong),
        0x09 => ora(sys, &address_mode::AddressMode::ImmediateM),
        0x0D => ora(sys, &address_mode::AddressMode::Absolute),
        0x0F => ora(sys, &address_mode::AddressMode::AbsLong),
        0x11 => ora(sys, &address_mode::AddressMode::DPIndirIndex),
        0x12 => ora(sys, &address_mode::AddressMode::DPIndir),
        0x15 => ora(sys, &address_mode::AddressMode::DPIndexX),
        0x19 => ora(sys, &address_mode::AddressMode::AbsIndexY),
        0x1D => ora(sys, &address_mode::AddressMode::AbsIndexX),
        0x1F => ora(sys, &address_mode::AddressMode::AbsLongIndex),
        0x26 => rol(sys, &address_mode::AddressMode::Direct),
        0x2A => rola(sys),
        0x2E => rol(sys, &address_mode::AddressMode::Absolute),
        0x36 => rol(sys, &address_mode::AddressMode::DPIndexX),
        0x3E => rol(sys, &address_mode::AddressMode::AbsIndexX),
        0x66 => ror(sys, &address_mode::AddressMode::Direct),
        0x6A => rora(sys),
        0x6E => ror(sys, &address_mode::AddressMode::Absolute),
        0x76 => ror(sys, &address_mode::AddressMode::DPIndexX),
        0x7E => ror(sys, &address_mode::AddressMode::AbsIndexX),
        0x60 => rts(sys),
        0xE1 => sbc(sys, &address_mode::AddressMode::DPIndexIndir),
        0xE5 => sbc(sys, &address_mode::AddressMode::Direct),
        0xE7 => sbc(sys, &address_mode::AddressMode::DPIndirLong),
        0xE9 => sbc(sys, &address_mode::AddressMode::ImmediateM),
        0xED => sbc(sys, &address_mode::AddressMode::Absolute),
        0xEF => sbc(sys, &address_mode::AddressMode::AbsLong),
        0xF1 => sbc(sys, &address_mode::AddressMode::DPIndirIndex),
        0xF2 => sbc(sys, &address_mode::AddressMode::DPIndir),
        0xF5 => sbc(sys, &address_mode::AddressMode::DPIndexX),
        0xF9 => sbc(sys, &address_mode::AddressMode::AbsIndexY),
        0xFD => sbc(sys, &address_mode::AddressMode::AbsIndexX),
        0xFF => sbc(sys, &address_mode::AddressMode::AbsLongIndex),
        0x38 => sec(sys),
        0xF8 => sed(sys),
        0x78 => sei(sys),
        0x81 => sta(sys, &address_mode::AddressMode::DPIndexIndir),
        0x85 => sta(sys, &address_mode::AddressMode::Direct),
        0x87 => sta(sys, &address_mode::AddressMode::DPIndirLong),
        0x8D => sta(sys, &address_mode::AddressMode::Absolute),
        0x8F => sta(sys, &address_mode::AddressMode::AbsLong),
        0x91 => sta(sys, &address_mode::AddressMode::DPIndirIndex),
        0x92 => sta(sys, &address_mode::AddressMode::DPIndir),
        0x95 => sta(sys, &address_mode::AddressMode::DPIndexX),
        0x99 => sta(sys, &address_mode::AddressMode::AbsIndexY),
        0x9D => sta(sys, &address_mode::AddressMode::AbsIndexX),
        0x9F => sta(sys, &address_mode::AddressMode::AbsLongIndex),
        0xDB => stp(sys),
        0x86 => stx(sys, &address_mode::AddressMode::Direct),
        0x8E => stx(sys, &address_mode::AddressMode::Absolute),
        0x96 => stx(sys, &address_mode::AddressMode::DPIndexY),
        0x84 => sty(sys, &address_mode::AddressMode::Direct),
        0x8C => sty(sys, &address_mode::AddressMode::Absolute),
        0x94 => sty(sys, &address_mode::AddressMode::DPIndexX),
        0x64 => stz(sys, &address_mode::AddressMode::Direct),
        0x74 => stz(sys, &address_mode::AddressMode::DPIndexX),
        0x9C => stz(sys, &address_mode::AddressMode::Absolute),
        0x9E => stz(sys, &address_mode::AddressMode::AbsIndexX),
        0xAA => tax(sys),
        0xA8 => tay(sys),
        0x5B => tcd(sys),
        0x1B => tcs(sys),
        0x7B => tdc(sys),
        0x14 => trb(sys, &address_mode::AddressMode::Direct),
        0x1C => trb(sys, &address_mode::AddressMode::Absolute),
        0x04 => tsb(sys, &address_mode::AddressMode::Direct),
        0x0C => tsb(sys, &address_mode::AddressMode::Absolute),
        0x3B => tsc(sys),
        0xBA => tsx(sys),
        0x8A => txa(sys),
        0x9A => txs(sys),
        0x9B => txy(sys),
        0x98 => tya(sys),
        0xBB => tyx(sys),
        0xCB => wai(sys),
        0xEB => xba(sys),
        0xFB => xce(sys),

        _ => todo!("0x{:02X}", op_code),
    }
}

pub fn adc(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::adc(sys, address);
    } else {
        helpers_16::adc(sys, address);
    }
}

pub fn and(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::and(sys, address);
    } else {
        helpers_16::and(sys, address);
    }
}

pub fn bit(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::bit(sys, address);
    } else {
        helpers_16::bit(sys, address);
    }
}

pub fn cmp(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::cmp(sys, address);
    } else {
        helpers_16::cmp(sys, address);
    }
}

pub fn inc(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::inc(sys, address);
    } else {
        helpers_16::inc(sys, address);
    }
}

pub fn dec(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::dec(sys, address);
    } else {
        helpers_16::dec(sys, address);
    }
}

pub fn rol(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::rol(sys, address);
    } else {
        helpers_16::rol(sys, address);
    }
}

pub fn ror(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::ror(sys, address);
    } else {
        helpers_16::ror(sys, address);
    }
}

pub fn asl(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::asl(sys, address);
    } else {
        helpers_16::asl(sys, address);
    }
}

pub fn lsr(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::lsr(sys, address);
    } else {
        helpers_16::lsr(sys, address);
    }
}

pub fn eor(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::eor(sys, address);
    } else {
        helpers_16::eor(sys, address);
    }
}

pub fn lda(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::lda(sys, address);
    } else {
        helpers_16::lda(sys, address);
    }
}

pub fn ora(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::ora(sys, address);
    } else {
        helpers_16::ora(sys, address);
    }
}

pub fn sbc(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::sbc(sys, address);
    } else {
        helpers_16::sbc(sys, address);
    }
}

pub fn sta(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::sta(sys, address);
    } else {
        helpers_16::sta(sys, address);
    }
}

pub fn stz(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::stz(sys, address);
    } else {
        helpers_16::stz(sys, address);
    }
}

pub fn trb(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::trb(sys, address);
    } else {
        helpers_16::trb(sys, address);
    }
}

pub fn tsb(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::tsb(sys, address);
    } else {
        helpers_16::tsb(sys, address);
    }
}

pub fn ldx(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_x(&sys.cpu) {
        helpers_8::ldx(sys, address);
    } else {
        helpers_16::ldx(sys, address);
    }
}

pub fn ldy(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_x(&sys.cpu) {
        helpers_8::ldy(sys, address);
    } else {
        helpers_16::ldy(sys, address);
    }
}

pub fn stx(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_x(&sys.cpu) {
        helpers_8::stx(sys, address);
    } else {
        helpers_16::stx(sys, address);
    }
}

pub fn sty(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_x(&sys.cpu) {
        helpers_8::sty(sys, address);
    } else {
        helpers_16::sty(sys, address);
    }
}

pub fn cpx(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_x(&sys.cpu) {
        helpers_8::cpx(sys, address);
    } else {
        helpers_16::cpx(sys, address);
    }
}

pub fn cpy(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_x(&sys.cpu) {
        helpers_8::cpy(sys, address);
    } else {
        helpers_16::cpy(sys, address);
    }
}

pub fn iny(sys: &mut Snes) {
    if status_flags::is_set_x(&sys.cpu) {
        helpers_8::iny(sys);
    } else {
        helpers_16::iny(sys);
    }
}

pub fn ina(sys: &mut Snes) {
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::ina(sys);
    } else {
        helpers_16::ina(sys);
    }
}

pub fn dex(sys: &mut Snes) {
    if status_flags::is_set_x(&sys.cpu) {
        helpers_8::dex(sys);
    } else {
        helpers_16::dex(sys);
    }
}

pub fn inx(sys: &mut Snes) {
    if status_flags::is_set_x(&sys.cpu) {
        helpers_8::inx(sys);
    } else {
        helpers_16::inx(sys);
    }
}

pub fn rola(sys: &mut Snes) {
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::rola(sys);
    } else {
        helpers_16::rola(sys);
    }
}

pub fn asla(sys: &mut Snes) {
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::asla(sys);
    } else {
        helpers_16::asla(sys);
    }
}

pub fn lsra(sys: &mut Snes) {
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::lsra(sys);
    } else {
        helpers_16::lsra(sys);
    }
}

pub fn rora(sys: &mut Snes) {
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::rora(sys);
    } else {
        helpers_16::rora(sys);
    }
}

pub fn dea(sys: &mut Snes) {
    if status_flags::is_set_m(&sys.cpu) {
        helpers_8::dea(sys);
    } else {
        helpers_16::dea(sys);
    }
}

pub fn dey(sys: &mut Snes) {
    if status_flags::is_set_x(&sys.cpu) {
        helpers_8::dey(sys);
    } else {
        helpers_16::dey(sys);
    }
}

pub fn bcc(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if !status_flags::is_set_c(&sys.cpu) {
        set_pc_address(&mut sys.cpu, address);
    }
}

pub fn bcs(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_c(&sys.cpu) {
        set_pc_address(&mut sys.cpu, address);
    }
}

pub fn bne(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if !status_flags::is_set_z(&sys.cpu) {
        set_pc_address(&mut sys.cpu, address);
    }
}

pub fn beq(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_z(&sys.cpu) {
        set_pc_address(&mut sys.cpu, address);
    }
}

pub fn bpl(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if !status_flags::is_set_n(&sys.cpu) {
        set_pc_address(&mut sys.cpu, address);
    }
}

pub fn bmi(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_n(&sys.cpu) {
        set_pc_address(&mut sys.cpu, address);
    }
}

pub fn bvc(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if !status_flags::is_set_v(&sys.cpu) {
        set_pc_address(&mut sys.cpu, address);
    }
}

pub fn bvs(sys: &mut Snes, mode: &address_mode::AddressMode) {
    let address = address_mode::deref_address(sys, mode);
    if status_flags::is_set_v(&sys.cpu) {
        set_pc_address(&mut sys.cpu, address);
    }
}

pub fn clc(sys: &mut Snes) {
    status_flags::set_c(&mut sys.cpu, false);
}

pub fn cld(sys: &mut Snes) {
    status_flags::set_d(&mut sys.cpu, false);
}

pub fn cli(sys: &mut Snes) {
    status_flags::set_i(&mut sys.cpu, false);
}

pub fn clv(sys: &mut Snes) {
    status_flags::set_v(&mut sys.cpu, false);
}

pub fn sec(sys: &mut Snes) {
    status_flags::set_c(&mut sys.cpu, true);
}

pub fn sed(sys: &mut Snes) {
    status_flags::set_d(&mut sys.cpu, true);
}

pub fn sei(sys: &mut Snes) {
    status_flags::set_i(&mut sys.cpu, true);
}

pub fn tax(sys: &mut Snes) {
    if status_flags::is_set_x(&sys.cpu) {
        let value = registers_8::get_a(&sys.cpu);
        registers_8::set_x(&mut sys.cpu, value);
    } else {
        let value = registers_16::get_a(&sys.cpu);
        registers_16::set_x(&mut sys.cpu, value);
    }
}

pub fn tay(sys: &mut Snes) {
    if status_flags::is_set_x(&sys.cpu) {
        let value = registers_8::get_a(&sys.cpu);
        registers_8::set_y(&mut sys.cpu, value);
    } else {
        let value = registers_16::get_a(&sys.cpu);
        registers_16::set_y(&mut sys.cpu, value);
    }
}

pub fn txa(sys: &mut Snes) {
    if status_flags::is_set_m(&sys.cpu) {
        let value = registers_8::get_x(&sys.cpu);
        registers_8::set_a(&mut sys.cpu, value);
    } else {
        let value = registers_16::get_x(&sys.cpu);
        registers_16::set_a(&mut sys.cpu, value);
    }
}

pub fn tya(sys: &mut Snes) {
    if status_flags::is_set_m(&sys.cpu) {
        let value = registers_8::get_y(&sys.cpu);
        registers_8::set_a(&mut sys.cpu, value);
    } else {
        let value = registers_16::get_y(&sys.cpu);
        registers_16::set_a(&mut sys.cpu, value);
    }
}

pub fn txy(sys: &mut Snes) {
    if status_flags::is_set_x(&sys.cpu) {
        let value = registers_8::get_x(&sys.cpu);
        registers_8::set_y(&mut sys.cpu, value);
    } else {
        let value = registers_16::get_x(&sys.cpu);
        registers_16::set_y(&mut sys.cpu, value);
    }
}

pub fn tyx(sys: &mut Snes) {
    if status_flags::is_set_x(&sys.cpu) {
        let value = registers_8::get_y(&sys.cpu);
        registers_8::set_x(&mut sys.cpu, value);
    } else {
        let value = registers_16::get_y(&sys.cpu);
        registers_16::set_x(&mut sys.cpu, value);
    }
}
