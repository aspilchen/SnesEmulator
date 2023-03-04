#include <catch2/catch_test_macros.hpp>
#include <cstring>
#include <vector>

#include "../Isa/Isa.hpp"

using namespace snes;

TEST_CASE("Addressing Mode Immediate", "[addressing][immediate]") {
    const int MAX_SIZE = 3;
    std::vector<uint8_t> data = {0x01, 0x00, 0x02, 0x10, 0x20, 0x30};
    for (auto size = 1; size < MAX_SIZE; size++) {
        for (auto start = 0; start < data.size() - size + 1; start++) {
            uint32_t expected = 0;
            uint32_t actual = addrImmediate(&data[start], size);
            std::memcpy(&expected, &data[start], size);
            REQUIRE(actual == expected);
        }
    }
}

TEST_CASE("Addressing Mode Direct Page", "[Addressing Mode]") {
    std::vector<uint8_t> data = {1,  2,  3,  4,  5,  6,  7,  8,  9, 10,
                                 11, 12, 13, 14, 15, 16, 17, 18, 19};
    SystemState state;

    SECTION("D=0") {
        state.registers.d = 0;
        for (auto i : data) {
            uint32_t expected = data[i];
            uint32_t actual = addrDP(&data[i], state);
            REQUIRE(actual == expected);
        }
    }

    SECTION("D=1") {
        state.registers.d = 1;
        for (auto i = 0; i < data.size() - state.registers.d; i++) {
            uint32_t expected = data[i + state.registers.d];
            uint32_t actual = addrDP(&data[i], state);
            REQUIRE(actual == expected);
        }
    }

    SECTION("D=10") {
        state.registers.d = 10;
        for (auto i = 0; i < data.size() - state.registers.d; i++) {
            uint32_t expected = data[i + state.registers.d];
            uint32_t actual = addrDP(&data[i], state);
            REQUIRE(actual == expected);
        }
    }
}