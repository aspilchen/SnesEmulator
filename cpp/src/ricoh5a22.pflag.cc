#include "ricoh5a22.h"

namespace snes
{
    void Ricoh5A22::op18_CLC(void)
    {
        state_.clear_status_carry();
    }

    void Ricoh5A22::opD8_CLD(void)
    {
        state_.clear_status_decimal();
    }

    void Ricoh5A22::op58_CLI(void)
    {
        state_.clear_status_irq();
    }

    void Ricoh5A22::opB8_CLV(void)
    {
        state_.clear_status_overflow();
    }

    void Ricoh5A22::opC2_REP(void)
    {
        const Byte arg = addr_immediate8();
        state_.clear_status(arg);
    }

    void Ricoh5A22::op38_SEC(void)
    {
        state_.set_status_carry();
    }

    void Ricoh5A22::opF8_SED(void)
    {
        state_.set_status_decimal();
    }

    void Ricoh5A22::op78_SEI(void)
    {
        state_.set_status_irq();
    }

    void Ricoh5A22::opE2_SEP(void)
    {
        const Byte arg = addr_immediate8();
        state_.set_status(arg);
    }

    void Ricoh5A22::opFB_XCE(void)
    {
        bool is_emulation = state_.is_status_emulation();
        bool is_carry = state_.is_status_carry();

        if (is_carry)
        {
            state_.set_status_emulation();
        }
        else
        {
            state_.clear_status_emulation();
        }

        if (is_emulation)
        {
            state_.set_status_carry();
        }
        else
        {
            state_.clear_status_carry();
        }
    }

} // namespace snes