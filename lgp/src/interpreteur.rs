use crate::instruction::{ Input, Instruction, Opcode, Operand, Register};

//constante
pub const INSTRUCTION_SIZE : i8 = 11;

//program input: 0, or a register: 1
/*
Input : 0b000 - 0b001 - 0b010 -0b011
registre : 0b100 - 0b101 - 0b110 - 0b111

*/

fn decode_operand(operand_bits: u8) -> Operand {

    if operand_bits & 0b100 == 0 {
        Operand::Input (Input::choose_input(operand_bits & 0b011))
    } else {
        Operand::Register (Register::choose_register(operand_bits & 0b011))
    } 
}
//000 00 000 000 |0 0000 
//opcode (3bits) -registre de sortie (2bits) -operande1 (3bits) -operande2 (3bits)
fn decode_instruction(inst_bits: u16) -> Instruction {
    //let instruction_bits = inst_bits >>5;
    let instruction_bits = inst_bits ;
    let bit_opcode  = ((instruction_bits & 0b11100000000)>> 8 ) as u8 ;
    let opcode = Opcode::choose_opcode (bit_opcode);

    let bit_destination  =((instruction_bits & 0b00011000000)>> 6 ) as u8 ;
    let destination = Register::choose_register (bit_destination );

    let bit_operande1 = ((instruction_bits & 0b00000111000)>> 3 ) as u8 ;
      let op1= decode_operand(bit_operande1) ;

    let bit_operande2 = ((instruction_bits & 0b00000000111)) as u8 ;
     let op2= decode_operand(bit_operande2);


    let instruction =  Instruction {
        opcode,
        output: destination,
        operande1: op1,
        operande2: op2,
    };

    instruction
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_operand (){
        assert_eq!(decode_operand(0b000),Operand::Input(Input::I0));
        assert_eq!(decode_operand(0b001),Operand::Input(Input::I1));
        assert_eq!(decode_operand(0b010),Operand::Input(Input::I2));
        assert_eq!(decode_operand(0b011),Operand::Input(Input::I3));

        assert_eq!(decode_operand(0),Operand::Input(Input::I0));
        assert_eq!(decode_operand(1),Operand::Input(Input::I1));
        assert_eq!(decode_operand(2),Operand::Input(Input::I2));
        assert_eq!(decode_operand(3),Operand::Input(Input::I3));

        assert_eq!(decode_operand(0b100),Operand::Register(Register::R0));
        assert_eq!(decode_operand(0b101),Operand::Register(Register::R1));
        assert_eq!(decode_operand(0b110),Operand::Register(Register::R2));
        assert_eq!(decode_operand(0b111),Operand::Register(Register::R3));

        assert_eq!(decode_operand(4),Operand::Register(Register::R0));
        assert_eq!(decode_operand(5),Operand::Register(Register::R1));
        assert_eq!(decode_operand(6),Operand::Register(Register::R2));
        assert_eq!(decode_operand(7),Operand::Register(Register::R3));
    }

    #[test]
    fn test_decode_instruction(){
        let bits1: u16 = 0b00001100011; // R1 = add (R0+I3)
        let instruction1 = decode_instruction(bits1);

        assert_eq!(instruction1.opcode, Opcode::Add);
        assert_eq!(instruction1.output, Register::R1);
        assert_eq!(instruction1.operande1,Operand::Register(Register::R0));
        assert_eq!(instruction1.operande2,Operand::Input(Input::I3));

        let bits2: u16 = 0b10011101110; // R3 = lN (R1 R2)
        let instruction2 = decode_instruction(bits2);

        assert_eq!(instruction2.opcode, Opcode::Ln);
        assert_eq!(instruction2.output, Register::R3);
        assert_eq!(instruction2.operande1,Operand::Register(Register::R1));
        assert_eq!(instruction2.operande2,Operand::Register(Register::R2));

    }

}
