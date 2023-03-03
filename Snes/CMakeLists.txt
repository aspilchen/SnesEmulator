cmake_minimum_required(VERSION 3.7)


project(SNESEmulator VERSION 1.0
                  DESCRIPTION "Emulator for the SNES"
                  LANGUAGES CXX)


option(PACKAGE_TESTS "Build the tests" ON)
if(PACKAGE_TESTS)
    enable_testing()
    add_subdirectory(Test)
endif()



# add_subdirectory(Test)