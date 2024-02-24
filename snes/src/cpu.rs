//! CPU functions such as fetch, read and execute are found here.

use crate::instructions::*;
use crate::memory::{self, ONE_BYTE};
use crate::snes_address::SnesAddress;
use crate::state::State;


/// Program counter
///
/// Increments [`State::pc`] by `amount`
///
/// # Todo
/// * Handle incrementing the PBR
pub fn increment_pc(state: &mut State, amount: u16) {
    let mut address = state.pc.get_address();
    address += amount;
    state.pc.set_address(address);
}

/// Stack pointer
///
/// * Increment [`State::s`] by `amount`
///
/// # Todo
/// *  Handle incrementing the stack bank
pub fn increment_stack(state: &mut State, amount: u16) {
    let address = state.s.get_address() + amount;
    state.s.set_address(address);
    // todo!("Handle stack bank");
}

/// Stack pointer
///
/// * Decrement [`State::s`] by `amount`
///
/// # Todo
///  Handle incrementing the stack bank.
pub fn decrement_stack(state: &mut State, amount: u16) {
    let address = state.s.get_address() - amount;
    state.s.set_address(address);
}

/// CPU cycles
///
/// * Increment [`State::cycles`] by n_cycles
pub fn add_cycles(state: &mut State, n_cycles: u32) {
    state.cycles += n_cycles;
}

/// Memory read
///
/// * Read two bytes from memory at address
/// * Counts cycles
/// * Does *not* increment program counter
///
/// # Todo
/// * Adjust cycle count depending on type of memory acccess
///
/// # Params
/// * `state` - Current system state
/// * `address` - Source address to read from
///
/// # Returns
/// Two bytes from memory at address location
pub fn read_word(state: &mut State, address: &SnesAddress) -> u16 {
    let result = memory::get_word(state, address);
    add_cycles(state, 2);
    return result;
}

/// Memory read
///
/// * Read one byte from memory at address
/// * Counts cycles
/// * Does *not* increment program counter
///
/// # Todo
/// * Adjust cycle count depending on type of memory access
///
/// # Params
/// * `state` - Current system state
/// * `address` - Source address to read from
///
/// # Returns
/// One byte from memory at address location
pub fn read_byte(state: &mut State, address: &SnesAddress) -> u8 {
    let result = memory::get_byte(&state, address);
    add_cycles(state, 1);
    return result;
}

/// Memory read
///
/// * Fetches next word from memory at [`State::pc`]
/// * Counts cycles
/// * Increments [`State::pc`]
///
/// # Params
/// * `state` - Current system state
///
/// # Returns
/// Next two byte from program counter address
pub fn fetch_word(state: &mut State) -> u16 {
    let result = memory::get_word(&state, &state.pc);
    let cycle_count = 2;
    add_cycles(state, cycle_count);
    increment_pc(state, memory::BYTES_IN_WORD);
    return result;
}

/// Fetches next byte
///
/// * Fetches next byte from memory at [`State::pc`]
/// * Counts cycles
/// * Increments [`State::pc`]
///
/// # Params
/// * `state` - Current system state
///
/// # Returns
/// Next byte from program counter address
pub fn fetch_byte(state: &mut State) -> u8 {
    let result = memory::get_byte(&state, &state.pc);
    let cycle_count = 1;
    add_cycles(state, cycle_count);
    increment_pc(state, memory::ONE_BYTE);
    return result;
}

/// Memory write
///
/// * Writes two byte `value` to memory at `address`
/// * Counts cycles
///
/// # Todo
/// * Adjust cycles based of type of memory access
///
/// # Params
/// * `state` - Current system state
/// * `address` - Destination address
/// * `value` - Two byte value to be written
pub fn write_word(state: &mut State, address: &SnesAddress, value: u16) {
    memory::put_word(state, address, value);
    let cycle_count = 2;
    add_cycles(state, cycle_count);
}

