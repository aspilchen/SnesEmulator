#include "SysClockObserver.hpp"

namespace snes {
SysClockObserver::SysClockObserver(std::mutex &mutex, std::condition_variable *cv)
    : lock(mutex), condition(cv) {}

void SysClockObserver::wait(void) { condition->wait(lock); }

}  // namespace snes
