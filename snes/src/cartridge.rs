//! Cartridge Memory mapping

pub enum CartridgeType {
    LoRom,
    HiRom,
    None,
}

/// LoRom Memory Map
pub mod lo_rom {
    use super::*;

    pub mod wram {
        use super::{make_address, split_addr};

        pub const FIRST_BANK: u8 = 0x7E;
        pub const LAST_BANK: u8 = 0x7F;
        pub const BLOCK_SIZE: u16 = 0xFFFF;
        pub const MAX_SIZE: usize = 0x020000;
        pub const BEGIN: usize = 0x7E0000;
        pub const STACK_INIT: u16 = 0x01FF;

        pub fn in_bounds(bank: u8, offset: u16) -> bool {
            let valid_bank = bank >= FIRST_BANK;
            let is_mirror = mirror::in_bounds(bank, offset);
            return valid_bank || is_mirror;
        }

        pub fn map_address(address: usize) -> usize {
            let ram_begin = super::rom::MAX_SIZE;
            let (bank, offset) = split_addr(address);
            let result = if mirror::in_bounds(bank, offset) {
                ram_begin + offset as usize
            } else {
                ram_begin + (address - BEGIN)
            };
            return result;
        }

        pub mod mirror {
            pub const FIRST_BANK: u8 = 0;
            pub const LAST_BANK: u8 = 0x3F;
            pub const BLOCK_SIZE: u16 = 0x2000;
            pub const REFLECT_BANK: u8 = 0x7E;

            pub fn in_bounds(bank: u8, offset: u16) -> bool {
                let valid_bank = bank <= LAST_BANK;
                let valid_offset = offset < BLOCK_SIZE;
                let is_mirror = valid_bank && valid_offset;
                let is_second_mirror = second_mirror::in_bounds(bank, offset);
                return is_mirror || is_second_mirror;
            }

            pub mod second_mirror {
                pub const FIRST_BANK: u8 = 0x80;
                pub const LAST_BANK: u8 = 0xBF;
                pub const BLOCK_SIZE: u16 = 0x2000;
                pub const REFLECT_BANK: u8 = 0x7E;

                pub fn in_bounds(bank: u8, offset: u16) -> bool {
                    let valid_bank = LAST_BANK >= bank && bank >= FIRST_BANK;
                    let valid_offset = offset < BLOCK_SIZE;
                    return valid_bank && valid_offset;
                }
            }
        }
    }

    pub mod rom {
        use super::*;

        pub const FIRST_BANK: u8 = 0x80;
        pub const LAST_BANK: u8 = 0xFF;
        pub const OFFSET: u16 = 0x8000;
        pub const BLOCK_SIZE: u16 = 0x8000;
        pub const MAX_SIZE: usize = 4000000;

        pub fn in_bounds(bank: u8, offset: u16) -> bool {
            let valid_bank = bank >= FIRST_BANK;
            let valid_offset = offset >= OFFSET;
            let is_rom = valid_bank && valid_offset;
            let is_mirror = mirror::in_bounds(bank, offset);
            return is_rom || is_mirror;
        }

        pub fn map_address(address: usize) -> usize {
            let (bank, offset) = split_addr(address);
            let mapped_offset = offset - OFFSET;
            let mapped_bank = BLOCK_SIZE * bank as u16;
            let result = (mapped_bank as usize) + (mapped_offset as usize);
            return result;
        }

        /// Rom addressing
        ///
        /// If incremented value leaves rom bounds (ex. crosses banks),
        /// value is "wrapped" to stay within rom bounds.
        pub fn wrapped_increment(address: usize) -> usize {
            return wrapped_add(address, 1);
        }

        /// Rom addressing
        ///
        /// If incremented value leaves rom bounds (ex. crosses banks),
        /// value is "wrapped" to stay within rom bounds.
        pub fn wrapped_add(address: usize, value: i16) -> usize {
            let (bank, offset) = split_addr(address);
            let (new_offset, wrap) = offset.overflowing_add_signed(value);
            let result = if wrap {
                if value < 0 {
                    make_address(bank - 1, offset)
                } else {
                    make_address(bank + 1, new_offset)
                }
            } else {
                make_address(bank, new_offset)
            };
            return result;
        }

        pub mod mirror {
            pub const FIRST_BANK: u8 = 0x00;
            pub const LAST_BANK: u8 = 0x7D;
            pub const OFFSET: u16 = 0x8000;
            pub const BLOCK_SIZE: u16 = 0x8000;

            pub fn in_bounds(bank: u8, offset: u16) -> bool {
                let valid_bank = bank < LAST_BANK;
                let valid_offset = offset >= OFFSET;
                return valid_bank && valid_offset;
            }
        }

        pub mod inturrupt_vectors {
            pub mod native {
                pub const COP: usize = 0x00FFE4;
                pub const BRK: usize = 0x00FFE6;
                pub const ABORT: usize = 0x00FFE8;
                pub const NMI: usize = 0x00FFEA;
                pub const IRQ: usize = 0x00FFEE;
            }

            pub mod emulation {
                pub const COP: usize = 0x00FFF4;
                pub const ABORT: usize = 0x00FFF8;
                pub const NMI: usize = 0x00FFFA;
                pub const RES: usize = 0x00FFFC;
                pub const IRQ_BRK: usize = 0x00FFFE;
            }
        }
    }

