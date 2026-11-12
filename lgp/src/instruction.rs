//! Module de définition des instructions du programme.
//!
//! Ce module définit les opcodes, les registres, les entrées,
//! les opérandes et la structure Instruction.
//! Il contient également les tests unitaires associés.


///  Définit les opérations disponibles dans le processeur.
/// Chaque opération est représentée par un opcode codé sur 3 bits.
#[derive(Debug, PartialEq, Clone,Copy)]
pub enum Opcode {
    /// Addition : code binaire 000
    Add,
    /// Soustraction : code binaire 001
    Sub,
    /// Multiplication : code binaire 010
    Mul,
    /// Division : code binaire 011
    Div,
    /// Logarithme népérien : code binaire 100
    Ln,
    /// Exponentielle : code binaire 101
    Exp,
    /// Maximum : code binaire 110
    Max,
    /// Minimum : code binaire 111
    Min,

}


impl Opcode {

    /// la fonction choose_opcode convertit une valeur numérique en opcode.
    ///
    /// # Arguments
    ///
    /// * bit_hex : valeur numérique représentant l'opcode (0 à 7).
    ///
    /// # Returns
    ///
    /// Retourne l'opération correspondante.
    ///
    /// # Panics
    ///
    /// Déclenche une panique si la valeur est supérieure à 7.
    pub fn choose_opcode (bit_hex : u8) -> Opcode{
        match bit_hex {
            0 => Opcode::Add,
            1 => Opcode::Sub,
            2 => Opcode::Mul,
            3 => Opcode::Div,
            4 => Opcode::Ln,
            5 => Opcode::Exp,
            6 => Opcode::Max,
            7 => Opcode::Min,
            _ => panic!("opcode invalide"),
        }
    }
}

/// Définit les registres utilisables.
/// Chaque registre est codé sur 2 bits.
#[derive(Debug, PartialEq, Clone,Copy)]
pub enum Register {
    /// Registre 0 : code binaire 00
    R0,
    /// Registre 1 : code binaire 01
    R1,
    /// Registre 2 : code binaire 10
    R2,
    /// Registre 3 : code binaire 11
    R3,
}

// choose_opcode pour le chois de l'operation
impl Register {
        
    /// choose_register convertit une valeur numérique en un registre du programme.
    ///
    /// Cette fonction permet d'associer une valeur comprise entre 0 et 3
    /// à l'une des quatre registres disponibles (R0, R1, R2 ou R3).
    ///
    /// # Arguments
    ///
    /// * num_registre - Valeur numérique représentant l'indice du registre (0 à 3).
    ///
    /// # Returns
    ///
    /// Retourne la variante Registre correspondant à la valeur fournie
    pub fn choose_register (num_registre : u8) -> Register{
        match num_registre {
            0 => Register::R0,
            1 => Register::R1,
            2 => Register::R2,
            3 => Register::R3,
            _ => panic!("Registre invalide"),
        }
    }

    /// index_registre retourne l'indice numérique correspondant au registre.
    ///
    /// Cette fonction permet de convertir un registre (R0, R1, R2 ou R3)
    /// en un indice utilisable pour accéder à une case d'un tableau.
    ///
    /// # Returns
    ///
    /// Retourne un usize correspondant à l'indice du registre

      pub fn index_registre(&self) -> usize {
        match self {
            Register::R0 => 0,
            Register::R1 => 1,
            Register::R2 => 2,
            Register::R3 => 3,
        }
      }

}


/// Définit les entrées.
/// Chaque entrées est codé sur 2 bits.
#[derive(Debug, PartialEq, Clone,Copy)]
pub enum Input {
    /// Input 0 : code binaire 00
    I0,
    /// Input 1 : code binaire 01
    I1,
    /// Input 2 : code binaire 10
    I2,
    /// Input 3 : code binaire 11
    I3,
}


impl Input {
    /// Convertit une valeur numérique en une entrée du programme.
    ///
    /// Cette fonction permet d'associer une valeur comprise entre 0 et 3
    /// à l'une des quatre entrées disponibles (I0, I1, I2 ou I3).
    ///
    /// # Arguments
    ///
    /// * num_input - Valeur numérique représentant l'indice de l'entrée (0 à 3).
    ///
    /// # Returns
    ///
    /// Retourne la variante Input correspondant à la valeur fournie 
    pub fn choose_input (num_input : u8) -> Input{
        match num_input {
            0 => Input::I0,
            1 => Input::I1,
            2 => Input::I2,
            3 => Input::I3,
            _ => panic!("input invalide"),
        }
    }

    /// index_input retourne l'indice numérique correspondant à l'entrée.
    ///
    /// Cette fonction permet de convertir une entrée (I0, I1, I2 ou I3)
    /// en un indice utilisable pour accéder à une case d'un tableau.
    ///
    /// # Returns
    ///
    /// Retourne un usize correspondant à l'indice de l'entrée
     pub fn index_input (&self) -> usize {
        match self {
            Input::I0 => 0,
            Input::I1 => 1,
            Input::I2 => 2,
            Input::I3 => 3,
        }
    }
}