/// Memory write
///
/// * Writes one byte `value` to memory at `address`
/// * Counts cycles
///
/// # Todo
/// * Adjust cycles based of type of memory access
///
/// # Params
/// * `state` - Current system state
/// * `address` - Destination address
/// * `value` - One byte value to be written
pub fn write_byte(state: &mut State, address: &SnesAddress, value: u8) {
    memory::put_byte(state, address, value);
    let cycle_count = 1;
    add_cycles(state, cycle_count);
}

/// Push to stack
///
/// * Stack pointer = [`State::s`]
/// * Write `value` to memory at stack pointer
/// * Decrement stack pointer
/// * Counts cycles
///
/// # Params
/// * `state` - Current system state
/// * `value` - Two byte value to be written
pub fn push_word(state: &mut State, value: u16) {
    decrement_stack(state, memory::BYTES_IN_WORD);
    let pc = state.pc.clone();
    memory::put_word(state, &pc, value);
    let cycle_count = 2;
    add_cycles(state, cycle_count);
}

/// Push to stack
///
/// * Stack pointer = [`State::s`]
/// * Decrement stack pointer
/// * Write `value` to memory at stack pointer
/// * Counts cycles
///
/// # Params
/// * `state` - Current system state
/// * `value` - One byte value to be written
pub fn push_byte(state: &mut State, value: u8) {
    decrement_stack(state, ONE_BYTE);
    let pc = state.pc.clone();
    memory::put_byte(state, &pc, value);
    let cycle_count = 1;
    add_cycles(state, cycle_count);
}

/// Pop from stack
///
/// * Stack pointer = [`State::s`]
/// * Read two bytes from memory at stack pointer
/// * Increment stack pointer
/// * Counts cycles
///
/// # Params
/// * `state` - Current system state
///
/// # Returns
/// Two byte value from top of stack
pub fn pop_word(state: &mut State) -> u16 {
    let result = memory::get_word(&state, &state.s);
    increment_stack(state, memory::BYTES_IN_WORD);
    let cycle_count = 2;
    add_cycles(state, cycle_count);
    return result;
}

/// Pop from stack
///
/// * Stack pointer = [`State::s`]
/// * Read one byte from memory at stack pointer
/// * Increment stack pointer
/// * Counts cycles
///
/// # Params
/// * `state` - Current system state
///
/// # Returns
/// One byte value from top of stack
pub fn pop_byte(state: &mut State) -> u8 {
    let result = memory::get_byte(&state, &state.pc);
    increment_pc(state, memory::ONE_BYTE);
    let cycle_count = 1;
    add_cycles(state, cycle_count);
    return result;
}

