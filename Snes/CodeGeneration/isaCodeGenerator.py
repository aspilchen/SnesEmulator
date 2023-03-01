# There are 256 instructions, many with the same general type (addition, subtraction, etc) and different address modes.
# I want to automate as much of c++ programming as possible from here.
#
# C++ code will use a table to map OpCodes to function objects. This file contains scripts to generate
# the C++ code for each instruction based on their descriptions from an online source. As well as the 
# OpCode enum, and the table to map OpCodes to instructions.
# 
# I may attempt another solution using C++ classes for each general instruction type (add, subtract, etc),
# and a class for each memory address mode. And use a table to choose the appropriate classes. However, I believe
# the current approach to be the more efficient and simpler solution.



from InstructionClass import *
from isaTable import *

def sortByCode(instructions):
    return sorted(instructions, key=lambda i: i.code)

def generateOpEnum(instructions):
    result = ["enum OpCode {"]
    for op in sortByCode(instructions):
        result.append(op.getEnumName() + ",")
    result.append("NUM_OPS")
    result.append("};")
    return result

def generateOpTable(instructions):
    table = ["const std::array<std::function<void(SystemState&)>, NUM_OPS> DISPATCH_TABLE = {"]
    for op in sortByCode(instructions):
        table.append(op.getFunctionName() + ",")
    table.append("};")
    return table

def generateFunctionDeclarations(instructions):
    return [i.getFunctionHeader() + ";" for i in instructions]

def generateFunctions(instructions):
    return [i.getFunctionDefinition() for i in instructions]


def updateIsaCPP(instructions):
    funcs = generateFunctions(instructions)
    cpp = open("../Isa/Isa.cpp", 'w')
    cpp.write("#include \"Isa.hpp\"\n\n")
    cpp.write("namespace snes {\n")
    for i in funcs:
        for j in i:
            cpp.write(j + '\n')
        cpp.write('\n')
    cpp.write("}")

instructions = [Instruction(o) for o in OPS]
enum = generateOpEnum(instructions)
headers = generateFunctionDeclarations(instructions)
table = generateOpTable(instructions)

# print(*enum, sep='\n')
# print(*table, sep='\n')
# print(*headers, sep='\n')

updateIsaCPP(instructions)