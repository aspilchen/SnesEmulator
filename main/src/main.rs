use std::io;

use snes::memory;
use snes::ricoh5a22;
use snes::snes::*;

fn display_immediate(name: &str, sys: &Snes) {
    let address = ricoh5a22::get_pc_address(&sys.cpu);
    let value = memory::io::byte::read(&sys.mem, address + 1);
    print!("{} #${:02X}", name, value);
}

fn display_dp(name: &str, sys: &Snes) {
    let address = ricoh5a22::get_pc_address(&sys.cpu);
    let value = memory::io::byte::read(&sys.mem, address + 1);
    print!("{} ${:02X}", name, value);
}

fn curr_op(sys: &Snes) {
    let address = ricoh5a22::get_pc_address(&sys.cpu);
    let op = memory::io::byte::read(&sys.mem, address);

    match op {
        0x61 => print!("ADC"),
        0x63 => print!("ADC"),
        0x65 => display_dp("ADC", sys),
        0x67 => print!("ADC"),
        0x69 => display_immediate("ADC", sys),
        0x6D => print!("ADC"),
        0x6F => print!("ADC"),
        0x71 => print!("ADC"),
        0x72 => print!("ADC"),
        0x73 => print!("ADC"),
        0x75 => print!("ADC"),
        0x77 => print!("ADC"),
        0x79 => print!("ADC"),
        0x7D => print!("ADC"),
        0x7F => print!("ADC"),
        0x21 => print!("AND"),
        0x23 => print!("AND"),
        0x25 => display_dp("AND", sys),
        0x27 => print!("AND"),
        0x29 => display_immediate("AND", sys),
        0x2D => print!("AND"),
        0x2F => print!("AND"),
        0x31 => print!("AND"),
        0x32 => print!("AND"),
        0x33 => print!("AND"),
        0x35 => print!("AND"),
        0x37 => print!("AND"),
        0x39 => print!("AND"),
        0x3D => print!("AND"),
        0x3F => print!("AND"),
        0x06 => display_dp("ASL", sys),
        0x0A => print!("ASL"),
        0x0E => print!("ASL"),
        0x16 => print!("ASL"),
        0x1E => print!("ASL"),
        0x90 => print!("BCC"),
        0xB0 => print!("BCS"),
        0xF0 => print!("BEQ"),
        0x24 => display_dp("BIT", sys),
        0x2C => print!("BIT"),
        0x34 => print!("BIT"),
        0x3C => print!("BIT"),
        0x89 => display_immediate("BIT", sys),
        0x30 => print!("BMI"),
        0xD0 => print!("BNE"),
        0x10 => print!("BPL"),
        0x80 => print!("BRA"),
        0x00 => print!("BRK"),
        0x82 => print!("BRL"),
        0x50 => print!("BVC"),
        0x70 => print!("BVS"),
        0x18 => print!("CLC"),
        0xD8 => print!("CLD"),
        0x58 => print!("CLI"),
        0xB8 => print!("CLV"),
        0xC1 => print!("CMP"),
        0xC3 => print!("CMP"),
        0xC5 => display_dp("CMP", sys),
        0xC7 => print!("CMP"),
        0xC9 => display_immediate("CMP", sys),
        0xCD => print!("CMP"),
        0xCF => print!("CMP"),
        0xD1 => print!("CMP"),
        0xD2 => print!("CMP"),
        0xD3 => print!("CMP"),
        0xD5 => print!("CMP"),
        0xD7 => print!("CMP"),
        0xD9 => print!("CMP"),
        0xDD => print!("CMP"),
        0xDF => print!("CMP"),
        0x02 => print!("COP"),
        0xE0 => display_immediate("CPX", sys),
        0xE4 => display_dp("CPX", sys),
        0xEC => print!("CPX"),
        0xC0 => display_immediate("CPY", sys),
        0xC4 => display_dp("CPY", sys),
        0xCC => print!("CPY"),
        0x3A => print!("DEC"),
        0xC6 => display_dp("DEC", sys),
        0xCE => print!("DEC"),
        0xD6 => print!("DEC"),
        0xDE => print!("DEC"),
        0xCA => print!("DEX"),
        0x88 => print!("DEY"),
        0x41 => print!("EOR"),
        0x43 => print!("EOR"),
        0x45 => display_dp("EOR", sys),
        0x47 => print!("EOR"),
        0x49 => display_immediate("EOR", sys),
        0x4D => print!("EOR"),
        0x4F => print!("EOR"),
        0x51 => print!("EOR"),
        0x52 => print!("EOR"),
        0x53 => print!("EOR"),
        0x55 => print!("EOR"),
        0x57 => print!("EOR"),
        0x59 => print!("EOR"),
        0x5D => print!("EOR"),
        0x5F => print!("EOR"),
        0x1A => print!("INC"),
        0xE6 => display_dp("INC", sys),
        0xEE => print!("INC"),
        0xF6 => print!("INC"),
        0xFE => print!("INC"),
        0xE8 => print!("INX"),
        0xC8 => print!("INY"),
        0x4C => print!("JMP"),
        0x5C => print!("JMP"),
        0x6C => print!("JMP"),
        0x7C => print!("JMP"),
        0xDC => print!("JMP"),
        0x20 => print!("JSR"),
        0x22 => print!("JSR"),
        0xFC => print!("JSR"),
        0xA1 => print!("LDA"),
        0xA3 => print!("LDA"),
        0xA5 => display_dp("LDA", sys),
        0xA7 => print!("LDA"),
        0xA9 => display_immediate("LDA", sys),
        0xAD => print!("LDA"),
        0xAF => print!("LDA"),
        0xB1 => print!("LDA"),
        0xB2 => print!("LDA"),
        0xB3 => print!("LDA"),
        0xB5 => print!("LDA"),
        0xB7 => print!("LDA"),
        0xB9 => print!("LDA"),
        0xBD => print!("LDA"),
        0xBF => print!("LDA"),
        0xA2 => display_immediate("LDX", sys),
        0xA6 => display_dp("LDX", sys),
        0xAE => print!("LDX"),
        0xB6 => print!("LDX"),
        0xBE => print!("LDX"),
        0xA0 => display_immediate("LDY", sys),
        0xA4 => display_dp("LDY", sys),
        0xAC => print!("LDY"),
        0xB4 => print!("LDY"),
        0xBC => print!("LDY"),
        0x46 => display_dp("LSR", sys),
        0x4A => print!("LSR"),
        0x4E => print!("LSR"),
        0x56 => print!("LSR"),
        0x5E => print!("LSR"),
        0x54 => print!("MVN"),
        0x44 => print!("MVP"),
        0xEA => print!("NOP"),
        0x01 => print!("ORA"),
        0x03 => print!("ORA"),
        0x05 => display_dp("ORA", sys),
        0x07 => print!("ORA"),
        0x09 => display_immediate("ORA", sys),
        0x0D => print!("ORA"),
        0x0F => print!("ORA"),
        0x11 => print!("ORA"),
        0x12 => print!("ORA"),
        0x13 => print!("ORA"),
        0x15 => print!("ORA"),
        0x17 => print!("ORA"),
        0x19 => print!("ORA"),
        0x1D => print!("ORA"),
        0x1F => print!("ORA"),
        0xF4 => print!("PEA"),
        0xD4 => print!("PEI"),
        0x62 => print!("PER"),
        0x48 => print!("PHA"),
        0x8B => print!("PHB"),
        0x0B => print!("PHD"),
        0x4B => print!("PHK"),
        0x08 => print!("PHP"),
        0xDA => print!("PHX"),
        0x5A => print!("PHY"),
        0x68 => print!("PLA"),
        0xAB => print!("PLB"),
        0x2B => print!("PLD"),
        0x28 => print!("PLP"),
        0xFA => print!("PLX"),
        0x7A => print!("PLY"),
        0xC2 => display_immediate("REP", sys),
        0x26 => display_dp("ROL", sys),
        0x2A => print!("ROL"),
        0x2E => print!("ROL"),
        0x36 => print!("ROL"),
        0x3E => print!("ROL"),
        0x66 => display_dp("ROR", sys),
        0x6A => print!("ROR"),
        0x6E => print!("ROR"),
        0x76 => print!("ROR"),
        0x7E => print!("ROR"),
        0x40 => print!("RTI"),
        0x6B => print!("RTL"),
        0x60 => print!("RTS"),
        0xE1 => print!("SBC"),
        0xE3 => print!("SBC"),
        0xE5 => display_dp("SBC", sys),
        0xE7 => print!("SBC"),
        0xE9 => display_immediate("SBC", sys),
        0xED => print!("SBC"),
        0xEF => print!("SBC"),
        0xF1 => print!("SBC"),
        0xF2 => print!("SBC"),
        0xF3 => print!("SBC"),
        0xF5 => print!("SBC"),
        0xF7 => print!("SBC"),
        0xF9 => print!("SBC"),
        0xFD => print!("SBC"),
        0xFF => print!("SBC"),
        0x38 => print!("SEC"),
        0xF8 => print!("SED"),
        0x78 => print!("SEI"),
        0xE2 => display_immediate("SEP", sys),
        0x81 => print!("STA"),
        0x83 => print!("STA"),
        0x85 => display_dp("STA", sys),
        0x87 => print!("STA"),
        0x8D => print!("STA"),
        0x8F => print!("STA"),
        0x91 => print!("STA"),
        0x92 => print!("STA"),
        0x93 => print!("STA"),
        0x95 => print!("STA"),
        0x97 => print!("STA"),
        0x99 => print!("STA"),
        0x9D => print!("STA"),
        0x9F => print!("STA"),
        0xDB => print!("STP"),
        0x86 => display_dp("STX", sys),
        0x8E => print!("STX"),
        0x96 => print!("STX"),
        0x84 => display_dp("STY", sys),
        0x8C => print!("STY"),
        0x94 => print!("STY"),
        0x64 => display_dp("STZ", sys),
        0x74 => print!("STZ"),
        0x9C => print!("STZ"),
        0x9E => print!("STZ"),
        0xAA => print!("TAX"),
        0xA8 => print!("TAY"),
        0x5B => print!("TCD"),
        0x1B => print!("TCS"),
        0x7B => print!("TDC"),
        0x14 => display_dp("TRB", sys),
        0x1C => print!("TRB"),
        0x04 => display_dp("TSB", sys),
        0x0C => print!("TSB"),
        0x3B => print!("TSC"),
        0xBA => print!("TSX"),
        0x8A => print!("TXA"),
        0x9A => print!("TXS"),
        0x9B => print!("TXY"),
        0x98 => print!("TYA"),
        0xBB => print!("TYX"),
        0xCB => print!("WAI"),
        0x42 => print!("WDM"),
        0xEB => print!("XBA"),
        0xFB => print!("XCE"),
        _ => todo!("0x{:02X}", op),
    }
    println!();
}

