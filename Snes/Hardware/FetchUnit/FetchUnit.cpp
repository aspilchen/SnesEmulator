#include "FetchUnit.hpp"

#include <iostream>

#include "../../ISA/ISA.hpp"

namespace snes {

FetchUnit::FetchUnit(std::vector<uint8_t> &currentInstructionRegister)
    : cir(currentInstructionRegister) {}

void FetchUnit::setProgramCounter(
    std::vector<uint8_t>::iterator programCounter) {
    pc = programCounter;
}

std::vector<uint8_t>::iterator FetchUnit::getProgramCounter(void) { return pc; }

void FetchUnit::fetch(void) {
    mar = pc;
    pc += INSTRUCTION_SIZES[*pc];
    cir.assign(mar, mar + INSTRUCTION_SIZES[*mar]);
}

}  // namespace snes