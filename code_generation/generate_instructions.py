from op_data import *


def op_mem(name):
    return f'{name}(sys, address)'

def op_implied(name):
    return f'{name}(sys)'

def multiple_output(data: OpData, base_name):
    name = base_name
    match data.addr:
        case 'Accumulator':
            name = f'{base_name}a(sys)'
        case _:
            name = f'{base_name}(sys, address)'
    return name

def flag_condition(flag):
    return f'cpu::status_flags::is_set_{flag}(&sys.cpu)'


def get_debug_statement(data: OpData):
    result = ''
    value = '<todo>'

    match data.addr:
        case "Immediate":
            result = f'{data.name} #&{value}'

        case "Direct Page":
            result = f'{data.name} &{value}'
        case "Absolute":
            result = f'{data.name} &{value}'
        case _:
            result = f'{data.name}' 

    return result
    


def get_address(addr_str):
    name = "let address = address_mode::"
    params = "(sys)"
    result = ''

    match addr_str:
        case "Immediate":
            name += "immediate16"

        case "Direct Page":
            name += "direct"
        # case "DP Indexed,X":
        #     name += " direct_indexed_x"
        # case "DP Indexed,Y":
        #     name += " direct_indexed_y"
        # case "DP Indexed Indirect,X":
        #     name += "direct_indexed_indirect"

        # case "DP Indirect":
        #     name += "direct_indirect"
        # case "DP Indirect Indexed, Y":
        #     name += "direct_indirect_indexed"
        # case "DP Indirect Long":
        #     name += "direct_indirect_long"
        # case "DP Indirect Long Indexed, Y":
        #     name += "direct_indirect_long_indexed(state)"

        case "Absolute":
            name += "absolute"
        # case "Absolute Indexed,Y":
        #     name += "absolute_indexed_y"
        # case "Absolute Indexed,X":
        #     name += "absolute_indexed_x"
        # case "Absolute Long":
        #     name += "absolute_long(state)"
        # case "Absolute Long Indexed,X":
        #     name += "absolute_long_indexed"

        case "Stack Relative":
            name += "stack_relative"
        # case "SR Indirect Indexed,Y":
        #     name += "direct_indirect_indexed(state)"
            
        case "Program Counter Relative":
            name += "pc_relative"
        case _:
            return f'let address = 0; //todo: implement {addr_str}' 
    result = name + params
    return result


def generate_conditional_instruction(data, condition, func):
    debug_str = get_debug_statement(data)
    result = [
        f'pub fn {data.f_name}(sys: &mut Snes)' + " {",
        f'   debug_println!(\"{debug_str}\");',
        f'   {get_address(data.addr)};',
        f'   if {condition} ' + '{',
        f'       helpers_8::{func}(sys, address);',
        '   } else {',
        f'       helpers_16::{func}(sys, address);',
        '   }',
        "}",
    ]
    return result

def generate_implied_conditional(data, condition, func):
    debug_str = get_debug_statement(data)
    result = [
        f'pub fn {data.f_name}(sys: &mut Snes)' + " {",
        f'   debug_println!(\"{debug_str}\");',
        f'   if {condition} ' + '{',
        f'       helpers_8::{func}(sys);',
        '   } else {',
        f'       helpers_16::{func};',
        '   }',
        "}",
    ]
    return result


def generate_simple_instruction(data, func):
    debug_str = get_debug_statement(data)
    result = [
        f'pub fn {data.f_name}(sys: &mut Snes)' + " {",
        f'   debug_println!(\"{debug_str}\");',
        f'  {get_address(data.addr)};',
        f'  {func}(sys, address)',
        "}",
    ]
    return result


def generate_empty_function(data: OpData):
    debug_str = get_debug_statement(data)
    result = [
    f'pub fn {data.f_name}(sys: &mut Snes)' + " {",
    f'   debug_println!(\"{debug_str}\");',
    f'  // todo!();',
    "}",
    ]
    return result