/// Hacky SNES debugger to step through ROM execution.
fn debugger(sys: &mut Snes) {
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).unwrap();

    loop {
        if input_str.trim() == "" {
            curr_op(sys);
            break;
        } else if input_str.trim() == "cpu" {
            print!("{}", sys.cpu);
        } else if input_str.trim() == "dump" {
            for i in 0x200..0x600 {
                if i % 0x100 == 0 {
                    println!();
                } else {
                    print!("{:02X} ", memory::io::byte::read(&sys.mem, i));
                }
            }
        } else if input_str.trim().as_bytes()[0] == b'x' {
            ricoh5a22::registers_8::set_x(&mut sys.cpu, 0);
            ricoh5a22::status_flags::set_z(&mut sys.cpu, true);
        } else {
            let address = usize::from_str_radix(&input_str.trim(), 16);
            if let Ok(addr) = address {
                let value = memory::io::byte::read(&sys.mem, addr);
                println!("{:02X}", value);
            }
        }
        input_str.clear();
        io::stdin().read_line(&mut input_str).unwrap();
    }
}

use test_game;
fn main() {
    let mut snes = Snes::default();
    let sys = &mut snes;

    test_game::load_binary(sys);
    test_game::run(sys);

    // Run game with debugger
    // test_game::run_inject_code(sys, debugger);
}
