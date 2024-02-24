//! All CPU instructions are implemented here
//!
//! Many of the instructions are similar with different address modes, and can operate
//! on 8 or 16 bit values depending on status flag settins. Therefor I decided to generate
//! as many as I could automatically. The generated instructions use their respective address
//! modes to aquire a target address. And they determine the param size based on the flags set
//! in the [`State`].

use crate::addr;
use crate::cpu;
use crate::snes_address::SnesAddress;
use crate::state::State;

// ============================================================================================================= //
// Op code implementations
// Implement these manually

pub fn bcc_90(state: &mut State) {
    let address = addr::pc_relative(state);
    if !state.p.is_set_c() {
        state.pc = address;
        cpu::add_cycles(state, 1);
    }
}

pub fn bcs_b0(state: &mut State) {
    let address = addr::pc_relative(state);
    if state.p.is_set_c() {
        state.pc = address;
        cpu::add_cycles(state, 1);
    }
}

pub fn beq_f0(state: &mut State) {
    let address = addr::pc_relative(state);
    if state.p.is_set_z() {
        state.pc = address;
        cpu::add_cycles(state, 1);
    }
}

pub fn bmi_30(state: &mut State) {
    let address = addr::pc_relative(state);
    if state.p.is_set_n() {
        state.pc = address;
        cpu::add_cycles(state, 1);
    }
}

pub fn bne_d0(state: &mut State) {
    let address = addr::pc_relative(state);
    if !state.p.is_set_z() {
        state.pc = address;
        cpu::add_cycles(state, 1);
    }
}

pub fn bpl_10(state: &mut State) {
    let address = addr::pc_relative_long(state);
    if !state.p.is_set_n() {
        state.pc = address;
        cpu::add_cycles(state, 1);
    }
}

pub fn bra_80(state: &mut State) {
    let address = addr::pc_relative(state);
    state.pc = address;
    cpu::add_cycles(state, 1);
}

pub fn brk_00(state: &mut State) {
    // TODO: implement brk_00
}

pub fn brl_82(state: &mut State) {
    let address = addr::pc_relative_long(state);
    state.pc = address;
    cpu::add_cycles(state, 1);
}

pub fn bvc_50(state: &mut State) {
    let address = addr::pc_relative(state);
    if !state.p.is_set_v() {
        state.pc = address;
        cpu::add_cycles(state, 1);
    }
}

pub fn bvs_70(state: &mut State) {
    let address = addr::pc_relative(state);
    if state.p.is_set_v() {
        state.pc = address;
        cpu::add_cycles(state, 1);
    }
}

pub fn clc_18(state: &mut State) {
    state.p.clear_c();
    cpu::add_cycles(state, 1);
}

pub fn cld_d8(state: &mut State) {
    state.p.clear_d();
    cpu::add_cycles(state, 1);
}

pub fn cli_58(state: &mut State) {
    state.p.clear_i();
    cpu::add_cycles(state, 1);
}

pub fn clv_b8(state: &mut State) {
    state.p.clear_v();
    cpu::add_cycles(state, 1);
}

pub fn cop_02(state: &mut State) {
    // TODO: implement cop_02
}

pub fn jmp_4c(state: &mut State) {
    let result = addr::absolute(state);
    state.pc = result;
}

pub fn jmp_5c(state: &mut State) {
    let result = addr::absolute_long(state);
    state.pc = result;
}

pub fn jmp_6c(state: &mut State) {
    let result = addr::absolute_indirect(state);
    state.pc = result;
}

pub fn jmp_7c(state: &mut State) {
    let address = addr::absolute_indexed_x(state);
    let index = state.x.get_16();
    let result = address + index;
    state.pc = result;
}

pub fn jmp_dc(state: &mut State) {
    todo!();
}

pub fn jsr_20(state: &mut State) {
    let new_addr = cpu::fetch_word(state);
    let curr_addr = state.pc.get_address();
    cpu::push_word(state, curr_addr);
    state.pc.set_address(new_addr);
}

pub fn jsr_22(state: &mut State) {
    let new_addr = addr::absolute_long(state);
    let curr_addr = state.pc.get_address();
    cpu::push_word(state, curr_addr);
    state.pc = new_addr;
}

pub fn jsr_fc(state: &mut State) {
    todo!();
}

pub fn mvn_54(state: &mut State) {
    todo!();
}

pub fn mvp_44(state: &mut State) {
    todo!();
}

pub fn nop_ea(state: &mut State) {
    todo!();
}

pub fn pea_f4(state: &mut State) {
    let arg = cpu::fetch_word(state);
    cpu::push_word(state, arg);
}

pub fn pei_d4(state: &mut State) {
    let address = addr::direct_indirect(state);
    let arg = cpu::read_word(state, &address);
    cpu::push_word(state, arg);
}

pub fn per_62(state: &mut State) {
    let address = addr::pc_relative(state);
    let arg = cpu::read_word(state, &address);
    cpu::push_word(state, arg);
}

pub fn pha_48(state: &mut State) {
    if state.p.is_set_m() {
    } else {
        let arg = state.a.get_16();
        cpu::push_word(state, arg);
    }
}

pub fn phb_8b(state: &mut State) {
    let arg = state.dbr;
    cpu::push_byte(state, arg);
}

pub fn phd_0b(state: &mut State) {
    let arg = state.d;
    cpu::push_word(state, arg);
}

pub fn phk_4b(state: &mut State) {
    let arg = state.get_pbr();
    cpu::push_byte(state, arg);
}

pub fn php_08(state: &mut State) {
    let arg = state.p.bits();
    cpu::push_byte(state, arg);
}

pub fn phx_da(state: &mut State) {
    if state.p.is_set_x() {
        todo!();
    } else {
        let arg = state.x.get_16();
        cpu::push_word(state, arg);
    }
}

pub fn phy_5a(state: &mut State) {
    if state.p.is_set_x() {
        todo!();
    } else {
        let arg = state.y.get_16();
        cpu::push_word(state, arg);
    }
}

pub fn pla_68(state: &mut State) {
    if state.p.is_set_m() {
        todo!();
    } else {
        let result = cpu::pop_word(state);
        state.a.set_16(result);
    }
}

pub fn plb_ab(state: &mut State) {
    let result = cpu::pop_byte(state);
    state.dbr = result;
}

pub fn pld_2b(state: &mut State) {
    let result = cpu::pop_word(state);
    state.d = result;
}

pub fn plp_28(state: &mut State) {
    let result = cpu::pop_byte(state);
    state.p.set_status_bits(result);
}

pub fn plx_fa(state: &mut State) {
    if state.p.is_set_x() {
        todo!();
    } else {
        let result = cpu::pop_word(state);
        state.x.set_16(result);
    }
}

pub fn ply_7a(state: &mut State) {
    if state.p.is_set_x() {
        todo!();
    } else {
        let result = cpu::pop_word(state);
        state.y.set_16(result);
    }
}

pub fn rep_c2(state: &mut State) {
    let arg = cpu::fetch_byte(state);
    state.p.clear_status_bits(arg);
}

pub fn rti_40(state: &mut State) {
    todo!();
}

pub fn rtl_6b(state: &mut State) {
    todo!();
}

pub fn rts_60(state: &mut State) {
    todo!();
}

