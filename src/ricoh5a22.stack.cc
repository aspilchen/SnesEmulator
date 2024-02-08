#include "ricoh5a22.h"

namespace snes
{
    void Ricoh5A22::opF4_PEA(void)
    {
        const Word arg = addr_immediate16();
        SnesPtr &stack_ptr = state_.stack();
        stack_ptr -= sizeof(arg);
        mem_.write(stack_ptr, arg);
    }

    void Ricoh5A22::opD4_PEI(void)
    {
        const SnesPtr arg_ptr = addr_direct_indirect();
        const Word arg = mem_.get_word(arg_ptr);
        SnesPtr &stack_ptr = state_.stack();
        stack_ptr -= sizeof(arg);
        mem_.write(stack_ptr, arg);
    }

    void Ricoh5A22::op62_PER(void)
    {
        const SnesPtr arg_ptr = addr_pc_relative_long();
        const Word arg = mem_.get_word(arg_ptr);
        SnesPtr &stack_ptr = state_.stack();
        stack_ptr -= sizeof(arg);
        mem_.write(stack_ptr, arg);
    }

    void Ricoh5A22::op48_PHA(void)
    {
        SnesPtr &stack_ptr = state_.stack();
        if (state_.is_status_memory())
        {
            const Byte arg = state_.acc8();
            stack_ptr -= sizeof(arg);
            mem_.write(stack_ptr, arg);
        }
        else
        {
            const Word arg = state_.acc16();
            stack_ptr -= sizeof(arg);
            mem_.write(stack_ptr, arg);
        }
    }

    void Ricoh5A22::op8B_PHB(void)
    {
        const Byte arg = state_.dbr();
        SnesPtr &stack_ptr = state_.stack();
        stack_ptr -= sizeof(arg);
        mem_.write(stack_ptr, arg);
    }

    void Ricoh5A22::op0B_PHD(void)
    {
        const Word arg = state_.dp();
        SnesPtr &stack_ptr = state_.stack();
        stack_ptr -= sizeof(arg);
        mem_.write(stack_ptr, arg);
    }

    void Ricoh5A22::op4B_PHK(void)
    {
        const Byte arg = state_.pbr();
        SnesPtr &stack_ptr = state_.stack();
        stack_ptr -= sizeof(arg);
        mem_.write(stack_ptr, arg);
    }

    void Ricoh5A22::op08_PHP(void)
    {
        const Word arg = state_.status();
        SnesPtr &stack_ptr = state_.stack();
        stack_ptr -= sizeof(arg);
        mem_.write(stack_ptr, arg);
    }

    void Ricoh5A22::opDA_PHX(void)
    {
        SnesPtr &stack_ptr = state_.stack();
        if (state_.is_status_index())
        {
            const Byte arg = state_.x8();
            stack_ptr -= sizeof(arg);
            mem_.write(stack_ptr, arg);
        }
        else
        {
            const Word arg = state_.x16();
            stack_ptr -= sizeof(arg);
            mem_.write(stack_ptr, arg);
        }
    }

    void Ricoh5A22::op5A_PHY(void)
    {
        SnesPtr &stack_ptr = state_.stack();
        if (state_.is_status_index())
        {
            const Byte arg = state_.y8();
            stack_ptr -= sizeof(arg);
            mem_.write(stack_ptr, arg);
        }
        else
        {
            const Word arg = state_.y16();
            stack_ptr -= sizeof(arg);
            mem_.write(stack_ptr, arg);
        }
    }

    void Ricoh5A22::op68_PLA(void)
    {
        SnesPtr &stack_ptr = state_.stack();
        if (state_.is_status_memory())
        {
            const Byte arg = mem_.get_byte(stack_ptr);
            stack_ptr -= sizeof(arg);
            mem_.write(stack_ptr, arg);
        }
        else
        {
            const Word arg = mem_.get_word(stack_ptr);
            stack_ptr -= sizeof(arg);
            mem_.write(stack_ptr, arg);
        }
    }

    void Ricoh5A22::opAB_PLB(void)
    {
        const SnesPtr stack_ptr = state_.stack();
        const Byte arg = mem_.get_byte(stack_ptr);
        state_.dbr() = arg;
        state_.stack() += sizeof(arg);
    }

    void Ricoh5A22::op2B_PLD(void)
    {
        const SnesPtr stack_ptr = state_.stack();
        const Word arg = mem_.get_word(stack_ptr);
        state_.dp() = arg;
        state_.stack() += sizeof(arg);
    }

    void Ricoh5A22::op28_PLP(void)
    {
        const SnesPtr stack_ptr = state_.stack();
        const Byte arg = mem_.get_byte(stack_ptr);
        state_.status() = arg;
        state_.stack() += sizeof(arg);
    }

    void Ricoh5A22::opFA_PLX(void)
    {
        const SnesPtr stack_ptr = state_.stack();
        if (state_.is_status_index())
        {
            const Byte arg = mem_.get_byte(stack_ptr);
            state_.set_x(arg);
            state_.stack() += sizeof(arg);
        }
        else
        {
            const Word arg = mem_.get_word(stack_ptr);
            state_.set_x(arg);
            state_.stack() += sizeof(arg);
        }
    }

    void Ricoh5A22::op7A_PLY(void)
    {
        const SnesPtr stack_ptr = state_.stack();
        if (state_.is_status_index())
        {
            const Byte arg = mem_.get_byte(stack_ptr);
            state_.set_y(arg);
            state_.stack() += sizeof(arg);
        }
        else
        {
            const Word arg = mem_.get_word(stack_ptr);
            state_.set_y(arg);
            state_.stack() += sizeof(arg);
        }
    }

} // namespace snes