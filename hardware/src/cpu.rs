use crate::state::State;
use crate::memory;

pub fn read_word(state: &mut State, address: usize) -> u16 {
    let cycle_count = 2;
    state.cycles += cycle_count;
    return memory::get_word(&state.mem, address);
}

pub fn read_byte(state: &mut State, address: usize) -> u8 {
    let cycle_count = 1;
    state.cycles += cycle_count;
    return memory::get_byte(&state.mem, address);
}

pub fn fetch_word(state: &mut State) -> u16 {
    let n_bytes = 2;
    let address = state.pc;
    let result = read_word(state, address);
    state.pc += n_bytes;
    return result;
}

pub fn fetch_byte(state: &mut State) -> u8 {
    let n_bytes = 1;
    let address = state.pc;
    let result = read_byte(state, address);
    state.pc += n_bytes;
    return result;
}

pub fn write_word(state: &mut State, address: usize, arg: u16) {
    let cycle_count = 2;
    state.cycles += cycle_count;
    memory::put_word(&mut state.mem, address, arg);
}