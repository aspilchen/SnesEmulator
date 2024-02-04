from op_data import OpData



def not_implemented(data:OpData):
    result = [
        f'void Ricoh5A22::{data.f_name}(void)' + '{',
        '    // Not implemented',
        '}'
    ]
    return result

def immediate_condition(data:OpData):
    arg = 'arg'
    condition = data.get_condition()
    func = data.get_action()
    result = [
    f'void Ricoh5A22::{data.f_name}(void)' + ' {',
    f'    if ({condition}) ' + '{',
    f'        const Byte {arg} = addr_immediate8();',
    f'        {func}({arg});',
    '    } else {',
    f'        const Word {arg} = addr_immediate16();',
    f'        {func}({arg});',
    '    }',
    '}'
    ]
    return result


def ptr_condition(data:OpData):
    ptr = 'ptr'
    arg = 'arg'

    addr = data.get_addr()
    condition = data.get_condition()
    func = data.get_action()

    arg_src_byte = data.get_arg_source(8)
    arg_src_word = data.get_arg_source(16)

    if arg_src_byte is None:
        arg_src_byte = f'mem_.get_byte({ptr})'

    if arg_src_word is None:
        arg_src_word = f'mem_.get_word({ptr})'


    result = [
    f'void Ricoh5A22::{data.f_name}(void)' + ' {',
    f'    const SnesPtr {ptr} = {addr}();',
    f'    if ({condition}) ' + '{',
    f'        const Byte {arg} = {arg_src_byte};',
    f'        {func}({arg});',
    '    } else {',
    f'        const Word {arg} = {arg_src_word};',
    f'        {func}({arg});',
    '    }',
    '}'
    ]
    return result


def ptr_condition_out(data:OpData):
    ptr = 'ptr'
    arg = 'arg'
    res = 'result'
    
    addr = data.get_addr()
    condition = data.get_condition()
    func = data.get_action()
    output = data.get_output(res, ptr)


    arg_src_byte = data.get_arg_source(8)
    arg_src_word = data.get_arg_source(16)

    if arg_src_byte is None:
        arg_src_byte = f'mem_.get_byte({ptr})'

    if arg_src_word is None:
        arg_src_word = f'mem_.get_word({ptr})'

    result = [
    f'void Ricoh5A22::{data.f_name}(void)' + ' {',
    f'    const SnesPtr {ptr} = {addr}();',
    f'    if ({condition}) ' + '{',
    f'        const Byte {arg} = {arg_src_byte};',
    f'        const Byte {res} = {func}({arg});',
    f'        {output};',
    '    } else {',
    f'        const Word {arg} = {arg_src_word};',
    f'        const Word {res} = {func}({arg});',
    f'        {output};',
    '    }',
    '}'
    ]
    return result


def register_condition(data:OpData):
    arg = 'arg'
    res = 'result'
    
    condition = data.get_condition()
    func = data.get_action()
    output = data.get_output(res)

    arg_src_byte = data.get_arg_source(8)
    arg_src_word = data.get_arg_source(16)

    result = [
    f'void Ricoh5A22::{data.f_name}(void)' + ' {',
    f'    if ({condition}) ' + '{',
    f'        const Byte {arg} = {arg_src_byte};',
    f'        const Byte {res} = {func}({arg});',
    f'        {output};',
    '    } else {',
    f'        const Word {arg} = {arg_src_word};',
    f'        const Word {res} = {func}({arg});',
    f'        {output};',
    '    }',
    '}'
    ]
    return result

def generate_batch_cc(ops: [OpData]):
    result = []
    for o in ops:
        if o.get_action() is None:
            result.append(not_implemented(o))
        elif o.is_immediate():
            result.append(immediate_condition(o))
        elif o.get_output_type() in ['a', 'y', 'x']:
            result.append(register_condition(o))
        elif o.get_output_type() == 'm':
            result.append(ptr_condition_out(o))
        else:
            result.append(ptr_condition(o))
    return result