#ifndef SHARED_MEMORY_HPP
#define SHARED_MEMORY_HPP

#include "../Registers/Registers.hpp"
#include "../MemoryMap/MemoryMap.hpp"

namespace snes
{
	class SystemState {
		public:
		SystemState(void) = default;


		Registers registers;
		MemoryMap memMap;
	};
} // namespace snes


#endif