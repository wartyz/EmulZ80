// Instrucciones IX  (DD)

use crate::cpu::opcodes::ix_bit::analiza_ix_bit;
use crate::cpu::opcodes::util::*;
use crate::ops::Op::{INC16, JPHL};
use crate::ops::{Local16, Local8, Op, R16, R8};

pub fn parse_ix(op: u8, n1: u8, n2: u8) -> (Op, usize) {
    //dbg_hex!(op,n2,n3);
    match op {
        0x00 => panic!("instruccion DD {:0X} no existe", op),
        0x01 => panic!("instruccion DD {:0X} no existe", op),
        0x02 => panic!("instruccion DD {:0X} no existe", op),
        0x03 => panic!("instruccion DD {:0X} no existe", op),
        0x04 => (Op::INC8(Local8::Reg8(R8::B)), 2),  // INC B
        0x05 => (Op::DEC8(Local8::Reg8(R8::B)), 2),  // DEC B
        0x06 => (Op::LD8BIG(Local8::Reg8(R8::B), Local8::Inmediato8(n1)), 3), // LD B,n
        0x07 => panic!("instruccion DD {:0X} no existe", op),
        0x08 => panic!("instruccion DD {:0X} no existe", op),
        0x09 => (Op::ADD16BIG(Local16::Reg(R16::IX), Local16::Reg(R16::BC)), 2), // ADD IX,BC
        0x0A => panic!("instruccion DD {:0X} no existe", op),
        0x0B => panic!("instruccion DD {:0X} no existe", op),
        0x0C => (Op::INC8(Local8::Reg8(R8::C)), 2),  // INC C
        0x0D => (Op::DEC8(Local8::Reg8(R8::C)), 2),  // DEC IXL
        0x0E => (Op::LD8BIG(Local8::Reg8(R8::C), Local8::Inmediato8(n1)), 3), // LD IXL,n
        0x0F => panic!("instruccion DD {:0X} no existe", op),
        0x10 => panic!("instruccion DD {:0X} no existe", op),
        0x11 => panic!("instruccion DD {:0X} no existe", op),
        0x12 => panic!("instruccion DD {:0X} no existe", op),
        0x13 => panic!("instruccion DD {:0X} no existe", op),
        0x14 => (Op::INC8(Local8::Reg8(R8::D)), 2),    // INC D
        0x15 => (Op::DEC8(Local8::Reg8(R8::D)), 2),    // DEC D
        0x16 => (Op::LD8BIG(Local8::Reg8(R8::D), Local8::Inmediato8(n1)), 2), // LD D,n
        0x17 => panic!("instruccion DD {:0X} no existe", op),
        0x18 => panic!("instruccion DD {:0X} no existe", op),
        0x19 => (Op::ADD16BIG(Local16::Reg(R16::IX), Local16::Reg(R16::DE)), 2), // ADD IX,DE
        0x1A => panic!("instruccion DD {:0X} no existe", op),
        0x1B => panic!("instruccion DD {:0X} no existe", op),
        0x1C => (Op::INC8(Local8::Reg8(R8::E)), 2),    // INC E
        0x1D => (Op::DEC8(Local8::Reg8(R8::E)), 2),    // DEC E
        0x1E => (Op::LD8BIG(Local8::Reg8(R8::E), Local8::Inmediato8(n1)), 2), // LD L,n
        0x1F => panic!("instruccion DD {:0X} no existe", op),
        0x20 => panic!("instruccion DD {:0X} no existe", op),
        // 0x21 => (LD16(Local16::Reg(reg), le_immediate(n2, op)), 4),
        0x21 => (Op::LD16BIG(Local16::Reg(R16::IX), le_immediate(n1, n2)), 4),
        //0x22 => (Op::LD16(le_imm_indir_le(n2, n3), Local16::Reg(reg)), 4),
        0x22 => (Op::LD16BIG(le_imm_indir_be(n2, n1), Local16::Reg(R16::IX)), 4),
        0x23 => (INC16(Local16::Reg(R16::IX), Local16::Reg(R16::IX)), 2),
        0x24 => (Op::INC8(Local8::Reg8(R8::IXH)), 2), // INC IXH segun deepseek (No estándar) hay que probarlo
        0x25 => (Op::DEC8(Local8::Reg8(R8::IXH)), 2), // DEC IXH (No estándar)
        0x26 => (Op::LD8BIG(Local8::Reg8(R8::IXH), Local8::Inmediato8(n1)), 3), // LD IXH,n (No estándar)
        0x27 => panic!("instruccion DD {:0X} no existe", op),
        0x28 => panic!("instruccion DD {:0X} no existe", op),
        0x29 => (Op::ADD16BIG(Local16::Reg(R16::IX), Local16::Reg(R16::IX)), 2), // ADD IX,IX
        //0x2A => (Op::LD16(Local16::Reg(reg), le_imm_indir_le(n2, n3)), 4),
        0x2A => (Op::LD16BIG(Local16::Reg(R16::IX), le_imm_indir_le(n1, n2)), 4),
        0x2B => (Op::DEC16(Local16::Reg(R16::IX), Local16::Reg(R16::IX)), 2),  // DEC IX
        0x2C => (Op::INC8(Local8::Reg8(R8::IXL)), 2), // INC IXL (No estándar)
        0x2D => (Op::DEC8(Local8::Reg8(R8::IXL)), 2), // DEC IXL (No estándar)
        0x2E => (Op::LD8BIG(Local8::Reg8(R8::IXL), Local8::Inmediato8(n1)), 3), // LD IXL,n (No estándar)
        0x2F => panic!("instruccion DD {:0X} no existe", op),
        0x30 => panic!("instruccion DD {:0X} no existe", op),
        0x31 => panic!("instruccion DD {:0X} no existe", op),
        0x32 => panic!("instruccion DD {:0X} no existe", op),
        0x33 => panic!("instruccion DD {:0X} no existe", op),
        0x34 => (Op::INC8(Local8::Indexado8(R16::IX, n1 as i8)), 3), // INC (IX+d)
        0x35 => (Op::DEC8(Local8::Indexado8(R16::IX, n1 as i8)), 3), // DEC (IX+d)
        0x36 => (Op::LD8BIG(Local8::Indexado8(R16::IX, n1 as i8), Local8::Inmediato8(n2)), 4),     // LD (IX+d),n
        0x37 => panic!("instruccion DD {:0X} no existe", op),
        0x38 => panic!("instruccion DD {:0X} no existe", op),
        0x39 => (Op::ADD16BIG(Local16::Reg(R16::IX), Local16::Reg(R16::SP)), 2), // ADD IX,SP
        0x3A => panic!("instruccion DD {:0X} no existe", op),
        0x3B => panic!("instruccion DD {:0X} no existe", op),
        0x3C => panic!("instruccion DD {:0X} no implementada", op),
        0x3D => panic!("instruccion DD {:0X} no implementada", op),
        0x3E => panic!("instruccion DD {:0X} no implementada", op),
        0x3F => panic!("instruccion DD {:0X} no existe", op),
        0x40 => panic!("instruccion DD {:0X} no implementada", op),
        0x41 => panic!("instruccion DD {:0X} no implementada", op),
        0x42 => panic!("instruccion DD {:0X} no implementada", op),
        0x43 => panic!("instruccion DD {:0X} no implementada", op),
        0x44 => (Op::LD8BIG(Local8::Reg8(R8::B), Local8::Reg8(R8::IXH)), 2), // LD B,IXH (No estándar)
        0x45 => (Op::LD8BIG(Local8::Reg8(R8::B), Local8::Reg8(R8::IXL)), 2), // LD B,IXL (No estándar)
        0x46 => (Op::LD8BIG(Local8::Reg8(R8::B), Local8::Indexado8(R16::IX, n1 as i8)), 3),  // LD B,(IX+d)
        0x47 => panic!("instruccion DD {:0X} no implementada", op),
        0x48 => panic!("instruccion DD {:0X} no implementada", op),
        0x49 => panic!("instruccion DD {:0X} no implementada", op),
        0x4A => panic!("instruccion DD {:0X} no implementada", op),
        0x4B => panic!("instruccion DD {:0X} no implementada", op),
        0x4C => (Op::LD8BIG(Local8::Reg8(R8::C), Local8::Reg8(R8::IXH)), 2), // LD C,IXH (No estándar)
        0x4D => (Op::LD8BIG(Local8::Reg8(R8::C), Local8::Reg8(R8::IXL)), 2), // LD C,IXL (No estándar)
        0x4E => (Op::LD8BIG(Local8::Reg8(R8::C), Local8::Indexado8(R16::IX, n1 as i8)), 3),  // LD C,(IX+d)
        0x4F => panic!("instruccion DD {:0X} no implementada", op),
        0x50 => panic!("instruccion DD {:0X} no implementada", op),
        0x51 => panic!("instruccion DD {:0X} no implementada", op),
        0x52 => panic!("instruccion DD {:0X} no implementada", op),
        0x53 => panic!("instruccion DD {:0X} no implementada", op),
        0x54 => (Op::LD8BIG(Local8::Reg8(R8::D), Local8::Reg8(R8::IXH)), 2), // LD D,IXH (No estándar)
        0x55 => (Op::LD8BIG(Local8::Reg8(R8::D), Local8::Reg8(R8::IXL)), 2), // LD D,IXL (No estándar)
        0x56 => (Op::LD8BIG(Local8::Reg8(R8::D), Local8::Indexado8(R16::IX, n1 as i8)), 3),  // LD D,(IX+d)
        0x57 => panic!("instruccion DD {:0X} no implementada", op),
        0x58 => panic!("instruccion DD {:0X} no implementada", op),
        0x59 => panic!("instruccion DD {:0X} no implementada", op),
        0x5A => panic!("instruccion DD {:0X} no implementada", op),
        0x5B => panic!("instruccion DD {:0X} no implementada", op),
        0x5C => (Op::LD8BIG(Local8::Reg8(R8::E), Local8::Reg8(R8::IXH)), 2), // LD E,IXH (No estándar)
        0x5D => (Op::LD8BIG(Local8::Reg8(R8::E), Local8::Reg8(R8::IXL)), 2), // LD E,IXL (No estándar)
        0x5E => (Op::LD8BIG(Local8::Reg8(R8::E), Local8::Indexado8(R16::IX, n1 as i8)), 3),  // LD E,(IX+d)
        0x5F => panic!("instruccion DD {:0X} no implementada", op),
        0x60 => (Op::LD8BIG(Local8::Reg8(R8::IXH), Local8::Reg8(R8::B)), 2), // LD IXH,B (No estándar)
        0x61 => (Op::LD8BIG(Local8::Reg8(R8::IXH), Local8::Reg8(R8::C)), 2), // LD IXH,C (No estándar)
        0x62 => (Op::LD8BIG(Local8::Reg8(R8::IXH), Local8::Reg8(R8::D)), 2), // LD IXH,D (No estándar)
        0x63 => (Op::LD8BIG(Local8::Reg8(R8::IXH), Local8::Reg8(R8::E)), 2), // LD IXH,E (No estándar)
        0x64 => (Op::NOP, 2), // LD IXH,IXH (No estándar - NOP)
        0x65 => (Op::LD8BIG(Local8::Reg8(R8::IXH), Local8::Reg8(R8::IXL)), 2), // LD IXH,IXL (No estándar)
        0x66 => (Op::LD8BIG(Local8::Reg8(R8::H), Local8::Indexado8(R16::IX, n1 as i8)), 3),  // LD H,(IX+d)
        0x67 => (Op::LD8BIG(Local8::Reg8(R8::IXH), Local8::Reg8(R8::A)), 2), // LD IXH,A (No estándar)
        0x68 => (Op::LD8BIG(Local8::Reg8(R8::IXL), Local8::Reg8(R8::B)), 2), // LD IXL,B (No estándar)
        0x69 => (Op::LD8BIG(Local8::Reg8(R8::IXL), Local8::Reg8(R8::C)), 2), // LD IXL,C (No estándar)
        0x6A => (Op::LD8BIG(Local8::Reg8(R8::IXL), Local8::Reg8(R8::D)), 2), // LD IXL,D (No estándar)
        0x6B => (Op::LD8BIG(Local8::Reg8(R8::IXL), Local8::Reg8(R8::E)), 2), // LD IXL,E (No estándar)
        0x6C => (Op::LD8BIG(Local8::Reg8(R8::IXL), Local8::Reg8(R8::IXH)), 2), // LD IXL,IXH (No estándar)
        0x6D => (Op::NOP, 2), // LD IXL,IXL (No estándar - NOP)
        0x6E => (Op::LD8BIG(Local8::Reg8(R8::L), Local8::Indexado8(R16::IX, n1 as i8)), 3),  // LD L,(IX+d)
        0x6F => (Op::LD8BIG(Local8::Reg8(R8::IXL), Local8::Reg8(R8::A)), 2), // LD IXL,A (No estándar)

        // Instrucciones de almacenamiento en (IX+d)
        0x70..=0x75 | 0x77 => {  // LD (IX+d),r
            let d = n1 as i8;
            let r = match op {
                0x70 => R8::B,
                0x71 => R8::C,
                0x72 => R8::D,
                0x73 => R8::E,
                0x74 => R8::H,
                0x75 => R8::L,
                0x77 => R8::A,
                _ => unreachable!(),
            };
            (Op::LD8BIG(Local8::Indexado8(R16::IX, d), Local8::Reg8(r)), 3)
        }
        0x76 => panic!("instruccion DD {:0X} no existe", op),
        0x78 => panic!("instruccion DD {:0X} no implementada", op),
        0x79 => panic!("instruccion DD {:0X} no implementada", op),
        0x7A => panic!("instruccion DD {:0X} no implementada", op),
        0x7B => panic!("instruccion DD {:0X} no implementada", op),
        0x7C => panic!("instruccion DD {:0X} no implementada", op),
        0x7D => panic!("instruccion DD {:0X} no implementada", op),
        0x7E => (Op::LD8BIG(Local8::Reg8(R8::A), Local8::Indexado8(R16::IX, n1 as i8)), 3),  // LD A,(IX+d)
        0x7F => panic!("instruccion DD {:0X} no implementada", op),
        0x80 => panic!("instruccion DD {:0X} no implementada", op),
        0x81 => panic!("instruccion DD {:0X} no implementada", op),
        0x82 => panic!("instruccion DD {:0X} no implementada", op),
        0x83 => panic!("instruccion DD {:0X} no implementada", op),
        0x84 => (Op::ADD8(Local8::Reg8(R8::A), Local8::Reg8(R8::IXH)), 2), // ADD A,IXH (No estándar)
        0x85 => (Op::ADD8(Local8::Reg8(R8::A), Local8::Reg8(R8::IXL)), 2), // ADD A,IXL (No estándar)
        0x86 => (Op::ADD8(Local8::Reg8(R8::A), Local8::Indexado8(R16::IX, n1 as i8)), 3), // ADD A,(IX+d)
        0x87 => panic!("instruccion DD {:0X} no implementada", op),
        0x88 => panic!("instruccion DD {:0X} no implementada", op),
        0x89 => panic!("instruccion DD {:0X} no implementada", op),
        0x8A => panic!("instruccion DD {:0X} no implementada", op),
        0x8B => panic!("instruccion DD {:0X} no implementada", op),
        0x8C => (Op::ADC8(Local8::Reg8(R8::A), Local8::Reg8(R8::IXH)), 2), // ADC A,IXH (No estándar)
        0x8D => (Op::ADC8(Local8::Reg8(R8::A), Local8::Reg8(R8::IXL)), 2), // ADC A,IXL (No estándar)
        0x8E => (Op::ADC8(Local8::Reg8(R8::A), Local8::Indexado8(R16::IX, n1 as i8)), 3), // ADC A,(IX+d)
        0x8F => panic!("instruccion DD {:0X} no implementada", op),
        0x90 => panic!("instruccion DD {:0X} no implementada", op),
        0x91 => panic!("instruccion DD {:0X} no implementada", op),
        0x92 => panic!("instruccion DD {:0X} no implementada", op),
        0x93 => panic!("instruccion DD {:0X} no implementada", op),
        0x94 => (Op::SUB8(Local8::Reg8(R8::A), Local8::Reg8(R8::IXH)), 2), // SUB IXH (No estándar)
        0x95 => (Op::SUB8(Local8::Reg8(R8::A), Local8::Reg8(R8::IXL)), 2), // SUB IXL (No estándar)
        0x96 => (Op::SUB8(Local8::Reg8(R8::A), Local8::Indexado8(R16::IX, n1 as i8)), 3), // SUB (IX+d)
        0x97 => panic!("instruccion DD {:0X} no implementada", op),
        0x98 => panic!("instruccion DD {:0X} no implementada", op),
        0x99 => panic!("instruccion DD {:0X} no implementada", op),
        0x9A => panic!("instruccion DD {:0X} no implementada", op),
        0x9B => panic!("instruccion DD {:0X} no implementada", op),
        0x9C => (Op::SBC(Local8::Reg8(R8::A), Local8::Reg8(R8::IXH)), 2), // SBC A,IXH (No estándar)
        0x9D => (Op::SBC(Local8::Reg8(R8::A), Local8::Reg8(R8::IXL)), 2), // SBC A,IXL (No estándar)
        0x9E => (Op::SBC(Local8::Reg8(R8::A), Local8::Indexado8(R16::IX, n1 as i8)), 3),  // SBC A,(IX+d)
        0x9F => panic!("instruccion DD {:0X} no implementada", op),
        0xA0 => panic!("instruccion DD {:0X} no implementada", op),
        0xA1 => panic!("instruccion DD {:0X} no implementada", op),
        0xA2 => panic!("instruccion DD {:0X} no implementada", op),
        0xA3 => panic!("instruccion DD {:0X} no implementada", op),
        0xA4 => (Op::AND(Local8::Reg8(R8::IXH)), 2), // AND IXH (No estándar)
        0xA5 => (Op::AND(Local8::Reg8(R8::IXL)), 2), // AND IXL (No estándar)
        0xA6 => (Op::AND(Local8::Indexado8(R16::IX, n1 as i8)), 3), // AND (IX+d)
        0xA7 => panic!("instruccion DD {:0X} no implementada", op),
        0xA8 => panic!("instruccion DD {:0X} no implementada", op),
        0xA9 => panic!("instruccion DD {:0X} no implementada", op),
        0xAA => panic!("instruccion DD {:0X} no implementada", op),
        0xAB => panic!("instruccion DD {:0X} no implementada", op),
        0xAC => (Op::XOR(Local8::Reg8(R8::IXH)), 2), // XOR IXH (No estándar)
        0xAD => (Op::XOR(Local8::Reg8(R8::IXL)), 2), // XOR IXL (No estándar)
        0xAE => (Op::XOR(Local8::Indexado8(R16::IX, n1 as i8)), 3), // XOR (IX+d)
        0xAF => panic!("instruccion DD {:0X} no implementada", op),
        0xB0 => panic!("instruccion DD {:0X} no implementada", op),
        0xB1 => panic!("instruccion DD {:0X} no implementada", op),
        0xB2 => panic!("instruccion DD {:0X} no implementada", op),
        0xB3 => panic!("instruccion DD {:0X} no implementada", op),
        0xB4 => (Op::OR(Local8::Reg8(R8::IXH)), 2), // OR IXH (No estándar)
        0xB5 => (Op::OR(Local8::Reg8(R8::IXL)), 2), // OR IXL (No estándar)
        0xB6 => (Op::OR(Local8::Indexado8(R16::IX, n1 as i8)), 3), // OR (IX+d)
        0xB7 => panic!("instruccion DD {:0X} no implementada", op),
        0xB8 => (Op::CP(Local8::Reg8(R8::B)), 2), // CP B (No estándar)
        0xB9 => (Op::CP(Local8::Reg8(R8::C)), 2), // CP IXL (No estándar)
        0xBA => (Op::CP(Local8::Reg8(R8::D)), 2), // CP IXL (No estándar)
        0xBB => (Op::CP(Local8::Reg8(R8::E)), 2), // CP IXL (No estándar)
        0xBC => (Op::CP(Local8::Reg8(R8::IXH)), 2), // CP IXH (No estándar)
        0xBD => (Op::CP(Local8::Reg8(R8::IXL)), 2), // CP IXL (No estándar)
        0xBE => (Op::CP(Local8::Indexado8(R16::IX, n1 as i8)), 3), // CP (IX+d)
        0xBF => panic!("instruccion DD {:0X} no implementada", op),

        0xC0..=0xCA => panic!("instruccion DD {:0X} no existe", op),
        // 0xCB  analiza_ix_bit(_: u8, n2: u8, n3: u8) -> (Op, usize)
        0xCB => analiza_ix_bit(op, n1, n2),
        0xCC..=0xCF => panic!("instruccion DD {:0X} no existe", op),
        0xD0..=0xDF => panic!("instruccion DD {:0X} no existe", op),
        0xE0 => panic!("instruccion DD {:0X} no existe", op),
        0xE1 => (Op::POP(Local16::Reg(R16::IX)), 2),
        0xE2 => panic!("instruccion DD {:0X} no existe", op),
        0xE3 => (Op::EXSPIX, 2),  // EX (SP),IX
        0xE4 => panic!("instruccion DD {:0X} no existe", op),
        0xE5 => (Op::PUSH(Local16::Reg(R16::IX)), 2),
        0xE6 => panic!("instruccion DD {:0X} no existe", op),
        0xE7 => panic!("instruccion DD {:0X} no existe", op),
        0xE8 => panic!("instruccion DD {:0X} no existe", op),
        //0xE9 => panic!("instruccion DD {:0X} no implementada", op),
        0xE9 => (JPHL(Local16::Reg(R16::IX)), 2),
        0xEA..=0xEF => panic!("instruccion DD {:0X} no existe", op),
        0xF0..=0xF8 => panic!("instruccion DD {:0X} no existe", op),
        //         0xE9 => (Op::JPHL(Local16::Reg(reg)), 2),  // JP (IX)
        //0xF9 => (Op::LD16(Local16::Reg(R16::SP), Local16::Reg(reg)), 2,),
        0xF9 => (Op::LD16BIG(Local16::Reg(R16::SP), Local16::Reg(R16::IX)), 2,),
        0xFA..=0xFF => panic!("instruccion DD {:0X} no existe", op),
        _op => unimplemented!("Error extraño IX no implementado, no porque {:?} {:02x}", R16::IX, op),
    }
}
