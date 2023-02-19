#include "DecodeUnit.hpp"

#include <iostream>

namespace snes {
DecodeUnit::DecodeUnit(std::vector<uint8_t> &currentInstructionRegister,
                       std::vector<uint8_t>::iterator memory)
    : cir(currentInstructionRegister), mem(memory) {}

// Using a big switch case for now because its the most simple.
// Trusting the compiler optimizations.
// Will uncomment cases as they are implemented. Leaving default to handle
// un-implemented instructions
void DecodeUnit::decode(void) {
    switch (cir[0]) {  // Address mode
        // case ADC0:  // DP Indexed Indirect,X
        //     break;
        // case ADC1:  // Stack Relative
        //     break;
        // case ADC2:  // Direct Page
        //     break;
        // case ADC3:  // DP Indirect Long
        //     break;
        case ADC4:  // Immediate
		    // Temporary placeholder
			decodedOperand = cir[1];
            break;
        // case ADC5:  // Absolute
        //     break;
        // case ADC6:  // Absolute Long
        //     break;
        // case ADC7:  // DP Indirect Indexed, Y
        //     break;
        // case ADC8:  // DP Indirect
        //     break;
        // case ADC9:  // SR Indirect Indexed,Y
        //     break;
        // case ADC10:  // DP Indexed,X
        //     break;
        // case ADC11:  // DP Indirect Long Indexed, Y
        //     break;
        // case ADC12:  // Absolute Indexed,Y
        //     break;
        // case ADC13:  // Absolute Indexed,X
        //     break;
        default:
            std::cout << "Instruction not implemented\n";
            break;
    }
}

}  // namespace snes
