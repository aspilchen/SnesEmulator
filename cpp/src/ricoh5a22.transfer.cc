#include "ricoh5a22.h"

namespace snes
{
    void Ricoh5A22::opAA_TAX(void)
    {
        if (verbose_)
        {
            std::cout << "TAX ";
        }

        if (state_.is_status_index())
        {
            state_.x8() = state_.acc8();
        }
        else
        {
            state_.x16() = state_.acc16();
        }
    }

    void Ricoh5A22::opA8_TAY(void)
    {
        if (verbose_)
        {
            std::cout << "TAY ";
        }

        if (state_.is_status_index())
        {
            state_.y8() = state_.acc8();
        }
        else
        {
            state_.y16() = state_.acc16();
        }
    }

    void Ricoh5A22::op5B_TCD(void)
    {
        if (verbose_)
        {
            std::cout << "TCD ";
        }
        state_.dp() = state_.acc16();
    }

    void Ricoh5A22::op1B_TCS(void)
    {
        if (verbose_)
        {
            std::cout << "TCS ";
        }
        state_.stack() = state_.acc16();
    }

    void Ricoh5A22::op7B_TDC(void)
    {
        if (verbose_)
        {
            std::cout << "TDC ";
        }
        state_.acc16() = state_.dp();
    }

    void Ricoh5A22::op3B_TSC(void)
    {
        if (verbose_)
        {
            std::cout << "TSC ";
        }
        state_.acc16() = state_.stack();
    }

    void Ricoh5A22::opBA_TSX(void)
    {
        if (verbose_)
        {
            std::cout << "TSX ";
        }
        state_.x16() = state_.stack();
    }

    void Ricoh5A22::op8A_TXA(void)
    {
        if (verbose_)
        {
            std::cout << "TXA ";
        }
        if (state_.is_status_memory() || state_.is_status_index())
        {
            state_.acc8() = state_.x8();
        }
        else
        {
            state_.acc16() = state_.x16();
        }
    }

    void Ricoh5A22::op9A_TXS(void)
    {
        if (verbose_)
        {
            std::cout << "TXS ";
        }
        state_.stack() = state_.x16();
    }

    void Ricoh5A22::op9B_TXY(void)
    {
        if (verbose_)
        {
            std::cout << "TXY ";
        }
        if (state_.is_status_index())
        {
            state_.y8() = state_.x8();
        }
        else
        {
            state_.y16() = state_.x16();
        }
    }

    void Ricoh5A22::op98_TYA(void)
    {
        if (verbose_)
        {
            std::cout << "TYA ";
        }
        if (state_.is_status_memory() || state_.is_status_index())
        {
            state_.acc8() = state_.y8();
        }
        else
        {
            state_.acc16() = state_.y16();
        }
    }

    void Ricoh5A22::opBB_TYX(void)
    {
        if (verbose_)
        {
            std::cout << "TYX ";
        }
        if (state_.is_status_index())
        {
            state_.x8() = state_.y8();
        }
        else
        {
            state_.x16() = state_.y16();
        }
    }

} // namespace snes