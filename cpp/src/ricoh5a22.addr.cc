#include "ricoh5a22.h"

namespace snes
{
    Word Ricoh5A22::addr_immediate16(void)
    {
        const Word result = get_word();
        if (verbose_)
        {
            std::cout << "#$" << std::setw(4) << std::setfill('0') << std::uppercase << std::hex << result;
        }
        return result;
    }

    Byte Ricoh5A22::addr_immediate8(void)
    {
        const Byte result = get_byte();
        if (verbose_)
        {
            std::cout << "#$" << std::setw(2) << std::setfill('0') << std::uppercase << std::hex << result;
        }
        return result;
    }

    SnesPtr Ricoh5A22::addr_absolute(void)
    {
        SnesPtr result = get_word();
        if (verbose_)
        {
            std::cout << "$" << std::setw(4) << std::setfill('0') << std::uppercase << std::hex << result;
        }
        result |= state_.dbr() << sizeof(Word);
        return result;
    }

    SnesPtr Ricoh5A22::addr_absolute_indexed_x(void)
    {
        SnesPtr result = addr_absolute();
        if (verbose_)
        {
            std::cout << ", x";
        }
        result += state_.x16();
        return result;
    }

    SnesPtr Ricoh5A22::addr_absolute_indexed_y(void)
    {
        SnesPtr result = addr_absolute();
        if (verbose_)
        {
            std::cout << ", y";
        }
        result += state_.y16();
        return result;
    }

    SnesPtr Ricoh5A22::addr_absolute_long(void)
    {
        SnesPtr result = 0;
        const int kThreeBytes = 3;
        mem_.get(&result, state_.pc(), kThreeBytes);
        if (verbose_)
        {
            std::cout << "$" << std::setw(6) << std::setfill('0') << std::uppercase << std::hex << result;
        }
        state_.pc() += kThreeBytes;
        return result;
    }

    SnesPtr Ricoh5A22::addr_absolute_long_indexed(void)
    {
        SnesPtr result = addr_absolute_long();
        if (verbose_)
        {
            std::cout << ", x";
        }
        result += state_.x16();
        return result;
    }

    SnesPtr Ricoh5A22::addr_absolute_indirect(void)
    {
        SnesPtr result = get_word();
        if (verbose_)
        {
            std::cout << "($" << std::setw(4) << std::setfill('0') << std::uppercase << std::hex << result << ")";
        }
        result = mem_.get_word(result);
        return result;
    }

    SnesPtr Ricoh5A22::addr_absolute_indexed_indirect(void)
    {
        SnesPtr result = get_word();
        if (verbose_)
        {
            std::cout << "($" << std::setw(4) << std::setfill('0') << std::uppercase << std::hex << result << ", x)";
        }
        result += state_.x16();
        result = mem_.get_word(result);
        return result;
    }

    SnesPtr Ricoh5A22::addr_direct(void)
    {
        SnesPtr result = get_byte();
        if (verbose_)
        {
            std::cout << "$" << std::setw(2) << std::setfill('0') << std::uppercase << std::hex << result;
        }
        result += state_.dp();
        return result;
    }

    SnesPtr Ricoh5A22::addr_direct_indirect(void)
    {
        if (verbose_)
        {
            std::cout << '(';
        }
        SnesPtr result = addr_direct();
        if (verbose_)
        {
            std::cout << ')';
        }
        result = mem_.get_word(result);
        result |= state_.dbr() << sizeof(Word);
        return result;
    }

    SnesPtr Ricoh5A22::addr_direct_indirect_long(void)
    {
        if (verbose_)
        {
            std::cout << '[';
        }
        SnesPtr result = addr_direct();
        if (verbose_)
        {
            std::cout << ']';
        }
        const int kThreeBytes = 3;
        mem_.get(&result, result, kThreeBytes);
        return result;
    }

    SnesPtr Ricoh5A22::addr_direct_indirect_indexed(void)
    {
        SnesPtr result = addr_direct_indirect();
        if (verbose_)
        {
            std::cout << ", y";
        }
        result += state_.y16();
        return result;
    }

    SnesPtr Ricoh5A22::addr_direct_indirect_indexed_long(void)
    {
        SnesPtr result = addr_direct_indirect_long();
        if (verbose_)
        {
            std::cout << ", y";
        }
        result += state_.y16();
        return result;
    }

    SnesPtr Ricoh5A22::addr_direct_indexed_x(void)
    {
        SnesPtr result = addr_direct();
        if (verbose_)
        {
            std::cout << ", x";
        }
        result += state_.x16();
        return result;
    }

    SnesPtr Ricoh5A22::addr_direct_indexed_y(void)
    {
        SnesPtr result = addr_direct();
        if (verbose_)
        {
            std::cout << ", y";
        }
        result += state_.y16();
        return result;
    }

    SnesPtr Ricoh5A22::addr_direct_indexed_indirect(void)
    {
        if (verbose_)
        {
            std::cout << '(';
        }
        SnesPtr result = addr_direct_indexed_x();
        if (verbose_)
        {
            std::cout << ')';
        }
        result = mem_.get_word(result);
        result |= state_.dbr() << sizeof(Word);
        return result;
    }

    SnesPtr Ricoh5A22::addr_pc_relative(void)
    {
        SnesPtr result = get_byte();
        if (verbose_)
        {
            std::cout << "$" << std::setw(4) << std::setfill('0') << std::uppercase << std::hex << result;
        }
        result += state_.pc();
        result &= 0xFFFF;
        return result;
    }

    SnesPtr Ricoh5A22::addr_pc_relative_long(void)
    {
        SnesPtr result = get_word();
        if (verbose_)
        {
            std::cout << "$" << std::setw(6) << std::setfill('0') << std::uppercase << std::hex << result;
        }
        result += state_.pc();
        result &= 0xFFFF;
        return result;
    }

    SnesPtr Ricoh5A22::addr_stack_relative(void)
    {
        SnesPtr result = get_byte();
        if (verbose_)
        {
            std::cout << result << ", s";
        }
        result += state_.stack();
        return result;
    }

    SnesPtr Ricoh5A22::addr_stack_relative_indirect_indexed(void)
    {
        if (verbose_)
        {
            std::cout << '(';
        }
        SnesPtr result = addr_stack_relative();
        if (verbose_)
        {
            std::cout << result << "), y";
        }
        result |= state_.dbr() << sizeof(Word);
        result += state_.y16();
        return result;
    }
} // namespace snes