// https://clrhome.org/table/

mod util;
mod bit;

mod ix;
mod _test_misc;
mod iy;
mod iy_bit;
mod ix_bit;
mod tests;
mod misc;

use crate::cpu::opcodes::misc::analiza;
use crate::cpu::opcodes::util::{le_imm_indir_be, le_imm_indir_le, le_immediate};
use crate::ops::Local8::{InmediatoIndirecto8, RegIndirecto8};
use crate::ops::Op::*;
use crate::ops::{Local16, Local8::*, Op, SaltoCondicional, R16, R8};

// Analiza una serie de bytes en un código de operación.
// Los códigos de operación pueden tener hasta cuatro bytes, pero a menudo son menos.
// El tamaño de uso de la tupla es la cantidad de bytes consumidos.
// El contador del programa debe incrementarse en esta cantidad
pub fn opcode(code: [u8; 4]) -> (Op, usize) {
    //dbg_hex!(code);
    //println_tee!("opcode a analizar {:02X} {:02X} {:02X} {:02X} ", code[0],code[1],code[2],code[3]);
    match code {
        [0x00, _, _, _] => (NOP, 1),
        [0x01, n1, n2, _] => (LD16BIG(Local16::Reg(R16::BC), le_immediate(n1, n2)), 3),
        [0x02, _, _, _] => (LD8BIG(RegIndirecto8(R16::BC), Reg8(R8::A)), 1),
        [0x03, _, _, _] => (INC16(Local16::Reg(R16::BC), Local16::Reg(R16::BC)), 1),
        [0x04, _, _, _] => (INC8(Reg8(R8::B)), 1),
        [0x05, _, _, _] => (DEC8(Reg8(R8::B)), 1),
        [0x06, n1, _, _] => (LD8BIG(Reg8(R8::B), Inmediato8(n1)), 2),
        [0x07, _, _, _] => (RLCA, 1),
        [0x08, _, _, _] => (EX16, 1),
        [0x09, _, _, _] => (ADD16BIG(Local16::Reg(R16::HL), Local16::Reg(R16::BC)), 1),
        [0x0A, _, _, _] => (LD8BIG(Reg8(R8::A), RegIndirecto8(R16::BC)), 1),
        [0x0B, _, _, _] => (DEC16(Local16::Reg(R16::BC), Local16::Reg(R16::BC)), 1),
        [0x0C, _, _, _] => (INC8(Reg8(R8::C)), 1),
        [0x0D, _, _, _] => (DEC8(Reg8(R8::C)), 1),
        [0x0E, n1, _, _] => (LD8BIG(Reg8(R8::C), Inmediato8(n1)), 2),
        [0x0F, _, _, _] => (RRCA, 1),
        [0x10, n1, _, _] => (DJNZ(n1 as i8), 2),
        [0x11, n1, n2, _] => (LD16BIG(Local16::Reg(R16::DE), le_immediate(n1, n2)), 3),
        [0x12, _, _, _] => (LD8BIG(RegIndirecto8(R16::DE), Reg8(R8::A)), 1),
        [0x13, _, _, _] => (INC16(Local16::Reg(R16::DE), Local16::Reg(R16::DE)), 1),
        [0x14, _, _, _] => (INC8(Reg8(R8::D)), 1),
        [0x15, _, _, _] => (DEC8(Reg8(R8::D)), 1),
        [0x16, n1, _, _] => (LD8BIG(Reg8(R8::D), Inmediato8(n1)), 2),
        [0x17, _, _, _] => (RLA, 1),
        [0x18, n1, _, _] => (JR(SaltoCondicional::Incondicional, n1 as i8), 2),
        [0x19, _, _, _] => (ADD16BIG(Local16::Reg(R16::HL), Local16::Reg(R16::DE)), 1),
        [0x1A, _, _, _] => (LD8BIG(Reg8(R8::A), RegIndirecto8(R16::DE)), 1),
        [0x1B, _, _, _] => (DEC16(Local16::Reg(R16::DE), Local16::Reg(R16::DE)), 1),
        [0x1C, _, _, _] => (INC8(Reg8(R8::E)), 1),
        [0x1D, _, _, _] => (DEC8(Reg8(R8::E)), 1),
        [0x1E, n1, _, _] => (LD8BIG(Reg8(R8::E), Inmediato8(n1)), 2),
        [0x1F, _, _, _] => (RRA, 1),
        [0x20, n1, _, _] => (JR(SaltoCondicional::NoCero, n1 as i8), 2),
        [0x21, n1, n2, _] => (LD16BIG(Local16::Reg(R16::HL), le_immediate(n1, n2)), 3),
        //[0x22, n1, n2, _] => (LD16(le_imm_indir_le(n1, n2), Local16::Reg(R16::HL)), 3,),
        [0x22, n1, n2, _] => (LD16BIG(le_imm_indir_be(n2, n1), Local16::Reg(R16::HL)), 3,),
        [0x23, _, _, _] => (INC16(Local16::Reg(R16::HL), Local16::Reg(R16::HL)), 1),
        [0x24, _, _, _] => (INC8(Reg8(R8::H)), 1),
        [0x25, _, _, _] => (DEC8(Reg8(R8::H)), 1),
        [0x26, n1, _, _] => (LD8BIG(Reg8(R8::H), Inmediato8(n1)), 2),
        [0x27, _, _, _] => (DAA, 1),
        [0x28, n1, _, _] => (JR(SaltoCondicional::Cero, n1 as i8), 2),
        [0x29, _, _, _] => (ADD16BIG(Local16::Reg(R16::HL), Local16::Reg(R16::HL)), 1),
        [0x2A, n1, n2, _] => (LD16BIG(Local16::Reg(R16::HL), le_imm_indir_le(n1, n2)), 3,),
        [0x2B, _, _, _] => (DEC16(Local16::Reg(R16::HL), Local16::Reg(R16::HL)), 1),
        [0x2C, _, _, _] => (INC8(Reg8(R8::L)), 1),
        [0x2D, _, _, _] => (DEC8(Reg8(R8::L)), 1),
        [0x2E, n1, _, _] => (LD8BIG(Reg8(R8::L), Inmediato8(n1)), 2),
        [0x2F, _, _, _] => (CPL, 1),
        [0x30, n1, _, _] => (JR(SaltoCondicional::NoCarry, n1 as i8), 2),
        // [0x31, n1, n2, _] => (LD16(Local16::Reg(R16::SP), le_immediate(n1, n2)), 3),
        [0x31, n1, n2, _] => (LD16BIG(Local16::Reg(R16::SP), le_immediate(n1, n2)), 3),
        [0x32, n1, n2, _] => {
            let addr = u16::from(n1) | (u16::from(n2) << 8);
            (LD8BIG(InmediatoIndirecto8(addr), Reg8(R8::A)), 3,)
        }
        [0x33, _, _, _] => (INC16(Local16::Reg(R16::SP), Local16::Reg(R16::SP)), 1),
        [0x34, _, _, _] => (INC8BIG(RegIndirecto8(R16::HL)), 1),
        [0x35, _, _, _] => (DEC8BIG(RegIndirecto8(R16::HL)), 1),
        [0x36, n1, _, _] => (LD8BIG(RegIndirecto8(R16::HL), Inmediato8(n1)), 2),
        [0x37, _, _, _] => (SCF, 1),
        [0x38, n1, _, _] => (JR(SaltoCondicional::Carry, n1 as i8), 2),
        [0x39, _, _, _] => (ADD16BIG(Local16::Reg(R16::HL), Local16::Reg(R16::SP)), 1),
        [0x3A, n1, n2, _] => {
            let addr = u16::from(n1) | (u16::from(n2) << 8);
            (LD8BIG(Reg8(R8::A), InmediatoIndirecto8(addr)), 3)
        }
        [0x3B, _, _, _] => (DEC16(Local16::Reg(R16::SP), Local16::Reg(R16::SP)), 1),
        [0x3C, _, _, _] => (INC8(Reg8(R8::A)), 1),
        [0x3D, _, _, _] => (DEC8(Reg8(R8::A)), 1),
        [0x3E, n1, _, _] => (LD8BIG(Reg8(R8::A), Inmediato8(n1)), 2),
        [0x3F, _, _, _] => (CCF, 1),
        [0x40, _, _, _] => (LD8BIG(Reg8(R8::B), Reg8(R8::B)), 1),
        [0x41, _, _, _] => (LD8BIG(Reg8(R8::B), Reg8(R8::C)), 1),
        [0x42, _, _, _] => (LD8BIG(Reg8(R8::B), Reg8(R8::D)), 1),
        [0x43, _, _, _] => (LD8BIG(Reg8(R8::B), Reg8(R8::E)), 1),
        [0x44, _, _, _] => (LD8BIG(Reg8(R8::B), Reg8(R8::H)), 1),
        [0x45, _, _, _] => (LD8BIG(Reg8(R8::B), Reg8(R8::L)), 1),
        [0x46, _, _, _] => (LD8BIG(Reg8(R8::B), RegIndirecto8(R16::HL)), 1),
        [0x47, _, _, _] => (LD8BIG(Reg8(R8::B), Reg8(R8::A)), 1),
        [0x48, _, _, _] => (LD8BIG(Reg8(R8::C), Reg8(R8::B)), 1),
        [0x49, _, _, _] => (LD8BIG(Reg8(R8::C), Reg8(R8::C)), 1),
        [0x4A, _, _, _] => (LD8BIG(Reg8(R8::C), Reg8(R8::D)), 1),
        [0x4B, _, _, _] => (LD8BIG(Reg8(R8::C), Reg8(R8::E)), 1),
        [0x4C, _, _, _] => (LD8BIG(Reg8(R8::C), Reg8(R8::H)), 1),
        [0x4D, _, _, _] => (LD8BIG(Reg8(R8::C), Reg8(R8::L)), 1),
        [0x4E, _, _, _] => (LD8BIG(Reg8(R8::C), RegIndirecto8(R16::HL)), 1),
        [0x4F, _, _, _] => (LD8BIG(Reg8(R8::C), Reg8(R8::A)), 1),
        [0x50, _, _, _] => (LD8BIG(Reg8(R8::D), Reg8(R8::B)), 1),
        [0x51, _, _, _] => (LD8BIG(Reg8(R8::D), Reg8(R8::C)), 1),
        [0x52, _, _, _] => (LD8BIG(Reg8(R8::D), Reg8(R8::D)), 1),
        [0x53, _, _, _] => (LD8BIG(Reg8(R8::D), Reg8(R8::E)), 1),
        [0x54, _, _, _] => (LD8BIG(Reg8(R8::D), Reg8(R8::H)), 1),
        [0x55, _, _, _] => (LD8BIG(Reg8(R8::D), Reg8(R8::L)), 1),
        [0x56, _, _, _] => (LD8BIG(Reg8(R8::D), RegIndirecto8(R16::HL)), 1),
        [0x57, _, _, _] => (LD8BIG(Reg8(R8::D), Reg8(R8::A)), 1),
        [0x58, _, _, _] => (LD8BIG(Reg8(R8::E), Reg8(R8::B)), 1),
        [0x59, _, _, _] => (LD8BIG(Reg8(R8::E), Reg8(R8::C)), 1),
        [0x5A, _, _, _] => (LD8BIG(Reg8(R8::E), Reg8(R8::D)), 1),
        [0x5B, _, _, _] => (LD8BIG(Reg8(R8::E), Reg8(R8::E)), 1),
        [0x5C, _, _, _] => (LD8BIG(Reg8(R8::E), Reg8(R8::H)), 1),
        [0x5D, _, _, _] => (LD8BIG(Reg8(R8::E), Reg8(R8::L)), 1),
        [0x5E, _, _, _] => (LD8BIG(Reg8(R8::E), RegIndirecto8(R16::HL)), 1),
        [0x5F, _, _, _] => (LD8BIG(Reg8(R8::E), Reg8(R8::A)), 1),
        [0x60, _, _, _] => (LD8BIG(Reg8(R8::H), Reg8(R8::B)), 1),
        [0x61, _, _, _] => (LD8BIG(Reg8(R8::H), Reg8(R8::C)), 1),
        [0x62, _, _, _] => (LD8BIG(Reg8(R8::H), Reg8(R8::D)), 1),
        [0x63, _, _, _] => (LD8BIG(Reg8(R8::H), Reg8(R8::E)), 1),
        [0x64, _, _, _] => (LD8BIG(Reg8(R8::H), Reg8(R8::H)), 1),
        [0x65, _, _, _] => (LD8BIG(Reg8(R8::H), Reg8(R8::L)), 1),
        [0x66, _, _, _] => (LD8BIG(Reg8(R8::H), RegIndirecto8(R16::HL)), 1),
        [0x67, _, _, _] => (LD8BIG(Reg8(R8::H), Reg8(R8::A)), 1),
        [0x68, _, _, _] => (LD8BIG(Reg8(R8::L), Reg8(R8::B)), 1),
        [0x69, _, _, _] => (LD8BIG(Reg8(R8::L), Reg8(R8::C)), 1),
        [0x6A, _, _, _] => (LD8BIG(Reg8(R8::L), Reg8(R8::D)), 1),
        [0x6B, _, _, _] => (LD8BIG(Reg8(R8::L), Reg8(R8::E)), 1),
        [0x6C, _, _, _] => (LD8BIG(Reg8(R8::L), Reg8(R8::H)), 1),
        [0x6D, _, _, _] => (LD8BIG(Reg8(R8::L), Reg8(R8::L)), 1),
        [0x6E, _, _, _] => (LD8BIG(Reg8(R8::L), RegIndirecto8(R16::HL)), 1),
        [0x6F, _, _, _] => (LD8BIG(Reg8(R8::L), Reg8(R8::A)), 1),
        [0x70, _, _, _] => (LD8BIG(RegIndirecto8(R16::HL), Reg8(R8::B)), 1),
        [0x71, _, _, _] => (LD8BIG(RegIndirecto8(R16::HL), Reg8(R8::C)), 1),
        [0x72, _, _, _] => (LD8BIG(RegIndirecto8(R16::HL), Reg8(R8::D)), 1),
        [0x73, _, _, _] => (LD8BIG(RegIndirecto8(R16::HL), Reg8(R8::E)), 1),
        [0x74, _, _, _] => (LD8BIG(RegIndirecto8(R16::HL), Reg8(R8::H)), 1),
        [0x75, _, _, _] => (LD8BIG(RegIndirecto8(R16::HL), Reg8(R8::L)), 1),
        [0x76, _, _, _] => (HALT, 1),
        [0x77, _, _, _] => (LD8BIG(RegIndirecto8(R16::HL), Reg8(R8::A)), 1),
        [0x78, _, _, _] => (LD8BIG(Reg8(R8::A), Reg8(R8::B)), 1),
        [0x79, _, _, _] => (LD8BIG(Reg8(R8::A), Reg8(R8::C)), 1),
        [0x7A, _, _, _] => (LD8BIG(Reg8(R8::A), Reg8(R8::D)), 1),
        [0x7B, _, _, _] => (LD8BIG(Reg8(R8::A), Reg8(R8::E)), 1),
        [0x7C, _, _, _] => (LD8BIG(Reg8(R8::A), Reg8(R8::H)), 1),
        [0x7D, _, _, _] => (LD8BIG(Reg8(R8::A), Reg8(R8::L)), 1),
        [0x7E, _, _, _] => (LD8BIG(Reg8(R8::A), RegIndirecto8(R16::HL)), 1),
        [0x7F, _, _, _] => (LD8BIG(Reg8(R8::A), Reg8(R8::A)), 1),
        [0x80, _, _, _] => (ADD8(Reg8(R8::A), Reg8(R8::B)), 1),
        [0x81, _, _, _] => (ADD8(Reg8(R8::A), Reg8(R8::C)), 1),
        [0x82, _, _, _] => (ADD8(Reg8(R8::A), Reg8(R8::D)), 1),
        [0x83, _, _, _] => (ADD8(Reg8(R8::A), Reg8(R8::E)), 1),
        [0x84, _, _, _] => (ADD8(Reg8(R8::A), Reg8(R8::H)), 1),
        [0x85, _, _, _] => (ADD8(Reg8(R8::A), Reg8(R8::L)), 1),
        [0x86, _, _, _] => (ADD8BIG(Reg8(R8::A), RegIndirecto8(R16::HL)), 1),
        [0x87, _, _, _] => (ADD8(Reg8(R8::A), Reg8(R8::A)), 1),
        [0x88, _, _, _] => (ADC8(Reg8(R8::A), Reg8(R8::B)), 1),
        [0x89, _, _, _] => (ADC8(Reg8(R8::A), Reg8(R8::C)), 1),
        [0x8A, _, _, _] => (ADC8(Reg8(R8::A), Reg8(R8::D)), 1),
        [0x8B, _, _, _] => (ADC8(Reg8(R8::A), Reg8(R8::E)), 1),
        [0x8C, _, _, _] => (ADC8(Reg8(R8::A), Reg8(R8::H)), 1),
        [0x8D, _, _, _] => (ADC8(Reg8(R8::A), Reg8(R8::L)), 1),
        [0x8E, _, _, _] => (ADC8BIG(Reg8(R8::A), RegIndirecto8(R16::HL)), 1),
        [0x8F, _, _, _] => (ADC8(Reg8(R8::A), Reg8(R8::A)), 1),
        [0x90, _, _, _] => (SUB8(Reg8(R8::A), Reg8(R8::B)), 1),
        [0x91, _, _, _] => (SUB8(Reg8(R8::A), Reg8(R8::C)), 1),
        [0x92, _, _, _] => (SUB8(Reg8(R8::A), Reg8(R8::D)), 1),
        [0x93, _, _, _] => (SUB8(Reg8(R8::A), Reg8(R8::E)), 1),
        [0x94, _, _, _] => (SUB8(Reg8(R8::A), Reg8(R8::H)), 1),
        [0x95, _, _, _] => (SUB8(Reg8(R8::A), Reg8(R8::L)), 1),
        [0x96, _, _, _] => (SUB8BIG(Reg8(R8::A), RegIndirecto8(R16::HL)), 1),
        [0x97, _, _, _] => (SUB8(Reg8(R8::A), Reg8(R8::A)), 1),
        [0x98, _, _, _] => (SBC(Reg8(R8::A), Reg8(R8::B)), 1),
        [0x99, _, _, _] => (SBC(Reg8(R8::A), Reg8(R8::C)), 1),
        [0x9A, _, _, _] => (SBC(Reg8(R8::A), Reg8(R8::D)), 1),
        [0x9B, _, _, _] => (SBC(Reg8(R8::A), Reg8(R8::E)), 1),
        [0x9C, _, _, _] => (SBC(Reg8(R8::A), Reg8(R8::H)), 1),
        [0x9D, _, _, _] => (SBC(Reg8(R8::A), Reg8(R8::L)), 1),
        [0x9E, _, _, _] => (SBCBIG(Reg8(R8::A), RegIndirecto8(R16::HL)), 1),
        [0x9F, _, _, _] => (SBC(Reg8(R8::A), Reg8(R8::A)), 1),
        [0xA0, _, _, _] => (AND(Reg8(R8::B)), 1),
        [0xA1, _, _, _] => (AND(Reg8(R8::C)), 1),
        [0xA2, _, _, _] => (AND(Reg8(R8::D)), 1),
        [0xA3, _, _, _] => (AND(Reg8(R8::E)), 1),
        [0xA4, _, _, _] => (AND(Reg8(R8::H)), 1),
        [0xA5, _, _, _] => (AND(Reg8(R8::L)), 1),
        [0xA6, _, _, _] => (ANDBIG(RegIndirecto8(R16::HL)), 1),
        [0xA7, _, _, _] => (AND(Reg8(R8::A)), 1),
        [0xA8, _, _, _] => (XOR(Reg8(R8::B)), 1),
        [0xA9, _, _, _] => (XOR(Reg8(R8::C)), 1),
        [0xAA, _, _, _] => (XOR(Reg8(R8::D)), 1),
        [0xAB, _, _, _] => (XOR(Reg8(R8::E)), 1),
        [0xAC, _, _, _] => (XOR(Reg8(R8::H)), 1),
        [0xAD, _, _, _] => (XOR(Reg8(R8::L)), 1),
        [0xAE, _, _, _] => (XORBIG(RegIndirecto8(R16::HL)), 1),
        [0xAF, _, _, _] => (XOR(Reg8(R8::A)), 1),
        [0xB0, _, _, _] => (OR(Reg8(R8::B)), 1),
        [0xB1, _, _, _] => (OR(Reg8(R8::C)), 1),
        [0xB2, _, _, _] => (OR(Reg8(R8::D)), 1),
        [0xB3, _, _, _] => (OR(Reg8(R8::E)), 1),
        [0xB4, _, _, _] => (OR(Reg8(R8::H)), 1),
        [0xB5, _, _, _] => (OR(Reg8(R8::L)), 1),
        [0xB6, _, _, _] => (ORBIG(RegIndirecto8(R16::HL)), 1),
        [0xB7, _, _, _] => (OR(Reg8(R8::A)), 1),
        [0xB8, _, _, _] => (CP(Reg8(R8::B)), 1),
        [0xB9, _, _, _] => (CP(Reg8(R8::C)), 1),
        [0xBA, _, _, _] => (CP(Reg8(R8::D)), 1),
        [0xBB, _, _, _] => (CP(Reg8(R8::E)), 1),
        [0xBC, _, _, _] => (CP(Reg8(R8::H)), 1),
        [0xBD, _, _, _] => (CP(Reg8(R8::L)), 1),
        [0xBE, _, _, _] => (CPBIG(RegIndirecto8(R16::HL)), 1),
        [0xBF, _, _, _] => (CP(Reg8(R8::A)), 1),
        [0xC0, _, _, _] => (RET(SaltoCondicional::NoCero), 1),
        [0xC1, _, _, _] => (POP(Local16::Reg(R16::BC)), 1),
        [0xC2, n1, n2, _] => {
            let addr = u16::from(n1) | (u16::from(n2) << 8);
            (JP(SaltoCondicional::NoCero, Local16::Inmediato(addr)), 3)
        }
        [0xC3, n1, n2, _] => {
            let addr = u16::from(n1) | (u16::from(n2) << 8);
            (JP(SaltoCondicional::Incondicional, Local16::Inmediato(addr)), 3)
        }
        // call nz,nn
        [0xC4, n1, n2, _] => (CALL(SaltoCondicional::NoCero, u16::from_le_bytes([n1, n2])), 3,),
        // push bc
        [0xC5, _, _, _] => (PUSH(Local16::Reg(R16::BC)), 1),
        [0xC6, n1, _, _] => (ADD8(Reg8(R8::A), Inmediato8(n1)), 2),
        [0xC7, _, _, _] => (RST(Local16::Inmediato(0x00)), 1),
        [0xC8, _, _, _] => (RET(SaltoCondicional::Cero), 1),
        [0xC9, _, _, _] => (RET(SaltoCondicional::Incondicional), 1),
        [0xCA, n1, n2, _] => {
            let addr = u16::from(n1) | (u16::from(n2) << 8);
            (JP(SaltoCondicional::Cero, Local16::Inmediato(addr)), 3)
        }

        [0xCB, op, _, _] => bit::analiza_bits(op), // Bits son todos 0xCB
        [0xCC, n1, n2, _] => (CALL(SaltoCondicional::Cero, u16::from_le_bytes([n1, n2])), 3,),
        [0xCD, n1, n2, _] => (CALL(SaltoCondicional::Incondicional, u16::from_le_bytes([n1, n2])), 3,),
        [0xCE, n1, _, _] => (ADC8(Reg8(R8::A), Inmediato8(n1)), 2),
        [0xCF, _, _, _] => (RST(Local16::Inmediato(0x08)), 1),
        [0xD0, _, _, _] => (RET(SaltoCondicional::NoCarry), 1),
        [0xD1, _, _, _] => (POP(Local16::Reg(R16::DE)), 1),
        [0xD2, n1, n2, _] => {
            let addr = u16::from(n1) | (u16::from(n2) << 8);
            (JP(SaltoCondicional::NoCarry, Local16::Inmediato(addr)), 3)
        }
        [0xD3, n1, _, _] => (OUTBE(Reg8(R8::A), Inmediato8(n1)), 2),
        // call nz,nn
        [0xD4, n1, n2, _] => (CALL(SaltoCondicional::NoCarry, u16::from_le_bytes([n1, n2])), 3,),
        [0xD5, _, _, _] => (PUSH(Local16::Reg(R16::DE)), 1),
        [0xD6, n1, _, _] => (SUB8(Reg8(R8::A), Inmediato8(n1)), 2),
        [0xD7, _, _, _] => (RST(Local16::Inmediato(0x10)), 1),
        [0xD8, _, _, _] => (RET(SaltoCondicional::Carry), 1),
        [0xD9, _, _, _] => (EXX, 1),
        [0xDA, n1, n2, _] => {
            let addr = u16::from(n1) | (u16::from(n2) << 8);
            (JP(SaltoCondicional::Carry, Local16::Inmediato(addr)), 3)
        }
        [0xDB, n1, _, _] => (IN(Reg8(R8::A), Inmediato8(n1)), 2),
        [0xDC, n1, n2, _] => (CALL(SaltoCondicional::Carry, u16::from_le_bytes([n1, n2])), 3,),
        [0xDD, o1, n1, n2] => ix::parse_ix(o1, n1, n2),
        [0xDE, n1, _, _] => (SBC(Reg8(R8::A), Inmediato8(n1)), 2),
        [0xDF, _, _, _] => (RST(Local16::Inmediato(0x18)), 1),
        [0xE0, _, _, _] => (RET(SaltoCondicional::ParidadImpar), 1),
        [0xE1, _, _, _] => (POP(Local16::Reg(R16::HL)), 1),
        [0xE2, n1, n2, _] => {
            let addr = u16::from(n1) | (u16::from(n2) << 8);
            (JP(SaltoCondicional::ParidadImpar, Local16::Inmediato(addr)), 3)
        }
        [0xE3, _, _, _] => (EXSPHL, 1),
        [0xE4, n1, n2, _] => (CALL(SaltoCondicional::ParidadImpar, u16::from_le_bytes([n1, n2])), 3,),
        [0xE5, _, _, _] => (PUSH(Local16::Reg(R16::HL)), 1),
        [0xE6, n1, _, _] => (AND(Inmediato8(n1)), 2),
        [0xE7, _, _, _] => (RST(Local16::Inmediato(0x20)), 1),
        [0xE8, _, _, _] => (RET(SaltoCondicional::ParidadPar), 1),
        [0xE9, _, _, _] => (JPHL(Local16::Reg(R16::HL)), 1),
        [0xEA, n1, n2, _] => {
            let addr = u16::from(n1) | (u16::from(n2) << 8);
            (JP(SaltoCondicional::ParidadPar, Local16::Inmediato(addr)), 3)
        }
        [0xEB, _, _, _] => (EX, 1),
        [0xEC, n1, n2, _] => (CALL(SaltoCondicional::ParidadPar, u16::from_le_bytes([n1, n2])), 3,),
        [0xED, op, n1, n2] => analiza(op, n1, n2),
        [0xEE, n1, _, _] => (XOR(Inmediato8(n1)), 2),
        [0xEF, _, _, _] => (RST(Local16::Inmediato(0x28)), 1),
        [0xF0, _, _, _] => (RET(SaltoCondicional::SignoPositivo), 1),
        [0xF1, _, _, _] => (POP(Local16::Reg(R16::AF)), 1),
        [0xF2, n1, n2, _] => {
            let addr = u16::from(n1) | (u16::from(n2) << 8);
            (JP(SaltoCondicional::SignoPositivo, Local16::Inmediato(addr)), 3)
        }
        [0xF3, _, _, _] => (DI, 1),
        [0xF4, n1, n2, _] => (CALL(SaltoCondicional::SignoPositivo, u16::from_le_bytes([n1, n2])), 3,),
        [0xF5, _, _, _] => (PUSH(Local16::Reg(R16::AF)), 1),
        [0xF6, n1, _, _] => (OR(Inmediato8(n1)), 2),
        [0xF7, _, _, _] => (RST(Local16::Inmediato(0x30)), 1),
        [0xF8, _, _, _] => (RET(SaltoCondicional::SignoNegativo), 1),
        [0xF9, _, _, _] => (LD16BIG(Local16::Reg(R16::SP), Local16::Reg(R16::HL)), 1),
        [0xFA, n1, n2, _] => {
            let addr = u16::from(n1) | (u16::from(n2) << 8);
            (JP(SaltoCondicional::SignoNegativo, Local16::Inmediato(addr)), 3)
        }
        [0xFB, _, _, _] => (EI, 1),
        [0xFC, n1, n2, _] => (CALL(SaltoCondicional::SignoNegativo, u16::from_le_bytes([n1, n2])), 3,),
        // ---------------------------------------------------------------
        // Decodificación de instrucciones IY con prefijo FD (0xFD)
        // ---------------------------------------------------------------
        [0xFD, op, n1, n2] => iy::parse_iy(op, n1, n2),

        [0xFE, n1, _, _] => (CP(Inmediato8(n1)), 2),
        [0xFF, _, _, _] => (RST(Local16::Inmediato(0x38)), 1),
    }
}

