#ifndef TYPES_HPP
#define TYPES_HPP

#include "../CpuRegisters/CpuRegisters.hpp"

#include <cstdint>
#include <functional>
#include <vector>


namespace snes
{
    using Rom = std::vector<uint8_t>;
    using ProgramCounter = Rom::iterator;
    using Instruction = std::function<void(CpuRegisters&)>;

} // namespace snes


#endif