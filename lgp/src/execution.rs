use crate::instruction::{ self, Input, Instruction, Opcode, Operand, Register};
use crate::interpreteur::NB_INSTRUCTION;

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

pub fn execute_programme_64b(instructions: Vec<Instruction> , registre: &[f32; 4],inputs: &[f32; 4] ) -> f32 {

  let mut result:[f32; 4] = [0.0; 4];
  let nb_instruction = NB_INSTRUCTION as usize ;

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
  }
    result[3]
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
}



