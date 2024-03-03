

use snes::snes::*;
use std::fs;


fn main() {
    let bin = fs::read("test_data/Gradius III (USA).sfc").expect("wtf");
    let mut sys = Snes::default();
    load_binary(&mut sys, &bin);
    run_for(&mut sys, 10);
}
