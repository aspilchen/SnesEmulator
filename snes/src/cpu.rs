//! CPU State

use core::fmt;

use bitmask_enum::bitmask;

use crate::memory::{self, Memory};

/// Status flags for the CPU
///
/// Flag names are taken from SNES hardware manuals.
/// The emulation flag is hidden, will be handled later.
#[bitmask(u8)]
pub enum Status {
    /// Carry
    C = 0b00000001,

    /// Zero
    Z = 0b00000010,

    /// IRQ
    I = 0b00000100,

    /// Decimal mode
    D = 0b00001000,

    /// In native mode
    /// - Index register size
    /// In emulation mode
    /// - Break
    XB = 0b00010000,

    /// Memory mode
    M = 0b00100000,

    /// Overflow
    V = 0b01000000,

    /// Negative
    N = 0b10000000,
    // 6502 emulation mode
    // E
}

/// Cpu State
pub struct Cpu {
    /// Accumulator
    pub a: [u8; 2],

    /// Index
    pub x: [u8; 2],

    /// Index
    pub y: [u8; 2],

    /// Stack Pointer
    pub s: u16,

    /// Data Bank
    pub dbr: u8,

    /// Direct Page Register
    pub d: u16,

    /// Program Bank
    pub pbr: u8,

    /// Status Register
    ///
    /// Status bits used for comparison operations and to
    /// control CPU behaviour.
    pub p: Status,

    /// Program Counter
    pub pc: u16,

    /// Emulation Flag
    ///
    /// Hidden flag to control emulation mode.
    /// When in emulation mode, system behaves like the NES.
    ///
    /// # Note
    /// As far as I can tell the SNES was meant to be backwards compatible
    /// with NES games, but this feature was never used. For now I will just
    /// be working with the native mode, and will not spend much time handling
    /// emulation features.
    pub e: bool,

    /// Cycle Count
    pub cycles: i32,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            a: [0, 0],
            x: [0, 0],
            y: [0, 0],
            s: 0x0100,
            dbr: 0,
            d: 0,
            pbr: 0,
            p: Status::none(),
            pc: 0,
            e: false,
            cycles: 0,
        }
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "------------------------\n")?;
        write!(
            f,
            "| A [0x{:02X}, 0x{:02X}] {:<6}|\n",
            self.a[0],
            self.a[1],
            registers_16::get_a(self)
        )?;
        write!(
            f,
            "| X [0x{:02X}, 0x{:02X}] {:<6}|\n",
            self.x[0],
            self.x[1],
            registers_16::get_x(self)
        )?;
        write!(
            f,
            "| Y [0x{:02X}, 0x{:02X}] {:<6}|\n",
            self.y[0],
            self.y[1],
            registers_16::get_y(self)
        )?;
        write!(f, "------------------------\n")?;
        write!(f, "| {:<5}{:<5}{:<5}{:<6}|\n", 'S', 'D', "DBR", "Cycle")?;
        write!(
            f,
            "| {:<5}{:<5}{:<5}{:<6}|\n",
            self.s, self.d, self.dbr, self.cycles
        )?;
        write!(f, "------------------------\n")?;
        write!(f, "|{:>10}:{:<11}|\n", "PBR", "PC")?;
        write!(f, "| hex    {:>02x}:{:<04x}       |\n", self.pbr, self.pc)?;
        write!(f, "------------------------\n")?;
        write!(f, "|{:>11}{:<11}|\n", "P", "")?;
        write!(f, "|       {:08b}       |\n", self.p)?;
        write!(f, "------------------------\n")
    }
}

pub fn add_cycles(cpu: &mut Cpu, amount: i32) {
    cpu.cycles += amount;
}

pub fn get_pc_address(cpu: &Cpu) -> usize {
    let bank = cpu.pbr as usize;
    let offset = cpu.pc as usize;
    let result = (bank << 16) + offset;
    return result;
}

pub fn set_pc_address(cpu: &mut Cpu, value: usize) {
    let bank = (value >> 16) as u8;
    let offset = value as u16;
    cpu.pbr = bank;
    cpu.pc = offset;
}

/// Set/Get status bits
pub mod status_flags {
    use super::*;

