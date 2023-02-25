/**
 * @file Ricoh5A22.cpp
 * @author Adam Spilchen
 * @brief
 * @version 0.1
 * @date 2023-02-17
 *
 * @copyright Copyright (c) 2023
 *
 * Emulates the SNES CPU. The fetch-decode-execute cycle will run in its own
 * thread managed by this class. For now this will hold a mutex to lock the
 * thread at the beginning of each cycle. Calling the "cycle()" method will
 * unlock the mutex, causing the thread to execute one cycle.
 *
 * There is a list of how many cycles each instruction should take. At some
 * point I will add a clock feature that counts at the SNES speed of 3.58 MHz
 * to 1.79 MHz.
 *
 * A counter will be set during the fetch stage that will count down at the SNES
 * speed. The host computer will process everything and then wait on the counter
 * to finish before completing the instruction.
 *
 */

#include "Ricoh5A22.hpp"

#include <iostream>

namespace snes {

Ricoh5A22::Ricoh5A22(void) : isRunning(false), {}

void Ricoh5A22::start(void) {
    if (isRunning) {
        return;
    }

    currStage = CpuStage::FETCH;
    isRunning = true;
    mutex.lock();
    thread = std::thread(threadFunc, this);
}

void Ricoh5A22::stop(void) {
    if (!isRunning) {
        return;
    }

    isRunning = false;
    mutex.unlock();
    thread.join();
}

void Ricoh5A22::cycle(void) { mutex.unlock(); }

void Ricoh5A22::fetch(void) {
    std::cout << "fetch\n";  // Temporary
    currStage = CpuStage::EXECUTE;
}

void Ricoh5A22::execute(void) {
    std::cout << "execute\n";  // Temporary
    currStage = CpuStage::FETCH;
}

}  // namespace snes