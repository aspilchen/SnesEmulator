pub fn op61_ADC(state: &mut state::State) {
   let address = addr_modes::direct_indexed_indirect(state);
   adc16(state, address);}

pub fn op63_ADC(state: &mut state::State) {
   let address = addr_modes::stack_relative(state);
   adc16(state, address);}

pub fn op65_ADC(state: &mut state::State) {
   let address = addr_modes::direct(state);
   adc16(state, address);}

pub fn op67_ADC(state: &mut state::State) {
   let address = addr_modes::direct_indirect_long(state);
   adc16(state, address);}

pub fn op69_ADC(state: &mut state::State) {
   let address = addr_modes::immediate16(state);
   adc16(state, address);}

pub fn op6D_ADC(state: &mut state::State) {
   let address = addr_modes::absolute(state);
   adc16(state, address);}

pub fn op6F_ADC(state: &mut state::State) {
   let address = addr_modes::absolute_long(state);
   adc16(state, address);}

pub fn op71_ADC(state: &mut state::State) {
   let address = addr_modes::direct_indirect_indexed(state);
   adc16(state, address);}

pub fn op72_ADC(state: &mut state::State) {
   let address = addr_modes::stack_relative_indirect_indexed(state);
   adc16(state, address);}

pub fn op73_ADC(state: &mut state::State) {
   let address = addr_modes::direct_indirec_indexed(state);
   adc16(state, address);}

pub fn op75_ADC(state: &mut state::State) {
   let address = addr_modes:: direct_indexed_x(state);
   adc16(state, address);}

pub fn op77_ADC(state: &mut state::State) {
   let address = addr_modes::direct_indirect_long_indexed(state);
   adc16(state, address);}

pub fn op79_ADC(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_y(state);
   adc16(state, address);}

pub fn op7D_ADC(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_x(state);
   adc16(state, address);}

pub fn op7F_ADC(state: &mut state::State) {
   let address = addr_modes::absolute_long_indexed(state);
   adc16(state, address);}

pub fn op21_AND(state: &mut state::State) {
   let address = addr_modes::direct_indexed_indirect(state);
}

pub fn op23_AND(state: &mut state::State) {
   let address = addr_modes::stack_relative(state);
}

