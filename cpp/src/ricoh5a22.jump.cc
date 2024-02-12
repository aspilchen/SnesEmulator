#include "ricoh5a22.h"

namespace snes
{

    void Ricoh5A22::op4C_JMP(void)
    {
        if (verbose_)
        {
            std::cout << "JMP ";
        }
        const SnesPtr arg = addr_absolute();
        state_.pc() = arg;
    }

    void Ricoh5A22::op5C_JMP(void)
    {
        if (verbose_)
        {
            std::cout << "JMP ";
        }
        const SnesPtr arg = addr_absolute_long();
        state_.pc() = arg;
    }

    void Ricoh5A22::op6C_JMP(void)
    {
        if (verbose_)
        {
            std::cout << "JMP ";
        }
        const SnesPtr arg = addr_absolute_indirect();
        state_.pc() = arg;
    }

    void Ricoh5A22::op7C_JMP(void)
    {
        if (verbose_)
        {
            std::cout << "JMP ";
        }
        const SnesPtr arg = addr_absolute_indexed_indirect();
        state_.pc() = arg;
    }

    void Ricoh5A22::opDC_JMP(void)
    {
        // TODO: implement opDC_JMP
    }

    void Ricoh5A22::op20_JSR(void)
    {
        if (verbose_)
        {
            std::cout << "JSR ";
        }
        const SnesPtr arg = addr_absolute();
        const Word pc = state_.pc();
        SnesPtr &stack_ptr = state_.stack();
        stack_ptr -= sizeof(Word);
        mem_.write(stack_ptr, pc);
        state_.pc() = arg;
    }

    void Ricoh5A22::op22_JSR(void)
    {
        if (verbose_)
        {
            std::cout << "JSR ";
        }
        const SnesPtr arg = addr_absolute_long();
        const Word pc = state_.pc();
        const Byte dbr = state_.dbr();
        SnesPtr &stack_ptr = state_.stack();
        stack_ptr -= sizeof(dbr);
        mem_.write(stack_ptr, dbr);
        stack_ptr -= sizeof(pc);
        mem_.write(stack_ptr, pc);
        state_.pc() = arg;
    }

    void Ricoh5A22::opFC_JSR(void)
    {
        if (verbose_)
        {
            std::cout << "JSR ";
        }
        const SnesPtr arg = addr_absolute_indexed_indirect();
        const Word pc = state_.pc();
        SnesPtr &stack_ptr = state_.stack();
        stack_ptr -= sizeof(pc);
        mem_.write(stack_ptr, pc);
        state_.pc() = arg;
    }

    void Ricoh5A22::op6B_RTL(void)
    {
        if (verbose_)
        {
            std::cout << "RTL ";
        }
        SnesPtr &stack_ptr = state_.stack();
        const Word new_pc = mem_.get_word(stack_ptr);
        stack_ptr += sizeof(new_pc);
        const Byte new_dbr = mem_.get_byte(stack_ptr);
        stack_ptr += sizeof(new_dbr);
        state_.dbr() = new_dbr;
        state_.pc() = new_pc;
    }

    void Ricoh5A22::op60_RTS(void)
    {
        if (verbose_)
        {
            std::cout << "RTS ";
        }
        SnesPtr &stack_ptr = state_.stack();
        const Word new_pc = mem_.get_word(stack_ptr);
        state_.stack() += sizeof(new_pc);
        state_.pc() = new_pc;
    }

} // namespace snes