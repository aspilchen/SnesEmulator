#include "DecodeUnit.hpp"
#include "../../ISA/ISA.hpp"

#include <vector>

using namespace snes;

int main() {
	std::vector<uint8_t> memory(64, 0);
	std::vector<uint8_t> cis(4, 0);

	DecodeUnit decoder(cis, memory.begin());

	cis[0] = ADC4;
	cis[1] = 10;
	decoder.decode();

	cis[0] = ADC0;
	decoder.decode();
}