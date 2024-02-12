# Hacky code generator for ALU instructions

from op_data import OpData

ops = [
    'ADC',
    'SBC',
    'AND',
    'EOR',
    'ORA',
    'TSB',
    'TRB',
    'ASL',
    'LSR',
    'ROL',
    'ROR',
    'BIT',
    'CMP',
    'CPX',
    'CPY',
    'DEA',
    'DEC',
    'DEX',
    'DEY',
    'INA',
    'INC',
    'INX',
    'INY',
    'NOP',
    'XBA'
]

class AluOp:
    def __init__(self, op_data):
        self.data = op_data

    def is_immediate(self):
        return self.data.addr == 'Immediate'

    def get_action(self):
        match(self.data.name):
            case 'ADC':
                return 'alu_.add_with_carry'
            case 'SBC':
                return 'alu_.subtract_with_carry'
            case 'AND':
                return 'alu_.logic_and'
            case 'EOR':
                return 'alu_.exclusive_or'
            case 'ORA':
                return 'alu_.logic_or'
            case 'TSB':
                return 'alu_.test_and_set'
            case 'TRB':
                return 'alu_.test_and_reset'
            case 'ASL':
                return 'alu_.arith_shift_left'
            case 'LSR':
                return 'alu_.logical_shift_right'
            case 'ROL':
                return 'alu_.rotate_left'
            case 'ROR':
                return 'alu_.rotate_right'
            case 'BIT':
                return 'alu_.bit'
            case 'CMP':
                return 'alu_.compare_with_acc'
            case 'CPX':
                return 'alu_.compare_with_x'
            case 'CPY':
                return 'alu_.compare_with_y'
            case 'DEC':
                return 'alu_.decrement'
            case 'DEY':
                return 'alu_.decrement'
            case 'DEX':
                return 'alu_.decrement'
            case 'INC':
                return 'alu_.increment'
            case 'INY':
                return 'alu_.increment'
            case 'INX':
                return 'alu_.increment'
            # case 'XBA':
            #     return 'alu_.exchange_acc'
            case _:
                return None

    def get_addr(self):
        match(self.data.addr):
            case 'DP Indexed Indirect,X':
                return 'addr_direct_indexed_indirect'
            case 'Stack Relative':
                return 'addr_stack_relative'
            case 'Direct Page':
                return 'addr_direct'
            case 'DP Indirect Long':
                return 'addr_direct_indirect_long'
            case 'Absolute':
                return 'addr_absolute'
            case 'Absolute Long':
                return 'addr_absolute_long'
            case 'DP Indirect Indexed, Y':
                return 'addr_direct_indirect_indexed'
            case 'DP Indirect':
                return 'addr_direct_indirect'
            case 'SR Indirect Indexed,Y':
                return 'addr_stack_relative_indirect_indexed'
            case 'DP Indexed,X':
                return 'addr_direct_indexed_x'
            case 'DP Indirect Long Indexed, Y':
                return 'addr_direct_indirect_indexed_long'
            case 'Absolute Indexed,Y':
                return 'addr_absolute_indexed_y'
            case 'Absolute Indexed,X':
                return 'addr_absolute_indexed_x'
            case 'Absolute Long Indexed,X':
                return 'addr_absolute_long_indexed'
            case _:
                return None

    def get_arg_source(self, size):
        x_src = ['INX', 'DEX']
        y_src = ['INY', 'DEY']

        if self.data.addr == 'Accumulator':
            return f'state_.acc{size}()'
        elif self.data.name in x_src:
            return f'state_.x{size}()'
        elif self.data.name in y_src:
            return f'state_.y{size}()'
        
        return None

    def get_condition(self):
        m_flag = ['ADC', 'SBC', 'AND', 'EOR', 'ORA', 'TSB', 'TRB', 'ASL', 'LSR', 'ROL', 'ROR', 'BIT', 'CMP', 'DEC', 'INC']
        x_flag = ['CPX', 'CPY', 'DEX', 'DEY', 'INX', 'INY']
        
        if self.data.name in m_flag:
            return 'state_.is_status_memory()'
        elif self.data.name in x_flag:
            return 'state_.is_status_index()'
        return None

    def get_output_type(self):
        a_or_address = ['ASL', 'LSR', 'ROL', 'ROR', 'INC', 'DEC']
        x = ['INX', 'DEX']
        y = ['INY', 'DEY']
        address = ['TSB', 'TRB']

        if self.data.addr == 'Accumulator' and self.data.name in a_or_address:
            return 'a'
        elif self.data.name in y:
            return 'y'
        elif self.data.name in x:
            return 'x'
        elif self.data.name in a_or_address or self.data.name in address:
            return 'm'
        else:
            return None
        
    def get_output(self, arg, ptr=''):
        out_type = self.get_output_type()
        match(out_type):
            case 'a':
                return f'state_.set_acc({arg})'
            case 'y':
                return f'state_.set_y({arg})'
            case 'x':
                return f'state_.set_x({arg})'
            case 'm':
                return f'mem_.write({ptr}, {arg})'
            case _:
                return None



def not_implemented(op:OpData):
    result = [
        f'void Ricoh5A22::{op.f_name}(void)' + '{',
        '    // Not implemented',
        '}'
    ]
    return result

# Immediate address that operates on byte if condition is true else word.
def immediate_condition(op:AluOp):
    arg = 'arg'
    condition = op.get_condition()
    func = op.get_action()
    result = [
    f'void Ricoh5A22::{op.data.f_name}(void)' + ' {',
     '    if(dissassemble_) {',
    f'        std::cout << \"{op.data.name} \";',
     '    }',
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

# Dereference address and operates on byte if condition is true else word.
def ptr_condition(data:AluOp):
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
    f'void Ricoh5A22::{data.data.f_name}(void)' + ' {',
     '    if(dissassemble_) {',
    f'        std::cout << \"{data.data.name} \";',
     '    }',
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

# Dereference address and operate on Byte if condition is true, else Word.
# Writes result back to address.
def ptr_condition_out(data:AluOp):
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
    f'void Ricoh5A22::{data.data.f_name}(void)' + ' {',
    '    if(dissassemble_) {',
    f'        std::cout << \"{data.data.name} \";',
     '    }',
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

# Acts directly on a register value. Operateson Byte if condition is true, else Word.
def register_condition(data:AluOp):
    arg = 'arg'
    res = 'result'
    condition = data.get_condition()
    func = data.get_action()
    output = data.get_output(res)
    arg_src_byte = data.get_arg_source(8)
    arg_src_word = data.get_arg_source(16)

    result = [
    f'void Ricoh5A22::{data.data.f_name}(void)' + ' {',
    '    if(dissassemble_) {',
    f'        std::cout << \"{data.data.name} \";',
     '    }',
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


def define(data):
    if data.name not in ops:
        return []

    o = AluOp(data)

    if o.get_action() is None:
        return not_implemented(data)
    elif o.is_immediate():
        return immediate_condition(o)
    elif o.get_output_type() in ['a', 'y', 'x']:
        return register_condition(o)
    elif o.get_output_type() == 'm':
        return ptr_condition_out(o)
    else:
        return ptr_condition(o)
    