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

// #include "../../SysClock/SysClock.hpp"
// #include "../../SysClockObserver/SysClockObserver.hpp"

#include <iostream>
#include <mutex>
#include <thread>
#include <functional>

namespace snes {
class Ricoh5A22 {
   public:
    enum class CpuStage {
        FETCH,
        DECODE,
        EXECUTE
    };

    Ricoh5A22(void);

    // void attachClock(SysClock &clock);
    void start(void);
    void stop(void);
    void cycle(void);

   private:
    static void threadFunc(Ricoh5A22 *cpu) {
        while (cpu->isRunning) {
            cpu->mutex.lock();
            switch (cpu->currStage)
            {
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
    // SysClockObserver clockObserver;
};
}  // namespace snes

#endif