pub fn sec_38(state: &mut State) {
    todo!();
}

pub fn sed_f8(state: &mut State) {
    todo!();
}

pub fn sei_78(state: &mut State) {
    todo!();
}

pub fn sep_e2(state: &mut State) {
    todo!();
}

pub fn stp_db(state: &mut State) {
    todo!();
}

pub fn wai_cb(state: &mut State) {
    todo!();
}

pub fn wdm_42(state: &mut State) {
    todo!();
}

pub fn xba_eb(state: &mut State) {
    todo!();
}

pub fn xce_fb(state: &mut State) {
    todo!();
}

// ============================================================================================================= //
// These are generated functions, do not change them manually

pub fn adc_61(state: &mut State) {
    let address = addr::direct_indexed_indirect(state);
    if state.p.is_set_m() {
        // adc8(state, &address);
    } else {
        adc16(state, &address);
    }
}

pub fn adc_63(state: &mut State) {
    let address = addr::stack_relative(state);
    if state.p.is_set_m() {
        // adc8(state, &address);
    } else {
        adc16(state, &address);
    }
}

pub fn adc_65(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_m() {
        // adc8(state, &address);
    } else {
        adc16(state, &address);
    }
}

pub fn adc_67(state: &mut State) {
    let address = addr::direct_indirect_long(state);
    if state.p.is_set_m() {
        // adc8(state, &address);
    } else {
        adc16(state, &address);
    }
}

pub fn adc_69(state: &mut State) {
    let address = addr::immediate16(state);
    if state.p.is_set_m() {
        // adc8(state, &address);
    } else {
        adc16(state, &address);
    }
}

pub fn adc_6d(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_m() {
        // adc8(state, &address);
    } else {
        adc16(state, &address);
    }
}

pub fn adc_6f(state: &mut State) {
    let address = addr::absolute_long(state);
    if state.p.is_set_m() {
        // adc8(state, &address);
    } else {
        adc16(state, &address);
    }
}

pub fn adc_71(state: &mut State) {
    let address = addr::direct_indirect_indexed(state);
    if state.p.is_set_m() {
        // adc8(state, &address);
    } else {
        adc16(state, &address);
    }
}

pub fn adc_72(state: &mut State) {
    let address = addr::direct_indirect(state);
    if state.p.is_set_m() {
        // adc8(state, &address);
    } else {
        adc16(state, &address);
    }
}

pub fn adc_73(state: &mut State) {
    let address = SnesAddress::from((0, 0)); // TODO: define address mode SR Indirect Indexed,Y;
    if state.p.is_set_m() {
        // adc8(state, &address);
    } else {
        adc16(state, &address);
    }
}

pub fn adc_75(state: &mut State) {
    let address = addr::direct_indexed_x(state);
    if state.p.is_set_m() {
        // adc8(state, &address);
    } else {
        adc16(state, &address);
    }
}

pub fn adc_77(state: &mut State) {
    let address = SnesAddress::from((0, 0)); // TODO: define address mode DP Indirect Long Indexed, Y;
    if state.p.is_set_m() {
        // adc8(state, &address);
    } else {
        adc16(state, &address);
    }
}

pub fn adc_79(state: &mut State) {
    let address = addr::absolute_indexed_y(state);
    if state.p.is_set_m() {
        // adc8(state, &address);
    } else {
        adc16(state, &address);
    }
}

pub fn adc_7d(state: &mut State) {
    let address = addr::absolute_indexed_x(state);
    if state.p.is_set_m() {
        // adc8(state, &address);
    } else {
        adc16(state, &address);
    }
}

pub fn adc_7f(state: &mut State) {
    let address = addr::absolute_long_indexed(state);
    if state.p.is_set_m() {
        // adc8(state, &address);
    } else {
        adc16(state, &address);
    }
}

pub fn and_21(state: &mut State) {
    let address = addr::direct_indexed_indirect(state);
    if state.p.is_set_m() {
        // and8(state, &address);
    } else {
        and16(state, &address);
    }
}

pub fn and_23(state: &mut State) {
    let address = addr::stack_relative(state);
    if state.p.is_set_m() {
        // and8(state, &address);
    } else {
        and16(state, &address);
    }
}

pub fn and_25(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_m() {
        // and8(state, &address);
    } else {
        and16(state, &address);
    }
}

pub fn and_27(state: &mut State) {
    let address = addr::direct_indirect_long(state);
    if state.p.is_set_m() {
        // and8(state, &address);
    } else {
        and16(state, &address);
    }
}

pub fn and_29(state: &mut State) {
    let address = addr::immediate16(state);
    if state.p.is_set_m() {
        // and8(state, &address);
    } else {
        and16(state, &address);
    }
}

pub fn and_2d(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_m() {
        // and8(state, &address);
    } else {
        and16(state, &address);
    }
}

pub fn and_2f(state: &mut State) {
    let address = addr::absolute_long(state);
    if state.p.is_set_m() {
        // and8(state, &address);
    } else {
        and16(state, &address);
    }
}

pub fn and_31(state: &mut State) {
    let address = addr::direct_indirect_indexed(state);
    if state.p.is_set_m() {
        // and8(state, &address);
    } else {
        and16(state, &address);
    }
}

pub fn and_32(state: &mut State) {
    let address = addr::direct_indirect(state);
    if state.p.is_set_m() {
        // and8(state, &address);
    } else {
        and16(state, &address);
    }
}

pub fn and_33(state: &mut State) {
    let address = SnesAddress::from((0, 0)); // TODO: define address mode SR Indirect Indexed,Y;
    if state.p.is_set_m() {
        // and8(state, &address);
    } else {
        and16(state, &address);
    }
}

pub fn and_35(state: &mut State) {
    let address = addr::direct_indexed_x(state);
    if state.p.is_set_m() {
        // and8(state, &address);
    } else {
        and16(state, &address);
    }
}

pub fn and_37(state: &mut State) {
    let address = SnesAddress::from((0, 0)); // TODO: define address mode DP Indirect Long Indexed, Y;
    if state.p.is_set_m() {
        // and8(state, &address);
    } else {
        and16(state, &address);
    }
}

pub fn and_39(state: &mut State) {
    let address = addr::absolute_indexed_y(state);
    if state.p.is_set_m() {
        // and8(state, &address);
    } else {
        and16(state, &address);
    }
}

pub fn and_3d(state: &mut State) {
    let address = addr::absolute_indexed_x(state);
    if state.p.is_set_m() {
        // and8(state, &address);
    } else {
        and16(state, &address);
    }
}

pub fn and_3f(state: &mut State) {
    let address = addr::absolute_long_indexed(state);
    if state.p.is_set_m() {
        // and8(state, &address);
    } else {
        and16(state, &address);
    }
}

pub fn asl_06(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_m() {
        // asl_mem8(state, &address);
    } else {
        asl_mem16(state, &address);
    }
}

pub fn asl_0a(state: &mut State) {
    if state.p.is_set_m() {
        // asl_acc8(state);
    } else {
        asl_acc16(state);
    }
}

pub fn asl_0e(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_m() {
        // asl_mem8(state, &address);
    } else {
        asl_mem16(state, &address);
    }
}

