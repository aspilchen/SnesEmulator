#include "Isa.hpp"

namespace snes {
void Adc0Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed Indirect,X
    regs.a += arg;     // Temporary, will handle carry later
}

void Adc1Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Stack Relative
    regs.a += arg;     // Temporary, will handle carry later
}

void Adc2Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    regs.a += arg;     // Temporary, will handle carry later
}

void Adc3Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect Long
    regs.a += arg;     // Temporary, will handle carry later
}

void Adc4Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 2;
    size += !(regs.p & CpuRegisters::StatusFlag::M);
    uint32_t arg = FETCH_ARG_FROM_ROM(regs.pc + 1, regs.pc + size);
    regs.pc += size;
    regs.a += arg;  // Temporary, will handle carry later
}

void Adc5Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    regs.a += arg;     // Temporary, will handle carry later
}

void Adc6Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 4;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Long
    regs.a += arg;     // Temporary, will handle carry later
}

void Adc7Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect Indexed, Y
    regs.a += arg;     // Temporary, will handle carry later
}

void Adc8Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect
    regs.a += arg;     // Temporary, will handle carry later
}

void Adc9Func(CpuRegisters &regs) {
    int numCycles = 7;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: SR Indirect Indexed,Y
    regs.a += arg;     // Temporary, will handle carry later
}

void Adc10Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed,X
    regs.a += arg;     // Temporary, will handle carry later
}

void Adc11Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg =
        0;          // Address mode not implemented: DP Indirect Long Indexed, Y
    regs.a += arg;  // Temporary, will handle carry later
}

void Adc12Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,Y
    regs.a += arg;     // Temporary, will handle carry later
}

void Adc13Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,X
    regs.a += arg;     // Temporary, will handle carry later
}

void Adc14Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 4;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Long Indexed,X
    regs.a += arg;     // Temporary, will handle carry later
}

void And0Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed Indirect,X
    // Execution not implemented
}

void And1Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Stack Relative
    // Execution not implemented
}

void And2Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void And3Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect Long
    // Execution not implemented
}

void And4Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 2;
    size += !(regs.p & CpuRegisters::StatusFlag::M);
    // uint32_t arg = FETCH_ARG_FROM_ROM(regs.pc + 1, regs.pc + size);
    // Execution not implemented
}

void And5Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void And6Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 4;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Long
    // Execution not implemented
}

void And7Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect Indexed, Y
    // Execution not implemented
}

void And8Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect
    // Execution not implemented
}

void And9Func(CpuRegisters &regs) {
    int numCycles = 7;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: SR Indirect Indexed,Y
    // Execution not implemented
}

void And10Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed,X
    // Execution not implemented
}

void And11Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg =
        0;  // Address mode not implemented: DP Indirect Long Indexed, Y
    // Execution not implemented
}

void And12Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,Y
    // Execution not implemented
}

void And13Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,X
    // Execution not implemented
}

void And14Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 4;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Long Indexed,X
    // Execution not implemented
}

void Asl0Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void Asl1Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Accumulator
    // Execution not implemented
}

void Asl2Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Asl3Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed,X
    // Execution not implemented
}

void Asl4Func(CpuRegisters &regs) {
    int numCycles = 7;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,X
    // Execution not implemented
}

void Bcc0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Program Counter Relative
    // Execution not implemented
}

void Bcs0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Program Counter Relative
    // Execution not implemented
}

void Beq0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Program Counter Relative
    // Execution not implemented
}

void Bit0Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void Bit1Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Bit2Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed,X
    // Execution not implemented
}

void Bit3Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,X
    // Execution not implemented
}

void Bit4Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 2;
    size += !(regs.p & CpuRegisters::StatusFlag::M);
    // uint32_t arg = FETCH_ARG_FROM_ROM(regs.pc + 1, regs.pc + size);
    // Execution not implemented
}

void Bmi0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Program Counter Relative
    // Execution not implemented
}

void Bne0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Program Counter Relative
    // Execution not implemented
}

void Bpl0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Program Counter Relative
    // Execution not implemented
}

void Bra0Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Program Counter Relative
    // Execution not implemented
}

void Brk0Func(CpuRegisters &regs) {
    int numCycles = 7;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Stack/Interrupt
    // Execution not implemented
}

void Brl0Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg =
        0;  // Address mode not implemented: Program Counter Relative Long
    // Execution not implemented
}

void Bvc0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Program Counter Relative
    // Execution not implemented
}

void Bvs0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Program Counter Relative
    // Execution not implemented
}

void Clc0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Cld0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Cli0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Clv0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Cmp0Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed Indirect,X
    // Execution not implemented
}

