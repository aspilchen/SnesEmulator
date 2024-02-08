#include "ricoh5a22.h"

namespace snes
{
    void Ricoh5A22::op90_BCC(void)
    {
        if (verbose_)
        {
            std::cout << "BCC ";
        }
        const SnesPtr arg = addr_pc_relative();
        if (!state_.is_status_carry())
        {
            state_.pc() = arg;
        }
    }

    void Ricoh5A22::opB0_BCS(void)
    {
        if (verbose_)
        {
            std::cout << "BCS ";
        }
        const SnesPtr arg = addr_pc_relative();
        if (state_.is_status_carry())
        {
            state_.pc() = arg;
        }
    }

    void Ricoh5A22::opF0_BEQ(void)
    {
        if (verbose_)
        {
            std::cout << "BEQ ";
        }
        const SnesPtr arg = addr_pc_relative();
        if (state_.is_status_zero())
        {
            state_.pc() = arg;
        }
    }

    void Ricoh5A22::op30_BMI(void)
    {
        if (verbose_)
        {
            std::cout << "BMI ";
        }
        const SnesPtr arg = addr_pc_relative();
        if (state_.is_status_negative())
        {
            state_.pc() = arg;
        }
    }

    void Ricoh5A22::opD0_BNE(void)
    {
        if (verbose_)
        {
            std::cout << "BNE ";
        }
        const SnesPtr arg = addr_pc_relative();
        if (!state_.is_status_zero())
        {
            state_.pc() = arg;
        }
    }

    void Ricoh5A22::op10_BPL(void)
    {
        if (verbose_)
        {
            std::cout << "BPL ";
        }
        const SnesPtr arg = addr_pc_relative();
        if (!state_.is_status_negative())
        {
            state_.pc() = arg;
        }
    }

    void Ricoh5A22::op80_BRA(void)
    {
        if (verbose_)
        {
            std::cout << "BRA ";
        }
        const SnesPtr arg = addr_pc_relative();
        state_.pc() = arg;
    }

    void Ricoh5A22::op82_BRL(void)
    {
        if (verbose_)
        {
            std::cout << "BRL ";
        }
        const SnesPtr arg = addr_pc_relative_long();
        state_.pc() = arg;
    }

    void Ricoh5A22::op50_BVC(void)
    {
        if (verbose_)
        {
            std::cout << "BVC ";
        }
        const SnesPtr arg = addr_pc_relative();
        if (!state_.is_status_overflow())
        {
            state_.pc() = arg;
        }
    }

    void Ricoh5A22::op70_BVS(void)
    {
        if (verbose_)
        {
            std::cout << "BVS ";
        }
        const SnesPtr arg = addr_pc_relative();
        if (state_.is_status_overflow())
        {
            state_.pc() = arg;
        }
    }

} // namespace snes