pub fn asl_16(state: &mut State) {
    let address = addr::direct_indexed_x(state);
    if state.p.is_set_m() {
        // asl_mem8(state, &address);
    } else {
        asl_mem16(state, &address);
    }
}

pub fn asl_1e(state: &mut State) {
    let address = addr::absolute_indexed_x(state);
    if state.p.is_set_m() {
        // asl_mem8(state, &address);
    } else {
        asl_mem16(state, &address);
    }
}

pub fn bit_24(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_m() {
        // bit8(state, &address);
    } else {
        bit16(state, &address);
    }
}

pub fn bit_2c(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_m() {
        // bit8(state, &address);
    } else {
        bit16(state, &address);
    }
}

pub fn bit_34(state: &mut State) {
    let address = addr::direct_indexed_x(state);
    if state.p.is_set_m() {
        // bit8(state, &address);
    } else {
        bit16(state, &address);
    }
}

pub fn bit_3c(state: &mut State) {
    let address = addr::absolute_indexed_x(state);
    if state.p.is_set_m() {
        // bit8(state, &address);
    } else {
        bit16(state, &address);
    }
}

pub fn bit_89(state: &mut State) {
    let address = addr::immediate16(state);
    if state.p.is_set_m() {
        // bit8(state, &address);
    } else {
        bit16(state, &address);
    }
}

pub fn cmp_c1(state: &mut State) {
    let address = addr::direct_indexed_indirect(state);
    if state.p.is_set_m() {
        // cmp8(state, &address);
    } else {
        cmp16(state, &address);
    }
}

pub fn cmp_c3(state: &mut State) {
    let address = addr::stack_relative(state);
    if state.p.is_set_m() {
        // cmp8(state, &address);
    } else {
        cmp16(state, &address);
    }
}

pub fn cmp_c5(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_m() {
        // cmp8(state, &address);
    } else {
        cmp16(state, &address);
    }
}

pub fn cmp_c7(state: &mut State) {
    let address = addr::direct_indirect_long(state);
    if state.p.is_set_m() {
        // cmp8(state, &address);
    } else {
        cmp16(state, &address);
    }
}

pub fn cmp_c9(state: &mut State) {
    let address = addr::immediate16(state);
    if state.p.is_set_m() {
        // cmp8(state, &address);
    } else {
        cmp16(state, &address);
    }
}

pub fn cmp_cd(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_m() {
        // cmp8(state, &address);
    } else {
        cmp16(state, &address);
    }
}

pub fn cmp_cf(state: &mut State) {
    let address = addr::absolute_long(state);
    if state.p.is_set_m() {
        // cmp8(state, &address);
    } else {
        cmp16(state, &address);
    }
}

pub fn cmp_d1(state: &mut State) {
    let address = addr::direct_indirect_indexed(state);
    if state.p.is_set_m() {
        // cmp8(state, &address);
    } else {
        cmp16(state, &address);
    }
}

pub fn cmp_d2(state: &mut State) {
    let address = addr::direct_indirect(state);
    if state.p.is_set_m() {
        // cmp8(state, &address);
    } else {
        cmp16(state, &address);
    }
}

pub fn cmp_d3(state: &mut State) {
    let address = SnesAddress::from((0, 0)); // TODO: define address mode SR Indirect Indexed,Y;
    if state.p.is_set_m() {
        // cmp8(state, &address);
    } else {
        cmp16(state, &address);
    }
}

pub fn cmp_d5(state: &mut State) {
    let address = addr::direct_indexed_x(state);
    if state.p.is_set_m() {
        // cmp8(state, &address);
    } else {
        cmp16(state, &address);
    }
}

pub fn cmp_d7(state: &mut State) {
    let address = SnesAddress::from((0, 0)); // TODO: define address mode DP Indirect Long Indexed, Y;
    if state.p.is_set_m() {
        // cmp8(state, &address);
    } else {
        cmp16(state, &address);
    }
}

pub fn cmp_d9(state: &mut State) {
    let address = addr::absolute_indexed_y(state);
    if state.p.is_set_m() {
        // cmp8(state, &address);
    } else {
        cmp16(state, &address);
    }
}

pub fn cmp_dd(state: &mut State) {
    let address = addr::absolute_indexed_x(state);
    if state.p.is_set_m() {
        // cmp8(state, &address);
    } else {
        cmp16(state, &address);
    }
}

pub fn cmp_df(state: &mut State) {
    let address = addr::absolute_long_indexed(state);
    if state.p.is_set_m() {
        // cmp8(state, &address);
    } else {
        cmp16(state, &address);
    }
}

pub fn cpx_e0(state: &mut State) {
    let address = addr::immediate16(state);
    if state.p.is_set_x() {
        // cpx8(state, &address);
    } else {
        cpx16(state, &address);
    }
}

pub fn cpx_e4(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_x() {
        // cpx8(state, &address);
    } else {
        cpx16(state, &address);
    }
}

pub fn cpx_ec(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_x() {
        // cpx8(state, &address);
    } else {
        cpx16(state, &address);
    }
}

pub fn cpy_c0(state: &mut State) {
    let address = addr::immediate16(state);
    if state.p.is_set_x() {
        // cpy8(state, &address);
    } else {
        cpy16(state, &address);
    }
}

pub fn cpy_c4(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_x() {
        // cpy8(state, &address);
    } else {
        cpy16(state, &address);
    }
}

pub fn cpy_cc(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_x() {
        // cpy8(state, &address);
    } else {
        cpy16(state, &address);
    }
}

pub fn dec_3a(state: &mut State) {
    if state.p.is_set_m() {
        // dec_acc8(state);
    } else {
        dec_acc16(state);
    }
}

pub fn dec_c6(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_m() {
        // dec_mem8(state, &address);
    } else {
        dec_mem16(state, &address);
    }
}

pub fn dec_ce(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_m() {
        // dec_mem8(state, &address);
    } else {
        dec_mem16(state, &address);
    }
}

pub fn dec_d6(state: &mut State) {
    let address = addr::direct_indexed_x(state);
    if state.p.is_set_m() {
        // dec_mem8(state, &address);
    } else {
        dec_mem16(state, &address);
    }
}

pub fn dec_de(state: &mut State) {
    let address = addr::absolute_indexed_x(state);
    if state.p.is_set_m() {
        // dec_mem8(state, &address);
    } else {
        dec_mem16(state, &address);
    }
}

pub fn dex_ca(state: &mut State) {
    if state.p.is_set_x() {
        // dex8(state);
    } else {
        dex16(state);
    }
}

pub fn dey_88(state: &mut State) {
    if state.p.is_set_x() {
        // dey8(state);
    } else {
        dey16(state);
    }
}

pub fn eor_41(state: &mut State) {
    let address = addr::direct_indexed_indirect(state);
    if state.p.is_set_m() {
        // eor8(state, &address);
    } else {
        eor16(state, &address);
    }
}

pub fn eor_43(state: &mut State) {
    let address = addr::stack_relative(state);
    if state.p.is_set_m() {
        // eor8(state, &address);
    } else {
        eor16(state, &address);
    }
}

pub fn eor_45(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_m() {
        // eor8(state, &address);
    } else {
        eor16(state, &address);
    }
}

