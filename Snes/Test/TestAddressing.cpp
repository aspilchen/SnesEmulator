#include <catch2/catch_test_macros.hpp>
#include <vector>

#include "../Isa/Isa.hpp"
#include <cstring>

using namespace snes;

TEST_CASE( "Addressing Mode Immediate", "[Addressing Mode]" ) {
    std::vector<uint8_t> data = {0x01, 0x00, 0x02, 0x10, 0x20, 0x30};
    const int MAX_SIZE = 3;
    for(auto size = 1; size < MAX_SIZE; size++) {
        for(auto start = 0; start < data.size() - size + 1; start++) {
            uint32_t expected = 0;
            uint32_t actual = addrImmediate(&data[start], size);
            std::memcpy(&expected, &data[start], size);
            REQUIRE(actual == expected);
        }
    }
}

TEST_CASE( "Addressing Mode Direct Page", "[Addressing Mode]" ) {
    std::vector<uint8_t> data = {0x01, 0x00, 0x02, 0x10, 0x20, 0x30};
    SystemState state;
    state.registers.p = 0;
    
    SECTION("Register D = 0"){}

}