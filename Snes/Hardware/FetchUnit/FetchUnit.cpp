#include "FetchUnit.hpp"

#include <iostream>

#include "../../ISA/ISA.hpp"

namespace snes {

FetchUnit::FetchUnit(std::vector<uint8_t> &memoryBufferRegister)
    : mbr(memoryBufferRegister) {}

void FetchUnit::setProgramCounter(std::vector<uint8_t>::iterator programCounter) {
    pc = programCounter;
}

void FetchUnit::fetch(void) {
    mar = pc;
    pc += INSTRUCTION_SIZES[*pc];
    mbr.assign(mar, mar + INSTRUCTION_SIZES[*mar]);
}

}  // namespace snes