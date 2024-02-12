from op_data import *

# This is just to generate a file with empty definitions.

ops = [
  'PHA',
  'PHX',
  'PHY',
  'PHD',
  'PHB',
  'PHK',
  'PHP',
  'PEA',
  'PEI',
  'PER',
  'PLA',
  'PLX',
  'PLY',
  'PLP',
  'PLD',
  'PLB'
]

def define(op:OpData):
    result = [
        f'void Ricoh5A22::{op.f_name}(void) ' + '{',
        f' // TODO: implement {op.f_name}',
        '}'
    ]
    return result