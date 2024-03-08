from op_data import *


def get_address_mode(data: OpData):
    immediate_m = {
        'ADC', 'SBC', 'LDA', 'AND', 'EOR', 'ORA',
        'TSB', 'TRB', 'BIT', 'CMP',
    }

    immediate_x = {
        'CPX', 'CPY', 'LDX', 'LDY', 
    }

    mode_map = {      
        'Absolute Long': 'AbsLong',
        'Absolute': 'Absolute',
        'Absolute Indexed,Y': 'AbsIndexY',
        'Absolute Indexed,X': 'AbsIndexX',
        'Absolute Long Indexed,X': 'AbsLongIndex',
        'Direct Page': 'Direct',
        'DP Indexed Indirect,X': 'DPIndexIndir',
        'DP Indexed,X': 'DPIndexX',
        'DP Indexed,Y': 'DPIndexY',
        'DP Indirect Indexed,Y': 'DPIndirIndex',
        'DP Indirect Indexed, Y': 'DPIndirIndex',
        'DP Indirect Long Indexed,Y': 'DPIndirLongIndex',
        'DP Indirect Long': 'DPIndirLong',
        'DP Indirect': 'DPIndir',
        'Program Counter Relative': 'PCRel',
    }

    addr = '&address_mode::AddressMode::'
    if data.name in immediate_m and data.addr == 'Immediate':
        return addr + 'ImmediateM'

    if data.name in immediate_x and data.addr == 'Immediate':
        return addr + 'ImmediateX'

    if data.addr in mode_map:
        return addr + mode_map[data.addr]

    return None


def generate_case(data: OpData):
    # if data.addr == 'Immediate':
    #     return f'0x{data.hex} => display_immediate(\"{data.name}\", sys)'
    # elif data.addr == 'Direct Page':
    #     return f'0x{data.hex} => display_dp(\"{data.name}\", sys)'
    # return f'0x{data.hex} => print!(\"{data.name}\")'

    no_params = {
        'RTS',
    }

    accum_map = {
        '3A': 'dea', 
        '1A': 'ina',
        '6A': 'rora',
        '2A': 'rola',
        '4A': 'lsra',
        '0A': 'asla',
    }

    if data.name in no_params or data.addr == 'Implied':
        return f'0x{data.hex} => {data.name.lower()}(sys)'

    if data.hex in accum_map:
        return f'0x{data.hex} => {accum_map[data.hex]}(sys)'

    mode = get_address_mode(data)
    if mode is not None:
        return f'0x{data.hex} => {data.name.lower()}(sys, {mode})'
    return None


data = read_data()

cases = []

for d in data:
    c = generate_case(d)
    if c is not None:
        cases.append(c)

with open('gen_match.rs', 'w') as f:
    f.write('match op_code {\n')
    for c in cases:
        f.write(f'  {c},\n')
    f.write('_ => todo!("0x{:02X}", op_code),')
    f.write('}\n')