/// Program execution
///
/// Decodes and executes instruction
///
/// # Params
/// * `state` - Current system state
/// * `op_code` - One byte op code to be executed
pub fn execute(state: &mut State, op_code: u8) {
    match op_code {
        0x61 => adc_61(state),
        0x63 => adc_63(state),
        0x65 => adc_65(state),
        0x67 => adc_67(state),
        0x69 => adc_69(state),
        0x6D => adc_6d(state),
        0x6F => adc_6f(state),
        0x71 => adc_71(state),
        0x72 => adc_72(state),
        0x73 => adc_73(state),
        0x75 => adc_75(state),
        0x77 => adc_77(state),
        0x79 => adc_79(state),
        0x7D => adc_7d(state),
        0x7F => adc_7f(state),
        0x21 => and_21(state),
        0x23 => and_23(state),
        0x25 => and_25(state),
        0x27 => and_27(state),
        0x29 => and_29(state),
        0x2D => and_2d(state),
        0x2F => and_2f(state),
        0x31 => and_31(state),
        0x32 => and_32(state),
        0x33 => and_33(state),
        0x35 => and_35(state),
        0x37 => and_37(state),
        0x39 => and_39(state),
        0x3D => and_3d(state),
        0x3F => and_3f(state),
        0x06 => asl_06(state),
        0x0A => asl_0a(state),
        0x0E => asl_0e(state),
        0x16 => asl_16(state),
        0x1E => asl_1e(state),
        0x90 => bcc_90(state),
        0xB0 => bcs_b0(state),
        0xF0 => beq_f0(state),
        0x24 => bit_24(state),
        0x2C => bit_2c(state),
        0x34 => bit_34(state),
        0x3C => bit_3c(state),
        0x89 => bit_89(state),
        0x30 => bmi_30(state),
        0xD0 => bne_d0(state),
        0x10 => bpl_10(state),
        0x80 => bra_80(state),
        0x00 => brk_00(state),
        0x82 => brl_82(state),
        0x50 => bvc_50(state),
        0x70 => bvs_70(state),
        0x18 => clc_18(state),
        0xD8 => cld_d8(state),
        0x58 => cli_58(state),
        0xB8 => clv_b8(state),
        0xC1 => cmp_c1(state),
        0xC3 => cmp_c3(state),
        0xC5 => cmp_c5(state),
        0xC7 => cmp_c7(state),
        0xC9 => cmp_c9(state),
        0xCD => cmp_cd(state),
        0xCF => cmp_cf(state),
        0xD1 => cmp_d1(state),
        0xD2 => cmp_d2(state),
        0xD3 => cmp_d3(state),
        0xD5 => cmp_d5(state),
        0xD7 => cmp_d7(state),
        0xD9 => cmp_d9(state),
        0xDD => cmp_dd(state),
        0xDF => cmp_df(state),
        0x02 => cop_02(state),
        0xE0 => cpx_e0(state),
        0xE4 => cpx_e4(state),
        0xEC => cpx_ec(state),
        0xC0 => cpy_c0(state),
        0xC4 => cpy_c4(state),
        0xCC => cpy_cc(state),
        0x3A => dec_3a(state),
        0xC6 => dec_c6(state),
        0xCE => dec_ce(state),
        0xD6 => dec_d6(state),
        0xDE => dec_de(state),
        0xCA => dex_ca(state),
        0x88 => dey_88(state),
        0x41 => eor_41(state),
        0x43 => eor_43(state),
        0x45 => eor_45(state),
        0x47 => eor_47(state),
        0x49 => eor_49(state),
        0x4D => eor_4d(state),
        0x4F => eor_4f(state),
        0x51 => eor_51(state),
        0x52 => eor_52(state),
        0x53 => eor_53(state),
        0x55 => eor_55(state),
        0x57 => eor_57(state),
        0x59 => eor_59(state),
        0x5D => eor_5d(state),
        0x5F => eor_5f(state),
        0x1A => inc_1a(state),
        0xE6 => inc_e6(state),
        0xEE => inc_ee(state),
        0xF6 => inc_f6(state),
        0xFE => inc_fe(state),
        0xE8 => inx_e8(state),
        0xC8 => iny_c8(state),
        0x4C => jmp_4c(state),
        0x5C => jmp_5c(state),
        0x6C => jmp_6c(state),
        0x7C => jmp_7c(state),
        0xDC => jmp_dc(state),
        0x20 => jsr_20(state),
        0x22 => jsr_22(state),
        0xFC => jsr_fc(state),
        0xA1 => lda_a1(state),
        0xA3 => lda_a3(state),
        0xA5 => lda_a5(state),
        0xA7 => lda_a7(state),
        0xA9 => lda_a9(state),
        0xAD => lda_ad(state),
        0xAF => lda_af(state),
        0xB1 => lda_b1(state),
        0xB2 => lda_b2(state),
        0xB3 => lda_b3(state),
        0xB5 => lda_b5(state),
        0xB7 => lda_b7(state),
        0xB9 => lda_b9(state),
        0xBD => lda_bd(state),
        0xBF => lda_bf(state),
        0xA2 => ldx_a2(state),
        0xA6 => ldx_a6(state),
        0xAE => ldx_ae(state),
        0xB6 => ldx_b6(state),
        0xBE => ldx_be(state),
        0xA0 => ldy_a0(state),
        0xA4 => ldy_a4(state),
        0xAC => ldy_ac(state),
        0xB4 => ldy_b4(state),
        0xBC => ldy_bc(state),
        0x46 => lsr_46(state),
        0x4A => lsr_4a(state),
        0x4E => lsr_4e(state),
        0x56 => lsr_56(state),
        0x5E => lsr_5e(state),
        0x54 => mvn_54(state),
        0x44 => mvp_44(state),
        0xEA => nop_ea(state),
        0x01 => ora_01(state),
        0x03 => ora_03(state),
        0x05 => ora_05(state),
        0x07 => ora_07(state),
        0x09 => ora_09(state),
        0x0D => ora_0d(state),
        0x0F => ora_0f(state),
        0x11 => ora_11(state),
        0x12 => ora_12(state),
        0x13 => ora_13(state),
        0x15 => ora_15(state),
        0x17 => ora_17(state),
        0x19 => ora_19(state),
        0x1D => ora_1d(state),
        0x1F => ora_1f(state),
        0xF4 => pea_f4(state),
        0xD4 => pei_d4(state),
        0x62 => per_62(state),
        0x48 => pha_48(state),
        0x8B => phb_8b(state),
        0x0B => phd_0b(state),
        0x4B => phk_4b(state),
        0x08 => php_08(state),
        0xDA => phx_da(state),
        0x5A => phy_5a(state),
        0x68 => pla_68(state),
        0xAB => plb_ab(state),
        0x2B => pld_2b(state),
        0x28 => plp_28(state),
        0xFA => plx_fa(state),
        0x7A => ply_7a(state),
        0xC2 => rep_c2(state),
        0x26 => rol_26(state),
        0x2A => rol_2a(state),
        0x2E => rol_2e(state),
        0x36 => rol_36(state),
        0x3E => rol_3e(state),
        0x66 => ror_66(state),
        0x6A => ror_6a(state),
        0x6E => ror_6e(state),
        0x76 => ror_76(state),
        0x7E => ror_7e(state),
        0x40 => rti_40(state),
        0x6B => rtl_6b(state),
        0x60 => rts_60(state),
        0xE1 => sbc_e1(state),
        0xE3 => sbc_e3(state),
        0xE5 => sbc_e5(state),
        0xE7 => sbc_e7(state),
        0xE9 => sbc_e9(state),
        0xED => sbc_ed(state),
        0xEF => sbc_ef(state),
        0xF1 => sbc_f1(state),
        0xF2 => sbc_f2(state),
        0xF3 => sbc_f3(state),
        0xF5 => sbc_f5(state),
        0xF7 => sbc_f7(state),
        0xF9 => sbc_f9(state),
        0xFD => sbc_fd(state),
        0xFF => sbc_ff(state),
        0x38 => sec_38(state),
        0xF8 => sed_f8(state),
        0x78 => sei_78(state),
        0xE2 => sep_e2(state),
        0x81 => sta_81(state),
        0x83 => sta_83(state),
        0x85 => sta_85(state),
        0x87 => sta_87(state),
        0x8D => sta_8d(state),
        0x8F => sta_8f(state),
        0x91 => sta_91(state),
        0x92 => sta_92(state),
        0x93 => sta_93(state),
        0x95 => sta_95(state),
        0x97 => sta_97(state),
        0x99 => sta_99(state),
        0x9D => sta_9d(state),
        0x9F => sta_9f(state),
        0xDB => stp_db(state),
        0x86 => stx_86(state),
        0x8E => stx_8e(state),
        0x96 => stx_96(state),
        0x84 => sty_84(state),
        0x8C => sty_8c(state),
        0x94 => sty_94(state),
        0x64 => stz_64(state),
        0x74 => stz_74(state),
        0x9C => stz_9c(state),
        0x9E => stz_9e(state),
        0xAA => tax_aa(state),
        0xA8 => tay_a8(state),
        0x5B => tcd_5b(state),
        0x1B => tcs_1b(state),
        0x7B => tdc_7b(state),
        0x14 => trb_14(state),
        0x1C => trb_1c(state),
        0x04 => tsb_04(state),
        0x0C => tsb_0c(state),
        0x3B => tsc_3b(state),
        0xBA => tsx_ba(state),
        0x8A => txa_8a(state),
        0x9A => txs_9a(state),
        0x9B => txy_9b(state),
        0x98 => tya_98(state),
        0xBB => tyx_bb(state),
        0xCB => wai_cb(state),
        0x42 => wdm_42(state),
        0xEB => xba_eb(state),
        0xFB => xce_fb(state),
    }
}
