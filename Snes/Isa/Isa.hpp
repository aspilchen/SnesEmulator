#ifndef ISA_HPP
#define ISA_HPP

#include <cstring>
#include <array>
#include <cstdint>
#include <functional>

#include "../SystemState/SystemState.hpp"

// Removed from CPU to make testing easier
namespace snes {
using Instruction = std::function<void(SystemState&)>;

const int MAX_INSTRUCTION_SIZE = 4;
const int BYTE_IN_BITS = 8;


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

// ---------- Addressing Functions ---------- //

inline uint32_t addrImmediate(uint8_t *ptr, int size) {
    uint32_t result = 0;
    std::memcpy(&result, ptr, size);
    return result;
}

inline uint32_t addrDP(uint8_t *ptr, SystemState &state) {
    const int N_BYTES = 1;
    return addrImmediate(ptr, N_BYTES) + state.registers.d;
}

inline uint32_t addrAbsolute(uint8_t *ptr, SystemState &state) {
	const int N_BYTES = 2;
	return (state.registers.dbr << N_BYTES * BYTE_IN_BITS) | addrImmediate(ptr, N_BYTES);
}

inline uint32_t addrAbsoluteLong(uint8_t *ptr, SystemState &state) {
    const int N_BYTES = 3;
	return addrImmediate(ptr, N_BYTES);
}

inline uint32_t addrDPIndirectIndexed(uint8_t *ptr, uint16_t &index, SystemState &state) {
	uint32_t addr = addrDP(ptr, state);
	return addrAbsolute(&(state.memMap.mem[addr]), state) + index;
}

inline uint32_t addrDPIndirectIndexedLong(uint8_t *ptr, uint16_t &index, SystemState &state) {
	uint32_t addr = addrDP(ptr, state);
	return addrAbsoluteLong(&(state.memMap.mem[addr]), state) + index;
}

inline uint32_t addrDPIndexed(uint8_t *ptr, uint16_t &index, SystemState &state) {
	return addrDP(ptr, state) + index;
}

inline uint32_t addrDPIndexedIndirect(uint8_t *ptr, uint16_t &index, SystemState &state) {
	uint32_t addr = addrDPIndexed(ptr, index, state);
	return addrAbsolute(&(state.memMap.mem[addr]), state);
}

inline uint32_t addrAbsoluteIndexed(uint8_t *ptr, uint16_t &index, SystemState &state) {
	return addrAbsolute(ptr, state) + index;
}

inline uint32_t addrAbsoluteLongIndexed(uint8_t *ptr, uint16_t &index, SystemState &state) {
	return addrAbsoluteLong(ptr, state) + index;
}

inline uint32_t addrPCRelative(SystemState &state) {
	const int N_BYTES = 1;
	int8_t offset = static_cast<int8_t>(addrImmediate(&*state.registers.pc, N_BYTES));
	return state.registers.pc + offset - state.memMap.mem.begin();
}

// ---------- Base Instructions ---------- //


void adcBase(uint8_t *ptr, SystemState &state) {
    uint16_t &p = state.registers.p;
    uint16_t &a = state.registers.a;
    const int sizeInBytes =  1 + !(p & Registers::M);
    uint32_t arg = addrImmediate(ptr, sizeInBytes);
    arg += a + ((p & Registers::C) > 0); // add with carry bit
    p &= !Registers::C; // clear carry bit
    p |= sizeInBytes == 1 ? arg > INT8_MAX : arg > INT16_MAX; // set carry bit if carry happens
    a = arg;
    state.registers.pc += sizeInBytes;
}

void sbcBase(uint8_t *ptr, SystemState &state) {
    uint16_t &p = state.registers.p;
    uint16_t &a = state.registers.a;
    int sizeInBytes =  1 + !(p & Registers::M);
    uint32_t arg = addrImmediate(ptr, sizeInBytes);
    arg -= a + ((p & Registers::C) > 0); // add with carry bit
    p &= !Registers::C; // clear carry bit
    p |= sizeInBytes == 1 ? arg > INT8_MAX : arg > INT16_MAX; // set carry bit if carry happens
    a = arg;   
    state.registers.pc += sizeInBytes;
}





void adc0(SystemState &state);  void adc1(SystemState &state);
void adc2(SystemState &state);  void adc3(SystemState &state);
void adc4(SystemState &state);  void adc5(SystemState &state);
void adc6(SystemState &state);  void adc7(SystemState &state);
void adc8(SystemState &state);  void adc9(SystemState &state);
void adc10(SystemState &state); void adc11(SystemState &state);
void adc12(SystemState &state); void adc13(SystemState &state);
void adc14(SystemState &state); void and0(SystemState &state);
void and1(SystemState &state);  void and2(SystemState &state);
void and3(SystemState &state);  void and4(SystemState &state);
void and5(SystemState &state);  void and6(SystemState &state);
void and7(SystemState &state);  void and8(SystemState &state);
void and9(SystemState &state);  void and10(SystemState &state);
void and11(SystemState &state); void and12(SystemState &state);
void and13(SystemState &state); void and14(SystemState &state);
void asl0(SystemState &state);  void asl1(SystemState &state);
void asl2(SystemState &state);  void asl3(SystemState &state);
void asl4(SystemState &state);  void bcc0(SystemState &state);
void bcs0(SystemState &state);  void beq0(SystemState &state);
void bit0(SystemState &state);  void bit1(SystemState &state);
void bit2(SystemState &state);  void bit3(SystemState &state);
void bit4(SystemState &state);  void bmi0(SystemState &state);
void bne0(SystemState &state);  void bpl0(SystemState &state);
void bra0(SystemState &state);  void brk0(SystemState &state);
void brl0(SystemState &state);  void bvc0(SystemState &state);
void bvs0(SystemState &state);  void clc0(SystemState &state);
void cld0(SystemState &state);  void cli0(SystemState &state);
void clv0(SystemState &state);  void cmp0(SystemState &state);
void cmp1(SystemState &state);  void cmp2(SystemState &state);
void cmp3(SystemState &state);  void cmp4(SystemState &state);
void cmp5(SystemState &state);  void cmp6(SystemState &state);
void cmp7(SystemState &state);  void cmp8(SystemState &state);
void cmp9(SystemState &state);  void cmp10(SystemState &state);
void cmp11(SystemState &state); void cmp12(SystemState &state);
void cmp13(SystemState &state); void cmp14(SystemState &state);
void cop0(SystemState &state);  void cpx0(SystemState &state);
void cpx1(SystemState &state);  void cpx2(SystemState &state);
void cpy0(SystemState &state);  void cpy1(SystemState &state);
void cpy2(SystemState &state);  void dec0(SystemState &state);
void dec1(SystemState &state);  void dec2(SystemState &state);
void dec3(SystemState &state);  void dec4(SystemState &state);
void dex0(SystemState &state);  void dey0(SystemState &state);
void eor0(SystemState &state);  void eor1(SystemState &state);
void eor2(SystemState &state);  void eor3(SystemState &state);
void eor4(SystemState &state);  void eor5(SystemState &state);
void eor6(SystemState &state);  void eor7(SystemState &state);
void eor8(SystemState &state);  void eor9(SystemState &state);
void eor10(SystemState &state); void eor11(SystemState &state);
void eor12(SystemState &state); void eor13(SystemState &state);
void eor14(SystemState &state); void inc0(SystemState &state);
void inc1(SystemState &state);  void inc2(SystemState &state);
void inc3(SystemState &state);  void inc4(SystemState &state);
void inx0(SystemState &state);  void iny0(SystemState &state);
void jmp0(SystemState &state);  void jmp1(SystemState &state);
void jmp2(SystemState &state);  void jmp3(SystemState &state);
void jmp4(SystemState &state);  void jsr0(SystemState &state);
void jsr1(SystemState &state);  void jsr2(SystemState &state);
void lda0(SystemState &state);  void lda1(SystemState &state);
void lda2(SystemState &state);  void lda3(SystemState &state);
void lda4(SystemState &state);  void lda5(SystemState &state);
void lda6(SystemState &state);  void lda7(SystemState &state);
void lda8(SystemState &state);  void lda9(SystemState &state);
void lda10(SystemState &state); void lda11(SystemState &state);
void lda12(SystemState &state); void lda13(SystemState &state);
void lda14(SystemState &state); void ldx0(SystemState &state);
void ldx1(SystemState &state);  void ldx2(SystemState &state);
void ldx3(SystemState &state);  void ldx4(SystemState &state);
void ldy0(SystemState &state);  void ldy1(SystemState &state);
void ldy2(SystemState &state);  void ldy3(SystemState &state);
void ldy4(SystemState &state);  void lsr0(SystemState &state);
void lsr1(SystemState &state);  void lsr2(SystemState &state);
void lsr3(SystemState &state);  void lsr4(SystemState &state);
void mvn0(SystemState &state);  void mvp0(SystemState &state);
void nop0(SystemState &state);  void ora0(SystemState &state);
void ora1(SystemState &state);  void ora2(SystemState &state);
void ora3(SystemState &state);  void ora4(SystemState &state);
void ora5(SystemState &state);  void ora6(SystemState &state);
void ora7(SystemState &state);  void ora8(SystemState &state);
void ora9(SystemState &state);  void ora10(SystemState &state);
void ora11(SystemState &state); void ora12(SystemState &state);
void ora13(SystemState &state); void ora14(SystemState &state);
void pea0(SystemState &state);  void pei0(SystemState &state);
void per0(SystemState &state);  void pha0(SystemState &state);
void phb0(SystemState &state);  void phd0(SystemState &state);
void phk0(SystemState &state);  void php0(SystemState &state);
void phx0(SystemState &state);  void phy0(SystemState &state);
void pla0(SystemState &state);  void plb0(SystemState &state);
void pld0(SystemState &state);  void plp0(SystemState &state);
void plx0(SystemState &state);  void ply0(SystemState &state);
void rep0(SystemState &state);  void rol0(SystemState &state);
void rol1(SystemState &state);  void rol2(SystemState &state);
void rol3(SystemState &state);  void rol4(SystemState &state);
void ror0(SystemState &state);  void ror1(SystemState &state);
void ror2(SystemState &state);  void ror3(SystemState &state);
void ror4(SystemState &state);  void rti0(SystemState &state);
void rtl0(SystemState &state);  void rts0(SystemState &state);
void sbc0(SystemState &state);  void sbc1(SystemState &state);
void sbc2(SystemState &state);  void sbc3(SystemState &state);
void sbc4(SystemState &state);  void sbc5(SystemState &state);
void sbc6(SystemState &state);  void sbc7(SystemState &state);
void sbc8(SystemState &state);  void sbc9(SystemState &state);
void sbc10(SystemState &state); void sbc11(SystemState &state);
void sbc12(SystemState &state); void sbc13(SystemState &state);
void sbc14(SystemState &state); void sec0(SystemState &state);
void sed0(SystemState &state);  void sei0(SystemState &state);
void sep0(SystemState &state);  void sta0(SystemState &state);
void sta1(SystemState &state);  void sta2(SystemState &state);
void sta3(SystemState &state);  void sta4(SystemState &state);
void sta5(SystemState &state);  void sta6(SystemState &state);
void sta7(SystemState &state);  void sta8(SystemState &state);
void sta9(SystemState &state);  void sta10(SystemState &state);
void sta11(SystemState &state); void sta12(SystemState &state);
void sta13(SystemState &state); void stp0(SystemState &state);
void stx0(SystemState &state);  void stx1(SystemState &state);
void stx2(SystemState &state);  void sty0(SystemState &state);
void sty1(SystemState &state);  void sty2(SystemState &state);
void stz0(SystemState &state);  void stz1(SystemState &state);
void stz2(SystemState &state);  void stz3(SystemState &state);
void tax0(SystemState &state);  void tay0(SystemState &state);
void tcd0(SystemState &state);  void tcs0(SystemState &state);
void tdc0(SystemState &state);  void trb0(SystemState &state);
void trb1(SystemState &state);  void tsb0(SystemState &state);
void tsb1(SystemState &state);  void tsc0(SystemState &state);
void tsx0(SystemState &state);  void txa0(SystemState &state);
void txs0(SystemState &state);  void txy0(SystemState &state);
void tya0(SystemState &state);  void tyx0(SystemState &state);
void wai0(SystemState &state);  void wdm0(SystemState &state);
void xba0(SystemState &state);  void xce0(SystemState &state);

const std::array<Instruction, NUM_OPS> DISPATCH_TABLE = {
    brk0,  ora0,  cop0,  ora1,  tsb0,  ora2,  asl0,  ora3,  php0,  ora4,  asl1,
    phd0,  tsb1,  ora5,  asl2,  ora6,  bpl0,  ora7,  ora8,  ora9,  trb0,  ora10,
    asl3,  ora11, clc0,  ora12, inc0,  tcs0,  trb1,  ora13, asl4,  ora14, jsr0,
    and0,  jsr1,  and1,  bit0,  and2,  rol0,  and3,  plp0,  and4,  rol1,  pld0,
    bit1,  and5,  rol2,  and6,  bmi0,  and7,  and8,  and9,  bit2,  and10, rol3,
    and11, sec0,  and12, dec0,  tsc0,  bit3,  and13, rol4,  and14, rti0,  eor0,
    wdm0,  eor1,  mvp0,  eor2,  lsr0,  eor3,  pha0,  eor4,  lsr1,  phk0,  jmp0,
    eor5,  lsr2,  eor6,  bvc0,  eor7,  eor8,  eor9,  mvn0,  eor10, lsr3,  eor11,
    cli0,  eor12, phy0,  tcd0,  jmp1,  eor13, lsr4,  eor14, rts0,  adc0,  per0,
    adc1,  stz0,  adc2,  ror0,  adc3,  pla0,  adc4,  ror1,  rtl0,  jmp2,  adc5,
    ror2,  adc6,  bvs0,  adc7,  adc8,  adc9,  stz1,  adc10, ror3,  adc11, sei0,
    adc12, ply0,  tdc0,  jmp3,  adc13, ror4,  adc14, bra0,  sta0,  brl0,  sta1,
    sty0,  sta2,  stx0,  sta3,  dey0,  bit4,  txa0,  phb0,  sty1,  sta4,  stx1,
    sta5,  bcc0,  sta6,  sta7,  sta8,  sty2,  sta9,  stx2,  sta10, tya0,  sta11,
    txs0,  txy0,  stz2,  sta12, stz3,  sta13, ldy0,  lda0,  ldx0,  lda1,  ldy1,
    lda2,  ldx1,  lda3,  tay0,  lda4,  tax0,  plb0,  ldy2,  lda5,  ldx2,  lda6,
    bcs0,  lda7,  lda8,  lda9,  ldy3,  lda10, ldx3,  lda11, clv0,  lda12, tsx0,
    tyx0,  ldy4,  lda13, ldx4,  lda14, cpy0,  cmp0,  rep0,  cmp1,  cpy1,  cmp2,
    dec1,  cmp3,  iny0,  cmp4,  dex0,  wai0,  cpy2,  cmp5,  dec2,  cmp6,  bne0,
    cmp7,  cmp8,  cmp9,  pei0,  cmp10, dec3,  cmp11, cld0,  cmp12, phx0,  stp0,
    jmp4,  cmp13, dec4,  cmp14, cpx0,  sbc0,  sep0,  sbc1,  cpx1,  sbc2,  inc1,
    sbc3,  inx0,  sbc4,  nop0,  xba0,  cpx2,  sbc5,  inc2,  sbc6,  beq0,  sbc7,
    sbc8,  sbc9,  pea0,  sbc10, inc3,  sbc11, sed0,  sbc12, plx0,  xce0,  jsr2,
    sbc13, inc4,  sbc14
};
}

#endif