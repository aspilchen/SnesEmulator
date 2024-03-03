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

use debug_print::debug_println;

use crate::address_mode;
use crate::cpu;
use crate::memory;
use crate::snes::Snes;

/// Helpers for 16 bit operations
mod helpers_16 {
    use super::*;
    use cpu::ops_16 as op;
    use cpu::registers_16 as reg;
    use memory::io::word::*;

    pub fn adc(sys: &mut Snes, address: usize) {
        let rhs = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        op::adc(cpu, rhs);
    }

    pub fn sbc(sys: &mut Snes, address: usize) {
        let rhs = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        op::sbc(cpu, rhs);
    }

    pub fn and(sys: &mut Snes, address: usize) {
        let rhs = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        op::and(cpu, rhs);
    }

    pub fn eor(sys: &mut Snes, address: usize) {
        let rhs = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        op::eor(cpu, rhs);
    }

    pub fn ora(sys: &mut Snes, address: usize) {
        let rhs = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        op::ora(cpu, rhs);
    }

    pub fn tsb(sys: &mut Snes, address: usize) {
        let rhs = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        let result = op::tsb(cpu, rhs);
        write(&mut sys.mem, address, result);
    }

    pub fn trb(sys: &mut Snes, address: usize) {
        let rhs = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        let result = op::trb(cpu, rhs);
        write(&mut sys.mem, address, result);
    }

    pub fn asl(sys: &mut Snes, address: usize) {
        let value = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        let result = op::asl(cpu, value);
        write(&mut sys.mem, address, result);
    }

    pub fn asla(sys: &mut Snes) {
        let value = reg::get_a(&sys.cpu);
        let cpu = &mut sys.cpu;
        let result = op::asl(cpu, value);
        reg::set_a(&mut sys.cpu, result);
    }

    pub fn lsr(sys: &mut Snes, address: usize) {
        let value = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        let result = op::lsr(cpu, value);
        write(&mut sys.mem, address, result);
    }

    pub fn lsra(sys: &mut Snes) {
        let value = reg::get_a(&sys.cpu);
        let cpu = &mut sys.cpu;
        let result = op::lsr(cpu, value);
        reg::set_a(&mut sys.cpu, result);
    }

    pub fn rol(sys: &mut Snes, address: usize) {
        let value = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        let result = op::rol(cpu, value);
        write(&mut sys.mem, address, result);
    }

    pub fn rola(sys: &mut Snes) {
        let value = reg::get_a(&sys.cpu);
        let cpu = &mut sys.cpu;
        let result = op::rol(cpu, value);
        reg::set_a(&mut sys.cpu, result);
    }

    pub fn ror(sys: &mut Snes, address: usize) {
        let value = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        let result = op::ror(cpu, value);
        write(&mut sys.mem, address, result);
    }

    pub fn rora(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_a(cpu);
        let result = op::ror(cpu, value);
        reg::set_a(cpu, result);
    }

    pub fn bit(sys: &mut Snes, address: usize) {
        let rhs = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        op::bit(cpu, rhs);
    }

    pub fn cmp(sys: &mut Snes, address: usize) {
        let lhs = reg::get_a(&sys.cpu);
        let rhs = read(&mut sys.mem, address);
        op::compare(&mut sys.cpu, lhs, rhs);
    }

    pub fn cpx(sys: &mut Snes, address: usize) {
        let lhs = reg::get_x(&sys.cpu);
        let rhs = read(&mut sys.mem, address);
        op::compare(&mut sys.cpu, lhs, rhs);
    }

    pub fn cpy(sys: &mut Snes, address: usize) {
        let lhs = reg::get_y(&sys.cpu);
        let rhs = read(&mut sys.mem, address);
        op::compare(&mut sys.cpu, lhs, rhs);
    }

    pub fn dea(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_a(cpu);
        let result = op::dec(cpu, value);
        reg::set_a(cpu, result);
    }

    pub fn dec(sys: &mut Snes, address: usize) {
        let value = read(&mut sys.mem, address);
        let result = op::dec(&mut sys.cpu, value);
        write(&mut sys.mem, address, result);
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
        let value = read(&mut sys.mem, address);
        let result = op::inc(&mut sys.cpu, value);
        write(&mut sys.mem, address, result);
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
        let value = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        reg::set_a(cpu, value);
        op::set_zn(cpu, value);
    }

    pub fn ldx(sys: &mut Snes, address: usize) {
        let value = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        reg::set_x(cpu, value);
        op::set_zn(cpu, value);
    }

    pub fn ldy(sys: &mut Snes, address: usize) {
        let value = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        reg::set_y(cpu, value);
        op::set_zn(cpu, value);
    }

    pub fn sta(sys: &mut Snes, address: usize) {
        let value = reg::get_a(&sys.cpu);
        write(&mut sys.mem, address, value);
    }

    pub fn sty(sys: &mut Snes, address: usize) {
        let value = reg::get_y(&sys.cpu);
        write(&mut sys.mem, address, value);
    }

    pub fn stx(sys: &mut Snes, address: usize) {
        let value = reg::get_x(&sys.cpu);
        write(&mut sys.mem, address, value);
    }

    pub fn stz(sys: &mut Snes, address: usize) {
        let value = 0;
        write(&mut sys.mem, address, value);
    }
}

/// Helpers for 8 bit operations
mod helpers_8 {
    use super::*;
    use cpu::ops_8 as op;
    use cpu::registers_8 as reg;
    use memory::io::byte::*;

    pub fn adc(sys: &mut Snes, address: usize) {
        let rhs = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        op::adc(cpu, rhs);
    }

    pub fn sbc(sys: &mut Snes, address: usize) {
        let rhs = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        op::sbc(cpu, rhs);
    }

    pub fn and(sys: &mut Snes, address: usize) {
        let rhs = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        op::and(cpu, rhs);
    }

    pub fn eor(sys: &mut Snes, address: usize) {
        let rhs = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        op::eor(cpu, rhs);
    }

    pub fn ora(sys: &mut Snes, address: usize) {
        let rhs = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        op::ora(cpu, rhs);
    }

    pub fn tsb(sys: &mut Snes, address: usize) {
        let rhs = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        let result = op::tsb(cpu, rhs);
        write(&mut sys.mem, address, result);
    }

    pub fn trb(sys: &mut Snes, address: usize) {
        let rhs = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        let result = op::trb(cpu, rhs);
        write(&mut sys.mem, address, result);
    }

    pub fn asl(sys: &mut Snes, address: usize) {
        let value = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        let result = op::asl(cpu, value);
        write(&mut sys.mem, address, result);
    }

    pub fn asla(sys: &mut Snes) {
        let value = reg::get_a(&sys.cpu);
        let cpu = &mut sys.cpu;
        let result = op::asl(cpu, value);
        reg::set_a(&mut sys.cpu, result);
    }

    pub fn lsr(sys: &mut Snes, address: usize) {
        let value = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        let result = op::lsr(cpu, value);
        write(&mut sys.mem, address, result);
    }

    pub fn lsra(sys: &mut Snes) {
        let value = reg::get_a(&sys.cpu);
        let cpu = &mut sys.cpu;
        let result = op::lsr(cpu, value);
        reg::set_a(&mut sys.cpu, result);
    }

    pub fn rol(sys: &mut Snes, address: usize) {
        let value = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        let result = op::rol(cpu, value);
        write(&mut sys.mem, address, result);
    }

    pub fn rola(sys: &mut Snes) {
        let value = reg::get_a(&sys.cpu);
        let cpu = &mut sys.cpu;
        let result = op::rol(cpu, value);
        reg::set_a(&mut sys.cpu, result);
    }

    pub fn ror(sys: &mut Snes, address: usize) {
        let value = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        let result = op::ror(cpu, value);
        write(&mut sys.mem, address, result);
    }

