from op_data import *



def flag_condition(flag):
    return f'status_flags::is_set_{flag}(&sys.cpu)'

def set_flag(flag, val):
    return f'status_flags::set_{flag}(&mut sys.cpu, {val});'


def if_else (condition, if_true, if_false):
    result = [
        f'if {condition}' + ' {',
    ] + if_true + ['} else {'] + if_false + ['}']
    return result

def if_elseif_els(cond_a, cond_b, bod_a, bod_b, bod_c):
    result = \
        [f'if {cond_a}' + '\{'] + \
        bod_a + \
        ['}' + f' else if {cond_b} ' + '{'] + \
        bod_b + \
        ['} else {'] +\
        bod_c +\
        ['}']
    return result


def call_helper_8(name, params):
    return f'helpers_8::{name}({params});'

def call_helper_16(name, params):
    return f'helpers_16::{name}({params});'

def register_8(f_call, params):
    return f'registers_8::{f_call}({params});'

def register_16(f_call, params):
    return f'registers_16::{f_call}({params});'


def get_addr():
    return f'let address = address_mode::deref_address(sys, mode);'


def make_func(name, body):
    result = [
        f'pub fn {name}(sys: &mut Snes, mode: &address_mode::AddressMode) ' + '{'
    ]
    result += body
    result += [
        '}'
    ]
    return result

def implied_condition(name, condition):
    if_true = [call_helper_8(name, 'sys')]
    if_false = [call_helper_16(name, 'sys')]
    i_e = if_else(flag_condition(condition), if_true, if_false)
    
    result = [f'pub fn {name}(sys: &mut Snes) ' + '{'] + \
    i_e + ['}']
    
    return result

def cond_2(name, condition):
    if_true = [call_helper_8(name, 'sys, address')]
    if_false = [call_helper_16(name, 'sys, address')]
    i_e = if_else(condition, if_true, if_false)
    
    body = [
        get_addr(),
    ] + i_e

    return make_func(name, body)


def branch(name, cond):
    body = [
        get_addr(),
        f'if {cond} ' + '{',
        'set_pc_address(&mut sys.cpu, address);',
        '}'
    ]
    return make_func(name, body)

def stats_flag(name, flag, cond):
    result = [
        f'pub fn {name}(sys: &mut Snes) ' + '{',
        set_flag(flag, cond),
        '}'
    ]
    return result

def transfer(name, cond, src, dest):
    c = flag_condition(cond)
    result = [
        f'pub fn {name}(sys: &mut Snes) ' + '{',
        f'if {c} ' + '{',
        'let value = ' + register_8(f'get_{src}', '&sys.cpu'),
        register_8(f'set_{dest}', '&mut sys.cpu, value'),
        '} else {',
        'let value = ' + register_16(f'get_{src}', '&sys.cpu'),
        register_16(f'set_{dest}', '&mut sys.cpu, value'),
        '} }'
    ]
    return result

def empty_func(name):
    body = [
        'todo!();'
    ]
    return make_func(name, body)

def generate_functions():
    manual = [
        'bra', 'brl', 'jmp',
       'rts', 'rtl', 'brk', 'cop',
       'rti', 'stp', 'wai',
    ]

    imp_conds = {
        ('ina', 'm'),
        ('dea','m'),
        ('inx', 'x'), 
        ('dex', 'x'),
        ('iny', 'x'), 
        ('dey', 'x'),
        ('asla', 'm'),
        ('lsra', 'm'),
        ('rola', 'm'),
        ('rora', 'm'),
    }

    conditional_m = [
        'adc', 'and', 'bit', 'cmp',
        'inc', 'dec', 'rol', 'ror', 
        'asl', 'lsr',
        'eor', 'lda', 'ora', 'sbc',
        'sta', 'stz', 'trb', 'tsb',
    ]


    conditional_x = [
        'ldx', 'ldy', 'stx', 'sty', 'cpx',
        'cpy',
    ]


    flag_sets = [
        ('clc', 'c', 'false'),
        ('cld', 'd', 'false'),
        ('cli', 'i', 'false'),
        ('clv', 'v', 'false'),
        ('sec', 'c', 'true'),
        ('sed', 'd', 'true'),
        ('sei', 'i', 'true'),
    ]

    transfers = [
        ('tax', 'x', 'a', 'x'),
        ('tay', 'x', 'a', 'y'),
        ('txa', 'm', 'x', 'a'),
        ('tya', 'm', 'y', 'a'),
        ('txy', 'x', 'x', 'y'),
        ('tyx', 'x', 'y', 'x')
    ]

    m = flag_condition('m')
    x = flag_condition('x')
    result = \
        [empty_func(name) for name in manual] +\
        [cond_2(name, m) for name in conditional_m] +\
        [cond_2(name, x) for name in conditional_x] +\
        [implied_condition(p[0], p[1]) for p in imp_conds] +\
        [branch('bcc', '!'+flag_condition('c'))] +\
        [branch('bcs', flag_condition('c'))] +\
        [branch('bne', '!'+flag_condition('z'))] +\
        [branch('beq', ''+flag_condition('z'))] +\
        [branch('bpl', '!'+flag_condition('n'))] +\
        [branch('bmi', ''+flag_condition('n'))] +\
        [branch('bvc', '!'+flag_condition('v'))] +\
        [branch('bvs', ''+flag_condition('v'))] +\
        [stats_flag(p[0], p[1], p[2]) for p in flag_sets] +\
        [transfer(p[0], p[1], p[2], p[3]) for p in transfers]
    
    return result


# data = read_data()

# generated = []
# manual = []

# for d in data:
#     result = generate_functions()
#     if result is not None:
#         generated.append(result)
#     else:
#         manual.append(generate_empty_function(d))

generated = generate_functions()

with open("gen_ops.rs", 'w') as f:
    for r in generated:
        for l in r:
            f.write(l + '\n')
        f.write('\n')