void Cmp1Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Stack Relative
    // Execution not implemented
}

void Cmp2Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void Cmp3Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect Long
    // Execution not implemented
}

void Cmp4Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 2;
    size += !(regs.p & CpuRegisters::StatusFlag::M);
    // uint32_t arg = FETCH_ARG_FROM_ROM(regs.pc + 1, regs.pc + size);
    // Execution not implemented
}

void Cmp5Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Cmp6Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 4;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Long
    // Execution not implemented
}

void Cmp7Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect Indexed, Y
    // Execution not implemented
}

void Cmp8Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect
    // Execution not implemented
}

void Cmp9Func(CpuRegisters &regs) {
    int numCycles = 7;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: SR Indirect Indexed,Y
    // Execution not implemented
}

void Cmp10Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed,X
    // Execution not implemented
}

void Cmp11Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg =
        0;  // Address mode not implemented: DP Indirect Long Indexed, Y
    // Execution not implemented
}

void Cmp12Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,Y
    // Execution not implemented
}

void Cmp13Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,X
    // Execution not implemented
}

void Cmp14Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 4;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Long Indexed,X
    // Execution not implemented
}

void Cop0Func(CpuRegisters &regs) {
    int numCycles = 7;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Stack/Interrupt
    // Execution not implemented
}

void Cpx0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 2;
    size += !(regs.p & CpuRegisters::StatusFlag::X);
    // uint32_t arg = FETCH_ARG_FROM_ROM(regs.pc + 1, regs.pc + size);
    // Execution not implemented
}

void Cpx1Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void Cpx2Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Cpy0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 2;
    size += !(regs.p & CpuRegisters::StatusFlag::X);
    // uint32_t arg = FETCH_ARG_FROM_ROM(regs.pc + 1, regs.pc + size);
    // Execution not implemented
}

void Cpy1Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void Cpy2Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Dec0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Accumulator
    // Execution not implemented
}

void Dec1Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void Dec2Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Dec3Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed,X
    // Execution not implemented
}

void Dec4Func(CpuRegisters &regs) {
    int numCycles = 7;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,X
    // Execution not implemented
}

void Dex0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Dey0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Eor0Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed Indirect,X
    // Execution not implemented
}

void Eor1Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Stack Relative
    // Execution not implemented
}

void Eor2Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void Eor3Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect Long
    // Execution not implemented
}

void Eor4Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 2;
    size += !(regs.p & CpuRegisters::StatusFlag::M);
    // uint32_t arg = FETCH_ARG_FROM_ROM(regs.pc + 1, regs.pc + size);
    // Execution not implemented
}

void Eor5Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Eor6Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 4;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Long
    // Execution not implemented
}

void Eor7Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect Indexed, Y
    // Execution not implemented
}

void Eor8Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect
    // Execution not implemented
}

void Eor9Func(CpuRegisters &regs) {
    int numCycles = 7;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: SR Indirect Indexed,Y
    // Execution not implemented
}

void Eor10Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed,X
    // Execution not implemented
}

void Eor11Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg =
        0;  // Address mode not implemented: DP Indirect Long Indexed, Y
    // Execution not implemented
}

void Eor12Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,Y
    // Execution not implemented
}

void Eor13Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,X
    // Execution not implemented
}

void Eor14Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 4;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Long Indexed,X
    // Execution not implemented
}

void Inc0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Accumulator
    // Execution not implemented
}

void Inc1Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void Inc2Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Inc3Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed,X
    // Execution not implemented
}

void Inc4Func(CpuRegisters &regs) {
    int numCycles = 7;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,X
    // Execution not implemented
}

void Inx0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Iny0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Jmp0Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Jmp1Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 4;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Long
    // Execution not implemented
}

void Jmp2Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indirect
    // Execution not implemented
}

void Jmp3Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg =
        0;  // Address mode not implemented: Absolute Indexed Indirect
    // Execution not implemented
}

void Jmp4Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indirect Long
    // Execution not implemented
}

void Jsr0Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Jsr1Func(CpuRegisters &regs) {
    int numCycles = 8;  // Implement variable cycle rules later
    int size = 4;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Long
    // Execution not implemented
}

void Jsr2Func(CpuRegisters &regs) {
    int numCycles = 8;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg =
        0;  // Address mode not implemented: Absolute Indexed Indirect
    // Execution not implemented
}

void Lda0Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed Indirect,X
    // Execution not implemented
}

void Lda1Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Stack Relative
    // Execution not implemented
}

void Lda2Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void Lda3Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect Long
    // Execution not implemented
}

void Lda4Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 2;
    size += !(regs.p & CpuRegisters::StatusFlag::M);
    // uint32_t arg = FETCH_ARG_FROM_ROM(regs.pc + 1, regs.pc + size);
    // Execution not implemented
}

