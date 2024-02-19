
use crate::{instructions::*, memory};

use std::fmt;
use bitmask::bitmask;
use std::mem;

bitmask! {
    pub mask Status: u8 where flags PFlags {
        N =  0b10000000, // Negative
        V =  0b01000000, // Overflow
        M =  0b00100000, // Accumulator register size (native mode only)
        XB = 0b00010000, // Index register size (native mode only)
        D =  0b00001000, // Decimal
        I =  0b00000100, // IRQ disable
        Z =  0b00000010, // Zero
        C =  0b00000001, // Carry
        // B = 0b00010000, // Break (emulation mode only) 
        //const E 	 		    6502 emulation mode
    }
}
pub struct State {
    a: [u8; 2],
     x: [u8; 2],
     y: [u8; 2],
    pub s: usize,
    pub pc: usize,
    pub dbr: u8,
    pub pbr: u8,
    pub d: u8,
    pub p: Status,
    pub cycles: u32,
    pub mem: Vec<u8>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            a: [0,0],
            x: [0,0],
            y: [0,0],
            s: 0,
            pc: 0,
            dbr: 0,
            pbr: 0,
            d: 0,
            p: Status::none(),
            cycles: 0,
            mem: vec![0; 1024],
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A: {} {:?}\n", u16::from_le_bytes(self.a), self.a)?;
        write!(f, "X: {} {:?}\n", u16::from_le_bytes(self.x), self.x)?;
        write!(f, "Y: {} {:?}\n", u16::from_le_bytes(self.y), self.y)?;
        write!(f, "P: {:08b}\n", *self.p)?;
        write!(f, "cycles: {}\n", self.cycles)?;
        return write!(f, "PC: {}", self.pc);
    }
}


impl State {
    pub fn get_a16(&self) -> u16 {
        return u16::from_le_bytes(self.a);
    }

    pub fn set_a16(&mut self, val: u16) {
        self.a = val.to_le_bytes();
    }

    pub fn get_x16(&self) -> u16 {
        return u16::from_le_bytes(self.x);
    }

    pub fn set_x16(&mut self, val: u16) {
        self.x = val.to_le_bytes();
    }

    pub fn get_y16(&self) -> u16 {
        return u16::from_le_bytes(self.y);
    }

    pub fn set_y16(&mut self, val: u16) {
        self.y = val.to_le_bytes();
    }

}


// impl Ricoh5A22 {
//     fn tax(&mut self) {
//         let value = u16::from_le_bytes(self.a);
//         self.set_zn(value);
//         self.x[0] = self.a[0];
//         self.x[1] = self.a[1];
//         self.cycles += 1;
//     }
    
//     fn tay(&mut self) {
//         let value = u16::from_le_bytes(self.a);
//         self.set_zn(value);
//         self.y[0] = self.a[0];
//         self.y[1] = self.a[1];
//         self.cycles += 1;
//     }
    
//     fn txa(&mut self) {
//         let value = u16::from_le_bytes(self.x);
//         self.set_zn(value);
//         self.a[0] = self.x[0];
//         self.a[1] = self.x[1];
//         self.cycles += 1;
//     }
    
//     fn tya(&mut self) {
//         let value = u16::from_le_bytes(self.y);
//         self.set_zn(value);
//         self.a[0] = self.y[0];
//         self.a[1] = self.y[1];
//         self.cycles += 1;
//     }
    
//     fn tsx(&mut self) {
//         let value = self.s as u16;
//         self.set_zn(value);
//         self.x = value.to_le_bytes();
//         self.cycles += 1;
//     }
    
//     fn txs(&mut self) {
//         let value = u16::from_le_bytes(self.x);
//         self.set_zn(value);
//         self.s = value as usize;
//     }
    
//     fn txy(&mut self) {
//         let value = u16::from_le_bytes(self.x);
//         self.set_zn(value);
//         self.y[0] = self.x[0];
//         self.y[1] = self.x[1];
//         self.cycles += 1;
//     }
   
//     fn tyx(&mut self) {
//         let value = u16::from_le_bytes(self.y);
//         self.set_zn(value);
//         self.x[0] = self.y[0];
//         self.x[1] = self.y[1];
//         self.cycles += 1;
//     }
    
// }




// impl Ricoh5A22 {
//     fn set_zn(&mut self, arg: u16) {
//         let is_zero = arg == 0;
//         if is_zero {
//             self.p.set(PFlags::Z);
//         } else  {
//             self.p.unset(PFlags::Z);
//         }

//         let is_negative = (arg as i16) < 0;
//         if is_negative {
//             self.p.set(PFlags::N);
//         } else  {
//             self.p.unset(PFlags::N);
//         }
//     }
// }