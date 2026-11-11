use crate::instruction::{Opcode,Register,Input,Operand,Instruction};


//constante
pub const INSTRUCTION_SIZE : i8 = 11;


//program input: 0, or a register: 1
fn decode_operand(operand_bits: u8) -> Operand {

    if operand_bits & 0b100 == 0 {
        Operand::Input
    } else {
        Operand::Register
    }
}