void Lda5Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Lda6Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 4;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Long
    // Execution not implemented
}

void Lda7Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect Indexed, Y
    // Execution not implemented
}

void Lda8Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect
    // Execution not implemented
}

void Lda9Func(CpuRegisters &regs) {
    int numCycles = 7;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: SR Indirect Indexed,Y
    // Execution not implemented
}

void Lda10Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed,X
    // Execution not implemented
}

void Lda11Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg =
        0;  // Address mode not implemented: DP Indirect Long Indexed, Y
    // Execution not implemented
}

void Lda12Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,Y
    // Execution not implemented
}

void Lda13Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,X
    // Execution not implemented
}

void Lda14Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 4;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Long Indexed,X
    // Execution not implemented
}

void Ldx0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 2;
    size += !(regs.p & CpuRegisters::StatusFlag::X);
    // uint32_t arg = FETCH_ARG_FROM_ROM(regs.pc + 1, regs.pc + size);
    // Execution not implemented
}

void Ldx1Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void Ldx2Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Ldx3Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed,Y
    // Execution not implemented
}

void Ldx4Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,Y
    // Execution not implemented
}

void Ldy0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 2;
    size += !(regs.p & CpuRegisters::StatusFlag::X);
    // uint32_t arg = FETCH_ARG_FROM_ROM(regs.pc + 1, regs.pc + size);
    // Execution not implemented
}

void Ldy1Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void Ldy2Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Ldy3Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed,X
    // Execution not implemented
}

void Ldy4Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,X
    // Execution not implemented
}

void Lsr0Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void Lsr1Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Accumulator
    // Execution not implemented
}

void Lsr2Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Lsr3Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed,X
    // Execution not implemented
}

void Lsr4Func(CpuRegisters &regs) {
    int numCycles = 7;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,X
    // Execution not implemented
}

void Mvn0Func(CpuRegisters &regs) {
    int numCycles = 1;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Block Move
    // Execution not implemented
}

void Mvp0Func(CpuRegisters &regs) {
    int numCycles = 1;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Block Move
    // Execution not implemented
}

void Nop0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Ora0Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed Indirect,X
    // Execution not implemented
}

void Ora1Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Stack Relative
    // Execution not implemented
}

void Ora2Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void Ora3Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect Long
    // Execution not implemented
}

void Ora4Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 2;
    size += !(regs.p & CpuRegisters::StatusFlag::M);
    // uint32_t arg = FETCH_ARG_FROM_ROM(regs.pc + 1, regs.pc + size);
    // Execution not implemented
}

void Ora5Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Ora6Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 4;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Long
    // Execution not implemented
}

void Ora7Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect Indexed, Y
    // Execution not implemented
}

void Ora8Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect
    // Execution not implemented
}

void Ora9Func(CpuRegisters &regs) {
    int numCycles = 7;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: SR Indirect Indexed,Y
    // Execution not implemented
}

void Ora10Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed,X
    // Execution not implemented
}

void Ora11Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg =
        0;  // Address mode not implemented: DP Indirect Long Indexed, Y
    // Execution not implemented
}

void Ora12Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,Y
    // Execution not implemented
}

void Ora13Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,X
    // Execution not implemented
}

void Ora14Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 4;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Long Indexed,X
    // Execution not implemented
}

void Pea0Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Stack (Absolute)
    // Execution not implemented
}

void Pei0Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Stack (DP Indirect)
    // Execution not implemented
}

void Per0Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Stack (PC Relative Long)
    // Execution not implemented
}

void Pha0Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Stack (Push)
    // Execution not implemented
}

void Phb0Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Stack (Push)
    // Execution not implemented
}

void Phd0Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Stack (Push)
    // Execution not implemented
}

void Phk0Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Stack (Push)
    // Execution not implemented
}

void Php0Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Stack (Push)
    // Execution not implemented
}

void Phx0Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Stack (Push)
    // Execution not implemented
}

void Phy0Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Stack (Push)
    // Execution not implemented
}

void Pla0Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Stack (Pull)
    // Execution not implemented
}

void Plb0Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Stack (Pull)
    // Execution not implemented
}

void Pld0Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Stack (Pull)
    // Execution not implemented
}

void Plp0Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Stack (Pull)
    // Execution not implemented
}

void Plx0Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Stack (Pull)
    // Execution not implemented
}

void Ply0Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Stack (Pull)
    // Execution not implemented
}

void Rep0Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 2;
    // uint32_t arg = FETCH_ARG_FROM_ROM(regs.pc + 1, regs.pc + size);
    // Execution not implemented
}

void Rol0Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void Rol1Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Accumulator
    // Execution not implemented
}