// // Funciones helper para grupos de opcodes
// fn handle_ld_r_r(op: u8) -> (Op, usize) {
//     let dest = Reg8::from((op >> 3) & 0x07);
//     let src = Reg8::from(op & 0x07);
//     (LD(dest.into(), src.into()), 1)
// }
//
// fn handle_alu_op(op: u8) -> (Op, usize) {
//     match op {
//         0x80..=0x87 => (ADD(Reg8::A, Reg8::from(op - 0x80)), 1),
//         0x88..=0x8F => (ADC(Reg8::A, Reg8::from(op - 0x88)), 1),
//         0x90..=0x97 => (SUB(Reg8::from(op - 0x90)), 1),
//         0x98..=0x9F => (SBC(Reg8::A, Reg8::from(op - 0x98)), 1),
//         0xA0..=0xA7 => (AND(Reg8::from(op - 0xA0)), 1),
//         0xA8..=0xAF => (XOR(Reg8::from(op - 0xA8)), 1),
//         0xB0..=0xB7 => (OR(Reg8::from(op - 0xB0)), 1),
//         0xB8..=0xBF => (CP(Reg8::from(op - 0xB8)), 1),
//         _ => (UNKNOWN, 1)
//     }
// }
//
// fn handle_control_flow(op: u8, b1: u8, b2: u8) -> (Op, usize) {
//     match op {
//         0xC0 => (RET(NZ), 1),
//         0xC1 => (POP(Reg16::BC), 1),
//         0xC2 => (JP(NZ, (b2 as u16) << 8 | b1 as u16), 3),
//         0xC3 => (JP((b2 as u16) << 8 | b1 as u16), 3),
//         0xC4 => (CALL(NZ, (b2 as u16) << 8 | b1 as u16), 3),
//         0xC5 => (PUSH(Reg16::BC), 1),
//         0xC6 => (ADD(Reg8::A, Localizacion8::Inmediato(b1)), 2),
//         0xC7 => (RST(0x00), 1),
//         // ... completar resto de 0xC8-0xFF
//         _ => (UNKNOWN, 1)
//     }
// }