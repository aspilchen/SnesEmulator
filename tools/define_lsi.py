# Hacky code generator for Load/Store instructions


from op_data import OpData

ops = [
    "LDA",
    "LDX",
    "LDY",
    "STA",
    "STX",
    "STY",
    "STZ"
]


class LsiOp:
    def __init__(self, data:OpData):
        self.data = data

    def is_immediate(self):
        return self.data.addr == 'Immediate'

    def get_condition(self):
        condition_mem = ['LDA', 'STA', 'STZ']
        condition_index = ['LDX', 'STX', 'LDY', 'STY']

        if self.data.name in condition_mem:
            return 'state_.is_status_memory()'
        elif self.data.name in condition_index:
            return 'state_.is_status_index()'
        else:
            return 'true'
    
    def get_addr(self):
        match(self.data.addr):
            case 'Absolute':
                return 'addr_absolute'
            case 'Absolute Long':
                return 'addr_absolute_long'
            case 'Direct Page':
                return 'addr_direct'
            case 'DP Indirect':
                return 'addr_direct_indirect'
            case 'DP Indirect Long':
                return 'addr_direct_indirect_long'
            case 'Absolute Indexed,X':
                return 'addr_absolute_indexed_x'
            case 'Absolute Long Indexed,X':
                return 'addr_absolute_long_indexed'
            case 'Absolute Indexed,Y':
                return 'addr_absolute_indexed_y'
            case 'DP Indexed,X':
                return 'addr_direct_indexed_x'
            case 'DP Indexed Indirect,X':
                return 'addr_direct_indexed_indirect'
            case 'DP Indirect Indexed, Y':
                return 'addr_direct_indirect_indexed'
            case 'DP Indirect Long Indexed, Y':
                return 'addr_direct_indirect_indexed_long'
            case 'Stack Relative':
                return 'addr_stack_relative'
            case 'SR Indirect Indexed,Y':
                return 'addr_stack_relative_indirect_indexed'
            case _:
                return None
            
    def op_type(self):
        load_ops = ['LDA', 'LDY', 'LDX']
        store_ops = ['STA', 'STY', 'STX', 'STZ']

        if self.data.name in load_ops:
            return 'load'
        elif self.data.name in store_ops:
            return 'store'
        return None
    

    def get_arg_source(self, size):
        match(self.data.name):
            case 'STA':
                return f'state_.acc{size}()'
            case 'STX':
                return f'state_.x{size}()'
            case 'STY':
                return f'state_.y{size}()'
            case 'STZ':
                return '0'


    def get_output(self):
        match(self.data.name):
            case 'LDA':
                return 'state_.set_acc'
            case 'LDX':
                return 'state_.set_x'
            case 'LDY':
                return 'state_.set_y'
            case _:
                return 'mem_.write'

def not_implemented(op:OpData):
    result = [
        f'void Ricoh5A22::{op.f_name}(void)' + '{',
        '    // Not implemented',
        '}'
    ]
    return result

def immediate_load(op:LsiOp):
    arg = 'arg'
    condition = op.get_condition()
    output = op.get_output()
    result = [
    f'void Ricoh5A22::{op.data.f_name}(void)' + ' {',
    '    if(dissassemble_) {',
    f'        std::cout << \"{op.data.name} \";',
     '    }',
    f'    if ({condition}) ' + '{',
    f'        const Byte {arg} = addr_immediate8();',
    f'        {output}({arg});',
    f'        state_.test_negative({arg});',
    f'        state_.test_zero({arg});',
    '    } else {',
    f'        const Word {arg} = addr_immediate16();',
    f'        {output}({arg});',
    f'        state_.test_negative({arg});',
    f'        state_.test_zero({arg});',
    '    }',
    '}'
    ]
    return result

def ptr_load(op:LsiOp):
    ptr = 'ptr'
    arg = 'arg'
    addr = op.get_addr()
    condition = op.get_condition()
    output = op.get_output()
    result = [
    f'void Ricoh5A22::{op.data.f_name}(void)' + ' {',
    '    if(dissassemble_) {',
    f'        std::cout << \"{op.data.name} \";',
     '    }',
    f'    const SnesPtr {ptr} = {addr}();',
    f'    if ({condition}) ' + '{',
    f'        const Byte {arg} = mem_.get_byte({ptr});',
    f'        {output}({arg});',
    f'        state_.test_negative({arg});',
    f'        state_.test_zero({arg});',
    '    } else {',
    f'        const Word {arg} = mem_.get_word({ptr});',
    f'        {output}({arg});',
    f'        state_.test_negative({arg});',
    f'        state_.test_zero({arg});',
    '    }',
    '}'
    ]
    return result

def store(op:LsiOp):
    ptr = 'ptr'
    arg = 'arg'
    addr = op.get_addr()
    condition = op.get_condition()
    output = op.get_output()
    result = [
    f'void Ricoh5A22::{op.data.f_name}(void)' + ' {',
    '    if(dissassemble_) {',
    f'        std::cout << \"{op.data.name} \";',
     '    }',
    f'    const SnesPtr {ptr} = {addr}();',
    f'    if ({condition}) ' + '{',
    f'        const Byte {arg} = {op.get_arg_source(8)};',
    f'        {output}({ptr}, {arg});',
    '    } else {',
    f'        const Word {arg} = {op.get_arg_source(16)};',
    f'        {output}({ptr}, {arg});',
    '    }',
    '}'
    ]
    return result


def define(data:OpData):
    if data.name not in ops:
        return []
    o = LsiOp(data)
    if o.is_immediate():
        return immediate_load(o)
    elif o.get_addr() is None:
        return not_implemented(data)
    elif o.op_type() == 'load':
        return ptr_load(o)
    elif o.op_type() == 'store':
        return store(o)
    else:
        return not_implemented(data)