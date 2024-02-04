#include "ricoh5a22.h"

namespace snes {

void Ricoh5A22::op61_ADC(void) {
    const SnesPtr ptr = addr_direct_indexed_indirect();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.add_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.add_with_carry(arg);
    }
}

void Ricoh5A22::op63_ADC(void) {
    const SnesPtr ptr = addr_stack_relative();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.add_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.add_with_carry(arg);
    }
}

void Ricoh5A22::op65_ADC(void) {
    const SnesPtr ptr = addr_direct();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.add_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.add_with_carry(arg);
    }
}

void Ricoh5A22::op67_ADC(void) {
    const SnesPtr ptr = addr_direct_indirect_long();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.add_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.add_with_carry(arg);
    }
}

void Ricoh5A22::op69_ADC(void) {
    if (state_.is_status_memory()) {
        const Byte arg = addr_immediate8();
        alu_.add_with_carry(arg);
    } else {
        const Word arg = addr_immediate16();
        alu_.add_with_carry(arg);
    }
}

void Ricoh5A22::op6D_ADC(void) {
    const SnesPtr ptr = addr_absolute();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.add_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.add_with_carry(arg);
    }
}

void Ricoh5A22::op6F_ADC(void) {
    const SnesPtr ptr = addr_absolute_long();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.add_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.add_with_carry(arg);
    }
}

void Ricoh5A22::op71_ADC(void) {
    const SnesPtr ptr = addr_direct_indirect_indexed();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.add_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.add_with_carry(arg);
    }
}

void Ricoh5A22::op72_ADC(void) {
    const SnesPtr ptr = addr_direct_indirect();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.add_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.add_with_carry(arg);
    }
}

void Ricoh5A22::op73_ADC(void) {
    const SnesPtr ptr = addr_stack_relative_indirect_indexed();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.add_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.add_with_carry(arg);
    }
}

void Ricoh5A22::op75_ADC(void) {
    const SnesPtr ptr = addr_direct_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.add_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.add_with_carry(arg);
    }
}

void Ricoh5A22::op77_ADC(void) {
    const SnesPtr ptr = addr_direct_indirect_indexed_long();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.add_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.add_with_carry(arg);
    }
}

void Ricoh5A22::op79_ADC(void) {
    const SnesPtr ptr = addr_absolute_indexed_y();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.add_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.add_with_carry(arg);
    }
}

void Ricoh5A22::op7D_ADC(void) {
    const SnesPtr ptr = addr_absolute_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.add_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.add_with_carry(arg);
    }
}

void Ricoh5A22::op7F_ADC(void) {
    const SnesPtr ptr = addr_absolute_long_indexed();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.add_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.add_with_carry(arg);
    }
}

void Ricoh5A22::op21_AND(void) {
    const SnesPtr ptr = addr_direct_indexed_indirect();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_and(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_and(arg);
    }
}

void Ricoh5A22::op23_AND(void) {
    const SnesPtr ptr = addr_stack_relative();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_and(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_and(arg);
    }
}

void Ricoh5A22::op25_AND(void) {
    const SnesPtr ptr = addr_direct();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_and(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_and(arg);
    }
}

void Ricoh5A22::op27_AND(void) {
    const SnesPtr ptr = addr_direct_indirect_long();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_and(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_and(arg);
    }
}

void Ricoh5A22::op29_AND(void) {
    if (state_.is_status_memory()) {
        const Byte arg = addr_immediate8();
        alu_.logic_and(arg);
    } else {
        const Word arg = addr_immediate16();
        alu_.logic_and(arg);
    }
}

void Ricoh5A22::op2D_AND(void) {
    const SnesPtr ptr = addr_absolute();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_and(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_and(arg);
    }
}

void Ricoh5A22::op2F_AND(void) {
    const SnesPtr ptr = addr_absolute_long();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_and(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_and(arg);
    }
}

void Ricoh5A22::op31_AND(void) {
    const SnesPtr ptr = addr_direct_indirect_indexed();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_and(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_and(arg);
    }
}

void Ricoh5A22::op32_AND(void) {
    const SnesPtr ptr = addr_direct_indirect();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_and(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_and(arg);
    }
}

void Ricoh5A22::op33_AND(void) {
    const SnesPtr ptr = addr_stack_relative_indirect_indexed();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_and(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_and(arg);
    }
}

