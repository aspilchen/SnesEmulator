
from cppNames import *



CREATE_PTR = PTR_TYPE + PTR_NAME
PC_ADDR = "&*state.registers.pc"


def callImmediate():
    return "uint32_t addr = state.registers.pc - state.memMap.mem.begin();"


def callBasicIndirect(cpp):
    return "uint32_t addr = {}({}, state);".format(cpp, PC_ADDR)


def callIndex(cpp, idx):
    return "uint32_t addr = {}({}, {}, state);".format(cpp, PC_ADDR, idx)



ADDRESS_RULES = {
    "Implied":"",
    "Immediate":callImmediate(),
    "Direct page":callBasicIndirect("addrDP"),
    "Absolute":callBasicIndirect("addrAbsolute"),
    "Absolute Long":callBasicIndirect("addrAbsoluteLong"),

    "DP Indirect Indexed,X":callIndex("addrDPIndirectIndexed", "state.registers.x"),
    "DP Indirect Indexed,Y":callIndex("addrDPIndirectIndexed", "state.registers.y"),
    "DP Indirect Indexed, X":callIndex("addrDPIndirectIndexed", "state.registers.x"),
    "DP Indirect Indexed, Y":callIndex("addrDPIndirectIndexed", "state.registers.y"),

    "DP Indirect Indexed,X":callIndex("addrDPIndirectIndexed", "state.registers.x"),
    "DP Indirect Indexed,Y":callIndex("addrDPIndirectIndexed", "state.registers.y"),
    "DP Indirect Indexed, X":callIndex("addrDPIndirectIndexed", "state.registers.x"),
    "DP Indirect Indexed, Y":callIndex("addrDPIndirectIndexed", "state.registers.y"),

    "DP Indirect Long Indexed,X":callIndex("addrDPIndirectIndexedLong", "state.registers.x"),
    "DP Indirect Long Indexed,Y":callIndex("addrDPIndirectIndexedLong", "state.registers.y"),
    "DP Indirect Long Indexed, X":callIndex("addrDPIndirectIndexedLong", "state.registers.x"),
    "DP Indirect Long Indexed, Y":callIndex("addrDPIndirectIndexedLong", "state.registers.y"),

    "DP Indexed,X":callIndex("addrDPIndexed", "state.registers.x"),
    "DP Indexed,Y":callIndex("addrDPIndexed", "state.registers.y"),
    "DP Indexed, X":callIndex("addrDPIndexed", "state.registers.x"),
    "DP Indexed, Y":callIndex("addrDPIndexed", "state.registers.y"),

    "DP Indexed Indirect,X":callIndex("addrDPIndexedIndirect", "state.registers.x"),
    "DP Indexed Indirect,Y":callIndex("addrDPIndexedIndirect", "state.registers.y"),
    "DP Indexed Indirect, X":callIndex("addrDPIndexedIndirect", "state.registers.x"),
    "DP Indexed Indirect, Y":callIndex("addrDPIndexedIndirect", "state.registers.y"),

    "Absolute Indexed,X":callIndex("addrAbsoluteIndexed", "state.registers.x"),
    "Absolute Indexed,Y":callIndex("addrAbsoluteIndexed", "state.registers.y"),
    "Absolute Indexed, X":callIndex("addrAbsoluteIndexed", "state.registers.x"),
    "Absolute Indexed, Y":callIndex("addrAbsoluteIndexed", "state.registers.y"),

    "Absolute Long Indexed,X":callIndex("addrAbsoluteLongIndexed", "state.registers.x"),
    "Absolute Long Indexed,Y":callIndex("addrAbsoluteLongIndexed", "state.registers.y"),
    "Absolute Long Indexed, X":callIndex("addrAbsoluteLongIndexed", "state.registers.x"),
    "Absolute Long Indexed, Y":callIndex("addrAbsoluteLongIndexed", "state.registers.y"),
}