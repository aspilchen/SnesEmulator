use bitmask_enum::bitmask;

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


/// Helpers to make reading and using more clear.
impl Status {

    pub fn is_set_c(&self) -> bool {
        return self.intersects(Status::C);
    }

    pub fn is_set_m(&self) -> bool {
        return self.intersects(Status::M);
    }

    pub fn is_set_n(&self) -> bool {
        return self.intersects(Status::N);
    }

    pub fn is_set_v(&self) -> bool {
        return self.intersects(Status::V);
    }

    pub fn is_set_x(&self) -> bool {
        return self.intersects(Status::XB);
    }

    pub fn is_set_z(&self) -> bool {
        return self.intersects(Status::Z);
    }

    pub fn set_c(&mut self) {
        *self |= Status::C;
    }

    pub fn set_z(&mut self) {
        *self |= Status::Z;
    }

    pub fn set_i(&mut self) {
        *self |= Status::I;
    }

    pub fn set_d(&mut self) {
        *self |= Status::D;
    }

    pub fn set_x(&mut self) {
        *self |= Status::XB;
    }

    pub fn set_m(&mut self) {
        *self |= Status::M;
    }

    pub fn set_v(&mut self) {
        *self |= Status::V;
    }

    pub fn set_n(&mut self) {
        *self |= Status::N;
    }

    pub fn clear_c(&mut self) {
        *self &= !Status::C;
    }

    pub fn clear_z(&mut self) {
        *self &= !Status::Z;
    }

    pub fn clear_i(&mut self) {
        *self &= !Status::I;
    }

    pub fn clear_d(&mut self) {
        *self &= !Status::D;
    }

    pub fn clear_x(&mut self) {
        *self &= !Status::XB;
    }

    pub fn clear_m(&mut self) {
        *self &= !Status::M;
    }

    pub fn clear_v(&mut self) {
        *self &= !Status::V;
    }

    pub fn clear_n(&mut self) {
        *self &= !Status::N;
    }

    pub fn clear_status_bits(&mut self, arg: u8) {
        let bits = Status::from(!arg);
        *self &= bits;
    }

    pub fn set_status_bits(&mut self, arg: u8) {
        let bits = Status::from(arg);
        *self |= bits;
    }

}