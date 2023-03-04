# SnesEmulator
Experimental SNES emulator. Attempting to do this from scratch with minimal 3rd party libraries, however I will likely use a graphics framework for the GUI and displaying the game, as well as a testing framework.

# Goals
Demonstrate a wide range of skills, knowledge and adaptability. Some of the things required for this project are.
- A strong understanding of C++
- Comfort with parallel programming (will be used later)
- Designing and refining the program architecture
- Adabtability and efficiently (ex. creating a code generator in Python to handle large parts of the ISA implementation)
- Deep understanding of computer architecture
- The ability to research and learn about new material
- Establishing and maintaining a build system
- Testing
- Basic use of GIT

# Dependencies
- [CMake V3.7](https://cmake.org/) or greater
- [Catch2](https://github.com/catchorg/Catch2): C++ unit testing framework. Chosen because it is small and fast/easy to set up. CMake will download and include this when the tests are first built.

# Some things to do
### Ricoh5A22 ISA
- [ ] Addressing modes
- [ ] Implement instructions 
- [ ] Inturrupts
- [ ] Test for logical correctness
- [ ] Test for time correctness (optimize if needed)

### Memory Mapping
- [ ] Detect and map simplest game format (LoRom with no peripherals)
- [ ] Add detection and handling for HiRom
- [ ] Handle other formats and peripherals

# References
- [Addressing mode details](https://wiki.superfamicom.org/jay%27s-asm-tutorial)
- [ISA details](https://wiki.superfamicom.org/65816-reference)