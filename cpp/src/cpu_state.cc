#include <cstring>

#include "cpu_state.h"

namespace snes
{

    CpuState::CpuState(void)
    {
        init();
    }

    void CpuState::init(void)
    {
        std::memset(&reg_, 0, sizeof(reg_));
    }

    // Clear status flags
    void CpuState::clear_status_break(void)
    {
        reg_.status &= ~CpuState::kBreak;
    }

    void CpuState::clear_status_carry(void)
    {
        reg_.status &= ~CpuState::kCarry;
    }

    void CpuState::clear_status_decimal(void)
    {
        reg_.status &= ~CpuState::kDecimal;
    }

    void CpuState::clear_status_emulation(void)
    {
        reg_.status &= ~kEmulation;
    }

    void CpuState::clear_status_index(void)
    {
        reg_.status &= ~CpuState::kIndex;
    }

    void CpuState::clear_status_irq(void)
    {
        reg_.status &= ~CpuState::kIRQ;
    }

    void CpuState::clear_status_memory(void)
    {
        reg_.status &= ~CpuState::kMemory;
    }

    void CpuState::clear_status_negative(void)
    {
        reg_.status &= ~kNegative;
    }

    void CpuState::clear_status_overflow(void)
    {
        reg_.status &= ~CpuState::kOverflow;
    }

    void CpuState::clear_status_zero(void)
    {
        reg_.status &= ~CpuState::kZero;
    }

    void CpuState::clear_status(Byte arg)
    {
        reg_.status &= ~arg;
    }

    // Set status flags
    void CpuState::set_status_break(void)
    {
        reg_.status |= CpuState::kBreak;
    }

    void CpuState::set_status_carry(void)
    {
        reg_.status |= CpuState::kCarry;
    }

    void CpuState::set_status_decimal(void)
    {
        reg_.status |= CpuState::kDecimal;
    }

    void CpuState::set_status_emulation(void)
    {
        reg_.status |= CpuState::kEmulation;
    }

    void CpuState::set_status_index(void)
    {
        reg_.status |= CpuState::kIndex;
    }

    void CpuState::set_status_irq(void)
    {
        reg_.status |= CpuState::kIRQ;
    }

    void CpuState::set_status_memory(void)
    {
        reg_.status |= CpuState::kMemory;
    }

    void CpuState::set_status_negative(void)
    {
        reg_.status |= CpuState::kNegative;
    }

    void CpuState::set_status_overflow(void)
    {
        reg_.status |= CpuState::kOverflow;
    }

    void CpuState::set_status_zero(void)
    {
        reg_.status |= CpuState::kZero;
    }

    void CpuState::set_status(Byte arg)
    {
        reg_.status |= arg;
    }

    // Get status flags
    bool CpuState::is_status_break(void)
    {
        return reg_.status & CpuState::kBreak;
    }

    bool CpuState::is_status_carry(void)
    {
        return reg_.status & CpuState::kCarry ? 1 : 0;
    }

    bool CpuState::is_status_decimal(void)
    {
        return reg_.status & CpuState::kDecimal;
    }

    bool CpuState::is_status_emulation(void)
    {
        return reg_.status & CpuState::kEmulation;
    }

    bool CpuState::is_status_index(void)
    {
        return reg_.status & CpuState::kIndex;
    }

    bool CpuState::is_status_irq(void)
    {
        return reg_.status & CpuState::kIRQ;
    }

    bool CpuState::is_status_memory(void)
    {
        return reg_.status & CpuState::kMemory;
    }

    bool CpuState::is_status_negative(void)
    {
        return reg_.status & CpuState::kNegative;
    }

    bool CpuState::is_status_overflow(void)
    {
        return reg_.status & CpuState::kOverflow;
    }

    bool CpuState::is_status_zero(void)
    {
        return reg_.status & CpuState::kZero;
    }

    Word CpuState::get_status(void)
    {
        return reg_.status;
    }

    void CpuState::test_negative(Word arg)
    {
        const bool kIsNegative = arg & kWordMSB;
        if (kIsNegative)
        {
            set_status_negative();
        }
        else
        {
            clear_status_negative();
        }
    }

    void CpuState::test_negative(Byte arg)
    {
        const bool kIsNegative = arg & kByteMSB;
        if (kIsNegative)
        {
            set_status_negative();
        }
        else
        {
            clear_status_negative();
        }
    }

    void CpuState::test_zero(Word arg)
    {
        const bool kIsZero = arg == 0;
        if (kIsZero)
        {
            set_status_zero();
        }
        else
        {
            clear_status_zero();
        }
    }

    void CpuState::test_zero(Byte arg)
    {
        const bool kIsZero = arg == 0;
        if (kIsZero)
        {
            set_status_zero();
        }
        else
        {
            clear_status_zero();
        }
    }

    void CpuState::test_carry_16(uint32_t arg)
    {
        // const bool kBitCarried = arg > std::numeric_limits<uint16_t>::max();
        const bool kBitCarried = arg > 0xFFFF;
        if (kBitCarried)
        {
            set_status_carry();
        }
        else
        {
            clear_status_carry();
        }
    }

    void CpuState::test_carry_8(uint32_t arg)
    {
        // const bool kBitCarried = arg > std::numeric_limits<uint8_t>::max();
        const bool kBitCarried = arg > 0xFF;
        if (kBitCarried)
        {
            set_status_carry();
        }
        else
        {
            clear_status_carry();
        }
    }

    void CpuState::set_acc(Word arg)
    {
        reg_.accumulator.data16 = arg;
    }

    void CpuState::set_acc(Byte arg)
    {
        reg_.accumulator.data8 = arg;
    }

    void CpuState::set_y(Word arg)
    {
        reg_.y_index.data16 = arg;
    }

    void CpuState::set_y(Byte arg)
    {
        reg_.y_index.data8 = arg;
    }

    void CpuState::set_x(Word arg)
    {
        reg_.x_index.data16 = arg;
    }

    void CpuState::set_x(Byte arg)
    {
        reg_.x_index.data8 = arg;
    }

    Word &CpuState::acc16(void)
    {
        return reg_.accumulator.data16;
    }

    Byte &CpuState::acc8(void)
    {
        return reg_.accumulator.data8;
    }

    Word &CpuState::y16(void)
    {
        return reg_.y_index.data16;
    }

    Byte &CpuState::y8(void)
    {
        return reg_.y_index.data8;
    }

    Word &CpuState::x16(void)
    {
        return reg_.x_index.data16;
    }

    Byte &CpuState::x8(void)
    {
        return reg_.x_index.data8;
    }

    Byte &CpuState::dbr(void)
    {
        return reg_.data_bank;
    }

    Byte &CpuState::pbr(void)
    {
        return reg_.program_bank;
    }

    Word &CpuState::dp(void)
    {
        return reg_.direct_page;
    }

    SnesPtr &CpuState::pc(void)
    {
        return reg_.program_counter;
    }

    SnesPtr &CpuState::stack(void)
    {
        return reg_.stack_pointer;
    }

    Word &CpuState::status(void)
    {
        return reg_.status;
    }

    uint32_t &CpuState::bus_open(void)
    {
        return bus_.open;
    }
} // namespace snes