    pub fn is_set_c(cpu: &Cpu) -> bool {
        return cpu.p.intersects(Status::C);
    }

    pub fn is_set_m(cpu: &Cpu) -> bool {
        return cpu.p.intersects(Status::M);
    }

    pub fn is_set_n(cpu: &Cpu) -> bool {
        return cpu.p.intersects(Status::N);
    }

    pub fn is_set_v(cpu: &Cpu) -> bool {
        return cpu.p.intersects(Status::V);
    }

    pub fn is_set_x(cpu: &Cpu) -> bool {
        return cpu.p.intersects(Status::XB);
    }

    pub fn is_set_z(cpu: &Cpu) -> bool {
        return cpu.p.intersects(Status::Z);
    }

    pub fn set_c(cpu: &mut Cpu, on: bool) {
        if on {
            cpu.p |= Status::C;
        } else {
            cpu.p &= !Status::C;
        }
    }

    pub fn set_z(cpu: &mut Cpu, on: bool) {
        if on {
            cpu.p |= Status::Z;
        } else {
            cpu.p &= !Status::Z;
        }
    }

    pub fn set_i(cpu: &mut Cpu, on: bool) {
        if on {
            cpu.p |= Status::I;
        } else {
            cpu.p &= !Status::I;
        }
    }

    pub fn set_d(cpu: &mut Cpu, on: bool) {
        if on {
            cpu.p |= Status::D;
        } else {
            cpu.p &= !Status::D;
        }
    }

    pub fn set_x(cpu: &mut Cpu, on: bool) {
        if on {
            cpu.p |= Status::XB;
        } else {
            cpu.p &= !Status::XB;
        }
    }

    pub fn set_m(cpu: &mut Cpu, on: bool) {
        if on {
            cpu.p |= Status::M;
        } else {
            cpu.p &= !Status::M;
        }
    }

    pub fn set_v(cpu: &mut Cpu, on: bool) {
        if on {
            cpu.p |= Status::V;
        } else {
            cpu.p &= !Status::V;
        }
    }

    pub fn set_n(cpu: &mut Cpu, on: bool) {
        if on {
            cpu.p |= Status::N;
        } else {
            cpu.p &= !Status::N;
        }
    }

    pub fn clear_status_bits(cpu: &mut Cpu, arg: u8) {
        let bits = Status::from(!arg);
        cpu.p &= bits;
    }

    pub fn set_status_bits(cpu: &mut Cpu, arg: u8) {
        let bits = Status::from(arg);
        cpu.p |= bits;
    }
}

/// Setters/getters for 16 bit registers
pub mod registers_16 {
    use super::*;

    pub fn set_a(cpu: &mut Cpu, value: u16) {
        cpu.a = value.to_le_bytes();
    }

    pub fn set_x(cpu: &mut Cpu, value: u16) {
        cpu.x = value.to_le_bytes();
    }

    pub fn set_y(cpu: &mut Cpu, value: u16) {
        cpu.y = value.to_le_bytes();
    }

    pub fn set_d(cpu: &mut Cpu, value: u16) {
        cpu.d = value;
    }

    pub fn get_a(cpu: &Cpu) -> u16 {
        return u16::from_le_bytes(cpu.a);
    }

    pub fn get_x(cpu: &Cpu) -> u16 {
        return u16::from_le_bytes(cpu.x);
    }

    pub fn get_y(cpu: &Cpu) -> u16 {
        return u16::from_le_bytes(cpu.y);
    }

    pub fn get_d(cpu: &Cpu) -> u16 {
        return cpu.d;
    }
}

/// Setters/getters for 8 bit registers
pub mod registers_8 {
    use super::*;

    pub fn set_a(cpu: &mut Cpu, value: u8) {
        cpu.a[0] = value;
    }

    pub fn set_x(cpu: &mut Cpu, value: u8) {
        cpu.x[0] = value;
    }

    pub fn set_y(cpu: &mut Cpu, value: u8) {
        cpu.y[0] = value;
    }

    pub fn set_dbr(cpu: &mut Cpu, value: u8) {
        cpu.dbr = value;
    }