void Ricoh5A22::op35_AND(void) {
    const SnesPtr ptr = addr_direct_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_and(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_and(arg);
    }
}

void Ricoh5A22::op37_AND(void) {
    const SnesPtr ptr = addr_direct_indirect_indexed_long();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_and(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_and(arg);
    }
}

void Ricoh5A22::op39_AND(void) {
    const SnesPtr ptr = addr_absolute_indexed_y();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_and(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_and(arg);
    }
}

void Ricoh5A22::op3D_AND(void) {
    const SnesPtr ptr = addr_absolute_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_and(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_and(arg);
    }
}

void Ricoh5A22::op3F_AND(void) {
    const SnesPtr ptr = addr_absolute_long_indexed();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_and(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_and(arg);
    }
}

void Ricoh5A22::op06_ASL(void) {
    const SnesPtr ptr = addr_direct();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.arith_shift_left(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.arith_shift_left(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::op0A_ASL(void) {
    if (state_.is_status_memory()) {
        const Byte arg = state_.acc8();
        const Byte result = alu_.arith_shift_left(arg);
        state_.set_acc(result);
    } else {
        const Word arg = state_.acc16();
        const Word result = alu_.arith_shift_left(arg);
        state_.set_acc(result);
    }
}

void Ricoh5A22::op0E_ASL(void) {
    const SnesPtr ptr = addr_absolute();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.arith_shift_left(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.arith_shift_left(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::op16_ASL(void) {
    const SnesPtr ptr = addr_direct_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.arith_shift_left(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.arith_shift_left(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::op1E_ASL(void) {
    const SnesPtr ptr = addr_absolute_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.arith_shift_left(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.arith_shift_left(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::op90_BCC(void){
    // Not implemented
}

void Ricoh5A22::opB0_BCS(void){
    // Not implemented
}

void Ricoh5A22::opF0_BEQ(void){
    // Not implemented
}

void Ricoh5A22::op24_BIT(void) {
    const SnesPtr ptr = addr_direct();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.bit(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.bit(arg);
    }
}

void Ricoh5A22::op2C_BIT(void) {
    const SnesPtr ptr = addr_absolute();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.bit(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.bit(arg);
    }
}

void Ricoh5A22::op34_BIT(void) {
    const SnesPtr ptr = addr_direct_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.bit(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.bit(arg);
    }
}

void Ricoh5A22::op3C_BIT(void) {
    const SnesPtr ptr = addr_absolute_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.bit(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.bit(arg);
    }
}

void Ricoh5A22::op89_BIT(void) {
    if (state_.is_status_memory()) {
        const Byte arg = addr_immediate8();
        alu_.bit(arg);
    } else {
        const Word arg = addr_immediate16();
        alu_.bit(arg);
    }
}

void Ricoh5A22::op30_BMI(void){
    // Not implemented
}

void Ricoh5A22::opD0_BNE(void){
    // Not implemented
}

void Ricoh5A22::op10_BPL(void){
    // Not implemented
}

void Ricoh5A22::op80_BRA(void){
    // Not implemented
}

void Ricoh5A22::op00_BRK(void){
    // Not implemented
}

void Ricoh5A22::op82_BRL(void){
    // Not implemented
}

void Ricoh5A22::op50_BVC(void){
    // Not implemented
}

void Ricoh5A22::op70_BVS(void){
    // Not implemented
}

void Ricoh5A22::op18_CLC(void){
    // Not implemented
}

void Ricoh5A22::opD8_CLD(void){
    // Not implemented
}

void Ricoh5A22::op58_CLI(void){
    // Not implemented
}

void Ricoh5A22::opB8_CLV(void){
    // Not implemented
}

void Ricoh5A22::opC1_CMP(void) {
    const SnesPtr ptr = addr_direct_indexed_indirect();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.compare_with_acc(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.compare_with_acc(arg);
    }
}

void Ricoh5A22::opC3_CMP(void) {
    const SnesPtr ptr = addr_stack_relative();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.compare_with_acc(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.compare_with_acc(arg);
    }
}

void Ricoh5A22::opC5_CMP(void) {
    const SnesPtr ptr = addr_direct();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.compare_with_acc(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.compare_with_acc(arg);
    }
}

void Ricoh5A22::opC7_CMP(void) {
    const SnesPtr ptr = addr_direct_indirect_long();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.compare_with_acc(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.compare_with_acc(arg);
    }
}

void Ricoh5A22::opC9_CMP(void) {
    if (state_.is_status_memory()) {
        const Byte arg = addr_immediate8();
        alu_.compare_with_acc(arg);
    } else {
        const Word arg = addr_immediate16();
        alu_.compare_with_acc(arg);
    }
}

void Ricoh5A22::opCD_CMP(void) {
    const SnesPtr ptr = addr_absolute();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.compare_with_acc(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.compare_with_acc(arg);
    }
}

void Ricoh5A22::opCF_CMP(void) {
    const SnesPtr ptr = addr_absolute_long();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.compare_with_acc(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.compare_with_acc(arg);
    }
}

void Ricoh5A22::opD1_CMP(void) {
    const SnesPtr ptr = addr_direct_indirect_indexed();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.compare_with_acc(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.compare_with_acc(arg);
    }
}

void Ricoh5A22::opD2_CMP(void) {
    const SnesPtr ptr = addr_direct_indirect();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.compare_with_acc(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.compare_with_acc(arg);
    }
}

void Ricoh5A22::opD3_CMP(void) {
    const SnesPtr ptr = addr_stack_relative_indirect_indexed();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.compare_with_acc(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.compare_with_acc(arg);
    }
}

void Ricoh5A22::opD5_CMP(void) {
    const SnesPtr ptr = addr_direct_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.compare_with_acc(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.compare_with_acc(arg);
    }
}

void Ricoh5A22::opD7_CMP(void) {
    const SnesPtr ptr = addr_direct_indirect_indexed_long();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.compare_with_acc(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.compare_with_acc(arg);
    }
}

void Ricoh5A22::opD9_CMP(void) {
    const SnesPtr ptr = addr_absolute_indexed_y();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.compare_with_acc(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.compare_with_acc(arg);
    }
}

void Ricoh5A22::opDD_CMP(void) {
    const SnesPtr ptr = addr_absolute_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.compare_with_acc(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.compare_with_acc(arg);
    }
}

void Ricoh5A22::opDF_CMP(void) {
    const SnesPtr ptr = addr_absolute_long_indexed();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.compare_with_acc(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.compare_with_acc(arg);
    }
}

void Ricoh5A22::op02_COP(void){
    // Not implemented
}

void Ricoh5A22::opE0_CPX(void) {
    if (state_.is_status_index()) {
        const Byte arg = addr_immediate8();
        alu_.compare_with_x(arg);
    } else {
        const Word arg = addr_immediate16();
        alu_.compare_with_x(arg);
    }
}

void Ricoh5A22::opE4_CPX(void) {
    const SnesPtr ptr = addr_direct();
    if (state_.is_status_index()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.compare_with_x(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.compare_with_x(arg);
    }
}

void Ricoh5A22::opEC_CPX(void) {
    const SnesPtr ptr = addr_absolute();
    if (state_.is_status_index()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.compare_with_x(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.compare_with_x(arg);
    }
}

void Ricoh5A22::opC0_CPY(void) {
    if (state_.is_status_index()) {
        const Byte arg = addr_immediate8();
        alu_.compare_with_y(arg);
    } else {
        const Word arg = addr_immediate16();
        alu_.compare_with_y(arg);
    }
}

void Ricoh5A22::opC4_CPY(void) {
    const SnesPtr ptr = addr_direct();
    if (state_.is_status_index()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.compare_with_y(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.compare_with_y(arg);
    }
}

void Ricoh5A22::opCC_CPY(void) {
    const SnesPtr ptr = addr_absolute();
    if (state_.is_status_index()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.compare_with_y(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.compare_with_y(arg);
    }
}

void Ricoh5A22::op3A_DEC(void) {
    if (state_.is_status_memory()) {
        const Byte arg = state_.acc8();
        const Byte result = alu_.decrement(arg);
        state_.set_acc(result);
    } else {
        const Word arg = state_.acc16();
        const Word result = alu_.decrement(arg);
        state_.set_acc(result);
    }
}

void Ricoh5A22::opC6_DEC(void) {
    const SnesPtr ptr = addr_direct();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.decrement(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.decrement(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::opCE_DEC(void) {
    const SnesPtr ptr = addr_absolute();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.decrement(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.decrement(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::opD6_DEC(void) {
    const SnesPtr ptr = addr_direct_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.decrement(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.decrement(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::opDE_DEC(void) {
    const SnesPtr ptr = addr_absolute_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.decrement(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.decrement(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::opCA_DEX(void) {
    if (state_.is_status_index()) {
        const Byte arg = state_.x8();
        const Byte result = alu_.decrement(arg);
        state_.set_x(result);
    } else {
        const Word arg = state_.x16();
        const Word result = alu_.decrement(arg);
        state_.set_x(result);
    }
}

void Ricoh5A22::op88_DEY(void) {
    if (state_.is_status_index()) {
        const Byte arg = state_.y8();
        const Byte result = alu_.decrement(arg);
        state_.set_y(result);
    } else {
        const Word arg = state_.y16();
        const Word result = alu_.decrement(arg);
        state_.set_y(result);
    }
}

void Ricoh5A22::op41_EOR(void) {
    const SnesPtr ptr = addr_direct_indexed_indirect();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.exclusive_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.exclusive_or(arg);
    }
}

void Ricoh5A22::op43_EOR(void) {
    const SnesPtr ptr = addr_stack_relative();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.exclusive_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.exclusive_or(arg);
    }
}

void Ricoh5A22::op45_EOR(void) {
    const SnesPtr ptr = addr_direct();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.exclusive_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.exclusive_or(arg);
    }
}

void Ricoh5A22::op47_EOR(void) {
    const SnesPtr ptr = addr_direct_indirect_long();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.exclusive_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.exclusive_or(arg);
    }
}

void Ricoh5A22::op49_EOR(void) {
    if (state_.is_status_memory()) {
        const Byte arg = addr_immediate8();
        alu_.exclusive_or(arg);
    } else {
        const Word arg = addr_immediate16();
        alu_.exclusive_or(arg);
    }
}

void Ricoh5A22::op4D_EOR(void) {
    const SnesPtr ptr = addr_absolute();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.exclusive_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.exclusive_or(arg);
    }
}

void Ricoh5A22::op4F_EOR(void) {
    const SnesPtr ptr = addr_absolute_long();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.exclusive_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.exclusive_or(arg);
    }
}

void Ricoh5A22::op51_EOR(void) {
    const SnesPtr ptr = addr_direct_indirect_indexed();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.exclusive_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.exclusive_or(arg);
    }
}

void Ricoh5A22::op52_EOR(void) {
    const SnesPtr ptr = addr_direct_indirect();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.exclusive_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.exclusive_or(arg);
    }
}

void Ricoh5A22::op53_EOR(void) {
    const SnesPtr ptr = addr_stack_relative_indirect_indexed();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.exclusive_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.exclusive_or(arg);
    }
}

void Ricoh5A22::op55_EOR(void) {
    const SnesPtr ptr = addr_direct_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.exclusive_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.exclusive_or(arg);
    }
}

void Ricoh5A22::op57_EOR(void) {
    const SnesPtr ptr = addr_direct_indirect_indexed_long();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.exclusive_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.exclusive_or(arg);
    }
}

void Ricoh5A22::op59_EOR(void) {
    const SnesPtr ptr = addr_absolute_indexed_y();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.exclusive_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.exclusive_or(arg);
    }
}

void Ricoh5A22::op5D_EOR(void) {
    const SnesPtr ptr = addr_absolute_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.exclusive_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.exclusive_or(arg);
    }
}

void Ricoh5A22::op5F_EOR(void) {
    const SnesPtr ptr = addr_absolute_long_indexed();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.exclusive_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.exclusive_or(arg);
    }
}

void Ricoh5A22::op1A_INC(void) {
    if (state_.is_status_memory()) {
        const Byte arg = state_.acc8();
        const Byte result = alu_.increment(arg);
        state_.set_acc(result);
    } else {
        const Word arg = state_.acc16();
        const Word result = alu_.increment(arg);
        state_.set_acc(result);
    }
}

void Ricoh5A22::opE6_INC(void) {
    const SnesPtr ptr = addr_direct();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.increment(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.increment(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::opEE_INC(void) {
    const SnesPtr ptr = addr_absolute();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.increment(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.increment(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::opF6_INC(void) {
    const SnesPtr ptr = addr_direct_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.increment(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.increment(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::opFE_INC(void) {
    const SnesPtr ptr = addr_absolute_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.increment(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.increment(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::opE8_INX(void) {
    if (state_.is_status_index()) {
        const Byte arg = state_.x8();
        const Byte result = alu_.increment(arg);
        state_.set_x(result);
    } else {
        const Word arg = state_.x16();
        const Word result = alu_.increment(arg);
        state_.set_x(result);
    }
}

void Ricoh5A22::opC8_INY(void) {
    if (state_.is_status_index()) {
        const Byte arg = state_.y8();
        const Byte result = alu_.increment(arg);
        state_.set_y(result);
    } else {
        const Word arg = state_.y16();
        const Word result = alu_.increment(arg);
        state_.set_y(result);
    }
}

void Ricoh5A22::op4C_JMP(void){
    // Not implemented
}

void Ricoh5A22::op5C_JMP(void){
    // Not implemented
}

void Ricoh5A22::op6C_JMP(void){
    // Not implemented
}

void Ricoh5A22::op7C_JMP(void){
    // Not implemented
}

void Ricoh5A22::opDC_JMP(void){
    // Not implemented
}

void Ricoh5A22::op20_JSR(void){
    // Not implemented
}

void Ricoh5A22::op22_JSR(void){
    // Not implemented
}

void Ricoh5A22::opFC_JSR(void){
    // Not implemented
}

void Ricoh5A22::opA1_LDA(void){
    // Not implemented
}

void Ricoh5A22::opA3_LDA(void){
    // Not implemented
}

void Ricoh5A22::opA5_LDA(void){
    // Not implemented
}

void Ricoh5A22::opA7_LDA(void){
    // Not implemented
}

void Ricoh5A22::opA9_LDA(void){
    // Not implemented
}

void Ricoh5A22::opAD_LDA(void){
    // Not implemented
}

void Ricoh5A22::opAF_LDA(void){
    // Not implemented
}

void Ricoh5A22::opB1_LDA(void){
    // Not implemented
}

void Ricoh5A22::opB2_LDA(void){
    // Not implemented
}

void Ricoh5A22::opB3_LDA(void){
    // Not implemented
}

void Ricoh5A22::opB5_LDA(void){
    // Not implemented
}

void Ricoh5A22::opB7_LDA(void){
    // Not implemented
}

void Ricoh5A22::opB9_LDA(void){
    // Not implemented
}

void Ricoh5A22::opBD_LDA(void){
    // Not implemented
}

void Ricoh5A22::opBF_LDA(void){
    // Not implemented
}

void Ricoh5A22::opA2_LDX(void){
    // Not implemented
}

void Ricoh5A22::opA6_LDX(void){
    // Not implemented
}

void Ricoh5A22::opAE_LDX(void){
    // Not implemented
}

void Ricoh5A22::opB6_LDX(void){
    // Not implemented
}

void Ricoh5A22::opBE_LDX(void){
    // Not implemented
}

void Ricoh5A22::opA0_LDY(void){
    // Not implemented
}

void Ricoh5A22::opA4_LDY(void){
    // Not implemented
}

void Ricoh5A22::opAC_LDY(void){
    // Not implemented
}

void Ricoh5A22::opB4_LDY(void){
    // Not implemented
}

void Ricoh5A22::opBC_LDY(void){
    // Not implemented
}

void Ricoh5A22::op46_LSR(void) {
    const SnesPtr ptr = addr_direct();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.logical_shift_right(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.logical_shift_right(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::op4A_LSR(void) {
    if (state_.is_status_memory()) {
        const Byte arg = state_.acc8();
        const Byte result = alu_.logical_shift_right(arg);
        state_.set_acc(result);
    } else {
        const Word arg = state_.acc16();
        const Word result = alu_.logical_shift_right(arg);
        state_.set_acc(result);
    }
}

void Ricoh5A22::op4E_LSR(void) {
    const SnesPtr ptr = addr_absolute();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.logical_shift_right(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.logical_shift_right(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::op56_LSR(void) {
    const SnesPtr ptr = addr_direct_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.logical_shift_right(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.logical_shift_right(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::op5E_LSR(void) {
    const SnesPtr ptr = addr_absolute_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.logical_shift_right(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.logical_shift_right(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::op54_MVN(void){
    // Not implemented
}

void Ricoh5A22::op44_MVP(void){
    // Not implemented
}

void Ricoh5A22::opEA_NOP(void){
    // Not implemented
}

void Ricoh5A22::op01_ORA(void) {
    const SnesPtr ptr = addr_direct_indexed_indirect();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_or(arg);
    }
}

void Ricoh5A22::op03_ORA(void) {
    const SnesPtr ptr = addr_stack_relative();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_or(arg);
    }
}

void Ricoh5A22::op05_ORA(void) {
    const SnesPtr ptr = addr_direct();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_or(arg);
    }
}

void Ricoh5A22::op07_ORA(void) {
    const SnesPtr ptr = addr_direct_indirect_long();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_or(arg);
    }
}

void Ricoh5A22::op09_ORA(void) {
    if (state_.is_status_memory()) {
        const Byte arg = addr_immediate8();
        alu_.logic_or(arg);
    } else {
        const Word arg = addr_immediate16();
        alu_.logic_or(arg);
    }
}

void Ricoh5A22::op0D_ORA(void) {
    const SnesPtr ptr = addr_absolute();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_or(arg);
    }
}

void Ricoh5A22::op0F_ORA(void) {
    const SnesPtr ptr = addr_absolute_long();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_or(arg);
    }
}

void Ricoh5A22::op11_ORA(void) {
    const SnesPtr ptr = addr_direct_indirect_indexed();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_or(arg);
    }
}

void Ricoh5A22::op12_ORA(void) {
    const SnesPtr ptr = addr_direct_indirect();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_or(arg);
    }
}

void Ricoh5A22::op13_ORA(void) {
    const SnesPtr ptr = addr_stack_relative_indirect_indexed();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_or(arg);
    }
}

void Ricoh5A22::op15_ORA(void) {
    const SnesPtr ptr = addr_direct_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_or(arg);
    }
}

void Ricoh5A22::op17_ORA(void) {
    const SnesPtr ptr = addr_direct_indirect_indexed_long();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_or(arg);
    }
}

void Ricoh5A22::op19_ORA(void) {
    const SnesPtr ptr = addr_absolute_indexed_y();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_or(arg);
    }
}

void Ricoh5A22::op1D_ORA(void) {
    const SnesPtr ptr = addr_absolute_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_or(arg);
    }
}

void Ricoh5A22::op1F_ORA(void) {
    const SnesPtr ptr = addr_absolute_long_indexed();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.logic_or(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.logic_or(arg);
    }
}

void Ricoh5A22::opF4_PEA(void){
    // Not implemented
}

void Ricoh5A22::opD4_PEI(void){
    // Not implemented
}

void Ricoh5A22::op62_PER(void){
    // Not implemented
}

void Ricoh5A22::op48_PHA(void){
    // Not implemented
}

void Ricoh5A22::op8B_PHB(void){
    // Not implemented
}

void Ricoh5A22::op0B_PHD(void){
    // Not implemented
}

void Ricoh5A22::op4B_PHK(void){
    // Not implemented
}

void Ricoh5A22::op08_PHP(void){
    // Not implemented
}

void Ricoh5A22::opDA_PHX(void){
    // Not implemented
}

void Ricoh5A22::op5A_PHY(void){
    // Not implemented
}

void Ricoh5A22::op68_PLA(void){
    // Not implemented
}

void Ricoh5A22::opAB_PLB(void){
    // Not implemented
}

void Ricoh5A22::op2B_PLD(void){
    // Not implemented
}

void Ricoh5A22::op28_PLP(void){
    // Not implemented
}

void Ricoh5A22::opFA_PLX(void){
    // Not implemented
}

void Ricoh5A22::op7A_PLY(void){
    // Not implemented
}

void Ricoh5A22::opC2_REP(void){
    // Not implemented
}

void Ricoh5A22::op26_ROL(void) {
    const SnesPtr ptr = addr_direct();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.rotate_left(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.rotate_left(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::op2A_ROL(void) {
    if (state_.is_status_memory()) {
        const Byte arg = state_.acc8();
        const Byte result = alu_.rotate_left(arg);
        state_.set_acc(result);
    } else {
        const Word arg = state_.acc16();
        const Word result = alu_.rotate_left(arg);
        state_.set_acc(result);
    }
}

void Ricoh5A22::op2E_ROL(void) {
    const SnesPtr ptr = addr_absolute();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.rotate_left(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.rotate_left(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::op36_ROL(void) {
    const SnesPtr ptr = addr_direct_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.rotate_left(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.rotate_left(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::op3E_ROL(void) {
    const SnesPtr ptr = addr_absolute_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.rotate_left(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.rotate_left(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::op66_ROR(void) {
    const SnesPtr ptr = addr_direct();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.rotate_right(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.rotate_right(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::op6A_ROR(void) {
    if (state_.is_status_memory()) {
        const Byte arg = state_.acc8();
        const Byte result = alu_.rotate_right(arg);
        state_.set_acc(result);
    } else {
        const Word arg = state_.acc16();
        const Word result = alu_.rotate_right(arg);
        state_.set_acc(result);
    }
}

void Ricoh5A22::op6E_ROR(void) {
    const SnesPtr ptr = addr_absolute();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.rotate_right(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.rotate_right(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::op76_ROR(void) {
    const SnesPtr ptr = addr_direct_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.rotate_right(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.rotate_right(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::op7E_ROR(void) {
    const SnesPtr ptr = addr_absolute_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.rotate_right(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.rotate_right(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::op40_RTI(void){
    // Not implemented
}

void Ricoh5A22::op6B_RTL(void){
    // Not implemented
}

void Ricoh5A22::op60_RTS(void){
    // Not implemented
}

void Ricoh5A22::opE1_SBC(void) {
    const SnesPtr ptr = addr_direct_indexed_indirect();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.subtract_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.subtract_with_carry(arg);
    }
}

void Ricoh5A22::opE3_SBC(void) {
    const SnesPtr ptr = addr_stack_relative();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.subtract_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.subtract_with_carry(arg);
    }
}

void Ricoh5A22::opE5_SBC(void) {
    const SnesPtr ptr = addr_direct();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.subtract_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.subtract_with_carry(arg);
    }
}

void Ricoh5A22::opE7_SBC(void) {
    const SnesPtr ptr = addr_direct_indirect_long();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.subtract_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.subtract_with_carry(arg);
    }
}

void Ricoh5A22::opE9_SBC(void) {
    if (state_.is_status_memory()) {
        const Byte arg = addr_immediate8();
        alu_.subtract_with_carry(arg);
    } else {
        const Word arg = addr_immediate16();
        alu_.subtract_with_carry(arg);
    }
}

void Ricoh5A22::opED_SBC(void) {
    const SnesPtr ptr = addr_absolute();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.subtract_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.subtract_with_carry(arg);
    }
}

void Ricoh5A22::opEF_SBC(void) {
    const SnesPtr ptr = addr_absolute_long();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.subtract_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.subtract_with_carry(arg);
    }
}

void Ricoh5A22::opF1_SBC(void) {
    const SnesPtr ptr = addr_direct_indirect_indexed();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.subtract_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.subtract_with_carry(arg);
    }
}

void Ricoh5A22::opF2_SBC(void) {
    const SnesPtr ptr = addr_direct_indirect();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.subtract_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.subtract_with_carry(arg);
    }
}

void Ricoh5A22::opF3_SBC(void) {
    const SnesPtr ptr = addr_stack_relative_indirect_indexed();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.subtract_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.subtract_with_carry(arg);
    }
}

void Ricoh5A22::opF5_SBC(void) {
    const SnesPtr ptr = addr_direct_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.subtract_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.subtract_with_carry(arg);
    }
}

void Ricoh5A22::opF7_SBC(void) {
    const SnesPtr ptr = addr_direct_indirect_indexed_long();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.subtract_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.subtract_with_carry(arg);
    }
}

void Ricoh5A22::opF9_SBC(void) {
    const SnesPtr ptr = addr_absolute_indexed_y();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.subtract_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.subtract_with_carry(arg);
    }
}

void Ricoh5A22::opFD_SBC(void) {
    const SnesPtr ptr = addr_absolute_indexed_x();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.subtract_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.subtract_with_carry(arg);
    }
}

void Ricoh5A22::opFF_SBC(void) {
    const SnesPtr ptr = addr_absolute_long_indexed();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        alu_.subtract_with_carry(arg);
    } else {
        const Word arg = mem_.get_word(ptr);
        alu_.subtract_with_carry(arg);
    }
}

void Ricoh5A22::op38_SEC(void){
    // Not implemented
}

void Ricoh5A22::opF8_SED(void){
    // Not implemented
}

void Ricoh5A22::op78_SEI(void){
    // Not implemented
}

void Ricoh5A22::opE2_SEP(void){
    // Not implemented
}

void Ricoh5A22::op81_STA(void){
    // Not implemented
}

void Ricoh5A22::op83_STA(void){
    // Not implemented
}

void Ricoh5A22::op85_STA(void){
    // Not implemented
}

void Ricoh5A22::op87_STA(void){
    // Not implemented
}

void Ricoh5A22::op8D_STA(void){
    // Not implemented
}

void Ricoh5A22::op8F_STA(void){
    // Not implemented
}

void Ricoh5A22::op91_STA(void){
    // Not implemented
}

void Ricoh5A22::op92_STA(void){
    // Not implemented
}

void Ricoh5A22::op93_STA(void){
    // Not implemented
}

void Ricoh5A22::op95_STA(void){
    // Not implemented
}

void Ricoh5A22::op97_STA(void){
    // Not implemented
}

void Ricoh5A22::op99_STA(void){
    // Not implemented
}

void Ricoh5A22::op9D_STA(void){
    // Not implemented
}

void Ricoh5A22::op9F_STA(void){
    // Not implemented
}

void Ricoh5A22::opDB_STP(void){
    // Not implemented
}

void Ricoh5A22::op86_STX(void){
    // Not implemented
}

void Ricoh5A22::op8E_STX(void){
    // Not implemented
}

void Ricoh5A22::op96_STX(void){
    // Not implemented
}

void Ricoh5A22::op84_STY(void){
    // Not implemented
}

void Ricoh5A22::op8C_STY(void){
    // Not implemented
}

void Ricoh5A22::op94_STY(void){
    // Not implemented
}

void Ricoh5A22::op64_STZ(void){
    // Not implemented
}

void Ricoh5A22::op74_STZ(void){
    // Not implemented
}

void Ricoh5A22::op9C_STZ(void){
    // Not implemented
}

void Ricoh5A22::op9E_STZ(void){
    // Not implemented
}

void Ricoh5A22::opAA_TAX(void){
    // Not implemented
}

void Ricoh5A22::opA8_TAY(void){
    // Not implemented
}

void Ricoh5A22::op5B_TCD(void){
    // Not implemented
}

void Ricoh5A22::op1B_TCS(void){
    // Not implemented
}

void Ricoh5A22::op7B_TDC(void){
    // Not implemented
}

void Ricoh5A22::op14_TRB(void) {
    const SnesPtr ptr = addr_direct();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.test_and_reset(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.test_and_reset(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::op1C_TRB(void) {
    const SnesPtr ptr = addr_absolute();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.test_and_reset(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.test_and_reset(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::op04_TSB(void) {
    const SnesPtr ptr = addr_direct();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.test_and_set(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.test_and_set(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::op0C_TSB(void) {
    const SnesPtr ptr = addr_absolute();
    if (state_.is_status_memory()) {
        const Byte arg = mem_.get_byte(ptr);
        const Byte result = alu_.test_and_set(arg);
        mem_.write(ptr, result);
    } else {
        const Word arg = mem_.get_word(ptr);
        const Word result = alu_.test_and_set(arg);
        mem_.write(ptr, result);
    }
}

void Ricoh5A22::op3B_TSC(void){
    // Not implemented
}

void Ricoh5A22::opBA_TSX(void){
    // Not implemented
}

void Ricoh5A22::op8A_TXA(void){
    // Not implemented
}

void Ricoh5A22::op9A_TXS(void){
    // Not implemented
}

void Ricoh5A22::op9B_TXY(void){
    // Not implemented
}

void Ricoh5A22::op98_TYA(void){
    // Not implemented
}

void Ricoh5A22::opBB_TYX(void){
    // Not implemented
}

void Ricoh5A22::opCB_WAI(void){
    // Not implemented
}

void Ricoh5A22::op42_WDM(void){
    // Not implemented
}

void Ricoh5A22::opEB_XBA(void){
    // Not implemented
}

void Ricoh5A22::opFB_XCE(void){
    // Not implemented
}

}