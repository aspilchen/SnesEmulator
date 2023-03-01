
#include <cstring>
#include <iostream>
#include <cstdint>
#include <vector>


int main() {
	std::vector<uint8_t> mem = {0x01, 0x00, 0x33, 0x44, 0x55};
	uint32_t val = 0;
	auto start = mem.begin();
	std::memcpy(&val, mem.data(), 2);

	std::cout << std::hex << val << "\n";

	val = 0;
	std::memcpy(&val, mem.data(), 3);
	std::cout << std::hex << val << "\n";
}