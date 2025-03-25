// ED

// MISC (ED)
use crate::cpu::opcodes::util::*;
use crate::ops::Local8::*;
use crate::ops::Op::*;
use crate::ops::*;

pub fn analiza(op: u8, n1: u8, n2: u8) -> (Op, usize) {
    match op {
        0x00 => panic!("instruccion ED {:02X}  no implementada", op),
        0x01 => panic!("instruccion ED {:02X}  no implementada", op),
        0x02 => panic!("instruccion ED {:02X}  no existe", op),
        0x03 => panic!("instruccion ED {:02X}  no existe", op),
        0x04 => panic!("instruccion ED {:02X}  no implementada", op),
        0x05 => panic!("instruccion ED {:02X}  no existe", op),
        0x06 => panic!("instruccion ED {:02X}  no existe", op),
        0x07 => panic!("instruccion ED {:02X}  no existe", op),
        0x08 => panic!("instruccion ED {:02X}  no implementada", op),
        0x09 => panic!("instruccion ED {:02X}  no implementada", op),
        0x0A => panic!("instruccion ED {:02X}  no existe", op),
        0x0B => panic!("instruccion ED {:02X}  no existe", op),
        0x0C => panic!("instruccion ED {:02X}  no implementada", op),
        0x0D => panic!("instruccion ED {:02X}  no existe", op),
        0x0E => panic!("instruccion ED {:02X}  no existe", op),
        0x0F => panic!("instruccion ED {:02X}  no existe", op),
        0x10 => panic!("instruccion ED {:02X}  no implementada", op),
        0x11 => panic!("instruccion ED {:02X}  no implementada", op),
        0x12 => panic!("instruccion ED {:02X}  no existe", op),
        0x13 => panic!("instruccion ED {:02X}  no existe", op),
        0x14 => panic!("instruccion ED {:02X}  no implementada", op),
        0x15 => panic!("instruccion ED {:02X}  no existe", op),
        0x16 => panic!("instruccion ED {:02X}  no existe", op),
        0x17 => panic!("instruccion ED {:02X}  no existe", op),
        0x18 => panic!("instruccion ED {:02X}  no implementada", op),
        0x19 => panic!("instruccion ED {:02X}  no implementada", op),
        0x1A => panic!("instruccion ED {:02X}  no existe", op),
        0x1B => panic!("instruccion ED {:02X}  no existe", op),
        0x1C => panic!("instruccion ED {:02X}  no implementada", op),
        0x1D => panic!("instruccion ED {:02X}  no existe", op),
        0x1E => panic!("instruccion ED {:02X}  no existe", op),
        0x1F => panic!("instruccion ED {:02X}  no existe", op),
        0x20 => panic!("instruccion ED {:02X}  no implementada", op),
        0x21 => panic!("instruccion ED {:02X}  no implementada", op),
        0x22 => panic!("instruccion ED {:02X}  no existe", op),
        0x23 => panic!("instruccion ED {:02X}  no existe", op),
        0x24 => panic!("instruccion ED {:02X}  no implementada", op),
        0x25 => panic!("instruccion ED {:02X}  no existe", op),
        0x26 => panic!("instruccion ED {:02X}  no existe", op),
        0x27 => panic!("instruccion ED {:02X}  no existe", op),
        0x28 => panic!("instruccion ED {:02X}  no implementada", op),
        0x29 => panic!("instruccion ED {:02X}  no implementada", op),
        0x2A => panic!("instruccion ED {:02X}  no existe", op),
        0x2B => panic!("instruccion ED {:02X}  no existe", op),
        0x2C => panic!("instruccion ED {:02X}  no implementada", op),
        0x2D => panic!("instruccion ED {:02X}  no existe", op),
        0x2E => panic!("instruccion ED {:02X}  no existe", op),
        0x2F => panic!("instruccion ED {:02X}  no existe", op),
        0x30 => panic!("instruccion ED {:02X}  no existe", op),
        0x31 => panic!("instruccion ED {:02X}  no existe", op),
        0x32 => panic!("instruccion ED {:02X}  no existe", op),
        0x33 => panic!("instruccion ED {:02X}  no existe", op),
        0x34 => panic!("instruccion ED {:02X}  no implementada", op),
        0x35 => panic!("instruccion ED {:02X}  no existe", op),
        0x36 => panic!("instruccion ED {:02X}  no existe", op),
        0x37 => panic!("instruccion ED {:02X}  no existe", op),
        0x38 => panic!("instruccion ED {:02X}  no implementada", op),
        0x39 => panic!("instruccion ED {:02X}  no implementada", op),
        0x3A => panic!("instruccion ED {:02X}  no existe", op),
        0x3B => panic!("instruccion ED {:02X}  no existe", op),
        0x3C => panic!("instruccion ED {:02X}  no implementada", op),
        0x3D => panic!("instruccion ED {:02X}  no existe", op),
        0x3E => panic!("instruccion ED {:02X}  no existe", op),
        0x3F => panic!("instruccion ED {:02X}  no existe", op),
        //0x40 implementada mas adelante,
        //0x41 implementada mas adelante,
        0x42 => (SBC16BIG(Local16::Reg(R16::HL), Local16::Reg(R16::BC)), 2),
        //0x43 => (LD16(le_imm_indir_le(n1, n2), Local16::Reg(R16::BC)), 4,),
        0x43 => (LD16BIG(le_imm_indir_be(n2, n1), Local16::Reg(R16::BC)), 4,),
        0x44 => (NEG, 2),
        0x45 => (RETN, 2),
        0x46 => (IM(Some(0)), 2),
        0x47 => (LD8BIG(Reg8(R8::I), Reg8(R8::A)), 2),
        0x48 => (IN(Reg8(R8::C), Reg8(R8::C)), 2),
        0x49 => (OUTBE(Reg8(R8::C), Reg8(R8::C)), 2),
        0x4A => (ADC16BIG(Local16::Reg(R16::HL), Local16::Reg(R16::BC)), 2),
        0x4B => (LD16BIG(Local16::Reg(R16::BC), le_imm_indir_le(n1, n2)), 4,),
        0x4C => (MLT(Local16::Reg(R16::BC)), 2),
        0x4D => (RETI, 1),
        0x4E => panic!("instruccion ED {:02X}  no implementada * IM 0", op),
        0x4F => (LD8BIG(Reg8(R8::R), Reg8(R8::A)), 2),
        //0x50 implementada mas adelante,
        //0x51 implementada mas adelante,
        0x52 => (SBC16BIG(Local16::Reg(R16::HL), Local16::Reg(R16::DE)), 2),
        //0x53 => (LD16(le_imm_indir_le(n1, n2), Local16::Reg(R16::DE)), 4,),
        0x53 => (LD16BIG(le_imm_indir_be(n2, n1), Local16::Reg(R16::DE)), 4,),
        0x54 => panic!("instruccion ED {:02X}  no implementada * NEG", op),
        0x55 => panic!("instruccion ED {:02X}  no implementada * RETN", op),
        0x56 => (IM(Some(1)), 2),
        0x57 => (LD8R(2), 2),                           // LD A,I
        0x58 => panic!("instruccion ED {:02X}  no implementada IN E,(C) ", op),
        0x59 => (OUTBE(Reg8(R8::E), Reg8(R8::C)), 2),
        0x5A => (ADC16BIG(Local16::Reg(R16::HL), Local16::Reg(R16::DE)), 2),
        0x5B => (LD16BIG(Local16::Reg(R16::DE), le_imm_indir_le(n1, n2)), 4), // LD DE,(nn)
        0x5C => panic!("instruccion ED {:02X}  no implementada * NEG ", op),
        0x5D => panic!("instruccion ED {:02X}  no implementada * RETN", op),
        0x5E => (IM(Some(2)), 2),                                                // IM 2
        0x5F => (LD8R(1), 2),                           // LD A,R
        //0x60 implementada mas adelante,
        //0x61 implementada mas adelante,),
        0x62 => panic!("instruccion ED {:02X}  no implementada SBC HL,HL", op),
        //0x63 => (LD16(le_imm_indir_le(n1, n2), Local16::Reg(R16::HL)), 4), // LD (nn),HL
        0x63 => (LD16BIG(le_imm_indir_be(n2, n1), Local16::Reg(R16::HL)), 4), // LD (nn),HL
        0x64 => panic!("instruccion ED {:02X}  no implementada * NEG", op),
        0x65 => panic!("instruccion ED {:02X}  no implementada * RETN", op),
        0x66 => panic!("instruccion ED {:02X}  no implementada * IM 0", op),
        0x67 => (RRD, 2),
        0x68 => panic!("instruccion ED {:02X}  no implementada", op),
        0x69 => (OUTBE(Reg8(R8::L), Reg8(R8::C)), 2),
        0x6A => (ADC16BIG(Local16::Reg(R16::HL), Local16::Reg(R16::HL)), 2),
        0x6B => (LD16BIG(Local16::Reg(R16::HL), le_imm_indir_le(n1, n2)), 4), // LD HL,(nn)
        0x6C => panic!("instruccion ED {:02X}  no implementada * NEG", op),
        0x6D => panic!("instruccion ED {:02X}  no implementada * RETN", op),
        0x6E => panic!("instruccion ED {:02X}  no implementada * IM 0", op),
        0x6F => (RLD, 2),
        0x70 => panic!("instruccion ED {:02X}  no implementada", op),
        0x71 => panic!("instruccion ED {:02X}  no implementada", op),
        0x72 => panic!("instruccion ED {:02X}  no implementada", op),
        //0x73 => (LD16(le_imm_indir_le(n1, n2), Local16::Reg(R16::SP)), 4), // LD (nn),SP
        0x73 => (LD16BIG(le_imm_indir_be(n2, n1), Local16::Reg(R16::SP)), 4), // LD (nn),SP
        0x74 => panic!("instruccion ED {:02X}  no implementada * NEG", op),
        0x75 => panic!("instruccion ED {:02X}  no implementada * RETN", op),
        0x76 => panic!("instruccion ED {:02X}  no implementada * IM 1", op),
        0x77 => panic!("instruccion ED {:02X}  no implementada * NOP *", op),
        0x78 => panic!("instruccion ED {:02X}  no implementada", op),
        0x79 => (OUTBE(Reg8(R8::A), Reg8(R8::C)), 2),
        0x7A => (ADC16BIG(Local16::Reg(R16::HL), Local16::Reg(R16::SP)), 2),
        0x7B => (LD16BIG(Local16::Reg(R16::SP), le_imm_indir_le(n1, n2)), 4), // LD SP,(nn)
        0x7C => panic!("instruccion ED {:02X}  no implementada * NEG", op),
        0x7D => panic!("instruccion ED {:02X}  no implementada * RETN", op),
        0x7E => panic!("instruccion ED {:02X}  no implementada * IM 1", op),
        0x7F => panic!("instruccion ED {:02X}  no implementada * NOP *", op),
        0x80 => panic!("instruccion ED {:02X}  no existe", op),
        0x81 => panic!("instruccion ED {:02X}  no existe", op),
        0x82 => panic!("instruccion ED {:02X}  no existe", op),
        0x83 => panic!("instruccion ED {:02X}  no implementada", op),
        0x84 => panic!("instruccion ED {:02X}  no existe", op),
        0x85 => panic!("instruccion ED {:02X}  no existe", op),
        0x86 => panic!("instruccion ED {:02X}  no existe", op),
        0x87 => panic!("instruccion ED {:02X}  no existe", op),
        0x88 => panic!("instruccion ED {:02X}  no existe", op),
        0x89 => panic!("instruccion ED {:02X}  no existe", op),
        0x8A => panic!("instruccion ED {:02X}  no existe", op),
        0x8B => panic!("instruccion ED {:02X}  no implementada", op),
        0x8C => panic!("instruccion ED {:02X}  no existe", op),
        0x8D => panic!("instruccion ED {:02X}  no existe", op),
        0x8E => panic!("instruccion ED {:02X}  no existe", op),
        0x8F => panic!("instruccion ED {:02X}  no existe", op),
        0x90 => panic!("instruccion ED {:02X}  no existe", op),
        0x91 => panic!("instruccion ED {:02X}  no existe", op),
        0x92 => panic!("instruccion ED {:02X}  no existe", op),
        0x93 => panic!("instruccion ED {:02X}  no implementada", op),
        0x94 => panic!("instruccion ED {:02X}  no existe", op),
        0x95 => panic!("instruccion ED {:02X}  no existe", op),
        0x96 => panic!("instruccion ED {:02X}  no existe", op),
        0x97 => panic!("instruccion ED {:02X}  no existe", op),
        0x98 => panic!("instruccion ED {:02X}  no existe", op),
        0x99 => panic!("instruccion ED {:02X}  no existe", op),
        0x9A => panic!("instruccion ED {:02X}  no existe", op),
        0x9B => panic!("instruccion ED {:02X}  no implementada", op),
        0x9C => panic!("instruccion ED {:02X}  no existe", op),
        0x9D => panic!("instruccion ED {:02X}  no existe", op),
        0x9E => panic!("instruccion ED {:02X}  no existe", op),
        0x9F => panic!("instruccion ED {:02X}  no existe", op),

        // el if lo cumple 0xED 0x40, 0xED 0x41, 0xED 0x50, 0xED 0x51, 0xED 0x60, 0xED 0x61,
        op if op & 0b1100_0110 == 0b0100_0000 => {
            // Grupo de instrucciones IN/OUT con registro C
            // Patrón binario: 01xx xxx0
            // Máscara: 1100 0110 (C6h) | Valor: 0100 0000 (40h)

            // Determinar si es IN u OUT usando el bit menos significativo
            let operation = if op & 0b1 == 0b1 {
                OUTBE  // Si bit 0 = 1: OUT (C), r
            } else {
                IN   // Si bit 0 = 0: IN r, (C)
            };

            // Extraer bits 3-5 para el registro (formato xxx000)
            let bits_de_reg = (op >> 3) & 0b111;

            // Convertir bits del registro a tipo Reg8
            if let Reg8(reg) = reg_bits(bits_de_reg) {
                // Construir la instrucción final
                (operation(Reg8(reg), Reg8(R8::C)), 2)
            } else {
                // Manejar caso inválido: Ejemplo: IN (HL),(C) no existe
                panic!("Operación ED {:02x} no soportada: Combinación registro no válida", op)
            }
        }

        0xA0 => (LDI, 2),
        0xA1 => (CPI, 2),
        0xA2 => (INI, 2),
        //0xA3 => panic!("instruccion ED {:02X} {:02X} {:02X}  no implementada OUTI", op, n1, n2),
        //  [0xD3, n1, _, _] => (OUT(Reg8(R8::A), Inmediato8(n1)), 2),
        0xA3 => (OUTI, 2),   // OUTI
        0xA4 => panic!("instruccion ED {:02X}  no existe", op),
        0xA5 => panic!("instruccion ED {:02X}  no existe", op),
        0xA6 => panic!("instruccion ED {:02X}  no existe", op),
        0xA7 => panic!("instruccion ED {:02X}  no existe", op),
        0xA8 => panic!("instruccion ED {:02X} {:02X} {:02X} no implementada LDD", op, n1, n2),
        //0xA9 => panic!("instruccion ED {:02X} {:02X} {:02X} no implementada CPD", op, n1, n2),
        0xA9 => (CPD(Reg8(R8::A), RegIndirecto8(R16::HL)), 2),
        0xAA => panic!("instruccion ED {:02X} {:02X} {:02X} no implementada IND", op, n1, n2),
        0xAB => panic!("instruccion ED {:02X} {:02X} {:02X} no implementada OUTD", op, n1, n2),
        0xAC => panic!("instruccion ED {:02X}  no existe", op),
        0xAD => panic!("instruccion ED {:02X}  no existe", op),
        0xAE => panic!("instruccion ED {:02X}  no existe", op),
        0xAF => panic!("instruccion ED {:02X}  no existe", op),
        0xB0 => (LDIR, 2),           // LDIR,
        0xB1 => (CPIR, 2),
        0xB2 => panic!("instruccion ED {:02X} {:02X} {:02X} no implementada INIR", op, n1, n2),
        0xB3 => panic!("instruccion ED {:02X} {:02X} {:02X} no implementada OTIR", op, n1, n2),
        0xB4 => panic!("instruccion ED {:02X}  no existe", op),
        0xB5 => panic!("instruccion ED {:02X}  no existe", op),
        0xB6 => panic!("instruccion ED {:02X}  no existe", op),
        0xB7 => panic!("instruccion ED {:02X}  no existe", op),
        0xB8 => (LDDR, 2),           // LDDR
        0xB9 => panic!("instruccion ED {:02X} {:02X} {:02X} no implementada CPDR", op, n1, n2),
        0xBA => panic!("instruccion ED {:02X} {:02X} {:02X} no implementada INDR", op, n1, n2),
        0xBB => panic!("instruccion ED {:02X} {:02X} {:02X} no implementada OTDR", op, n1, n2),
        0xBC => panic!("instruccion ED {:02X}  no existe", op),
        0xBD => panic!("instruccion ED {:02X}  no existe", op),
        0xBE => panic!("instruccion ED {:02X}  no existe", op),
        0xBF => panic!("instruccion ED {:02X}  no existe", op),

        // [0xED, op, _, _] if op & 0b1100_0111 == 0b0100_0000 => {
        //     // Grupo de instrucciones IN especiales
        //     // Patrón binario: 01xx xxx0 0xxx
        //     // Máscara: 1100 0111 (C7h) | Valor: 0100 0000 (40h)
        //
        //     // Extraer bits 3-5 para el registro (formato xxx000)
        //     let bits_de_reg = (op >> 3) & 0b111;
        //
        //     if let Reg8(reg) = reg_bits(bits_de_reg) {
        //         // Construir instrucción IN r,(C)
        //         (IN(Reg8(reg), Reg8(R8::C)), 2)
        //     } else {
        //         // Bloquear ejecución para combinaciones inválidas como IN (HL),(C)
        //         panic!("Operación IN inválida con registro HL: ED {:02x}", op)
        //     }
        // }
        _op => unimplemented!("IX no implementado ED {:02X} {:02X} {:02X}", op, n1, n2),
    }
}

