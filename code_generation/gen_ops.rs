// Implement these manually

pub fn bcc_90(state: &mut State) {
  // TODO: implement bcc_90
}

pub fn bcs_b0(state: &mut State) {
  // TODO: implement bcs_b0
}

pub fn beq_f0(state: &mut State) {
  // TODO: implement beq_f0
}

pub fn bmi_30(state: &mut State) {
  // TODO: implement bmi_30
}

pub fn bne_d0(state: &mut State) {
  // TODO: implement bne_d0
}

pub fn bpl_10(state: &mut State) {
  // TODO: implement bpl_10
}

pub fn bra_80(state: &mut State) {
  // TODO: implement bra_80
}

pub fn brk_00(state: &mut State) {
  // TODO: implement brk_00
}

pub fn brl_82(state: &mut State) {
  // TODO: implement brl_82
}

pub fn bvc_50(state: &mut State) {
  // TODO: implement bvc_50
}

pub fn bvs_70(state: &mut State) {
  // TODO: implement bvs_70
}

pub fn clc_18(state: &mut State) {
  // TODO: implement clc_18
}

pub fn cld_d8(state: &mut State) {
  // TODO: implement cld_d8
}

pub fn cli_58(state: &mut State) {
  // TODO: implement cli_58
}

pub fn clv_b8(state: &mut State) {
  // TODO: implement clv_b8
}

pub fn cop_02(state: &mut State) {
  // TODO: implement cop_02
}

pub fn jmp_4c(state: &mut State) {
  // TODO: implement jmp_4c
}

pub fn jmp_5c(state: &mut State) {
  // TODO: implement jmp_5c
}

pub fn jmp_6c(state: &mut State) {
  // TODO: implement jmp_6c
}

pub fn jmp_7c(state: &mut State) {
  // TODO: implement jmp_7c
}

pub fn jmp_dc(state: &mut State) {
  // TODO: implement jmp_dc
}

pub fn jsr_20(state: &mut State) {
  // TODO: implement jsr_20
}

pub fn jsr_22(state: &mut State) {
  // TODO: implement jsr_22
}

pub fn jsr_fc(state: &mut State) {
  // TODO: implement jsr_fc
}

pub fn mvn_54(state: &mut State) {
  // TODO: implement mvn_54
}

pub fn mvp_44(state: &mut State) {
  // TODO: implement mvp_44
}

pub fn nop_ea(state: &mut State) {
  // TODO: implement nop_ea
}

pub fn pea_f4(state: &mut State) {
  // TODO: implement pea_f4
}

pub fn pei_d4(state: &mut State) {
  // TODO: implement pei_d4
}

pub fn per_62(state: &mut State) {
  // TODO: implement per_62
}

pub fn pha_48(state: &mut State) {
  // TODO: implement pha_48
}

pub fn phb_8b(state: &mut State) {
  // TODO: implement phb_8b
}

pub fn phd_0b(state: &mut State) {
  // TODO: implement phd_0b
}

pub fn phk_4b(state: &mut State) {
  // TODO: implement phk_4b
}

pub fn php_08(state: &mut State) {
  // TODO: implement php_08
}

pub fn phx_da(state: &mut State) {
  // TODO: implement phx_da
}

pub fn phy_5a(state: &mut State) {
  // TODO: implement phy_5a
}

pub fn pla_68(state: &mut State) {
  // TODO: implement pla_68
}

pub fn plb_ab(state: &mut State) {
  // TODO: implement plb_ab
}

pub fn pld_2b(state: &mut State) {
  // TODO: implement pld_2b
}

pub fn plp_28(state: &mut State) {
  // TODO: implement plp_28
}

pub fn plx_fa(state: &mut State) {
  // TODO: implement plx_fa
}

pub fn ply_7a(state: &mut State) {
  // TODO: implement ply_7a
}

pub fn rep_c2(state: &mut State) {
  // TODO: implement rep_c2
}

pub fn rti_40(state: &mut State) {
  // TODO: implement rti_40
}

pub fn rtl_6b(state: &mut State) {
  // TODO: implement rtl_6b
}

pub fn rts_60(state: &mut State) {
  // TODO: implement rts_60
}

pub fn sec_38(state: &mut State) {
  // TODO: implement sec_38
}

pub fn sed_f8(state: &mut State) {
  // TODO: implement sed_f8
}

pub fn sei_78(state: &mut State) {
  // TODO: implement sei_78
}

pub fn sep_e2(state: &mut State) {
  // TODO: implement sep_e2
}

pub fn stp_db(state: &mut State) {
  // TODO: implement stp_db
}

pub fn wai_cb(state: &mut State) {
  // TODO: implement wai_cb
}

pub fn wdm_42(state: &mut State) {
  // TODO: implement wdm_42
}

pub fn xba_eb(state: &mut State) {
  // TODO: implement xba_eb
}

