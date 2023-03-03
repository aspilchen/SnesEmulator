Include(FetchContent)

FetchContent_Declare(
  Catch2
  GIT_REPOSITORY https://github.com/catchorg/Catch2.git
  GIT_TAG        v3.0.1 # or a later release
)

FetchContent_MakeAvailable(Catch2)

add_executable(TestAddressing TestAddressing.cpp ../Isa/Isa.cpp)
target_link_libraries(TestAddressing PRIVATE Catch2::Catch2WithMain)

include(CTest)
include(Catch)
catch_discover_tests(TestAddressing)