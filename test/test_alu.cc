#include <gtest/gtest.h>
#include <cpu_state.h>
#include <alu.h>

using namespace snes;

TEST(alu_test, adc16_arith)
{
    CpuState state;
    Alu alu(state);

    state.set_acc((Word)0);

    Word expected = 0;
    for (auto i = 0; i < 100; i++)
    {
        alu.add_with_carry((Word)(i * state.acc16()));
        expected += (i * expected);
        EXPECT_EQ(state.acc16(), expected);
    }
}

TEST(alu_test, adc16_zero_flag)
{
    CpuState state;
    Alu alu(state);
    state.set_acc((Word)0);
    alu.add_with_carry((Word)0);
    EXPECT_TRUE(state.is_status_zero());
    EXPECT_EQ(state.acc16(), 0);

    alu.add_with_carry((Word)0xFFFF);
    EXPECT_FALSE(state.is_status_zero());
    EXPECT_EQ(state.acc16(), 0xFFFF);

    alu.add_with_carry((Word)1);
    EXPECT_TRUE(state.is_status_zero());
    EXPECT_EQ(state.acc16(), 0);

    state.set_acc((Word)1);
    alu.add_with_carry((Word)-1);
    EXPECT_TRUE(state.is_status_zero());
    EXPECT_EQ(state.acc16(), 0);
}

TEST(alu_test, adc8_zero_flag)
{
    CpuState state;
    Alu alu(state);
    state.set_acc((Byte)0);
    alu.add_with_carry((Byte)0);
    EXPECT_TRUE(state.is_status_zero());
    EXPECT_EQ(state.acc8(), 0);

    alu.add_with_carry((Byte)0xFF);
    EXPECT_FALSE(state.is_status_zero());
    EXPECT_EQ(state.acc8(), 0xFF);

    alu.add_with_carry((Byte)1);
    EXPECT_TRUE(state.is_status_zero());
    EXPECT_EQ(state.acc8(), 0);

    state.set_acc((Byte)1);
    alu.add_with_carry((Byte)-1);
    EXPECT_TRUE(state.is_status_zero());
    EXPECT_EQ(state.acc8(), 0);
}

TEST(alu_test, adc16_negative_flag)
{
    CpuState state;
    Alu alu(state);
    state.set_acc((Word)0);
    alu.add_with_carry((Word)0);
    EXPECT_FALSE(state.is_status_negative());
    EXPECT_EQ(state.acc16(), 0);

    alu.add_with_carry((Word)0xFFFF);
    EXPECT_TRUE(state.is_status_negative());
    EXPECT_EQ(state.acc16(), 0xFFFF);

    alu.add_with_carry((Word)1);
    EXPECT_TRUE(state.is_status_zero());
    EXPECT_EQ(state.acc16(), 0);

    state.set_acc((Word)1);
    alu.add_with_carry((Word)-1);
    EXPECT_TRUE(state.is_status_zero());
    EXPECT_EQ(state.acc16(), 0);
}