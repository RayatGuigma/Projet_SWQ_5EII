use crate::instruction::{Opcode,Register,Input,Operand,Instruction};


//constante
pub const INSTRUCTION_SIZE : i8 = 11;


//program input: 0, or a register: 1
/*
Input : 0b000 - 0b001 - 0b010 -0b011
registre : 0b100 - 0b101 - 0b110 - 0b111

*/


fn decode_operand(operand_bits: u8) -> Operand {

    if operand_bits & 0b100 == 0 {
        Operand::Input
    } else {
        Operand::Register
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_operand (){
        assert_eq!(decode_operand(0b000),Operand::Input);
        assert_eq!(decode_operand(0b001),Operand::Input);
        assert_eq!(decode_operand(0b010),Operand::Input);
        assert_eq!(decode_operand(0b011),Operand::Input);

        assert_eq!(decode_operand(0),Operand::Input);
        assert_eq!(decode_operand(1),Operand::Input);
        assert_eq!(decode_operand(2),Operand::Input);
        assert_eq!(decode_operand(3),Operand::Input);

        assert_eq!(decode_operand(0b100),Operand::Register);
        assert_eq!(decode_operand(0b101),Operand::Register);
        assert_eq!(decode_operand(0b110),Operand::Register);
        assert_eq!(decode_operand(0b111),Operand::Register);

        assert_eq!(decode_operand(4),Operand::Register);
        assert_eq!(decode_operand(5),Operand::Register);
        assert_eq!(decode_operand(6),Operand::Register);
        assert_eq!(decode_operand(7),Operand::Register);
    }

}