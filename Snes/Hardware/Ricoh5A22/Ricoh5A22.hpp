/**
 * @file Ricoh5A22.hpp
 * @author Adam Spilchen
 * @brief
 * @version 0.1
 * @date 2023-02-17
 *
 * @copyright Copyright (c) 2023
 *
 * Simulate the SNES CPU.
 */

#ifndef RICOH5A22_HPP
#define RICOH5A22_HPP

#include <functional>
#include <iostream>
#include <mutex>
#include <thread>

namespace snes {
class Ricoh5A22 {
   public:
    enum StatusFlag {
        NEGATIVE = 1 << 7,        // N #$80 10000000 Negative
        OVERFLOW = 1 << 6,        // V #$40 01000000 Overflow
        ACCUM_REG_SIZE = 1 << 5,  // M #$20 00100000 Accumulator register size
                                  // (native mode only), (0 = 16-bit, 1 = 8-bit)
        IDX_REG_SIZE = 1 << 4,    // X #$10 00010000 Index register size (native
                                  // mode only), (0 = 16-bit, 1 = 8-bit)
        DECIMAL = 1 << 3,         // D #$08 00001000 Decimal
        IRQ = 1 << 2,             // I #$04 00000100 IRQ disable
        ZERO = 1 << 1,            // Z #$02 00000010 Zero
        CARRY = 1,                // C #$01 00000001 Carry
        EMULATION_MODE = 1,       // E 6502 emulation mode
        BREAK = 1 << 4            // B #$10 00010000 Break (emulation mode only)
    };

    enum class CpuStage { FETCH, DECODE, EXECUTE };

    Ricoh5A22(void);

    void setStatusFlags(uint8_t bitFlags);
    void clearStatusFlags(uint8_t bitFlags);
    uint8_t getProcessorStatus(void);
    
    void start(void);
    void stop(void);
    void cycle(void);
    // void setClockSync(std::function<void(void)> clockSyncFunctor);

   private:
    static void threadFunc(Ricoh5A22 *cpu) {
        while (cpu->isRunning) {
            cpu->mutex.lock();
            switch (cpu->currStage) {
                case CpuStage::FETCH:
                    cpu->fetch();
                    break;
                case CpuStage::DECODE:
                    cpu->decode();
                    break;
                case CpuStage::EXECUTE:
                    cpu->execute();
                    break;
                default:
                    break;
            }
        }
    }

    void fetch(void);
    void decode(void);
    void execute(void);

    CpuStage currStage;
    bool isRunning;
    std::thread thread;
    std::mutex mutex;

    int accumulatorSize;
    int indexSize;

    uint16_t a;                         // accumulator
    uint16_t x, y;                      // index
    uint16_t s;                         // stack pointer
    uint16_t db;                        // data bank
    uint16_t pb;                        // program bank
    uint8_t p;                          // processor status
    std::vector<uint8_t>::iterator pc;  // program counter

    // std::function<void(void)> clockSync;
};
}  // namespace snes

#endif