
from cppNames import *

def callBase(op):
    return "{}(&(state.memMap.mem[addr]), state);".format(op)


# The actions each instruction performs
EXECUTION_RULES = {
    # Arithmatic
    "ADC":callBase("adcBase"),
    "SBC":callBase("sbcBase"),
    # "AND":"// bitwise and",
    # "ORA":"// bitwise or",
    # "TSB":"// Test and set bits",
    # "TRB":"// Test and reset bits",
    # "ASL":"// Arithmetic shift left",
    # "LSR":"// Logical shift right",
    # "ROL":"// Rotate left",
    # "ROR":"// Rotate right",
    # "BIT":"// test bits, setting immediate value or address",
    # "CMP":"// Compare accumulator with memory",
    # "CPX":"// Compare register X with memory",
    # "CPY":"// Compare register Y with memory",
    # "DEA":"// Decrement Accumulator",
    # "DEC":"// Decrement",
    # "DEX":"// Decrement X register",
    # "DEY":"// Decrement Y register",
    # "INA":"// Increment Accumulator",
    # "INC":"// Increment",
    # "INX":"// Increment X register",
    # "INY":"// Increment Y register",
    # "DEX":"",
    # "DEY":"regs.t--;",
    # "INA":"regs.a++;",
    # "INC":"// Increment",
    # "INX":"regs.x++;",
    # "INY":"regs.y++;",
    # "NOP":"// No operation",
    # "XBA":"// Exchange bytes of accumulator",

    # Load and store
    # "LDA":"regs.a = arg; // change to handle size mode later",
    # "LDX":"regs.x = arg; // change to handle size mode later",
    # "LDY":"regs.y = arg; // change to handle size mode later",
    # "STA":"// Store accumulator in memory",
    # "STX":"// Store register X in memory",
    # "STY":"// Store register Y in memory",
    # "STZ":"// Store zero in memory",

    # Transfer instructions
    # "TAX":"// Transfer Accumulator to index register X",
    # "TAY":"// Transfer Accumulator to index register Y",
    # "TCD":"// Transfer 16-bit Accumulator to Direct Page register",
    # "TCS":"// Transfer 16-bit Accumulator to Stack Pointer",
    # "TDC":"// Transfer Direct Page register to 16-bit Accumulator",
    # "TSC":"// Transfer Stack Pointer to 16-bit Accumulator",
    # "TSX":"// Transfer Stack Pointer to index register X",
    # "TXA":"// Transfer index register X to Accumulator",
    # "TXS":"// Transfer index register X to Stack Pointer",
    # "TXY":"// Transfer index register X to index register Y",
    # "TYA":"// Transfer index register Y to Accumulator",
    # "TYX":"// Transfer index register Y to index register X",

    # Branch instructions
    # "BCC":"// Branch if Carry flag is clear (C=0)",
    # "BCS":"// Branch if Carry flag is set (C=1)",
    # "BNE":"// Branch if not equal (Z=0)",
    # "BEQ":"// Branch if equal (Z=1)",
    # "BPL":"// Branch if plus (N=0)",
    # "BMI":"// Branch if minus (N=1)",
    # "BVC":"// Branch if overflow flag is clear (V=0)",
    # "BVS":"// Branch if overflow flag is set (V=1)",
    # "BRA":"// Branch Always (unconditional)",
    # "BRL":"// Branch Always Long (unconditional)",

    # Jump and call instructions
    # "JMP":"// Jump",
    # "JML":"// Jump long",
    # "JSR":"// Jump and save return address",
    # "JSL":"// Jump long and save return address",
    # "RTS":"// Return from subroutine",
    # "RTL":"// Return long from subroutine",

    # Inturrupt instructions
    # "BRK":"// Generate software interrupt",
    # "COP":"// Generate coprocessor interrupt",
    # "RTI":"// Return from interrupt",
    # "STP":"// Stop processor until RESET",
    # "WAI":"// Wait for hardware interrupt ",

    # P-flag instructions
    # "CLC":"regs.r &= ~CpuData.StatusFlag.C;",
    # "CLD":"regs.r &= ~CpuData.StatusFlag.D;",
    # "CLI":"regs.r &= ~CpuData.StatusFlag.I;",
    # "CLV":"regs.r &= ~CpuData.StatusFlag.V;",
    # "REP":"regs.r &= !arg;",
    # "SEC":"regs.r |= CpuData.StatusFlag.C;",
    # "SED":"regs.r |= CpuData.StatusFlag.D;",
    # "SEP":"regs.r |= arg;",
    # "SEI":"regs.r |= CpuData.StatusFlag.I;",
    # "XCE":"// Exchange carry flag with emulation flag ",

    # Stack instructions
    # "PHA":"// Push Accumulator",
    # "PHX":"// Push index register X",
    # "PHY":"// Push index register Y",
    # "PHD":"// Push direct page register",
    # "PHB":"// Push data bank register",
    # "PHK":"// Push Program Bank Register",
    # "PHP":"// Push processor status",
    # "PEA":"// Push effective address",
    # "PEI":"// Push effective indirect address",
    # "PER":"// Push effective relative address",
    # "PLA":"// Pull Accumulator 	Pull instructions",
    # "PLX":"// Pull index register X",
    # "PLY":"// Pull index register Y",
    # "PLP":"// Pull processor status",
    # "PLD":"// Pull direct page register",
    # "PLB":"// Pull data bank register"
}