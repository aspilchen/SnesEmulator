// #include <cpu_core.h>
#include <ricoh5a22.h>

#include <memory.h>
#include <variant>
#include <iostream>
#include <bitset>

using namespace snes;

void PrintState(CpuState state) {
    std::bitset<16> status(state.status());
    std::cout << "accumulator: (word)" << state.acc16() << " (byte)" << (int)state.acc8() << "\n" 
        << "y_index " << state.y16() << "\n" 
        << "x_index " << state.x8() << "\n" 
        << "stack_pointer " << (int)state.stack() << "\n" 
        << "data_bank " << (int)state.dbr() << "\n" 
        << "direct_page " << (int)state.dp() << "\n" 
        << "program_bank " << (int)state.pbr() << "\n" 
        << "status " << status << "\n" 
        << "program_counter " << (int)state.pc() << "\n";
}



int main(int, char**){
    SnesMemory mem;
    Ricoh5A22 cpu(mem);
    
    mem.write(0, (Byte)0x69);
    mem.write(1, (Word)10);
    mem.write(3, (Byte)0x69);
    mem.write(4, (Word)20);
    cpu.execute();
    cpu.execute();

    PrintState(cpu.state());
}
