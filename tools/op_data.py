import requests
from bs4 import BeautifulSoup as bs
import json

filename = 'op_data.json'


class OpData:
    def __init__(self, data):
        self.name = data['name']
        self.hex = data['hex']
        self.addr = data['addr']
        self.f_name = 'op' + data['hex'] + '_' + data['name']

    def is_immediate(self):
        return self.addr == 'Immediate'

    def get_action(self):
        match(self.name):
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
        match(self.addr):
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

        if self.addr == 'Accumulator':
            return f'state_.acc{size}()'
        elif self.name in x_src:
            return f'state_.x{size}()'
        elif self.name in y_src:
            return f'state_.y{size}()'
        
        return None

    def get_condition(self):
        m_flag = ['ADC', 'SBC', 'AND', 'EOR', 'ORA', 'TSB', 'TRB', 'ASL', 'LSR', 'ROL', 'ROR', 'BIT', 'CMP', 'DEC', 'INC']
        x_flag = ['CPX', 'CPY', 'DEX', 'DEY', 'INX', 'INY']
        
        if self.name in m_flag:
            return 'state_.is_status_memory()'
        elif self.name in x_flag:
            return 'state_.is_status_index()'
        return None

    def get_output_type(self):
        a_or_address = ['ASL', 'LSR', 'ROL', 'ROR', 'INC', 'DEC']
        x = ['INX', 'DEX']
        y = ['INY', 'DEY']
        address = ['TSB', 'TRB']

        if self.addr == 'Accumulator' and self.name in a_or_address:
            return 'a'
        elif self.name in y:
            return 'y'
        elif self.name in x:
            return 'x'
        elif self.name in a_or_address or self.name in address:
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


def scrape_table():
    url = 'https://wiki.superfamicom.org/65816-reference#instructions-60'
    page = requests.get(url)
    soup = bs(page.content, 'html.parser')
    tables = soup.find_all('table')
    return tables[3]

def parse_table(table):
    rows = table.find_all('tr')
    ops = []

    for r in rows:
        tds = r.find_all('td')
        if not tds:
            continue
        # op = str(tds[0])[4:7]
        op = tds[0].get_text()[:3]
        hx = tds[3].get_text()
        addr = tds[4].get_text()
        ops.append({'name':op, 'hex':hx, 'addr':addr})
    return ops


def save_data(op_data):
    with open(filename, 'w') as f:
        f.write(json.dumps(op_data))

def read_data():
    with open(filename, 'r') as f:
        data = json.load(f)
    result = [OpData(d) for d in data]
    return result