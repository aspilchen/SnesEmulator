#include "ricoh5a22.h"

namespace snes
{
    void Ricoh5A22::op61_ADC(void)
    {
        if (verbose_)
        {
            std::cout << "ADC ";
        }
        const SnesPtr ptr = addr_direct_indexed_indirect();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.add_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.add_with_carry(arg);
        }
    }

    void Ricoh5A22::op63_ADC(void)
    {
        if (verbose_)
        {
            std::cout << "ADC ";
        }
        const SnesPtr ptr = addr_stack_relative();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.add_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.add_with_carry(arg);
        }
    }

    void Ricoh5A22::op65_ADC(void)
    {
        if (verbose_)
        {
            std::cout << "ADC ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.add_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.add_with_carry(arg);
        }
    }

    void Ricoh5A22::op67_ADC(void)
    {
        if (verbose_)
        {
            std::cout << "ADC ";
        }
        const SnesPtr ptr = addr_direct_indirect_long();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.add_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.add_with_carry(arg);
        }
    }

    void Ricoh5A22::op69_ADC(void)
    {
        if (verbose_)
        {
            std::cout << "ADC ";
        }
        if (state_.is_status_memory())
        {
            const Byte arg = addr_immediate8();
            alu_.add_with_carry(arg);
        }
        else
        {
            const Word arg = addr_immediate16();
            alu_.add_with_carry(arg);
        }
    }

    void Ricoh5A22::op6D_ADC(void)
    {
        if (verbose_)
        {
            std::cout << "ADC ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.add_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.add_with_carry(arg);
        }
    }

    void Ricoh5A22::op6F_ADC(void)
    {
        if (verbose_)
        {
            std::cout << "ADC ";
        }
        const SnesPtr ptr = addr_absolute_long();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.add_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.add_with_carry(arg);
        }
    }

    void Ricoh5A22::op71_ADC(void)
    {
        if (verbose_)
        {
            std::cout << "ADC ";
        }
        const SnesPtr ptr = addr_direct_indirect_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.add_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.add_with_carry(arg);
        }
    }

    void Ricoh5A22::op72_ADC(void)
    {
        if (verbose_)
        {
            std::cout << "ADC ";
        }
        const SnesPtr ptr = addr_direct_indirect();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.add_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.add_with_carry(arg);
        }
    }

    void Ricoh5A22::op73_ADC(void)
    {
        if (verbose_)
        {
            std::cout << "ADC ";
        }
        const SnesPtr ptr = addr_stack_relative_indirect_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.add_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.add_with_carry(arg);
        }
    }

    void Ricoh5A22::op75_ADC(void)
    {
        if (verbose_)
        {
            std::cout << "ADC ";
        }
        const SnesPtr ptr = addr_direct_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.add_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.add_with_carry(arg);
        }
    }

    void Ricoh5A22::op77_ADC(void)
    {
        if (verbose_)
        {
            std::cout << "ADC ";
        }
        const SnesPtr ptr = addr_direct_indirect_indexed_long();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.add_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.add_with_carry(arg);
        }
    }

    void Ricoh5A22::op79_ADC(void)
    {
        if (verbose_)
        {
            std::cout << "ADC ";
        }
        const SnesPtr ptr = addr_absolute_indexed_y();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.add_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.add_with_carry(arg);
        }
    }

    void Ricoh5A22::op7D_ADC(void)
    {
        if (verbose_)
        {
            std::cout << "ADC ";
        }
        const SnesPtr ptr = addr_absolute_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.add_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.add_with_carry(arg);
        }
    }

    void Ricoh5A22::op7F_ADC(void)
    {
        if (verbose_)
        {
            std::cout << "ADC ";
        }
        const SnesPtr ptr = addr_absolute_long_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.add_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.add_with_carry(arg);
        }
    }

    void Ricoh5A22::op21_AND(void)
    {
        if (verbose_)
        {
            std::cout << "AND ";
        }
        const SnesPtr ptr = addr_direct_indexed_indirect();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_and(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_and(arg);
        }
    }

    void Ricoh5A22::op23_AND(void)
    {
        if (verbose_)
        {
            std::cout << "AND ";
        }
        const SnesPtr ptr = addr_stack_relative();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_and(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_and(arg);
        }
    }

    void Ricoh5A22::op25_AND(void)
    {
        if (verbose_)
        {
            std::cout << "AND ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_and(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_and(arg);
        }
    }

    void Ricoh5A22::op27_AND(void)
    {
        if (verbose_)
        {
            std::cout << "AND ";
        }
        const SnesPtr ptr = addr_direct_indirect_long();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_and(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_and(arg);
        }
    }

    void Ricoh5A22::op29_AND(void)
    {
        if (verbose_)
        {
            std::cout << "AND ";
        }
        if (state_.is_status_memory())
        {
            const Byte arg = addr_immediate8();
            alu_.logic_and(arg);
        }
        else
        {
            const Word arg = addr_immediate16();
            alu_.logic_and(arg);
        }
    }

    void Ricoh5A22::op2D_AND(void)
    {
        if (verbose_)
        {
            std::cout << "AND ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_and(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_and(arg);
        }
    }

    void Ricoh5A22::op2F_AND(void)
    {
        if (verbose_)
        {
            std::cout << "AND ";
        }
        const SnesPtr ptr = addr_absolute_long();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_and(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_and(arg);
        }
    }

    void Ricoh5A22::op31_AND(void)
    {
        if (verbose_)
        {
            std::cout << "AND ";
        }
        const SnesPtr ptr = addr_direct_indirect_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_and(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_and(arg);
        }
    }

    void Ricoh5A22::op32_AND(void)
    {
        if (verbose_)
        {
            std::cout << "AND ";
        }
        const SnesPtr ptr = addr_direct_indirect();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_and(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_and(arg);
        }
    }

    void Ricoh5A22::op33_AND(void)
    {
        if (verbose_)
        {
            std::cout << "AND ";
        }
        const SnesPtr ptr = addr_stack_relative_indirect_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_and(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_and(arg);
        }
    }

    void Ricoh5A22::op35_AND(void)
    {
        if (verbose_)
        {
            std::cout << "AND ";
        }
        const SnesPtr ptr = addr_direct_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_and(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_and(arg);
        }
    }

    void Ricoh5A22::op37_AND(void)
    {
        if (verbose_)
        {
            std::cout << "AND ";
        }
        const SnesPtr ptr = addr_direct_indirect_indexed_long();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_and(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_and(arg);
        }
    }

    void Ricoh5A22::op39_AND(void)
    {
        if (verbose_)
        {
            std::cout << "AND ";
        }
        const SnesPtr ptr = addr_absolute_indexed_y();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_and(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_and(arg);
        }
    }

    void Ricoh5A22::op3D_AND(void)
    {
        if (verbose_)
        {
            std::cout << "AND ";
        }
        const SnesPtr ptr = addr_absolute_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_and(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_and(arg);
        }
    }

    void Ricoh5A22::op3F_AND(void)
    {
        if (verbose_)
        {
            std::cout << "AND ";
        }
        const SnesPtr ptr = addr_absolute_long_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_and(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_and(arg);
        }
    }

    void Ricoh5A22::op06_ASL(void)
    {
        if (verbose_)
        {
            std::cout << "ASL ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.arith_shift_left(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.arith_shift_left(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::op0A_ASL(void)
    {
        if (verbose_)
        {
            std::cout << "ASL ";
        }
        if (state_.is_status_memory())
        {
            const Byte arg = state_.acc8();
            const Byte result = alu_.arith_shift_left(arg);
            state_.set_acc(result);
        }
        else
        {
            const Word arg = state_.acc16();
            const Word result = alu_.arith_shift_left(arg);
            state_.set_acc(result);
        }
    }

    void Ricoh5A22::op0E_ASL(void)
    {
        if (verbose_)
        {
            std::cout << "ASL ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.arith_shift_left(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.arith_shift_left(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::op16_ASL(void)
    {
        if (verbose_)
        {
            std::cout << "ASL ";
        }
        const SnesPtr ptr = addr_direct_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.arith_shift_left(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.arith_shift_left(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::op1E_ASL(void)
    {
        if (verbose_)
        {
            std::cout << "ASL ";
        }
        const SnesPtr ptr = addr_absolute_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.arith_shift_left(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.arith_shift_left(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::op24_BIT(void)
    {
        if (verbose_)
        {
            std::cout << "BIT ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.bit(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.bit(arg);
        }
    }

    void Ricoh5A22::op2C_BIT(void)
    {
        if (verbose_)
        {
            std::cout << "BIT ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.bit(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.bit(arg);
        }
    }

    void Ricoh5A22::op34_BIT(void)
    {
        if (verbose_)
        {
            std::cout << "BIT ";
        }
        const SnesPtr ptr = addr_direct_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.bit(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.bit(arg);
        }
    }

    void Ricoh5A22::op3C_BIT(void)
    {
        if (verbose_)
        {
            std::cout << "BIT ";
        }
        const SnesPtr ptr = addr_absolute_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.bit(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.bit(arg);
        }
    }

    void Ricoh5A22::op89_BIT(void)
    {
        if (verbose_)
        {
            std::cout << "BIT ";
        }
        if (state_.is_status_memory())
        {
            const Byte arg = addr_immediate8();
            alu_.bit(arg);
        }
        else
        {
            const Word arg = addr_immediate16();
            alu_.bit(arg);
        }
    }

    void Ricoh5A22::opC1_CMP(void)
    {
        if (verbose_)
        {
            std::cout << "CMP ";
        }
        const SnesPtr ptr = addr_direct_indexed_indirect();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.compare_with_acc(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.compare_with_acc(arg);
        }
    }

    void Ricoh5A22::opC3_CMP(void)
    {
        if (verbose_)
        {
            std::cout << "CMP ";
        }
        const SnesPtr ptr = addr_stack_relative();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.compare_with_acc(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.compare_with_acc(arg);
        }
    }

    void Ricoh5A22::opC5_CMP(void)
    {
        if (verbose_)
        {
            std::cout << "CMP ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.compare_with_acc(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.compare_with_acc(arg);
        }
    }

    void Ricoh5A22::opC7_CMP(void)
    {
        if (verbose_)
        {
            std::cout << "CMP ";
        }
        const SnesPtr ptr = addr_direct_indirect_long();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.compare_with_acc(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.compare_with_acc(arg);
        }
    }

    void Ricoh5A22::opC9_CMP(void)
    {
        if (verbose_)
        {
            std::cout << "CMP ";
        }
        if (state_.is_status_memory())
        {
            const Byte arg = addr_immediate8();
            alu_.compare_with_acc(arg);
        }
        else
        {
            const Word arg = addr_immediate16();
            alu_.compare_with_acc(arg);
        }
    }

    void Ricoh5A22::opCD_CMP(void)
    {
        if (verbose_)
        {
            std::cout << "CMP ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.compare_with_acc(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.compare_with_acc(arg);
        }
    }

    void Ricoh5A22::opCF_CMP(void)
    {
        if (verbose_)
        {
            std::cout << "CMP ";
        }
        const SnesPtr ptr = addr_absolute_long();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.compare_with_acc(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.compare_with_acc(arg);
        }
    }

    void Ricoh5A22::opD1_CMP(void)
    {
        if (verbose_)
        {
            std::cout << "CMP ";
        }
        const SnesPtr ptr = addr_direct_indirect_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.compare_with_acc(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.compare_with_acc(arg);
        }
    }

    void Ricoh5A22::opD2_CMP(void)
    {
        if (verbose_)
        {
            std::cout << "CMP ";
        }
        const SnesPtr ptr = addr_direct_indirect();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.compare_with_acc(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.compare_with_acc(arg);
        }
    }

    void Ricoh5A22::opD3_CMP(void)
    {
        if (verbose_)
        {
            std::cout << "CMP ";
        }
        const SnesPtr ptr = addr_stack_relative_indirect_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.compare_with_acc(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.compare_with_acc(arg);
        }
    }

    void Ricoh5A22::opD5_CMP(void)
    {
        if (verbose_)
        {
            std::cout << "CMP ";
        }
        const SnesPtr ptr = addr_direct_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.compare_with_acc(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.compare_with_acc(arg);
        }
    }

    void Ricoh5A22::opD7_CMP(void)
    {
        if (verbose_)
        {
            std::cout << "CMP ";
        }
        const SnesPtr ptr = addr_direct_indirect_indexed_long();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.compare_with_acc(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.compare_with_acc(arg);
        }
    }

    void Ricoh5A22::opD9_CMP(void)
    {
        if (verbose_)
        {
            std::cout << "CMP ";
        }
        const SnesPtr ptr = addr_absolute_indexed_y();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.compare_with_acc(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.compare_with_acc(arg);
        }
    }

    void Ricoh5A22::opDD_CMP(void)
    {
        if (verbose_)
        {
            std::cout << "CMP ";
        }
        const SnesPtr ptr = addr_absolute_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.compare_with_acc(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.compare_with_acc(arg);
        }
    }

    void Ricoh5A22::opDF_CMP(void)
    {
        if (verbose_)
        {
            std::cout << "CMP ";
        }
        const SnesPtr ptr = addr_absolute_long_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.compare_with_acc(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.compare_with_acc(arg);
        }
    }

    void Ricoh5A22::opE0_CPX(void)
    {
        if (verbose_)
        {
            std::cout << "CPX ";
        }
        if (state_.is_status_index())
        {
            const Byte arg = addr_immediate8();
            alu_.compare_with_x(arg);
        }
        else
        {
            const Word arg = addr_immediate16();
            alu_.compare_with_x(arg);
        }
    }

    void Ricoh5A22::opE4_CPX(void)
    {
        if (verbose_)
        {
            std::cout << "CPX ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_index())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.compare_with_x(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.compare_with_x(arg);
        }
    }

    void Ricoh5A22::opEC_CPX(void)
    {
        if (verbose_)
        {
            std::cout << "CPX ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_index())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.compare_with_x(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.compare_with_x(arg);
        }
    }

    void Ricoh5A22::opC0_CPY(void)
    {
        if (verbose_)
        {
            std::cout << "CPY ";
        }
        if (state_.is_status_index())
        {
            const Byte arg = addr_immediate8();
            alu_.compare_with_y(arg);
        }
        else
        {
            const Word arg = addr_immediate16();
            alu_.compare_with_y(arg);
        }
    }

    void Ricoh5A22::opC4_CPY(void)
    {
        if (verbose_)
        {
            std::cout << "CPY ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_index())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.compare_with_y(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.compare_with_y(arg);
        }
    }

    void Ricoh5A22::opCC_CPY(void)
    {
        if (verbose_)
        {
            std::cout << "CPY ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_index())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.compare_with_y(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.compare_with_y(arg);
        }
    }

    void Ricoh5A22::op3A_DEC(void)
    {
        if (verbose_)
        {
            std::cout << "DEC ";
        }
        if (state_.is_status_memory())
        {
            const Byte arg = state_.acc8();
            const Byte result = alu_.decrement(arg);
            state_.set_acc(result);
        }
        else
        {
            const Word arg = state_.acc16();
            const Word result = alu_.decrement(arg);
            state_.set_acc(result);
        }
    }

    void Ricoh5A22::opC6_DEC(void)
    {
        if (verbose_)
        {
            std::cout << "DEC ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.decrement(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.decrement(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::opCE_DEC(void)
    {
        if (verbose_)
        {
            std::cout << "DEC ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.decrement(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.decrement(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::opD6_DEC(void)
    {
        if (verbose_)
        {
            std::cout << "DEC ";
        }
        const SnesPtr ptr = addr_direct_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.decrement(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.decrement(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::opDE_DEC(void)
    {
        if (verbose_)
        {
            std::cout << "DEC ";
        }
        const SnesPtr ptr = addr_absolute_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.decrement(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.decrement(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::opCA_DEX(void)
    {
        if (verbose_)
        {
            std::cout << "DEX ";
        }
        if (state_.is_status_index())
        {
            const Byte arg = state_.x8();
            const Byte result = alu_.decrement(arg);
            state_.set_x(result);
        }
        else
        {
            const Word arg = state_.x16();
            const Word result = alu_.decrement(arg);
            state_.set_x(result);
        }
    }

    void Ricoh5A22::op88_DEY(void)
    {
        if (verbose_)
        {
            std::cout << "DEY ";
        }
        if (state_.is_status_index())
        {
            const Byte arg = state_.y8();
            const Byte result = alu_.decrement(arg);
            state_.set_y(result);
        }
        else
        {
            const Word arg = state_.y16();
            const Word result = alu_.decrement(arg);
            state_.set_y(result);
        }
    }

    void Ricoh5A22::op41_EOR(void)
    {
        if (verbose_)
        {
            std::cout << "EOR ";
        }
        const SnesPtr ptr = addr_direct_indexed_indirect();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.exclusive_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.exclusive_or(arg);
        }
    }

    void Ricoh5A22::op43_EOR(void)
    {
        if (verbose_)
        {
            std::cout << "EOR ";
        }
        const SnesPtr ptr = addr_stack_relative();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.exclusive_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.exclusive_or(arg);
        }
    }

    void Ricoh5A22::op45_EOR(void)
    {
        if (verbose_)
        {
            std::cout << "EOR ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.exclusive_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.exclusive_or(arg);
        }
    }

    void Ricoh5A22::op47_EOR(void)
    {
        if (verbose_)
        {
            std::cout << "EOR ";
        }
        const SnesPtr ptr = addr_direct_indirect_long();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.exclusive_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.exclusive_or(arg);
        }
    }

    void Ricoh5A22::op49_EOR(void)
    {
        if (verbose_)
        {
            std::cout << "EOR ";
        }
        if (state_.is_status_memory())
        {
            const Byte arg = addr_immediate8();
            alu_.exclusive_or(arg);
        }
        else
        {
            const Word arg = addr_immediate16();
            alu_.exclusive_or(arg);
        }
    }

    void Ricoh5A22::op4D_EOR(void)
    {
        if (verbose_)
        {
            std::cout << "EOR ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.exclusive_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.exclusive_or(arg);
        }
    }

    void Ricoh5A22::op4F_EOR(void)
    {
        if (verbose_)
        {
            std::cout << "EOR ";
        }
        const SnesPtr ptr = addr_absolute_long();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.exclusive_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.exclusive_or(arg);
        }
    }

    void Ricoh5A22::op51_EOR(void)
    {
        if (verbose_)
        {
            std::cout << "EOR ";
        }
        const SnesPtr ptr = addr_direct_indirect_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.exclusive_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.exclusive_or(arg);
        }
    }

    void Ricoh5A22::op52_EOR(void)
    {
        if (verbose_)
        {
            std::cout << "EOR ";
        }
        const SnesPtr ptr = addr_direct_indirect();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.exclusive_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.exclusive_or(arg);
        }
    }

    void Ricoh5A22::op53_EOR(void)
    {
        if (verbose_)
        {
            std::cout << "EOR ";
        }
        const SnesPtr ptr = addr_stack_relative_indirect_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.exclusive_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.exclusive_or(arg);
        }
    }

    void Ricoh5A22::op55_EOR(void)
    {
        if (verbose_)
        {
            std::cout << "EOR ";
        }
        const SnesPtr ptr = addr_direct_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.exclusive_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.exclusive_or(arg);
        }
    }

    void Ricoh5A22::op57_EOR(void)
    {
        if (verbose_)
        {
            std::cout << "EOR ";
        }
        const SnesPtr ptr = addr_direct_indirect_indexed_long();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.exclusive_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.exclusive_or(arg);
        }
    }

    void Ricoh5A22::op59_EOR(void)
    {
        if (verbose_)
        {
            std::cout << "EOR ";
        }
        const SnesPtr ptr = addr_absolute_indexed_y();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.exclusive_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.exclusive_or(arg);
        }
    }

    void Ricoh5A22::op5D_EOR(void)
    {
        if (verbose_)
        {
            std::cout << "EOR ";
        }
        const SnesPtr ptr = addr_absolute_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.exclusive_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.exclusive_or(arg);
        }
    }

    void Ricoh5A22::op5F_EOR(void)
    {
        if (verbose_)
        {
            std::cout << "EOR ";
        }
        const SnesPtr ptr = addr_absolute_long_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.exclusive_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.exclusive_or(arg);
        }
    }

    void Ricoh5A22::op1A_INC(void)
    {
        if (verbose_)
        {
            std::cout << "INC ";
        }
        if (state_.is_status_memory())
        {
            const Byte arg = state_.acc8();
            const Byte result = alu_.increment(arg);
            state_.set_acc(result);
        }
        else
        {
            const Word arg = state_.acc16();
            const Word result = alu_.increment(arg);
            state_.set_acc(result);
        }
    }

    void Ricoh5A22::opE6_INC(void)
    {
        if (verbose_)
        {
            std::cout << "INC ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.increment(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.increment(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::opEE_INC(void)
    {
        if (verbose_)
        {
            std::cout << "INC ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.increment(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.increment(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::opF6_INC(void)
    {
        if (verbose_)
        {
            std::cout << "INC ";
        }
        const SnesPtr ptr = addr_direct_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.increment(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.increment(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::opFE_INC(void)
    {
        if (verbose_)
        {
            std::cout << "INC ";
        }
        const SnesPtr ptr = addr_absolute_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.increment(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.increment(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::opE8_INX(void)
    {
        if (verbose_)
        {
            std::cout << "INX ";
        }
        if (state_.is_status_index())
        {
            const Byte arg = state_.x8();
            const Byte result = alu_.increment(arg);
            state_.set_x(result);
        }
        else
        {
            const Word arg = state_.x16();
            const Word result = alu_.increment(arg);
            state_.set_x(result);
        }
    }

    void Ricoh5A22::opC8_INY(void)
    {
        if (verbose_)
        {
            std::cout << "INY ";
        }
        if (state_.is_status_index())
        {
            const Byte arg = state_.y8();
            const Byte result = alu_.increment(arg);
            state_.set_y(result);
        }
        else
        {
            const Word arg = state_.y16();
            const Word result = alu_.increment(arg);
            state_.set_y(result);
        }
    }

    void Ricoh5A22::op46_LSR(void)
    {
        if (verbose_)
        {
            std::cout << "LSR ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.logical_shift_right(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.logical_shift_right(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::op4A_LSR(void)
    {
        if (verbose_)
        {
            std::cout << "LSR ";
        }
        if (state_.is_status_memory())
        {
            const Byte arg = state_.acc8();
            const Byte result = alu_.logical_shift_right(arg);
            state_.set_acc(result);
        }
        else
        {
            const Word arg = state_.acc16();
            const Word result = alu_.logical_shift_right(arg);
            state_.set_acc(result);
        }
    }

    void Ricoh5A22::op4E_LSR(void)
    {
        if (verbose_)
        {
            std::cout << "LSR ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.logical_shift_right(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.logical_shift_right(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::op56_LSR(void)
    {
        if (verbose_)
        {
            std::cout << "LSR ";
        }
        const SnesPtr ptr = addr_direct_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.logical_shift_right(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.logical_shift_right(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::op5E_LSR(void)
    {
        if (verbose_)
        {
            std::cout << "LSR ";
        }
        const SnesPtr ptr = addr_absolute_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.logical_shift_right(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.logical_shift_right(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::opEA_NOP(void)
    {
        // Not implemented
    }

    void Ricoh5A22::op01_ORA(void)
    {
        if (verbose_)
        {
            std::cout << "ORA ";
        }
        const SnesPtr ptr = addr_direct_indexed_indirect();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_or(arg);
        }
    }

    void Ricoh5A22::op03_ORA(void)
    {
        if (verbose_)
        {
            std::cout << "ORA ";
        }
        const SnesPtr ptr = addr_stack_relative();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_or(arg);
        }
    }

    void Ricoh5A22::op05_ORA(void)
    {
        if (verbose_)
        {
            std::cout << "ORA ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_or(arg);
        }
    }

    void Ricoh5A22::op07_ORA(void)
    {
        if (verbose_)
        {
            std::cout << "ORA ";
        }
        const SnesPtr ptr = addr_direct_indirect_long();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_or(arg);
        }
    }

    void Ricoh5A22::op09_ORA(void)
    {
        if (verbose_)
        {
            std::cout << "ORA ";
        }
        if (state_.is_status_memory())
        {
            const Byte arg = addr_immediate8();
            alu_.logic_or(arg);
        }
        else
        {
            const Word arg = addr_immediate16();
            alu_.logic_or(arg);
        }
    }

    void Ricoh5A22::op0D_ORA(void)
    {
        if (verbose_)
        {
            std::cout << "ORA ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_or(arg);
        }
    }

    void Ricoh5A22::op0F_ORA(void)
    {
        if (verbose_)
        {
            std::cout << "ORA ";
        }
        const SnesPtr ptr = addr_absolute_long();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_or(arg);
        }
    }

    void Ricoh5A22::op11_ORA(void)
    {
        if (verbose_)
        {
            std::cout << "ORA ";
        }
        const SnesPtr ptr = addr_direct_indirect_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_or(arg);
        }
    }

    void Ricoh5A22::op12_ORA(void)
    {
        if (verbose_)
        {
            std::cout << "ORA ";
        }
        const SnesPtr ptr = addr_direct_indirect();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_or(arg);
        }
    }

    void Ricoh5A22::op13_ORA(void)
    {
        if (verbose_)
        {
            std::cout << "ORA ";
        }
        const SnesPtr ptr = addr_stack_relative_indirect_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_or(arg);
        }
    }

    void Ricoh5A22::op15_ORA(void)
    {
        if (verbose_)
        {
            std::cout << "ORA ";
        }
        const SnesPtr ptr = addr_direct_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_or(arg);
        }
    }

    void Ricoh5A22::op17_ORA(void)
    {
        if (verbose_)
        {
            std::cout << "ORA ";
        }
        const SnesPtr ptr = addr_direct_indirect_indexed_long();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_or(arg);
        }
    }

    void Ricoh5A22::op19_ORA(void)
    {
        if (verbose_)
        {
            std::cout << "ORA ";
        }
        const SnesPtr ptr = addr_absolute_indexed_y();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_or(arg);
        }
    }

    void Ricoh5A22::op1D_ORA(void)
    {
        if (verbose_)
        {
            std::cout << "ORA ";
        }
        const SnesPtr ptr = addr_absolute_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_or(arg);
        }
    }

    void Ricoh5A22::op1F_ORA(void)
    {
        if (verbose_)
        {
            std::cout << "ORA ";
        }
        const SnesPtr ptr = addr_absolute_long_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.logic_or(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.logic_or(arg);
        }
    }

    void Ricoh5A22::op26_ROL(void)
    {
        if (verbose_)
        {
            std::cout << "ROL ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.rotate_left(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.rotate_left(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::op2A_ROL(void)
    {
        if (verbose_)
        {
            std::cout << "ROL ";
        }
        if (state_.is_status_memory())
        {
            const Byte arg = state_.acc8();
            const Byte result = alu_.rotate_left(arg);
            state_.set_acc(result);
        }
        else
        {
            const Word arg = state_.acc16();
            const Word result = alu_.rotate_left(arg);
            state_.set_acc(result);
        }
    }

    void Ricoh5A22::op2E_ROL(void)
    {
        if (verbose_)
        {
            std::cout << "ROL ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.rotate_left(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.rotate_left(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::op36_ROL(void)
    {
        if (verbose_)
        {
            std::cout << "ROL ";
        }
        const SnesPtr ptr = addr_direct_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.rotate_left(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.rotate_left(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::op3E_ROL(void)
    {
        if (verbose_)
        {
            std::cout << "ROL ";
        }
        const SnesPtr ptr = addr_absolute_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.rotate_left(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.rotate_left(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::op66_ROR(void)
    {
        if (verbose_)
        {
            std::cout << "ROR ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.rotate_right(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.rotate_right(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::op6A_ROR(void)
    {
        if (verbose_)
        {
            std::cout << "ROR ";
        }
        if (state_.is_status_memory())
        {
            const Byte arg = state_.acc8();
            const Byte result = alu_.rotate_right(arg);
            state_.set_acc(result);
        }
        else
        {
            const Word arg = state_.acc16();
            const Word result = alu_.rotate_right(arg);
            state_.set_acc(result);
        }
    }

    void Ricoh5A22::op6E_ROR(void)
    {
        if (verbose_)
        {
            std::cout << "ROR ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.rotate_right(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.rotate_right(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::op76_ROR(void)
    {
        if (verbose_)
        {
            std::cout << "ROR ";
        }
        const SnesPtr ptr = addr_direct_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.rotate_right(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.rotate_right(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::op7E_ROR(void)
    {
        if (verbose_)
        {
            std::cout << "ROR ";
        }
        const SnesPtr ptr = addr_absolute_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.rotate_right(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.rotate_right(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::opE1_SBC(void)
    {
        if (verbose_)
        {
            std::cout << "SBC ";
        }
        const SnesPtr ptr = addr_direct_indexed_indirect();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.subtract_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.subtract_with_carry(arg);
        }
    }

    void Ricoh5A22::opE3_SBC(void)
    {
        if (verbose_)
        {
            std::cout << "SBC ";
        }
        const SnesPtr ptr = addr_stack_relative();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.subtract_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.subtract_with_carry(arg);
        }
    }

    void Ricoh5A22::opE5_SBC(void)
    {
        if (verbose_)
        {
            std::cout << "SBC ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.subtract_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.subtract_with_carry(arg);
        }
    }

    void Ricoh5A22::opE7_SBC(void)
    {
        if (verbose_)
        {
            std::cout << "SBC ";
        }
        const SnesPtr ptr = addr_direct_indirect_long();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.subtract_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.subtract_with_carry(arg);
        }
    }

    void Ricoh5A22::opE9_SBC(void)
    {
        if (verbose_)
        {
            std::cout << "SBC ";
        }
        if (state_.is_status_memory())
        {
            const Byte arg = addr_immediate8();
            alu_.subtract_with_carry(arg);
        }
        else
        {
            const Word arg = addr_immediate16();
            alu_.subtract_with_carry(arg);
        }
    }

    void Ricoh5A22::opED_SBC(void)
    {
        if (verbose_)
        {
            std::cout << "SBC ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.subtract_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.subtract_with_carry(arg);
        }
    }

    void Ricoh5A22::opEF_SBC(void)
    {
        if (verbose_)
        {
            std::cout << "SBC ";
        }
        const SnesPtr ptr = addr_absolute_long();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.subtract_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.subtract_with_carry(arg);
        }
    }

    void Ricoh5A22::opF1_SBC(void)
    {
        if (verbose_)
        {
            std::cout << "SBC ";
        }
        const SnesPtr ptr = addr_direct_indirect_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.subtract_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.subtract_with_carry(arg);
        }
    }

    void Ricoh5A22::opF2_SBC(void)
    {
        if (verbose_)
        {
            std::cout << "SBC ";
        }
        const SnesPtr ptr = addr_direct_indirect();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.subtract_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.subtract_with_carry(arg);
        }
    }

    void Ricoh5A22::opF3_SBC(void)
    {
        if (verbose_)
        {
            std::cout << "SBC ";
        }
        const SnesPtr ptr = addr_stack_relative_indirect_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.subtract_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.subtract_with_carry(arg);
        }
    }

    void Ricoh5A22::opF5_SBC(void)
    {
        if (verbose_)
        {
            std::cout << "SBC ";
        }
        const SnesPtr ptr = addr_direct_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.subtract_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.subtract_with_carry(arg);
        }
    }

    void Ricoh5A22::opF7_SBC(void)
    {
        if (verbose_)
        {
            std::cout << "SBC ";
        }
        const SnesPtr ptr = addr_direct_indirect_indexed_long();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.subtract_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.subtract_with_carry(arg);
        }
    }

    void Ricoh5A22::opF9_SBC(void)
    {
        if (verbose_)
        {
            std::cout << "SBC ";
        }
        const SnesPtr ptr = addr_absolute_indexed_y();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.subtract_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.subtract_with_carry(arg);
        }
    }

    void Ricoh5A22::opFD_SBC(void)
    {
        if (verbose_)
        {
            std::cout << "SBC ";
        }
        const SnesPtr ptr = addr_absolute_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.subtract_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.subtract_with_carry(arg);
        }
    }

    void Ricoh5A22::opFF_SBC(void)
    {
        if (verbose_)
        {
            std::cout << "SBC ";
        }
        const SnesPtr ptr = addr_absolute_long_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            alu_.subtract_with_carry(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            alu_.subtract_with_carry(arg);
        }
    }

    void Ricoh5A22::op14_TRB(void)
    {
        if (verbose_)
        {
            std::cout << "TRB ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.test_and_reset(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.test_and_reset(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::op1C_TRB(void)
    {
        if (verbose_)
        {
            std::cout << "TRB ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.test_and_reset(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.test_and_reset(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::op04_TSB(void)
    {
        if (verbose_)
        {
            std::cout << "TSB ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.test_and_set(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.test_and_set(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::op0C_TSB(void)
    {
        if (verbose_)
        {
            std::cout << "TSB ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            const Byte result = alu_.test_and_set(arg);
            mem_.write(ptr, result);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            const Word result = alu_.test_and_set(arg);
            mem_.write(ptr, result);
        }
    }

    void Ricoh5A22::opEB_XBA(void)
    {
        // Not implemented
    }

} // namespace snes