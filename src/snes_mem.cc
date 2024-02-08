#include "snes_mem.h"

#include <cstring>

namespace snes
{
    SnesMemory::SnesMemory(void)
    {
        mem_.resize(1024);
    }

    void SnesMemory::get(void *dest, SnesPtr src, int n_bytes)
    {
        std::memcpy(dest, &mem_[src], n_bytes);
    }

    Word SnesMemory::get_word(SnesPtr src)
    {
        Word result;
        std::memcpy(&result, &mem_[src], sizeof(Word));
        return result;
    }

    Byte SnesMemory::get_byte(SnesPtr src)
    {
        return mem_[src];
    }

    void SnesMemory::write(SnesPtr dest, Word val)
    {
        std::memcpy(&mem_[dest], &val, sizeof(val));
    }

    void SnesMemory::write(SnesPtr dest, Byte val)
    {
        mem_[dest] = val;
    }
} // namespace snes
