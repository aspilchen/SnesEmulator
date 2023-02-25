#include "CpuRegisters.hpp"

namespace snes {
CpuRegisters::CpuRegisters(void) : mbr(4, 0), cir(4, 0) {}

void CpuRegisters::setStatusFlags(uint8_t bitFlags) { p |= bitFlags; }

void CpuRegisters::clearStatusFlags(uint8_t bitFlags) { p &= ~bitFlags; }
}  // namespace snes