void Rol2Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Rol3Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed,X
    // Execution not implemented
}

void Rol4Func(CpuRegisters &regs) {
    int numCycles = 7;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,X
    // Execution not implemented
}

void Ror0Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void Ror1Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Accumulator
    // Execution not implemented
}

void Ror2Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Ror3Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed,X
    // Execution not implemented
}

void Ror4Func(CpuRegisters &regs) {
    int numCycles = 7;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,X
    // Execution not implemented
}

void Rti0Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Stack (RTI)
    // Execution not implemented
}

void Rtl0Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Stack (RTL)
    // Execution not implemented
}

void Rts0Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Stack (RTS)
    // Execution not implemented
}

void Sbc0Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed Indirect,X
    // Execution not implemented
}

void Sbc1Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Stack Relative
    // Execution not implemented
}

void Sbc2Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void Sbc3Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect Long
    // Execution not implemented
}

void Sbc4Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 2;
    size += !(regs.p & CpuRegisters::StatusFlag::M);
    // uint32_t arg = FETCH_ARG_FROM_ROM(regs.pc + 1, regs.pc + size);
    // Execution not implemented
}

void Sbc5Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Sbc6Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 4;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Long
    // Execution not implemented
}

void Sbc7Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect Indexed, Y
    // Execution not implemented
}

void Sbc8Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect
    // Execution not implemented
}

void Sbc9Func(CpuRegisters &regs) {
    int numCycles = 7;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: SR Indirect Indexed,Y
    // Execution not implemented
}

void Sbc10Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed,X
    // Execution not implemented
}

void Sbc11Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg =
        0;  // Address mode not implemented: DP Indirect Long Indexed, Y
    // Execution not implemented
}

void Sbc12Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,Y
    // Execution not implemented
}

void Sbc13Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,X
    // Execution not implemented
}

void Sbc14Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 4;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Long Indexed,X
    // Execution not implemented
}

void Sec0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Sed0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Sei0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Sep0Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 2;
    // uint32_t arg = FETCH_ARG_FROM_ROM(regs.pc + 1, regs.pc + size);
    // Execution not implemented
}

void Sta0Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed Indirect,X
    // Execution not implemented
}

void Sta1Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Stack Relative
    // Execution not implemented
}

void Sta2Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void Sta3Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect Long
    // Execution not implemented
}

void Sta4Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Sta5Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 4;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Long
    // Execution not implemented
}

void Sta6Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect Indexed, Y
    // Execution not implemented
}

void Sta7Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indirect
    // Execution not implemented
}

void Sta8Func(CpuRegisters &regs) {
    int numCycles = 7;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: SR Indirect Indexed,Y
    // Execution not implemented
}

void Sta9Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed,X
    // Execution not implemented
}

void Sta10Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg =
        0;  // Address mode not implemented: DP Indirect Long Indexed, Y
    // Execution not implemented
}

void Sta11Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,Y
    // Execution not implemented
}

void Sta12Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,X
    // Execution not implemented
}

void Sta13Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 4;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Long Indexed,X
    // Execution not implemented
}

void Stp0Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Stx0Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void Stx1Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Stx2Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed,Y
    // Execution not implemented
}

void Sty0Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void Sty1Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Sty2Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed,X
    // Execution not implemented
}

void Stz0Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void Stz1Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: DP Indexed,X
    // Execution not implemented
}

void Stz2Func(CpuRegisters &regs) {
    int numCycles = 4;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Stz3Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute Indexed,X
    // Execution not implemented
}

void Tax0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Tay0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Tcd0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Tcs0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Tdc0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Trb0Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void Trb1Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Tsb0Func(CpuRegisters &regs) {
    int numCycles = 5;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented: Direct Page
    // Execution not implemented
}

void Tsb1Func(CpuRegisters &regs) {
    int numCycles = 6;  // Implement variable cycle rules later
    int size = 3;
    uint32_t arg = 0;  // Address mode not implemented: Absolute
    // Execution not implemented
}

void Tsc0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Tsx0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Txa0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Txs0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Txy0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Tya0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Tyx0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Wai0Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Wdm0Func(CpuRegisters &regs) {
    int numCycles = 0;  // Implement variable cycle rules later
    int size = 2;
    uint32_t arg = 0;  // Address mode not implemented:
    // Execution not implemented
}

void Xba0Func(CpuRegisters &regs) {
    int numCycles = 3;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

void Xce0Func(CpuRegisters &regs) {
    int numCycles = 2;  // Implement variable cycle rules later
    int size = 1;
    uint32_t arg = 0;  // Address mode not implemented: Implied
    // Execution not implemented
}

}  // namespace snes
