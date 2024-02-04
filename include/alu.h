#ifndef __ALU_H
#define __ALU_H

#include "cpu_state.h"

namespace snes
{

    /**
     * @brief Arithmetic logic unit. Performs addition, subtraction etc.. and modifies the CpuState accordingly.
     * 
     */
    class Alu {
        public:
            Alu(CpuState& state);

            void add_with_carry(Word arg);
            void add_with_carry(Byte arg);

            void subtract_with_carry(Word arg);
            void subtract_with_carry(Byte arg);

            void logic_and(Word arg);
            void logic_and(Byte arg);

            void exclusive_or(Word arg);
            void exclusive_or(Byte arg);

            void logic_or(Word arg);
            void logic_or(Byte arg);

            Word test_and_set(Word arg);
            Byte test_and_set(Byte arg);

            Word test_and_reset(Word arg);
            Byte test_and_reset(Byte arg);

            void arith_shift_left_a16(void);
            void arith_shift_left_a8(void);
            Word arith_shift_left(Word arg);
            Byte arith_shift_left(Byte arg);

            void logical_shift_right_a16(void);
            void logical_shift_right_a8(void);
            Word logical_shift_right(Word arg);
            Byte logical_shift_right(Byte arg);            

            void rotate_left_a16(void);
            void rotate_left_a8(void);
            Word rotate_left(Word arg);
            Byte rotate_left(Byte arg);

            void rotate_right_a16(void);
            void rotate_right_a8(void);
            Word rotate_right(Word arg);
            Byte rotate_right(Byte arg);

            void compare_with_acc(Word arg);
            void compare_with_acc(Byte arg);

            void compare_with_x(Word arg);
            void compare_with_x(Byte arg);

            void compare_with_y(Word arg);
            void compare_with_y(Byte arg);

            Word decrement(Word arg);
            Byte decrement(Byte arg);
            void decrement_a16(void);
            void decrement_a8(void);
            void decrement_x16(void);
            void decrement_x8(void);
            void decrement_y16(void);
            void decrement_y8(void);

            Word increment(Word arg);
            Byte increment(Byte arg);
            void increment_a16(void);
            void increment_a8(void);
            void increment_x16(void);
            void increment_x8(void);
            void increment_y16(void);
            void increment_y8(void);

            void exchange_acc(void);

            // Weird ops that need more documentation
            void bit(Word arg);
            void bit(Byte arg);
        private:

            uint8_t carry(void);

            void test_negative(Word arg);
            void test_negative(Byte arg);
            void test_zero(Word arg);
            void test_zero(Byte arg);
            void test_carry_16(uint32_t arg);
            void test_carry_8(uint32_t arg);

            CpuState& state_;
    };
} // namespace snes


#endif