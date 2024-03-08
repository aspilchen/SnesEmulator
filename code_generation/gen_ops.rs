pub fn bra(sys: &mut Snes, mode: &address_mode::AddressMode) {
todo!();
}

pub fn brl(sys: &mut Snes, mode: &address_mode::AddressMode) {
todo!();
}

pub fn jmp(sys: &mut Snes, mode: &address_mode::AddressMode) {
todo!();
}

pub fn rts(sys: &mut Snes, mode: &address_mode::AddressMode) {
todo!();
}

pub fn rtl(sys: &mut Snes, mode: &address_mode::AddressMode) {
todo!();
}

pub fn brk(sys: &mut Snes, mode: &address_mode::AddressMode) {
todo!();
}

pub fn cop(sys: &mut Snes, mode: &address_mode::AddressMode) {
todo!();
}

pub fn rti(sys: &mut Snes, mode: &address_mode::AddressMode) {
todo!();
}

pub fn stp(sys: &mut Snes, mode: &address_mode::AddressMode) {
todo!();
}

pub fn wai(sys: &mut Snes, mode: &address_mode::AddressMode) {
todo!();
}

pub fn adc(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_m(&sys.cpu) {
helpers_8::adc(sys, address);
} else {
helpers_16::adc(sys, address);
}
}

pub fn and(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_m(&sys.cpu) {
helpers_8::and(sys, address);
} else {
helpers_16::and(sys, address);
}
}

pub fn bit(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_m(&sys.cpu) {
helpers_8::bit(sys, address);
} else {
helpers_16::bit(sys, address);
}
}

pub fn cmp(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_m(&sys.cpu) {
helpers_8::cmp(sys, address);
} else {
helpers_16::cmp(sys, address);
}
}

pub fn inc(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_m(&sys.cpu) {
helpers_8::inc(sys, address);
} else {
helpers_16::inc(sys, address);
}
}

pub fn dec(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_m(&sys.cpu) {
helpers_8::dec(sys, address);
} else {
helpers_16::dec(sys, address);
}
}

pub fn rol(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_m(&sys.cpu) {
helpers_8::rol(sys, address);
} else {
helpers_16::rol(sys, address);
}
}

pub fn ror(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_m(&sys.cpu) {
helpers_8::ror(sys, address);
} else {
helpers_16::ror(sys, address);
}
}

pub fn asl(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_m(&sys.cpu) {
helpers_8::asl(sys, address);
} else {
helpers_16::asl(sys, address);
}
}

pub fn lsr(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_m(&sys.cpu) {
helpers_8::lsr(sys, address);
} else {
helpers_16::lsr(sys, address);
}
}

pub fn eor(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_m(&sys.cpu) {
helpers_8::eor(sys, address);
} else {
helpers_16::eor(sys, address);
}
}

pub fn lda(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_m(&sys.cpu) {
helpers_8::lda(sys, address);
} else {
helpers_16::lda(sys, address);
}
}

pub fn ora(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_m(&sys.cpu) {
helpers_8::ora(sys, address);
} else {
helpers_16::ora(sys, address);
}
}

pub fn sbc(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_m(&sys.cpu) {
helpers_8::sbc(sys, address);
} else {
helpers_16::sbc(sys, address);
}
}

pub fn sta(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_m(&sys.cpu) {
helpers_8::sta(sys, address);
} else {
helpers_16::sta(sys, address);
}
}

pub fn stz(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_m(&sys.cpu) {
helpers_8::stz(sys, address);
} else {
helpers_16::stz(sys, address);
}
}

pub fn trb(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_m(&sys.cpu) {
helpers_8::trb(sys, address);
} else {
helpers_16::trb(sys, address);
}
}

pub fn tsb(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_m(&sys.cpu) {
helpers_8::tsb(sys, address);
} else {
helpers_16::tsb(sys, address);
}
}

pub fn ldx(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_x(&sys.cpu) {
helpers_8::ldx(sys, address);
} else {
helpers_16::ldx(sys, address);
}
}

pub fn ldy(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_x(&sys.cpu) {
helpers_8::ldy(sys, address);
} else {
helpers_16::ldy(sys, address);
}
}

pub fn stx(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_x(&sys.cpu) {
helpers_8::stx(sys, address);
} else {
helpers_16::stx(sys, address);
}
}

pub fn sty(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_x(&sys.cpu) {
helpers_8::sty(sys, address);
} else {
helpers_16::sty(sys, address);
}
}

pub fn cpx(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_x(&sys.cpu) {
helpers_8::cpx(sys, address);
} else {
helpers_16::cpx(sys, address);
}
}

pub fn cpy(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_x(&sys.cpu) {
helpers_8::cpy(sys, address);
} else {
helpers_16::cpy(sys, address);
}
}

pub fn iny(sys: &mut Snes) {
if status_flags::is_set_x(&sys.cpu) {
helpers_8::iny(sys);
} else {
helpers_16::iny(sys);
}
}

pub fn ina(sys: &mut Snes) {
if status_flags::is_set_m(&sys.cpu) {
helpers_8::ina(sys);
} else {
helpers_16::ina(sys);
}
}

pub fn dex(sys: &mut Snes) {
if status_flags::is_set_x(&sys.cpu) {
helpers_8::dex(sys);
} else {
helpers_16::dex(sys);
}
}

