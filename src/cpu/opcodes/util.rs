use crate::ops::{Local16, Local8, R16, R8};

// Many instructions use a common bit pattern to designate single registers.
pub fn reg_bits(bits: u8) -> Local8 {
    match bits & 0b111 {
        0b111 => Local8::Reg8(R8::A),
        0b000 => Local8::Reg8(R8::B),
        0b001 => Local8::Reg8(R8::C),
        0b010 => Local8::Reg8(R8::D),
        0b011 => Local8::Reg8(R8::E),
        0b100 => Local8::Reg8(R8::H),
        0b101 => Local8::Reg8(R8::L),
        0b110 => Local8::RegIndirecto8(R16::HL),
        _ => unreachable!(),
    }
}
pub fn le_immediate(n0: u8, n1: u8) -> Local16 {
    Local16::Inmediato(u16::from_le_bytes([n0, n1]))
}

// recibe una localizacion16 n->bajo  n1->alto devuelve una localizacion16 con un u16
pub fn le_imm_indir_le(n0: u8, n1: u8) -> Local16 {
    Local16::InmediatoIndirecto(u16::from_le_bytes([n0, n1]))
}

pub fn le_imm_indir_be(n0: u8, n1: u8) -> Local16 {
    Local16::InmediatoIndirecto(u16::from_be_bytes([n0, n1]))
}

// Cambiada
// pub fn jump_conditional(c: u8) -> JumpConditional {
//     match c & 0b111 {
//         0b000 => JumpConditional::NonZero,
//         0b001 => JumpConditional::Zero,
//         0b010 => JumpConditional::NoCarry,
//         0b011 => JumpConditional::Carry,
//         0b100 => JumpConditional::ParityOdd,
//         0b101 => JumpConditional::ParityEven,
//         0b110 => JumpConditional::SignPositive,
//         0b111 => JumpConditional::SignNegative,
//         _ => unreachable!(),
//     }
// }