#include "ricoh5a22.h"
    
namespace snes
{
    Word Ricoh5A22::addr_immediate16(void) {
        const Word result = mem_.get_word(state_.pc());
        state_.pc() += 2;
        return result;
    }

    Byte Ricoh5A22::addr_immediate8(void) {
        const Byte result = mem_.get_byte(state_.pc());
        state_.pc() += 1;
        return result;
    }

    SnesPtr Ricoh5A22::addr_absolute(void) {
        SnesPtr result = state_.dbr() << sizeof(Word);
        result |= addr_immediate16();
        return result;
    }

    SnesPtr Ricoh5A22::addr_absolute_indexed_x(void) {
        SnesPtr result = addr_absolute();
        result += state_.x16();
        return result;
    }

    SnesPtr Ricoh5A22::addr_absolute_indexed_y(void) {
        SnesPtr result = addr_absolute();
        result += state_.y16();
        return result;
    }


    SnesPtr Ricoh5A22::addr_absolute_long(void) {
        SnesPtr result = 0;
        const int kThreeBytes = 3;
        mem_.get(&result, state_.pc(), kThreeBytes);
        return result;
    }

    SnesPtr Ricoh5A22::addr_absolute_long_indexed(void) {
        SnesPtr result = addr_absolute_long();
        result += state_.x16();
        return result;
    }

    SnesPtr Ricoh5A22::addr_absolute_indirect(void) {
        SnesPtr result = addr_immediate16();
        result = mem_.get_word(result);
        return result;
    }

    SnesPtr Ricoh5A22::addr_absolute_indexed_indirect(void) {
        SnesPtr result = addr_immediate16();
        result += state_.x16();
        result = mem_.get_word(result);
        return result;
    }


    SnesPtr Ricoh5A22::addr_direct(void) {
        SnesPtr result = addr_immediate8();
        result += state_.dp();
        return result;
    }

    SnesPtr Ricoh5A22::addr_direct_indirect(void) {
        SnesPtr result = addr_direct();
        result = mem_.get_word(result);
        result |= state_.dbr() << sizeof(Word);
        return result;
    }

    SnesPtr Ricoh5A22::addr_direct_indirect_long(void) {
        SnesPtr result = addr_direct();
        const int kThreeBytes = 3;
        mem_.get(&result, result, kThreeBytes);
        return result;
    }

    SnesPtr Ricoh5A22::addr_direct_indirect_indexed(void) {
        SnesPtr result = addr_direct_indirect();
        result += state_.y16();
        return result;
    }

    SnesPtr Ricoh5A22::addr_direct_indirect_indexed_long(void) {
        SnesPtr result = addr_direct_indirect_long();
        result += state_.y16();
        return result;
    }


    SnesPtr Ricoh5A22::addr_direct_indexed_x(void) {
        SnesPtr result = addr_direct();
        result += state_.x16();
        return result;
    }

    SnesPtr Ricoh5A22::addr_direct_indexed_y(void) {
        SnesPtr result = addr_direct();
        result += state_.y16();
        return result;
    }

    SnesPtr Ricoh5A22::addr_direct_indexed_indirect(void) {
        SnesPtr result =  addr_direct_indexed_x();
        result = mem_.get_word(result);
        result |= state_.dbr() << sizeof(Word);
        return result;
    }

    SnesPtr Ricoh5A22::addr_pc_relative(void) {
        SnesPtr result = addr_immediate8();
        result += state_.pc();
        result &= 0xFFFF;
        return result;
    }

    SnesPtr Ricoh5A22::addr_pc_relative_long(void) {
        SnesPtr result = addr_immediate16();
        result += state_.pc();
        result &= 0xFFFF;
        return result;
    }


    SnesPtr Ricoh5A22::addr_stack_relative(void) {
        SnesPtr result = addr_immediate8();
        result += state_.stack();
        return result;
    }

    SnesPtr Ricoh5A22::addr_stack_relative_indirect_indexed(void) {
        SnesPtr result = addr_stack_relative();
        result |= state_.dbr() << sizeof(Word);
        result += state_.y16();
        return result;
    }
} // namespace snes