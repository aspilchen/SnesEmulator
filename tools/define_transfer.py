from op_data import *

# Transfter instructions are a lot more specific and probably easier to implement manually.
# This is just to generate a file with empty definitions.

ops = [
  'TAX',
  'TAY',
  'TCD',
  'TCS',
  'TDC',
  'TSC',
  'TSX',
  'TXA',
  'TXS',
  'TXY',
  'TYA',
  'TYX'
]


def define(op:OpData):
    result = [
        f'void Ricoh5A22::{op.f_name}(void) ' + '{',
        f' // TODO: implement {op.f_name}',
        '}'
    ]
    return result