    pub fn set_pbr(cpu: &mut Cpu, value: u8) {
        cpu.pbr = value;
    }

    pub fn get_a(cpu: &Cpu) -> u8 {
        return cpu.a[0];
    }

    pub fn get_x(cpu: &Cpu) -> u8 {
        return cpu.x[0];
    }

    pub fn get_y(cpu: &Cpu) -> u8 {
        return cpu.y[0];
    }

    pub fn get_dbr(cpu: &Cpu) -> u8 {
        return cpu.dbr;
    }

    pub fn get_pbr(cpu: &Cpu) -> u8 {
        return cpu.pbr;
    }
}

/// 16 bit operations
pub mod ops_16 {
    use super::*;
    use registers_16 as reg;

    pub fn set_zn(cpu: &mut Cpu, value: u16) {
        let is_zero = value == 0;
        let is_negative = (value as i16) < 0;
        status_flags::set_z(cpu, is_zero);
        status_flags::set_n(cpu, is_negative);
    }

    pub fn lda(cpu: &mut Cpu, value: u16) {
        cpu.a = value.to_le_bytes();
        set_zn(cpu, value);
    }

    pub fn ldx(cpu: &mut Cpu, value: u16) {
        cpu.x = value.to_le_bytes();
        set_zn(cpu, value);
    }

    pub fn ldy(cpu: &mut Cpu, value: u16) {
        cpu.y = value.to_le_bytes();
        set_zn(cpu, value);
    }

    pub fn adc(cpu: &mut Cpu, rhs: u16) -> u16 {
        let lhs = reg::get_a(cpu);
        let (mut result, mut carry) = lhs.overflowing_add(rhs);

        let is_carry_set = status_flags::is_set_c(cpu);
        if is_carry_set {
            let (tmp_result, tmp_carry) = result.overflowing_add(1);
            result = tmp_result;
            carry |= tmp_carry;
        }

        let signed_overflow = ((lhs as i16) < 0 && (rhs as i16) < 0 && (result as i16) > 0)
            || ((lhs as i16) > 0 && (rhs as i16) > 0 && (result as i16) < 0);

        lda(cpu, result);
        status_flags::set_v(cpu, signed_overflow);
        status_flags::set_c(cpu, carry);
        return result;
    }

    pub fn sbc(cpu: &mut Cpu, rhs: u16) -> u16 {
        let lhs = reg::get_a(cpu);
        let (mut result, mut carry) = lhs.overflowing_sub(rhs);

        let is_carry_set = status_flags::is_set_c(cpu);
        if is_carry_set {
            let (tmp_result, tmp_carry) = result.overflowing_sub(1);
            result = tmp_result;
            carry |= tmp_carry;
        }

        let signed_overflow = ((lhs as i16) < 0 && (rhs as i16) > 0 && result > 0)
            || ((lhs as i16) > 0 && (rhs as i16) < 0 && (result as i16) < 0);

        lda(cpu, result);
        status_flags::set_v(cpu, signed_overflow);
        status_flags::set_c(cpu, carry);
        return result;
    }

    pub fn and(cpu: &mut Cpu, rhs: u16) -> u16 {
        let lhs = reg::get_a(cpu);
        let result = lhs & rhs;
        lda(cpu, result);
        return result;
    }

    pub fn eor(cpu: &mut Cpu, rhs: u16) -> u16 {
        let lhs = reg::get_a(cpu);
        let result = lhs ^ rhs;
        lda(cpu, result);
        set_zn(cpu, result);
        return result;
    }

    pub fn ora(cpu: &mut Cpu, rhs: u16) -> u16 {
        let lhs = reg::get_a(cpu);
        let result = lhs | rhs;
        lda(cpu, result);
        return result;
    }

    pub fn tsb(cpu: &mut Cpu, rhs: u16) -> u16 {
        let lhs = reg::get_a(cpu);
        let result = lhs | rhs;
        set_zn(cpu, result);
        return result;
    }

    pub fn trb(cpu: &mut Cpu, rhs: u16) -> u16 {
        let lhs = reg::get_a(cpu);
        let result = !lhs & rhs;
        set_zn(cpu, rhs);
        return result;
    }

