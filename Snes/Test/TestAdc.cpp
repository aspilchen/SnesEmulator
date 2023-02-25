
#include "../CpuRegisters/CpuRegisters.hpp"
#include "../Isa/Isa.hpp"

#include <iostream>

using namespace snes;

int main() {
    Rom rom = {ADC4, 0xF0, 0xF0, 5};
    CpuRegisters regs;

    // regs.p = 0 | CpuRegisters::StatusFlag::M;
    regs.p = 0;
    regs.a = 5;
    regs.pc = rom.begin();

    Instruction instr = INSTRUCTION_TABLE[ADC4];
    instr(regs);

    std::cout << std::hex << regs.a << "\n";
    std::cout << static_cast<int>(*(regs.pc)) << "\n";
    std::cout << std::dec << regs.pc - rom.begin() << "\n";
}