    pub fn reserve_memory() -> Vec<u8> {
        let size = wram::MAX_SIZE + rom::MAX_SIZE;
        return vec![0; size];
    }

    pub fn map_address(address: usize) -> usize {
        let (bank, offset) = split_addr(address);
        let result = if rom::in_bounds(bank, offset) {
            rom::map_address(address)
        } else {
            address
        };
        return result;
    }
}

pub fn get_bank(address: usize) -> u8 {
    let two_bytes = 16;
    let result = (address >> two_bytes) as u8;
    return result;
}

pub fn get_page(address: usize) -> usize {
    let bitmask = 0x00FF00;
    return address & bitmask;
}

pub fn get_offset(address: usize) -> u16 {
    let bitmask = 0x00FFFF;
    let result = (address & bitmask) as u16;
    return result;
}

pub fn split_addr(address: usize) -> (u8, u16) {
    let bank = get_bank(address);
    let offset = get_offset(address);
    return (bank, offset);
}

pub fn make_address(bank: u8, offset: u16) -> usize {
    let bank = (bank as usize) << 16;
    let offset = offset as usize;
    let result = bank + offset;
    return result;
}

//     pub mod Header {
//         pub const BEGIN: usize = 0x008FC0;
//         pub const N_BYTES: usize = 64;

//         pub mod Title {
//             pub const N_BYTES: usize = 21;
//             pub const BEGIN: usize = 0x00FFC0;
//             pub const END: usize = BEGIN + N_BYTES;
//         }

//         pub mod MapMode {
//             pub const N_BYTES: usize = 1;
//             pub const BEGIN: usize = 0x00FFD5;
//             pub const END: usize = BEGIN + N_BYTES;
//         }

//         pub mod Memory {
//             pub const N_BYTES: usize = 3;
//             pub const BEGIN: usize = 0x00FFD6;
//             pub const ROM_TYPE: usize = BEGIN;
//             pub const ROM_SIZE: usize = BEGIN + 1;
//             pub const SRWAM_SIZE: usize = BEGIN + 2;
//             pub const END: usize = BEGIN + N_BYTES;
//         }

//         pub mod DeveloperId {
//             pub const N_BYTES: usize = 2;
//             pub const BEGIN: usize = 0x00FFD9;
//             pub const END: usize = BEGIN + N_BYTES;
//         }

//         pub mod Version {
//             pub const N_BYTES: usize = 1;
//             pub const BEGIN: usize = 0x00FFDB;
//             pub const END: usize = BEGIN + N_BYTES;
//         }

//         pub mod ChecksumComplement {
//             pub const N_BYTES: usize = 2;
//             pub const BEGIN: usize = 0x00FFDC;
//             pub const END: usize = BEGIN + N_BYTES;
//         }

//         pub mod Checksum {
//             pub const N_BYTES: usize = 2;
//             pub const BEGIN: usize = 0x00FFDE;
//             pub const END: usize = BEGIN + N_BYTES;
//         }
//     }

//     pub mod NativeInturruptVectors {
//         pub const COP: usize = 0x00FFE4;
//         pub const BRK: usize = 0x00FFE6;
//         pub const ABORT: usize = 0x00FFE8;
//         pub const NMI: usize = 0x00FFEA;
//         pub const IRQ: usize = 0x00FFEE;
//     }

//     pub mod EmulationInturruptVectors {
//         pub const COP: usize = 0x00FFF4;
//         pub const ABORT: usize = 0x00FFF8;
//         pub const NMI: usize = 0x00FFFA;
//         pub const RES: usize = 0x00FFFC;
//         pub const IRQ_BRK: usize = 0x00FFFE;
//     }

//     pub mod Wram {
//         pub const BANK_BEGIN: u8 = 0x7E;
//         pub const BANK_END: u8 = 0x80;
//         pub const BEGIN: usize = 0x7E0000;
//         pub const END: usize = 0x800000;
//         pub const MIRROR_BANK_BEGIN: u8 = 0x00;
//         pub const MIRROR_BANK_END: u8 = 0x40;
//         pub const MIRROR_SIZE: u16 = 0x2000;
//     }

//     pub mod Rom {
//         pub const BANK_BEGIN: u8 = 0x00;
//         pub const BANK_END: u8 = 0x7E;
//         pub const BLOCK_SIZE: u16 = 8000;
//         pub const OFFSET: u16 = 0x8000;
//     }

//     fn map_address(address: usize) -> usize {
//         let above_wram = address >= VirtualAddress::Wram::END;
//         let is_wram = address >= VirtualAddress::Wram::BEGIN && !above_wram;

//         let (bank, offset) = if above_wram {
//             memory::split_addr(address - VirtualAddress::Wram::END)
//         } else if is_wram {
//             memory::split_addr(address + PhysicalAddress::Wram::SHIFT)
//         } else {
//             memory::split_addr(address)
//         };

//         let is_wram_mirror = (bank < PhysicalAddress::Wram::MIRROR_BANK_END)
//             && (offset < PhysicalAddress::Wram::MIRROR_SIZE);

//         let result = if is_wram_mirror {
//             memory::make_address(PhysicalAddress::Wram::BANK_BEGIN, offset)
//         } else {
//             memory::make_address(bank, offset)
//         };
//         return result;
//     }
// }
