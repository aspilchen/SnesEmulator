#include "ricoh5a22.h"

namespace snes
{
    void Ricoh5A22::opA1_LDA(void)
    {
        if (verbose_)
        {
            std::cout << "LDA ";
        }
        const SnesPtr ptr = addr_direct_indexed_indirect();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opA3_LDA(void)
    {
        if (verbose_)
        {
            std::cout << "LDA ";
        }
        const SnesPtr ptr = addr_stack_relative();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opA5_LDA(void)
    {
        if (verbose_)
        {
            std::cout << "LDA ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opA7_LDA(void)
    {
        if (verbose_)
        {
            std::cout << "LDA ";
        }
        const SnesPtr ptr = addr_direct_indirect_long();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opA9_LDA(void)
    {
        if (verbose_)
        {
            std::cout << "LDA ";
        }
        if (state_.is_status_memory())
        {
            const Byte arg = addr_immediate8();
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = addr_immediate16();
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opAD_LDA(void)
    {
        if (verbose_)
        {
            std::cout << "LDA ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opAF_LDA(void)
    {
        if (verbose_)
        {
            std::cout << "LDA ";
        }
        const SnesPtr ptr = addr_absolute_long();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opB1_LDA(void)
    {
        if (verbose_)
        {
            std::cout << "LDA ";
        }
        const SnesPtr ptr = addr_direct_indirect_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opB2_LDA(void)
    {
        if (verbose_)
        {
            std::cout << "LDA ";
        }
        const SnesPtr ptr = addr_direct_indirect();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opB3_LDA(void)
    {
        if (verbose_)
        {
            std::cout << "LDA ";
        }
        const SnesPtr ptr = addr_stack_relative_indirect_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opB5_LDA(void)
    {
        if (verbose_)
        {
            std::cout << "LDA ";
        }
        const SnesPtr ptr = addr_direct_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opB7_LDA(void)
    {
        if (verbose_)
        {
            std::cout << "LDA ";
        }
        const SnesPtr ptr = addr_direct_indirect_indexed_long();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opB9_LDA(void)
    {
        if (verbose_)
        {
            std::cout << "LDA ";
        }
        const SnesPtr ptr = addr_absolute_indexed_y();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opBD_LDA(void)
    {
        if (verbose_)
        {
            std::cout << "LDA ";
        }
        const SnesPtr ptr = addr_absolute_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opBF_LDA(void)
    {
        if (verbose_)
        {
            std::cout << "LDA ";
        }
        const SnesPtr ptr = addr_absolute_long_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            state_.set_acc(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opA2_LDX(void)
    {
        if (verbose_)
        {
            std::cout << "LDX ";
        }
        if (state_.is_status_index())
        {
            const Byte arg = addr_immediate8();
            state_.set_x(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = addr_immediate16();
            state_.set_x(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opA6_LDX(void)
    {
        if (verbose_)
        {
            std::cout << "LDX ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_index())
        {
            const Byte arg = mem_.get_byte(ptr);
            state_.set_x(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            state_.set_x(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opAE_LDX(void)
    {
        if (verbose_)
        {
            std::cout << "LDX ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_index())
        {
            const Byte arg = mem_.get_byte(ptr);
            state_.set_x(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            state_.set_x(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opB6_LDX(void)
    {
        // Not implemented
    }

    void Ricoh5A22::opBE_LDX(void)
    {
        if (verbose_)
        {
            std::cout << "LDX ";
        }
        const SnesPtr ptr = addr_absolute_indexed_y();
        if (state_.is_status_index())
        {
            const Byte arg = mem_.get_byte(ptr);
            state_.set_x(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            state_.set_x(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opA0_LDY(void)
    {
        if (verbose_)
        {
            std::cout << "LDY ";
        }
        if (state_.is_status_index())
        {
            const Byte arg = addr_immediate8();
            state_.set_y(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = addr_immediate16();
            state_.set_y(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opA4_LDY(void)
    {
        if (verbose_)
        {
            std::cout << "LDY ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_index())
        {
            const Byte arg = mem_.get_byte(ptr);
            state_.set_y(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            state_.set_y(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opAC_LDY(void)
    {
        if (verbose_)
        {
            std::cout << "LDY ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_index())
        {
            const Byte arg = mem_.get_byte(ptr);
            state_.set_y(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            state_.set_y(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opB4_LDY(void)
    {
        if (verbose_)
        {
            std::cout << "LDY ";
        }
        const SnesPtr ptr = addr_direct_indexed_x();
        if (state_.is_status_index())
        {
            const Byte arg = mem_.get_byte(ptr);
            state_.set_y(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            state_.set_y(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::opBC_LDY(void)
    {
        if (verbose_)
        {
            std::cout << "LDY ";
        }
        const SnesPtr ptr = addr_absolute_indexed_x();
        if (state_.is_status_index())
        {
            const Byte arg = mem_.get_byte(ptr);
            state_.set_y(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
        else
        {
            const Word arg = mem_.get_word(ptr);
            state_.set_y(arg);
            state_.test_negative(arg);
            state_.test_zero(arg);
        }
    }

    void Ricoh5A22::op81_STA(void)
    {
        if (verbose_)
        {
            std::cout << "STA ";
        }
        const SnesPtr ptr = addr_direct_indexed_indirect();
        if (state_.is_status_memory())
        {
            const Byte arg = state_.acc8();
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = state_.acc16();
            mem_.write(ptr, arg);
        }
    }

    void Ricoh5A22::op83_STA(void)
    {
        if (verbose_)
        {
            std::cout << "STA ";
        }
        const SnesPtr ptr = addr_stack_relative();
        if (state_.is_status_memory())
        {
            const Byte arg = state_.acc8();
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = state_.acc16();
            mem_.write(ptr, arg);
        }
    }

    void Ricoh5A22::op85_STA(void)
    {
        if (verbose_)
        {
            std::cout << "STA ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_memory())
        {
            const Byte arg = state_.acc8();
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = state_.acc16();
            mem_.write(ptr, arg);
        }
    }

    void Ricoh5A22::op87_STA(void)
    {
        if (verbose_)
        {
            std::cout << "STA ";
        }
        const SnesPtr ptr = addr_direct_indirect_long();
        if (state_.is_status_memory())
        {
            const Byte arg = state_.acc8();
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = state_.acc16();
            mem_.write(ptr, arg);
        }
    }

    void Ricoh5A22::op8D_STA(void)
    {
        if (verbose_)
        {
            std::cout << "STA ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_memory())
        {
            const Byte arg = state_.acc8();
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = state_.acc16();
            mem_.write(ptr, arg);
        }
    }

    void Ricoh5A22::op8F_STA(void)
    {
        if (verbose_)
        {
            std::cout << "STA ";
        }
        const SnesPtr ptr = addr_absolute_long();
        if (state_.is_status_memory())
        {
            const Byte arg = state_.acc8();
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = state_.acc16();
            mem_.write(ptr, arg);
        }
    }

    void Ricoh5A22::op91_STA(void)
    {
        if (verbose_)
        {
            std::cout << "STA ";
        }
        const SnesPtr ptr = addr_direct_indirect_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = state_.acc8();
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = state_.acc16();
            mem_.write(ptr, arg);
        }
    }

    void Ricoh5A22::op92_STA(void)
    {
        if (verbose_)
        {
            std::cout << "STA ";
        }
        const SnesPtr ptr = addr_direct_indirect();
        if (state_.is_status_memory())
        {
            const Byte arg = state_.acc8();
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = state_.acc16();
            mem_.write(ptr, arg);
        }
    }

    void Ricoh5A22::op93_STA(void)
    {
        if (verbose_)
        {
            std::cout << "STA ";
        }
        const SnesPtr ptr = addr_stack_relative_indirect_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = state_.acc8();
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = state_.acc16();
            mem_.write(ptr, arg);
        }
    }

    void Ricoh5A22::op95_STA(void)
    {
        if (verbose_)
        {
            std::cout << "STA ";
        }
        const SnesPtr ptr = addr_direct_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = state_.acc8();
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = state_.acc16();
            mem_.write(ptr, arg);
        }
    }

    void Ricoh5A22::op97_STA(void)
    {
        if (verbose_)
        {
            std::cout << "STA ";
        }
        const SnesPtr ptr = addr_direct_indirect_indexed_long();
        if (state_.is_status_memory())
        {
            const Byte arg = state_.acc8();
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = state_.acc16();
            mem_.write(ptr, arg);
        }
    }

    void Ricoh5A22::op99_STA(void)
    {
        if (verbose_)
        {
            std::cout << "STA ";
        }
        const SnesPtr ptr = addr_absolute_indexed_y();
        if (state_.is_status_memory())
        {
            const Byte arg = state_.acc8();
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = state_.acc16();
            mem_.write(ptr, arg);
        }
    }

    void Ricoh5A22::op9D_STA(void)
    {
        if (verbose_)
        {
            std::cout << "STA ";
        }
        const SnesPtr ptr = addr_absolute_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = state_.acc8();
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = state_.acc16();
            mem_.write(ptr, arg);
        }
    }

    void Ricoh5A22::op9F_STA(void)
    {
        if (verbose_)
        {
            std::cout << "STA ";
        }
        const SnesPtr ptr = addr_absolute_long_indexed();
        if (state_.is_status_memory())
        {
            const Byte arg = state_.acc8();
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = state_.acc16();
            mem_.write(ptr, arg);
        }
    }

    void Ricoh5A22::op86_STX(void)
    {
        if (verbose_)
        {
            std::cout << "STX ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_index())
        {
            const Byte arg = state_.x8();
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = state_.x16();
            mem_.write(ptr, arg);
        }
    }

    void Ricoh5A22::op8E_STX(void)
    {
        if (verbose_)
        {
            std::cout << "STX ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_index())
        {
            const Byte arg = state_.x8();
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = state_.x16();
            mem_.write(ptr, arg);
        }
    }

    void Ricoh5A22::op96_STX(void)
    {
        // Not implemented
    }

    void Ricoh5A22::op84_STY(void)
    {
        if (verbose_)
        {
            std::cout << "STY ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_index())
        {
            const Byte arg = state_.y8();
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = state_.y16();
            mem_.write(ptr, arg);
        }
    }

    void Ricoh5A22::op8C_STY(void)
    {
        if (verbose_)
        {
            std::cout << "STY ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_index())
        {
            const Byte arg = state_.y8();
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = state_.y16();
            mem_.write(ptr, arg);
        }
    }

    void Ricoh5A22::op94_STY(void)
    {
        if (verbose_)
        {
            std::cout << "STY ";
        }
        const SnesPtr ptr = addr_direct_indexed_x();
        if (state_.is_status_index())
        {
            const Byte arg = state_.y8();
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = state_.y16();
            mem_.write(ptr, arg);
        }
    }

    void Ricoh5A22::op64_STZ(void)
    {
        if (verbose_)
        {
            std::cout << "STZ ";
        }
        const SnesPtr ptr = addr_direct();
        if (state_.is_status_memory())
        {
            const Byte arg = 0;
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = 0;
            mem_.write(ptr, arg);
        }
    }

    void Ricoh5A22::op74_STZ(void)
    {
        if (verbose_)
        {
            std::cout << "STZ ";
        }
        const SnesPtr ptr = addr_direct_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = 0;
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = 0;
            mem_.write(ptr, arg);
        }
    }

    void Ricoh5A22::op9C_STZ(void)
    {
        if (verbose_)
        {
            std::cout << "STZ ";
        }
        const SnesPtr ptr = addr_absolute();
        if (state_.is_status_memory())
        {
            const Byte arg = 0;
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = 0;
            mem_.write(ptr, arg);
        }
    }

    void Ricoh5A22::op9E_STZ(void)
    {
        if (verbose_)
        {
            std::cout << "STZ ";
        }
        const SnesPtr ptr = addr_absolute_indexed_x();
        if (state_.is_status_memory())
        {
            const Byte arg = 0;
            mem_.write(ptr, arg);
        }
        else
        {
            const Word arg = 0;
            mem_.write(ptr, arg);
        }
    }

} // namespace snes