use crate::state;
use crate::cpu;
use std::mem;


pub fn immediate16(state: &mut state::State) -> usize {
    let two_bytes = 2;
    let result = state.pc;
    state.pc += two_bytes;
    return result;
}

 pub fn immediate8(state: &mut state::State) -> usize {
    let one_byte = 1;
    let result = state.pc;
    state.pc += one_byte;
    return result;
}

 pub fn direct(state: &mut state::State) -> usize {
    let base = cpu::fetch_byte(state) as usize;
    let dp = state.d as usize;
    let result = base + dp;
    return result;
}

 pub fn absolute(state: &mut state::State) -> usize {
    let two_bytes = 16;
    let shifted_dbr = (state.dbr as usize) << two_bytes;
    let base = cpu::fetch_word(state) as usize;
    let result = shifted_dbr + base;
    return result;
}

 pub fn absolute_long(state: &mut state::State) -> usize {
    let mut bytes = [0; mem::size_of::<usize>()];
    bytes[0] = cpu::fetch_byte(state);
    bytes[1] = cpu::fetch_byte(state);
    bytes[2] = cpu::fetch_byte(state);
    let result = usize::from_le_bytes(bytes);
    return result;
}

 pub fn absolute_long_indexed(state: &mut state::State) -> usize {
    let index = state.get_a16() as usize;
    let result = absolute_long(state) + index;
    return result;
}

 pub fn absolute_indexed_x(state: &mut state::State) -> usize {
    let arg = cpu::fetch_word(state);
    let index = state.get_x16();
    let base = arg.wrapping_add(index) as usize;
    let two_bytes = 16;
    let shifted_dbr = (state.dbr as usize) << two_bytes;
    let result = shifted_dbr + base;
    return result; 
}

 pub fn absolute_indexed_y(state: &mut state::State) -> usize {
    let arg = cpu::fetch_word(state);
    let index = state.get_y16();
    let base = arg.wrapping_add(index) as usize;
    let two_bytes = 16;
    let shifted_dbr = (state.dbr as usize) << two_bytes;
    let result = shifted_dbr + base;
    return result; 
}

 pub fn direct_indexed_x(state: &mut state::State) -> usize {
    let index = state.get_x16() as usize;
    let secondary = direct(state);
    let result = secondary + index;
    return result;
}

 pub fn direct_indexed_y(state: &mut state::State) -> usize {
    let index = state.get_y16() as usize;
    let secondary = direct(state);
    let result = secondary + index;
    return result;
}

//  pub fn absolute_indexed_indirect(state: &mut state::State) -> usize {
//     let 
//     let x = u16::from_le_bytes(state.x);
//     let abs = cpu::fetch_word(state) + x;
//     let result = state.mem.get_word(abs as usize);
//     return result as usize;
// }

 pub fn direct_indexed_indirect(state: &mut state::State) -> usize {
    let secondary = direct_indexed_x(state);
    let two_bytes = 16;
    let shifted_dbr = (state.dbr as usize) << two_bytes;
    let base = cpu::read_word(state, secondary) as usize;
    let result = shifted_dbr + base;
    return result;
}

 pub fn direct_indirect(state: &mut state::State) -> usize {
     let secondary = direct(state);
    let two_bytes = 16;
    let shifted_dbr = (state.dbr as usize) << two_bytes;
    let base = cpu::read_word(state, secondary) as usize;
    let result = shifted_dbr + base;
    return result;
}

 pub fn direct_indirect_indexed(state: &mut state::State) -> usize {
    let index = state.get_y16();
    let base = direct_indirect(state) as u16;
    let base = base.wrapping_add(index) as usize;
    let two_bytes = 16;
    let shifted_dbr = (state.dbr as usize) << two_bytes;
    let result = shifted_dbr + base;
    return result;
}

 pub fn direct_indirect_indexed_long(state: &mut state::State) -> usize {
    let secondary = direct(state);
    let mut bytes = [0; mem::size_of::<usize>()];
    bytes[0] = cpu::read_byte(state, secondary);
    bytes[1] = cpu::read_byte(state, secondary+1);
    bytes[2] = cpu::read_byte(state, secondary+2);
    let index = state.get_y16() as usize;
    let base = usize::from_le_bytes(bytes);
    let result = base + index;
    return result;
}

 pub fn direct_indirect_long(state: &mut state::State) -> usize {
    let secondary = direct(state);
    let mut bytes = [0; mem::size_of::<usize>()];
    bytes[0] = cpu::read_byte(state, secondary);
    bytes[1] = cpu::read_byte(state, secondary+1);
    bytes[2] = cpu::read_byte(state, secondary+2);
    let result = usize::from_le_bytes(bytes);
    return result;
}