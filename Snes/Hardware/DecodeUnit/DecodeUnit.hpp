#ifndef DECODE_UNIT_HPP
#define DECODE_UNIT_HPP

#include "../../ISA/ISA.hpp"

#include <cstdint>
#include <vector>

namespace snes {
class DecodeUnit {
   public:
    DecodeUnit(std::vector<uint8_t> &currentInstructionRegister, std::vector<uint8_t>::iterator memory);

    void decode(void);

    uint16_t decodedOperand; // Temporary, will change to appropriate register in the future.
   private:
    std::vector<uint8_t> &cir;
    std::vector<uint8_t>::iterator mem;
};
}  // namespace snes

#endif