// Segun DeepSeek
// pub fn analiza(op: u8, n1: u8, n2: u8) -> (Op, usize) {
//     dbg_hex!(op, n1, n2);
//     match op {
//         // Instrucciones no implementadas o inválidas
//         0x00..0x3F => panic!("instruccion ED {:02X}  rara mirar tabla", op),
//
//         // Grupo IN/OUT usando registro C
//         0x40 => (INFLAGS(Reg8(R8::B), Reg8(R8::C)), 2),  // IN B,(C)
//         0x41 => (OUT(Reg8(R8::B), Reg8(R8::C)), 2),      // OUT (C),B
//         0x42 => (SBC16(Local16::Reg(R16::HL), Local16::Reg(R16::BC)), 2),
//         0x43 => (LD16(le_imm_indir_le(n1, n2), Local16::Reg(R16::BC)), 4),
//         0x44 => (NEG, 2),
//         0x45 => (RETN, 2),
//         0x46 => (IM(false), 2),
//         0x47 => (LD8(Reg8(R8::I), Reg8(R8::A)), 2),
//         0x48 => (INFLAGS(Reg8(R8::C), Reg8(R8::C)), 2),  // IN C,(C)
//         0x49 => (OUT(Reg8(R8::C), Reg8(R8::C)), 2),      // OUT (C),C
//         0x4A => (ADC16BIG(Local16::Reg(R16::HL), Local16::Reg(R16::BC)), 2),
//         0x4B => (LD16BIG(Local16::Reg(R16::BC), le_imm_indir_le(n1, n2)), 4),
//
//         // Instrucciones de interrupciones y modos
//         0x4C => panic!("instruccion ED {:02X} no existe", op),  // NEG inválido
//         0x4D => (RETI, 2),                                      // RETI
//         0x4E => (IM(false), 2),                                 // IM 0
//         0x4F => (LD8(Reg8(R8::R), Reg8(R8::A)), 2),             // LD R,A
//
//         // Continuación grupo IN/OUT
//         0x50 => (INFLAGS(Reg8(R8::D), Reg8(R8::C)), 2),  // IN D,(C)
//         0x51 => (OUT(Reg8(R8::D), Reg8(R8::C)), 2),      // OUT (C),D
//         0x52 => (SBC16(Local16::Reg(R16::HL), Local16::Reg(R16::DE)), 2),
//         0x53 => (LD16(le_imm_indir_le(n1, n2), Local16::Reg(R16::DE)), 4),
//         0x54 => panic!("instruccion ED {:02X} no existe", op),  // NEG inválido
//         0x55 => panic!("instruccion ED {:02X} no existe", op),  // RETN inválido
//         0x56 => (IM(true), 2),                                  // IM 1
//         0x57 => (LD8(Reg8(R8::A), Reg8(R8::I)), 2),
//         0x58 => (INFLAGS(Reg8(R8::E), Reg8(R8::C)), 2),  // IN E,(C)
//         0x59 => (OUT(Reg8(R8::E), Reg8(R8::C)), 2),      // OUT (C),E
//         0x5A => (ADC16BIG(Local16::Reg(R16::HL), Local16::Reg(R16::DE)), 2),
//
//         // Instrucciones de carga 16-bit
//         0x5B => (LD16BIG(Local16::Reg(R16::DE), le_imm_indir_le(n1, n2)), 4), // LD DE,(nn)
//         0x5C => panic!("instruccion ED {:02X} no existe", op),                // NEG inválido
//         0x5D => panic!("instruccion ED {:02X} no existe", op),                // RETN inválido
//         0x5E => (IM(true), 2),                                                // IM 2
//         0x5F => (LD8(Reg8(R8::A), Reg8(R8::R)), 2),                           // LD A,R
//
//         // Instrucciones aritméticas
//         0x60 => (INFLAGS(Reg8(R8::H), Reg8(R8::C)), 2),  // IN H,(C)
//         0x61 => (OUT(Reg8(R8::H), Reg8(R8::C)), 2),      // OUT (C),H
//         0x62 => (SBC16(Local16::Reg(R16::HL), Local16::Reg(R16::HL)), 2),
//         0x63 => (LD16(le_imm_indir_le(n1, n2), Local16::Reg(R16::HL)), 4), // LD (nn),HL
//         0x64 => panic!("instruccion ED {:02X} no existe", op),             // NEG inválido
//         0x65 => panic!("instruccion ED {:02X} no existe", op),             // RETN inválido
//         0x66 => panic!("instruccion ED {:02X} no existe", op),             // IM inválido
//
//         // Instrucciones de rotación
//         0x67 => (RRD, 2),
//         0x6A => (ADC16BIG(Local16::Reg(R16::HL), Local16::Reg(R16::HL)), 2),
//         0x6F => (RLD, 2),
//
//         // Instrucciones especiales
//         0x7A => (ADC16BIG(Local16::Reg(R16::HL), Local16::Reg(R16::SP)), 2),
//
//         // Grupo bloque IN/OUT automático
//         0xA0 => (LDI, 2),
//         0xA1 => (CPI, 2),
//         0xA2 => (INI, 2),
//         0xA3 => (OUTI, 2),
//         0xA8 => (LDD, 2),
//         0xA9 => (CPD, 2),
//         0xAA => (IND, 2),
//         0xAB => (OUTD, 2),
//
//         // Grupo bloque repetitivo
//         0xB0 => (LDIR, 2),
//         0xB1 => (CPIR, 2),
//         0xB2 => (INIR, 2),
//         0xB3 => (OTIR, 2),
//         0xB8 => (LDDR, 2),
//         0xB9 => (CPDR, 2),
//         0xBA => (INDR, 2),
//         0xBB => (OTDR, 2),
//
//         // Manejador genérico para códigos no implementados
//         _op => unimplemented!("Opcode ED {:02X} no implementado", op),
//     }
// }