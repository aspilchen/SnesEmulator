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
 * unlock the mutex, causing the thread to execute one cycle. This is used to
 * simulate the system timing, an external clock can control the speed of the
 * system. There is a skeleton in place to use a conditional_variable that
 * allows one call from the clock to signal all waiting threads. However that
 * will require the clock to be more fleshed out and running on its own thread
 * as well, and the program must be terminated in a specific order with the
 * clock last to prevent deadlocks. So for now, the cycle will be used to keep
 * it simple.
 *
 *
 */

#include "Ricoh5A22.hpp"

#include <iostream>

namespace snes {

Ricoh5A22::Ricoh5A22(void) : isRunning(false) {}

//    void Ricoh5A22::attachClock(SysClock &clk) {
//		clockObserver = clk.getObserver();
//	}

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
    currStage = CpuStage::DECODE;
}

void Ricoh5A22::decode(void) {
    std::cout << "decode\n"; // Temporary
    currStage = CpuStage::EXECUTE;
}

void Ricoh5A22::execute(void) {
    std::cout << "execute\n"; // Temporary
    currStage = CpuStage::FETCH;
}

}  // namespace snes