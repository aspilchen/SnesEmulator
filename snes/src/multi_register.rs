/// Variable size register
///
/// The SNES has some registers that are 16 bits, but treated as
/// 8 bit registers depending on certain status flags.
///
/// This provides an easy interface to interact with such registers.
///
/// # Note
/// The 8 bit interface only provides access to the registers low byte.
///
/// # Reminders
/// SNES memory is in little endian. Keep that in mind if making any changes
/// here.
#[derive(Debug)]
pub struct MultiRegister {
    data: [u8; 2],
}

impl Default for MultiRegister {
    fn default() -> Self {
        Self { data: [0, 0] }
    }
}

impl MultiRegister {
    /// Get 16 bit value
    ///
    /// # Returns
    /// Two byte value stored in register
    pub fn get_16(&self) -> u16 {
        let result = u16::from_le_bytes(self.data);
        return result;
    }

    /// Get 8 bit value
    ///
    /// # Returns
    /// Low byte stored in register
    pub fn get_8(&self) -> u8 {
        let result = self.data[0];
        return result;
    }

    /// Set 16 bit value
    ///
    /// # Params
    /// * `value` - Two byte value to store
    pub fn set_16(&mut self, value: u16) {
        self.data = value.to_le_bytes();
    }

    /// Set 8 bit value
    ///
    /// # Params
    /// * `value` - One byte value to store
    pub fn set_8(&mut self, value: u8) {
        self.data[0] = value;
    }

    /// Swap low and high bytes
    pub fn swap_bytes(&mut self) {
        self.data.swap(0, 1);
    }
}