pub fn inx(sys: &mut Snes) {
if status_flags::is_set_x(&sys.cpu) {
helpers_8::inx(sys);
} else {
helpers_16::inx(sys);
}
}

pub fn rola(sys: &mut Snes) {
if status_flags::is_set_m(&sys.cpu) {
helpers_8::rola(sys);
} else {
helpers_16::rola(sys);
}
}

pub fn asla(sys: &mut Snes) {
if status_flags::is_set_m(&sys.cpu) {
helpers_8::asla(sys);
} else {
helpers_16::asla(sys);
}
}

pub fn lsra(sys: &mut Snes) {
if status_flags::is_set_m(&sys.cpu) {
helpers_8::lsra(sys);
} else {
helpers_16::lsra(sys);
}
}

pub fn rora(sys: &mut Snes) {
if status_flags::is_set_m(&sys.cpu) {
helpers_8::rora(sys);
} else {
helpers_16::rora(sys);
}
}

pub fn dea(sys: &mut Snes) {
if status_flags::is_set_m(&sys.cpu) {
helpers_8::dea(sys);
} else {
helpers_16::dea(sys);
}
}

pub fn dey(sys: &mut Snes) {
if status_flags::is_set_x(&sys.cpu) {
helpers_8::dey(sys);
} else {
helpers_16::dey(sys);
}
}

pub fn bcc(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if !status_flags::is_set_c(&sys.cpu) {
set_pc_address(&mut sys.cpu, address);
}
}

pub fn bcs(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_c(&sys.cpu) {
set_pc_address(&mut sys.cpu, address);
}
}

pub fn bne(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if !status_flags::is_set_z(&sys.cpu) {
set_pc_address(&mut sys.cpu, address);
}
}

pub fn beq(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_z(&sys.cpu) {
set_pc_address(&mut sys.cpu, address);
}
}

pub fn bpl(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if !status_flags::is_set_n(&sys.cpu) {
set_pc_address(&mut sys.cpu, address);
}
}

pub fn bmi(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_n(&sys.cpu) {
set_pc_address(&mut sys.cpu, address);
}
}

pub fn bvc(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if !status_flags::is_set_v(&sys.cpu) {
set_pc_address(&mut sys.cpu, address);
}
}

pub fn bvs(sys: &mut Snes, mode: &address_mode::AddressMode) {
let address = address_mode::deref_address(sys, mode);
if status_flags::is_set_v(&sys.cpu) {
set_pc_address(&mut sys.cpu, address);
}
}

pub fn clc(sys: &mut Snes) {
status_flags::set_c(&mut sys.cpu, false);
}

pub fn cld(sys: &mut Snes) {
status_flags::set_d(&mut sys.cpu, false);
}

pub fn cli(sys: &mut Snes) {
status_flags::set_i(&mut sys.cpu, false);
}

pub fn clv(sys: &mut Snes) {
status_flags::set_v(&mut sys.cpu, false);
}

pub fn sec(sys: &mut Snes) {
status_flags::set_c(&mut sys.cpu, true);
}

pub fn sed(sys: &mut Snes) {
status_flags::set_d(&mut sys.cpu, true);
}

pub fn sei(sys: &mut Snes) {
status_flags::set_i(&mut sys.cpu, true);
}

pub fn tax(sys: &mut Snes) {
if status_flags::is_set_x(&sys.cpu) {
let value = registers_8::get_a(&sys.cpu);
registers_8::set_x(&mut sys.cpu, value);
} else {
let value = registers_16::get_a(&sys.cpu);
registers_16::set_x(&mut sys.cpu, value);
} }

pub fn tay(sys: &mut Snes) {
if status_flags::is_set_x(&sys.cpu) {
let value = registers_8::get_a(&sys.cpu);
registers_8::set_y(&mut sys.cpu, value);
} else {
let value = registers_16::get_a(&sys.cpu);
registers_16::set_y(&mut sys.cpu, value);
} }

pub fn txa(sys: &mut Snes) {
if status_flags::is_set_m(&sys.cpu) {
let value = registers_8::get_x(&sys.cpu);
registers_8::set_a(&mut sys.cpu, value);
} else {
let value = registers_16::get_x(&sys.cpu);
registers_16::set_a(&mut sys.cpu, value);
} }

pub fn tya(sys: &mut Snes) {
if status_flags::is_set_m(&sys.cpu) {
let value = registers_8::get_y(&sys.cpu);
registers_8::set_a(&mut sys.cpu, value);
} else {
let value = registers_16::get_y(&sys.cpu);
registers_16::set_a(&mut sys.cpu, value);
} }

pub fn txy(sys: &mut Snes) {
if status_flags::is_set_x(&sys.cpu) {
let value = registers_8::get_x(&sys.cpu);
registers_8::set_y(&mut sys.cpu, value);
} else {
let value = registers_16::get_x(&sys.cpu);
registers_16::set_y(&mut sys.cpu, value);
} }

pub fn tyx(sys: &mut Snes) {
if status_flags::is_set_x(&sys.cpu) {
let value = registers_8::get_y(&sys.cpu);
registers_8::set_x(&mut sys.cpu, value);
} else {
let value = registers_16::get_y(&sys.cpu);
registers_16::set_x(&mut sys.cpu, value);
} }

