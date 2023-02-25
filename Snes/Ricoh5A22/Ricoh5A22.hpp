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
    using Memory = std::vector<uint8_t>;
    using ProgramCounter = Memory::iterator;


    enum class CpuStage { FETCH, DECODE, EXECUTE };

    Ricoh5A22(void);

    void start(void);
    void stop(void);
    void cycle(void);

   private:
    static void threadFunc(Ricoh5A22* cpu) {
        while (cpu->isRunning) {
            cpu->mutex.lock();
            switch (cpu->currStage) {
                case CpuStage::FETCH:
                    break;
                case CpuStage::EXECUTE:
                    break;
            }
        }
    }

    void fetch(void);
    void execute(void);

    CpuStage currStage;
    bool isRunning;
    std::thread thread;
    std::mutex mutex;
};
}  // namespace snes

#endif