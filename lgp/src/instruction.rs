//instruction sur 11bits
// opcode three bits: + : 000, − : 001, × : 010, ÷ : 011, ln : 100, exp : 101, max : 110, min : 111.
// registre two bits: 𝑅[0] : 00,𝑅[1] : 01,𝑅[2] : 10,𝑅[3] : 11.
//input 3 bits: 𝐼[0] : 00,𝐼[1] : 01,𝐼[2] : 10,𝐼[3] : 11.
//program input: 0, or a register: 1

pub enum Opcode {
    add,
    sub,
    mul,
    div,
    ln,
    exp,
    max,
    min,

}


// choose_opcode pour le chois de l'operation
impl Opcode {
    
    pub fn choose_opcode (bit_hex : u8) -> Opcode{
        match bit_hex {
            0 => Opcode::add,
            1 => Opcode::sub,
            2 => Opcode::mul,
            3 => Opcode::div,
            4 => Opcode::ln,
            5 => Opcode::exp,
            6 => Opcode::max,
            7 => Opcode::min,
        }
    }
}

pub enum Register {
    R1,
    R2,
    R3,
    R4,
}

// choose_opcode pour le chois de l'operation
impl Register {
    
    pub fn choose_register (nb_registre : u8) -> Register{
        match bit_hex {
            0 => Register::R1,
            1 => Register::R2,
            2 => Register::R3,
            3 => Register::R4,
        }
    }
}

pub struct Instruction {
    pub opcode    : Opcode,
    pub output    : Register,
    pub operande1 :  i8,
    pub operande2 :  i8,

}

