from op_data import *

# This is just to generate a file with empty definitions.

ops = [
  'CLC',
  'CLD',
  'CLI',
  'CLV',
  'REP',
  'SEC',
  'SED',
  'SEP',
  'SEI',
  'XCE',
]

def define(op:OpData):
    result = [
        f'void Ricoh5A22::{op.f_name}(void) ' + '{',
        f' // TODO: implement {op.f_name}',
        '}'
    ]
    return result