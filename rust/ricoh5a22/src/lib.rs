use memory::Memory;
use std::fmt;
use bitmask::bitmask;

pub enum AddressMode {
    Immediate,
}

pub enum Instruction {
    ADC(AddressMode),
    Error(u8),
}

bitmask! {
    mask Status: u8 where flags PFlags {
        N = 0b10000000, // Negative
        V = 0b01000000, // Overflow
        Z = 0b00000010, // Zero
        C = 0b00000001, // Carry
        D = 0b00001000, // Decimal
        I = 0b00000100, // IRQ disable
        XB = 0b00010000, // Index register size (native mode only)
        M = 0b00100000, // Accumulator register size (native mode only)
        // B = 0b00010000, // Break (emulation mode only) 
        //const E 	 		    6502 emulation mode
    }
}

pub struct Ricoh5A22 {
    a: [u8; 2],
    x: [u8; 2],
    y: [u8; 2],
    p: Status,
    pc: usize,
    mem: Memory,
}

impl Default for Ricoh5A22 {
    fn default() -> Self {
        Self {
            a: [0,0],
            x: [0,0],
            y: [0,0],
            p: Status::none(),
            pc: 0,
            mem: Memory::default(),
        }
    }
}


impl fmt::Display for Ricoh5A22 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A: {} {:?}\n", u16::from_le_bytes(self.a), self.a)?;
        write!(f, "X: {} {:?}\n", u16::from_le_bytes(self.x), self.x)?;
        write!(f, "Y: {} {:?}\n", u16::from_le_bytes(self.y), self.y)?;
        write!(f, "P: {:08b}\n", *self.p)?;
        return write!(f, "PC: {}", self.pc);
    }
}


impl Ricoh5A22 {

    pub fn load(&mut self, binary: Vec<u8>) {
        self.mem.load(binary);
    }

    pub fn fetch(&mut self) -> Instruction {
        let op = self.fetch_byte();
        match op {
            0x69 => return Instruction::ADC(AddressMode::Immediate),
            _    => return Instruction::Error(op),
        }
    }

    pub fn execute(&mut self, op: &Instruction) {
        match op {
            Instruction::ADC(addr) => self.adc(addr),
            Instruction::Error(code) => println!("Op {:#04X}", code),
        }
    }

    pub fn fetch_word(&mut self) -> u16 {
        let word = self.mem.get_word(self.pc);
        self.pc += 2;
        return word;
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let byte = self.mem.get_byte(self.pc);
        self.pc += 1;
        return byte;
    }

    pub fn addr_immediate16(&mut self) -> usize {
        let result = self.pc;
        self.pc += 2;
        return result;
    }

    pub fn addr_immediate8(&mut self) -> usize {
        let result = self.pc;
        self.pc += 1;
        return result;
    }

    pub fn get_address(&mut self, addr: &AddressMode) -> usize {
        match addr {
            AddressMode::Immediate => return self.addr_immediate16(),
            _ => return 0,
        }
    }

    pub fn set_status_carry(&mut self) {
        self.p.set(PFlags::C);
    }

    pub fn adc(&mut self, addr: &AddressMode) {
        let addr = self.get_address(&addr);
        let arg = self.mem.get_word(addr);
        self.adc16(arg);
    }

    pub fn adc16(&mut self, arg: u16) {
        let acc = u16::from_le_bytes(self.a);
        
        let result = 
            if self.p.intersects(PFlags::C) {acc.checked_add(arg + 1)}
            else {acc.checked_add(arg)};

        match result {
            Some(x) => self.a = x.to_le_bytes(),
            _ => {
                self.set_status_carry();
                let result = acc.wrapping_add(arg);
                self.a = result.to_le_bytes();
            }
        }
    }

    pub fn adc8(&mut self, arg: u8) {
        let acc = self.a[0];
        
        let result = 
            if self.p.intersects(PFlags::C) {acc.checked_add(arg + 1)}
            else {acc.checked_add(arg)};

        match result {
            Some(x) => self.a[0] = x,
            _ => {
                self.set_status_carry();
                let result = acc.wrapping_add(arg);
                self.a[0] = result;
            }
        }
    }
}