pub fn xce_fb(state: &mut State) {
  // TODO: implement xce_fb
}


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
   todo!("implement SR Indirect Indexed,Y");
   if state.p.is_set_m() {
       // adc8(state, &address);
   } else {
       adc16(state, &address);
   }
}

pub fn adc_75(state: &mut State) {
   let address = addr:: direct_indexed_x(state);
   if state.p.is_set_m() {
       // adc8(state, &address);
   } else {
       adc16(state, &address);
   }
}

pub fn adc_77(state: &mut State) {
   todo!("implement DP Indirect Long Indexed, Y");
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
   todo!("implement SR Indirect Indexed,Y");
   if state.p.is_set_m() {
       // and8(state, &address);
   } else {
       and16(state, &address);
   }
}

pub fn and_35(state: &mut State) {
   let address = addr:: direct_indexed_x(state);
   if state.p.is_set_m() {
       // and8(state, &address);
   } else {
       and16(state, &address);
   }
}

pub fn and_37(state: &mut State) {
   todo!("implement DP Indirect Long Indexed, Y");
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
   let address = addr:: direct_indexed_x(state);
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
   let address = addr:: direct_indexed_x(state);
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
   todo!("implement SR Indirect Indexed,Y");
   if state.p.is_set_m() {
       // cmp8(state, &address);
   } else {
       cmp16(state, &address);
   }
}

pub fn cmp_d5(state: &mut State) {
   let address = addr:: direct_indexed_x(state);
   if state.p.is_set_m() {
       // cmp8(state, &address);
   } else {
       cmp16(state, &address);
   }
}

pub fn cmp_d7(state: &mut State) {
   todo!("implement DP Indirect Long Indexed, Y");
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
   let address = addr:: direct_indexed_x(state);
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
   todo!("implement SR Indirect Indexed,Y");
   if state.p.is_set_m() {
       // eor8(state, &address);
   } else {
       eor16(state, &address);
   }
}

pub fn eor_55(state: &mut State) {
   let address = addr:: direct_indexed_x(state);
   if state.p.is_set_m() {
       // eor8(state, &address);
   } else {
       eor16(state, &address);
   }
}

pub fn eor_57(state: &mut State) {
   todo!("implement DP Indirect Long Indexed, Y");
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
   let address = addr:: direct_indexed_x(state);
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
   todo!("implement SR Indirect Indexed,Y");
   if state.p.is_set_m() {
       // lda8(state, &address);
   } else {
       lda16(state, &address);
   }
}

pub fn lda_b5(state: &mut State) {
   let address = addr:: direct_indexed_x(state);
   if state.p.is_set_m() {
       // lda8(state, &address);
   } else {
       lda16(state, &address);
   }
}

pub fn lda_b7(state: &mut State) {
   todo!("implement DP Indirect Long Indexed, Y");
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
   let address = addr:: direct_indexed_y(state);
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
   let address = addr:: direct_indexed_x(state);
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
   let address = addr:: direct_indexed_x(state);
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
   todo!("implement SR Indirect Indexed,Y");
   if state.p.is_set_m() {
       // ora8(state, &address);
   } else {
       ora16(state, &address);
   }
}

pub fn ora_15(state: &mut State) {
   let address = addr:: direct_indexed_x(state);
   if state.p.is_set_m() {
       // ora8(state, &address);
   } else {
       ora16(state, &address);
   }
}

pub fn ora_17(state: &mut State) {
   todo!("implement DP Indirect Long Indexed, Y");
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
   let address = addr:: direct_indexed_x(state);
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
   let address = addr:: direct_indexed_x(state);
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
   todo!("implement SR Indirect Indexed,Y");
   if state.p.is_set_m() {
       // sbc8(state, &address);
   } else {
       sbc16(state, &address);
   }
}

pub fn sbc_f5(state: &mut State) {
   let address = addr:: direct_indexed_x(state);
   if state.p.is_set_m() {
       // sbc8(state, &address);
   } else {
       sbc16(state, &address);
   }
}

pub fn sbc_f7(state: &mut State) {
   todo!("implement DP Indirect Long Indexed, Y");
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
   todo!("implement SR Indirect Indexed,Y");
   if state.p.is_set_m() {
       // sta8(state, &address);
   } else {
       sta16(state, &address);
   }
}

pub fn sta_95(state: &mut State) {
   let address = addr:: direct_indexed_x(state);
   if state.p.is_set_m() {
       // sta8(state, &address);
   } else {
       sta16(state, &address);
   }
}

pub fn sta_97(state: &mut State) {
   todo!("implement DP Indirect Long Indexed, Y");
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
   let address = addr:: direct_indexed_y(state);
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
   let address = addr:: direct_indexed_x(state);
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
   let address = addr:: direct_indexed_x(state);
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

