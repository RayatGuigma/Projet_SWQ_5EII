use crate::instruction::{ self, Input, Instruction, Opcode, Operand, Register};

//constante
/// Nombre de bits utilisés pour coder une instruction.
pub const INSTRUCTION_SIZE : usize = 11;
/// Nombre d'instructions contenues dans un programme de 64 bits.
pub const NB_INSTRUCTION :usize = 5;



/// Décode un opérande à partir de sa représentation binaire.
///
/// Le bit de poids fort permet de déterminer le type d'opérande :
/// - 0 : l'opérande correspond à une entrée.
/// - 1 : l'opérande correspond à un registre.
///
/// Les deux bits restants permettent de déterminer l'indice
/// de l'entrée ou du registre.
///
/// # Arguments
///
/// * operand_bits - Valeur binaire codée sur 3 bits représentant l'opérande.
///
/// # Returns
///
/// Retourne un Operand correspondant à une entrée ou à un registre.

fn decode_operand(operand_bits: u8) -> Operand {

    if operand_bits & 0b100 == 0 {
        Operand::Input (Input::choose_input(operand_bits & 0b011))
    } else {
        Operand::Register (Register::choose_register(operand_bits & 0b011))
    } 
}

/// Décode une instruction codée sur 11 bits.
///
/// Une instruction est organisée de la manière suivante :
///
/// * Opcode : 3 bits 
/// * Registre de destination : 2 bits 
/// * Premier opérande : 3 bits 
/// * Deuxième opérande : 3 bits 
///
/// La fonction extrait chaque champ à l'aide de masques binaires
/// et de décalages, puis construit une structure Instruction.
///
/// # Arguments
///
/// * inst_bits - Instruction codée sur 11 bits, stockée dans un u16
///
/// # Returns
///
/// Retourne une structure Instruction contenant l'opération,
/// le registre de destination et les deux opérandes.


//000 00 000 000 |0 0000 
//opcode (3bits) -registre de sortie (2bits) -operande1 (3bits
fn decode_instruction_11b(inst_bits: u16) -> Instruction {
    //let instruction_bits = inst_bits >>5;
    // Recupopcode (bits 10 à 8)
    let instruction_bits = inst_bits ;
    let bit_opcode  = ((instruction_bits & 0b11100000000)>> 8 ) as u8 ;
    let opcode = Opcode::choose_opcode (bit_opcode);

    // Recup registre de destination (bits 7 à 6)
    let bit_destination  =((instruction_bits & 0b00011000000)>> 6 ) as u8 ;
    let destination = Register::choose_register (bit_destination );

    // Recup op1 (bits 5 à 3)
    let bit_operande1 = ((instruction_bits & 0b00000111000)>> 3 ) as u8 ;
      let op1= decode_operand(bit_operande1) ;

      // Recup op2 (bits 2 à 0)
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


/// Décode un programme de 64 bits contenant 5 instructions.
///
/// La fonction extrait successivement les 5 instructions du programme.
/// Chaque instruction occupe 11 bits. Pour chaque instruction,
/// un décalage binaire permet de récupérer les 11 bits correspondants,
/// puis la fonction decode_instruction_11b réalise son décodage.
///
/// # Arguments
///
/// * programme - Programme complet codé sur 64 bits.
///
/// # Returns
///
/// Retourne un vecteur d'instruction contenant les
/// 5 instructions décodées, dans leur ordre d'exécution.

pub fn decode_instruction_64b (programme: u64)  -> Vec<Instruction> {
        let mut instructions = Vec::new();

         for i in 0..NB_INSTRUCTION{
            let shift = i * INSTRUCTION_SIZE;
            let instruction_actuel = ((programme >> shift) & 0x7FF) as u16;
            instructions.push(decode_instruction_11b(instruction_actuel));
         } 
         instructions
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
        let instruction1 = decode_instruction_11b(bits1);

        assert_eq!(instruction1.opcode, Opcode::Add);
        assert_eq!(instruction1.output, Register::R1);
        assert_eq!(instruction1.operande1,Operand::Register(Register::R0));
        assert_eq!(instruction1.operande2,Operand::Input(Input::I3));

        let bits2: u16 = 0b10011101110; // R3 = lN (R1 R2)
        let instruction2 = decode_instruction_11b(bits2);

        assert_eq!(instruction2.opcode, Opcode::Ln);
        assert_eq!(instruction2.output, Register::R3);
        assert_eq!(instruction2.operande1,Operand::Register(Register::R1));
        assert_eq!(instruction2.operande2,Operand::Register(Register::R2));

    }
    #[test]
    fn test_decode_instruction_64b(){
        //011 01 001 111; // R1 = DIV (I1 R3)
        let bits: u64 = 0b1100110010000110001110011101110101100010100100000011001101001111;
         // R1 = add (R0+I3), R3 = lN (R1 R2), R2 = EXP (I1 I2), R0 = MUL (I0 R2) , R1 = DIV (I1 R3)

         let instructions = decode_instruction_64b(bits);
        //for i in 0..NB_INSTRUCTION
        
        // R1 = add (R0+I3)
        assert_eq!(instructions[4].opcode, Opcode::Add);
        assert_eq!(instructions[4].output, Register::R1);
        assert_eq!(instructions[4].operande1,Operand::Register(Register::R0));
        assert_eq!(instructions[4].operande2,Operand::Input(Input::I3));

        // R3 = lN (R1 R2)
        assert_eq!(instructions[3].opcode, Opcode::Ln);
        assert_eq!(instructions[3].output, Register::R3);
        assert_eq!(instructions[3].operande1,Operand::Register(Register::R1));
        assert_eq!(instructions[3].operande2,Operand::Register(Register::R2));

        //R2 = EXP (I1 I2)
        assert_eq!(instructions[2].opcode, Opcode::Exp);
        assert_eq!(instructions[2].output, Register::R2);
        assert_eq!(instructions[2].operande1,Operand::Input(Input::I1));
        assert_eq!(instructions[2].operande2,Operand::Input(Input::I2));

        //R0 = MUL (I0 R2) 
                assert_eq!(instructions[1].opcode, Opcode::Mul);
        assert_eq!(instructions[1].output, Register::R0);
        assert_eq!(instructions[1].operande1,Operand::Input(Input::I0));
        assert_eq!(instructions[1].operande2,Operand::Register(Register::R2));

        //R1 = DIV (I1 R3)
        assert_eq!(instructions[0].opcode, Opcode::Div);
        assert_eq!(instructions[0].output, Register::R1);
        assert_eq!(instructions[0].operande1,Operand::Input(Input::I1));
        assert_eq!(instructions[0].operande2,Operand::Register(Register::R3));


    }

}
