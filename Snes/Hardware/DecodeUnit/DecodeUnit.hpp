#ifndef DECODE_UNIT_HPP
#define DECODE_UNIT_HPP

#include <cstdint>
#include <vector>

namespace snes {
class DecodeUnit {
   public:
    DecodeUnit(std::vector<uint8_t> &currentInstructionRegister);

    void decode(void);

   private:
    std::vector<uint8_t> &cir;
};
}  // namespace snes

#endif