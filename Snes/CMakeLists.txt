cmake_minimum_required(VERSION 3.7)


project(SNESEmulator VERSION 1.0
                  DESCRIPTION "Emulator for the SNES"
                  LANGUAGES CXX)

add_subdirectory(Hardware)
add_subdirectory(Test)