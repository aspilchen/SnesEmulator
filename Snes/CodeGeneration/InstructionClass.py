
from cppNames import *
from addressingRules import ADDRESS_RULES
from executionRules import EXECUTION_RULES

class Instruction:
    nameCounts = dict()

    def __init__(self, op):
        self.description = str(op[2])
        self.code = str(op[3])
        self.addressMode = str(op[4])
        self.flagsSet = str(op[5])
        self.size = str(op[6])
        self.cycles = str(op[7])
        tmpName = op[0]
        if tmpName not in Instruction.nameCounts:
            Instruction.nameCounts[tmpName] = 0
        else:
            Instruction.nameCounts[tmpName] += 1
        self.name = "{}{}".format(tmpName, Instruction.nameCounts[tmpName])

    def getEnumName(self):
        return self.name

    def getFunctionName(self):
        return self.name.lower()

    def getFunctionHeader(self):
        return "void {}({} &{})".format(self.name.lower(), STATE_TYPE, STATE_NAME)

    def getFunctionBody(self):
        body = []
        if self.addressMode in ADDRESS_RULES:
            body.append(ADDRESS_RULES[self.addressMode])
        else:
            body.append("// Not implemented")
            return body
        if self.name[:3].upper() in EXECUTION_RULES:
            body.append(EXECUTION_RULES[self.name[:3]])
        else:
            body.append("// Instruction not implemented")
        return body

    def getFunctionDefinition(self):
        result = [self.getFunctionHeader() + " {"]
        result += self.getFunctionBody()
        result.append("}")
        return result
