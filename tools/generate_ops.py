# quick hacky code generation for cpu ops.

from op_data import *
import define_alu
import define_lsi
import define_transfer
import define_branch
import define_jmp
import define_inturrupt
import define_pflag
import define_stack
import argparse

parser = argparse.ArgumentParser(
    prog='Code Generator',
    description='Generate c++ code'
)

parser.add_argument('-a', '--alu', action='store_true')
parser.add_argument('-l', '--lsi', action='store_true')
parser.add_argument('-t', '--transfer', action='store_true')
parser.add_argument('-b', '--branch', action='store_true')
parser.add_argument('-j', '--jump', action='store_true')
parser.add_argument('-i', '--inturrupt', action='store_true')
parser.add_argument('-p', '--pflag', action='store_true')
parser.add_argument('-s', '--stack', action='store_true')
parser.add_argument('-v', '--verbose', action='store_true')
parser.add_argument('-d', '--dry', action='store_true')

implemented_ops = [] + define_alu.ops + define_lsi.ops + define_transfer.ops + define_branch.ops + define_jmp.ops + define_inturrupt.ops + define_pflag.ops + define_stack.ops


def generate_cc(data, file_name, op_filter, func, dry, verbose):
    result = []
    ops = filter(lambda x: x.name in op_filter, data)
    result = [func(o) for o in ops]

    if verbose:
        for o in result:
            for l in o:
                print(l)
            print()
    
    if not dry:
        with open(file_name, 'w') as f:
            f.write('#include \"ricoh5a22.h\"\n\n')
            f.write('namespace snes {\n')
            for o in result:
                for l in o:
                    f.write(f'{l}\n')
                f.write('\n')
            f.write('} // namespace snes')



def generate_undefined(data, dry=False, verbose=False):
    ops = filter(lambda x: x.name not in implemented_ops, data)
    result = [define_alu.not_implemented(o) for o in ops]

    # if verbose:
    #     for o in result:
    #         for l in o:
    #             print(l)
    #         print()
    
    if not dry:
        with open('ricoh5a22.undefined.cc', 'w') as f:
            f.write('#include \"ricoh5a22.h\"\n\n')
            f.write('namespace snes {\n')
            for o in result:
                for l in o:
                    f.write(f'{l}\n')
                f.write('\n')
            f.write('} // namespace snes')


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

    if args.alu:
        generate_cc(data, 'ricoh5a22.alu.cc', define_alu.ops, define_alu.define, args.dry, args.verbose)

    if args.lsi:
        generate_cc(data, 'ricoh5a22.lsi.cc', define_lsi.ops, define_lsi.define, args.dry, args.verbose)

    if args.transfer:
        generate_cc(data, 'ricoh5a22.transfer.cc', define_transfer.ops, define_transfer.define, args.dry, args.verbose)

    if args.branch:
        generate_cc(data, 'ricoh5a22.branch.cc', define_branch.ops, define_branch.define, args.dry, args.verbose)

    if args.jump:
        generate_cc(data, 'ricoh5a22.jump.cc', define_jmp.ops, define_jmp.define, args.dry, args.verbose)

    if args.inturrupt:
        generate_cc(data, 'ricoh5a22.inturrupt.cc', define_inturrupt.ops, define_inturrupt.define, args.dry, args.verbose)

    if args.pflag:
        generate_cc(data, 'ricoh5a22.pflag.cc', define_pflag.ops, define_pflag.define, args.dry, args.verbose)
    
    if args.stack:
        generate_cc(data, 'ricoh5a22.stack.cc', define_stack.ops, define_stack.define, args.dry, args.verbose)


    generate_undefined(data, args.dry, args.verbose)
    
if __name__ == '__main__':
    main()