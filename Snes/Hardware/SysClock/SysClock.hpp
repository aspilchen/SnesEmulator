// Experiment for later. Try to synchronize the hardware emulation to run at the
// SNES speed. Each hardware thread will have an observer that waits for a
// signal sent by this clock.

#ifndef SYSCLOCK_HPP
#define SYSCLOCK_HPP

#include "../../SysClockObserver/SysClockObserver.hpp"

#include <thread>
#include <condition_variable>

namespace snes {

class SysClock {
   public:

    SysClock(void) = default;

    SysClockObserver getObserver(void);

    void cycle(void);
    void start(void);
    void stop(void);


   private:
    

    std::mutex conditionalMutex;
    std::condition_variable conditionVar;
};
}  // namespace snes

#endif