pub fn op25_AND(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn op27_AND(state: &mut state::State) {
   let address = addr_modes::direct_indirect_long(state);
}

pub fn op29_AND(state: &mut state::State) {
   let address = addr_modes::immediate16(state);
}

pub fn op2D_AND(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn op2F_AND(state: &mut state::State) {
   let address = addr_modes::absolute_long(state);
}

pub fn op31_AND(state: &mut state::State) {
   let address = addr_modes::direct_indirect_indexed(state);
}

pub fn op32_AND(state: &mut state::State) {
   let address = addr_modes::stack_relative_indirect_indexed(state);
}

pub fn op33_AND(state: &mut state::State) {
   let address = addr_modes::direct_indirec_indexed(state);
}

pub fn op35_AND(state: &mut state::State) {
   let address = addr_modes:: direct_indexed_x(state);
}

pub fn op37_AND(state: &mut state::State) {
   let address = addr_modes::direct_indirect_long_indexed(state);
}

pub fn op39_AND(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_y(state);
}

pub fn op3D_AND(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_x(state);
}

pub fn op3F_AND(state: &mut state::State) {
   let address = addr_modes::absolute_long_indexed(state);
}

pub fn op06_ASL(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn op0A_ASL(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Accumulator;
}

pub fn op0E_ASL(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn op16_ASL(state: &mut state::State) {
   let address = addr_modes:: direct_indexed_x(state);
}

pub fn op1E_ASL(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_x(state);
}

pub fn op90_BCC(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Program Counter Relative;
}

pub fn opB0_BCS(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Program Counter Relative;
}

pub fn opF0_BEQ(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Program Counter Relative;
}

pub fn op24_BIT(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn op2C_BIT(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn op34_BIT(state: &mut state::State) {
   let address = addr_modes:: direct_indexed_x(state);
}

pub fn op3C_BIT(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_x(state);
}

pub fn op89_BIT(state: &mut state::State) {
   let address = addr_modes::immediate16(state);
}

pub fn op30_BMI(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Program Counter Relative;
}

pub fn opD0_BNE(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Program Counter Relative;
}

pub fn op10_BPL(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Program Counter Relative;
}

pub fn op80_BRA(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Program Counter Relative;
}

pub fn op00_BRK(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Stack/Interrupt;
}

pub fn op82_BRL(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Program Counter Relative Long;
}

pub fn op50_BVC(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Program Counter Relative;
}

pub fn op70_BVS(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Program Counter Relative;
}

pub fn op18_CLC(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn opD8_CLD(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn op58_CLI(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn opB8_CLV(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn opC1_CMP(state: &mut state::State) {
   let address = addr_modes::direct_indexed_indirect(state);
}

pub fn opC3_CMP(state: &mut state::State) {
   let address = addr_modes::stack_relative(state);
}

pub fn opC5_CMP(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn opC7_CMP(state: &mut state::State) {
   let address = addr_modes::direct_indirect_long(state);
}

pub fn opC9_CMP(state: &mut state::State) {
   let address = addr_modes::immediate16(state);
}

pub fn opCD_CMP(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn opCF_CMP(state: &mut state::State) {
   let address = addr_modes::absolute_long(state);
}

pub fn opD1_CMP(state: &mut state::State) {
   let address = addr_modes::direct_indirect_indexed(state);
}

pub fn opD2_CMP(state: &mut state::State) {
   let address = addr_modes::stack_relative_indirect_indexed(state);
}

pub fn opD3_CMP(state: &mut state::State) {
   let address = addr_modes::direct_indirec_indexed(state);
}

pub fn opD5_CMP(state: &mut state::State) {
   let address = addr_modes:: direct_indexed_x(state);
}

pub fn opD7_CMP(state: &mut state::State) {
   let address = addr_modes::direct_indirect_long_indexed(state);
}

pub fn opD9_CMP(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_y(state);
}

pub fn opDD_CMP(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_x(state);
}

pub fn opDF_CMP(state: &mut state::State) {
   let address = addr_modes::absolute_long_indexed(state);
}

pub fn op02_COP(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Stack/Interrupt;
}

pub fn opE0_CPX(state: &mut state::State) {
   let address = addr_modes::immediate16(state);
}

pub fn opE4_CPX(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn opEC_CPX(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn opC0_CPY(state: &mut state::State) {
   let address = addr_modes::immediate16(state);
}

pub fn opC4_CPY(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn opCC_CPY(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn op3A_DEC(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Accumulator;
}

pub fn opC6_DEC(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn opCE_DEC(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn opD6_DEC(state: &mut state::State) {
   let address = addr_modes:: direct_indexed_x(state);
}

pub fn opDE_DEC(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_x(state);
}

pub fn opCA_DEX(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn op88_DEY(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn op41_EOR(state: &mut state::State) {
   let address = addr_modes::direct_indexed_indirect(state);
}

pub fn op43_EOR(state: &mut state::State) {
   let address = addr_modes::stack_relative(state);
}

pub fn op45_EOR(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn op47_EOR(state: &mut state::State) {
   let address = addr_modes::direct_indirect_long(state);
}

pub fn op49_EOR(state: &mut state::State) {
   let address = addr_modes::immediate16(state);
}

pub fn op4D_EOR(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn op4F_EOR(state: &mut state::State) {
   let address = addr_modes::absolute_long(state);
}

pub fn op51_EOR(state: &mut state::State) {
   let address = addr_modes::direct_indirect_indexed(state);
}

pub fn op52_EOR(state: &mut state::State) {
   let address = addr_modes::stack_relative_indirect_indexed(state);
}

pub fn op53_EOR(state: &mut state::State) {
   let address = addr_modes::direct_indirec_indexed(state);
}

pub fn op55_EOR(state: &mut state::State) {
   let address = addr_modes:: direct_indexed_x(state);
}

pub fn op57_EOR(state: &mut state::State) {
   let address = addr_modes::direct_indirect_long_indexed(state);
}

pub fn op59_EOR(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_y(state);
}

pub fn op5D_EOR(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_x(state);
}

pub fn op5F_EOR(state: &mut state::State) {
   let address = addr_modes::absolute_long_indexed(state);
}

pub fn op1A_INC(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Accumulator;
}

pub fn opE6_INC(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn opEE_INC(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn opF6_INC(state: &mut state::State) {
   let address = addr_modes:: direct_indexed_x(state);
}

pub fn opFE_INC(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_x(state);
}

pub fn opE8_INX(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn opC8_INY(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn op4C_JMP(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn op5C_JMP(state: &mut state::State) {
   let address = addr_modes::absolute_long(state);
}

pub fn op6C_JMP(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Absolute Indirect;
}

pub fn op7C_JMP(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Absolute Indexed Indirect;
}

pub fn opDC_JMP(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Absolute Indirect Long;
}

pub fn op20_JSR(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn op22_JSR(state: &mut state::State) {
   let address = addr_modes::absolute_long(state);
}

pub fn opFC_JSR(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Absolute Indexed Indirect;
}

pub fn opA1_LDA(state: &mut state::State) {
   let address = addr_modes::direct_indexed_indirect(state);
}

pub fn opA3_LDA(state: &mut state::State) {
   let address = addr_modes::stack_relative(state);
}

pub fn opA5_LDA(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn opA7_LDA(state: &mut state::State) {
   let address = addr_modes::direct_indirect_long(state);
}

pub fn opA9_LDA(state: &mut state::State) {
   let address = addr_modes::immediate16(state);
}

pub fn opAD_LDA(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn opAF_LDA(state: &mut state::State) {
   let address = addr_modes::absolute_long(state);
}

pub fn opB1_LDA(state: &mut state::State) {
   let address = addr_modes::direct_indirect_indexed(state);
}

pub fn opB2_LDA(state: &mut state::State) {
   let address = addr_modes::stack_relative_indirect_indexed(state);
}

pub fn opB3_LDA(state: &mut state::State) {
   let address = addr_modes::direct_indirec_indexed(state);
}

pub fn opB5_LDA(state: &mut state::State) {
   let address = addr_modes:: direct_indexed_x(state);
}

pub fn opB7_LDA(state: &mut state::State) {
   let address = addr_modes::direct_indirect_long_indexed(state);
}

pub fn opB9_LDA(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_y(state);
}

pub fn opBD_LDA(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_x(state);
}

pub fn opBF_LDA(state: &mut state::State) {
   let address = addr_modes::absolute_long_indexed(state);
}

pub fn opA2_LDX(state: &mut state::State) {
   let address = addr_modes::immediate16(state);
}

pub fn opA6_LDX(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn opAE_LDX(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn opB6_LDX(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode DP Indexed,Y;
}

pub fn opBE_LDX(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_y(state);
}

pub fn opA0_LDY(state: &mut state::State) {
   let address = addr_modes::immediate16(state);
}

pub fn opA4_LDY(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn opAC_LDY(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn opB4_LDY(state: &mut state::State) {
   let address = addr_modes:: direct_indexed_x(state);
}

pub fn opBC_LDY(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_x(state);
}

pub fn op46_LSR(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn op4A_LSR(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Accumulator;
}

pub fn op4E_LSR(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn op56_LSR(state: &mut state::State) {
   let address = addr_modes:: direct_indexed_x(state);
}

pub fn op5E_LSR(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_x(state);
}

pub fn op54_MVN(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Block Move;
}

pub fn op44_MVP(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Block Move;
}

pub fn opEA_NOP(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn op01_ORA(state: &mut state::State) {
   let address = addr_modes::direct_indexed_indirect(state);
}

pub fn op03_ORA(state: &mut state::State) {
   let address = addr_modes::stack_relative(state);
}

pub fn op05_ORA(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn op07_ORA(state: &mut state::State) {
   let address = addr_modes::direct_indirect_long(state);
}

pub fn op09_ORA(state: &mut state::State) {
   let address = addr_modes::immediate16(state);
}

pub fn op0D_ORA(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn op0F_ORA(state: &mut state::State) {
   let address = addr_modes::absolute_long(state);
}

pub fn op11_ORA(state: &mut state::State) {
   let address = addr_modes::direct_indirect_indexed(state);
}

pub fn op12_ORA(state: &mut state::State) {
   let address = addr_modes::stack_relative_indirect_indexed(state);
}

pub fn op13_ORA(state: &mut state::State) {
   let address = addr_modes::direct_indirec_indexed(state);
}

pub fn op15_ORA(state: &mut state::State) {
   let address = addr_modes:: direct_indexed_x(state);
}

pub fn op17_ORA(state: &mut state::State) {
   let address = addr_modes::direct_indirect_long_indexed(state);
}

pub fn op19_ORA(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_y(state);
}

pub fn op1D_ORA(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_x(state);
}

pub fn op1F_ORA(state: &mut state::State) {
   let address = addr_modes::absolute_long_indexed(state);
}

pub fn opF4_PEA(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Stack (Absolute);
}

pub fn opD4_PEI(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Stack (DP Indirect);
}

pub fn op62_PER(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Stack (PC Relative Long);
}

pub fn op48_PHA(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Stack (Push);
}

pub fn op8B_PHB(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Stack (Push);
}

pub fn op0B_PHD(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Stack (Push);
}

pub fn op4B_PHK(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Stack (Push);
}

pub fn op08_PHP(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Stack (Push);
}

pub fn opDA_PHX(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Stack (Push);
}

pub fn op5A_PHY(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Stack (Push);
}

pub fn op68_PLA(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Stack (Pull);
}

pub fn opAB_PLB(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Stack (Pull);
}

pub fn op2B_PLD(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Stack (Pull);
}

pub fn op28_PLP(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Stack (Pull);
}

pub fn opFA_PLX(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Stack (Pull);
}

pub fn op7A_PLY(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Stack (Pull);
}

pub fn opC2_REP(state: &mut state::State) {
   let address = addr_modes::immediate16(state);
}

pub fn op26_ROL(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn op2A_ROL(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Accumulator;
}

pub fn op2E_ROL(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn op36_ROL(state: &mut state::State) {
   let address = addr_modes:: direct_indexed_x(state);
}

pub fn op3E_ROL(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_x(state);
}

pub fn op66_ROR(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn op6A_ROR(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Accumulator;
}

pub fn op6E_ROR(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn op76_ROR(state: &mut state::State) {
   let address = addr_modes:: direct_indexed_x(state);
}

pub fn op7E_ROR(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_x(state);
}

pub fn op40_RTI(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Stack (RTI);
}

pub fn op6B_RTL(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Stack (RTL);
}

pub fn op60_RTS(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Stack (RTS);
}

pub fn opE1_SBC(state: &mut state::State) {
   let address = addr_modes::direct_indexed_indirect(state);
}

pub fn opE3_SBC(state: &mut state::State) {
   let address = addr_modes::stack_relative(state);
}

pub fn opE5_SBC(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn opE7_SBC(state: &mut state::State) {
   let address = addr_modes::direct_indirect_long(state);
}

pub fn opE9_SBC(state: &mut state::State) {
   let address = addr_modes::immediate16(state);
}

pub fn opED_SBC(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn opEF_SBC(state: &mut state::State) {
   let address = addr_modes::absolute_long(state);
}

pub fn opF1_SBC(state: &mut state::State) {
   let address = addr_modes::direct_indirect_indexed(state);
}

pub fn opF2_SBC(state: &mut state::State) {
   let address = addr_modes::stack_relative_indirect_indexed(state);
}

pub fn opF3_SBC(state: &mut state::State) {
   let address = addr_modes::direct_indirec_indexed(state);
}

pub fn opF5_SBC(state: &mut state::State) {
   let address = addr_modes:: direct_indexed_x(state);
}

pub fn opF7_SBC(state: &mut state::State) {
   let address = addr_modes::direct_indirect_long_indexed(state);
}

pub fn opF9_SBC(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_y(state);
}

pub fn opFD_SBC(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_x(state);
}

pub fn opFF_SBC(state: &mut state::State) {
   let address = addr_modes::absolute_long_indexed(state);
}

pub fn op38_SEC(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn opF8_SED(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn op78_SEI(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn opE2_SEP(state: &mut state::State) {
   let address = addr_modes::immediate16(state);
}

pub fn op81_STA(state: &mut state::State) {
   let address = addr_modes::direct_indexed_indirect(state);
}

pub fn op83_STA(state: &mut state::State) {
   let address = addr_modes::stack_relative(state);
}

pub fn op85_STA(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn op87_STA(state: &mut state::State) {
   let address = addr_modes::direct_indirect_long(state);
}

pub fn op8D_STA(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn op8F_STA(state: &mut state::State) {
   let address = addr_modes::absolute_long(state);
}

pub fn op91_STA(state: &mut state::State) {
   let address = addr_modes::direct_indirect_indexed(state);
}

pub fn op92_STA(state: &mut state::State) {
   let address = addr_modes::stack_relative_indirect_indexed(state);
}

pub fn op93_STA(state: &mut state::State) {
   let address = addr_modes::direct_indirec_indexed(state);
}

pub fn op95_STA(state: &mut state::State) {
   let address = addr_modes:: direct_indexed_x(state);
}

pub fn op97_STA(state: &mut state::State) {
   let address = addr_modes::direct_indirect_long_indexed(state);
}

pub fn op99_STA(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_y(state);
}

pub fn op9D_STA(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_x(state);
}

pub fn op9F_STA(state: &mut state::State) {
   let address = addr_modes::absolute_long_indexed(state);
}

pub fn opDB_STP(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn op86_STX(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn op8E_STX(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn op96_STX(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode DP Indexed,Y;
}

pub fn op84_STY(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn op8C_STY(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn op94_STY(state: &mut state::State) {
   let address = addr_modes:: direct_indexed_x(state);
}

pub fn op64_STZ(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn op74_STZ(state: &mut state::State) {
   let address = addr_modes:: direct_indexed_x(state);
}

pub fn op9C_STZ(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn op9E_STZ(state: &mut state::State) {
   let address = addr_modes::absolute_indexed_x(state);
}

pub fn opAA_TAX(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn opA8_TAY(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn op5B_TCD(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn op1B_TCS(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn op7B_TDC(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn op14_TRB(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn op1C_TRB(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn op04_TSB(state: &mut state::State) {
   let address = addr_modes::direct(state);
}

pub fn op0C_TSB(state: &mut state::State) {
   let address = addr_modes::absolute(state);
}

pub fn op3B_TSC(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn opBA_TSX(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn op8A_TXA(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn op9A_TXS(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn op9B_TXY(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn op98_TYA(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn opBB_TYX(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn opCB_WAI(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn op42_WDM(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode ;
}

pub fn opEB_XBA(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

pub fn opFB_XCE(state: &mut state::State) {
   let address = addr_modes::0; // TODO: define address mode Implied;
}

