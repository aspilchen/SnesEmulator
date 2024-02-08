#include "ricoh5a22.h"

namespace snes
{
    Ricoh5A22::Ricoh5A22(SnesMemory &memory) : mem_(memory), alu_(state_)
    {
        init();
    }

    void Ricoh5A22::init(void)
    {
        state_.stack() = 500;
    }

    void Ricoh5A22::set_verbose(bool is_on)
    {
        verbose_ = is_on;
    }

    void Ricoh5A22::execute(void)
    {
        Byte op = get_byte();

        switch (op)
        {
        case 0x61:
            op61_ADC();
            break;
        case 0x63:
            op63_ADC();
            break;
        case 0x65:
            op65_ADC();
            break;
        case 0x67:
            op67_ADC();
            break;
        case 0x69:
            op69_ADC();
            break;
        case 0x6D:
            op6D_ADC();
            break;
        case 0x6F:
            op6F_ADC();
            break;
        case 0x71:
            op71_ADC();
            break;
        case 0x72:
            op72_ADC();
            break;
        case 0x73:
            op73_ADC();
            break;
        case 0x75:
            op75_ADC();
            break;
        case 0x77:
            op77_ADC();
            break;
        case 0x79:
            op79_ADC();
            break;
        case 0x7D:
            op7D_ADC();
            break;
        case 0x7F:
            op7F_ADC();
            break;
        case 0x21:
            op21_AND();
            break;
        case 0x23:
            op23_AND();
            break;
        case 0x25:
            op25_AND();
            break;
        case 0x27:
            op27_AND();
            break;
        case 0x29:
            op29_AND();
            break;
        case 0x2D:
            op2D_AND();
            break;
        case 0x2F:
            op2F_AND();
            break;
        case 0x31:
            op31_AND();
            break;
        case 0x32:
            op32_AND();
            break;
        case 0x33:
            op33_AND();
            break;
        case 0x35:
            op35_AND();
            break;
        case 0x37:
            op37_AND();
            break;
        case 0x39:
            op39_AND();
            break;
        case 0x3D:
            op3D_AND();
            break;
        case 0x3F:
            op3F_AND();
            break;
        case 0x06:
            op06_ASL();
            break;
        case 0x0A:
            op0A_ASL();
            break;
        case 0x0E:
            op0E_ASL();
            break;
        case 0x16:
            op16_ASL();
            break;
        case 0x1E:
            op1E_ASL();
            break;
        case 0x90:
            op90_BCC();
            break;
        case 0xB0:
            opB0_BCS();
            break;
        case 0xF0:
            opF0_BEQ();
            break;
        case 0x24:
            op24_BIT();
            break;
        case 0x2C:
            op2C_BIT();
            break;
        case 0x34:
            op34_BIT();
            break;
        case 0x3C:
            op3C_BIT();
            break;
        case 0x89:
            op89_BIT();
            break;
        case 0x30:
            op30_BMI();
            break;
        case 0xD0:
            opD0_BNE();
            break;
        case 0x10:
            op10_BPL();
            break;
        case 0x80:
            op80_BRA();
            break;
        case 0x00:
            op00_BRK();
            break;
        case 0x82:
            op82_BRL();
            break;
        case 0x50:
            op50_BVC();
            break;
        case 0x70:
            op70_BVS();
            break;
        case 0x18:
            op18_CLC();
            break;
        case 0xD8:
            opD8_CLD();
            break;
        case 0x58:
            op58_CLI();
            break;
        case 0xB8:
            opB8_CLV();
            break;
        case 0xC1:
            opC1_CMP();
            break;
        case 0xC3:
            opC3_CMP();
            break;
        case 0xC5:
            opC5_CMP();
            break;
        case 0xC7:
            opC7_CMP();
            break;
        case 0xC9:
            opC9_CMP();
            break;
        case 0xCD:
            opCD_CMP();
            break;
        case 0xCF:
            opCF_CMP();
            break;
        case 0xD1:
            opD1_CMP();
            break;
        case 0xD2:
            opD2_CMP();
            break;
        case 0xD3:
            opD3_CMP();
            break;
        case 0xD5:
            opD5_CMP();
            break;
        case 0xD7:
            opD7_CMP();
            break;
        case 0xD9:
            opD9_CMP();
            break;
        case 0xDD:
            opDD_CMP();
            break;
        case 0xDF:
            opDF_CMP();
            break;
        case 0x02:
            op02_COP();
            break;
        case 0xE0:
            opE0_CPX();
            break;
        case 0xE4:
            opE4_CPX();
            break;
        case 0xEC:
            opEC_CPX();
            break;
        case 0xC0:
            opC0_CPY();
            break;
        case 0xC4:
            opC4_CPY();
            break;
        case 0xCC:
            opCC_CPY();
            break;
        case 0x3A:
            op3A_DEC();
            break;
        case 0xC6:
            opC6_DEC();
            break;
        case 0xCE:
            opCE_DEC();
            break;
        case 0xD6:
            opD6_DEC();
            break;
        case 0xDE:
            opDE_DEC();
            break;
        case 0xCA:
            opCA_DEX();
            break;
        case 0x88:
            op88_DEY();
            break;
        case 0x41:
            op41_EOR();
            break;
        case 0x43:
            op43_EOR();
            break;
        case 0x45:
            op45_EOR();
            break;
        case 0x47:
            op47_EOR();
            break;
        case 0x49:
            op49_EOR();
            break;
        case 0x4D:
            op4D_EOR();
            break;
        case 0x4F:
            op4F_EOR();
            break;
        case 0x51:
            op51_EOR();
            break;
        case 0x52:
            op52_EOR();
            break;
        case 0x53:
            op53_EOR();
            break;
        case 0x55:
            op55_EOR();
            break;
        case 0x57:
            op57_EOR();
            break;
        case 0x59:
            op59_EOR();
            break;
        case 0x5D:
            op5D_EOR();
            break;
        case 0x5F:
            op5F_EOR();
            break;
        case 0x1A:
            op1A_INC();
            break;
        case 0xE6:
            opE6_INC();
            break;
        case 0xEE:
            opEE_INC();
            break;
        case 0xF6:
            opF6_INC();
            break;
        case 0xFE:
            opFE_INC();
            break;
        case 0xE8:
            opE8_INX();
            break;
        case 0xC8:
            opC8_INY();
            break;
        case 0x4C:
            op4C_JMP();
            break;
        case 0x5C:
            op5C_JMP();
            break;
        case 0x6C:
            op6C_JMP();
            break;
        case 0x7C:
            op7C_JMP();
            break;
        case 0xDC:
            opDC_JMP();
            break;
        case 0x20:
            op20_JSR();
            break;
        case 0x22:
            op22_JSR();
            break;
        case 0xFC:
            opFC_JSR();
            break;
        case 0xA1:
            opA1_LDA();
            break;
        case 0xA3:
            opA3_LDA();
            break;
        case 0xA5:
            opA5_LDA();
            break;
        case 0xA7:
            opA7_LDA();
            break;
        case 0xA9:
            opA9_LDA();
            break;
        case 0xAD:
            opAD_LDA();
            break;
        case 0xAF:
            opAF_LDA();
            break;
        case 0xB1:
            opB1_LDA();
            break;
        case 0xB2:
            opB2_LDA();
            break;
        case 0xB3:
            opB3_LDA();
            break;
        case 0xB5:
            opB5_LDA();
            break;
        case 0xB7:
            opB7_LDA();
            break;
        case 0xB9:
            opB9_LDA();
            break;
        case 0xBD:
            opBD_LDA();
            break;
        case 0xBF:
            opBF_LDA();
            break;
        case 0xA2:
            opA2_LDX();
            break;
        case 0xA6:
            opA6_LDX();
            break;
        case 0xAE:
            opAE_LDX();
            break;
        case 0xB6:
            opB6_LDX();
            break;
        case 0xBE:
            opBE_LDX();
            break;
        case 0xA0:
            opA0_LDY();
            break;
        case 0xA4:
            opA4_LDY();
            break;
        case 0xAC:
            opAC_LDY();
            break;
        case 0xB4:
            opB4_LDY();
            break;
        case 0xBC:
            opBC_LDY();
            break;
        case 0x46:
            op46_LSR();
            break;
        case 0x4A:
            op4A_LSR();
            break;
        case 0x4E:
            op4E_LSR();
            break;
        case 0x56:
            op56_LSR();
            break;
        case 0x5E:
            op5E_LSR();
            break;
        case 0x54:
            op54_MVN();
            break;
        case 0x44:
            op44_MVP();
            break;
        case 0xEA:
            opEA_NOP();
            break;
        case 0x01:
            op01_ORA();
            break;
        case 0x03:
            op03_ORA();
            break;
        case 0x05:
            op05_ORA();
            break;
        case 0x07:
            op07_ORA();
            break;
        case 0x09:
            op09_ORA();
            break;
        case 0x0D:
            op0D_ORA();
            break;
        case 0x0F:
            op0F_ORA();
            break;
        case 0x11:
            op11_ORA();
            break;
        case 0x12:
            op12_ORA();
            break;
        case 0x13:
            op13_ORA();
            break;
        case 0x15:
            op15_ORA();
            break;
        case 0x17:
            op17_ORA();
            break;
        case 0x19:
            op19_ORA();
            break;
        case 0x1D:
            op1D_ORA();
            break;
        case 0x1F:
            op1F_ORA();
            break;
        case 0xF4:
            opF4_PEA();
            break;
        case 0xD4:
            opD4_PEI();
            break;
        case 0x62:
            op62_PER();
            break;
        case 0x48:
            op48_PHA();
            break;
        case 0x8B:
            op8B_PHB();
            break;
        case 0x0B:
            op0B_PHD();
            break;
        case 0x4B:
            op4B_PHK();
            break;
        case 0x08:
            op08_PHP();
            break;
        case 0xDA:
            opDA_PHX();
            break;
        case 0x5A:
            op5A_PHY();
            break;
        case 0x68:
            op68_PLA();
            break;
        case 0xAB:
            opAB_PLB();
            break;
        case 0x2B:
            op2B_PLD();
            break;
        case 0x28:
            op28_PLP();
            break;
        case 0xFA:
            opFA_PLX();
            break;
        case 0x7A:
            op7A_PLY();
            break;
        case 0xC2:
            opC2_REP();
            break;
        case 0x26:
            op26_ROL();
            break;
        case 0x2A:
            op2A_ROL();
            break;
        case 0x2E:
            op2E_ROL();
            break;
        case 0x36:
            op36_ROL();
            break;
        case 0x3E:
            op3E_ROL();
            break;
        case 0x66:
            op66_ROR();
            break;
        case 0x6A:
            op6A_ROR();
            break;
        case 0x6E:
            op6E_ROR();
            break;
        case 0x76:
            op76_ROR();
            break;
        case 0x7E:
            op7E_ROR();
            break;
        case 0x40:
            op40_RTI();
            break;
        case 0x6B:
            op6B_RTL();
            break;
        case 0x60:
            op60_RTS();
            break;
        case 0xE1:
            opE1_SBC();
            break;
        case 0xE3:
            opE3_SBC();
            break;
        case 0xE5:
            opE5_SBC();
            break;
        case 0xE7:
            opE7_SBC();
            break;
        case 0xE9:
            opE9_SBC();
            break;
        case 0xED:
            opED_SBC();
            break;
        case 0xEF:
            opEF_SBC();
            break;
        case 0xF1:
            opF1_SBC();
            break;
        case 0xF2:
            opF2_SBC();
            break;
        case 0xF3:
            opF3_SBC();
            break;
        case 0xF5:
            opF5_SBC();
            break;
        case 0xF7:
            opF7_SBC();
            break;
        case 0xF9:
            opF9_SBC();
            break;
        case 0xFD:
            opFD_SBC();
            break;
        case 0xFF:
            opFF_SBC();
            break;
        case 0x38:
            op38_SEC();
            break;
        case 0xF8:
            opF8_SED();
            break;
        case 0x78:
            op78_SEI();
            break;
        case 0xE2:
            opE2_SEP();
            break;
        case 0x81:
            op81_STA();
            break;
        case 0x83:
            op83_STA();
            break;
        case 0x85:
            op85_STA();
            break;
        case 0x87:
            op87_STA();
            break;
        case 0x8D:
            op8D_STA();
            break;
        case 0x8F:
            op8F_STA();
            break;
        case 0x91:
            op91_STA();
            break;
        case 0x92:
            op92_STA();
            break;
        case 0x93:
            op93_STA();
            break;
        case 0x95:
            op95_STA();
            break;
        case 0x97:
            op97_STA();
            break;
        case 0x99:
            op99_STA();
            break;
        case 0x9D:
            op9D_STA();
            break;
        case 0x9F:
            op9F_STA();
            break;
        case 0xDB:
            opDB_STP();
            break;
        case 0x86:
            op86_STX();
            break;
        case 0x8E:
            op8E_STX();
            break;
        case 0x96:
            op96_STX();
            break;
        case 0x84:
            op84_STY();
            break;
        case 0x8C:
            op8C_STY();
            break;
        case 0x94:
            op94_STY();
            break;
        case 0x64:
            op64_STZ();
            break;
        case 0x74:
            op74_STZ();
            break;
        case 0x9C:
            op9C_STZ();
            break;
        case 0x9E:
            op9E_STZ();
            break;
        case 0xAA:
            opAA_TAX();
            break;
        case 0xA8:
            opA8_TAY();
            break;
        case 0x5B:
            op5B_TCD();
            break;
        case 0x1B:
            op1B_TCS();
            break;
        case 0x7B:
            op7B_TDC();
            break;
        case 0x14:
            op14_TRB();
            break;
        case 0x1C:
            op1C_TRB();
            break;
        case 0x04:
            op04_TSB();
            break;
        case 0x0C:
            op0C_TSB();
            break;
        case 0x3B:
            op3B_TSC();
            break;
        case 0xBA:
            opBA_TSX();
            break;
        case 0x8A:
            op8A_TXA();
            break;
        case 0x9A:
            op9A_TXS();
            break;
        case 0x9B:
            op9B_TXY();
            break;
        case 0x98:
            op98_TYA();
            break;
        case 0xBB:
            opBB_TYX();
            break;
        case 0xCB:
            opCB_WAI();
            break;
        case 0x42:
            op42_WDM();
            break;
        case 0xEB:
            opEB_XBA();
            break;
        case 0xFB:
            opFB_XCE();
            break;
        default:
            break;
        }
        if (verbose_)
        {
            std::cout << "\n";
        }
    }

    const CpuState &Ricoh5A22::state(void) const
    {
        return state_;
    }

    Word Ricoh5A22::get_word(void)
    {
        SnesPtr &pc = state_.pc();
        const Word result = mem_.get_word(pc);
        pc += sizeof(result);
        return result;
    }

    Byte Ricoh5A22::get_byte(void)
    {
        SnesPtr &pc = state_.pc();
        const Byte result = mem_.get_byte(pc);
        pc += sizeof(result);
        return result;
    }

} // namespace snes
