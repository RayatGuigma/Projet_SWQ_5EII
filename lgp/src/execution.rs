use crate::instruction::{ self, Input, Instruction, Opcode, Operand, Register};
use crate::interpreteur::{NB_INSTRUCTION, decode_instruction_64b};

/// Récupère la valeur numérique correspondant à un opérande.
///
/// L'opérande peut désigner un registre ou une entrée du programme.
/// La fonction utilise l'indice associé à cet opérande pour accéder
/// à la valeur correspondante dans le tableau.
///
/// # Arguments
///
/// * operand - Opérande à évaluer (registre ou entrée).
/// * registre - Tableau contenant les valeurs des 4 registres.
/// * inputs - Tableau contenant les valeurs des 4 entrées.
///
/// # Returns
///
/// Retourne la valeur ddécimale correspondant à l'opérande.
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

/// Exécute un programme contenant plusieurs instructions.
///
/// Pour chaque instruction, la fonction récupère les valeurs
/// des deux opérandes, effectue l'opération correspondante
/// selon l'opcode, puis stocke le résultat dans le tableau.
///
/// Les opérations disponibles sont :
/// - Addition
/// - Soustraction
/// - Multiplication
/// - Division
/// - Logarithme népérien
/// - Exponentielle
/// - Maximum
/// - Minimum
///
/// # Arguments
///
/// * instructions - Vecteur contenant les instructions à exécuter.
/// * registre - Tableau contenant les valeurs initiales des 4 registres.
/// * inputs - Tableau contenant les valeurs des 4 entrées.
///
/// # Returns
///
/// Retourne un tableau contenant les résultats des instructions exécutées.
///
/// 




pub fn execute_programme_64b(instructions: &Vec<Instruction> , registre: &mut[f32; 4],inputs: &[f32; 4] ) -> [f32; NB_INSTRUCTION] {


  //let nb_instruction = NB_INSTRUCTION as usize ;
    let mut result:[f32; 5] = [0.0; 5];

    for i in 0..NB_INSTRUCTION{
      let a = get_operand(instructions[i].operande1, registre, inputs);
      let b = get_operand(instructions[i].operande2, registre, inputs);
        let index_ouput = instructions[i].output.index_registre();

        println!("Avant instruction {}", i);
println!("Registres : {:?}", registre);


        result[i] = match instructions[i].opcode {
            Opcode::Add => a + b,
            Opcode::Sub => a - b,
            Opcode::Mul => a * b,
            Opcode::Div => a / b,
            Opcode::Ln =>  a.ln(),
            Opcode::Exp => a.exp(),
            Opcode::Max => a.max(b),
            Opcode::Min => a.min(b),
            

    };
    registre[index_ouput]= result[i] ;
    //instructions[i].output = result[i
    //mettre a jour le registre ?
    
println!("Résultat : {}", result[i]);
println!("Après instruction {}", i);
println!("Registres : {:?}", registre);
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
        let registre_initial: [f32; 4] = [12.0, 16.5, 8.02, 13.9];
        let mut registre = registre_initial;
        let inputs: [f32; 4]   = [9.8, 12.5, 53.0, 11.2];

        let instructions = decode_instruction_64b(bits);
        let result: [f32; NB_INSTRUCTION]= execute_programme_64b (&instructions,&mut registre,&inputs);
       
       //test non fnctionnel car je fais des egalité avec des val éronnées


        //R1 = DIV (I1 R3)
        let div = inputs[1] / registre_initial[3];
        assert_eq!(result[0],div );
       

        //R0 = MUL (I0 R2) 
        let mul = inputs[0] * registre_initial[2];
        assert_eq!(result[1], mul );

        //R2 = EXP (I1 I2)
        let exp = inputs[1].exp();
        assert_eq!(result[2],exp);

        // R3 = lN (R1 R2)
        let ln = div.ln();
        assert_eq!(result[3], ln);
        
         // R1 = add (R0+I3)
         let add = mul + inputs[3];
        assert_eq!(result[4], add);  //[12.0, 23.2, 8.02, 13.9]*/


    }

}





