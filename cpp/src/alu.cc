#include "alu.h"

namespace snes
{

    Alu::Alu(CpuState &state) : state_(state) {}

    void Alu::add_with_carry(Word arg)
    {
        const Word acc = state_.acc16();
        const uint32_t result = acc + arg + carry();
        state_.test_negative((Word)result);
        state_.test_zero((Word)result);
        state_.test_carry_16((Word)result);
        const bool kSignedOverflow = (acc ^ result) & (arg ^ result) & kWordMSB;
        if (kSignedOverflow)
        {
            state_.set_status_overflow();
        }
        else
        {
            state_.clear_status_overflow();
        }
        state_.acc16() = result;
    }

    void Alu::add_with_carry(Byte arg)
    {
        const Byte acc = state_.acc8();
        const int32_t result = acc + arg + carry();
        state_.test_negative((Byte)result);
        state_.test_zero((Byte)result);
        state_.test_carry_8((Byte)result);
        const bool kSignedOverflow = (acc ^ result) & (arg ^ result) & kByteMSB;
        if (kSignedOverflow)
        {
            state_.set_status_overflow();
        }
        else
        {
            state_.clear_status_overflow();
        }
        state_.acc8() = result;
    }

    void Alu::subtract_with_carry(Word arg)
    {
        const Word acc = state_.acc16();
        const uint32_t result = acc - arg - carry();
        state_.test_negative((Word)result);
        state_.test_zero((Word)result);
        state_.test_carry_16((Word)result);
        const bool kSignedOverflow = (acc ^ arg) & (acc ^ result) & kWordMSB;
        if (kSignedOverflow)
        {
            state_.set_status_overflow();
        }
        else
        {
            state_.clear_status_overflow();
        }
        state_.acc16() = result;
    }

    void Alu::subtract_with_carry(Byte arg)
    {
        Byte acc = state_.acc8();
        const uint32_t result = acc - arg - carry();
        state_.test_negative((Byte)result);
        state_.test_zero((Byte)result);
        state_.test_carry_8((Byte)result);
        const bool kSignedOverflow = (acc ^ arg) & (acc ^ result) & kByteMSB;
        if (kSignedOverflow)
        {
            state_.set_status_overflow();
        }
        else
        {
            state_.clear_status_overflow();
        }
        state_.acc8() = result;
    }

    void Alu::logic_and(Word arg)
    {
        const Word result = state_.acc16() & arg;
        state_.test_zero(result);
        state_.test_negative(result);
        state_.acc16() = result;
    }

    void Alu::logic_and(Byte arg)
    {
        const Byte result = state_.acc8() & arg;
        state_.test_zero(result);
        state_.test_negative(result);
        state_.acc8() = result;
    }

    void Alu::exclusive_or(Word arg)
    {
        const Word result = state_.acc16() ^ arg;
        state_.test_zero(result);
        state_.test_negative(result);
        state_.acc16() = result;
    }

    void Alu::exclusive_or(Byte arg)
    {
        const Byte result = state_.acc8() ^ arg;
        state_.test_zero(result);
        state_.test_negative(result);
        state_.acc8() = result;
    }

    void Alu::logic_or(Word arg)
    {
        const Word result = state_.acc16() | arg;
        state_.test_negative(result);
        state_.test_zero(result);
        state_.acc16() = result;
    }

    void Alu::logic_or(Byte arg)
    {
        const Byte result = state_.acc8() | arg;
        state_.test_negative(result);
        state_.test_zero(result);
        state_.acc8() = result;
    }

    Word Alu::test_and_set(Word arg)
    {
        const Word result = state_.acc16() | arg;
        state_.test_zero(result);
        return result;
    }

    Byte Alu::test_and_set(Byte arg)
    {
        const Byte result = state_.acc8() | arg;
        state_.test_zero(result);
        return result;
    }

    Word Alu::test_and_reset(Word arg)
    {
        const Word result = ~state_.acc16() & arg;
        state_.test_zero(result);
        return result;
    }

    Byte Alu::test_and_reset(Byte arg)
    {
        const Byte result = ~state_.acc8() & arg;
        state_.test_zero(result);
        return result;
    }

    void Alu::arith_shift_left_a16(void)
    {
        const Word arg = state_.acc16();
        const Word result = arith_shift_left(arg);
        state_.acc16() = result;
    }

    void Alu::arith_shift_left_a8(void)
    {
        const Byte arg = state_.acc8();
        const Byte result = arith_shift_left(arg);
        state_.acc8() = result;
    }

    Word Alu::arith_shift_left(Word arg)
    {
        const Word acc = state_.acc16();
        const uint32_t result = acc << 1;
        state_.test_negative((Word)result);
        state_.test_zero((Word)result);
        state_.test_carry_8(result);
        return result;
    }