    pub fn asl(cpu: &mut Cpu, value: u16) -> u16 {
        let carry = (value as i16) < 0;

        // If high bit is set, clear it to avoid avoid overflow and set carry flag.
        let result: u16 = if carry {
            let bitmask = 0x7FFF;
            (value & bitmask) << 1
        } else {
            value << 1
        };

        set_zn(cpu, result);
        status_flags::set_c(cpu, carry);
        return result;
    }

    pub fn lsr(cpu: &mut Cpu, value: u16) -> u16 {
        let carry = (value & 1) > 0;
        let result = value >> 1;
        set_zn(cpu, result);
        status_flags::set_c(cpu, carry);
        (cpu, carry);
        return result;
    }

    pub fn rol(cpu: &mut Cpu, value: u16) -> u16 {
        let is_carry_set = status_flags::is_set_c(cpu);
        let carry = (value as i16) < 0;

        let mut result = if carry {
            let bitmask = 0x7FFF;
            (value & bitmask) << 1
        } else {
            value << 1
        };

        if is_carry_set {
            result += 1;
        }

        set_zn(cpu, result);
        status_flags::set_c(cpu, carry);
        return result;
    }

    pub fn ror(cpu: &mut Cpu, value: u16) -> u16 {
        let is_carry_set = status_flags::is_set_c(cpu);
        let carry = (value & 1) > 0;

        let result: u16 = if is_carry_set {
            (value >> 1) | 0x8000
        } else {
            value >> 1
        };

        set_zn(cpu, result);
        status_flags::set_c(cpu, carry);
        return result;
    }

    pub fn bit(cpu: &mut Cpu, rhs: u16) -> u16 {
        let lhs = reg::get_a(cpu);
        let result = lhs & rhs;
        let is_set_bit_14 = (result & 1 << 13) > 0;
        status_flags::set_v(cpu, is_set_bit_14);
        set_zn(cpu, result);
        return result;
    }

    pub fn compare(cpu: &mut Cpu, lhs: u16, rhs: u16) {
        let result = lhs.wrapping_sub(rhs);
        let greater_equal = lhs >= rhs;
        status_flags::set_c(cpu, greater_equal);
        set_zn(cpu, result);
    }

    pub fn dec(cpu: &mut Cpu, value: u16) -> u16 {
        let result = value.wrapping_sub(1);
        set_zn(cpu, result);
        return result;
    }

    pub fn inc(cpu: &mut Cpu, value: u16) -> u16 {
        let result = value.wrapping_add(1);
        set_zn(cpu, value);
        return result;
    }
}
/// 8 bit operations
pub mod ops_8 {
    use super::*;
    use registers_8 as reg;

    pub fn set_zn(cpu: &mut Cpu, value: u8) {
        let is_zero = value == 0;
        let is_negative = (value as i8) < 0;
        status_flags::set_z(cpu, is_zero);
        status_flags::set_n(cpu, is_negative);
    }

    pub fn lda(cpu: &mut Cpu, value: u8) {
        cpu.a[0] = value;
    }

    pub fn ldx(cpu: &mut Cpu, value: u8) {
        cpu.x[0] = value;
    }

    pub fn ldy(cpu: &mut Cpu, value: u8) {
        cpu.y[0] = value;
    }

    pub fn adc(cpu: &mut Cpu, rhs: u8) -> u8 {
        let lhs = reg::get_a(cpu);
        let (mut result, mut carry) = lhs.overflowing_add(rhs);

        let is_carry_set = status_flags::is_set_c(cpu);
        if is_carry_set {
            let (tmp_result, tmp_carry) = result.overflowing_add(1);
            result = tmp_result;
            carry |= tmp_carry;
        }

        let signed_overflow = ((lhs as i8) < 0 && (rhs as i8) < 0 && (result as i8) > 0)
            || ((lhs as i8) > 0 && (rhs as i8) > 0 && (result as i8) < 0);

        lda(cpu, result);
        status_flags::set_v(cpu, signed_overflow);
        status_flags::set_c(cpu, carry);
        return result;
    }

