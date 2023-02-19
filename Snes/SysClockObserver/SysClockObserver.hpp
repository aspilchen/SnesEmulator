#ifndef SYSCLOCK_OBSERVER_HPP
#define SYSCLOCK_OBSERVER_HPP

#include <condition_variable>
#include <thread>

namespace snes {
class SysClockObserver {
   public:
    SysClockObserver(void) = default;
    SysClockObserver(std::mutex &mutex, std::condition_variable *cv);

    SysClockObserver &operator=(SysClockObserver &&observer) {
        lock = std::move<>(observer.lock);
        condition = std::move<>(observer.condition);
        return *this;
    }

    void wait(void);

   private:
    std::unique_lock<std::mutex> lock;
    std::condition_variable *condition;
};
}  // namespace snes

#endif