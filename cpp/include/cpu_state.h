#ifndef __CPU_STATE_H
#define __CPU_STATE_H

#include "snes_mem.h"

namespace snes
{

    /**
     * @brief Contains all data regarding the state of the CPU
     *
     */
    class CpuState
    {
    public:
        /**
         * @brief Bit flags for the status register.
         *
         */
        enum StatusFlag
        {
            kEmulation = 1 << 8,
            kNegative = 1 << 7,
            kOverflow = 1 << 6,
            kMemory = 1 << 5,
            kIndex = 1 << 4,
            kBreak = 1 << 4,
            kDecimal = 1 << 3,
            kIRQ = 1 << 2,
            kZero = 1 << 1,
            kCarry = 1
        };

        CpuState(void);

        void init(void);

        void clear_status_break(void);
        void clear_status_carry(void);
        void clear_status_decimal(void);
        void clear_status_emulation(void);
        void clear_status_index(void);
        void clear_status_irq(void);
        void clear_status_memory(void);
        void clear_status_negative(void);
        void clear_status_overflow(void);
        void clear_status_zero(void);
        void clear_status(Byte arg);

        void set_status_break(void);
        void set_status_carry(void);
        void set_status_decimal(void);
        void set_status_emulation(void);
        void set_status_index(void);
        void set_status_irq(void);
        void set_status_memory(void);
        void set_status_negative(void);
        void set_status_overflow(void);
        void set_status_zero(void);
        void set_status(Byte arg);

        bool is_status_break(void);
        bool is_status_carry(void);
        bool is_status_decimal(void);
        bool is_status_emulation(void);
        bool is_status_index(void);
        bool is_status_irq(void);
        bool is_status_memory(void);
        bool is_status_negative(void);
        bool is_status_overflow(void);
        bool is_status_zero(void);
        Word get_status(void);

        void test_negative(Word arg);
        void test_negative(Byte arg);
        void test_zero(Word arg);
        void test_zero(Byte arg);
        void test_carry_16(uint32_t arg);
        void test_carry_8(uint32_t arg);

        void set_acc(Word arg);
        void set_acc(Byte arg);

        void set_y(Word arg);
        void set_y(Byte arg);

        void set_x(Word arg);
        void set_x(Byte arg);

        Word &acc16(void);
        Byte &acc8(void);

        Word &y16(void);
        Byte &y8(void);

        Word &x16(void);
        Byte &x8(void);

        Byte &dbr(void);
        Byte &pbr(void);
        Word &dp(void);
        SnesPtr &pc(void);
        SnesPtr &stack(void);
        Word &status(void);

        uint32_t &bus_open(void);

    private:
        union VariableRegister
        {
            Word data16;
            Byte data8;
        };

        struct
        {
            union VariableRegister accumulator;
            union VariableRegister y_index;
            union VariableRegister x_index;
            Byte data_bank;
            Byte program_bank;
            Word direct_page;
            SnesPtr program_counter;
            SnesPtr stack_pointer;
            Word status;
        } reg_;

        struct
        {
            uint32_t open;
        } bus_;
    };
} // namespace snes

#endif