def generate_function(data: OpData):
    basic_conditionals = {
        'ADC', 'SBC', 'AND', 'EOR', 'ORA', 'TSB', 'TRB', 'BIT', 'CMP', 'CPX', 'CPY',
        'DEC', 'INC', 'LDA', 'LDX', 'LDY', 'STA', 'STX', 'STY', 'STZ', 
    }

    conditions = {
        'm': ['ADC', 'SBC', 'AND', 'EOR', 'ORA', 'TSB', 'TRB', 'BIT', 'CMP','DEC', 'INC', 'LDA','STA','STZ'],
        'x': [ 'CPX', 'CPY','LDX', 'LDY','STX', 'STY',]
    }

    result = generate_empty_function(data)

    if data.name in basic_conditionals:
        condition = ''
        if data.name in conditions['m']:
            condition = flag_condition('m')
        else:
            condition = flag_condition('x')
        return generate_conditional_instruction(data, condition, data.name.lower())
    
    return result

    condition = flag_condition('m')
    fun_a = ''
    fun_b = ''

    match data.name:
        case "ADC": 
            fun_a = op_mem('adc')
            fun_b = op_mem('adc')
        case "SBC": 
            fun_a = op_mem("sbc8")
            fun_b = op_mem("sbc16")
        case "AND": 
            fun_a = op_mem("and8")
            fun_b = op_mem("and16")
        case "EOR": 
            fun_a = op_mem("eor8")
            fun_b = op_mem("eor16")
        case "ORA": 
            fun_a = op_mem("ora8")
            fun_b = op_mem("ora16")
        case "TSB": 
            fun_a = op_mem("tsb8")
            fun_b = op_mem("tsb16")
        case "TRB": 
            fun_a = op_mem("trb8")
            fun_b = op_mem("trb16")
        case "ASL": 
            fun_a = multiple_output(data, 'asl', 8)
            fun_b = multiple_output(data, 'asl', 16)
        case "LSR": 
            fun_a = multiple_output(data, 'lsr', 8)
            fun_b = multiple_output(data, 'lsr', 16)
        case "ROL": 
            fun_a = multiple_output(data, 'rol', 8)
            fun_b = multiple_output(data, 'rol', 16)
        case "ROR": 
            fun_a = multiple_output(data, 'ror', 8)
            fun_b = multiple_output(data, 'ror', 16)
        case "BIT": 
            fun_a = op_mem("bit8")
            fun_b = op_mem("bit16")
        case "CMP": 
            fun_a = op_mem("cmp8")
            fun_b = op_mem("cmp16")
        case "CPX":
            condition = flag_condition("x")
            fun_a = op_mem("cpx8")
            fun_b = op_mem("cpx16")
        case "CPY": 
            condition = flag_condition("x")
            fun_a = op_mem("cpy8")
            fun_b = op_mem("cpy16")
        case "DEC": 
            fun_a = multiple_output(data, "dec", 8)
            fun_b = multiple_output(data, "dec", 16)
        case "DEX": 
            condition = flag_condition("x")
            fun_a = op_implied("dex8")
            fun_b = op_implied("dex16")
        case "DEY": 
            condition = flag_condition("x")
            fun_a = op_implied("dey8")
            fun_b = op_implied("dey16")
        case "INA": 
            fun_a = op_implied("ina8")
            fun_b = op_implied("ina16")
        case "INC": 
            fun_a = multiple_output(data, "inc", 8)
            fun_b = multiple_output(data, "inc", 16)
        case "INX": 
            condition = flag_condition("x")
            fun_a = op_implied("inx8")
            fun_b = op_implied("inx16")
        case "INY": 
            condition = flag_condition("x")
            fun_a = op_implied("iny8")
            fun_b = op_implied("iny16")
        case "LDA": 
            fun_a = op_mem("lda8")
            fun_b = op_mem("lda16")
        case "LDX": 
            condition = flag_condition("x")
            fun_a = op_mem("ldx8")
            fun_b = op_mem("ldx16")
        case "LDY": 
            condition = flag_condition("x")
            fun_a = op_mem("ldy8")
            fun_b = op_mem("ldy16")
        case "STA": 
            fun_a = op_mem("sta8")
            fun_b = op_mem("sta16")
        case "STX": 
            condition = flag_condition("x")
            fun_a = op_mem("stx8")
            fun_b = op_mem("stx16")
        case "STY": 
            condition = flag_condition("x")
            fun_a = op_mem("sty8")
            fun_b = op_mem("sty16")
        case "STZ":
            fun_a = op_mem("stz8")
            fun_b = op_mem("stz16")
        # case "TAX":
        #     fun_a = op_implied("tax8")
        #     fun_b = op_implied("tax16")
        # case "TAY":
        #     fun_a = op_implied("tay8")
        #     fun_b = op_implied("tay16")
        # case "TCD":
        #     fun_a = op_implied("tcd8")
        #     fun_b = op_implied("tcd16")
        # case "TCS":
        #     fun_a = op_implied("tcs8")
        #     fun_b = op_implied("tcs16")
        # case "TDC":
        #     fun_a = op_implied("tdc8")
        #     fun_b = op_implied("tdc16")
        # case "TSC":
        #     fun_a = op_implied("tsc8")
        #     fun_b = op_implied("tsc16")
        # case "TSX":
        #     fun_a = op_implied("tsx8")
        #     fun_b = op_implied("tsx16")
        # case "TXA":
        #     fun_a = op_implied("txa8")
        #     fun_b = op_implied("txa16")
        # case "TXS":
        #     fun_a = op_implied("txs8")
        #     fun_b = op_implied("txs16")
        # case "TXY":
        #     fun_a = op_implied("txy8")
        #     fun_b = op_implied("txy16")
        # case "TYA":
        #     fun_a = op_implied("tya8")
        #     fun_b = op_implied("tya16")
        # case "TYX":
        #     fun_a = op_implied("tyx8")
        #     fun_b = op_implied("tyx16")
        case _:
            return None
    
    
    if data.addr == 'Accumulator' or data.addr == 'Implied':
        return generate_implied_conditional(data, condition, fun_a, fun_b)
    else:
        return generate_conditional_instruction(data, condition, fun_a, fun_b)


data = read_data()

generated = []
manual = []

for d in data:
    result = generate_function(d)
    if result is not None:
        generated.append(result)
    else:
        manual.append(generate_empty_function(d))

with open("gen_ops.rs", 'w') as f:
    f.write("// Implement these manually\n\n")
    for r in manual:
        for l in r:
            f.write(l + '\n')
        f.write('\n')
    f.write("\n// These are generated functions, do not change them manually\n\n")
    for r in generated:
        for l in r:
            f.write(l + '\n')
        f.write('\n')


