#ifndef __RICOH5A22_H
#define __RICOH5A22_H

#include <iostream>
#include <iomanip>

#include "cpu_state.h"
#include "alu.h"
#include "snes_mem.h"

namespace snes
{
    /**
     * @brief Executes the fetch/execute cycle. Works together with the CpuCore to emulate the Ricoh5A22 CPU.
     *
     */
    class Ricoh5A22
    {
    public:
        Ricoh5A22(SnesMemory &memory);

        void init(void);

        void execute(void);

        const CpuState &state(void) const;
        void set_state(CpuState state);

        void set_verbose(bool is_on);

        // Ops are implemented in ricoh5a22_ops.cc
        void op61_ADC(void);
        void op63_ADC(void);
        void op65_ADC(void);
        void op67_ADC(void);
        void op69_ADC(void);
        void op6D_ADC(void);
        void op6F_ADC(void);
        void op71_ADC(void);
        void op72_ADC(void);
        void op73_ADC(void);
        void op75_ADC(void);
        void op77_ADC(void);
        void op79_ADC(void);
        void op7D_ADC(void);
        void op7F_ADC(void);
        void op21_AND(void);
        void op23_AND(void);
        void op25_AND(void);
        void op27_AND(void);
        void op29_AND(void);
        void op2D_AND(void);
        void op2F_AND(void);
        void op31_AND(void);
        void op32_AND(void);
        void op33_AND(void);
        void op35_AND(void);
        void op37_AND(void);
        void op39_AND(void);
        void op3D_AND(void);
        void op3F_AND(void);
        void op06_ASL(void);
        void op0A_ASL(void);
        void op0E_ASL(void);
        void op16_ASL(void);
        void op1E_ASL(void);
        void op90_BCC(void);
        void opB0_BCS(void);
        void opF0_BEQ(void);
        void op24_BIT(void);
        void op2C_BIT(void);
        void op34_BIT(void);
        void op3C_BIT(void);
        void op89_BIT(void);
        void op30_BMI(void);
        void opD0_BNE(void);
        void op10_BPL(void);
        void op80_BRA(void);
        void op00_BRK(void);
        void op82_BRL(void);
        void op50_BVC(void);
        void op70_BVS(void);
        void op18_CLC(void);
        void opD8_CLD(void);
        void op58_CLI(void);
        void opB8_CLV(void);
        void opC1_CMP(void);
        void opC3_CMP(void);
        void opC5_CMP(void);
        void opC7_CMP(void);
        void opC9_CMP(void);
        void opCD_CMP(void);
        void opCF_CMP(void);
        void opD1_CMP(void);
        void opD2_CMP(void);
        void opD3_CMP(void);
        void opD5_CMP(void);
        void opD7_CMP(void);
        void opD9_CMP(void);
        void opDD_CMP(void);
        void opDF_CMP(void);
        void op02_COP(void);
        void opE0_CPX(void);
        void opE4_CPX(void);
        void opEC_CPX(void);
        void opC0_CPY(void);
        void opC4_CPY(void);
        void opCC_CPY(void);
        void op3A_DEC(void);
        void opC6_DEC(void);
        void opCE_DEC(void);
        void opD6_DEC(void);
        void opDE_DEC(void);
        void opCA_DEX(void);
        void op88_DEY(void);
        void op41_EOR(void);
        void op43_EOR(void);
        void op45_EOR(void);
        void op47_EOR(void);
        void op49_EOR(void);
        void op4D_EOR(void);
        void op4F_EOR(void);
        void op51_EOR(void);
        void op52_EOR(void);
        void op53_EOR(void);
        void op55_EOR(void);
        void op57_EOR(void);
        void op59_EOR(void);
        void op5D_EOR(void);
        void op5F_EOR(void);
        void op1A_INC(void);
        void opE6_INC(void);
        void opEE_INC(void);
        void opF6_INC(void);
        void opFE_INC(void);
        void opE8_INX(void);
        void opC8_INY(void);
        void op4C_JMP(void);
        void op5C_JMP(void);
        void op6C_JMP(void);
        void op7C_JMP(void);
        void opDC_JMP(void);
        void op20_JSR(void);
        void op22_JSR(void);
        void opFC_JSR(void);
        void opA1_LDA(void);
        void opA3_LDA(void);
        void opA5_LDA(void);
        void opA7_LDA(void);
        void opA9_LDA(void);
        void opAD_LDA(void);
        void opAF_LDA(void);
        void opB1_LDA(void);
        void opB2_LDA(void);
        void opB3_LDA(void);
        void opB5_LDA(void);
        void opB7_LDA(void);
        void opB9_LDA(void);
        void opBD_LDA(void);
        void opBF_LDA(void);
        void opA2_LDX(void);
        void opA6_LDX(void);
        void opAE_LDX(void);
        void opB6_LDX(void);
        void opBE_LDX(void);
        void opA0_LDY(void);
        void opA4_LDY(void);
        void opAC_LDY(void);
        void opB4_LDY(void);
        void opBC_LDY(void);
        void op46_LSR(void);
        void op4A_LSR(void);
        void op4E_LSR(void);
        void op56_LSR(void);
        void op5E_LSR(void);
        void op54_MVN(void);
        void op44_MVP(void);
        void opEA_NOP(void);
        void op01_ORA(void);
        void op03_ORA(void);
        void op05_ORA(void);
        void op07_ORA(void);
        void op09_ORA(void);
        void op0D_ORA(void);
        void op0F_ORA(void);
        void op11_ORA(void);
        void op12_ORA(void);
        void op13_ORA(void);
        void op15_ORA(void);
        void op17_ORA(void);
        void op19_ORA(void);
        void op1D_ORA(void);
        void op1F_ORA(void);
        void opF4_PEA(void);
        void opD4_PEI(void);
        void op62_PER(void);
        void op48_PHA(void);
        void op8B_PHB(void);
        void op0B_PHD(void);
        void op4B_PHK(void);
        void op08_PHP(void);
        void opDA_PHX(void);
        void op5A_PHY(void);
        void op68_PLA(void);
        void opAB_PLB(void);
        void op2B_PLD(void);
        void op28_PLP(void);
        void opFA_PLX(void);
        void op7A_PLY(void);
        void opC2_REP(void);
        void op26_ROL(void);
        void op2A_ROL(void);
        void op2E_ROL(void);
        void op36_ROL(void);
        void op3E_ROL(void);
        void op66_ROR(void);
        void op6A_ROR(void);
        void op6E_ROR(void);
        void op76_ROR(void);
        void op7E_ROR(void);
        void op40_RTI(void);
        void op6B_RTL(void);
        void op60_RTS(void);
        void opE1_SBC(void);
        void opE3_SBC(void);
        void opE5_SBC(void);
        void opE7_SBC(void);
        void opE9_SBC(void);
        void opED_SBC(void);
        void opEF_SBC(void);
        void opF1_SBC(void);
        void opF2_SBC(void);
        void opF3_SBC(void);
        void opF5_SBC(void);
        void opF7_SBC(void);
        void opF9_SBC(void);
        void opFD_SBC(void);
        void opFF_SBC(void);
        void op38_SEC(void);
        void opF8_SED(void);
        void op78_SEI(void);
        void opE2_SEP(void);
        void op81_STA(void);
        void op83_STA(void);
        void op85_STA(void);
        void op87_STA(void);
        void op8D_STA(void);
        void op8F_STA(void);
        void op91_STA(void);
        void op92_STA(void);
        void op93_STA(void);
        void op95_STA(void);
        void op97_STA(void);
        void op99_STA(void);
        void op9D_STA(void);
        void op9F_STA(void);
        void opDB_STP(void);
        void op86_STX(void);
        void op8E_STX(void);
        void op96_STX(void);
        void op84_STY(void);
        void op8C_STY(void);
        void op94_STY(void);
        void op64_STZ(void);
        void op74_STZ(void);
        void op9C_STZ(void);
        void op9E_STZ(void);
        void opAA_TAX(void);
        void opA8_TAY(void);
        void op5B_TCD(void);
        void op1B_TCS(void);
        void op7B_TDC(void);
        void op14_TRB(void);
        void op1C_TRB(void);
        void op04_TSB(void);
        void op0C_TSB(void);
        void op3B_TSC(void);
        void opBA_TSX(void);
        void op8A_TXA(void);
        void op9A_TXS(void);
        void op9B_TXY(void);
        void op98_TYA(void);
        void opBB_TYX(void);
        void opCB_WAI(void);
        void op42_WDM(void);
        void opEB_XBA(void);
        void opFB_XCE(void);

