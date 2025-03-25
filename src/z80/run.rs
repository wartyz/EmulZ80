use crate::cpu::opcodes;
use crate::ops::Local16::*;
use crate::ops::Local8::*;
use crate::ops::*;
use crate::z80::Z80;
use crate::{println_tee, DEBUG, VALOR_BREAKPOINT};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};

// Experimento
use std::fs::OpenOptions;
use std::io::Write;
use zilog_z80::cpu::CPU;
// macro_rules! println_tee {
//     ($($arg:tt)*) => {{
//         let message = format!($($arg)*);
//         // Imprime en consola
//         println!("{}", message);
//         // Escribe en el archivo (manejo básico de errores)
//         let _ = OpenOptions::new()
//             .append(true)
//             .create(true)
//             .open("salida.txt")
//             .and_then(|mut file| writeln!(file, "{}", message));
//     }};
// }

impl Z80 {
    // Carga una función en la memoria.
    // Esto se hace asignando los bytes proporcionados a la memoria, comenzando en 0x0000
    // ¡Solo tienes 16 kibibytes con los que trabajar, así que ten cuidado!

    pub fn load(&mut self, program: &[u8]) {
        for (i, b) in program.iter().enumerate() {
            self.mem.mem[i] = *b
        }
    }

    // Para imprimir en hexadecimal
    fn format_opcode1(&self, op: &Op) -> String {
        match op {
            Op::JP(cond, Inmediato(val)) =>
                format!("JP({:?}, Immediate({:04X}))", cond, val),
            Op::LD16BIG(cond, Inmediato(val)) =>
                format!("LD16({:?}, Inmediato({:04X}))", cond, val),
            // Op::LD8(cond, Inmediato8(val)) =>
            //     format!("LD8({:?}, Inmediato({:02X}))", cond, val),
            Op::OUTBE(cond, Inmediato8(val)) =>
                format!("OUT({:?}, Inmediato({:02X}))", cond, val),

            _ => format!("{:?}", op), // Para otros casos, usa el Debug normal
        }
    }