    Byte Alu::arith_shift_left(Byte arg)
    {
        const Byte acc = state_.acc8();
        const uint32_t result = acc << 1;
        state_.test_negative((Byte)result);
        state_.test_zero((Byte)result);
        state_.test_carry_8(result);
        return result;
    }

    void Alu::logical_shift_right_a16(void)
    {
        const Word arg = state_.acc16();
        const Word result = logical_shift_right(arg);
        state_.acc16() = result;
    }

    void Alu::logical_shift_right_a8(void)
    {
        const Byte arg = state_.acc8();
        const Byte result = logical_shift_right(arg);
        state_.acc8() = result;
    }

    Word Alu::logical_shift_right(Word arg)
    {
        const Word result = arg >> 1;
        const bool carry = arg & 1;
        if (carry)
        {
            state_.set_status_carry();
        }
        else
        {
            state_.clear_status_carry();
        }
        state_.clear_status_negative();
        state_.test_zero(result);
        return result;
    }

    Byte Alu::logical_shift_right(Byte arg)
    {
        const Byte result = arg >> 1;
        const bool carry = arg & 1;
        if (carry)
        {
            state_.set_status_carry();
        }
        else
        {
            state_.clear_status_carry();
        }
        state_.clear_status_negative();
        state_.test_zero(result);
        return result;
    }

    void Alu::rotate_left_a16(void)
    {
        const Word arg = state_.acc16();
        const Word result = rotate_left(arg);
        state_.acc16() = result;
    }

    void Alu::rotate_left_a8(void)
    {
        const Byte arg = state_.acc8();
        const Byte result = rotate_left(arg);
        state_.acc8();
    }

    Word Alu::rotate_left(Word arg)
    {
        const uint32_t result = (arg << 1) | carry();
        state_.test_negative((Word)result);
        state_.test_zero((Word)result);
        state_.test_carry_16(result);
        return result;
    }

    Byte Alu::rotate_left(Byte arg)
    {
        const uint32_t result = (arg << 1) | carry();
        state_.test_negative((Byte)result);
        state_.test_zero((Byte)result);
        state_.test_carry_8(result);
        return result;
    }

    void Alu::rotate_right_a16(void)
    {
        const Word arg = state_.acc16();
        const Word result = rotate_right(arg);
        state_.acc16() = result;
    }

    void Alu::rotate_right_a8(void)
    {
        const Byte arg = state_.acc8();
        const Byte result = rotate_right(arg);
        state_.acc8() = result;
    }

    Word Alu::rotate_right(Word arg)
    {
        const Word result = (arg >> 1) | (carry() << 15);
        const bool carry = arg & 1;
        if (carry)
        {
            state_.set_status_carry();
        }
        else
        {
            state_.clear_status_carry();
        }
        state_.test_negative((Word)result);
        state_.test_zero((Word)result);
        return result;
    }

    Byte Alu::rotate_right(Byte arg)
    {
        const Byte result = (arg >> 1) | (carry() << 7);
        const bool carry = arg & 1;
        if (carry)
        {
            state_.set_status_carry();
        }
        else
        {
            state_.clear_status_carry();
        }
        state_.test_negative((Byte)result);
        state_.test_zero((Byte)result);
        return result;
    }

    void Alu::bit(Word arg)
    {
        const Word acc = state_.acc16();
        const Word result = acc & arg;
        const bool isNegative = (arg >> 8) & CpuState::kNegative; // Use bit 16 if arg is a Word
        const bool isOverflow = (arg >> 8) & CpuState::kOverflow; // Use bit 15 if arg is a Word

        if (isNegative)
        {
            state_.set_status_negative();
        }
        else
        {
            state_.clear_status_negative();
        }

        if (isOverflow)
        {
            state_.set_status_overflow();
        }
        else
        {
            state_.clear_status_overflow();
        }

        state_.test_zero(result);
    }

    void Alu::bit(Byte arg)
    {
        const Byte acc = state_.acc8();
        const Byte result = acc & arg;
        const bool isNegative = arg & CpuState::kNegative;
        const bool isOverflow = arg & CpuState::kOverflow;

        if (isNegative)
        {
            state_.set_status_negative();
        }
        else
        {
            state_.clear_status_negative();
        }

        if (isOverflow)
        {
            state_.set_status_overflow();
        }
        else
        {
            state_.clear_status_overflow();
        }

        state_.test_zero(result);
    }

    void Alu::compare_with_acc(Word arg)
    {
        const Word acc = state_.acc16();
        state_.clear_status_negative();
        state_.clear_status_zero();
        state_.clear_status_carry();

        if (acc > arg)
        {
            state_.set_status_carry();
        }

        if (acc == arg)
        {
            state_.set_status_zero();
        }

        if (acc < arg)
        {
            state_.set_status_negative();
        }
    }

