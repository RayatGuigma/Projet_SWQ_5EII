use crate::instruction::{ self, Input, Instruction, Opcode, Operand, Register};


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
