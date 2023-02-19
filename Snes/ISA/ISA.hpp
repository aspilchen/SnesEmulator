#ifndef ISA_HPP
#define ISA_HPP

# include<vector>

namespace snes {
typedef enum {
    BRK0,    // Break
    ORA0,    // OR Accumulator with Memory
    COP0,    // Co-Processor
    ORA1,    // OR Accumulator with Memory
    TSB0,    // Test and Set Memory Bits Against Accumulator
    ORA2,    // OR Accumulator with Memory
    ASL0,    // Arithmetic Shift Left
    ORA3,    // OR Accumulator with Memory
    PHP0,    // Push Processor Status Register
    ORA4,    // OR Accumulator with Memory
    ASL1,    // Arithmetic Shift Left
    PHD0,    // Push Direct Page Register
    TSB1,    // Test and Set Memory Bits Against Accumulator
    ORA5,    // OR Accumulator with Memory
    ASL2,    // Arithmetic Shift Left
    ORA6,    // OR Accumulator with Memory
    BPL0,    // Branch if Plus
    ORA7,    // OR Accumulator with Memory
    ORA8,    // OR Accumulator with Memory
    ORA9,    // OR Accumulator with Memory
    TRB0,    // Test and Reset Memory Bits Against Accumulator
    ORA10,    // OR Accumulator with Memory
    ASL3,    // Arithmetic Shift Left
    ORA11,    // OR Accumulator with Memory
    CLC0,    // Clear Carry
    ORA12,    // OR Accumulator with Memory
    INC0,    // Increment
    TCS0,    // Transfer 16-bit Accumulator to Stack Pointer
    TRB1,    // Test and Reset Memory Bits Against Accumulator
    ORA13,    // OR Accumulator with Memory
    ASL4,    // Arithmetic Shift Left
    ORA14,    // OR Accumulator with Memory
    JSR0,    // Jump to Subroutine
    AND0,    // AND Accumulator with Memory
    JSR1,    // Jump to Subroutine
    AND1,    // AND Accumulator with Memory
    BIT0,    // Test Bits
    AND2,    // AND Accumulator with Memory
    ROL0,    // Rotate Memory or Accumulator Left
    AND3,    // AND Accumulator with Memory
    PLP0,    // Pull Processor Status Register
    AND4,    // AND Accumulator with Memory
    ROL1,    // Rotate Memory or Accumulator Left
    PLD0,    // Pull Direct Page Register
    BIT1,    // Test Bits
    AND5,    // AND Accumulator with Memory
    ROL2,    // Rotate Memory or Accumulator Left
    AND6,    // AND Accumulator with Memory
    BMI0,    // Branch if Minus
    AND7,    // AND Accumulator with Memory
    AND8,    // AND Accumulator with Memory
    AND9,    // AND Accumulator with Memory
    BIT2,    // Test Bits
    AND10,    // AND Accumulator with Memory
    ROL3,    // Rotate Memory or Accumulator Left
    AND11,    // AND Accumulator with Memory
    SEC0,    // Set Carry Flag
    AND12,    // AND Accumulator with Memory
    DEC0,    // Decrement
    TSC0,    // Transfer Stack Pointer to 16-bit Accumulator
    BIT3,    // Test Bits
    AND13,    // AND Accumulator with Memory
    ROL4,    // Rotate Memory or Accumulator Left
    AND14,    // AND Accumulator with Memory
    RTI0,    // Return from Interrupt
    EOR0,    // Exclusive-OR Accumulator with Memory
    WDM0,    // Reserved for Future Expansion
    EOR1,    // Exclusive-OR Accumulator with Memory
    MVP0,    // Block Move Positive
    EOR2,    // Exclusive-OR Accumulator with Memory
    LSR0,    // Logical Shift Memory or Accumulator Right
    EOR3,    // Exclusive-OR Accumulator with Memory
    PHA0,    // Push Accumulator
    EOR4,    // Exclusive-OR Accumulator with Memory
    LSR1,    // Logical Shift Memory or Accumulator Right
    PHK0,    // Push Program Bank Register
    JMP0,    // Jump
    EOR5,    // Exclusive-OR Accumulator with Memory
    LSR2,    // Logical Shift Memory or Accumulator Right
    EOR6,    // Exclusive-OR Accumulator with Memory
    BVC0,    // Branch if Overflow Clear
    EOR7,    // Exclusive-OR Accumulator with Memory
    EOR8,    // Exclusive-OR Accumulator with Memory
    EOR9,    // Exclusive-OR Accumulator with Memory
    MVN0,    // Block Move Negative
    EOR10,    // Exclusive-OR Accumulator with Memory
    LSR3,    // Logical Shift Memory or Accumulator Right
    EOR11,    // Exclusive-OR Accumulator with Memory
    CLI0,    // Clear Interrupt Disable Flag
    EOR12,    // Exclusive-OR Accumulator with Memory
    PHY0,    // Push Index Register Y
    TCD0,    // Transfer 16-bit Accumulator to Direct Page Register
    JMP1,    // Jump
    EOR13,    // Exclusive-OR Accumulator with Memory
    LSR4,    // Logical Shift Memory or Accumulator Right
    EOR14,    // Exclusive-OR Accumulator with Memory
    RTS0,    // Return from Subroutine
    ADC0,    // Add With Carry
    PER0,    // Push Effective PC Relative Indirect Address
    ADC1,    // Add With Carry
    STZ0,    // Store Zero to Memory
    ADC2,    // Add With Carry
    ROR0,    // Rotate Memory or Accumulator Right
    ADC3,    // Add With Carry
    PLA0,    // Pull Accumulator
    ADC4,    // Add With Carry
    ROR1,    // Rotate Memory or Accumulator Right
    RTL0,    // Return from Subroutine Long
    JMP2,    // Jump
    ADC5,    // Add With Carry
    ROR2,    // Rotate Memory or Accumulator Right
    ADC6,    // Add With Carry
    BVS0,    // Branch if Overflow Set
    ADC7,    // Add With Carry
    ADC8,    // Add With Carry
    ADC9,    // Add With Carry
    STZ1,    // Store Zero to Memory
    ADC10,    // Add With Carry
    ROR3,    // Rotate Memory or Accumulator Right
    ADC11,    // Add With Carry
    SEI0,    // Set Interrupt Disable Flag
    ADC12,    // Add With Carry
    PLY0,    // Pull Index Register Y
    TDC0,    // Transfer Direct Page Register to 16-bit Accumulator
    JMP3,    // Jump
    ADC13,    // Add With Carry
    ROR4,    // Rotate Memory or Accumulator Right
    ADC14,    // Add With Carry
    BRA0,    // Branch Always
    STA0,    // Store Accumulator to Memory
    BRL0,    // Branch Long Always
    STA1,    // Store Accumulator to Memory
    STY0,    // Store Index Register Y to Memory
    STA2,    // Store Accumulator to Memory
    STX0,    // Store Index Register X to Memory
    STA3,    // Store Accumulator to Memory
    DEY0,    // Decrement Index Register Y
    BIT4,    // Test Bits
    TXA0,    // Transfer Index Register X to Accumulator
    PHB0,    // Push Data Bank Register
    STY1,    // Store Index Register Y to Memory
    STA4,    // Store Accumulator to Memory
    STX1,    // Store Index Register X to Memory
    STA5,    // Store Accumulator to Memory
    BCC0,    // Branch if Carry Clear
    STA6,    // Store Accumulator to Memory
    STA7,    // Store Accumulator to Memory
    STA8,    // Store Accumulator to Memory
    STY2,    // Store Index Register Y to Memory
    STA9,    // Store Accumulator to Memory
    STX2,    // Store Index Register X to Memory
    STA10,    // Store Accumulator to Memory
    TYA0,    // Transfer Index Register Y to Accumulator
    STA11,    // Store Accumulator to Memory
    TXS0,    // Transfer Index Register X to Stack Pointer
    TXY0,    // Transfer Index Register X to Index Register Y
    STZ2,    // Store Zero to Memory
    STA12,    // Store Accumulator to Memory
    STZ3,    // Store Zero to Memory
    STA13,    // Store Accumulator to Memory
    LDY0,    // Load Index Register Y from Memory
    LDA0,    // Load Accumulator from Memory
    LDX0,    // Load Index Register X from Memory
    LDA1,    // Load Accumulator from Memory
    LDY1,    // Load Index Register Y from Memory
    LDA2,    // Load Accumulator from Memory
    LDX1,    // Load Index Register X from Memory
    LDA3,    // Load Accumulator from Memory
    TAY0,    // Transfer Accumulator to Index Register Y
    LDA4,    // Load Accumulator from Memory
    TAX0,    // Transfer Accumulator to Index Register X
    PLB0,    // Pull Data Bank Register
    LDY2,    // Load Index Register Y from Memory
    LDA5,    // Load Accumulator from Memory
    LDX2,    // Load Index Register X from Memory
    LDA6,    // Load Accumulator from Memory
    BCS0,    // Branch if Carry Set
    LDA7,    // Load Accumulator from Memory
    LDA8,    // Load Accumulator from Memory
    LDA9,    // Load Accumulator from Memory
    LDY3,    // Load Index Register Y from Memory
    LDA10,    // Load Accumulator from Memory
    LDX3,    // Load Index Register X from Memory
    LDA11,    // Load Accumulator from Memory
    CLV0,    // Clear Overflow Flag
    LDA12,    // Load Accumulator from Memory
    TSX0,    // Transfer Stack Pointer to Index Register X
    TYX0,    // Transfer Index Register Y to Index Register X
    LDY4,    // Load Index Register Y from Memory
    LDA13,    // Load Accumulator from Memory
    LDX4,    // Load Index Register X from Memory
    LDA14,    // Load Accumulator from Memory
    CPY0,    // Compare Index Register Y with Memory
    CMP0,    // Compare Accumulator with Memory
    REP0,    // Reset Processor Status Bits
    CMP1,    // Compare Accumulator with Memory
    CPY1,    // Compare Index Register Y with Memory
    CMP2,    // Compare Accumulator with Memory
    DEC1,    // Decrement
    CMP3,    // Compare Accumulator with Memory
    INY0,    // Increment Index Register Y
    CMP4,    // Compare Accumulator with Memory
    DEX0,    // Decrement Index Register X
    WAI0,    // Wait for Interrupt
    CPY2,    // Compare Index Register Y with Memory
    CMP5,    // Compare Accumulator with Memory
    DEC2,    // Decrement
    CMP6,    // Compare Accumulator with Memory
    BNE0,    // Branch if Not Equal
    CMP7,    // Compare Accumulator with Memory
    CMP8,    // Compare Accumulator with Memory
    CMP9,    // Compare Accumulator with Memory
    PEI0,    // Push Effective Indirect Address
    CMP10,    // Compare Accumulator with Memory
    DEC3,    // Decrement
    CMP11,    // Compare Accumulator with Memory
    CLD0,    // Clear Decimal Mode Flag
    CMP12,    // Compare Accumulator with Memory
    PHX0,    // Push Index Register X
    STP0,    // Stop Processor
    JMP4,    // Jump
    CMP13,    // Compare Accumulator with Memory
    DEC4,    // Decrement
    CMP14,    // Compare Accumulator with Memory
    CPX0,    // Compare Index Register X with Memory
    SBC0,    // Subtract with Borrow from Accumulator
    SEP0,    // Set Processor Status Bits
    SBC1,    // Subtract with Borrow from Accumulator
    CPX1,    // Compare Index Register X with Memory
    SBC2,    // Subtract with Borrow from Accumulator
    INC1,    // Increment
    SBC3,    // Subtract with Borrow from Accumulator
    INX0,    // Increment Index Register X
    SBC4,    // Subtract with Borrow from Accumulator
    NOP0,    // No Operation
    XBA0,    // Exchange B and A 8-bit Accumulators
    CPX2,    // Compare Index Register X with Memory
    SBC5,    // Subtract with Borrow from Accumulator
    INC2,    // Increment
    SBC6,    // Subtract with Borrow from Accumulator
    BEQ0,    // Branch if Equal
    SBC7,    // Subtract with Borrow from Accumulator
    SBC8,    // Subtract with Borrow from Accumulator
    SBC9,    // Subtract with Borrow from Accumulator
    PEA0,    // Push Effective Absolute Address
    SBC10,    // Subtract with Borrow from Accumulator
    INC3,    // Increment
    SBC11,    // Subtract with Borrow from Accumulator
    SED0,    // Set Decimal Flag
    SBC12,    // Subtract with Borrow from Accumulator
    PLX0,    // Pull Index Register X
    XCE0,    // Exchange Carry and Emulation Flags
    JSR2,    // Jump to Subroutine
    SBC13,    // Subtract with Borrow from Accumulator
    INC4,    // Increment
    SBC14,    // Subtract with Borrow from Accumulator
} OpCode;

const std::vector<int> INSTRUCTION_SIZES = {
    2,2,2,2,2,2,2,2,1,2,1,1,3,3,3,4,2,2,2,2,
    2,2,2,2,1,3,1,1,3,3,3,4,3,2,4,2,2,2,2,2,
    1,2,1,1,3,3,3,4,2,2,2,2,2,2,2,2,1,3,1,1,
    3,3,3,4,1,2,2,2,3,2,2,2,1,2,1,1,3,3,3,4,
    2,2,2,2,3,2,2,2,1,3,1,1,4,3,3,4,1,2,3,2,
    2,2,2,2,1,2,1,1,3,3,3,4,2,2,2,2,2,2,2,2,
    1,3,1,1,3,3,3,4,2,2,3,2,2,2,2,2,1,2,1,1,
    3,3,3,4,2,2,2,2,2,2,2,2,1,3,1,1,3,3,3,4,
    2,2,2,2,2,2,2,2,1,2,1,1,3,3,3,4,2,2,2,2,
    2,2,2,2,1,3,1,1,3,3,3,4,2,2,2,2,2,2,2,2,
    1,2,1,1,3,3,3,4,2,2,2,2,2,2,2,2,1,3,1,1,
    3,3,3,4,2,2,2,2,2,2,2,2,1,2,1,1,3,3,3,4,
    2,2,2,2,3,2,2,2,1,3,1,1,3,3,3,4,
};

}

#endif