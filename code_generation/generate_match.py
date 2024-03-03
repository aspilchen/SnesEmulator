from op_data import *


def generate_case(data: OpData):
    return f'0x{data.hex} => {data.f_name}(sys)'



data = read_data()

cases = []

for d in data:
    c = generate_case(d)
    cases.append(c)

with open('gen_match.rs', 'w') as f:
    f.write('match op_code {\n')
    for c in cases:
        f.write(f'  {c},\n')
    f.write('}\n')