pub fn eor_47(state: &mut State) {
    let address = addr::direct_indirect_long(state);
    if state.p.is_set_m() {
        // eor8(state, &address);
    } else {
        eor16(state, &address);
    }
}

pub fn eor_49(state: &mut State) {
    let address = addr::immediate16(state);
    if state.p.is_set_m() {
        // eor8(state, &address);
    } else {
        eor16(state, &address);
    }
}

pub fn eor_4d(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_m() {
        // eor8(state, &address);
    } else {
        eor16(state, &address);
    }
}

pub fn eor_4f(state: &mut State) {
    let address = addr::absolute_long(state);
    if state.p.is_set_m() {
        // eor8(state, &address);
    } else {
        eor16(state, &address);
    }
}

pub fn eor_51(state: &mut State) {
    let address = addr::direct_indirect_indexed(state);
    if state.p.is_set_m() {
        // eor8(state, &address);
    } else {
        eor16(state, &address);
    }
}

pub fn eor_52(state: &mut State) {
    let address = addr::direct_indirect(state);
    if state.p.is_set_m() {
        // eor8(state, &address);
    } else {
        eor16(state, &address);
    }
}

pub fn eor_53(state: &mut State) {
    let address = SnesAddress::from((0, 0)); // TODO: define address mode SR Indirect Indexed,Y;
    if state.p.is_set_m() {
        // eor8(state, &address);
    } else {
        eor16(state, &address);
    }
}

pub fn eor_55(state: &mut State) {
    let address = addr::direct_indexed_x(state);
    if state.p.is_set_m() {
        // eor8(state, &address);
    } else {
        eor16(state, &address);
    }
}

pub fn eor_57(state: &mut State) {
    let address = SnesAddress::from((0, 0)); // TODO: define address mode DP Indirect Long Indexed, Y;
    if state.p.is_set_m() {
        // eor8(state, &address);
    } else {
        eor16(state, &address);
    }
}

pub fn eor_59(state: &mut State) {
    let address = addr::absolute_indexed_y(state);
    if state.p.is_set_m() {
        // eor8(state, &address);
    } else {
        eor16(state, &address);
    }
}

pub fn eor_5d(state: &mut State) {
    let address = addr::absolute_indexed_x(state);
    if state.p.is_set_m() {
        // eor8(state, &address);
    } else {
        eor16(state, &address);
    }
}

pub fn eor_5f(state: &mut State) {
    let address = addr::absolute_long_indexed(state);
    if state.p.is_set_m() {
        // eor8(state, &address);
    } else {
        eor16(state, &address);
    }
}

pub fn inc_1a(state: &mut State) {
    if state.p.is_set_m() {
        // inc_acc8(state);
    } else {
        inc_acc16(state);
    }
}

pub fn inc_e6(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_m() {
        // inc_mem8(state, &address);
    } else {
        inc_mem16(state, &address);
    }
}

pub fn inc_ee(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_m() {
        // inc_mem8(state, &address);
    } else {
        inc_mem16(state, &address);
    }
}

pub fn inc_f6(state: &mut State) {
    let address = addr::direct_indexed_x(state);
    if state.p.is_set_m() {
        // inc_mem8(state, &address);
    } else {
        inc_mem16(state, &address);
    }
}

pub fn inc_fe(state: &mut State) {
    let address = addr::absolute_indexed_x(state);
    if state.p.is_set_m() {
        // inc_mem8(state, &address);
    } else {
        inc_mem16(state, &address);
    }
}

pub fn inx_e8(state: &mut State) {
    if state.p.is_set_x() {
        // inx8(state);
    } else {
        inx16(state);
    }
}

pub fn iny_c8(state: &mut State) {
    if state.p.is_set_x() {
        // iny8(state);
    } else {
        iny16(state);
    }
}

pub fn lda_a1(state: &mut State) {
    let address = addr::direct_indexed_indirect(state);
    if state.p.is_set_m() {
        // lda8(state, &address);
    } else {
        lda16(state, &address);
    }
}

pub fn lda_a3(state: &mut State) {
    let address = addr::stack_relative(state);
    if state.p.is_set_m() {
        // lda8(state, &address);
    } else {
        lda16(state, &address);
    }
}

pub fn lda_a5(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_m() {
        // lda8(state, &address);
    } else {
        lda16(state, &address);
    }
}

pub fn lda_a7(state: &mut State) {
    let address = addr::direct_indirect_long(state);
    if state.p.is_set_m() {
        // lda8(state, &address);
    } else {
        lda16(state, &address);
    }
}

pub fn lda_a9(state: &mut State) {
    let address = addr::immediate16(state);
    if state.p.is_set_m() {
        // lda8(state, &address);
    } else {
        lda16(state, &address);
    }
}

pub fn lda_ad(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_m() {
        // lda8(state, &address);
    } else {
        lda16(state, &address);
    }
}

pub fn lda_af(state: &mut State) {
    let address = addr::absolute_long(state);
    if state.p.is_set_m() {
        // lda8(state, &address);
    } else {
        lda16(state, &address);
    }
}

pub fn lda_b1(state: &mut State) {
    let address = addr::direct_indirect_indexed(state);
    if state.p.is_set_m() {
        // lda8(state, &address);
    } else {
        lda16(state, &address);
    }
}

pub fn lda_b2(state: &mut State) {
    let address = addr::direct_indirect(state);
    if state.p.is_set_m() {
        // lda8(state, &address);
    } else {
        lda16(state, &address);
    }
}

pub fn lda_b3(state: &mut State) {
    let address = SnesAddress::from((0, 0)); // TODO: define address mode SR Indirect Indexed,Y;
    if state.p.is_set_m() {
        // lda8(state, &address);
    } else {
        lda16(state, &address);
    }
}

pub fn lda_b5(state: &mut State) {
    let address = addr::direct_indexed_x(state);
    if state.p.is_set_m() {
        // lda8(state, &address);
    } else {
        lda16(state, &address);
    }
}

pub fn lda_b7(state: &mut State) {
    let address = SnesAddress::from((0, 0)); // TODO: define address mode DP Indirect Long Indexed, Y;
    if state.p.is_set_m() {
        // lda8(state, &address);
    } else {
        lda16(state, &address);
    }
}

pub fn lda_b9(state: &mut State) {
    let address = addr::absolute_indexed_y(state);
    if state.p.is_set_m() {
        // lda8(state, &address);
    } else {
        lda16(state, &address);
    }
}

pub fn lda_bd(state: &mut State) {
    let address = addr::absolute_indexed_x(state);
    if state.p.is_set_m() {
        // lda8(state, &address);
    } else {
        lda16(state, &address);
    }
}

pub fn lda_bf(state: &mut State) {
    let address = addr::absolute_long_indexed(state);
    if state.p.is_set_m() {
        // lda8(state, &address);
    } else {
        lda16(state, &address);
    }
}

pub fn ldx_a2(state: &mut State) {
    let address = addr::immediate16(state);
    if state.p.is_set_x() {
        // ldx8(state, &address);
    } else {
        ldx16(state, &address);
    }
}

pub fn ldx_a6(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_x() {
        // ldx8(state, &address);
    } else {
        ldx16(state, &address);
    }
}

pub fn ldx_ae(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_x() {
        // ldx8(state, &address);
    } else {
        ldx16(state, &address);
    }
}