        // Address modes are public to make testing easier

        Byte addr_immediate8(void);  //
        Word addr_immediate16(void); //
        SnesPtr addr_absolute_indexed_indirect(void);
        SnesPtr addr_absolute_indexed_x(void); //
        SnesPtr addr_absolute_indexed_y(void); //
        SnesPtr addr_absolute_indirect(void);
        SnesPtr addr_absolute_long_indexed(void);        //
        SnesPtr addr_absolute_long(void);                //
        SnesPtr addr_absolute(void);                     //
        SnesPtr addr_direct_indexed_indirect(void);      //
        SnesPtr addr_direct_indexed_x(void);             //
        SnesPtr addr_direct_indexed_y(void);             //
        SnesPtr addr_direct_indirect_indexed_long(void); //
        SnesPtr addr_direct_indirect_indexed(void);
        SnesPtr addr_direct_indirect_long(void); //
        SnesPtr addr_direct_indirect(void);      //
        SnesPtr addr_direct(void);               //
        SnesPtr addr_pc_relative_long(void);
        SnesPtr addr_pc_relative(void);
        SnesPtr addr_stack_relative_indirect_indexed(void); //
        SnesPtr addr_stack_relative(void);                  //

        Word get_word(void);
        Byte get_byte(void);

    private:
        bool verbose_;
        SnesMemory &mem_;
        CpuState state_;
        Alu alu_;
    };
} // namespace snes

#endif