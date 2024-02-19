from op_data import *



def get_address(addr_str):
    result = "let address = addr_modes::"
    match addr_str:
        case "DP Indexed Indirect,X":
            result += "direct_indexed_indirect(state)"
        case "Stack Relative":
            result += "stack_relative(state)"
        case "Direct Page":
            result += "direct(state)"
        case "DP Indirect Long":
            result += "direct_indirect_long(state)"
        case "Immediate":
            result += "immediate16(state)"
        case "Absolute":
            result += "absolute(state)"
        case "Absolute Long":
            result += "absolute_long(state)"
        case "DP Indirect Indexed, Y":
            result += "direct_indirect_indexed(state)"
        case "DP Indirect":
            result += "stack_relative_indirect_indexed(state)"
        case "SR Indirect Indexed,Y":
            result += "direct_indirec_indexed(state)"
        case "DP Indexed,X":
            result += " direct_indexed_x(state)"
        case "DP Indirect Long Indexed, Y":
            result += "direct_indirect_long_indexed(state)"
        case "Absolute Indexed,Y":
            result += "absolute_indexed_y(state)"
        case "Absolute Indexed,X":
            result += "absolute_indexed_x(state)"
        case "Absolute Long Indexed,X":
            result += "absolute_long_indexed(state)"
        case _:
            result += f'0; // TODO: define address mode {addr_str}' 
    return result + ';'


def generate_adc(data: OpData):
    result = [
        f'pub fn {data.f_name}(state: &mut state::State)' + " {",
        '   ' + get_address(data.addr),
        '   adc16(state, address);',
        "}",
    ]
    return result

def generate_and(data: OpData):
    result = [
        f'pub fn {data.f_name}(state: &mut state::State)' + " {",
        '   ' + get_address(data.addr),
        '   and16(state, address);',
        "}",
    ]
    return result


def generate_function(data: OpData):
    if data.name == 'ADC':
        return generate_adc(data)


    # addr = get_address(data.addr)
    result = [
        f'pub fn {data.f_name}(state: &mut state::State)' + " {",
        '   ' + get_address(data.addr),
        "}",
    ]
    return result


data = read_data()

result = []

for d in data:
    result.append(generate_function(d))

with open("instructions.rs", 'w') as f:
    for r in result:
        for l in r:
            f.write(l + '\n')
        f.write('\n')