pub fn ldx_b6(state: &mut State) {
    let address = addr::direct_indexed_y(state);
    if state.p.is_set_x() {
        // ldx8(state, &address);
    } else {
        ldx16(state, &address);
    }
}

pub fn ldx_be(state: &mut State) {
    let address = addr::absolute_indexed_y(state);
    if state.p.is_set_x() {
        // ldx8(state, &address);
    } else {
        ldx16(state, &address);
    }
}

pub fn ldy_a0(state: &mut State) {
    let address = addr::immediate16(state);
    if state.p.is_set_x() {
        // ldy8(state, &address);
    } else {
        ldy16(state, &address);
    }
}

pub fn ldy_a4(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_x() {
        // ldy8(state, &address);
    } else {
        ldy16(state, &address);
    }
}

pub fn ldy_ac(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_x() {
        // ldy8(state, &address);
    } else {
        ldy16(state, &address);
    }
}

pub fn ldy_b4(state: &mut State) {
    let address = addr::direct_indexed_x(state);
    if state.p.is_set_x() {
        // ldy8(state, &address);
    } else {
        ldy16(state, &address);
    }
}

pub fn ldy_bc(state: &mut State) {
    let address = addr::absolute_indexed_x(state);
    if state.p.is_set_x() {
        // ldy8(state, &address);
    } else {
        ldy16(state, &address);
    }
}

pub fn lsr_46(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_m() {
        // lsr_mem8(state, &address);
    } else {
        lsr_mem16(state, &address);
    }
}

pub fn lsr_4a(state: &mut State) {
    if state.p.is_set_m() {
        // lsr_acc8(state);
    } else {
        lsr_acc16(state);
    }
}

pub fn lsr_4e(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_m() {
        // lsr_mem8(state, &address);
    } else {
        lsr_mem16(state, &address);
    }
}

pub fn lsr_56(state: &mut State) {
    let address = addr::direct_indexed_x(state);
    if state.p.is_set_m() {
        // lsr_mem8(state, &address);
    } else {
        lsr_mem16(state, &address);
    }
}

pub fn lsr_5e(state: &mut State) {
    let address = addr::absolute_indexed_x(state);
    if state.p.is_set_m() {
        // lsr_mem8(state, &address);
    } else {
        lsr_mem16(state, &address);
    }
}

pub fn ora_01(state: &mut State) {
    let address = addr::direct_indexed_indirect(state);
    if state.p.is_set_m() {
        // ora8(state, &address);
    } else {
        ora16(state, &address);
    }
}

pub fn ora_03(state: &mut State) {
    let address = addr::stack_relative(state);
    if state.p.is_set_m() {
        // ora8(state, &address);
    } else {
        ora16(state, &address);
    }
}

pub fn ora_05(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_m() {
        // ora8(state, &address);
    } else {
        ora16(state, &address);
    }
}

pub fn ora_07(state: &mut State) {
    let address = addr::direct_indirect_long(state);
    if state.p.is_set_m() {
        // ora8(state, &address);
    } else {
        ora16(state, &address);
    }
}

pub fn ora_09(state: &mut State) {
    let address = addr::immediate16(state);
    if state.p.is_set_m() {
        // ora8(state, &address);
    } else {
        ora16(state, &address);
    }
}

pub fn ora_0d(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_m() {
        // ora8(state, &address);
    } else {
        ora16(state, &address);
    }
}

pub fn ora_0f(state: &mut State) {
    let address = addr::absolute_long(state);
    if state.p.is_set_m() {
        // ora8(state, &address);
    } else {
        ora16(state, &address);
    }
}

pub fn ora_11(state: &mut State) {
    let address = addr::direct_indirect_indexed(state);
    if state.p.is_set_m() {
        // ora8(state, &address);
    } else {
        ora16(state, &address);
    }
}

pub fn ora_12(state: &mut State) {
    let address = addr::direct_indirect(state);
    if state.p.is_set_m() {
        // ora8(state, &address);
    } else {
        ora16(state, &address);
    }
}

pub fn ora_13(state: &mut State) {
    let address = SnesAddress::from((0, 0)); // TODO: define address mode SR Indirect Indexed,Y;
    if state.p.is_set_m() {
        // ora8(state, &address);
    } else {
        ora16(state, &address);
    }
}

pub fn ora_15(state: &mut State) {
    let address = addr::direct_indexed_x(state);
    if state.p.is_set_m() {
        // ora8(state, &address);
    } else {
        ora16(state, &address);
    }
}

pub fn ora_17(state: &mut State) {
    let address = SnesAddress::from((0, 0)); // TODO: define address mode DP Indirect Long Indexed, Y;
    if state.p.is_set_m() {
        // ora8(state, &address);
    } else {
        ora16(state, &address);
    }
}

pub fn ora_19(state: &mut State) {
    let address = addr::absolute_indexed_y(state);
    if state.p.is_set_m() {
        // ora8(state, &address);
    } else {
        ora16(state, &address);
    }
}

pub fn ora_1d(state: &mut State) {
    let address = addr::absolute_indexed_x(state);
    if state.p.is_set_m() {
        // ora8(state, &address);
    } else {
        ora16(state, &address);
    }
}

pub fn ora_1f(state: &mut State) {
    let address = addr::absolute_long_indexed(state);
    if state.p.is_set_m() {
        // ora8(state, &address);
    } else {
        ora16(state, &address);
    }
}

pub fn rol_26(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_m() {
        // rol_mem8(state, &address);
    } else {
        rol_mem16(state, &address);
    }
}

pub fn rol_2a(state: &mut State) {
    if state.p.is_set_m() {
        // rol_acc8(state);
    } else {
        rol_acc16(state);
    }
}

pub fn rol_2e(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_m() {
        // rol_mem8(state, &address);
    } else {
        rol_mem16(state, &address);
    }
}

pub fn rol_36(state: &mut State) {
    let address = addr::direct_indexed_x(state);
    if state.p.is_set_m() {
        // rol_mem8(state, &address);
    } else {
        rol_mem16(state, &address);
    }
}

pub fn rol_3e(state: &mut State) {
    let address = addr::absolute_indexed_x(state);
    if state.p.is_set_m() {
        // rol_mem8(state, &address);
    } else {
        rol_mem16(state, &address);
    }
}

pub fn ror_66(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_m() {
        // ror_mem8(state, &address);
    } else {
        ror_mem16(state, &address);
    }
}

pub fn ror_6a(state: &mut State) {
    if state.p.is_set_m() {
        // ror_acc8(state);
    } else {
        ror_acc16(state);
    }
}

pub fn ror_6e(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_m() {
        // ror_mem8(state, &address);
    } else {
        ror_mem16(state, &address);
    }
}

pub fn ror_76(state: &mut State) {
    let address = addr::direct_indexed_x(state);
    if state.p.is_set_m() {
        // ror_mem8(state, &address);
    } else {
        ror_mem16(state, &address);
    }
}

pub fn ror_7e(state: &mut State) {
    let address = addr::absolute_indexed_x(state);
    if state.p.is_set_m() {
        // ror_mem8(state, &address);
    } else {
        ror_mem16(state, &address);
    }
}

