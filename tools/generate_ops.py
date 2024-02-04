from op_data import *
from define_function import *
import argparse


parser = argparse.ArgumentParser(
    prog='Code Generator',
    description='Generate c++ code'
)


parser.add_argument('-s', '--switch', action='store_true')
parser.add_argument('-c', '--cc', action='store_true')
# parser.add_argument('-h', '--header', action="store_true")
parser.add_argument('-u', '--update', action='store_true')
parser.add_argument('-v', '--verbose', action='store_true')
parser.add_argument('-d', '--dry', action='store_true')



def generate_cc(data:OpData, dry=False, verbose=False):
    output = generate_batch_cc(data)

    if verbose:
        for o in output:
            for l in o:
                print(l)
            print()

    if dry:
        return
    
    with open('ricoh5a22_ops.cc', 'w') as f:
        f.write('#include \"ricoh5a22.h\"\n')
        f.write('\n')
        f.write('namespace snes {\n\n')
        for o in output:
            for l in o:
                f.write(f'{l}\n')
            f.write('\n')
        f.write('}')




def generate_switch(data:[OpData], dry=False, verbose=False):
    
    result = [
        f'switch (op) ' + '{'
    ]

    for o in data:
        result.append(f'case 0x{o.hex}:')
        result.append(f'    {o.f_name}();')
        result.append('    break;')
    result.append('}')

    if verbose:
        for l in result:
            print(l)

    if dry:
        return
    
    with open('switch.cc', 'w') as f:
        for l in result:
            f.write(f'{l}\n')


def main():
    args = parser.parse_args()

    data = []
    try:
        data = read_data()
    except IOError:
        data = scrape_table()
        save_data(data)
        data = parse_table(data)

    if args.cc:
        generate_cc(data, args.dry, args.verbose)

    if args.switch:
        generate_switch(data, args.dry, args.verbose)

    
if __name__ == '__main__':
    main()