use snes::state;
use snes::cpu;
// use snes::instructions;
use snes::memory;

fn main() {
    let n_cycles = 6;
    let mut state = state::State::default();
    let binary: Vec<u8> = vec![0x69, 1, 0, 0x69, 10, 0];
    memory::load(&mut state, &binary);

    
    // while state.cycles < 10 {
        let op = cpu::fetch_byte(&mut state);
        cpu::execute(&mut state, op);
        println!("{}", state);
        let op = cpu::fetch_byte(&mut state);
        cpu::execute(&mut state, op);
        println!("{}", state);
    // }

}
