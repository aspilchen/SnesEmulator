from op_data import *


def op_mem(name):
    return f'{name}(state, &address)'

def op_implied(name):
    return f'{name}(state)'

def multiple_output(data: OpData, base_name, size):
    name = base_name
    match data.addr:
        case 'Accumulator':
            name = f'{base_name}_acc{size}(state)'
        case _:
            name = f'{base_name}_mem{size}(state, &address)'
    return name

def flag_condition(flag):
    return f'state.p.is_set_{flag}()'

def get_address(addr_str):
    result = "let address = addr::"
    match addr_str:
        case "Immediate":
            result += "immediate16(state)"

        case "Direct Page":
            result += "direct(state)"
        case "DP Indexed,X":
            result += " direct_indexed_x(state)"
        case "DP Indexed,Y":
            result += " direct_indexed_y(state)"
        case "DP Indexed Indirect,X":
            result += "direct_indexed_indirect(state)"

        case "DP Indirect":
            result += "direct_indirect(state)"
        case "DP Indirect Indexed, Y":
            result += "direct_indirect_indexed(state)"
        case "DP Indirect Long":
            result += "direct_indirect_long(state)"
        # case "DP Indirect Long Indexed, Y":
        #     result += "direct_indirect_long_indexed(state)"

        case "Absolute":
            result += "absolute(state)"
        case "Absolute Indexed,Y":
            result += "absolute_indexed_y(state)"
        case "Absolute Indexed,X":
            result += "absolute_indexed_x(state)"
        case "Absolute Long":
            result += "absolute_long(state)"
        case "Absolute Long Indexed,X":
            result += "absolute_long_indexed(state)"

        case "Stack Relative":
            result += "stack_relative(state)"
        # case "SR Indirect Indexed,Y":
        #     result += "direct_indirect_indexed(state)"
            
        case "Program Counter Relative":
            result += "pc_relative(state)"
        case _:
            result = f'todo!(\"implement {addr_str}\")' 
    return result


def generate_conditional_instruction(data, condition, func_a, func_b):
    result = [
        f'pub fn {data.f_name}(state: &mut State)' + " {",
        f'   {get_address(data.addr)};',
        f'   if {condition} ' + '{',
        f'       // {func_a};',
        '   } else {',
        f'       {func_b};',
        '   }',
        "}",
    ]
    return result

def generate_implied_conditional(data, condition, func_a, func_b):
    result = [
        f'pub fn {data.f_name}(state: &mut State)' + " {",
        f'   if {condition} ' + '{',
        f'       // {func_a};',
        '   } else {',
        f'       {func_b};',
        '   }',
        "}",
    ]
    return result


def generate_simple_instruction(data, func):
    result = [
        f'pub fn {data.f_name}(state: &mut State)' + " {",
        f'  {get_address(data.addr)};',
        f'  {func}(state, address)',
        "}",
    ]
    return result


def generate_empty_function(data: OpData):
    result = [
    f'pub fn {data.f_name}(state: &mut State)' + " {",
    f'  // TODO: implement {data.f_name}',
    "}",
    ]
    return result

def generate_function(data: OpData):

    condition = flag_condition('m')
    fun_a = ''
    fun_b = ''

    match data.name:
        case "ADC": 
            fun_a = op_mem('adc8')
            fun_b = op_mem('adc16')
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
        case "TAX":
            fun_a = op_implied("tax8")
            fun_b = op_implied("tax16")
        case "TAY":
            fun_a = op_implied("tay8")
            fun_b = op_implied("tay16")
        case "TCD":
            fun_a = op_implied("tcd8")
            fun_b = op_implied("tcd16")
        case "TCS":
            fun_a = op_implied("tcs8")
            fun_b = op_implied("tcs16")
        case "TDC":
            fun_a = op_implied("tdc8")
            fun_b = op_implied("tdc16")
        case "TSC":
            fun_a = op_implied("tsc8")
            fun_b = op_implied("tsc16")
        case "TSX":
            fun_a = op_implied("tsx8")
            fun_b = op_implied("tsx16")
        case "TXA":
            fun_a = op_implied("txa8")
            fun_b = op_implied("txa16")
        case "TXS":
            fun_a = op_implied("txs8")
            fun_b = op_implied("txs16")
        case "TXY":
            fun_a = op_implied("txy8")
            fun_b = op_implied("txy16")
        case "TYA":
            fun_a = op_implied("tya8")
            fun_b = op_implied("tya16")
        case "TYX":
            fun_a = op_implied("tyx8")
            fun_b = op_implied("tyx16")
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


