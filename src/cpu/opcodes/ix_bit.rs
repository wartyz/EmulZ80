use crate::ops::Op;

// Instrucciones FDCB IY bit
// pub fn analiza_iy_bit(n1: u8, n2: u8, n3: u8) -> (Op, usize) {
//     dbg_hex!(n1,n2,n3);
//     match n3 {
//         0xCE => panic!("Estoy en CE"),
//         _op => unimplemented!("Instruccion FDCB no implementado Reg = IY    0xFD 0xCB 0x{:02X} 0x{:02X}", n2, n3),
//     }
// }

/*pub fn analiza_iy_bit(n1: u8, n2: u8, n3: u8) -> (Op, usize) {
    dbg_hex!(n1, n2, n3);

    // El formato del opcode para BIT b, (IY+d) es: 01bbb110
    let mascara_bit = 0b11000111; // Máscara para aislar bits de operación y registro
    let patron_esperado = 0b01000110; // Patrón base BIT (IY+d) (0x46)

    match n3 {
        // Casos 0x40-0x4F, 0x50-0x5F, 0x60-0x6F, 0x70-0x7F
        op if (op & mascara_bit) == patron_esperado => {
            // Extraer el bit (bbb en bits 5-3)
            let bit_num = (op >> 3) & 0b111;

            // El desplazamiento (d) está en n2 (como i8)
            let d = n2 as i8;

            // Construir el direccionamiento indexado
            let loc = Local8::Indexado8(R16::IY, d);

            // La instrucción ocupa 4 bytes (FD CB d op)
            (Op::BIT(bit_num, loc), 4)
        }

        // Ejemplo para DEC (IY+d) (opcode 0x35)
        0x35 => {
            let d = n2 as i8;
            let loc = Local8::Indexado8(R16::IY, d);
            (Op::DEC8(loc), 3)
        }

        _op => unimplemented!(
            "Instrucción FDCB no implementada: 0xFD 0xCB 0x{:02X} 0x{:02X}",
            n2, n3
        ),
    }
}*/

// pub fn analiza_iy_bit(n1: u8, n2: u8, n3: u8) -> (Op, usize) {
//     dbg_hex!(n1, n2, n3);
//
//     let d = n2 as i8;
//     let loc = Local8::Indexado8(R16::IY, d);
//     let op_grp = n3 >> 6;
//     let op_sub = (n3 >> 3) & 0b111;
//     let reg_bits = n3 & 0b111;
//
//     match op_grp {
//         // Grupo 00: Rotaciones/Shifts (operan solo en la ubicación en memoria)
//         0b00 => {
//             let operation = match op_sub {
//                 0b000 => Op::RLC,
//                 0b001 => Op::RRC,
//                 0b010 => Op::RL,
//                 0b011 => Op::RR,
//                 0b100 => Op::SLA,
//                 0b101 => Op::SRA,
//                 0b110 => Op::SLL,
//                 0b111 => Op::SRL,
//                 _ => unreachable!(),
//             };
//
//             // Si hay registro destino (ej: RLC B,(IY+d))
//             let op = match reg_bits {
//                 0b000 => Op::LD(Local8::Reg8(R8::B), operation(loc)),
//                 0b001 => Op::LD(Local8::Reg8(R8::C), operation(loc)),
//                 0b010 => Op::LD(Local8::Reg8(R8::D), operation(loc)),
//                 0b011 => Op::LD(Local8::Reg8(R8::E), operation(loc)),
//                 0b100 => Op::LD(Local8::Reg8(R8::H), operation(loc)),
//                 0b101 => Op::LD(Local8::Reg8(R8::L), operation(loc)),
//                 0b111 => Op::LD(Local8::Reg8(R8::A), operation(loc)),
//                 _ => operation(loc), // Operación directa en memoria
//             };
//
//             (op, 4)
//         }
//
//         // Grupo 01: BIT
//         0b01 => {
//             let bit_num = op_sub;
//             (Op::BIT(bit_num, loc), 4)
//         }
//
//         // Grupo 10: RES
//         0b10 => {
//             let bit_num = op_sub;
//             let dest = match reg_bits {
//                 0b000 => Local8::Reg8(R8::B),
//                 0b001 => Local8::Reg8(R8::C),
//                 0b010 => Local8::Reg8(R8::D),
//                 0b011 => Local8::Reg8(R8::E),
//                 0b100 => Local8::Reg8(R8::H),
//                 0b101 => Local8::Reg8(R8::L),
//                 0b111 => Local8::Reg8(R8::A),
//                 _ => loc.clone(),
//             };
//             (Op::RES(bit_num, dest, loc), 4)
//         }
//
//         // Grupo 11: SET
//         0b11 => {
//             let bit_num = op_sub;
//             let dest = match reg_bits {
//                 0b000 => Local8::Reg8(R8::B),
//                 0b001 => Local8::Reg8(R8::C),
//                 0b010 => Local8::Reg8(R8::D),
//                 0b011 => Local8::Reg8(R8::E),
//                 0b100 => Local8::Reg8(R8::H),
//                 0b101 => Local8::Reg8(R8::L),
//                 0b111 => Local8::Reg8(R8::A),
//                 _ => loc.clone(),
//             };
//             (Op::SET(bit_num, dest, loc), 4)
//         }
//
//         _ => unreachable!(),
//     }
// }

pub fn analiza_ix_bit(_: u8, n2: u8, n3: u8) -> (Op, usize) {
    // let d = n2 as i8;
    // let loc = Local8::Indexado8(R16::IY, d);
    // let op_grp = n3 >> 6;
    // let op_sub = (n3 >> 3) & 0b111;
    // let reg_bits = n3 & 0b111;
    //
    // match op_grp {
    //     // Grupo 00: Rotaciones/Shifts (operaciones en memoria)
    //     0b00 => {
    //         let operation = match op_sub {
    //             0b000 => Op::RLC,
    //             0b001 => Op::RRC,
    //             0b010 => Op::RL,
    //             0b011 => Op::RR,
    //             0b100 => Op::SLA,
    //             0b101 => Op::SRA,
    //             0b110 => Op::SLL,
    //             0b111 => Op::SRL,
    //             _ => unreachable!(),
    //         };
    //
    //         match reg_bits {
    //             // Operación directa en memoria (ej: RLC (IY+d))
    //             0b110 => (operation(loc), 4),
    //
    //             // Operación + carga en registro (ej: RLC B,(IY+d))
    //             _ => {
    //                 let reg = match reg_bits {
    //                     0b000 => R8::B,
    //                     0b001 => R8::C,
    //                     0b010 => R8::D,
    //                     0b011 => R8::E,
    //                     0b100 => R8::H,
    //                     0b101 => R8::L,
    //                     0b111 => R8::A,
    //                     _ => unreachable!(),
    //                 };
    //                 // Primero realiza la operación en memoria, luego carga en el registro
    //                 (Op::LD8(Local8::Reg8(reg), loc), 4)
    //             }
    //         }
    //     }
    //
    //     // Resto de grupos (BIT/RES/SET)...
    //     0b01 => (Op::BIT(op_sub, loc), 4),
    //     0b10 => (Op::RES(op_sub, loc), 4),
    //     0b11 => (Op::SET(op_sub, loc), 4),
    //     _ => unimplemented!("Instrucción no soportada"),
    // }
    unimplemented!("Instrucción no soportada")
}