pub fn sbc_e1(state: &mut State) {
    let address = addr::direct_indexed_indirect(state);
    if state.p.is_set_m() {
        // sbc8(state, &address);
    } else {
        sbc16(state, &address);
    }
}

pub fn sbc_e3(state: &mut State) {
    let address = addr::stack_relative(state);
    if state.p.is_set_m() {
        // sbc8(state, &address);
    } else {
        sbc16(state, &address);
    }
}

pub fn sbc_e5(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_m() {
        // sbc8(state, &address);
    } else {
        sbc16(state, &address);
    }
}

pub fn sbc_e7(state: &mut State) {
    let address = addr::direct_indirect_long(state);
    if state.p.is_set_m() {
        // sbc8(state, &address);
    } else {
        sbc16(state, &address);
    }
}

pub fn sbc_e9(state: &mut State) {
    let address = addr::immediate16(state);
    if state.p.is_set_m() {
        // sbc8(state, &address);
    } else {
        sbc16(state, &address);
    }
}

pub fn sbc_ed(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_m() {
        // sbc8(state, &address);
    } else {
        sbc16(state, &address);
    }
}

pub fn sbc_ef(state: &mut State) {
    let address = addr::absolute_long(state);
    if state.p.is_set_m() {
        // sbc8(state, &address);
    } else {
        sbc16(state, &address);
    }
}

pub fn sbc_f1(state: &mut State) {
    let address = addr::direct_indirect_indexed(state);
    if state.p.is_set_m() {
        // sbc8(state, &address);
    } else {
        sbc16(state, &address);
    }
}

pub fn sbc_f2(state: &mut State) {
    let address = addr::direct_indirect(state);
    if state.p.is_set_m() {
        // sbc8(state, &address);
    } else {
        sbc16(state, &address);
    }
}

pub fn sbc_f3(state: &mut State) {
    let address = SnesAddress::from((0, 0)); // TODO: define address mode SR Indirect Indexed,Y;
    if state.p.is_set_m() {
        // sbc8(state, &address);
    } else {
        sbc16(state, &address);
    }
}

pub fn sbc_f5(state: &mut State) {
    let address = addr::direct_indexed_x(state);
    if state.p.is_set_m() {
        // sbc8(state, &address);
    } else {
        sbc16(state, &address);
    }
}

pub fn sbc_f7(state: &mut State) {
    let address = SnesAddress::from((0, 0)); // TODO: define address mode DP Indirect Long Indexed, Y;
    if state.p.is_set_m() {
        // sbc8(state, &address);
    } else {
        sbc16(state, &address);
    }
}

pub fn sbc_f9(state: &mut State) {
    let address = addr::absolute_indexed_y(state);
    if state.p.is_set_m() {
        // sbc8(state, &address);
    } else {
        sbc16(state, &address);
    }
}

pub fn sbc_fd(state: &mut State) {
    let address = addr::absolute_indexed_x(state);
    if state.p.is_set_m() {
        // sbc8(state, &address);
    } else {
        sbc16(state, &address);
    }
}

pub fn sbc_ff(state: &mut State) {
    let address = addr::absolute_long_indexed(state);
    if state.p.is_set_m() {
        // sbc8(state, &address);
    } else {
        sbc16(state, &address);
    }
}

pub fn sta_81(state: &mut State) {
    let address = addr::direct_indexed_indirect(state);
    if state.p.is_set_m() {
        // sta8(state, &address);
    } else {
        sta16(state, &address);
    }
}

pub fn sta_83(state: &mut State) {
    let address = addr::stack_relative(state);
    if state.p.is_set_m() {
        // sta8(state, &address);
    } else {
        sta16(state, &address);
    }
}

pub fn sta_85(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_m() {
        // sta8(state, &address);
    } else {
        sta16(state, &address);
    }
}

pub fn sta_87(state: &mut State) {
    let address = addr::direct_indirect_long(state);
    if state.p.is_set_m() {
        // sta8(state, &address);
    } else {
        sta16(state, &address);
    }
}

pub fn sta_8d(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_m() {
        // sta8(state, &address);
    } else {
        sta16(state, &address);
    }
}

pub fn sta_8f(state: &mut State) {
    let address = addr::absolute_long(state);
    if state.p.is_set_m() {
        // sta8(state, &address);
    } else {
        sta16(state, &address);
    }
}

pub fn sta_91(state: &mut State) {
    let address = addr::direct_indirect_indexed(state);
    if state.p.is_set_m() {
        // sta8(state, &address);
    } else {
        sta16(state, &address);
    }
}

pub fn sta_92(state: &mut State) {
    let address = addr::direct_indirect(state);
    if state.p.is_set_m() {
        // sta8(state, &address);
    } else {
        sta16(state, &address);
    }
}

pub fn sta_93(state: &mut State) {
    let address = SnesAddress::from((0, 0)); // TODO: define address mode SR Indirect Indexed,Y;
    if state.p.is_set_m() {
        // sta8(state, &address);
    } else {
        sta16(state, &address);
    }
}

pub fn sta_95(state: &mut State) {
    let address = addr::direct_indexed_x(state);
    if state.p.is_set_m() {
        // sta8(state, &address);
    } else {
        sta16(state, &address);
    }
}

pub fn sta_97(state: &mut State) {
    let address = SnesAddress::from((0, 0)); // TODO: define address mode DP Indirect Long Indexed, Y;
    if state.p.is_set_m() {
        // sta8(state, &address);
    } else {
        sta16(state, &address);
    }
}

pub fn sta_99(state: &mut State) {
    let address = addr::absolute_indexed_y(state);
    if state.p.is_set_m() {
        // sta8(state, &address);
    } else {
        sta16(state, &address);
    }
}

pub fn sta_9d(state: &mut State) {
    let address = addr::absolute_indexed_x(state);
    if state.p.is_set_m() {
        // sta8(state, &address);
    } else {
        sta16(state, &address);
    }
}

pub fn sta_9f(state: &mut State) {
    let address = addr::absolute_long_indexed(state);
    if state.p.is_set_m() {
        // sta8(state, &address);
    } else {
        sta16(state, &address);
    }
}

pub fn stx_86(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_x() {
        // stx8(state, &address);
    } else {
        stx16(state, &address);
    }
}

pub fn stx_8e(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_x() {
        // stx8(state, &address);
    } else {
        stx16(state, &address);
    }
}

pub fn stx_96(state: &mut State) {
    let address = addr::direct_indexed_y(state);
    if state.p.is_set_x() {
        // stx8(state, &address);
    } else {
        stx16(state, &address);
    }
}

pub fn sty_84(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_x() {
        // sty8(state, &address);
    } else {
        sty16(state, &address);
    }
}

pub fn sty_8c(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_x() {
        // sty8(state, &address);
    } else {
        sty16(state, &address);
    }
}

pub fn sty_94(state: &mut State) {
    let address = addr::direct_indexed_x(state);
    if state.p.is_set_x() {
        // sty8(state, &address);
    } else {
        sty16(state, &address);
    }
}

pub fn stz_64(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_m() {
        // stz8(state, &address);
    } else {
        stz16(state, &address);
    }
}

pub fn stz_74(state: &mut State) {
    let address = addr::direct_indexed_x(state);
    if state.p.is_set_m() {
        // stz8(state, &address);
    } else {
        stz16(state, &address);
    }
}

