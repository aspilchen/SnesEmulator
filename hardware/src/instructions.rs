use crate::state;
use crate::cpu;
use crate::address_modes;

fn set_zero(state: &mut state::State, arg: u16) {
    let is_zero = arg == 0;
    if is_zero {
        state.p.set(state::PFlags::Z);
    } else  {
        state.p.unset(state::PFlags::Z);
    }
}

fn set_negative(state: &mut state::State, arg: u16) {
    let is_negative = (arg as i16) < 0;
    if is_negative {
        state.p.set(state::PFlags::N);
    } else  {
        state.p.unset(state::PFlags::N);
    }
}

fn set_zn16(state: &mut state::State, arg: u16) {
    set_zero(state, arg);
    set_negative(state, arg);
}

fn set_carry(state: &mut state::State, carry: bool) {
    if carry {
        state.p.set(state::PFlags::C);
    } else {
        state.p.unset(state::PFlags::C);
    }
}

fn set_overflow(state: &mut state::State, overflow: bool) {
    if overflow {
        state.p.set(state::PFlags::V);
    } else {
        state.p.unset(state::PFlags::V);
    }
}

fn adc16(state: &mut state::State, address: usize) {
    let acc = state.get_a16();
    let arg = cpu::read_word(state, address);
    let (mut result, mut carry, ) = acc.overflowing_add(arg);
    
    let is_carry_set = state.p.intersects(state::PFlags::C);
    if is_carry_set {
        let (tmp_result, tmp_carry) = result.overflowing_add(1);
        result = tmp_result;
        carry |= tmp_carry;
    }

    let signed_overflow = ((acc as i16) < 0 && (arg as i16) < 0 && result > 0) ||
        ((acc as i16) > 0 && (arg as i16) > 0 && result < 0);

    set_overflow(state, signed_overflow);
    set_carry(state, carry);
    set_zn16(state, result);
    state.set_a16(result);
}

fn and16(state: &mut state::State, address: usize) {
    let arg = cpu::read_word(state, address);
    let a = state.get_a16();
    let result = a & arg;
    set_zn16(state, result);
    state.set_a16(a);
}

fn eor16(state: &mut state::State, address: usize) {
    let arg = cpu::read_word(state, address);
    let a = state.get_a16();
    let result = a ^ arg;
    set_zn16(state, result);
    state.set_a16(result);
}

fn ora16(state: &mut state::State, address: usize) {
    let arg = cpu::read_word(state, address);
    let a = state.get_a16();
    let result = a | arg;
    set_zn16(state, result);
    state.set_a16(result);
}

fn tsb16(state: &mut state::State, address: usize) {
    let arg = cpu::read_word(state, address);
    let a = state.get_a16();
    let result = a | arg;
    set_zero(state, result);
    cpu::write_word(state, address, result);
}

fn trb16(state: &mut state::State, address: usize) {
    let arg = cpu::read_word(state, address);
    let a = state.get_a16();
    let result = !a & arg;
    set_zero(state, arg);
    cpu::write_word(state, address, result);
}

fn asl16_mem(state: &mut state::State, address: usize) {
    let arg = cpu::read_word(state, address);
    let carry = (arg as i16) < 0;

    let result: u16 = if carry {
        (arg & 0x7FFF) << 1
    } else {
        arg << 1
    };

    set_zn16(state, result);
    set_carry(state, carry);
    cpu::write_word(state, address, result);
}

fn asl16_acc(state: &mut state::State) {
    let arg = state.get_a16();
    let carry = (arg as i16) < 0;

    let result: u16 = if carry {
        (arg & 0x7FFF) << 1
    } else {
        arg << 1
    };

    set_zn16(state, result);
    set_carry(state, carry);
    state.set_a16(result);
}

fn lsr16_mem(state: &mut state::State, address: usize) {
    let arg = cpu::read_word(state, address);
    let carry = (arg & 1) > 0;
    let result = arg >> 1;
    set_zn16(state, result);
    set_carry(state, carry);
    cpu::write_word(state, address, result);
}

fn lsr16_acc(state: &mut state::State) {
    let arg = state.get_a16();
    let carry = (arg & 1) > 0;
    let result = arg >> 1;
    set_zn16(state, result);
    set_carry(state, carry);
    state.set_a16(result);
}

fn rol16_mem(state: &mut state::State, address: usize) {
    let arg = cpu::read_word(state, address);
    let is_carry_set = state.p.intersects(state::PFlags::C);
    let carry = (arg as i16) < 0;

    let mut result = if carry {
        (arg & 0x7FFF) << 1
    } else {
        arg << 1
    };

    if is_carry_set {
        result += 1;
    }

    set_zn16(state,result);
    set_carry(state, carry);
    cpu::write_word(state, address, result);
}

fn rol16_acc(state: &mut state::State) {
    let arg = state.get_a16();
    let is_carry_set = state.p.intersects(state::PFlags::C);
    let carry = (arg as i16) < 0;

    let mut result = if carry {
        (arg & 0x7FFF) << 1
    } else {
        arg << 1
    };

    if is_carry_set {
        result += 1;
    }

    set_zn16(state,result);
    set_carry(state, carry);
    state.set_a16(result);
}

fn ror16_mem(state: &mut state::State, address: usize) {
    let arg = cpu::read_word(state, address);
    let is_carry_set = state.p.intersects(state::PFlags::C);
    let carry = (arg & 1) > 0;

    let result: u16 = if is_carry_set {
        (arg >> 1) | 0x8000
    } else {
        arg >> 1
    };
    
    set_zn16(state, result);
    set_carry(state, carry);
    cpu::write_word(state, address, result);
}

fn ror16_acc(state: &mut state::State, address: usize) {
    let arg = state.get_a16();
    let is_carry_set = state.p.intersects(state::PFlags::C);
    let carry = (arg & 1) > 0;

    let result: u16 = if is_carry_set {
        (arg >> 1) | 0x8000
    } else {
        arg >> 1
    };
    
    set_zn16(state, result);
    set_carry(state, carry);
    state.set_a16(result);
}


fn lda16(state: &mut state::State, address: usize) {
    let result = cpu::read_word(state, address);
    set_zn16(state, result);
    state.set_a16(result);
}
      
fn ldx16(state: &mut state::State, address: usize) {
    let result = cpu::read_word(state, address);
    set_zn16(state, result);
    state.set_x16(result);
}

fn ldy16(state: &mut state::State, address: usize) {
    let result = cpu::read_word(state, address);
    set_zn16(state, result);
    state.set_y16(result);
}

fn sta16(state: &mut state::State, address: usize) {
    let result = state.get_a16();
    cpu::write_word(state, address, result);
}

fn sty16(state: &mut state::State, address: usize) {
    let result = state.get_y16();
    cpu::write_word(state, address, result);
}

fn stx16(state: &mut state::State, address: usize) {
    let result = state.get_x16();
    cpu::write_word(state, address, result);
}

fn stz16(state: &mut state::State, address: usize) {
    let result = 0;
    cpu::write_word(state, address, result);
}