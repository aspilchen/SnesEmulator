# About
Currently emulates the Ricoh5A22 (Super Nintendo CPU).

I tried to automate as much of the instruction implementation as I could using python. The tools folder has some hacky scripts that scrape a table from a website containing information on all the instructions. And then uses that information to generate as many instructions as I could. Some just generate empty functions as placeholders because it was easier to implement them manually.

The Ricoh5A22 class is basically the CPUs interface to access instructions, and perform a fetch/execute cycle. Ricoh5A22 is the owner of the CpuState, and some helper classes that make things a little easier. It has a "disassembly mode" to output whatever its .

The instruction implementations are separated by category using the format ricoh5a22.**category**.cc.

The test folder just has a placeholder for now. For my purposes, it is not worth testing each instruction and status flag individually. Eventually I will have some more serious tests that involve loading/executing binary files.

There is still a lot of work to be done. However I may be moving this project from C++ to Rust to get more practice with that language. So I may just finish the CPU with C++ to have a working model in a language I am familiar with. And then try to re-create it with Rust, and progress from there.