pub fn stz_9c(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_m() {
        // stz8(state, &address);
    } else {
        stz16(state, &address);
    }
}

pub fn stz_9e(state: &mut State) {
    let address = addr::absolute_indexed_x(state);
    if state.p.is_set_m() {
        // stz8(state, &address);
    } else {
        stz16(state, &address);
    }
}

pub fn tax_aa(state: &mut State) {
    if state.p.is_set_m() {
        // tax8(state);
    } else {
        tax16(state);
    }
}

pub fn tay_a8(state: &mut State) {
    if state.p.is_set_m() {
        // tay8(state);
    } else {
        tay16(state);
    }
}

pub fn tcd_5b(state: &mut State) {
    if state.p.is_set_m() {
        // tcd8(state);
    } else {
        tcd16(state);
    }
}

pub fn tcs_1b(state: &mut State) {
    if state.p.is_set_m() {
        // tcs8(state);
    } else {
        tcs16(state);
    }
}

pub fn tdc_7b(state: &mut State) {
    if state.p.is_set_m() {
        // tdc8(state);
    } else {
        tdc16(state);
    }
}

pub fn trb_14(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_m() {
        // trb8(state, &address);
    } else {
        trb16(state, &address);
    }
}

pub fn trb_1c(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_m() {
        // trb8(state, &address);
    } else {
        trb16(state, &address);
    }
}

pub fn tsb_04(state: &mut State) {
    let address = addr::direct(state);
    if state.p.is_set_m() {
        // tsb8(state, &address);
    } else {
        tsb16(state, &address);
    }
}

pub fn tsb_0c(state: &mut State) {
    let address = addr::absolute(state);
    if state.p.is_set_m() {
        // tsb8(state, &address);
    } else {
        tsb16(state, &address);
    }
}

pub fn tsc_3b(state: &mut State) {
    if state.p.is_set_m() {
        // tsc8(state);
    } else {
        tsc16(state);
    }
}

pub fn tsx_ba(state: &mut State) {
    if state.p.is_set_m() {
        // tsx8(state);
    } else {
        tsx16(state);
    }
}

pub fn txa_8a(state: &mut State) {
    if state.p.is_set_m() {
        // txa8(state);
    } else {
        txa16(state);
    }
}

pub fn txs_9a(state: &mut State) {
    if state.p.is_set_m() {
        // txs8(state);
    } else {
        txs16(state);
    }
}

pub fn txy_9b(state: &mut State) {
    if state.p.is_set_m() {
        // txy8(state);
    } else {
        txy16(state);
    }
}

pub fn tya_98(state: &mut State) {
    if state.p.is_set_m() {
        // tya8(state);
    } else {
        tya16(state);
    }
}

pub fn tyx_bb(state: &mut State) {
    if state.p.is_set_m() {
        // tyx8(state);
    } else {
        tyx16(state);
    }
}

// ============================================================================================================= //

// Many instructions have the same logic with different address modes.
// Logic is implemented separately from those instructions.

// Helpers

fn set_zero(state: &mut State, arg: u16) {
    let is_zero = arg == 0;
    if is_zero {
        state.p.set_z();
    } else {
        state.p.clear_z();
    }
}

fn set_negative(state: &mut State, arg: u16) {
    let is_negative = (arg as i16) < 0;
    if is_negative {
        state.p.set_n();
    } else {
        state.p.clear_n();
    }
}

fn set_carry(state: &mut State, carry: bool) {
    if carry {
        state.p.set_c();
    } else {
        state.p.clear_c();
    }
}

fn set_overflow(state: &mut State, overflow: bool) {
    if overflow {
        state.p.set_v();
    } else {
        state.p.clear_v();
    }
}

fn set_zn16(state: &mut State, arg: u16) {
    set_zero(state, arg);
    set_negative(state, arg);
}

fn adc16(state: &mut State, address: &SnesAddress) {
    let value = state.a.get_16();
    let arg = cpu::read_word(state, address);
    let (mut result, mut carry) = value.overflowing_add(arg);

    let is_carry_set = state.p.is_set_c();
    if is_carry_set {
        let (tmp_result, tmp_carry) = result.overflowing_add(1);
        result = tmp_result;
        carry |= tmp_carry;
    }

    let signed_overflow = ((value as i16) < 0 && (arg as i16) < 0 && result > 0)
        || ((value as i16) > 0 && (arg as i16) > 0 && (result as i16) < 0);

    set_overflow(state, signed_overflow);
    set_carry(state, carry);
    set_zn16(state, result);
    state.a.set_16(result);
}

fn sbc16(state: &mut State, address: &SnesAddress) {
    let value = state.a.get_16();
    let arg = cpu::read_word(state, address);
    let (mut result, mut carry) = value.overflowing_sub(arg);

    let is_carry_set = state.p.is_set_c();
    if is_carry_set {
        let (tmp_result, tmp_carry) = result.overflowing_sub(1);
        result = tmp_result;
        carry |= tmp_carry;
    }

    let signed_overflow = ((value as i16) < 0 && (arg as i16) > 0 && result > 0)
        || ((value as i16) > 0 && (arg as i16) < 0 && (result as i16) < 0);

    set_overflow(state, signed_overflow);
    set_carry(state, carry);
    set_zn16(state, result);
    state.a.set_16(result);
}

fn and16(state: &mut State, address: &SnesAddress) {
    let value = state.a.get_16();
    let arg = cpu::read_word(state, address);
    let result = value & arg;
    set_zn16(state, result);
    state.a.set_16(value);
}

fn eor16(state: &mut State, address: &SnesAddress) {
    let value = state.a.get_16();
    let arg = cpu::read_word(state, address);
    let result = value ^ arg;
    set_zn16(state, result);
    state.a.set_16(result);
}

fn ora16(state: &mut State, address: &SnesAddress) {
    let value = state.a.get_16();
    let arg = cpu::read_word(state, address);
    let result = value | arg;
    set_zn16(state, result);
    state.a.set_16(result);
}

fn tsb16(state: &mut State, address: &SnesAddress) {
    let value = state.a.get_16();
    let arg = cpu::read_word(state, address);
    let result = value | arg;
    set_zero(state, result);
    cpu::write_word(state, address, result);
}

fn trb16(state: &mut State, address: &SnesAddress) {
    let value = state.a.get_16();
    let arg = cpu::read_word(state, address);
    let result = !value & arg;
    set_zero(state, arg);
    cpu::write_word(state, address, result);
}

// ASL
fn asl16(state: &mut State, value: u16) -> u16 {
    let carry = (value as i16) < 0;

    // If high bit is set, clear it to avoid avoid overflow and set carry flag.
    let result: u16 = if carry {
        (value & 0x7FFF) << 1
    } else {
        value << 1
    };

    set_zn16(state, result);
    set_carry(state, carry);
    return result;
}

fn asl_mem16(state: &mut State, address: &SnesAddress) {
    let value = cpu::read_word(state, address);
    let result = asl16(state, value);
    cpu::write_word(state, address, result);
}

fn asl_acc16(state: &mut State) {
    let value = state.a.get_16();
    let result = asl16(state, value);
    state.a.set_16(result);
}