    pub fn rora(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_a(cpu);
        let result = op::ror(cpu, value);
        reg::set_a(cpu, result);
    }

    pub fn bit(sys: &mut Snes, address: usize) {
        let rhs = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        op::bit(cpu, rhs);
    }

    pub fn cmp(sys: &mut Snes, address: usize) {
        let lhs = reg::get_a(&sys.cpu);
        let rhs = read(&mut sys.mem, address);
        op::compare(&mut sys.cpu, lhs, rhs);
    }

    pub fn cpx(sys: &mut Snes, address: usize) {
        let lhs = reg::get_x(&sys.cpu);
        let rhs = read(&mut sys.mem, address);
        op::compare(&mut sys.cpu, lhs, rhs);
    }

    pub fn cpy(sys: &mut Snes, address: usize) {
        let lhs = reg::get_y(&sys.cpu);
        let rhs = read(&mut sys.mem, address);
        op::compare(&mut sys.cpu, lhs, rhs);
    }

    pub fn dea(sys: &mut Snes) {
        let cpu = &mut sys.cpu;
        let value = reg::get_a(cpu);
        let result = op::dec(cpu, value);
        reg::set_a(cpu, result);
    }

    pub fn dec(sys: &mut Snes, address: usize) {
        let value = read(&mut sys.mem, address);
        let result = op::dec(&mut sys.cpu, value);
        write(&mut sys.mem, address, result);
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
        let value = read(&mut sys.mem, address);
        let result = op::inc(&mut sys.cpu, value);
        write(&mut sys.mem, address, result);
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
        let value = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        reg::set_a(cpu, value);
        op::set_zn(cpu, value);
    }

    pub fn ldx(sys: &mut Snes, address: usize) {
        let value = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        reg::set_x(cpu, value);
        op::set_zn(cpu, value);
    }

    pub fn ldy(sys: &mut Snes, address: usize) {
        let value = read(&mut sys.mem, address);
        let cpu = &mut sys.cpu;
        reg::set_y(cpu, value);
        op::set_zn(cpu, value);
    }

    pub fn sta(sys: &mut Snes, address: usize) {
        let value = reg::get_a(&sys.cpu);
        write(&mut sys.mem, address, value);
    }

    pub fn sty(sys: &mut Snes, address: usize) {
        let value = reg::get_y(&sys.cpu);
        write(&mut sys.mem, address, value);
    }

    pub fn stx(sys: &mut Snes, address: usize) {
        let value = reg::get_x(&sys.cpu);
        write(&mut sys.mem, address, value);
    }

    pub fn stz(sys: &mut Snes, address: usize) {
        let value = 0;
        write(&mut sys.mem, address, value);
    }
}

// Implement these manually


// These are generated functions, do not change them manually

