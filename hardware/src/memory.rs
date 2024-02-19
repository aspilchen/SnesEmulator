
pub fn load(mem: &mut Vec<u8>, binary: Vec<u8>) {
  mem[0 .. binary.len()].copy_from_slice(&binary[..]);
}

pub fn put_word(mem: &mut Vec<u8>, address: usize, value: u16) {
  let result = u16::to_le_bytes(value);
  mem[address] = result[0];
  mem[address+1] = result[1];
}

pub fn put_byte(mem: &mut Vec<u8>, address: usize, value: u8) {
  mem[address] = value;
}

pub fn get_word(mem: &Vec<u8>, address:usize) -> u16 {
  let bytes = [mem[address], mem[address+1]];
  let result = u16::from_le_bytes(bytes);
  return result;
}

pub fn get_byte(mem: &Vec<u8>, address:usize) -> u8 {
  return mem[address];
}
