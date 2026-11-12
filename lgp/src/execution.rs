use crate::instruction::{ self, Input, Instruction, Opcode, Operand, Register};
use crate::interpreteur::{NB_INSTRUCTION, decode_instruction_64b};


fn get_operand (operand : Operand, registre: &[f32; 4],inputs: &[f32; 4] ) -> f32{
     
   match operand {
    Operand::Register(reg) => {
        registre[reg.index_registre()]
    }

    Operand::Input(input) => {
        inputs[input.index_input()]
    }
    
}
}
//corriger nb instruction
pub fn execute_programme_64b(instructions: Vec<Instruction> , registre: &[f32; 4],inputs: &[f32; 4] ) -> [f32; 5] {


  let nb_instruction = NB_INSTRUCTION as usize ;
    let mut result:[f32; 5] = [0.0; 5];

    for i in 0..nb_instruction{
      let a = get_operand(instructions[i].operande1, registre, inputs);
      let b = get_operand(instructions[i].operande2, registre, inputs);

        result[i] = match instructions[i].opcode {
            Opcode::Add => a + b,
            Opcode::Sub => a - b,
            Opcode::Mul => a * b,
            Opcode::Div => a / b,
            Opcode::Ln =>  a.ln(),
            Opcode::Exp => a.exp(),
            Opcode::Max => a.max(b),
            Opcode::Min => a.min(b),
            

    }
    //instructions[i].output = result[i
    //mettre a jour le registre ?
  }
    result
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_operand() {

        let registre: [f32; 4] = [12.0, 16.5, 8.02, 13.9];
        let inputs: [f32; 4] = [9.8, 12.5, 53.0, 11.2];

        let op1 = Operand::Register(Register::R0);
        let op2 = Operand::Register(Register::R2);
        let op3 = Operand::Input(Input::I1);
        let op4 = Operand::Input(Input::I3);

        assert_eq!(get_operand(op1, &registre, &inputs), 12.0);
        assert_eq!(get_operand(op2, &registre, &inputs), 8.02);
        assert_eq!(get_operand(op3, &registre, &inputs), 12.5);
        assert_eq!(get_operand(op4, &registre, &inputs), 11.2);
    }

     #[test]
    fn test_execute_programme_64b(){
        //011 01 001 111; // R1 = DIV (I1 R3)
        let bits: u64 = 0b1100110010000110001110011101110101100010100100000011001101001111;
         // R1 = add (R0+I3), R3 = lN (R1 R2), R2 = EXP (I1 I2), R0 = MUL (I0 R2) , R1 = DIV (I1 R3)
                                // 0 -   1 -    2 -   3
        let registre: [f32; 4] = [12.0, 16.5, 8.02, 13.9];
        let inputs: [f32; 4]   = [9.8, 12.5, 53.0, 11.2];

        let instructions = decode_instruction_64b(bits);
        let result= execute_programme_64b (instructions,&registre,&inputs);
        // R1 = add (R0+I3)
        assert_eq!(result[4], registre[0]+inputs[3]);

        // R3 = lN (R1 R2)
        assert_eq!(result[3], registre[1].ln() );

        //R2 = EXP (I1 I2)
        assert_eq!(result[2],inputs[1].exp());

        //R0 = MUL (I0 R2) 
        assert_eq!(result[1], inputs[0]*registre[2] );
        
        //R1 = DIV (I1 R3)
        assert_eq!(result[0],inputs[1]/registre[3] );
        

    }

}





