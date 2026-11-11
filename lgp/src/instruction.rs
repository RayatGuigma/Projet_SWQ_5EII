//instruction sur 11bits
// opcode three bits: + : 000, − : 001, × : 010, ÷ : 011, ln : 100, exp : 101, max : 110, min : 111.
// registre two bits: 𝑅[0] : 00,𝑅[1] : 01,𝑅[2] : 10,𝑅[3] : 11.
//input 3 bits: 𝐼[0] : 00,𝐼[1] : 01,𝐼[2] : 10,𝐼[3] : 11.
//program input: 0, or a register: 1
#[derive(Debug, PartialEq)]
pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
    Ln,
    Exp,
    Max,
    Min,

}


// choose_opcode pour le chois de l'operation
impl Opcode {
    
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
#[derive(Debug, PartialEq)]
pub enum Register {
    R1,
    R2,
    R3,
    R4,
}

// choose_opcode pour le chois de l'operation
impl Register {
    
    pub fn choose_register (num_registre : u8) -> Register{
        match num_registre {
            0 => Register::R1,
            1 => Register::R2,
            2 => Register::R3,
            3 => Register::R4,
            _ => panic!("Registre invalide"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Input {
    I1,
    I2,
    I3,
    I4,
}

// choose_opcode pour le chois de l'operation
impl Input {
    
    pub fn choose_Input (num_input : u8) -> Input{
        match num_input {
            0 => Input::I1,
            1 => Input::I2,
            2 => Input::I3,
            3 => Input::I4,
            _ => panic!("input invalide"),
        }
    }
}

pub struct Instruction {
    pub opcode    : Opcode,
    pub output    : Register,
    pub operande1 :  i8,
    pub operande2 :  i8,

}



#[cfg(test)]
mod tests {
    use super::*;

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
    #[test]
    #[should_panic(expected = "opcode invalide")]
    fn test_choose_opcode_invalide(){
        Opcode::choose_opcode(8);
        Opcode::choose_opcode(15);
    }

}
