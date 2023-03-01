#ifndef MEMORY_MAP_HPP
#define MEMORY_MAP_HPP

#include <cstdint>
#include <vector>

namespace snes {
class MemoryMap {
   public:
	using Memory = std::vector<uint8_t>;
   using MemPtr = Memory::iterator;

   MemoryMap(void) = default;


   Memory mem;
};
}  // namespace snes

#endif