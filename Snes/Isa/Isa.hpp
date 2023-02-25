#ifndef ISA_HPP
#define ISA_HPP

#include <array>
#include <cstdint>
#include <functional>

#include "../CpuRegisters/CpuRegisters.hpp"
#include "../Types/Types.hpp"

// Removed from CPU to make testing easier
namespace snes {

const int MAX_INSTRUCTION_SIZE = 4;
const int BYTE_IN_BITS = 8;

inline uint32_t FETCH_ARG_FROM_ROM(ProgramCounter start, ProgramCounter end) {
    uint32_t result = *start;
    start++;
    while (start != end) {
        result = (result << BYTE_IN_BITS) | *start;
        start++;
    }
    return result;
}

enum OpCode { 
    BRK0, ORA0, COP0, ORA1, TSB0, ORA2, ASL0, ORA3, PHP0, ORA4, ASL1,
    PHD0, TSB1, ORA5, ASL2, ORA6, BPL0, ORA7, ORA8, ORA9, TRB0, ORA10, ASL3,
    ORA11, CLC0, ORA12, INC0, TCS0, TRB1, ORA13, ASL4, ORA14, JSR0, AND0, JSR1,
    AND1, BIT0, AND2, ROL0, AND3, PLP0, AND4, ROL1, PLD0, BIT1, AND5, ROL2,
    AND6, BMI0, AND7, AND8, AND9, BIT2, AND10, ROL3, AND11, SEC0, AND12, DEC0,
    TSC0, BIT3, AND13, ROL4, AND14, RTI0, EOR0, WDM0, EOR1, MVP0, EOR2, LSR0,
    EOR3, PHA0, EOR4, LSR1, PHK0, JMP0, EOR5, LSR2, EOR6, BVC0, EOR7, EOR8,
    EOR9, MVN0, EOR10, LSR3, EOR11, CLI0, EOR12, PHY0, TCD0, JMP1, EOR13, LSR4,
    EOR14, RTS0, ADC0, PER0, ADC1, STZ0, ADC2, ROR0, ADC3, PLA0, ADC4, ROR1,
    RTL0, JMP2, ADC5, ROR2, ADC6, BVS0, ADC7, ADC8, ADC9, STZ1, ADC10, ROR3,
    ADC11, SEI0, ADC12, PLY0, TDC0, JMP3, ADC13, ROR4, ADC14, BRA0, STA0, BRL0,
    STA1, STY0, STA2, STX0, STA3, DEY0, BIT4, TXA0, PHB0, STY1, STA4, STX1,
    STA5, BCC0, STA6, STA7, STA8, STY2, STA9, STX2, STA10, TYA0, STA11, TXS0,
    TXY0, STZ2, STA12, STZ3, STA13, LDY0, LDA0, LDX0, LDA1, LDY1, LDA2, LDX1,
    LDA3, TAY0, LDA4, TAX0, PLB0, LDY2, LDA5, LDX2, LDA6, BCS0, LDA7, LDA8,
    LDA9, LDY3, LDA10, LDX3, LDA11, CLV0, LDA12, TSX0, TYX0, LDY4, LDA13, LDX4,
    LDA14, CPY0, CMP0, REP0, CMP1, CPY1, CMP2, DEC1, CMP3, INY0, CMP4, DEX0,
    WAI0, CPY2, CMP5, DEC2, CMP6, BNE0, CMP7, CMP8, CMP9, PEI0, CMP10, DEC3,
    CMP11, CLD0, CMP12, PHX0, STP0, JMP4, CMP13, DEC4, CMP14, CPX0, SBC0, SEP0,
    SBC1, CPX1, SBC2, INC1, SBC3, INX0, SBC4, NOP0, XBA0, CPX2, SBC5, INC2,
    SBC6, BEQ0, SBC7, SBC8, SBC9, PEA0, SBC10, INC3, SBC11, SED0, SBC12, PLX0,
    XCE0, JSR2, SBC13, INC4, SBC14, NUM_OPS
};

void Brk0Func(CpuRegisters &regs); void Ora0Func(CpuRegisters &regs);
void Cop0Func(CpuRegisters &regs); void Ora1Func(CpuRegisters &regs);
void Tsb0Func(CpuRegisters &regs); void Ora2Func(CpuRegisters &regs);
void Asl0Func(CpuRegisters &regs); void Ora3Func(CpuRegisters &regs);
void Php0Func(CpuRegisters &regs); void Ora4Func(CpuRegisters &regs);
void Asl1Func(CpuRegisters &regs); void Phd0Func(CpuRegisters &regs);
void Tsb1Func(CpuRegisters &regs); void Ora5Func(CpuRegisters &regs);
void Asl2Func(CpuRegisters &regs); void Ora6Func(CpuRegisters &regs);
void Bpl0Func(CpuRegisters &regs); void Ora7Func(CpuRegisters &regs);
void Ora8Func(CpuRegisters &regs); void Ora9Func(CpuRegisters &regs);
void Trb0Func(CpuRegisters &regs); void Ora10Func(CpuRegisters &regs);
void Asl3Func(CpuRegisters &regs); void Ora11Func(CpuRegisters &regs);
void Clc0Func(CpuRegisters &regs); void Ora12Func(CpuRegisters &regs);
void Inc0Func(CpuRegisters &regs); void Tcs0Func(CpuRegisters &regs);
void Trb1Func(CpuRegisters &regs); void Ora13Func(CpuRegisters &regs);
void Asl4Func(CpuRegisters &regs); void Ora14Func(CpuRegisters &regs);
void Jsr0Func(CpuRegisters &regs); void And0Func(CpuRegisters &regs);
void Jsr1Func(CpuRegisters &regs); void And1Func(CpuRegisters &regs);
void Bit0Func(CpuRegisters &regs); void And2Func(CpuRegisters &regs);
void Rol0Func(CpuRegisters &regs); void And3Func(CpuRegisters &regs);
void Plp0Func(CpuRegisters &regs); void And4Func(CpuRegisters &regs);
void Rol1Func(CpuRegisters &regs); void Pld0Func(CpuRegisters &regs);
void Bit1Func(CpuRegisters &regs); void And5Func(CpuRegisters &regs);
void Rol2Func(CpuRegisters &regs); void And6Func(CpuRegisters &regs);
void Bmi0Func(CpuRegisters &regs); void And7Func(CpuRegisters &regs);
void And8Func(CpuRegisters &regs); void And9Func(CpuRegisters &regs);
void Bit2Func(CpuRegisters &regs); void And10Func(CpuRegisters &regs);
void Rol3Func(CpuRegisters &regs); void And11Func(CpuRegisters &regs);
void Sec0Func(CpuRegisters &regs); void And12Func(CpuRegisters &regs);
void Dec0Func(CpuRegisters &regs); void Tsc0Func(CpuRegisters &regs);
void Bit3Func(CpuRegisters &regs); void And13Func(CpuRegisters &regs);
void Rol4Func(CpuRegisters &regs); void And14Func(CpuRegisters &regs);
void Rti0Func(CpuRegisters &regs); void Eor0Func(CpuRegisters &regs);
void Wdm0Func(CpuRegisters &regs); void Eor1Func(CpuRegisters &regs);
void Mvp0Func(CpuRegisters &regs); void Eor2Func(CpuRegisters &regs);
void Lsr0Func(CpuRegisters &regs); void Eor3Func(CpuRegisters &regs);
void Pha0Func(CpuRegisters &regs); void Eor4Func(CpuRegisters &regs);
void Lsr1Func(CpuRegisters &regs); void Phk0Func(CpuRegisters &regs);
void Jmp0Func(CpuRegisters &regs); void Eor5Func(CpuRegisters &regs);
void Lsr2Func(CpuRegisters &regs); void Eor6Func(CpuRegisters &regs);
void Bvc0Func(CpuRegisters &regs); void Eor7Func(CpuRegisters &regs);
void Eor8Func(CpuRegisters &regs); void Eor9Func(CpuRegisters &regs);
void Mvn0Func(CpuRegisters &regs); void Eor10Func(CpuRegisters &regs);
void Lsr3Func(CpuRegisters &regs); void Eor11Func(CpuRegisters &regs);
void Cli0Func(CpuRegisters &regs); void Eor12Func(CpuRegisters &regs);
void Phy0Func(CpuRegisters &regs); void Tcd0Func(CpuRegisters &regs);
void Jmp1Func(CpuRegisters &regs); void Eor13Func(CpuRegisters &regs);
void Lsr4Func(CpuRegisters &regs); void Eor14Func(CpuRegisters &regs);
void Rts0Func(CpuRegisters &regs); void Adc0Func(CpuRegisters &regs);
void Per0Func(CpuRegisters &regs); void Adc1Func(CpuRegisters &regs);
void Stz0Func(CpuRegisters &regs); void Adc2Func(CpuRegisters &regs);
void Ror0Func(CpuRegisters &regs); void Adc3Func(CpuRegisters &regs);
void Pla0Func(CpuRegisters &regs); void Adc4Func(CpuRegisters &regs);
void Ror1Func(CpuRegisters &regs); void Rtl0Func(CpuRegisters &regs);
void Jmp2Func(CpuRegisters &regs); void Adc5Func(CpuRegisters &regs);
void Ror2Func(CpuRegisters &regs); void Adc6Func(CpuRegisters &regs);
void Bvs0Func(CpuRegisters &regs); void Adc7Func(CpuRegisters &regs);
void Adc8Func(CpuRegisters &regs); void Adc9Func(CpuRegisters &regs);
void Stz1Func(CpuRegisters &regs); void Adc10Func(CpuRegisters &regs);
void Ror3Func(CpuRegisters &regs); void Adc11Func(CpuRegisters &regs);
void Sei0Func(CpuRegisters &regs); void Adc12Func(CpuRegisters &regs);
void Ply0Func(CpuRegisters &regs); void Tdc0Func(CpuRegisters &regs);
void Jmp3Func(CpuRegisters &regs); void Adc13Func(CpuRegisters &regs);
void Ror4Func(CpuRegisters &regs); void Adc14Func(CpuRegisters &regs);
void Bra0Func(CpuRegisters &regs); void Sta0Func(CpuRegisters &regs);
void Brl0Func(CpuRegisters &regs); void Sta1Func(CpuRegisters &regs);
void Sty0Func(CpuRegisters &regs); void Sta2Func(CpuRegisters &regs);
void Stx0Func(CpuRegisters &regs); void Sta3Func(CpuRegisters &regs);
void Dey0Func(CpuRegisters &regs); void Bit4Func(CpuRegisters &regs);
void Txa0Func(CpuRegisters &regs); void Phb0Func(CpuRegisters &regs);
void Sty1Func(CpuRegisters &regs); void Sta4Func(CpuRegisters &regs);
void Stx1Func(CpuRegisters &regs); void Sta5Func(CpuRegisters &regs);
void Bcc0Func(CpuRegisters &regs); void Sta6Func(CpuRegisters &regs);
void Sta7Func(CpuRegisters &regs); void Sta8Func(CpuRegisters &regs);
void Sty2Func(CpuRegisters &regs); void Sta9Func(CpuRegisters &regs);
void Stx2Func(CpuRegisters &regs); void Sta10Func(CpuRegisters &regs);
void Tya0Func(CpuRegisters &regs); void Sta11Func(CpuRegisters &regs);
void Txs0Func(CpuRegisters &regs); void Txy0Func(CpuRegisters &regs);
void Stz2Func(CpuRegisters &regs); void Sta12Func(CpuRegisters &regs);
void Stz3Func(CpuRegisters &regs); void Sta13Func(CpuRegisters &regs);
void Ldy0Func(CpuRegisters &regs); void Lda0Func(CpuRegisters &regs);
void Ldx0Func(CpuRegisters &regs); void Lda1Func(CpuRegisters &regs);
void Ldy1Func(CpuRegisters &regs); void Lda2Func(CpuRegisters &regs);
void Ldx1Func(CpuRegisters &regs); void Lda3Func(CpuRegisters &regs);
void Tay0Func(CpuRegisters &regs); void Lda4Func(CpuRegisters &regs);
void Tax0Func(CpuRegisters &regs); void Plb0Func(CpuRegisters &regs);
void Ldy2Func(CpuRegisters &regs); void Lda5Func(CpuRegisters &regs);
void Ldx2Func(CpuRegisters &regs); void Lda6Func(CpuRegisters &regs);
void Bcs0Func(CpuRegisters &regs); void Lda7Func(CpuRegisters &regs);
void Lda8Func(CpuRegisters &regs); void Lda9Func(CpuRegisters &regs);
void Ldy3Func(CpuRegisters &regs); void Lda10Func(CpuRegisters &regs);
void Ldx3Func(CpuRegisters &regs); void Lda11Func(CpuRegisters &regs);
void Clv0Func(CpuRegisters &regs); void Lda12Func(CpuRegisters &regs);
void Tsx0Func(CpuRegisters &regs); void Tyx0Func(CpuRegisters &regs);
void Ldy4Func(CpuRegisters &regs); void Lda13Func(CpuRegisters &regs);
void Ldx4Func(CpuRegisters &regs); void Lda14Func(CpuRegisters &regs);
void Cpy0Func(CpuRegisters &regs); void Cmp0Func(CpuRegisters &regs);
void Rep0Func(CpuRegisters &regs); void Cmp1Func(CpuRegisters &regs);
void Cpy1Func(CpuRegisters &regs); void Cmp2Func(CpuRegisters &regs);
void Dec1Func(CpuRegisters &regs); void Cmp3Func(CpuRegisters &regs);
void Iny0Func(CpuRegisters &regs); void Cmp4Func(CpuRegisters &regs);
void Dex0Func(CpuRegisters &regs); void Wai0Func(CpuRegisters &regs);
void Cpy2Func(CpuRegisters &regs); void Cmp5Func(CpuRegisters &regs);
void Dec2Func(CpuRegisters &regs); void Cmp6Func(CpuRegisters &regs);
void Bne0Func(CpuRegisters &regs); void Cmp7Func(CpuRegisters &regs);
void Cmp8Func(CpuRegisters &regs); void Cmp9Func(CpuRegisters &regs);
void Pei0Func(CpuRegisters &regs); void Cmp10Func(CpuRegisters &regs);
void Dec3Func(CpuRegisters &regs); void Cmp11Func(CpuRegisters &regs);
void Cld0Func(CpuRegisters &regs); void Cmp12Func(CpuRegisters &regs);
void Phx0Func(CpuRegisters &regs); void Stp0Func(CpuRegisters &regs);
void Jmp4Func(CpuRegisters &regs); void Cmp13Func(CpuRegisters &regs);
void Dec4Func(CpuRegisters &regs); void Cmp14Func(CpuRegisters &regs);
void Cpx0Func(CpuRegisters &regs); void Sbc0Func(CpuRegisters &regs);
void Sep0Func(CpuRegisters &regs); void Sbc1Func(CpuRegisters &regs);
void Cpx1Func(CpuRegisters &regs); void Sbc2Func(CpuRegisters &regs);
void Inc1Func(CpuRegisters &regs); void Sbc3Func(CpuRegisters &regs);
void Inx0Func(CpuRegisters &regs); void Sbc4Func(CpuRegisters &regs);
void Nop0Func(CpuRegisters &regs); void Xba0Func(CpuRegisters &regs);
void Cpx2Func(CpuRegisters &regs); void Sbc5Func(CpuRegisters &regs);
void Inc2Func(CpuRegisters &regs); void Sbc6Func(CpuRegisters &regs);
void Beq0Func(CpuRegisters &regs); void Sbc7Func(CpuRegisters &regs);
void Sbc8Func(CpuRegisters &regs); void Sbc9Func(CpuRegisters &regs);
void Pea0Func(CpuRegisters &regs); void Sbc10Func(CpuRegisters &regs);
void Inc3Func(CpuRegisters &regs); void Sbc11Func(CpuRegisters &regs);
void Sed0Func(CpuRegisters &regs); void Sbc12Func(CpuRegisters &regs);
void Plx0Func(CpuRegisters &regs); void Xce0Func(CpuRegisters &regs);
void Jsr2Func(CpuRegisters &regs); void Sbc13Func(CpuRegisters &regs);
void Inc4Func(CpuRegisters &regs); void Sbc14Func(CpuRegisters &regs);

const std::array<Instruction, NUM_OPS> INSTRUCTION_TABLE{
    Brk0Func,  Ora0Func,  Cop0Func,  Ora1Func,  Tsb0Func,  Ora2Func,  Asl0Func,
    Ora3Func,  Php0Func,  Ora4Func,  Asl1Func,  Phd0Func,  Tsb1Func,  Ora5Func,
    Asl2Func,  Ora6Func,  Bpl0Func,  Ora7Func,  Ora8Func,  Ora9Func,  Trb0Func,
    Ora10Func, Asl3Func,  Ora11Func, Clc0Func,  Ora12Func, Inc0Func,  Tcs0Func,
    Trb1Func,  Ora13Func, Asl4Func,  Ora14Func, Jsr0Func,  And0Func,  Jsr1Func,
    And1Func,  Bit0Func,  And2Func,  Rol0Func,  And3Func,  Plp0Func,  And4Func,
    Rol1Func,  Pld0Func,  Bit1Func,  And5Func,  Rol2Func,  And6Func,  Bmi0Func,
    And7Func,  And8Func,  And9Func,  Bit2Func,  And10Func, Rol3Func,  And11Func,
    Sec0Func,  And12Func, Dec0Func,  Tsc0Func,  Bit3Func,  And13Func, Rol4Func,
    And14Func, Rti0Func,  Eor0Func,  Wdm0Func,  Eor1Func,  Mvp0Func,  Eor2Func,
    Lsr0Func,  Eor3Func,  Pha0Func,  Eor4Func,  Lsr1Func,  Phk0Func,  Jmp0Func,
    Eor5Func,  Lsr2Func,  Eor6Func,  Bvc0Func,  Eor7Func,  Eor8Func,  Eor9Func,
    Mvn0Func,  Eor10Func, Lsr3Func,  Eor11Func, Cli0Func,  Eor12Func, Phy0Func,
    Tcd0Func,  Jmp1Func,  Eor13Func, Lsr4Func,  Eor14Func, Rts0Func,  Adc0Func,
    Per0Func,  Adc1Func,  Stz0Func,  Adc2Func,  Ror0Func,  Adc3Func,  Pla0Func,
    Adc4Func,  Ror1Func,  Rtl0Func,  Jmp2Func,  Adc5Func,  Ror2Func,  Adc6Func,
    Bvs0Func,  Adc7Func,  Adc8Func,  Adc9Func,  Stz1Func,  Adc10Func, Ror3Func,
    Adc11Func, Sei0Func,  Adc12Func, Ply0Func,  Tdc0Func,  Jmp3Func,  Adc13Func,
    Ror4Func,  Adc14Func, Bra0Func,  Sta0Func,  Brl0Func,  Sta1Func,  Sty0Func,
    Sta2Func,  Stx0Func,  Sta3Func,  Dey0Func,  Bit4Func,  Txa0Func,  Phb0Func,
    Sty1Func,  Sta4Func,  Stx1Func,  Sta5Func,  Bcc0Func,  Sta6Func,  Sta7Func,
    Sta8Func,  Sty2Func,  Sta9Func,  Stx2Func,  Sta10Func, Tya0Func,  Sta11Func,
    Txs0Func,  Txy0Func,  Stz2Func,  Sta12Func, Stz3Func,  Sta13Func, Ldy0Func,
    Lda0Func,  Ldx0Func,  Lda1Func,  Ldy1Func,  Lda2Func,  Ldx1Func,  Lda3Func,
    Tay0Func,  Lda4Func,  Tax0Func,  Plb0Func,  Ldy2Func,  Lda5Func,  Ldx2Func,
    Lda6Func,  Bcs0Func,  Lda7Func,  Lda8Func,  Lda9Func,  Ldy3Func,  Lda10Func,
    Ldx3Func,  Lda11Func, Clv0Func,  Lda12Func, Tsx0Func,  Tyx0Func,  Ldy4Func,
    Lda13Func, Ldx4Func,  Lda14Func, Cpy0Func,  Cmp0Func,  Rep0Func,  Cmp1Func,
    Cpy1Func,  Cmp2Func,  Dec1Func,  Cmp3Func,  Iny0Func,  Cmp4Func,  Dex0Func,
    Wai0Func,  Cpy2Func,  Cmp5Func,  Dec2Func,  Cmp6Func,  Bne0Func,  Cmp7Func,
    Cmp8Func,  Cmp9Func,  Pei0Func,  Cmp10Func, Dec3Func,  Cmp11Func, Cld0Func,
    Cmp12Func, Phx0Func,  Stp0Func,  Jmp4Func,  Cmp13Func, Dec4Func,  Cmp14Func,
    Cpx0Func,  Sbc0Func,  Sep0Func,  Sbc1Func,  Cpx1Func,  Sbc2Func,  Inc1Func,
    Sbc3Func,  Inx0Func,  Sbc4Func,  Nop0Func,  Xba0Func,  Cpx2Func,  Sbc5Func,
    Inc2Func,  Sbc6Func,  Beq0Func,  Sbc7Func,  Sbc8Func,  Sbc9Func,  Pea0Func,
    Sbc10Func, Inc3Func,  Sbc11Func, Sed0Func,  Sbc12Func, Plx0Func,  Xce0Func,
    Jsr2Func,  Sbc13Func, Inc4Func,  Sbc14Func};
}  // namespace snes

#endif