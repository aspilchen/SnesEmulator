pub struct Memory {
    mem: [u8; 1024]
}

impl Default for Memory {
  fn default() -> Self {
    Self {mem: [0; 1024]}
  }
}

impl Memory {
  
  pub fn load(&mut self, binary: Vec<u8>) {
    self.mem[0 .. binary.len()].copy_from_slice(&binary[..]);
  }

  pub fn put_word(&mut self, addr: usize, value: u16) {
    let result = u16::to_le_bytes(value);
    self.mem[addr..addr+2].copy_from_slice(&result);
  }
  pub fn get_word(&mut self, addr:usize) -> u16 {
    let result = [self.mem[addr], self.mem[addr+1]];
    let result = u16::from_le_bytes(result);
    return result;
  }
  
  pub fn put_byte(&mut self, addr: usize, value: u8) {
    self.mem[addr] = value;
  }
  
  pub fn get_byte(&mut self, addr:usize) -> u8 {
    return self.mem[addr];
  }
}
    

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
