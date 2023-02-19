# SnesEmulator
Experimental SNES emulator. Attempting to do this from scratch with no 3rd party libraries just to learn. The goal is to keep it as simple 
and easy as possible. I am not trying to accurately represent the hardware, just the function.

System timing will be a huge factor in the future. For now I just want it to work. Trying to design things with a system clock in mind. I
don't know how this will perform in reality, but it will be a neat experiment.

Optimization will come last (if needed) once things actually work.

# Some things to do
- Create a memory map to store the SNES game data, and map SNES addresses to real addresses
	- Mapper has to determine ROM memory type (HiRom, LowRom etc..) and find header to provide reset vector
- Successfully simulate booting a game
- Successfully fetch-decode-execute instructions in the correct order
- Set up build system

# General Concept
Each hardware unit (CPU, etc..) will execute in its own thread. The class managing that units thread will act as the "physical" unit,
holding any registers or bus connections to other units as needed.

I want to take advantage of modern hardware to (hopefully) execute the instructions and whatever else faster than the actual SNES would. But
keep a record of how many cycles each action SHOULD take, and use the clock to keep things running at the correct speed. Otherwise games
would be running in hyperspeed. If the host computer can act faster than the SNES clock, my hope is that this will prevent the slow-down 
issues the SNES experiences when too much is happening at once.