    pub fn sbc(cpu: &mut Cpu, rhs: u8) -> u8 {
        let lhs = reg::get_a(cpu);
        let (mut result, mut carry) = lhs.overflowing_sub(rhs);

        let is_carry_set = status_flags::is_set_c(cpu);
        if is_carry_set {
            let (tmp_result, tmp_carry) = result.overflowing_sub(1);
            result = tmp_result;
            carry |= tmp_carry;
        }

        let signed_overflow = ((lhs as i8) < 0 && (rhs as i8) > 0 && result > 0)
            || ((lhs as i8) > 0 && (rhs as i8) < 0 && (result as i8) < 0);

        lda(cpu, result);
        status_flags::set_v(cpu, signed_overflow);
        status_flags::set_c(cpu, carry);
        return result;
    }

    pub fn and(cpu: &mut Cpu, rhs: u8) -> u8 {
        let lhs = reg::get_a(cpu);
        let result = lhs & rhs;
        lda(cpu, result);
        return result;
    }

    pub fn eor(cpu: &mut Cpu, rhs: u8) -> u8 {
        let lhs = reg::get_a(cpu);
        let result = lhs ^ rhs;
        lda(cpu, result);
        return result;
    }

    pub fn ora(cpu: &mut Cpu, rhs: u8) -> u8 {
        let lhs = reg::get_a(cpu);
        let result = lhs | rhs;
        lda(cpu, result);
        return result;
    }

    pub fn tsb(cpu: &mut Cpu, rhs: u8) -> u8 {
        let lhs = reg::get_a(cpu);
        let result = lhs | rhs;
        set_zn(cpu, result);
        return result;
    }

    pub fn trb(cpu: &mut Cpu, rhs: u8) -> u8 {
        let lhs = reg::get_a(cpu);
        let result = !lhs & rhs;
        set_zn(cpu, rhs);
        return result;
    }

    pub fn asl(cpu: &mut Cpu, value: u8) -> u8 {
        let carry = (value as i8) < 0;

        // If high bit is set, clear it to avoid avoid overflow and set carry flag.
        let result: u8 = if carry {
            let bitmask = 0x7F;
            (value & bitmask) << 1
        } else {
            value << 1
        };

        set_zn(cpu, result);
        status_flags::set_c(cpu, carry);
        (cpu, carry);
        return result;
    }

    pub fn lsr(cpu: &mut Cpu, value: u8) -> u8 {
        let carry = (value & 1) > 0;
        let result = value >> 1;
        set_zn(cpu, result);
        status_flags::set_c(cpu, carry);
        (cpu, carry);
        return result;
    }

    pub fn rol(cpu: &mut Cpu, value: u8) -> u8 {
        let is_carry_set = status_flags::is_set_c(cpu);
        let carry = (value as i8) < 0;

        let mut result = if carry {
            let bitmask = 0x7F;
            (value & bitmask) << 1
        } else {
            value << 1
        };

        if is_carry_set {
            result += 1;
        }

        set_zn(cpu, result);
        status_flags::set_c(cpu, carry);
        return result;
    }

    pub fn ror(cpu: &mut Cpu, value: u8) -> u8 {
        let is_carry_set = status_flags::is_set_c(cpu);
        let carry = (value & 1) > 0;

        let result: u8 = if is_carry_set {
            (value >> 1) | 0x80
        } else {
            value >> 1
        };

        set_zn(cpu, result);
        status_flags::set_c(cpu, carry);
        return result;
    }

    pub fn bit(cpu: &mut Cpu, rhs: u8) -> u8 {
        let lhs = reg::get_a(cpu);
        let result = lhs & rhs;
        let is_set_bit_6 = (result & (1 << 5)) > 0;
        status_flags::set_v(cpu, is_set_bit_6);
        set_zn(cpu, result);
        return result;
    }

    pub fn compare(cpu: &mut Cpu, lhs: u8, rhs: u8) {
        let result = lhs.wrapping_sub(rhs);
        let greater_equal = lhs >= rhs;
        status_flags::set_c(cpu, greater_equal);
        set_zn(cpu, result);
    }

    pub fn dec(cpu: &mut Cpu, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        set_zn(cpu, result);
        return result;
    }

    pub fn inc(cpu: &mut Cpu, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        set_zn(cpu, value);
        return result;
    }
}
