// #include <cpu_core.h>
#include <ricoh5a22.h>

#include <memory.h>
#include <variant>
#include <iostream>
#include <bitset>

using namespace snes;

void PrintState(CpuState state)
{
    std::bitset<16> status(state.status());
    std::cout << "accumulator: (word)" << state.acc16() << " (byte)" << (int)state.acc8() << "\n"
              << "y_index " << state.y16() << "\n"
              << "x_index " << state.x8() << "\n"
              << "stack_pointer " << (int)state.stack() << "\n"
              << "data_bank " << (int)state.dbr() << "\n"
              << "direct_page " << (int)state.dp() << "\n"
              << "program_bank " << (int)state.pbr() << "\n"
              << "status " << status << "\n"
              << "program_counter " << (int)state.pc() << "\n\n";
}

int main(int, char **)
{
    SnesMemory mem;
    Ricoh5A22 cpu(mem);

    cpu.set_verbose(true);

    // lda 10
    mem.write(0, (Byte)0xA9);
    mem.write(1, (Word)10);

    // cmp 30
    mem.write(3, (Byte)0xC9);
    mem.write(4, (Word)30);

    // branch if equal to 50
    mem.write(6, (Byte)0xF0);
    mem.write(7, (Byte)42);

    // jsr add 10
    mem.write(8, (Byte)0x20);
    mem.write(9, (Word)20);

    // jump 3
    mem.write(11, (Byte)0x4C);
    mem.write(12, (Word)3);

    // subroutine add 10
    mem.write(20, (Byte)0x69);
    mem.write(21, (Word)10);
    mem.write(23, (Byte)0x60);

    mem.write(50, (Byte)0x69);
    mem.write(51, (Word)5);

    int max_cycles = 20;
    for (auto i = 0; i < max_cycles; i++)
    {
        cpu.execute();
    }

    std::cout << "done\n";
}
