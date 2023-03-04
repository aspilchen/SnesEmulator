#include "Isa.hpp"

namespace snes {



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



void adc0(SystemState &state) {
    uint32_t addr =
        addrDPIndexedIndirect(&*state.registers.pc, state.registers.x, state);
    adcBase(&(state.memMap.mem[addr]), state);
}

void adc1(SystemState &state) {
    // Not implemented
}

void adc2(SystemState &state) {
    // Not implemented
}

void adc3(SystemState &state) {
    // Not implemented
}

void adc4(SystemState &state) {
    uint32_t addr = state.registers.pc - state.memMap.mem.begin();
    adcBase(&(state.memMap.mem[addr]), state);
}

void adc5(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    adcBase(&(state.memMap.mem[addr]), state);
}

void adc6(SystemState &state) {
    uint32_t addr = addrAbsoluteLong(&*state.registers.pc, state);
    adcBase(&(state.memMap.mem[addr]), state);
}

void adc7(SystemState &state) {
    uint32_t addr =
        addrDPIndirectIndexed(&*state.registers.pc, state.registers.y, state);
    adcBase(&(state.memMap.mem[addr]), state);
}

void adc8(SystemState &state) {
    // Not implemented
}

void adc9(SystemState &state) {
    // Not implemented
}

void adc10(SystemState &state) {
    uint32_t addr =
        addrDPIndexed(&*state.registers.pc, state.registers.x, state);
    adcBase(&(state.memMap.mem[addr]), state);
}

void adc11(SystemState &state) {
    uint32_t addr = addrDPIndirectIndexedLong(&*state.registers.pc,
                                              state.registers.y, state);
    adcBase(&(state.memMap.mem[addr]), state);
}

void adc12(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.y, state);
    adcBase(&(state.memMap.mem[addr]), state);
}

void adc13(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.x, state);
    adcBase(&(state.memMap.mem[addr]), state);
}

void adc14(SystemState &state) {
    uint32_t addr =
        addrAbsoluteLongIndexed(&*state.registers.pc, state.registers.x, state);
    adcBase(&(state.memMap.mem[addr]), state);
}