    // Para imprimir en hexadecimal
    fn fmat_op(&self, op: &Op, direccion_donde_esta: u16) -> String {
        match op {
            // Operaciones lógicas con un registro de 8 bits
            Op::AND(Reg8(reg)) => format!("AND {}", self.reg8_to_str(reg)),
            Op::AND(Inmediato8(n)) => format!("AND 0x{}", n),
            Op::ANDBIG(RegIndirecto8(reg)) => format!("AND ({})", self.reg16_to_str(reg)),

            Op::OR(Reg8(reg)) => format!("OR {}", self.reg8_to_str(reg)),
            Op::OR(Inmediato8(n)) => format!("OR 0x{}", n),
            Op::ORBIG(RegIndirecto8(reg)) => format!("OR ({})", self.reg16_to_str(reg)),

            Op::XOR(Reg8(reg)) => format!("XOR {}", self.reg8_to_str(reg)),
            Op::XOR(Inmediato8(n)) => format!("XOR 0x{}", n),
            Op::XORBIG(RegIndirecto8(reg)) => format!("XOR ({})", self.reg16_to_str(reg)),

            // Comparación
            Op::CP(Reg8(reg)) => format!("CP {}", self.reg8_to_str(reg)),
            Op::CP(Inmediato8(val)) => format!("CP 0x{:02X}", val),
            Op::CPBIG(RegIndirecto8(reg)) => format!("CP ({})", self.reg16_to_str(reg)),

            // Decremento
            Op::DEC8(RegIndirecto8(reg)) =>
                format!("DEC ({})", self.reg16_to_str(reg)),
            Op::DEC8BIG(RegIndirecto8(reg)) =>
                format!("DEC ({})", self.reg16_to_str(reg)),

            Op::DEC8(Reg8(reg)) =>
                format!("DEC {}", self.reg8_to_str(reg)),
            Op::DEC8BIG(Reg8(reg)) =>
                format!("DEC {}", self.reg8_to_str(reg)),

            Op::DEC8(Indexado8(reg, n)) =>
                format!("DEC ({}+0x{:02X}) ", self.reg16_to_str(reg), n),
            Op::DEC8BIG(Indexado8(reg, n)) =>
                format!("DEC ({}+0x{:02X}) ", self.reg16_to_str(reg), n),

            Op::DEC16(Reg(reg), Reg(_)) =>
                format!("DEC {}", self.reg16_to_str(reg)),
            Op::DEC16BIG(Reg(reg), Reg(_)) =>
                format!("DEC {}", self.reg16_to_str(reg)),

            // Incremento
            Op::INC8(Reg8(reg)) |
            Op::INC8BIG(Reg8(reg)) => format!("INC {}", self.reg8_to_str(reg)),

            Op::INC8(RegIndirecto8(reg)) |
            Op::INC8BIG(RegIndirecto8(reg)) => format!("INC ({})", self.reg16_to_str(reg)),

            Op::INC16(Reg(reg), Reg(_)) =>
                format!("INC {}", self.reg16_to_str(reg)),
            Op::INC16BIG(Reg(reg), Reg(_)) =>
                format!("INC {}", self.reg16_to_str(reg)),

            // Saltos condicionales e incondicionales
            Op::JP(SaltoCondicional::Incondicional, Inmediato(val)) =>
                format!("JP,0x{:04X}", val),

            Op::JP(SaltoCondicional::NoCero, Inmediato(val)) =>
                format!("JP NZ,0x{:04X}", val),
            Op::JP(SaltoCondicional::NoCarry, Inmediato(val)) =>
                format!("JP NC,0x{:04X}", val),
            Op::JP(SaltoCondicional::ParidadImpar, Inmediato(val)) =>
                format!("JP PO,0x{:04X}", val),
            Op::JP(SaltoCondicional::SignoPositivo, Inmediato(val)) =>
                format!("JP P,0x{:04X}", val),

            Op::JP(SaltoCondicional::Cero, Inmediato(val)) =>
                format!("JP Z,0x{:04X}", val),
            Op::JP(SaltoCondicional::Carry, Inmediato(val)) =>
                format!("JP C,0x{:04X}", val),
            Op::JP(SaltoCondicional::ParidadPar, Inmediato(val)) =>
                format!("JP PE,0x{:04X}", val),
            Op::JP(SaltoCondicional::SignoNegativo, Inmediato(val)) =>
                format!("JP M,0x{:04X}", val),

            Op::JPHL(Reg(reg1)) =>
                format!("JP ({})", self.reg16_to_str(reg1)),

            Op::JR(SaltoCondicional::Incondicional, val) => {
                // hay que usar la direccion donde esta la instruccion no el pc, que estara en otro sitio
                format!("JR 0x{:04X}", direccion_donde_esta
                    .wrapping_add_signed(*val as i16)
                    .wrapping_add_signed(2))
            }
            Op::JR(SaltoCondicional::Cero, val) => {
                // hay que usar la direccion donde esta la instruccion no el pc, que estara en otro sitio
                format!("JR Z,0x{:04X}", direccion_donde_esta
                    .wrapping_add_signed(*val as i16)
                    .wrapping_add_signed(2))
            }
            Op::JR(SaltoCondicional::NoCero, val) => {
                // hay que usar la direccion donde esta la instruccion no el pc, que estara en otro sitio
                format!("JR NZ,0x{:04X}", direccion_donde_esta
                    .wrapping_add_signed(*val as i16)
                    .wrapping_add_signed(2))
            }
            Op::JR(SaltoCondicional::Carry, val) => {
                // hay que usar la direccion donde esta la instruccion no el pc, que estara en otro sitio
                format!("JR C,0x{:04X}", direccion_donde_esta
                    .wrapping_add_signed(*val as i16)
                    .wrapping_add_signed(2))
            }
            Op::JR(SaltoCondicional::NoCarry, val) => {
                // hay que usar la direccion donde esta la instruccion no el pc, que estara en otro sitio
                format!("JR NC,0x{:04X}", direccion_donde_esta
                    .wrapping_add_signed(*val as i16)
                    .wrapping_add_signed(2))
            }
            Op::DJNZ(v) => format!("DJNZ {}", v),

            // Aritmética de 16 bits
            Op::ADD16BIG(Reg(reg1), Reg(reg2)) |
            // Op::ADD16(Reg(reg1), Reg(reg2)) =>
            //     format!("ADD {},{}", self.reg16_to_str(reg1), self.reg16_to_str(reg2)),

            Op::SBC16BIG(Reg(reg1), Reg(reg2)) =>
                format!("SBC {},{}", self.reg16_to_str(reg1), self.reg16_to_str(reg2)),

            Op::SBC(Reg8(reg1), Reg8(reg2)) |
            Op::SBCBIG(Reg8(reg1), Reg8(reg2)) =>
                format!("SBC {},{}", self.reg8_to_str(reg1), self.reg8_to_str(reg2)),

            Op::SBC(Reg8(R8::A), RegIndirecto8(r)) |
            Op::SBCBIG(Reg8(R8::A), RegIndirecto8(r)) =>
                format!("SBC A,({})", self.reg16_to_str(r)),

            Op::SBC(Reg8(R8::A), Inmediato8(n)) |
            Op::SBCBIG(Reg8(R8::A), Inmediato8(n)) =>
                format!("SBC A,0x{}", n),

            // Aritmetica de 8 bits
            Op::SUB8(Reg8(R8::A), Reg8(r)) |
            Op::SUB8BIG(Reg8(R8::A), Reg8(r)) =>
                format!("SUB {}", self.reg8_to_str(r)),

            Op::SUB8(Reg8(R8::A), RegIndirecto8(r)) |
            Op::SUB8BIG(Reg8(R8::A), RegIndirecto8(r)) =>
                format!("SUB ({})", self.reg16_to_str(r)),

            Op::SUB8(Reg8(R8::A), Inmediato8(n)) |
            Op::SUB8BIG(Reg8(R8::A), Inmediato8(n)) =>
                format!("SUB 0x{}", n),

            Op::ADC8(Reg8(R8::A), RegIndirecto8(r)) |
            Op::ADC8BIG(Reg8(R8::A), RegIndirecto8(r)) =>
                format!("ADC A({})", self.reg16_to_str(r)),

            Op::ADC8(Reg8(r1), Reg8(r2)) |
            Op::ADC8BIG(Reg8(r1), Reg8(r2)) =>
                format!("ADC {},{}", self.reg8_to_str(r1), self.reg8_to_str(r2)),

            Op::ADC8(Reg8(R8::A), Inmediato8(n)) |
            Op::ADC8BIG(Reg8(R8::A), Inmediato8(n)) =>
                format!("ADC A,0x{}", n),

            Op::ADD8(Reg8(R8::A), RegIndirecto8(r)) |
            Op::ADD8BIG(Reg8(R8::A), RegIndirecto8(r)) =>
                format!("ADD A ({})", self.reg16_to_str(r)),

            Op::ADD8(Reg8(R8::A), Inmediato8(n)) |
            Op::ADD8BIG(Reg8(R8::A), Inmediato8(n)) =>
                format!("ADD A,0x{}", n),

            Op::ADD8(Reg8(r1), Reg8(r2)) |
            Op::ADD8BIG(Reg8(r1), Reg8(r2)) =>
                format!("ADD {},{}", self.reg8_to_str(r1), self.reg8_to_str(r2)),

            Op::ADD8(Reg8(R8::A), Indexado8(r, n)) |
            Op::ADD8BIG(Reg8(R8::A), Indexado8(r, n)) =>
                format!("ADD A,({}+0x{:02X})", self.reg16_to_str(r), n),

            // Carga de 8 bits
            //Op::LD8(RegIndirecto8(reg), Reg8(val)) |
            Op::LD8BIG(RegIndirecto8(reg), Reg8(val)) =>
                format!("LD ({}),{}", self.reg16_to_str(reg), self.reg8_to_str(val)),
            //Op::LD8(RegIndirecto8(reg), Inmediato8(val)) |
            Op::LD8BIG(RegIndirecto8(reg), Inmediato8(val)) =>
                format!("LD ({}),{}", self.reg16_to_str(reg), val),

            //Op::LD8(Reg8(reg1), RegIndirecto8(reg2)) |
            Op::LD8BIG(Reg8(reg1), RegIndirecto8(reg2)) =>
                format!("LD {},({})", self.reg8_to_str(reg1), reg2),

            Op::LD8BIG(Reg8(reg1), Reg8(reg2)) =>
                format!("LD {},{}", self.reg8_to_str(reg1), self.reg8_to_str(reg2)),

            Op::LD8BIG(Reg8(reg), Inmediato8(val)) =>
                format!("LD {},0x{:02X}", self.reg8_to_str(reg), val),

            Op::LD8BIG(Indexado8(reg1, n), Reg8(reg2)) =>
                format!("LD ({}+{}),{}", self.reg16_to_str(reg1), n, self.reg8_to_str(reg2)),

            Op::LD8BIG(Indexado8(reg1, n), Inmediato8(val)) =>
                format!("LD ({}+0x{:02X}),{}", self.reg16_to_str(reg1), n, val),

            Op::LD8BIG(Reg8(reg1), Indexado8(reg2, n)) =>
                format!("LD {},({}+0x{:02X})", self.reg8_to_str(reg1), self.reg16_to_str(reg2), n),

            Op::LD8BIG(InmediatoIndirecto8(n), Reg8(reg)) =>
                format!("LD (0x{:04X}),{}", n, self.reg8_to_str(reg)),

            Op::LD8BIG(Reg8(reg), InmediatoIndirecto8(n)) =>
                format!("LD {},(0x{:04X})", self.reg8_to_str(reg), n),

            // Carga de 16 bits
            // Op::LD16(Reg(reg), Inmediato(val)) =>
            //     format!("LD {} 0x{:04X}", self.reg16_to_str(reg), val),
            //Op::LD16(Reg(reg1), Reg(reg2)) |
            Op::LD16BIG(Reg(reg1), Reg(reg2)) =>
                format!("LD {},{}", self.reg16_to_str(reg1), self.reg16_to_str(reg2)),
            Op::LD16BIG(Reg(reg), Inmediato(val)) =>
                format!("LD {} 0x{:04X}", self.reg16_to_str(reg), val),
            Op::LD16BIG(Reg(reg), InmediatoIndirecto(val)) =>
                format!("LD {} 0x{:04X}", self.reg16_to_str(reg), val),
            // Op::LD16(Reg(reg), InmediatoIndirecto(val)) =>
            //     format!("LD {},(0x{:04X})", self.reg16_to_str(reg), val),
            // Op::LD16(InmediatoIndirecto(val), Reg(reg)) =>
            //     format!("LD (0x{:04X}),{} ", val, self.reg16_to_str(reg)),

            Op::LD16BIG(Reg(reg), InmediatoIndirecto(val)) =>
                format!("LD {},(0x{:04X})", self.reg16_to_str(reg), val),
            Op::LD16BIG(InmediatoIndirecto(val), Reg(reg)) =>
                format!("LD (0x{:04X}),{} ", val, self.reg16_to_str(reg)),

            // Operaciones de desplazamiento y rotación
            Op::RLCA => format!("RLCA"),
            Op::RLA => format!("RLA"),
            Op::RRCA => format!("RRCA"),
            Op::RRA => format!("RRA"),
            Op::RLC(Reg8(reg)) => format!("RLC {}", self.reg8_to_str(reg)),
            Op::SLA(Reg8(reg)) => format!("SLA {}", self.reg8_to_str(reg)),
            Op::SRA(Reg8(reg)) => format!("SRA {}", self.reg8_to_str(reg)),
            Op::SRL(Reg8(reg)) => format!("SRL {}", self.reg8_to_str(reg)),
            Op::RL(Reg8(reg)) => format!("RL {}", self.reg8_to_str(reg)),
            Op::RR(Reg8(reg)) => format!("RR {}", self.reg8_to_str(reg)),

            // Operaciones de pila
            Op::PUSH(Reg(reg)) => format!("PUSH {}", self.reg16_to_str(reg)),
            Op::POP(Reg(reg)) => format!("POP {}", self.reg16_to_str(reg)),

            // Operaciones de intercambio y complemento
            Op::EX16 => format!("EX AF,AF'"),
            Op::EXSPHL => format!("EX (SP),HL"),
            Op::EX => format!("EX DE,HL"),
            Op::EXSPIX => format!("EX (SP),IX"),
            Op::EXSPIY => format!("EX (SP),IY"),
            Op::CPL => format!("CPL"),
            Op::DAA => format!("DAA"),
            Op::SCF => format!("SCF"),
            Op::CCF => format!("CCF"),

            // Llamadas (calls)
            Op::CALL(SaltoCondicional::Incondicional, val) =>
                format!("CALL 0x{:04X}", val),

            Op::CALL(SaltoCondicional::Cero, val) =>
                format!("CALL Z,0x{:04X}", val),
            Op::CALL(SaltoCondicional::NoCero, val) =>
                format!("CALL NZ,0x{:04X}", val),

            Op::CALL(SaltoCondicional::Carry, val) =>
                format!("CALL C,0x{:04X}", val),
            Op::CALL(SaltoCondicional::NoCarry, val) =>
                format!("CALL NC,0x{:04X}", val),

            Op::CALL(SaltoCondicional::ParidadPar, val) =>
                format!("CALL PE,0x{:04X}", val),
            Op::CALL(SaltoCondicional::ParidadImpar, val) =>
                format!("CALL PO,0x{:04X}", val),

            Op::CALL(SaltoCondicional::SignoNegativo, val) =>
                format!("CALL M,0x{:04X}", val),
            Op::CALL(SaltoCondicional::SignoPositivo, val) =>
                format!("CALL P,0x{:04X}", val),

            // Retornos (RET)
            Op::RET(SaltoCondicional::Incondicional) => format!("RET"),
            Op::RET(SaltoCondicional::Cero) => format!("RET Z "),
            Op::RET(SaltoCondicional::NoCero) => format!("RET NZ"),
            Op::RET(SaltoCondicional::Carry) => format!("RET C"),
            Op::RET(SaltoCondicional::NoCarry) => format!("RET NC"),
            Op::RET(SaltoCondicional::ParidadPar) => format!("RET PE"),
            Op::RET(SaltoCondicional::ParidadImpar) => format!("RET PO"),
            Op::RET(SaltoCondicional::SignoNegativo) => format!("RET M"),
            Op::RET(SaltoCondicional::SignoPositivo) => format!("RET P"),

            Op::RETI => format!("RETI"),

            // Operaciones de salida
            Op::OUTBE(Reg8(reg), Inmediato8(val)) =>
                format!("OUT(0x{:02X}),{}", val, self.reg8_to_str(reg)),
            Op::OUTBE(Reg8(reg1), Reg8(reg2)) =>
                format!("OUT ({}),{}", self.reg8_to_str(reg2), self.reg8_to_str(reg1)),

            // Operaciones de entrada
            Op::IN(Reg8(reg), Inmediato8(val)) =>
                format!("IN {},(0x{:02X}),", self.reg8_to_str(reg), val),
            Op::IN(Reg8(reg1), Reg8(reg2)) =>
                format!("IN {},({}),", self.reg8_to_str(reg1), self.reg8_to_str(reg2)),

            // Reset a una dirección fija
            Op::RST(Inmediato(v)) => format!("RST 0x{:02X}", v),

            // Operaciones con bit
            Op::BIT(v, Indexado8(R16::IY, n)) =>
                format!("BIT {},(IY+0x{:02X})", v, n),
            Op::RESBIG(v, Indexado8(R16::IY, n)) =>
                format!("RES {},(IY+0x{:02X})", v, n),

            Op::RESBIG(n, RegIndirecto8(reg)) =>
                format!("RES {},({})", n, self.reg16_to_str(reg)),

            Op::SETBIG(v, Indexado8(reg, n)) =>
                format!("BIT {},({}+0x{:02X})", v, self.reg16_to_str(reg), n),

            Op::SETBIG(n, RegIndirecto8(reg)) =>
                format!("SET {},({})", n, self.reg16_to_str(reg)),

            Op::SLL(Reg8(reg)) =>
                format!("SLL {}", self.reg8_to_str(reg)),

            // Interrupciones
            Op::IM(o) => format!("IM {:02X}", o.unwrap()),
            // Caso por defecto: devolver el debug string de la operación
            _ => format!("{:?}", op),
        }
    }

