#ifndef FETCH_UNIT_HPP
#define FETCH_UNIT_HPP

#include <cstdint>
#include <vector>

namespace snes {
class FetchUnit {
   public:
    FetchUnit(std::vector<uint8_t> &memoryBufferRegister);

    void setProgramCounter(std::vector<uint8_t>::iterator programCounter);

    void fetch(void);

   private:
    std::vector<uint8_t>::iterator pc;
    std::vector<uint8_t>::iterator mar;
    std::vector<uint8_t> &mbr;
};
}  // namespace snes

#endif