void and0(SystemState &state) {
    uint32_t addr =
        addrDPIndexedIndirect(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void and1(SystemState &state) {
    // Not implemented
}

void and2(SystemState &state) {
    // Not implemented
}

void and3(SystemState &state) {
    // Not implemented
}

void and4(SystemState &state) {
    uint32_t addr = state.registers.pc - state.memMap.mem.begin();
    // Instruction not implemented
}

void and5(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void and6(SystemState &state) {
    uint32_t addr = addrAbsoluteLong(&*state.registers.pc, state);
    // Instruction not implemented
}

void and7(SystemState &state) {
    uint32_t addr =
        addrDPIndirectIndexed(&*state.registers.pc, state.registers.y, state);
    // Instruction not implemented
}

void and8(SystemState &state) {
    // Not implemented
}

void and9(SystemState &state) {
    // Not implemented
}

void and10(SystemState &state) {
    uint32_t addr =
        addrDPIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void and11(SystemState &state) {
    uint32_t addr = addrDPIndirectIndexedLong(&*state.registers.pc,
                                              state.registers.y, state);
    // Instruction not implemented
}

void and12(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.y, state);
    // Instruction not implemented
}

void and13(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void and14(SystemState &state) {
    uint32_t addr =
        addrAbsoluteLongIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void asl0(SystemState &state) {
    // Not implemented
}

void asl1(SystemState &state) {
    // Not implemented
}

void asl2(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void asl3(SystemState &state) {
    uint32_t addr =
        addrDPIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void asl4(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void bcc0(SystemState &state) {
    // Not implemented
}

void bcs0(SystemState &state) {
    // Not implemented
}

void beq0(SystemState &state) {
    // Not implemented
}

void bit0(SystemState &state) {
    // Not implemented
}

void bit1(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void bit2(SystemState &state) {
    uint32_t addr =
        addrDPIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void bit3(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void bit4(SystemState &state) {
    uint32_t addr = state.registers.pc - state.memMap.mem.begin();
    // Instruction not implemented
}

void bmi0(SystemState &state) {
    // Not implemented
}

void bne0(SystemState &state) {
    // Not implemented
}

void bpl0(SystemState &state) {
    // Not implemented
}

void bra0(SystemState &state) {
    // Not implemented
}

void brk0(SystemState &state) {
    // Not implemented
}

void brl0(SystemState &state) {
    // Not implemented
}

void bvc0(SystemState &state) {
    // Not implemented
}

void bvs0(SystemState &state) {
    // Not implemented
}

void clc0(SystemState &state) {
    // Instruction not implemented
}

void cld0(SystemState &state) {
    // Instruction not implemented
}

void cli0(SystemState &state) {
    // Instruction not implemented
}

void clv0(SystemState &state) {
    // Instruction not implemented
}

void cmp0(SystemState &state) {
    uint32_t addr =
        addrDPIndexedIndirect(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void cmp1(SystemState &state) {
    // Not implemented
}

void cmp2(SystemState &state) {
    // Not implemented
}

void cmp3(SystemState &state) {
    // Not implemented
}

void cmp4(SystemState &state) {
    uint32_t addr = state.registers.pc - state.memMap.mem.begin();
    // Instruction not implemented
}

void cmp5(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void cmp6(SystemState &state) {
    uint32_t addr = addrAbsoluteLong(&*state.registers.pc, state);
    // Instruction not implemented
}

void cmp7(SystemState &state) {
    uint32_t addr =
        addrDPIndirectIndexed(&*state.registers.pc, state.registers.y, state);
    // Instruction not implemented
}

void cmp8(SystemState &state) {
    // Not implemented
}

void cmp9(SystemState &state) {
    // Not implemented
}

void cmp10(SystemState &state) {
    uint32_t addr =
        addrDPIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void cmp11(SystemState &state) {
    uint32_t addr = addrDPIndirectIndexedLong(&*state.registers.pc,
                                              state.registers.y, state);
    // Instruction not implemented
}

void cmp12(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.y, state);
    // Instruction not implemented
}

void cmp13(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void cmp14(SystemState &state) {
    uint32_t addr =
        addrAbsoluteLongIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void cop0(SystemState &state) {
    // Not implemented
}

void cpx0(SystemState &state) {
    uint32_t addr = state.registers.pc - state.memMap.mem.begin();
    // Instruction not implemented
}

void cpx1(SystemState &state) {
    // Not implemented
}

void cpx2(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void cpy0(SystemState &state) {
    uint32_t addr = state.registers.pc - state.memMap.mem.begin();
    // Instruction not implemented
}

void cpy1(SystemState &state) {
    // Not implemented
}

void cpy2(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void dec0(SystemState &state) {
    // Not implemented
}

void dec1(SystemState &state) {
    // Not implemented
}

void dec2(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void dec3(SystemState &state) {
    uint32_t addr =
        addrDPIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void dec4(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void dex0(SystemState &state) {
    // Instruction not implemented
}

void dey0(SystemState &state) {
    // Instruction not implemented
}

void eor0(SystemState &state) {
    uint32_t addr =
        addrDPIndexedIndirect(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void eor1(SystemState &state) {
    // Not implemented
}

void eor2(SystemState &state) {
    // Not implemented
}

void eor3(SystemState &state) {
    // Not implemented
}

void eor4(SystemState &state) {
    uint32_t addr = state.registers.pc - state.memMap.mem.begin();
    // Instruction not implemented
}

void eor5(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void eor6(SystemState &state) {
    uint32_t addr = addrAbsoluteLong(&*state.registers.pc, state);
    // Instruction not implemented
}

void eor7(SystemState &state) {
    uint32_t addr =
        addrDPIndirectIndexed(&*state.registers.pc, state.registers.y, state);
    // Instruction not implemented
}

void eor8(SystemState &state) {
    // Not implemented
}

void eor9(SystemState &state) {
    // Not implemented
}

void eor10(SystemState &state) {
    uint32_t addr =
        addrDPIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void eor11(SystemState &state) {
    uint32_t addr = addrDPIndirectIndexedLong(&*state.registers.pc,
                                              state.registers.y, state);
    // Instruction not implemented
}

void eor12(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.y, state);
    // Instruction not implemented
}

void eor13(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void eor14(SystemState &state) {
    uint32_t addr =
        addrAbsoluteLongIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void inc0(SystemState &state) {
    // Not implemented
}

void inc1(SystemState &state) {
    // Not implemented
}

void inc2(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void inc3(SystemState &state) {
    uint32_t addr =
        addrDPIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void inc4(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void inx0(SystemState &state) {
    // Instruction not implemented
}

void iny0(SystemState &state) {
    // Instruction not implemented
}

void jmp0(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void jmp1(SystemState &state) {
    uint32_t addr = addrAbsoluteLong(&*state.registers.pc, state);
    // Instruction not implemented
}

void jmp2(SystemState &state) {
    // Not implemented
}

void jmp3(SystemState &state) {
    // Not implemented
}

void jmp4(SystemState &state) {
    // Not implemented
}

void jsr0(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void jsr1(SystemState &state) {
    uint32_t addr = addrAbsoluteLong(&*state.registers.pc, state);
    // Instruction not implemented
}

void jsr2(SystemState &state) {
    // Not implemented
}

void lda0(SystemState &state) {
    uint32_t addr =
        addrDPIndexedIndirect(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void lda1(SystemState &state) {
    // Not implemented
}

void lda2(SystemState &state) {
    // Not implemented
}

void lda3(SystemState &state) {
    // Not implemented
}

void lda4(SystemState &state) {
    uint32_t addr = state.registers.pc - state.memMap.mem.begin();
    // Instruction not implemented
}

void lda5(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void lda6(SystemState &state) {
    uint32_t addr = addrAbsoluteLong(&*state.registers.pc, state);
    // Instruction not implemented
}

void lda7(SystemState &state) {
    uint32_t addr =
        addrDPIndirectIndexed(&*state.registers.pc, state.registers.y, state);
    // Instruction not implemented
}

void lda8(SystemState &state) {
    // Not implemented
}

void lda9(SystemState &state) {
    // Not implemented
}

void lda10(SystemState &state) {
    uint32_t addr =
        addrDPIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void lda11(SystemState &state) {
    uint32_t addr = addrDPIndirectIndexedLong(&*state.registers.pc,
                                              state.registers.y, state);
    // Instruction not implemented
}

void lda12(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.y, state);
    // Instruction not implemented
}

void lda13(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void lda14(SystemState &state) {
    uint32_t addr =
        addrAbsoluteLongIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void ldx0(SystemState &state) {
    uint32_t addr = state.registers.pc - state.memMap.mem.begin();
    // Instruction not implemented
}

void ldx1(SystemState &state) {
    // Not implemented
}

void ldx2(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void ldx3(SystemState &state) {
    uint32_t addr =
        addrDPIndexed(&*state.registers.pc, state.registers.y, state);
    // Instruction not implemented
}

void ldx4(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.y, state);
    // Instruction not implemented
}

void ldy0(SystemState &state) {
    uint32_t addr = state.registers.pc - state.memMap.mem.begin();
    // Instruction not implemented
}

void ldy1(SystemState &state) {
    // Not implemented
}

void ldy2(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void ldy3(SystemState &state) {
    uint32_t addr =
        addrDPIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void ldy4(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void lsr0(SystemState &state) {
    // Not implemented
}

void lsr1(SystemState &state) {
    // Not implemented
}

void lsr2(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void lsr3(SystemState &state) {
    uint32_t addr =
        addrDPIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void lsr4(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void mvn0(SystemState &state) {
    // Not implemented
}

void mvp0(SystemState &state) {
    // Not implemented
}

void nop0(SystemState &state) {
    // Instruction not implemented
}

void ora0(SystemState &state) {
    uint32_t addr =
        addrDPIndexedIndirect(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void ora1(SystemState &state) {
    // Not implemented
}

void ora2(SystemState &state) {
    // Not implemented
}

void ora3(SystemState &state) {
    // Not implemented
}

void ora4(SystemState &state) {
    uint32_t addr = state.registers.pc - state.memMap.mem.begin();
    // Instruction not implemented
}

void ora5(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void ora6(SystemState &state) {
    uint32_t addr = addrAbsoluteLong(&*state.registers.pc, state);
    // Instruction not implemented
}

void ora7(SystemState &state) {
    uint32_t addr =
        addrDPIndirectIndexed(&*state.registers.pc, state.registers.y, state);
    // Instruction not implemented
}

void ora8(SystemState &state) {
    // Not implemented
}

void ora9(SystemState &state) {
    // Not implemented
}

void ora10(SystemState &state) {
    uint32_t addr =
        addrDPIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void ora11(SystemState &state) {
    uint32_t addr = addrDPIndirectIndexedLong(&*state.registers.pc,
                                              state.registers.y, state);
    // Instruction not implemented
}

void ora12(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.y, state);
    // Instruction not implemented
}

void ora13(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void ora14(SystemState &state) {
    uint32_t addr =
        addrAbsoluteLongIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void pea0(SystemState &state) {
    // Not implemented
}

void pei0(SystemState &state) {
    // Not implemented
}

void per0(SystemState &state) {
    // Not implemented
}

void pha0(SystemState &state) {
    // Not implemented
}

void phb0(SystemState &state) {
    // Not implemented
}

void phd0(SystemState &state) {
    // Not implemented
}

void phk0(SystemState &state) {
    // Not implemented
}

void php0(SystemState &state) {
    // Not implemented
}

void phx0(SystemState &state) {
    // Not implemented
}

void phy0(SystemState &state) {
    // Not implemented
}

void pla0(SystemState &state) {
    // Not implemented
}

void plb0(SystemState &state) {
    // Not implemented
}

void pld0(SystemState &state) {
    // Not implemented
}

void plp0(SystemState &state) {
    // Not implemented
}

void plx0(SystemState &state) {
    // Not implemented
}

void ply0(SystemState &state) {
    // Not implemented
}

void rep0(SystemState &state) {
    uint32_t addr = state.registers.pc - state.memMap.mem.begin();
    // Instruction not implemented
}

void rol0(SystemState &state) {
    // Not implemented
}

void rol1(SystemState &state) {
    // Not implemented
}

void rol2(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void rol3(SystemState &state) {
    uint32_t addr =
        addrDPIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void rol4(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void ror0(SystemState &state) {
    // Not implemented
}

void ror1(SystemState &state) {
    // Not implemented
}

void ror2(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void ror3(SystemState &state) {
    uint32_t addr =
        addrDPIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void ror4(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void rti0(SystemState &state) {
    // Not implemented
}

void rtl0(SystemState &state) {
    // Not implemented
}

void rts0(SystemState &state) {
    // Not implemented
}

void sbc0(SystemState &state) {
    uint32_t addr =
        addrDPIndexedIndirect(&*state.registers.pc, state.registers.x, state);
    sbcBase(&(state.memMap.mem[addr]), state);
}

void sbc1(SystemState &state) {
    // Not implemented
}

void sbc2(SystemState &state) {
    // Not implemented
}

void sbc3(SystemState &state) {
    // Not implemented
}

void sbc4(SystemState &state) {
    uint32_t addr = state.registers.pc - state.memMap.mem.begin();
    sbcBase(&(state.memMap.mem[addr]), state);
}

void sbc5(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    sbcBase(&(state.memMap.mem[addr]), state);
}

void sbc6(SystemState &state) {
    uint32_t addr = addrAbsoluteLong(&*state.registers.pc, state);
    sbcBase(&(state.memMap.mem[addr]), state);
}

void sbc7(SystemState &state) {
    uint32_t addr =
        addrDPIndirectIndexed(&*state.registers.pc, state.registers.y, state);
    sbcBase(&(state.memMap.mem[addr]), state);
}

void sbc8(SystemState &state) {
    // Not implemented
}

void sbc9(SystemState &state) {
    // Not implemented
}

void sbc10(SystemState &state) {
    uint32_t addr =
        addrDPIndexed(&*state.registers.pc, state.registers.x, state);
    sbcBase(&(state.memMap.mem[addr]), state);
}

void sbc11(SystemState &state) {
    uint32_t addr = addrDPIndirectIndexedLong(&*state.registers.pc,
                                              state.registers.y, state);
    sbcBase(&(state.memMap.mem[addr]), state);
}

void sbc12(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.y, state);
    sbcBase(&(state.memMap.mem[addr]), state);
}

void sbc13(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.x, state);
    sbcBase(&(state.memMap.mem[addr]), state);
}

void sbc14(SystemState &state) {
    uint32_t addr =
        addrAbsoluteLongIndexed(&*state.registers.pc, state.registers.x, state);
    sbcBase(&(state.memMap.mem[addr]), state);
}

void sec0(SystemState &state) {
    // Instruction not implemented
}

void sed0(SystemState &state) {
    // Instruction not implemented
}

void sei0(SystemState &state) {
    // Instruction not implemented
}

void sep0(SystemState &state) {
    uint32_t addr = state.registers.pc - state.memMap.mem.begin();
    // Instruction not implemented
}

void sta0(SystemState &state) {
    uint32_t addr =
        addrDPIndexedIndirect(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void sta1(SystemState &state) {
    // Not implemented
}

void sta2(SystemState &state) {
    // Not implemented
}

void sta3(SystemState &state) {
    // Not implemented
}

void sta4(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void sta5(SystemState &state) {
    uint32_t addr = addrAbsoluteLong(&*state.registers.pc, state);
    // Instruction not implemented
}

void sta6(SystemState &state) {
    uint32_t addr =
        addrDPIndirectIndexed(&*state.registers.pc, state.registers.y, state);
    // Instruction not implemented
}

void sta7(SystemState &state) {
    // Not implemented
}

void sta8(SystemState &state) {
    // Not implemented
}

void sta9(SystemState &state) {
    uint32_t addr =
        addrDPIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void sta10(SystemState &state) {
    uint32_t addr = addrDPIndirectIndexedLong(&*state.registers.pc,
                                              state.registers.y, state);
    // Instruction not implemented
}

void sta11(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.y, state);
    // Instruction not implemented
}

void sta12(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void sta13(SystemState &state) {
    uint32_t addr =
        addrAbsoluteLongIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void stp0(SystemState &state) {
    // Instruction not implemented
}

void stx0(SystemState &state) {
    // Not implemented
}

void stx1(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void stx2(SystemState &state) {
    uint32_t addr =
        addrDPIndexed(&*state.registers.pc, state.registers.y, state);
    // Instruction not implemented
}

void sty0(SystemState &state) {
    // Not implemented
}

void sty1(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void sty2(SystemState &state) {
    uint32_t addr =
        addrDPIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void stz0(SystemState &state) {
    // Not implemented
}

void stz1(SystemState &state) {
    uint32_t addr =
        addrDPIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void stz2(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void stz3(SystemState &state) {
    uint32_t addr =
        addrAbsoluteIndexed(&*state.registers.pc, state.registers.x, state);
    // Instruction not implemented
}

void tax0(SystemState &state) {
    // Instruction not implemented
}

void tay0(SystemState &state) {
    // Instruction not implemented
}

void tcd0(SystemState &state) {
    // Instruction not implemented
}

void tcs0(SystemState &state) {
    // Instruction not implemented
}

void tdc0(SystemState &state) {
    // Instruction not implemented
}

void trb0(SystemState &state) {
    // Not implemented
}

void trb1(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void tsb0(SystemState &state) {
    // Not implemented
}

void tsb1(SystemState &state) {
    uint32_t addr = addrAbsolute(&*state.registers.pc, state);
    // Instruction not implemented
}

void tsc0(SystemState &state) {
    // Instruction not implemented
}

void tsx0(SystemState &state) {
    // Instruction not implemented
}

void txa0(SystemState &state) {
    // Instruction not implemented
}

void txs0(SystemState &state) {
    // Instruction not implemented
}

void txy0(SystemState &state) {
    // Instruction not implemented
}

void tya0(SystemState &state) {
    // Instruction not implemented
}

void tyx0(SystemState &state) {
    // Instruction not implemented
}

void wai0(SystemState &state) {
    // Instruction not implemented
}

void wdm0(SystemState &state) {
    // Not implemented
}

void xba0(SystemState &state) {
    // Instruction not implemented
}

void xce0(SystemState &state) {
    // Instruction not implemented
}

}  // namespace snes