pub fn adc_61(sys: &mut Snes) {
    debug_println!("ADC");
    let address = 0; //todo: implement DP Indexed Indirect,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::adc(sys, address);
    } else {
        helpers_16::adc(sys, address);
    }
 }
 
 pub fn adc_63(sys: &mut Snes) {
    debug_println!("ADC");
    let address = address_mode::stack_relative(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::adc(sys, address);
    } else {
        helpers_16::adc(sys, address);
    }
 }
 
 pub fn adc_65(sys: &mut Snes) {
    debug_println!("ADC &<todo>");
    let address = address_mode::direct(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::adc(sys, address);
    } else {
        helpers_16::adc(sys, address);
    }
 }
 
 pub fn adc_67(sys: &mut Snes) {
    debug_println!("ADC");
    let address = 0; //todo: implement DP Indirect Long;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::adc(sys, address);
    } else {
        helpers_16::adc(sys, address);
    }
 }
 
 pub fn adc_69(sys: &mut Snes) {
    debug_println!("ADC #&<todo>");
    let address = address_mode::immediate16(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::adc(sys, address);
    } else {
        helpers_16::adc(sys, address);
    }
 }
 
 pub fn adc_6d(sys: &mut Snes) {
    debug_println!("ADC &<todo>");
    let address = address_mode::absolute(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::adc(sys, address);
    } else {
        helpers_16::adc(sys, address);
    }
 }
 
 pub fn adc_6f(sys: &mut Snes) {
    debug_println!("ADC");
    let address = 0; //todo: implement Absolute Long;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::adc(sys, address);
    } else {
        helpers_16::adc(sys, address);
    }
 }
 
 pub fn adc_71(sys: &mut Snes) {
    debug_println!("ADC");
    let address = 0; //todo: implement DP Indirect Indexed, Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::adc(sys, address);
    } else {
        helpers_16::adc(sys, address);
    }
 }
 
 pub fn adc_72(sys: &mut Snes) {
    debug_println!("ADC");
    let address = 0; //todo: implement DP Indirect;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::adc(sys, address);
    } else {
        helpers_16::adc(sys, address);
    }
 }
 
 pub fn adc_73(sys: &mut Snes) {
    debug_println!("ADC");
    let address = 0; //todo: implement SR Indirect Indexed,Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::adc(sys, address);
    } else {
        helpers_16::adc(sys, address);
    }
 }
 
 pub fn adc_75(sys: &mut Snes) {
    debug_println!("ADC");
    let address = 0; //todo: implement DP Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::adc(sys, address);
    } else {
        helpers_16::adc(sys, address);
    }
 }
 
 pub fn adc_77(sys: &mut Snes) {
    debug_println!("ADC");
    let address = 0; //todo: implement DP Indirect Long Indexed, Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::adc(sys, address);
    } else {
        helpers_16::adc(sys, address);
    }
 }
 
 pub fn adc_79(sys: &mut Snes) {
    debug_println!("ADC");
    let address = 0; //todo: implement Absolute Indexed,Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::adc(sys, address);
    } else {
        helpers_16::adc(sys, address);
    }
 }
 
 pub fn adc_7d(sys: &mut Snes) {
    debug_println!("ADC");
    let address = 0; //todo: implement Absolute Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::adc(sys, address);
    } else {
        helpers_16::adc(sys, address);
    }
 }
 
 pub fn adc_7f(sys: &mut Snes) {
    debug_println!("ADC");
    let address = 0; //todo: implement Absolute Long Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::adc(sys, address);
    } else {
        helpers_16::adc(sys, address);
    }
 }
 
 pub fn and_21(sys: &mut Snes) {
    debug_println!("AND");
    let address = 0; //todo: implement DP Indexed Indirect,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::and(sys, address);
    } else {
        helpers_16::and(sys, address);
    }
 }
 
 pub fn and_23(sys: &mut Snes) {
    debug_println!("AND");
    let address = address_mode::stack_relative(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::and(sys, address);
    } else {
        helpers_16::and(sys, address);
    }
 }
 
 pub fn and_25(sys: &mut Snes) {
    debug_println!("AND &<todo>");
    let address = address_mode::direct(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::and(sys, address);
    } else {
        helpers_16::and(sys, address);
    }
 }
 
 pub fn and_27(sys: &mut Snes) {
    debug_println!("AND");
    let address = 0; //todo: implement DP Indirect Long;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::and(sys, address);
    } else {
        helpers_16::and(sys, address);
    }
 }
 
 pub fn and_29(sys: &mut Snes) {
    debug_println!("AND #&<todo>");
    let address = address_mode::immediate16(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::and(sys, address);
    } else {
        helpers_16::and(sys, address);
    }
 }
 
 pub fn and_2d(sys: &mut Snes) {
    debug_println!("AND &<todo>");
    let address = address_mode::absolute(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::and(sys, address);
    } else {
        helpers_16::and(sys, address);
    }
 }
 
 pub fn and_2f(sys: &mut Snes) {
    debug_println!("AND");
    let address = 0; //todo: implement Absolute Long;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::and(sys, address);
    } else {
        helpers_16::and(sys, address);
    }
 }
 
 pub fn and_31(sys: &mut Snes) {
    debug_println!("AND");
    let address = 0; //todo: implement DP Indirect Indexed, Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::and(sys, address);
    } else {
        helpers_16::and(sys, address);
    }
 }
 
 pub fn and_32(sys: &mut Snes) {
    debug_println!("AND");
    let address = 0; //todo: implement DP Indirect;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::and(sys, address);
    } else {
        helpers_16::and(sys, address);
    }
 }
 
 pub fn and_33(sys: &mut Snes) {
    debug_println!("AND");
    let address = 0; //todo: implement SR Indirect Indexed,Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::and(sys, address);
    } else {
        helpers_16::and(sys, address);
    }
 }
 
 pub fn and_35(sys: &mut Snes) {
    debug_println!("AND");
    let address = 0; //todo: implement DP Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::and(sys, address);
    } else {
        helpers_16::and(sys, address);
    }
 }
 
 pub fn and_37(sys: &mut Snes) {
    debug_println!("AND");
    let address = 0; //todo: implement DP Indirect Long Indexed, Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::and(sys, address);
    } else {
        helpers_16::and(sys, address);
    }
 }
 
 pub fn and_39(sys: &mut Snes) {
    debug_println!("AND");
    let address = 0; //todo: implement Absolute Indexed,Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::and(sys, address);
    } else {
        helpers_16::and(sys, address);
    }
 }
 
 pub fn and_3d(sys: &mut Snes) {
    debug_println!("AND");
    let address = 0; //todo: implement Absolute Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::and(sys, address);
    } else {
        helpers_16::and(sys, address);
    }
 }
 
 pub fn and_3f(sys: &mut Snes) {
    debug_println!("AND");
    let address = 0; //todo: implement Absolute Long Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::and(sys, address);
    } else {
        helpers_16::and(sys, address);
    }
 }
 
 pub fn asl_06(sys: &mut Snes) {
    debug_println!("ASL &<todo>");
   // todo!();
 }
 
 pub fn asl_0a(sys: &mut Snes) {
    debug_println!("ASL");
   // todo!();
 }
 
 pub fn asl_0e(sys: &mut Snes) {
    debug_println!("ASL &<todo>");
   // todo!();
 }
 
 pub fn asl_16(sys: &mut Snes) {
    debug_println!("ASL");
   // todo!();
 }
 
 pub fn asl_1e(sys: &mut Snes) {
    debug_println!("ASL");
   // todo!();
 }
 
 pub fn bcc_90(sys: &mut Snes) {
    debug_println!("BCC");
   // todo!();
 }
 
 pub fn bcs_b0(sys: &mut Snes) {
    debug_println!("BCS");
   // todo!();
 }
 
 pub fn beq_f0(sys: &mut Snes) {
    debug_println!("BEQ");
   // todo!();
 }
 
 pub fn bit_24(sys: &mut Snes) {
    debug_println!("BIT &<todo>");
    let address = address_mode::direct(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::bit(sys, address);
    } else {
        helpers_16::bit(sys, address);
    }
 }
 
 pub fn bit_2c(sys: &mut Snes) {
    debug_println!("BIT &<todo>");
    let address = address_mode::absolute(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::bit(sys, address);
    } else {
        helpers_16::bit(sys, address);
    }
 }
 
 pub fn bit_34(sys: &mut Snes) {
    debug_println!("BIT");
    let address = 0; //todo: implement DP Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::bit(sys, address);
    } else {
        helpers_16::bit(sys, address);
    }
 }
 
 pub fn bit_3c(sys: &mut Snes) {
    debug_println!("BIT");
    let address = 0; //todo: implement Absolute Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::bit(sys, address);
    } else {
        helpers_16::bit(sys, address);
    }
 }
 
 pub fn bit_89(sys: &mut Snes) {
    debug_println!("BIT #&<todo>");
    let address = address_mode::immediate16(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::bit(sys, address);
    } else {
        helpers_16::bit(sys, address);
    }
 }
 
 pub fn bmi_30(sys: &mut Snes) {
    debug_println!("BMI");
   // todo!();
 }
 
 pub fn bne_d0(sys: &mut Snes) {
    debug_println!("BNE");
   // todo!();
 }
 
 pub fn bpl_10(sys: &mut Snes) {
    debug_println!("BPL");
   // todo!();
 }
 
 pub fn bra_80(sys: &mut Snes) {
    debug_println!("BRA");
   // todo!();
 }
 
 pub fn brk_00(sys: &mut Snes) {
    debug_println!("BRK");
   // todo!();
 }
 
 pub fn brl_82(sys: &mut Snes) {
    debug_println!("BRL");
   // todo!();
 }
 
 pub fn bvc_50(sys: &mut Snes) {
    debug_println!("BVC");
   // todo!();
 }
 
 pub fn bvs_70(sys: &mut Snes) {
    debug_println!("BVS");
   // todo!();
 }
 
 pub fn clc_18(sys: &mut Snes) {
    debug_println!("CLC");
   // todo!();
 }
 
 pub fn cld_d8(sys: &mut Snes) {
    debug_println!("CLD");
   // todo!();
 }
 
 pub fn cli_58(sys: &mut Snes) {
    debug_println!("CLI");
   // todo!();
 }
 
 pub fn clv_b8(sys: &mut Snes) {
    debug_println!("CLV");
   // todo!();
 }
 
 pub fn cmp_c1(sys: &mut Snes) {
    debug_println!("CMP");
    let address = 0; //todo: implement DP Indexed Indirect,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::cmp(sys, address);
    } else {
        helpers_16::cmp(sys, address);
    }
 }
 
 pub fn cmp_c3(sys: &mut Snes) {
    debug_println!("CMP");
    let address = address_mode::stack_relative(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::cmp(sys, address);
    } else {
        helpers_16::cmp(sys, address);
    }
 }
 
 pub fn cmp_c5(sys: &mut Snes) {
    debug_println!("CMP &<todo>");
    let address = address_mode::direct(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::cmp(sys, address);
    } else {
        helpers_16::cmp(sys, address);
    }
 }
 
 pub fn cmp_c7(sys: &mut Snes) {
    debug_println!("CMP");
    let address = 0; //todo: implement DP Indirect Long;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::cmp(sys, address);
    } else {
        helpers_16::cmp(sys, address);
    }
 }
 
 pub fn cmp_c9(sys: &mut Snes) {
    debug_println!("CMP #&<todo>");
    let address = address_mode::immediate16(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::cmp(sys, address);
    } else {
        helpers_16::cmp(sys, address);
    }
 }
 
 pub fn cmp_cd(sys: &mut Snes) {
    debug_println!("CMP &<todo>");
    let address = address_mode::absolute(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::cmp(sys, address);
    } else {
        helpers_16::cmp(sys, address);
    }
 }
 
 pub fn cmp_cf(sys: &mut Snes) {
    debug_println!("CMP");
    let address = 0; //todo: implement Absolute Long;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::cmp(sys, address);
    } else {
        helpers_16::cmp(sys, address);
    }
 }
 
 pub fn cmp_d1(sys: &mut Snes) {
    debug_println!("CMP");
    let address = 0; //todo: implement DP Indirect Indexed, Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::cmp(sys, address);
    } else {
        helpers_16::cmp(sys, address);
    }
 }
 
 pub fn cmp_d2(sys: &mut Snes) {
    debug_println!("CMP");
    let address = 0; //todo: implement DP Indirect;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::cmp(sys, address);
    } else {
        helpers_16::cmp(sys, address);
    }
 }
 
 pub fn cmp_d3(sys: &mut Snes) {
    debug_println!("CMP");
    let address = 0; //todo: implement SR Indirect Indexed,Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::cmp(sys, address);
    } else {
        helpers_16::cmp(sys, address);
    }
 }
 
 pub fn cmp_d5(sys: &mut Snes) {
    debug_println!("CMP");
    let address = 0; //todo: implement DP Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::cmp(sys, address);
    } else {
        helpers_16::cmp(sys, address);
    }
 }
 
 pub fn cmp_d7(sys: &mut Snes) {
    debug_println!("CMP");
    let address = 0; //todo: implement DP Indirect Long Indexed, Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::cmp(sys, address);
    } else {
        helpers_16::cmp(sys, address);
    }
 }
 
 pub fn cmp_d9(sys: &mut Snes) {
    debug_println!("CMP");
    let address = 0; //todo: implement Absolute Indexed,Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::cmp(sys, address);
    } else {
        helpers_16::cmp(sys, address);
    }
 }
 
 pub fn cmp_dd(sys: &mut Snes) {
    debug_println!("CMP");
    let address = 0; //todo: implement Absolute Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::cmp(sys, address);
    } else {
        helpers_16::cmp(sys, address);
    }
 }
 
 pub fn cmp_df(sys: &mut Snes) {
    debug_println!("CMP");
    let address = 0; //todo: implement Absolute Long Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::cmp(sys, address);
    } else {
        helpers_16::cmp(sys, address);
    }
 }
 
 pub fn cop_02(sys: &mut Snes) {
    debug_println!("COP");
   // todo!();
 }
 
 pub fn cpx_e0(sys: &mut Snes) {
    debug_println!("CPX #&<todo>");
    let address = address_mode::immediate16(sys);
    if cpu::status_flags::is_set_x(&sys.cpu) {
        helpers_8::cpx(sys, address);
    } else {
        helpers_16::cpx(sys, address);
    }
 }
 
 pub fn cpx_e4(sys: &mut Snes) {
    debug_println!("CPX &<todo>");
    let address = address_mode::direct(sys);
    if cpu::status_flags::is_set_x(&sys.cpu) {
        helpers_8::cpx(sys, address);
    } else {
        helpers_16::cpx(sys, address);
    }
 }
 
 pub fn cpx_ec(sys: &mut Snes) {
    debug_println!("CPX &<todo>");
    let address = address_mode::absolute(sys);
    if cpu::status_flags::is_set_x(&sys.cpu) {
        helpers_8::cpx(sys, address);
    } else {
        helpers_16::cpx(sys, address);
    }
 }
 
 pub fn cpy_c0(sys: &mut Snes) {
    debug_println!("CPY #&<todo>");
    let address = address_mode::immediate16(sys);
    if cpu::status_flags::is_set_x(&sys.cpu) {
        helpers_8::cpy(sys, address);
    } else {
        helpers_16::cpy(sys, address);
    }
 }
 
 pub fn cpy_c4(sys: &mut Snes) {
    debug_println!("CPY &<todo>");
    let address = address_mode::direct(sys);
    if cpu::status_flags::is_set_x(&sys.cpu) {
        helpers_8::cpy(sys, address);
    } else {
        helpers_16::cpy(sys, address);
    }
 }
 
 pub fn cpy_cc(sys: &mut Snes) {
    debug_println!("CPY &<todo>");
    let address = address_mode::absolute(sys);
    if cpu::status_flags::is_set_x(&sys.cpu) {
        helpers_8::cpy(sys, address);
    } else {
        helpers_16::cpy(sys, address);
    }
 }
 
 pub fn dec_3a(sys: &mut Snes) {
    debug_println!("DEC");
    let address = 0; //todo: implement Accumulator;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::dec(sys, address);
    } else {
        helpers_16::dec(sys, address);
    }
 }
 
 pub fn dec_c6(sys: &mut Snes) {
    debug_println!("DEC &<todo>");
    let address = address_mode::direct(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::dec(sys, address);
    } else {
        helpers_16::dec(sys, address);
    }
 }
 
 pub fn dec_ce(sys: &mut Snes) {
    debug_println!("DEC &<todo>");
    let address = address_mode::absolute(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::dec(sys, address);
    } else {
        helpers_16::dec(sys, address);
    }
 }
 
 pub fn dec_d6(sys: &mut Snes) {
    debug_println!("DEC");
    let address = 0; //todo: implement DP Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::dec(sys, address);
    } else {
        helpers_16::dec(sys, address);
    }
 }
 
 pub fn dec_de(sys: &mut Snes) {
    debug_println!("DEC");
    let address = 0; //todo: implement Absolute Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::dec(sys, address);
    } else {
        helpers_16::dec(sys, address);
    }
 }
 
 pub fn dex_ca(sys: &mut Snes) {
    debug_println!("DEX");
   // todo!();
 }
 
 pub fn dey_88(sys: &mut Snes) {
    debug_println!("DEY");
   // todo!();
 }
 
 pub fn eor_41(sys: &mut Snes) {
    debug_println!("EOR");
    let address = 0; //todo: implement DP Indexed Indirect,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::eor(sys, address);
    } else {
        helpers_16::eor(sys, address);
    }
 }
 
 pub fn eor_43(sys: &mut Snes) {
    debug_println!("EOR");
    let address = address_mode::stack_relative(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::eor(sys, address);
    } else {
        helpers_16::eor(sys, address);
    }
 }
 
 pub fn eor_45(sys: &mut Snes) {
    debug_println!("EOR &<todo>");
    let address = address_mode::direct(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::eor(sys, address);
    } else {
        helpers_16::eor(sys, address);
    }
 }
 
 pub fn eor_47(sys: &mut Snes) {
    debug_println!("EOR");
    let address = 0; //todo: implement DP Indirect Long;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::eor(sys, address);
    } else {
        helpers_16::eor(sys, address);
    }
 }
 
 pub fn eor_49(sys: &mut Snes) {
    debug_println!("EOR #&<todo>");
    let address = address_mode::immediate16(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::eor(sys, address);
    } else {
        helpers_16::eor(sys, address);
    }
 }
 
 pub fn eor_4d(sys: &mut Snes) {
    debug_println!("EOR &<todo>");
    let address = address_mode::absolute(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::eor(sys, address);
    } else {
        helpers_16::eor(sys, address);
    }
 }
 
 pub fn eor_4f(sys: &mut Snes) {
    debug_println!("EOR");
    let address = 0; //todo: implement Absolute Long;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::eor(sys, address);
    } else {
        helpers_16::eor(sys, address);
    }
 }
 
 pub fn eor_51(sys: &mut Snes) {
    debug_println!("EOR");
    let address = 0; //todo: implement DP Indirect Indexed, Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::eor(sys, address);
    } else {
        helpers_16::eor(sys, address);
    }
 }
 
 pub fn eor_52(sys: &mut Snes) {
    debug_println!("EOR");
    let address = 0; //todo: implement DP Indirect;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::eor(sys, address);
    } else {
        helpers_16::eor(sys, address);
    }
 }
 
 pub fn eor_53(sys: &mut Snes) {
    debug_println!("EOR");
    let address = 0; //todo: implement SR Indirect Indexed,Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::eor(sys, address);
    } else {
        helpers_16::eor(sys, address);
    }
 }
 
 pub fn eor_55(sys: &mut Snes) {
    debug_println!("EOR");
    let address = 0; //todo: implement DP Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::eor(sys, address);
    } else {
        helpers_16::eor(sys, address);
    }
 }
 
 pub fn eor_57(sys: &mut Snes) {
    debug_println!("EOR");
    let address = 0; //todo: implement DP Indirect Long Indexed, Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::eor(sys, address);
    } else {
        helpers_16::eor(sys, address);
    }
 }
 
 pub fn eor_59(sys: &mut Snes) {
    debug_println!("EOR");
    let address = 0; //todo: implement Absolute Indexed,Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::eor(sys, address);
    } else {
        helpers_16::eor(sys, address);
    }
 }
 
 pub fn eor_5d(sys: &mut Snes) {
    debug_println!("EOR");
    let address = 0; //todo: implement Absolute Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::eor(sys, address);
    } else {
        helpers_16::eor(sys, address);
    }
 }
 
 pub fn eor_5f(sys: &mut Snes) {
    debug_println!("EOR");
    let address = 0; //todo: implement Absolute Long Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::eor(sys, address);
    } else {
        helpers_16::eor(sys, address);
    }
 }
 
 pub fn inc_1a(sys: &mut Snes) {
    debug_println!("INC");
    let address = 0; //todo: implement Accumulator;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::inc(sys, address);
    } else {
        helpers_16::inc(sys, address);
    }
 }
 
 pub fn inc_e6(sys: &mut Snes) {
    debug_println!("INC &<todo>");
    let address = address_mode::direct(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::inc(sys, address);
    } else {
        helpers_16::inc(sys, address);
    }
 }
 
 pub fn inc_ee(sys: &mut Snes) {
    debug_println!("INC &<todo>");
    let address = address_mode::absolute(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::inc(sys, address);
    } else {
        helpers_16::inc(sys, address);
    }
 }
 
 pub fn inc_f6(sys: &mut Snes) {
    debug_println!("INC");
    let address = 0; //todo: implement DP Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::inc(sys, address);
    } else {
        helpers_16::inc(sys, address);
    }
 }
 
 pub fn inc_fe(sys: &mut Snes) {
    debug_println!("INC");
    let address = 0; //todo: implement Absolute Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::inc(sys, address);
    } else {
        helpers_16::inc(sys, address);
    }
 }
 
 pub fn inx_e8(sys: &mut Snes) {
    debug_println!("INX");
   // todo!();
 }
 
 pub fn iny_c8(sys: &mut Snes) {
    debug_println!("INY");
   // todo!();
 }
 
 pub fn jmp_4c(sys: &mut Snes) {
    debug_println!("JMP &<todo>");
   // todo!();
 }
 
 pub fn jmp_5c(sys: &mut Snes) {
    debug_println!("JMP");
   // todo!();
 }
 
 pub fn jmp_6c(sys: &mut Snes) {
    debug_println!("JMP");
   // todo!();
 }
 
 pub fn jmp_7c(sys: &mut Snes) {
    debug_println!("JMP");
   // todo!();
 }
 
 pub fn jmp_dc(sys: &mut Snes) {
    debug_println!("JMP");
   // todo!();
 }
 
 pub fn jsr_20(sys: &mut Snes) {
    debug_println!("JSR &<todo>");
   // todo!();
 }
 
 pub fn jsr_22(sys: &mut Snes) {
    debug_println!("JSR");
   // todo!();
 }
 
 pub fn jsr_fc(sys: &mut Snes) {
    debug_println!("JSR");
   // todo!();
 }
 
 pub fn lda_a1(sys: &mut Snes) {
    debug_println!("LDA");
    let address = 0; //todo: implement DP Indexed Indirect,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::lda(sys, address);
    } else {
        helpers_16::lda(sys, address);
    }
 }
 
 pub fn lda_a3(sys: &mut Snes) {
    debug_println!("LDA");
    let address = address_mode::stack_relative(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::lda(sys, address);
    } else {
        helpers_16::lda(sys, address);
    }
 }
 
 pub fn lda_a5(sys: &mut Snes) {
    debug_println!("LDA &<todo>");
    let address = address_mode::direct(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::lda(sys, address);
    } else {
        helpers_16::lda(sys, address);
    }
 }
 
 pub fn lda_a7(sys: &mut Snes) {
    debug_println!("LDA");
    let address = 0; //todo: implement DP Indirect Long;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::lda(sys, address);
    } else {
        helpers_16::lda(sys, address);
    }
 }
 
 pub fn lda_a9(sys: &mut Snes) {
    debug_println!("LDA #&<todo>");
    let address = address_mode::immediate16(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::lda(sys, address);
    } else {
        helpers_16::lda(sys, address);
    }
 }
 
 pub fn lda_ad(sys: &mut Snes) {
    debug_println!("LDA &<todo>");
    let address = address_mode::absolute(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::lda(sys, address);
    } else {
        helpers_16::lda(sys, address);
    }
 }
 
 pub fn lda_af(sys: &mut Snes) {
    debug_println!("LDA");
    let address = 0; //todo: implement Absolute Long;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::lda(sys, address);
    } else {
        helpers_16::lda(sys, address);
    }
 }
 
 pub fn lda_b1(sys: &mut Snes) {
    debug_println!("LDA");
    let address = 0; //todo: implement DP Indirect Indexed, Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::lda(sys, address);
    } else {
        helpers_16::lda(sys, address);
    }
 }
 
 pub fn lda_b2(sys: &mut Snes) {
    debug_println!("LDA");
    let address = 0; //todo: implement DP Indirect;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::lda(sys, address);
    } else {
        helpers_16::lda(sys, address);
    }
 }
 
 pub fn lda_b3(sys: &mut Snes) {
    debug_println!("LDA");
    let address = 0; //todo: implement SR Indirect Indexed,Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::lda(sys, address);
    } else {
        helpers_16::lda(sys, address);
    }
 }
 
 pub fn lda_b5(sys: &mut Snes) {
    debug_println!("LDA");
    let address = 0; //todo: implement DP Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::lda(sys, address);
    } else {
        helpers_16::lda(sys, address);
    }
 }
 
 pub fn lda_b7(sys: &mut Snes) {
    debug_println!("LDA");
    let address = 0; //todo: implement DP Indirect Long Indexed, Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::lda(sys, address);
    } else {
        helpers_16::lda(sys, address);
    }
 }
 
 pub fn lda_b9(sys: &mut Snes) {
    debug_println!("LDA");
    let address = 0; //todo: implement Absolute Indexed,Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::lda(sys, address);
    } else {
        helpers_16::lda(sys, address);
    }
 }
 
 pub fn lda_bd(sys: &mut Snes) {
    debug_println!("LDA");
    let address = 0; //todo: implement Absolute Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::lda(sys, address);
    } else {
        helpers_16::lda(sys, address);
    }
 }
 
 pub fn lda_bf(sys: &mut Snes) {
    debug_println!("LDA");
    let address = 0; //todo: implement Absolute Long Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::lda(sys, address);
    } else {
        helpers_16::lda(sys, address);
    }
 }
 
 pub fn ldx_a2(sys: &mut Snes) {
    debug_println!("LDX #&<todo>");
    let address = address_mode::immediate16(sys);
    if cpu::status_flags::is_set_x(&sys.cpu) {
        helpers_8::ldx(sys, address);
    } else {
        helpers_16::ldx(sys, address);
    }
 }
 
 pub fn ldx_a6(sys: &mut Snes) {
    debug_println!("LDX &<todo>");
    let address = address_mode::direct(sys);
    if cpu::status_flags::is_set_x(&sys.cpu) {
        helpers_8::ldx(sys, address);
    } else {
        helpers_16::ldx(sys, address);
    }
 }
 
 pub fn ldx_ae(sys: &mut Snes) {
    debug_println!("LDX &<todo>");
    let address = address_mode::absolute(sys);
    if cpu::status_flags::is_set_x(&sys.cpu) {
        helpers_8::ldx(sys, address);
    } else {
        helpers_16::ldx(sys, address);
    }
 }
 
 pub fn ldx_b6(sys: &mut Snes) {
    debug_println!("LDX");
    let address = 0; //todo: implement DP Indexed,Y;
    if cpu::status_flags::is_set_x(&sys.cpu) {
        helpers_8::ldx(sys, address);
    } else {
        helpers_16::ldx(sys, address);
    }
 }
 
 pub fn ldx_be(sys: &mut Snes) {
    debug_println!("LDX");
    let address = 0; //todo: implement Absolute Indexed,Y;
    if cpu::status_flags::is_set_x(&sys.cpu) {
        helpers_8::ldx(sys, address);
    } else {
        helpers_16::ldx(sys, address);
    }
 }
 
 pub fn ldy_a0(sys: &mut Snes) {
    debug_println!("LDY #&<todo>");
    let address = address_mode::immediate16(sys);
    if cpu::status_flags::is_set_x(&sys.cpu) {
        helpers_8::ldy(sys, address);
    } else {
        helpers_16::ldy(sys, address);
    }
 }
 
 pub fn ldy_a4(sys: &mut Snes) {
    debug_println!("LDY &<todo>");
    let address = address_mode::direct(sys);
    if cpu::status_flags::is_set_x(&sys.cpu) {
        helpers_8::ldy(sys, address);
    } else {
        helpers_16::ldy(sys, address);
    }
 }
 
 pub fn ldy_ac(sys: &mut Snes) {
    debug_println!("LDY &<todo>");
    let address = address_mode::absolute(sys);
    if cpu::status_flags::is_set_x(&sys.cpu) {
        helpers_8::ldy(sys, address);
    } else {
        helpers_16::ldy(sys, address);
    }
 }
 
 pub fn ldy_b4(sys: &mut Snes) {
    debug_println!("LDY");
    let address = 0; //todo: implement DP Indexed,X;
    if cpu::status_flags::is_set_x(&sys.cpu) {
        helpers_8::ldy(sys, address);
    } else {
        helpers_16::ldy(sys, address);
    }
 }
 
 pub fn ldy_bc(sys: &mut Snes) {
    debug_println!("LDY");
    let address = 0; //todo: implement Absolute Indexed,X;
    if cpu::status_flags::is_set_x(&sys.cpu) {
        helpers_8::ldy(sys, address);
    } else {
        helpers_16::ldy(sys, address);
    }
 }
 
 pub fn lsr_46(sys: &mut Snes) {
    debug_println!("LSR &<todo>");
   // todo!();
 }
 
 pub fn lsr_4a(sys: &mut Snes) {
    debug_println!("LSR");
   // todo!();
 }
 
 pub fn lsr_4e(sys: &mut Snes) {
    debug_println!("LSR &<todo>");
   // todo!();
 }
 
 pub fn lsr_56(sys: &mut Snes) {
    debug_println!("LSR");
   // todo!();
 }
 
 pub fn lsr_5e(sys: &mut Snes) {
    debug_println!("LSR");
   // todo!();
 }
 
 pub fn mvn_54(sys: &mut Snes) {
    debug_println!("MVN");
   // todo!();
 }
 
 pub fn mvp_44(sys: &mut Snes) {
    debug_println!("MVP");
   // todo!();
 }
 
 pub fn nop_ea(sys: &mut Snes) {
    debug_println!("NOP");
   // todo!();
 }
 
 pub fn ora_01(sys: &mut Snes) {
    debug_println!("ORA");
    let address = 0; //todo: implement DP Indexed Indirect,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::ora(sys, address);
    } else {
        helpers_16::ora(sys, address);
    }
 }
 
 pub fn ora_03(sys: &mut Snes) {
    debug_println!("ORA");
    let address = address_mode::stack_relative(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::ora(sys, address);
    } else {
        helpers_16::ora(sys, address);
    }
 }
 
 pub fn ora_05(sys: &mut Snes) {
    debug_println!("ORA &<todo>");
    let address = address_mode::direct(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::ora(sys, address);
    } else {
        helpers_16::ora(sys, address);
    }
 }
 
 pub fn ora_07(sys: &mut Snes) {
    debug_println!("ORA");
    let address = 0; //todo: implement DP Indirect Long;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::ora(sys, address);
    } else {
        helpers_16::ora(sys, address);
    }
 }
 
 pub fn ora_09(sys: &mut Snes) {
    debug_println!("ORA #&<todo>");
    let address = address_mode::immediate16(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::ora(sys, address);
    } else {
        helpers_16::ora(sys, address);
    }
 }
 
 pub fn ora_0d(sys: &mut Snes) {
    debug_println!("ORA &<todo>");
    let address = address_mode::absolute(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::ora(sys, address);
    } else {
        helpers_16::ora(sys, address);
    }
 }
 
 pub fn ora_0f(sys: &mut Snes) {
    debug_println!("ORA");
    let address = 0; //todo: implement Absolute Long;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::ora(sys, address);
    } else {
        helpers_16::ora(sys, address);
    }
 }
 
 pub fn ora_11(sys: &mut Snes) {
    debug_println!("ORA");
    let address = 0; //todo: implement DP Indirect Indexed, Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::ora(sys, address);
    } else {
        helpers_16::ora(sys, address);
    }
 }
 
 pub fn ora_12(sys: &mut Snes) {
    debug_println!("ORA");
    let address = 0; //todo: implement DP Indirect;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::ora(sys, address);
    } else {
        helpers_16::ora(sys, address);
    }
 }
 
 pub fn ora_13(sys: &mut Snes) {
    debug_println!("ORA");
    let address = 0; //todo: implement SR Indirect Indexed,Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::ora(sys, address);
    } else {
        helpers_16::ora(sys, address);
    }
 }
 
 pub fn ora_15(sys: &mut Snes) {
    debug_println!("ORA");
    let address = 0; //todo: implement DP Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::ora(sys, address);
    } else {
        helpers_16::ora(sys, address);
    }
 }
 
 pub fn ora_17(sys: &mut Snes) {
    debug_println!("ORA");
    let address = 0; //todo: implement DP Indirect Long Indexed, Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::ora(sys, address);
    } else {
        helpers_16::ora(sys, address);
    }
 }
 
 pub fn ora_19(sys: &mut Snes) {
    debug_println!("ORA");
    let address = 0; //todo: implement Absolute Indexed,Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::ora(sys, address);
    } else {
        helpers_16::ora(sys, address);
    }
 }
 
 pub fn ora_1d(sys: &mut Snes) {
    debug_println!("ORA");
    let address = 0; //todo: implement Absolute Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::ora(sys, address);
    } else {
        helpers_16::ora(sys, address);
    }
 }
 
 pub fn ora_1f(sys: &mut Snes) {
    debug_println!("ORA");
    let address = 0; //todo: implement Absolute Long Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::ora(sys, address);
    } else {
        helpers_16::ora(sys, address);
    }
 }
 
 pub fn pea_f4(sys: &mut Snes) {
    debug_println!("PEA");
   // todo!();
 }
 
 pub fn pei_d4(sys: &mut Snes) {
    debug_println!("PEI");
   // todo!();
 }
 
 pub fn per_62(sys: &mut Snes) {
    debug_println!("PER");
   // todo!();
 }
 
 pub fn pha_48(sys: &mut Snes) {
    debug_println!("PHA");
   // todo!();
 }
 
 pub fn phb_8b(sys: &mut Snes) {
    debug_println!("PHB");
   // todo!();
 }
 
 pub fn phd_0b(sys: &mut Snes) {
    debug_println!("PHD");
   // todo!();
 }
 
 pub fn phk_4b(sys: &mut Snes) {
    debug_println!("PHK");
   // todo!();
 }
 
 pub fn php_08(sys: &mut Snes) {
    debug_println!("PHP");
   // todo!();
 }
 
 pub fn phx_da(sys: &mut Snes) {
    debug_println!("PHX");
   // todo!();
 }
 
 pub fn phy_5a(sys: &mut Snes) {
    debug_println!("PHY");
   // todo!();
 }
 
 pub fn pla_68(sys: &mut Snes) {
    debug_println!("PLA");
   // todo!();
 }
 
 pub fn plb_ab(sys: &mut Snes) {
    debug_println!("PLB");
   // todo!();
 }
 
 pub fn pld_2b(sys: &mut Snes) {
    debug_println!("PLD");
   // todo!();
 }
 
 pub fn plp_28(sys: &mut Snes) {
    debug_println!("PLP");
   // todo!();
 }
 
 pub fn plx_fa(sys: &mut Snes) {
    debug_println!("PLX");
   // todo!();
 }
 
 pub fn ply_7a(sys: &mut Snes) {
    debug_println!("PLY");
   // todo!();
 }
 
 pub fn rep_c2(sys: &mut Snes) {
    debug_println!("REP #&<todo>");
   // todo!();
 }
 
 pub fn rol_26(sys: &mut Snes) {
    debug_println!("ROL &<todo>");
   // todo!();
 }
 
 pub fn rol_2a(sys: &mut Snes) {
    debug_println!("ROL");
   // todo!();
 }
 
 pub fn rol_2e(sys: &mut Snes) {
    debug_println!("ROL &<todo>");
   // todo!();
 }
 
 pub fn rol_36(sys: &mut Snes) {
    debug_println!("ROL");
   // todo!();
 }
 
 pub fn rol_3e(sys: &mut Snes) {
    debug_println!("ROL");
   // todo!();
 }
 
 pub fn ror_66(sys: &mut Snes) {
    debug_println!("ROR &<todo>");
   // todo!();
 }
 
 pub fn ror_6a(sys: &mut Snes) {
    debug_println!("ROR");
   // todo!();
 }
 
 pub fn ror_6e(sys: &mut Snes) {
    debug_println!("ROR &<todo>");
   // todo!();
 }
 
 pub fn ror_76(sys: &mut Snes) {
    debug_println!("ROR");
   // todo!();
 }
 
 pub fn ror_7e(sys: &mut Snes) {
    debug_println!("ROR");
   // todo!();
 }
 
 pub fn rti_40(sys: &mut Snes) {
    debug_println!("RTI");
   // todo!();
 }
 
 pub fn rtl_6b(sys: &mut Snes) {
    debug_println!("RTL");
   // todo!();
 }
 
 pub fn rts_60(sys: &mut Snes) {
    debug_println!("RTS");
   // todo!();
 }
 
 pub fn sbc_e1(sys: &mut Snes) {
    debug_println!("SBC");
    let address = 0; //todo: implement DP Indexed Indirect,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sbc(sys, address);
    } else {
        helpers_16::sbc(sys, address);
    }
 }
 
 pub fn sbc_e3(sys: &mut Snes) {
    debug_println!("SBC");
    let address = address_mode::stack_relative(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sbc(sys, address);
    } else {
        helpers_16::sbc(sys, address);
    }
 }
 
 pub fn sbc_e5(sys: &mut Snes) {
    debug_println!("SBC &<todo>");
    let address = address_mode::direct(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sbc(sys, address);
    } else {
        helpers_16::sbc(sys, address);
    }
 }
 
 pub fn sbc_e7(sys: &mut Snes) {
    debug_println!("SBC");
    let address = 0; //todo: implement DP Indirect Long;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sbc(sys, address);
    } else {
        helpers_16::sbc(sys, address);
    }
 }
 
 pub fn sbc_e9(sys: &mut Snes) {
    debug_println!("SBC #&<todo>");
    let address = address_mode::immediate16(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sbc(sys, address);
    } else {
        helpers_16::sbc(sys, address);
    }
 }
 
 pub fn sbc_ed(sys: &mut Snes) {
    debug_println!("SBC &<todo>");
    let address = address_mode::absolute(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sbc(sys, address);
    } else {
        helpers_16::sbc(sys, address);
    }
 }
 
 pub fn sbc_ef(sys: &mut Snes) {
    debug_println!("SBC");
    let address = 0; //todo: implement Absolute Long;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sbc(sys, address);
    } else {
        helpers_16::sbc(sys, address);
    }
 }
 
 pub fn sbc_f1(sys: &mut Snes) {
    debug_println!("SBC");
    let address = 0; //todo: implement DP Indirect Indexed, Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sbc(sys, address);
    } else {
        helpers_16::sbc(sys, address);
    }
 }
 
 pub fn sbc_f2(sys: &mut Snes) {
    debug_println!("SBC");
    let address = 0; //todo: implement DP Indirect;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sbc(sys, address);
    } else {
        helpers_16::sbc(sys, address);
    }
 }
 
 pub fn sbc_f3(sys: &mut Snes) {
    debug_println!("SBC");
    let address = 0; //todo: implement SR Indirect Indexed,Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sbc(sys, address);
    } else {
        helpers_16::sbc(sys, address);
    }
 }
 
 pub fn sbc_f5(sys: &mut Snes) {
    debug_println!("SBC");
    let address = 0; //todo: implement DP Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sbc(sys, address);
    } else {
        helpers_16::sbc(sys, address);
    }
 }
 
 pub fn sbc_f7(sys: &mut Snes) {
    debug_println!("SBC");
    let address = 0; //todo: implement DP Indirect Long Indexed, Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sbc(sys, address);
    } else {
        helpers_16::sbc(sys, address);
    }
 }
 
 pub fn sbc_f9(sys: &mut Snes) {
    debug_println!("SBC");
    let address = 0; //todo: implement Absolute Indexed,Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sbc(sys, address);
    } else {
        helpers_16::sbc(sys, address);
    }
 }
 
 pub fn sbc_fd(sys: &mut Snes) {
    debug_println!("SBC");
    let address = 0; //todo: implement Absolute Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sbc(sys, address);
    } else {
        helpers_16::sbc(sys, address);
    }
 }
 
 pub fn sbc_ff(sys: &mut Snes) {
    debug_println!("SBC");
    let address = 0; //todo: implement Absolute Long Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sbc(sys, address);
    } else {
        helpers_16::sbc(sys, address);
    }
 }
 
 pub fn sec_38(sys: &mut Snes) {
    debug_println!("SEC");
   // todo!();
 }
 
 pub fn sed_f8(sys: &mut Snes) {
    debug_println!("SED");
   // todo!();
 }
 
 pub fn sei_78(sys: &mut Snes) {
    debug_println!("SEI");
   // todo!();
 }
 
 pub fn sep_e2(sys: &mut Snes) {
    debug_println!("SEP #&<todo>");
   // todo!();
 }
 
 pub fn sta_81(sys: &mut Snes) {
    debug_println!("STA");
    let address = 0; //todo: implement DP Indexed Indirect,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sta(sys, address);
    } else {
        helpers_16::sta(sys, address);
    }
 }
 
 pub fn sta_83(sys: &mut Snes) {
    debug_println!("STA");
    let address = address_mode::stack_relative(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sta(sys, address);
    } else {
        helpers_16::sta(sys, address);
    }
 }
 
 pub fn sta_85(sys: &mut Snes) {
    debug_println!("STA &<todo>");
    let address = address_mode::direct(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sta(sys, address);
    } else {
        helpers_16::sta(sys, address);
    }
 }
 
 pub fn sta_87(sys: &mut Snes) {
    debug_println!("STA");
    let address = 0; //todo: implement DP Indirect Long;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sta(sys, address);
    } else {
        helpers_16::sta(sys, address);
    }
 }
 
 pub fn sta_8d(sys: &mut Snes) {
    debug_println!("STA &<todo>");
    let address = address_mode::absolute(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sta(sys, address);
    } else {
        helpers_16::sta(sys, address);
    }
 }
 
 pub fn sta_8f(sys: &mut Snes) {
    debug_println!("STA");
    let address = 0; //todo: implement Absolute Long;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sta(sys, address);
    } else {
        helpers_16::sta(sys, address);
    }
 }
 
 pub fn sta_91(sys: &mut Snes) {
    debug_println!("STA");
    let address = 0; //todo: implement DP Indirect Indexed, Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sta(sys, address);
    } else {
        helpers_16::sta(sys, address);
    }
 }
 
 pub fn sta_92(sys: &mut Snes) {
    debug_println!("STA");
    let address = 0; //todo: implement DP Indirect;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sta(sys, address);
    } else {
        helpers_16::sta(sys, address);
    }
 }
 
 pub fn sta_93(sys: &mut Snes) {
    debug_println!("STA");
    let address = 0; //todo: implement SR Indirect Indexed,Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sta(sys, address);
    } else {
        helpers_16::sta(sys, address);
    }
 }
 
 pub fn sta_95(sys: &mut Snes) {
    debug_println!("STA");
    let address = 0; //todo: implement DP Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sta(sys, address);
    } else {
        helpers_16::sta(sys, address);
    }
 }
 
 pub fn sta_97(sys: &mut Snes) {
    debug_println!("STA");
    let address = 0; //todo: implement DP Indirect Long Indexed, Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sta(sys, address);
    } else {
        helpers_16::sta(sys, address);
    }
 }
 
 pub fn sta_99(sys: &mut Snes) {
    debug_println!("STA");
    let address = 0; //todo: implement Absolute Indexed,Y;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sta(sys, address);
    } else {
        helpers_16::sta(sys, address);
    }
 }
 
 pub fn sta_9d(sys: &mut Snes) {
    debug_println!("STA");
    let address = 0; //todo: implement Absolute Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sta(sys, address);
    } else {
        helpers_16::sta(sys, address);
    }
 }
 
 pub fn sta_9f(sys: &mut Snes) {
    debug_println!("STA");
    let address = 0; //todo: implement Absolute Long Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::sta(sys, address);
    } else {
        helpers_16::sta(sys, address);
    }
 }
 
 pub fn stp_db(sys: &mut Snes) {
    debug_println!("STP");
   // todo!();
 }
 
 pub fn stx_86(sys: &mut Snes) {
    debug_println!("STX &<todo>");
    let address = address_mode::direct(sys);
    if cpu::status_flags::is_set_x(&sys.cpu) {
        helpers_8::stx(sys, address);
    } else {
        helpers_16::stx(sys, address);
    }
 }
 
 pub fn stx_8e(sys: &mut Snes) {
    debug_println!("STX &<todo>");
    let address = address_mode::absolute(sys);
    if cpu::status_flags::is_set_x(&sys.cpu) {
        helpers_8::stx(sys, address);
    } else {
        helpers_16::stx(sys, address);
    }
 }
 
 pub fn stx_96(sys: &mut Snes) {
    debug_println!("STX");
    let address = 0; //todo: implement DP Indexed,Y;
    if cpu::status_flags::is_set_x(&sys.cpu) {
        helpers_8::stx(sys, address);
    } else {
        helpers_16::stx(sys, address);
    }
 }
 
 pub fn sty_84(sys: &mut Snes) {
    debug_println!("STY &<todo>");
    let address = address_mode::direct(sys);
    if cpu::status_flags::is_set_x(&sys.cpu) {
        helpers_8::sty(sys, address);
    } else {
        helpers_16::sty(sys, address);
    }
 }
 
 pub fn sty_8c(sys: &mut Snes) {
    debug_println!("STY &<todo>");
    let address = address_mode::absolute(sys);
    if cpu::status_flags::is_set_x(&sys.cpu) {
        helpers_8::sty(sys, address);
    } else {
        helpers_16::sty(sys, address);
    }
 }
 
 pub fn sty_94(sys: &mut Snes) {
    debug_println!("STY");
    let address = 0; //todo: implement DP Indexed,X;
    if cpu::status_flags::is_set_x(&sys.cpu) {
        helpers_8::sty(sys, address);
    } else {
        helpers_16::sty(sys, address);
    }
 }
 
 pub fn stz_64(sys: &mut Snes) {
    debug_println!("STZ &<todo>");
    let address = address_mode::direct(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::stz(sys, address);
    } else {
        helpers_16::stz(sys, address);
    }
 }
 
 pub fn stz_74(sys: &mut Snes) {
    debug_println!("STZ");
    let address = 0; //todo: implement DP Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::stz(sys, address);
    } else {
        helpers_16::stz(sys, address);
    }
 }
 
 pub fn stz_9c(sys: &mut Snes) {
    debug_println!("STZ &<todo>");
    let address = address_mode::absolute(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::stz(sys, address);
    } else {
        helpers_16::stz(sys, address);
    }
 }
 
 pub fn stz_9e(sys: &mut Snes) {
    debug_println!("STZ");
    let address = 0; //todo: implement Absolute Indexed,X;
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::stz(sys, address);
    } else {
        helpers_16::stz(sys, address);
    }
 }
 
 pub fn tax_aa(sys: &mut Snes) {
    debug_println!("TAX");
   // todo!();
 }
 
 pub fn tay_a8(sys: &mut Snes) {
    debug_println!("TAY");
   // todo!();
 }
 
 pub fn tcd_5b(sys: &mut Snes) {
    debug_println!("TCD");
   // todo!();
 }
 
 pub fn tcs_1b(sys: &mut Snes) {
    debug_println!("TCS");
   // todo!();
 }
 
 pub fn tdc_7b(sys: &mut Snes) {
    debug_println!("TDC");
   // todo!();
 }
 
 pub fn trb_14(sys: &mut Snes) {
    debug_println!("TRB &<todo>");
    let address = address_mode::direct(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::trb(sys, address);
    } else {
        helpers_16::trb(sys, address);
    }
 }
 
 pub fn trb_1c(sys: &mut Snes) {
    debug_println!("TRB &<todo>");
    let address = address_mode::absolute(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::trb(sys, address);
    } else {
        helpers_16::trb(sys, address);
    }
 }
 
 pub fn tsb_04(sys: &mut Snes) {
    debug_println!("TSB &<todo>");
    let address = address_mode::direct(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::tsb(sys, address);
    } else {
        helpers_16::tsb(sys, address);
    }
 }
 
 pub fn tsb_0c(sys: &mut Snes) {
    debug_println!("TSB &<todo>");
    let address = address_mode::absolute(sys);
    if cpu::status_flags::is_set_m(&sys.cpu) {
        helpers_8::tsb(sys, address);
    } else {
        helpers_16::tsb(sys, address);
    }
 }
 
 pub fn tsc_3b(sys: &mut Snes) {
    debug_println!("TSC");
   // todo!();
 }
 
 pub fn tsx_ba(sys: &mut Snes) {
    debug_println!("TSX");
   // todo!();
 }
 
 pub fn txa_8a(sys: &mut Snes) {
    debug_println!("TXA");
   // todo!();
 }
 
 pub fn txs_9a(sys: &mut Snes) {
    debug_println!("TXS");
   // todo!();
 }
 
 pub fn txy_9b(sys: &mut Snes) {
    debug_println!("TXY");
   // todo!();
 }
 
 pub fn tya_98(sys: &mut Snes) {
    debug_println!("TYA");
   // todo!();
 }
 
 pub fn tyx_bb(sys: &mut Snes) {
    debug_println!("TYX");
   // todo!();
 }
 
 pub fn wai_cb(sys: &mut Snes) {
    debug_println!("WAI");
   // todo!();
 }
 
 pub fn wdm_42(sys: &mut Snes) {
    debug_println!("WDM");
   // todo!();
 }
 
 pub fn xba_eb(sys: &mut Snes) {
    debug_println!("XBA");
   // todo!();
 }
 
 pub fn xce_fb(sys: &mut Snes) {
    debug_println!("XCE");
   // todo!();
 }
 
 