#ifndef CPU_DATA_HPP
#define CPU_DATA_HPP

#include <cstdint>
#include <vector>

namespace snes {
class CpuData {
   public:
    
    CpuData(void) = default;

    enum StatusFlag {
        N = 1 << 7,  // #$80 10000000 Negative
        V = 1 << 6,  // #$40 01000000 Overflow
        M = 1 << 5,  // #$20 00100000 Accumulator register size
                     // (native mode only), (0 = 16-bit, 1 = 8-bit)
        X = 1 << 4,  // #$10 00010000 Index register size (native
                     // mode only), (0 = 16-bit, 1 = 8-bit)
        D = 1 << 3,  // #$08 00001000 Decimal
        I = 1 << 2,  // #$04 00000100 IRQ disable
        Z = 1 << 1,  // #$02 00000010 Zero
        C = 1,       // #$01 00000001 Carry
        E = 1,       // 6502 emulation mode
        B = 1 << 4   // #$10 00010000 Break (emulation mode only)
    };

    // void setStatusFlags(uint8_t bitFlags);
    // void clearStatusFlags(uint8_t bitFlags);


    // Accumulator
    uint16_t a;  // The accumulator. This is the math register.
                 // It stores one of two operands or the result of most
                 // arithmetic and logical operations.

    // Index            
    uint16_t x, y;  // The index registers. These can be used
                    // to reference memory, to pass data to memory, or as
                    // counters for loops.

    // Stack Pointer    
    uint16_t s;     // The stack pointer, points to the next
                    // available(unused) location on the stack.

    // Data Bank        
    uint16_t dbr;   // Data bank register, holds the default
                    // bank for memory transfers.

    // Direct Page      
    uint16_t d;   // Direct page register, used for direct page
                  // addressing modes. Holds the memory bank address of the data
                  // the CPU is accessing.

    // Program Bank     
    uint16_t pb;  // Program Bank, holds the bank address of
                  // all instruction fetches.

    // Processor Status 
    uint16_t p;   // Holds various important flags, results of
                  // tests and 65816 processing states.

    // Program Counter
    std::vector<uint8_t>::iterator pc;  // Holds the memory address of the
                                        // current CPU instruction


    uint16_t mbr; // Holds fetched data, or data waiting to be written

    std::vector<uint8_t> rom;
};
}  // namespace snes

#endif