    void Alu::compare_with_acc(Byte arg)
    {
        const Byte acc = state_.acc8();
        state_.clear_status_negative();
        state_.clear_status_zero();
        state_.clear_status_carry();

        if (acc > arg)
        {
            state_.set_status_carry();
        }

        if (acc == arg)
        {
            state_.set_status_zero();
        }

        if (acc < arg)
        {
            state_.set_status_negative();
        }
    }

    void Alu::compare_with_x(Word arg)
    {
        const Word x = state_.x16();
        state_.clear_status_negative();
        state_.clear_status_zero();
        state_.clear_status_carry();

        if (x > arg)
        {
            state_.set_status_carry();
        }

        if (x == arg)
        {
            state_.set_status_zero();
        }

        if (x < arg)
        {
            state_.set_status_negative();
        }
    }

    void Alu::compare_with_x(Byte arg)
    {
        const Byte x = state_.x8();
        state_.clear_status_negative();
        state_.clear_status_zero();
        state_.clear_status_carry();

        if (x > arg)
        {
            state_.set_status_carry();
        }

        if (x == arg)
        {
            state_.set_status_zero();
        }

        if (x < arg)
        {
            state_.set_status_negative();
        }
    }

    void Alu::compare_with_y(Word arg)
    {
        const Word y = state_.y16();
        state_.clear_status_negative();
        state_.clear_status_zero();
        state_.clear_status_carry();

        if (y > arg)
        {
            state_.set_status_carry();
        }

        if (y == arg)
        {
            state_.set_status_zero();
        }

        if (y < arg)
        {
            state_.set_status_negative();
        }
    }

    void Alu::compare_with_y(Byte arg)
    {
        const Byte y = state_.y8();
        state_.clear_status_negative();
        state_.clear_status_zero();
        state_.clear_status_carry();

        if (y > arg)
        {
            state_.set_status_carry();
        }

        if (y == arg)
        {
            state_.set_status_zero();
        }

        if (y < arg)
        {
            state_.set_status_negative();
        }
    }

    Word Alu::decrement(Word arg)
    {
        const Word result = arg - 1;
        state_.test_negative(result);
        state_.test_zero(result);
        return result;
    }

    Byte Alu::decrement(Byte arg)
    {
        const Byte result = arg - 1;
        state_.test_negative(result);
        state_.test_zero(result);
        return result;
    }

    void Alu::decrement_a16(void)
    {
        const Word arg = state_.acc16();
        const Word result = decrement(arg);
        state_.acc16() = result;
    }

    void Alu::decrement_a8(void)
    {
        const Byte arg = state_.acc8();
        const Byte result = decrement(arg);
        state_.acc8() = result;
    }

    void Alu::decrement_x16(void)
    {
        const Word arg = state_.x16();
        const Word result = decrement(arg);
        state_.x16() = result;
    }

    void Alu::decrement_x8(void)
    {
        const Byte arg = state_.x8();
        const Byte result = decrement(arg);
        state_.x8() = result;
    }

    void Alu::decrement_y16(void)
    {
        const Word arg = state_.y16();
        const Word result = decrement(arg);
        state_.y16() = result;
    }

    void Alu::decrement_y8(void)
    {
        const Byte arg = state_.y8();
        const Byte result = decrement(arg);
        state_.y8() = result;
    }

    Word Alu::increment(Word arg)
    {
        const Word result = arg + 1;
        state_.test_negative(result);
        state_.test_zero(result);
        return result;
    }

    Byte Alu::increment(Byte arg)
    {
        const Byte result = arg + 1;
        state_.test_negative(result);
        state_.test_zero(result);
        return result;
    }

    void Alu::increment_a16(void)
    {
        const Word arg = state_.acc16();
        const Word result = increment(arg);
        state_.acc16() = arg;
    }

    void Alu::increment_a8(void)
    {
        const Byte arg = state_.acc8();
        const Byte result = increment(arg);
        state_.acc8() = result;
    }

    void Alu::increment_x16(void)
    {
        const Word arg = state_.x16();
        const Word result = increment(arg);
        state_.x16() = arg;
    }

    void Alu::increment_x8(void)
    {
        const Byte arg = state_.x8();
        const Byte result = increment(arg);
        state_.x8() = result;
    }

    void Alu::increment_y16(void)
    {
        const Word arg = state_.y16();
        const Word result = increment(arg);
        state_.y16() = arg;
    }

    void Alu::increment_y8(void)
    {
        const Byte arg = state_.y8();
        const Byte result = increment(arg);
        state_.y8() = result;
    }

    void Alu::exchange_acc(void)
    {
        const Word acc = state_.acc16();
        const Word result = (acc >> 8) | (acc << 8);
        state_.acc16() = result;
        state_.test_negative(state_.acc8());
        state_.test_zero(state_.acc8());
    }

    uint8_t Alu::carry(void)
    {
        uint8_t result = state_.is_status_carry() ? 1 : 0;
        return result;
    }

} // namespace snes
