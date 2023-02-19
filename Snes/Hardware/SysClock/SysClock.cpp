#include "SysClock.hpp"

namespace snes {


    SysClockObserver SysClock::getObserver(void) {
		return SysClockObserver(conditionalMutex, &conditionVar);
	}

    void SysClock::cycle(void) {
		conditionVar.notify_all();
	}

    void SysClock::start(void) {};

    void SysClock::stop(void) {};

}  // namespace snes