/// Définit les opérandes des instruction.
/// Chaque operande est codé sur 3 bits.
#[derive(Debug, PartialEq, Clone,Copy)]
pub enum Operand {
    Register(Register),
    Input(Input),
}


/// Représente une instruction du programme.
///
/// Une instruction contient l'opération à effectuer, le registre
/// dans lequel stocker le résultat et les deux opérandes utilisés
/// pour réaliser le calcul.
///
/// # Fields
///
/// * Opcode - Opération à effectuer (addition, soustraction,
///   multiplication, division, logarithme, exponentielle, maximum ou minimum).
/// * Output - Registre dans lequel sera stocké le résultat de l'opération.
/// * Operande1 - Premier opérande, qui peut être un registre ou une entrée.
/// * Operande2 - Deuxième opérande, qui peut être un registre ou une entrée.
#[derive(Debug, PartialEq)]
pub struct Instruction {
    /// Opération à effectuer.
    pub opcode    : Opcode,
    /// Registre de destination du résultat.
    pub output    : Register,
    /// Premier opérande de l'instruction.
    pub operande1 : Operand,
    /// Deuxième opérande de l'instruction.
    pub operande2 : Operand,

}



#[cfg(test)]
mod tests {
    use super::*;
    /// La fonction test_choose_opcode_valide vérifie que chaque valeur numérique valide
    /// correspond au bon opcode.
    #[test]
    fn test_choose_opcode_valide (){
       assert_eq!(Opcode::choose_opcode(0), Opcode::Add);
       assert_eq!(Opcode::choose_opcode(1), Opcode::Sub);
       assert_eq!(Opcode::choose_opcode(2), Opcode::Mul);
       assert_eq!(Opcode::choose_opcode(3), Opcode::Div);
       assert_eq!(Opcode::choose_opcode(4), Opcode::Ln);
       assert_eq!(Opcode::choose_opcode(5), Opcode::Exp);
       assert_eq!(Opcode::choose_opcode(6), Opcode::Max);
       assert_eq!(Opcode::choose_opcode(7), Opcode::Min);
    }

    /// La fonction test_choose_opcode_invalide vérifie qu'une valeur d'opcode invalide provoque une panique.
    #[test]
    #[should_panic(expected = "opcode invalide")]
    fn test_choose_opcode_invalide(){
        Opcode::choose_opcode(8);
        //Opcode::choose_opcode(15);
    }

    /// La fonction test_choose_register_valide vérifie que les valeurs numériques de 0 à 3
    /// sont correctement converties en registres.
    #[test]
     fn test_choose_register_valide (){
        assert_eq!(Register::choose_register(0),Register::R0);
        assert_eq!(Register::choose_register(1),Register::R1);
        assert_eq!(Register::choose_register(2),Register::R2);
        assert_eq!(Register::choose_register(3),Register::R3);

     }

     /// La fonction test_choose_register_invalide vérifie qu'une valeur de registre invalide provoque une panique.
      #[test]
    #[should_panic(expected = "Registre invalide")]
    fn test_choose_register_invalide(){
        Register::choose_register(4);
        //Register::choose_register(10);
    }

    /// la fonction test_choose_input_valide vérifie que les valeurs numériques de 0 à 3
    /// sont correctement converties en entrées.
     #[test]
     fn test_choose_input_valide (){
        assert_eq!(Input::choose_input(0),Input::I0);
        assert_eq!(Input::choose_input(1),Input::I1);
        assert_eq!(Input::choose_input(2),Input::I2);
        assert_eq!(Input::choose_input(3),Input::I3);

     }

     /// la fonction test_choose_input_invalide vérifie qu'une valeur d'entrée invalide provoque une panique.
    #[test]
    #[should_panic(expected = "input invalide")]
    fn test_choose_input_invalide(){
        Input::choose_input(4);
        Input::choose_input(10);

    }

    /// la fonction test_index_input vérifie que chaque entrée retourne le bon indice numérique.
    #[test]
    fn test_index_input() {
        assert_eq!(Input::I0.index_input(), 0);
        assert_eq!(Input::I1.index_input(), 1);
        assert_eq!(Input::I2.index_input(), 2);
        assert_eq!(Input::I3.index_input(), 3);
    }

    /// La test_index_registre fonction vérifie que chaque registre retourne le bon indice numérique.
    #[test]
    fn test_index_registre() {
        assert_eq!(Register::R0.index_registre(), 0);
        assert_eq!(Register::R1.index_registre(), 1);
        assert_eq!(Register::R2.index_registre(), 2);
        assert_eq!(Register::R3.index_registre(), 3);
    }

}
