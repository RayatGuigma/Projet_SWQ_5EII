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

pub enum Register {
    R1,
    R2,
    R3,
    R4,
}


pub struct Instruction {
    pub opcode    : Opcode,
    pub output    : Register,
    pub operande1 :  i8,
    pub operande2 :  i8,

}