    fn reg16_to_str(&self, reg: &R16) -> &'static str {
        match reg {
            R16::AF => "AF",
            R16::BC => "BC",
            R16::DE => "DE",
            R16::HL => "HL",
            R16::AFP => "AF'",
            R16::BCP => "BC'",
            R16::DEP => "DE'",
            R16::HLP => "HL'",
            R16::IX => "IX",
            R16::IY => "IY",
            R16::SP => "SP",
        }
    }

    fn reg8_to_str(&self, reg: &R8) -> &'static str {
        match reg {
            R8::A => "A",
            R8::B => "B",
            R8::C => "C",
            R8::D => "D",
            R8::E => "E",
            R8::H => "H",
            R8::L => "L",

            R8::F => "F",
            R8::I => "I",
            R8::R => "R",
            R8::AP => "A'",
            R8::FP => "F'",
            R8::BP => "B'",
            R8::DP => "D'",
            R8::HP => "H'",
            R8::LP => "L'",
            R8::IFF1 => "IFF1",
            R8::IFF2 => "IFF2",
            R8::IM => "IM",

            _ => " ",
        }
    }

    // Ejecuta una simple instruccion.
    // El contador de programa es actualizado con la nueva posicion, preparado para el siguiente paso
    // Panico si el contador de programa esta mas lejos del final de la memoria de la CPU
    pub fn step(&mut self, c: &mut CPU) -> usize {
        let mut bytes_consumidos_devueltos = 0;

        if !self.es_halted {
            let pc = self.reg.get_pc();
            c.reg.pc = pc; // Experimento zilog

            let (opc, bytes_consumidos) =
                self.analiza_opcode(pc as usize).expect("Rango de memoria excedido");

            bytes_consumidos_devueltos = bytes_consumidos;

            for n in 0..bytes_consumidos {
                c.bus.write_byte(pc + n as u16, self.mem.mem[pc as usize + n]);
            }

            //println_tee!("bytes consumidos = {}",bytes_consumidos);

            if DEBUG {
                println_tee!("Voy a ejecutar {:?} en direccion: {:04X}", self.format_opcode1(&opc), self.reg.pc);
            }
            if self.debug.debug_zilog {
                self.ex_zilog(c, true);
            }

            let pc = self
                .exec_with_offset(opc) //dbg!(opc))
                .unwrap_or(pc.wrapping_add(bytes_consumidos as u16));
            self.reg.set_pc(pc);

            if self.debug.debug_zilog {
                self.ex_zilog(c, false);
            }
        }
        // ojo BREAKPOINT         ***********
        if self.reg.pc == VALOR_BREAKPOINT || self.debug.hacer_debug {
            self.es_halted = true;   // ponerlo o quitarlo segun se quiera hacer paso a paso
            //println_tee!("acabado en breakpoint pc = {:04X}", self.registros.pc);
            //panic!("final");
        }
        bytes_consumidos_devueltos
    }

    // Funcion que ejecuta zilog
    fn ex_zilog(&mut self, c: &mut CPU, ejecuta: bool) {
        assert_eq!(c.reg.a, self.reg.a);

        assert_eq!(c.reg.flags.c, self.reg.get_flag(&StatusFlag::Carry));
        assert_eq!(c.reg.flags.z, self.reg.get_flag(&StatusFlag::Zero));
        assert_eq!(c.reg.flags.p, self.reg.get_flag(&StatusFlag::ParityOverflow));
        assert_eq!(c.reg.flags.s, self.reg.get_flag(&StatusFlag::Sign));
        assert_eq!(c.reg.flags.h, self.reg.get_flag(&StatusFlag::HalfCarry));
        assert_eq!(c.reg.flags.n, self.reg.get_flag(&StatusFlag::AddSubtract));

        assert_eq!(c.reg.b, self.reg.b);
        assert_eq!(c.reg.c, self.reg.c);
        assert_eq!(c.reg.d, self.reg.d);
        assert_eq!(c.reg.e, self.reg.e);
        assert_eq!(c.reg.h, self.reg.h);
        assert_eq!(c.reg.l, self.reg.l);

        assert_eq!(c.reg.pc, self.reg.pc);
        assert_eq!(c.reg.sp, self.reg.sp);
        assert_eq!(c.reg.get_ix(), self.reg.ix);
        assert_eq!(c.reg.get_iy(), self.reg.iy);

        assert_eq!(c.reg.r, self.reg.r);
        assert_eq!(c.reg.i, self.reg.i);

        //let hlzilog = c.bus.read_byte(c.reg.get_hl());
        let hlzilog = c.bus.read_byte(c.reg.get_hl());

        let hlmio = self.mem.mem[self.reg.get_reg16_big_endian(&R16::HL) as usize];

        // Registros prima
        assert_eq!(c.alt.a, self.reg.ap);
        let f = (c.alt.get_af() & 0b0000_0000_1111_1111) as u8;
        assert_eq!(f, self.reg.fp);
        assert_eq!(c.alt.b, self.reg.bp);
        assert_eq!(c.alt.c, self.reg.cp);
        assert_eq!(c.alt.d, self.reg.dp);
        assert_eq!(c.alt.e, self.reg.ep);
        assert_eq!(c.alt.h, self.reg.hp);
        assert_eq!(c.alt.l, self.reg.lp);

        assert_eq!(hlzilog, hlmio);

        if ejecuta {
            c.execute();
        }

        //dbg_hex!(c.reg.a);
    }

    // Analiza la instrucción de la CPU en la ubicación dada.
    // Si la ubicación existe en la memoria, devuelve el código de operación y el tamaño del código de operación en bytes
    // De lo contrario, devuelve None.
    // Pánico si no se encuentra un código de operación válido y la ubicación especificada
    pub fn analiza_opcode(&self, location: usize) -> Option<(Op, usize)> {
        let mem = self.mem.mem;
        let byte = match mem.get(location) {
            Some(byte) => *byte,
            None => return None,
        };

        let opcode_horizonte = [
            byte,
            mem.get(location + 1).map_or(0x00, |i| *i),
            mem.get(location + 2).map_or(0x00, |i| *i),
            mem.get(location + 3).map_or(0x00, |i| *i),
        ];

        Some(opcodes::opcode(opcode_horizonte))
    }

    // Inicia ejecucion.
    // El contador de programa es puesto a 0x0000, y las instrucciones son ejecutadas hasta encontrar HALT.
    // Si el programa no contiene HALT, el emulador simplemente continua hasta que acabe la memoria.
    pub fn run(
        &mut self,
        canvas: &mut Canvas<Window>,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
        c: &mut CPU, // de emulador zilog_z80
    ) {
        if !self.es_halted {
            if self.reg.pc == self.debug.breakpoint {
                self.debug.hacer_debug = true;
                //self.es_halted = true;
            }
            // experimento
            //self.debug.hacer_debug = true;

            //if self.registros.pc >= VALOR_BREAKPOINT {
            //if self.debug.hacer_debug {
            if self.debug.hacer_debug {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.clear();
                let _ = self.render_registros(canvas, font, texture_creator);

                let pc = self.reg.pc;

                //dbg!(pc);
                let _ = self.desensambla(pc, canvas, font, texture_creator);
                let _ = self.presenta_pantalla(canvas);
                let _ = self.colores_memoria(canvas);
                let _ = self.presenta_stack(canvas, font, texture_creator);
                let _ = self.presenta_out(canvas, font, texture_creator);

                canvas.present();
            }

            self.step(c);
        }
        // ojo BREAKPOINT         ***********
        //if self.reg.pc == VALOR_BREAKPOINT && self.debug.hacer_debug {
        // if let Some(dir_brk) = self.debug.breakpoint {
        //     if self.reg.pc == dir_brk && self.debug.hacer_debug {
        //         self.es_halted = true;   // ponerlo o quitarlo segun se quiera hacer paso a paso
        //         //println_tee!("acabado en breakpoint pc = {:04X}", self.registros.pc);
        //         //panic!("final");
        //     }
        // }

        //}
    }

    fn presenta_out(
        &self,
        canvas: &mut Canvas<Window>,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
    ) -> Result<(), String> {
        for (port, output_device) in &self.output_devices {
            // Primero obtenemos el Rc y luego el borrow
            let buffer_rc = output_device.get_buffer();
            let buffer = buffer_rc.borrow();

            for n in 0..buffer.len() {
                let puerto = format!("{:02X}", port);
                //let b = format!("{:02X}", buffer[n]);

                let surface1 = font
                    .render(&*puerto)
                    .solid(Color::WHITE)
                    .map_err(|e| e.to_string())?;
                let texture1 = texture_creator
                    .create_texture_from_surface(&surface1)
                    .map_err(|e| e.to_string())?;

                let (width1, height1) = surface1.size();

                //let y = 410 + n as i32 * 20 - self.registros.pc as i32 * 20;
                let y = 410;

                let rect1 = Rect::new(900, y, width1, height1);
                canvas.copy(&texture1, None, Some(rect1))?;

                // *******************************************************

                let dispositivo = format!("{:02X}", buffer[n]);
                let surface2 = font
                    .render(&*dispositivo)
                    .solid(Color::WHITE)
                    .map_err(|e| e.to_string())?;
                let texture2 = texture_creator
                    .create_texture_from_surface(&surface2)
                    .map_err(|e| e.to_string())?;

                let (width2, height2) = surface2.size();

                //let y = 410 + n as i32 * 20 - self.registros.pc as i32 * 20;
                let y = 410;
                let rect2 = Rect::new(950, y, width2, height2);
                canvas.copy(&texture2, None, Some(rect2))?;
            }
        }

        Ok(())
    }
    // Presenta un rectangulo de 512 x 128 pixeles de color que representan a la memoria
    fn presenta_stack(
        &self,
        canvas: &mut Canvas<Window>,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
    ) -> Result<(), String> {
        let sp = self.reg.sp;

        let mut kk = format!("SP: {:04X} --->", sp);
        kk = kk.to_owned() + self.byt_h(sp, 22).as_str();
        let txt = kk;

        let _ = self.rend_txt(canvas, font, texture_creator, &txt, 10, 776, Color::GREEN);

        Ok(())
    }

    // Presenta un rectangulo de 512 x 128 pixeles de color que representan a la memoria
    fn colores_memoria(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let ancho_bit_map = 512;
        let alto_bit_map = 128;
        let posx = 933;
        let posy = 10;
        let mut r;

        for y in 0..alto_bit_map {
            for x in 0..ancho_bit_map {
                r = self.mem.mem[x + y * ancho_bit_map];
                //dbg_hex!(x + y * alto_bit_map);
                if r == 0 {
                    canvas.set_draw_color(Color::RGB(0, 255, 0));
                } else {
                    canvas.set_draw_color(Color::RGB(r, 0, 0));
                }

                canvas.draw_point(Point::new((x + posx) as i32, (y + posy) as i32));
            }
        }

        Ok(())
    }

    fn render_registros(
        &self,
        canvas: &mut Canvas<Window>,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
    ) -> Result<(), String> {
        let f = self.reg.get_reg8(R8::F);
        let flags = format!(
            "Flags: S->{:01}   Z->{:01}   H->{:01}   PV->{:01}   N->{:01}   C->{:01}",
            (f >> 7) & 1, (f >> 6) & 1, (f >> 4) & 1, (f >> 2) & 1, (f >> 1) & 1, f & 1
        );

        let lines = [
            format!(
                "A: {:02X}  B: {:02X}  C: {:02X}  D: {:02X}  E: {:02X}  H: {:02X}  L: {:02X}  I: {:02X}  R: {:02X}",
                self.reg.get_reg8(R8::A),
                self.reg.get_reg8(R8::B),
                self.reg.get_reg8(R8::C),
                self.reg.get_reg8(R8::D),
                self.reg.get_reg8(R8::E),
                self.reg.get_reg8(R8::H),
                self.reg.get_reg8(R8::L),
                self.reg.get_reg8(R8::I),
                self.reg.get_reg8(R8::R)
            ),
            format!(
                "PC: {:04X}            IX: {:04X}  IY: {:04X}",
                self.reg.get_pc(),
                //self.registros.get_reg16_lit_endian(&R16::SP),
                self.reg.get_reg16_lit_endian(&R16::IX),
                self.reg.get_reg16_lit_endian(&R16::IY)
            ),
            flags,
            format!(
                "IFF1: {} IFF2: {}  IM: {}",
                self.reg.get_reg8(R8::IFF1),
                self.reg.get_reg8(R8::IFF2),
                self.reg.get_reg8(R8::IM)
            ),
        ];

        let mut y_offset = 800;
        for line in &lines {
            self.rend_txt(canvas, font, texture_creator, line, 10, y_offset, Color::WHITE)?;
            y_offset += 25;
        }

        Ok(())
    }

    // Para pantalla rectangular del spectrum
    fn presenta_pantalla(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        // Pruebas ***************
        // for n in 0x4000..0x4002 {
        //     dbg_hex!(self.memoria.memoria[n]);
        // }

        // 1. Definir el rectangulo de fondo que hara el color del borde
        let rect0 = Rect::new(10, 10, 256 * 3 + 20, 192 * 3 + 20);

        // 2. Establecer color de dibujo del rectangulo del borde
        let r = (self.color_borde & 0b0000_0100) >> 2;
        let g = (self.color_borde & 0b0000_0010) >> 1;
        let b = self.color_borde & 0b0000_0001;

        canvas.set_draw_color(Color::RGB(255 * r, 255 * g, 255 * b));

        // 3. Dibujar el rectángulo del borde (relleno)
        canvas.fill_rect(rect0)?;

        // *******************************************************************

        // 1. Definir el rectángulo (x, y, width, height)
        let rect1 = Rect::new(20, 20, 256 * 3, 192 * 3);

        // 2. Establecer color de dibujo (blanco)
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        // 3. Dibujar el rectángulo (relleno)
        canvas.fill_rect(rect1)?;

        // (Opcional) Si quieres solo el borde:
        // canvas.draw_rect(rect)?;
        Ok(())
    }

    fn desensambla(
        &mut self,
        dir_i: u16,
        cvas: &mut Canvas<Window>,
        font: &Font,
        textu_c: &TextureCreator<WindowContext>,
    ) -> Result<(), String> {
        let offset_d: u16 = 0;
        let mut d = dir_i;
        let mut linea_y = 0; // Contador de líneas para evitar espacios en blanco
        let mut y = 0;
        // while d < dir_i.wrapping_add(30) || y < 700 {
        while y < 840 {
            let (opc, bytes_consumidos) =
                self.analiza_opcode(d as usize).expect("Rango de memoria excedido");
            let dir_esta = offset_d.wrapping_add(d);
            //let y = 410 + d as i32 * 20 - self.registros.pc as i32 * 20;
            y = 410 + linea_y * 20; // Usa linea_y en lugar de d

            // Renderizar dirección, bytes e instrucción
            self.rend_txt(cvas, font, textu_c, &format!("{:04X}", d), 1060, y, Color::GREEN)?;
            self.rend_txt(cvas, font, textu_c, &self.byt_h(d, bytes_consumidos), 1120, y, Color::WHITE)?;
            self.rend_txt(cvas, font, textu_c, &self.fmat_op(&opc, dir_esta), 1250, y, Color::WHITE)?;

            d += bytes_consumidos as u16;
            linea_y += 1; // Incrementar la línea sin importar los bytes consumidos
        }
        Ok(())
    }

    fn rend_txt(
        &self,
        canvas: &mut Canvas<Window>,
        font: &Font,
        texture_creator: &TextureCreator<WindowContext>,
        text: &str,
        x: i32,
        y: i32,
        color: Color,
    ) -> Result<(), String> {
        let surface = font.render(text).solid(color).map_err(|e| e.to_string())?;
        let texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
        let (width, height) = surface.size();
        let rect = Rect::new(x, y, width, height);
        canvas.copy(&texture, None, Some(rect))?;
        Ok(())
    }

    fn byt_h(&self, d: u16, bytes_consumidos: usize) -> String {
        (0..bytes_consumidos)
            .map(|n| format!("{:02X} ", self.mem.mem[d.wrapping_add(n as u16) as usize]))
            .collect::<String>()
    }
}