// LSR
fn lsr16(state: &mut State, value: u16) -> u16 {
    let carry = (value & 1) > 0;
    let result = value >> 1;
    set_zn16(state, result);
    set_carry(state, carry);
    return result;
}

fn lsr_mem16(state: &mut State, address: &SnesAddress) {
    let value = cpu::read_word(state, address);
    let result = lsr16(state, value);
    cpu::write_word(state, address, result);
}

fn lsr_acc16(state: &mut State) {
    let value = state.a.get_16();
    let result = lsr16(state, value);
    state.a.set_16(result);
}

// ROL

fn rol16(state: &mut State, value: u16) -> u16 {
    let is_carry_set = state.p.is_set_c();
    let carry = (value as i16) < 0;

    let mut result = if carry {
        (value & 0x7FFF) << 1
    } else {
        value << 1
    };

    if is_carry_set {
        result += 1;
    }

    set_zn16(state, result);
    set_carry(state, carry);
    return result;
}

fn rol_mem16(state: &mut State, address: &SnesAddress) {
    let value = cpu::read_word(state, address);
    let result = rol16(state, value);
    cpu::write_word(state, address, result);
}

fn rol_acc16(state: &mut State) {
    let value = state.a.get_16();
    let result = rol16(state, value);
    state.a.set_16(result);
}

// ROR

fn ror16(state: &mut State, value: u16) -> u16 {
    let is_carry_set = state.p.is_set_c();
    let carry = (value & 1) > 0;

    let result: u16 = if is_carry_set {
        (value >> 1) | 0x8000
    } else {
        value >> 1
    };

    set_zn16(state, result);
    set_carry(state, carry);

    return result;
}

fn ror_mem16(state: &mut State, address: &SnesAddress) {
    let value = cpu::read_word(state, address);
    let result = ror16(state, value);
    cpu::write_word(state, address, result);
}

fn ror_acc16(state: &mut State) {
    let value = state.a.get_16();
    let result = ror16(state, value);
    state.a.set_16(result);
}

fn bit16(state: &mut State, address: &SnesAddress) {
    let value = state.a.get_16();
    let arg = cpu::read_word(state, address);
    let result = value & arg;
    let is_set_bit_14 = (result & 1 << 13) > 0;
    set_overflow(state, is_set_bit_14);
    set_zn16(state, result);
}

fn compare16(state: &mut State, lhs: u16, rhs: u16) {
    let result = lhs.wrapping_sub(rhs);
    let greater_equal = lhs >= rhs;
    set_carry(state, greater_equal);
    set_zn16(state, result);
}

fn cmp16(state: &mut State, address: &SnesAddress) {
    let lhs = state.a.get_16();
    let rhs = cpu::read_word(state, address);
    compare16(state, lhs, rhs);
}

fn cpx16(state: &mut State, address: &SnesAddress) {
    let lhs = state.x.get_16();
    let rhs = cpu::read_word(state, address);
    compare16(state, lhs, rhs);
}

fn cpy16(state: &mut State, address: &SnesAddress) {
    let lhs = state.y.get_16();
    let rhs = cpu::read_word(state, address);
    compare16(state, lhs, rhs);
}

fn dec16(state: &mut State, value: u16) -> u16 {
    let result = value.wrapping_sub(1);
    set_zn16(state, result);
    return result;
}

fn dec_acc16(state: &mut State) {
    let value = state.a.get_16();
    let result = dec16(state, value);
    state.a.set_16(result);
}

fn dec_mem16(state: &mut State, address: &SnesAddress) {
    let value = cpu::read_word(state, address);
    let result = dec16(state, value);
    cpu::write_word(state, address, result);
}

fn dex16(state: &mut State) {
    let value = state.x.get_16();
    let result = dec16(state, value);
    state.x.set_16(result);
}

fn dey16(state: &mut State) {
    let value = state.y.get_16();
    let result = dec16(state, value);
    state.y.set_16(result);
}

fn inc16(state: &mut State, value: u16) -> u16 {
    let result = value.wrapping_add(1);
    set_zn16(state, value);
    return result;
}

fn inc_acc16(state: &mut State) {
    let value = state.a.get_16();
    let result = inc16(state, value);
    state.a.set_16(result);
}

fn inc_mem16(state: &mut State, address: &SnesAddress) {
    let value = cpu::read_word(state, address);
    let result = inc16(state, value);
    cpu::write_word(state, address, result);
}

fn inx16(state: &mut State) {
    let value = state.x.get_16();
    let result = inc16(state, value);
    state.x.set_16(result);
}

fn iny16(state: &mut State) {
    let value = state.y.get_16();
    let result = inc16(state, value);
    state.y.set_16(result);
}

fn lda16(state: &mut State, address: &SnesAddress) {
    let result = cpu::read_word(state, address);
    set_zn16(state, result);
    state.a.set_16(result);
}

fn ldx16(state: &mut State, address: &SnesAddress) {
    let result = cpu::read_word(state, address);
    set_zn16(state, result);
    state.x.set_16(result);
}

fn ldy16(state: &mut State, address: &SnesAddress) {
    let result = cpu::read_word(state, address);
    set_zn16(state, result);
    state.y.set_16(result);
}

fn sta16(state: &mut State, address: &SnesAddress) {
    let result = state.a.get_16();
    cpu::write_word(state, address, result);
}

fn sty16(state: &mut State, address: &SnesAddress) {
    let result = state.y.get_16();
    cpu::write_word(state, address, result);
}

fn stx16(state: &mut State, address: &SnesAddress) {
    let result = state.x.get_16();
    cpu::write_word(state, address, result);
}

fn stz16(state: &mut State, address: &SnesAddress) {
    let result = 0;
    cpu::write_word(state, address, result);
}

fn tax16(state: &mut State) {
    let a = state.a.get_16();
    state.x.set_16(a);
    set_zn16(state, a);
}

fn tay16(state: &mut State) {
    let a = state.a.get_16();
    state.y.set_16(a);
    set_zn16(state, a);
}

fn tcd16(state: &mut State) {
    let a = state.a.get_16();
    state.d = a;
    set_zn16(state, a);
}

fn tcs16(state: &mut State) {
    let value = state.a.get_16();
    state.s.set_address(value);
}

fn tdc16(state: &mut State) {
    let value = state.d;
    state.a.set_16(value);
    set_zn16(state, value);
}

fn tsc16(state: &mut State) {
    let value = state.s.get_address();
    state.a.set_16(value);
    set_zn16(state, value);
}

fn tsx16(state: &mut State) {
    let value = state.s.get_address();
    state.x.set_16(value);
    set_zn16(state, value);
}

fn txa16(state: &mut State) {
    let value = state.x.get_16();
    state.a.set_16(value);
    set_zn16(state, value);
}

fn txs16(state: &mut State) {
    let value = state.x.get_16();
    state.s.set_address(value);
}

fn txy16(state: &mut State) {
    let value = state.x.get_16();
    state.y.set_16(value);
    set_zn16(state, value);
}

fn tya16(state: &mut State) {
    let value = state.y.get_16();
    state.a.set_16(value);
    set_zn16(state, value);
}

fn tyx16(state: &mut State) {
    let value = state.y.get_16();
    state.x.set_16(value);
    set_zn16(state, value);
}
