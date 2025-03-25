use crate::cpu::opcodes::iy_bit::analiza_iy_bit;
use crate::cpu::opcodes::util::{le_imm_indir_be, le_imm_indir_le, le_immediate};
use crate::ops::Local8::*;
use crate::ops::Op::*;
use crate::ops::*;

// instrucciones FD    Registro IY
pub fn parse_iy(op: u8, n1: u8, n2: u8) -> (Op, usize) {
    match op {
        0x00..=0x03 => panic!("instruccion FD {:02X} no existe", op),
        0x04 => (INC8(Reg8(R8::B)), 2),
        0x05 => (DEC8(Reg8(R8::B)), 2),
        0x06 => (LD8BIG(Reg8(R8::B), Inmediato8(n1)), 3),
        0x07 => panic!("instruccion FD {:02X} no existe", op),
        0x08 => panic!("instruccion FD {:02X} no existe", op),
        0x09 => (ADD16BIG(Local16::Reg(R16::IY), Local16::Reg(R16::BC)), 2),
        0x0A => panic!("instruccion FD {:02X}  no existe", op),
        0x0B => panic!("instruccion FD {:02X}  no existe", op),
        0x0C => panic!("instruccion FD {:02X}  no implementada", op),
        0x0D => panic!("instruccion FD {:02X}  no implementada", op),
        0x0E => panic!("instruccion FD {:02X}  no implementada", op),
        0x0F..=0x13 => panic!("instruccion FD {:02X}  no existe", op),
        0x14 => (INC8(Reg8(R8::D)), 2),
        0x15 => panic!("instruccion FD {:02X}  no implementada", op),
        0x16 => panic!("instruccion FD {:02X}  no implementada", op),
        0x17 => panic!("instruccion FD {:02X}  no existe", op),
        0x18 => panic!("instruccion FD {:02X}  no existe", op),
        0x19 => (ADD16BIG(Local16::Reg(R16::IY), Local16::Reg(R16::DE)), 2),
        0x1A => panic!("instruccion FD {:02X}  no existe", op),
        0x1B => panic!("instruccion FD {:02X}  no existe", op),
        0x1C => panic!("instruccion FD {:02X}  no implementada", op),
        0x1D => panic!("instruccion FD {:02X}  no implementada", op),
        0x1E => panic!("instruccion FD {:02X}  no implementada", op),
        0x1F => panic!("instruccion FD {:02X}  no existe", op),
        0x20 => panic!("instruccion FD {:0X}  no existe", op),
        //0x21 => (LD16(Local16::Reg(reg), le_immediate(n2, n3)), 4),
        0x21 => (LD16BIG(Local16::Reg(R16::IY), le_immediate(n1, n2)), 4),
        //0x22 => (LD16(le_imm_indir_le(n2, op), Local16::Reg(reg)), 4),
        0x22 => {
            (LD16BIG(le_imm_indir_be(n2, n1), Local16::Reg(R16::IY)), 4)
        }
        0x23 => (INC16(Local16::Reg(R16::IY), Local16::Reg(R16::IY)), 2),  // INC IY
        0x24 => panic!("instruccion FD {:02X}  no implementada", op),
        0x25 => panic!("instruccion FD {:02X}  no implementada", op),
        0x26 => panic!("instruccion FD {:02X}  no implementada", op),
        0x27 => panic!("instruccion FD {:02X}  no existe", op),
        0x28 => panic!("instruccion FD {:02X}  no existe", op),
        0x29 => (ADD16BIG(Local16::Reg(R16::IY), Local16::Reg(R16::IY)), 2), // ADD IY, IY
        //0x2A => (LD16(Local16::Reg(reg), le_imm_indir_le(n1, n2)), 4),
        0x2A => (LD16BIG(Local16::Reg(R16::IY), le_imm_indir_le(n1, n2)), 4),
        0x2B => (DEC16(Local16::Reg(R16::IY), Local16::Reg(R16::IY)), 2),  // DEC IY
        0x2C => panic!("instruccion FD {:02X}  no implementada", op),
        0x2D => panic!("instruccion FD {:02X}  no implementada", op),
        0x2E => panic!("instruccion FD {:02X}  no implementada", op),
        0x2F => panic!("instruccion FD {:02X}  no existe", op),
        0x30..=0x33 => panic!("instruccion FD {:02X}  no existe", op),
        0x34 => (INC8(Indexado8(R16::IY, n1 as i8)), 3), // INC (IY+d)
        0x35 => (DEC8(Indexado8(R16::IY, n1 as i8)), 3), // DEC (IY+d) - Opcode: FD 35 d
        0x36 => (LD8BIG(Indexado8(R16::IY, n1 as i8), Inmediato8(n2)), 4), // LD (IY+d), n
        0x37 => panic!("instruccion FD {:02X}  no existe", op),
        0x38 => panic!("instruccion FD {:02X}  no existe", op),
        0x39 => (ADD16BIG(Local16::Reg(R16::IY), Local16::Reg(R16::SP)), 2), // ADD IY, SP
        0x3A => panic!("instruccion FD {:02X}  no existe", op),
        0x3B => panic!("instruccion FD {:02X}  no existe", op),
        0x3C => panic!("instruccion FD {:02X}  no implementada", op),
        0x3D => panic!("instruccion FD {:02X}  no implementada", op),
        0x3E => panic!("instruccion FD {:02X}  no implementada", op),
        0x3F => panic!("instruccion FD {:02X}  no existe", op),
        0x40 => panic!("instruccion FD {:02X}  no implementada", op),
        0x41 => panic!("instruccion FD {:02X}  no implementada", op),
        0x42 => panic!("instruccion FD {:02X}  no implementada", op),
        0x43 => panic!("instruccion FD {:02X}  no implementada", op),
        0x44 => panic!("instruccion FD {:02X}  no implementada", op),
        0x45 => panic!("instruccion FD {:02X}  no implementada", op),
        0x46 => (LD8BIG(Reg8(R8::B), Indexado8(R16::IY, n1 as i8)), 3), // LD B, (IY+d)
        0x47 => panic!("instruccion FD {:02X}  no implementada", op),
        0x48 => panic!("instruccion FD {:02X}  no implementada", op),
        0x49 => panic!("instruccion FD {:02X}  no implementada", op),
        0x4A => panic!("instruccion FD {:02X}  no implementada", op),
        0x4B => panic!("instruccion FD {:02X}  no implementada", op),
        0x4C => panic!("instruccion FD {:02X}  no implementada", op),
        0x4D => panic!("instruccion FD {:02X}  no implementada", op),
        0x4E => (LD8BIG(Reg8(R8::C), Indexado8(R16::IY, n1 as i8)), 3), // LD C, (IY+d)
        0x4F => panic!("instruccion FD {:02X}  no implementada", op),
        0x50 => panic!("instruccion FD {:02X}  no implementada", op),
        0x51 => panic!("instruccion FD {:02X}  no implementada", op),
        0x52 => panic!("instruccion FD {:02X}  no implementada", op),
        0x53 => panic!("instruccion FD {:02X}  no implementada", op),
        0x54 => panic!("instruccion FD {:02X}  no implementada", op),
        0x55 => panic!("instruccion FD {:02X}  no implementada", op),
        0x56 => (LD8BIG(Reg8(R8::D), Indexado8(R16::IY, n1 as i8)), 3), // LD D, (IY+d)
        0x57 => panic!("instruccion FD {:02X}  no implementada", op),
        0x58 => panic!("instruccion FD {:02X}  no implementada", op),
        0x59 => panic!("instruccion FD {:02X}  no implementada", op),
        0x5A => panic!("instruccion FD {:02X}  no implementada", op),
        0x5B => panic!("instruccion FD {:02X}  no implementada", op),
        0x5C => panic!("instruccion FD {:02X}  no implementada", op),
        0x5D => panic!("instruccion FD {:02X}  no implementada", op),
        0x5E => (LD8BIG(Reg8(R8::E), Indexado8(R16::IY, n1 as i8)), 3), // LD E, (IY+d)
        0x5F => panic!("instruccion FD {:02X}  no implementada", op),
        0x60 => panic!("instruccion FD {:02X}  no implementada", op),
        0x61 => panic!("instruccion FD {:02X}  no implementada", op),
        0x62 => panic!("instruccion FD {:02X}  no implementada", op),
        0x63 => panic!("instruccion FD {:02X}  no implementada", op),
        0x64 => panic!("instruccion FD {:02X}  no implementada", op),
        0x65 => panic!("instruccion FD {:02X}  no implementada", op),
        0x66 => (LD8BIG(Reg8(R8::H), Indexado8(R16::IY, n1 as i8)), 3), // LD H, (IY+d)
        0x67 => panic!("instruccion FD {:02X}  no implementada", op),
        0x68 => panic!("instruccion FD {:02X}  no implementada", op),
        0x69 => panic!("instruccion FD {:02X}  no implementada", op),
        0x6A => panic!("instruccion FD {:02X}  no implementada", op),
        0x6B => panic!("instruccion FD {:02X}  no implementada", op),
        0x6C => panic!("instruccion FD {:02X}  no implementada", op),
        0x6D => panic!("instruccion FD {:02X}  no implementada", op),
        0x6E => (LD8BIG(Reg8(R8::L), Indexado8(R16::IY, n1 as i8)), 3), // LD L, (IY+d)
        0x6F => panic!("instruccion FD {:02X}  no implementada", op),
        // LD (IY+d), r - Opcode FD 70+d r
        0x70..=0x75 | 0x77 => { // Rango de opcodes para LD (IY+d), r
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
            let loc = Indexado8(R16::IY, d);
            (LD8BIG(loc, Reg8(r)), 3)
        }
        0x76 => panic!("instruccion FD {:02X}  no existe", op),
        0x78 => panic!("instruccion FD {:02X}  no implementada", op),
        0x79 => panic!("instruccion FD {:02X}  no implementada", op),
        0x7A => panic!("instruccion FD {:02X}  no implementada", op),
        0x7B => panic!("instruccion FD {:02X}  no implementada", op),
        0x7C => panic!("instruccion FD {:02X}  no implementada", op),
        0x7D => panic!("instruccion FD {:02X}  no implementada", op),
        0x7E => (LD8BIG(Reg8(R8::A), Indexado8(R16::IY, n1 as i8)), 3), // LD A, (IY+d)
        0x7F => panic!("instruccion FD {:02X}  no implementada", op),
        0x80 => panic!("instruccion FD {:02X}  no implementada", op),
        0x81 => panic!("instruccion FD {:02X}  no implementada", op),
        0x82 => panic!("instruccion FD {:02X}  no implementada", op),
        0x83 => panic!("instruccion FD {:02X}  no implementada", op),
        0x84 => (ADD8(Reg8(R8::A), Reg8(R8::IYH)), 2),
        0x85 => (ADD8(Reg8(R8::A), Reg8(R8::IYL)), 2),
        0x86 => (ADD8(Reg8(R8::A), Indexado8(R16::IY, n1 as i8)), 3), // ADD A, (IY+d)
        0x87 => panic!("instruccion FD {:02X}  no implementada", op),
        0x88 => panic!("instruccion FD {:02X}  no implementada", op),
        0x89 => panic!("instruccion FD {:02X}  no implementada", op),
        0x8A => panic!("instruccion FD {:02X}  no implementada", op),
        0x8B => panic!("instruccion FD {:02X}  no implementada", op),
        0x8C => (ADC8(Reg8(R8::A), Reg8(R8::IYH)), 2),
        0x8D => (ADC8(Reg8(R8::A), Reg8(R8::IYL)), 2),
        0x8E => (ADC8(Reg8(R8::A), Indexado8(R16::IY, n1 as i8)), 3), // ADC A, (IY+d)
        0x8F => panic!("instruccion FD {:02X}  no implementada", op),
        0x90 => panic!("instruccion FD {:02X}  no implementada", op),
        0x91 => panic!("instruccion FD {:02X}  no implementada", op),
        0x92 => panic!("instruccion FD {:02X}  no implementada", op),
        0x93 => panic!("instruccion FD {:02X}  no implementada", op),
        0x94 => panic!("instruccion FD {:02X}  no implementada", op),
        0x95 => panic!("instruccion FD {:02X}  no implementada", op),
        0x96 => (SUB8(Reg8(R8::A), Indexado8(R16::IY, n1 as i8)), 3), // SUB (IY+d)
        0x97 => panic!("instruccion FD {:02X}  no implementada", op),
        0x98 => panic!("instruccion FD {:02X}  no implementada", op),
        0x99 => panic!("instruccion FD {:02X}  no implementada", op),
        0x9A => panic!("instruccion FD {:02X}  no implementada", op),
        0x9B => panic!("instruccion FD {:02X}  no implementada", op),
        0x9C => panic!("instruccion FD {:02X}  no implementada", op),
        0x9D => panic!("instruccion FD {:02X}  no implementada", op),
        0x9E => (SBC(Reg8(R8::A), Indexado8(R16::IY, n1 as i8)), 3), // SBC A, (IY+n2 as i8)
        0x9F => panic!("instruccion FD {:02X}  no implementada", op),
        0xA0 => panic!("instruccion FD {:02X}  no implementada", op),
        0xA1 => panic!("instruccion FD {:02X}  no implementada", op),
        0xA2 => panic!("instruccion FD {:02X}  no implementada", op),
        0xA3 => panic!("instruccion FD {:02X}  no implementada", op),
        0xA4 => panic!("instruccion FD {:02X}  no implementada", op),
        0xA5 => panic!("instruccion FD {:02X}  no implementada", op),
        0xA6 => (AND(Indexado8(R16::IY, n1 as i8)), 3), // AND (IY+d)
        0xA7 => panic!("instruccion FD {:02X}  no implementada", op),
        0xA8 => panic!("instruccion FD {:02X}  no implementada", op),
        0xA9 => panic!("instruccion FD {:02X}  no implementada", op),
        0xAA => panic!("instruccion FD {:02X}  no implementada", op),
        0xAB => panic!("instruccion FD {:02X}  no implementada", op),
        0xAC => panic!("instruccion FD {:02X}  no implementada", op),
        0xAD => panic!("instruccion FD {:02X}  no implementada", op),
        0xAE => (XOR(Indexado8(R16::IY, n1 as i8)), 3), // XOR (IY+d)
        0xAF => panic!("instruccion FD {:02X}  no implementada", op),
        0xB0 => panic!("instruccion FD {:02X}  no implementada", op),
        0xB1 => panic!("instruccion FD {:02X}  no implementada", op),
        0xB2 => panic!("instruccion FD {:02X}  no implementada", op),
        0xB3 => panic!("instruccion FD {:02X}  no implementada", op),
        0xB4 => panic!("instruccion FD {:02X}  no implementada", op),
        0xB5 => panic!("instruccion FD {:02X}  no implementada", op),
        0xB6 => panic!("instruccion FD {:02X}  no implementada", op),
        0xB7 => panic!("instruccion FD {:02X}  no implementada", op),
        0xB8 => panic!("instruccion FD {:02X}  no implementada", op),
        0xB9 => panic!("instruccion FD {:02X}  no implementada", op),
        0xBA => panic!("instruccion FD {:02X}  no implementada", op),
        0xBB => panic!("instruccion FD {:02X}  no implementada", op),
        0xBC => panic!("instruccion FD {:02X}  no implementada", op),
        0xBD => panic!("instruccion FD {:02X}  no implementada", op),
        0xBE => (CP(Indexado8(R16::IY, n1 as i8)), 3), // CP (IY+d)
        0xBF => panic!("instruccion FD {:02X}  no implementada", op),
        0xC0..=0xCA => panic!("instruccion FD {:02X}  no existe", op),
        0xCB => analiza_iy_bit(op, n1, n2),
        0xCC..0xCF => panic!("instruccion FD {:02X}  no existe", op),
        0xE0 => panic!("instruccion FD {:02X}  no existe", op),
        0xE1 => (POP(Local16::Reg(R16::IY)), 2),
        0xE2 => panic!("instruccion FD {:02X}  no existe", op),
        0xE3 => (EXSPIY, 2),  // EX (SP),IY
        0xE4 => panic!("instruccion FD {:02X}  no existe", op),
        0xE5 => (PUSH(Local16::Reg(R16::IY)), 2),
        0xE6..=0xE8 => panic!("instruccion FD {:02X}  no existe", op),
        0xE9 => (JP(SaltoCondicional::Incondicional, Local16::Reg(R16::IY)), 2),  // JP (IY)
        0xEA..=0xEF => panic!("instruccion FD {:02X}  no existe", op),
        0xF0..=0xF8 => panic!("instruccion FD {:02X}  no existe", op),
        //0xF9 => (LD16(Local16::Reg(R16::SP), Local16::Reg(R16::IY)), 2,),
        0xF9 => (LD16BIG(Local16::Reg(R16::SP), Local16::Reg(R16::IY)), 2,),
        0xFA..=0xFF => panic!("instruccion FD {:02X}  no existe", op),

        _op => unimplemented!("IY (FD HH) no implementado {:?} {:02X}", R16::IY, op),
    }
}