mod run;
mod test;
pub mod io;

use crate::ops::Local16::*;
use crate::ops::R16::{AF, AFP};
use crate::ops::*;
use crate::sonido::suena;

use crate::{cpu, ops, VALOR_BREAKPOINT};
use dbg_hex::dbg_hex;
use fyrox_resource::core::num_traits::WrappingAdd;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Debug {
    pub hacer_debug: bool,
    pub breakpoint: u16,
    pub debug_zilog: bool,

}
impl Default for Debug {
    fn default() -> Self {
        Self {
            hacer_debug: false,
            breakpoint: VALOR_BREAKPOINT,
            debug_zilog: true,

        }
    }
}

pub struct Z80 {
    pub reg: cpu::reg::Registros,
    pub mem: cpu::mem::Memoria,

    pub es_halted: bool,
    // pub video_updated: bool,    // Indica si la memoria de video cambió
    pub input_devices: HashMap<u8, Box<dyn io::InputDevice>>,
    pub output_devices: HashMap<u8, Box<dyn io::OutputDevice>>,
    pub iff1: bool,
    pub iff2: bool,
    // Modo
    pub im: Option<u8>,
    // Interrupcion enmascarable
    pub int: Option<u8>,
    // Interrupcion no enmascarable
    nmi: bool,
    pub debug: Debug,
    //pub ula: Ula,
    pub color_borde: u8,
}

impl Default for Z80 {
    fn default() -> Self {
        let mut registros = cpu::reg::Registros::default();
        registros.set_reg16_lit_endian(&R16::SP, cpu::mem::MEMORY_SIZE as u16);
        Self {
            reg: registros,
            mem: cpu::mem::Memoria::default(),
            es_halted: true,
            // video_updated: false,
            input_devices: HashMap::new(),
            output_devices: HashMap::new(),
            iff1: false,
            iff2: false,
            im: None,
            int: None,
            nmi: false,
            //breakpoint: VALOR_BREAKPOINT,
            debug: Debug::default(),
            //ula: Ula::new(),
            color_borde: 0,
        }
    }
}

// impl PartialEq<Local16> for &Local16 {
//     fn eq(&self, other: &Local16) -> bool {
//         todo!()
//     }
// }

impl Z80 {
    //const ONE_IMM: Localizacion8 = Localizacion8::Inmediato(1);
    //const ACC: Localizacion8 = Localizacion8::Reg(Reg8::A);
    //const HL_INDIRECT: Localizacion8 = Localizacion8::RegIndirecto(Reg16::HL);

    // Ejecuta una instruccion.
    // El registro PC no es incrementado
    // pub fn exec(&mut self, op: Op) {
    //     let _ = self.exec_with_offset(op);
    // }

    // Crea una peticion de interrupcion enmascarable
    pub fn int_request(&mut self, byte: u8) {
        self.int = Some(byte);
    }

    // Create a una peticion de interrupcion no enmascarable
    pub fn nmi_request(&mut self) {
        self.nmi = true;
    }
    pub fn exec_with_offset(&mut self, op: Op) -> Option<u16> {
        match op {
            //Op::ADC16(dst, src) => self.add16_le(&dst, &src, true),
            Op::ADC16BIG(dst, src) => self.add16_be(&dst, &src, true, true),
            //Op::ADD16(dst, src) => self.add16_le(&dst, &src, false),
            //Op::ADD16(dst, src) => self.add16_le(&dst, &src, false),
            Op::ADD16BIG(dst, src) => self.add16_be(&dst, &src, false, false),
            //Op::ADD16(dst, src) => self.add16_le(&dst, &src, false),
            Op::ADD8(dst, src) => self.add(&dst, &src, false),
            Op::ADD8BIG(dst, src) => self.add_big(&dst, &src, false),
            Op::ADC8(dst, src) => self.add(&dst, &src, true),
            Op::ADC8BIG(dst, src) => self.add_big(&dst, &src, true),
            //Op::LD16(dst, src) => self.set_loc16_lit_endian(&dst, self.get_loc16_big_endian(&src)),
            Op::AND(src) => self.bool_op(&src, |d, s| d & s, true),
            Op::ANDBIG(src) => self.bool_op_big(&src, |d, s| d & s, true),
            Op::BIT(b, loc) => self.get_bit(b, &loc),
            Op::CALL(cond, addr) => return self.call(cond, addr),
            Op::CCF => self.toggle_carry(),
            Op::CP(src) => self.resta(&Local8::Reg8(R8::A), &src, false, false, true),
            Op::CPBIG(src) => self.resta_big(&Local8::Reg8(R8::A), &src, false, false, true),
            Op::CPD(dst, src) => self.cpd(&dst, &src),
            Op::CPI => self.cpi(),
            Op::CPIR => self.cpir(),
            Op::CPL => self.complemento(),
            Op::DAA => self.daa(),
            //Op::DEC8(dst) => self.resta(&dst, &Local8::Inmediato8(1), false, true),
            Op::DEC8(dst) => self.resta(&dst, &Local8::Inmediato8(1), false, true, false),
            Op::DEC8BIG(dst) => self.resta_big(&dst, &Local8::Inmediato8(1), false, true, false),
            Op::DEC16(dst, src) => self.dec16(&dst, &src),
            Op::DI => self.di(),
            Op::DJNZ(offset) => return self.decrement_jump(offset),
            Op::EI => self.ei(),
            Op::EX => self.ex(),
            Op::EX16 => self.intercambia16(&Reg(AF), &Reg(AFP)),
            Op::EXSPHL => self.ex_sp_hl(),
            Op::EXSPIX => self.ex_sp_ix(),
            Op::EXSPIY => self.ex_sp_iy(),
            Op::EXX => self.exx(),
            Op::HALT => self.es_halted = true,
            Op::IM(b) => self.im(b),
            Op::IN(dst, src_port) => self.read_in(&src_port, &dst),
            Op::INFLAGS(dst, src_port) => self.read_in_flags(&src_port, &dst),
            Op::INC16(dst, src) => self.inc16(&dst, &src),
            Op::INC8(dst) => self.add8(&dst, &Local8::Inmediato8(1), false, false),
            Op::INC8BIG(dst) => self.add8_big(&dst, &Local8::Inmediato8(1), false),
            Op::JP(cond, addr) => return self.jump_cond(cond, &addr),
            Op::JPHL(loc) => return self.jumphl(&loc),
            Op::JR(cond, offset) => return self.jump_relative(cond, offset),
            //Op::LD8(dst, src) => self.set_loc8_lit_endian(&dst, self.get_loc8_lit_endian(&src)),
            Op::LD8BIG(dst, src) => self.set_loc8_big_endian(&dst, self.get_loc8_big_endian(&src)),
            Op::LD8R(r) => self.ld8r(r),
            Op::LDI => self.ldi(),
            Op::LDDR => self.lddr(),
            Op::LDIR => self.ldir(),
            Op::LD16BIG(dst, src) => self.set_loc16_big_endian(&dst, self.get_loc16_big_endian(&src)),
            Op::MLT(Reg(r)) => self.mult(r),
            Op::NEG => self.negate(),
            Op::OUTI => self.outi(),
            //Op::OUT(src, dst_port) => self.write_out_le(&dst_port, &src),
            Op::OUTBE(src, dst_port) => self.write_out_be(&dst_port, &src),
            Op::OR(src) => self.bool_op(&src, |d, s| d | s, false),
            Op::ORBIG(src) => self.bool_op_big(&src, |d, s| d | s, false),
            Op::POP(dst) => self.pop(&dst),
            Op::PUSH(src) => self.push(&src),

            Op::RLA => self.rota_izquierda_thru_acc(&Local8::Reg8(R8::A), false),
            Op::RLCA => self.rota_izquierda(&Local8::Reg8(R8::A), false),
            Op::RRCA => self.rota_derecha(&Local8::Reg8(R8::A), false),

            Op::RRA => self.rota_derecha_thru_acc(&Local8::Reg8(R8::A), false),

            // Op::RES(b, loc) => self.reset_bit_lit(b, &loc),
            Op::RESBIG(b, loc) => self.reset_bit_big(b, &loc),
            Op::RET(cond) => return self.ret(cond),
            Op::RETBIG(cond) => return self.ret_big(cond),
            //  Op::RET(cond) => return self.ret(cond),
            Op::RETI => return self.reti(),
            Op::RETN => return self.retn(),
            Op::RL(reg) => self.rota_izquierda_thru_acc(&reg, true),
            Op::RLBIG(reg) => self.rota_izquierda_thru_acc_big(&reg, true),
            Op::RLC(reg) => self.rota_izquierda(&reg, true),
            Op::RLD => self.rotate_nibble_left(),
            Op::RLCBIG(reg) => self.rota_izquierda_big(&reg, true),
            Op::RR(reg) => self.rota_derecha_thru_acc(&reg, true),
            Op::RRBIG(reg) => self.rota_derecha_thru_acc_big(&reg, true),
            Op::RRC(reg) => self.rota_derecha(&reg, true),
            Op::RRCBIG(reg) => self.rota_derecha_big(&reg, true),

            Op::RRD => self.rotate_nibble_right(),

            Op::RST(dst) => return self.jump_incondicional_con_push_lit(&dst), // Lo pongo yo
            Op::RSTBIG(dst) => return self.jump_incondicional_con_push_big(&dst), // Lo pongo yo
            Op::SCF => self.set_carry(),
            Op::SUB8(dst, src) => self.resta(&dst, &src, false, true, true),
            Op::SUB8BIG(dst, src) => self.resta_big(&dst, &src, false, true, true),
            Op::SBC(dst, src) => self.resta(&dst, &src, true, true, true),
            Op::SBC16BIG(dst, src) => self.resta16_big(&dst, &src, true, true),
            Op::SBCBIG(dst, src) => self.resta_big(&dst, &src, true, true, true),

            Op::SLA(loc) => self.shift_left(&loc),
            Op::SLABIG(loc) => self.shift_left_big(&loc),
            Op::SRA(loc) => self.shift_right(&loc, true),
            Op::SRABIG(loc) => self.shift_right_big(&loc, true),
            Op::SLL(loc) => self.shift_left_sll_lit(&loc), // SLL no documentada ?????
            Op::SLLBIG(loc) => self.shift_left_sll_big(&loc), // SLL no documentada ?????
            Op::SRL(loc) => self.shift_right(&loc, false),
            Op::SRLBIG(loc) => self.shift_right_big(&loc, false),

            // Op::SET(b, loc) => self.set_bit_lit(b, &loc),
            Op::SETBIG(b, loc) => self.set_bit_big(b, &loc),

            Op::XOR(src) => self.bool_op(&src, |d, s| d ^ s, false),
            Op::XORBIG(src) => self.bool_op_big(&src, |d, s| d ^ s, false),

            _ => ()
        };
        None
    }

    // ED A9  Compara A con (HL)    HL <- HL-1, BC <- BC-1
    // S afectado    Z=1 si A = (HL)     H=1 si borrow del bit 4   P/V=1 si BC-1 != 0  N=1   C no afectado
    pub fn cpd(
        &mut self,
        dst: &Local8,
        src: &Local8,
    ) {
        let v1 = self.get_loc8_big_endian(dst);
        let v2 = self.get_loc8_big_endian(src);

        let (sum, _) = v1.overflowing_sub(v2);

        // Bandera de suma/resta
        self.reg
            .set_flag(&StatusFlag::AddSubtract, true);

        self.reg
            .set_flag(&StatusFlag::HalfCarry, (v1 & 0x0F) < (v2 & 0x0F));

        // Bandera de cero
        self.reg
            .set_flag(&StatusFlag::Zero, sum == 0);

        // Bandera de signo
        self.reg
            .set_flag(&StatusFlag::Sign, (sum & 0x80) != 0);

        // Solo para el caso de ED A9 CPD  ya que en esta instruccion P/V=1 si BC-1 != 0
        if self.reg.get_reg16_big_endian(&R16::BC).wrapping_sub(1) != 0 {
            self.reg.set_flag(&StatusFlag::ParityOverflow, true);
        } else {
            self.reg.set_flag(&StatusFlag::ParityOverflow, false);
        }

        // DEcrementa HL
        self.reg.set_reg16_big_endian(&R16::HL, self.reg.get_reg16_big_endian(&R16::HL).wrapping_sub(1));
        // DEcrementa BC
        self.reg.set_reg16_big_endian(&R16::BC, self.reg.get_reg16_big_endian(&R16::BC).wrapping_sub(1));
    }
    pub fn mult(&mut self, dst: R16) {
        // ED 4C   Intruccion no documentada   coge un registro de 16 bits multiplica entre si los
        // registros componentes y pone el resultado en el registro de 16 bits. No afecta flags

        let pareja = self.reg.get_reg16_big_endian(&dst);
        let alto = (pareja & 0xFF00) >> 8;
        let bajo = pareja & 0x00FF;
        let resultado = alto.wrapping_mul(bajo);
        self.reg.set_reg16_big_endian(&dst, resultado);
    }
    pub fn outi(&mut self) {
        // ED A3 (C) ← (HL), B ← B – 1, HL ← HL + 1
        // Z=1 si B – 1 = 0  N=1 Carry afectado, (no lo dice la documentacion)   H, PV desconocido

        // En C se almacena temporalmente lo que hay en (HL)
        // Luego, después de que B se decrementa, C se coloca en la mitad inferior (A0 a A7) del bus de direcciones
        // para seleccionar el dispositivo de E/S en uno de los 256 puertos posibles.
        // B se puede utilizar como un contador de bytes y su valor decrementado se
        // coloca en la mitad superior (A8 a A15) del bus de direcciones. El byte que se va a generar se coloca
        // en el bus de datos y se escribe en un dispositivo periférico seleccionado.
        // Finalmente, el par de registros HL se incrementa.

        // Decrementamos B
        let bm = self.reg.get_reg8(R8::B).wrapping_sub(1);
        self.reg.set_reg8(R8::B, bm);

        // Ponemos lo que hay en C en el port 0x__FE
        //self.registros.set_reg8(R8::C, 0xFE);

        // valor a enviar el u8 de la direccion de memoria que apunta HL
        let src = &Local8::RegIndirecto8(R16::HL);

        // Puerto de destino lo que dice C
        let dst_port = &Local8::Reg8(R8::C);

        //self.write_out_le(&dst_port, &src);
        self.write_out_be(&dst_port, &src);

        // Incrementamos HL
        let hl1 = self.reg.get_reg16_big_endian(&R16::HL).wrapping_add(1);
        self.reg.set_reg16_big_endian(&R16::HL, hl1);
    }
    // Es igual que LD8 pero afecta a los flags, se usa solo para ED 5F   A <- R    y    ED 57 A <-I
    // si es R reg=1     si es I reg=2  TODO--> podria ser booleana, pero no se si hbara mas instrucciones
    pub fn ld8r(&mut self, reg: u8) {
        let r = self.reg.get_reg8(R8::R);
        let i = self.reg.get_reg8(R8::I);
        let a = R8::A;

        // ED 5F   A <- R
        // Carry no afectado, S=1 si R es negativo  Z=1 si R=0  H=0  PV contiene el contenido de IFF2  N=0
        // Si ocurre una interrupcion durante la ejecucion de esta instruccion el flag parity=0
        if reg == 1 {
            self.reg.set_reg8(a, r);
            self.reg
                .set_flag(&StatusFlag::Sign, (r & 0b1000_0000) != 0); // S
            self.reg
                .set_flag(&StatusFlag::Zero, r == 0); // Z
        }
        // ED 57   A <- I
        // Carry no afectado, S=1 si I es negativo  Z=1 si I=0  H=0  PV contiene el contenido de IFF2  N=0
        // Si ocurre una interrupcion durante la ejecucion de esta instruccion el flag parity=0
        if reg == 2 {
            self.reg.set_reg8(a, i);
            self.reg
                .set_flag(&StatusFlag::Sign, (i & 0b1000_0000) != 0); // S
            self.reg
                .set_flag(&StatusFlag::Zero, i == 0); // Z
        }

        self.reg
            .set_flag(&StatusFlag::HalfCarry, false); // H
        self.reg
            .set_flag(&StatusFlag::AddSubtract, false); // N
        self.reg
            .set_flag(&StatusFlag::ParityOverflow, self.iff2); // PV

    }

    pub fn ldir(&mut self) {
        // 0xED 0xB8 (DE) ← (HL), DE ← DE – 1, HL ← HL – 1, BC ← BC – 1
        // S, Z no afectados    H=0  PV=0  N=0
        while self.get_loc16_big_endian(&Reg(R16::BC)) != 0 {
            let bc = self.get_loc16_big_endian(&Reg(R16::BC));
            //dbg_hex!(bc);
            let de = self.get_loc16_big_endian(&Reg(R16::DE));
            //dbg_hex!(de);
            let hl = self.get_loc16_big_endian(&Reg(R16::HL));
            //dbg_hex!(hl);

            self.mem.mem[de as usize] = self.mem.mem[hl as usize];

            self.reg.set_reg16_big_endian(&R16::DE, de.wrapping_add(1));
            self.reg.set_reg16_big_endian(&R16::HL, hl.wrapping_add(1));
            self.reg.set_reg16_big_endian(&R16::BC, bc.wrapping_sub(1));

            // Volvemos hacia atras para volver a ejecutarlo
            //self.registros.set_pc(self.registros.pc.wrapping_sub(2));
        }

        self.reg
            .set_flag(&StatusFlag::AddSubtract, false); // N
        self.reg
            .set_flag(&StatusFlag::HalfCarry, false); // H
        self.reg
            .set_flag(&StatusFlag::ParityOverflow, false); // PV

    }
    pub fn lddr(&mut self) {
        // 0xED 0xB8 (DE) ← (HL), DE ← DE – 1, HL ← HL – 1, BC ← BC – 1
        // S, Z no afectados    H=0  PV=0  N=0
        while self.get_loc16_big_endian(&Reg(R16::BC)) != 0 {
            let bc = self.get_loc16_big_endian(&Reg(R16::BC));

            let de = self.get_loc16_big_endian(&Reg(R16::DE));

            let hl = self.get_loc16_big_endian(&Reg(R16::HL));

            self.mem.mem[de as usize] = self.mem.mem[hl as usize];

            self.reg.set_reg16_big_endian(&R16::DE, de.wrapping_sub(1));
            self.reg.set_reg16_big_endian(&R16::HL, hl.wrapping_sub(1));
            self.reg.set_reg16_big_endian(&R16::BC, bc.wrapping_sub(1));

            // Volvemos hacia atras para volver a ejecutarlo
            //dbg_hex!(self.registros.pc);
            //self.registros.set_pc(self.registros.pc.wrapping_sub(2));
            //dbg_hex!(self.registros.pc);
        }

        self.reg
            .set_flag(&StatusFlag::AddSubtract, false); // N
        self.reg
            .set_flag(&StatusFlag::HalfCarry, false); // H
        self.reg
            .set_flag(&StatusFlag::ParityOverflow, false); // PV

    }
    pub fn cpi(&mut self) {
        // ED A1 A-(HL)    HL<-HL+1    BC<-BC-1
        // Carry no afectado N=1     PV=1 si BC-1!=0     H=1 si hay borrow desde el bit 4
        // Z = 1 si A = (HL) ,S afetado por el signo

        let kk1h = ((self.reg.h as u16) << 8) | (self.reg.l as u16);

        // Al llamar a resta se actualizan los flags
        self.resta_big(&Local8::Reg8(R8::A),
                       &Local8::RegIndirecto8(R16::HL), false, false, false);

        // Calculo de flag Zero antes de incrementar HL
        let a = self.get_loc8_big_endian(&Local8::Reg8(R8::A));
        let hl0 = self.get_loc8_big_endian(&Local8::RegIndirecto8(R16::HL));
        if a == hl0 {
            self.reg.set_flag(&StatusFlag::Zero, true);
        } else {
            self.reg.set_flag(&StatusFlag::Zero, false);
        }

        // Calculo del flag ParytyOverflow antes de incrementar BC
        if self.get_loc16_big_endian(&Reg(R16::BC)).wrapping_sub(1) != 0 {
            self.reg.set_flag(&StatusFlag::ParityOverflow, true);
        } else {
            self.reg.set_flag(&StatusFlag::ParityOverflow, false);
        }

        // Incrementamos y decrementamos parejas de registros
        self.reg
            .set_reg16_big_endian(&R16::HL, self.reg
                .get_reg16_big_endian(&R16::HL).wrapping_add(1));

        self.reg
            .set_reg16_big_endian(&R16::BC, self.reg
                .get_reg16_big_endian(&R16::BC).wrapping_sub(1));
    }

    pub fn cpir(&mut self) {
        while self.get_loc16_big_endian(&Reg(R16::BC)) != 0 {

            //cont += 1;
            //dbg!(cont);
            //dbg_hex!(get_bc_test_big(&mut z80, &mut c));

            //cpi();
            //ejecutar(&mut z80, 0xED, 0xA1, 0, 0, &mut c);

            // To esto es igual que cpi() *****************:

            let kk1h = ((self.reg.h as u16) << 8) | (self.reg.l as u16);

            // Al llamar a resta se actualizan los flags
            self.resta_big(&Local8::Reg8(R8::A),
                           &Local8::RegIndirecto8(R16::HL), false, false, false);

            // Calculo de flag Zero antes de incrementar HL
            let a = self.get_loc8_big_endian(&Local8::Reg8(R8::A));
            let hl0 = self.get_loc8_big_endian(&Local8::RegIndirecto8(R16::HL));
            if a == hl0 {
                self.reg.set_flag(&StatusFlag::Zero, true);
            } else {
                self.reg.set_flag(&StatusFlag::Zero, false);
            }

            // Calculo del flag ParytyOverflow antes de incrementar BC
            if self.get_loc16_big_endian(&Reg(R16::BC)).wrapping_sub(1) != 0 {
                self.reg.set_flag(&StatusFlag::ParityOverflow, true);
            } else {
                self.reg.set_flag(&StatusFlag::ParityOverflow, false);
            }

            // Incrementamos y decrementamos parejas de registros
            self.reg
                .set_reg16_big_endian(&R16::HL, self.reg
                    .get_reg16_big_endian(&R16::HL).wrapping_add(1));

            self.reg
                .set_reg16_big_endian(&R16::BC, self.reg
                    .get_reg16_big_endian(&R16::BC).wrapping_sub(1));

            // ********************************************+

            if self.reg.get_flag(&StatusFlag::Zero) {
                break;
            }
        }
    }

    pub fn ldi(&mut self) {
        // 0xED 0xA0 (DE) ← (HL), DE ← DE + 1, HL ← HL + 1, BC ← BC – 1
        let bc = self.get_loc16_big_endian(&Reg(R16::BC));
        //dbg_hex!(bc);
        let de = self.get_loc16_big_endian(&Reg(R16::DE));
        //dbg_hex!(de);
        let hl = self.get_loc16_big_endian(&Reg(R16::HL));
        //dbg_hex!(hl);

        self.mem.mem[de as usize] = self.mem.mem[hl as usize];

        self.reg.set_reg16_big_endian(&R16::DE, de.wrapping_add(1));
        self.reg.set_reg16_big_endian(&R16::HL, hl.wrapping_add(1));
        self.reg.set_reg16_big_endian(&R16::BC, bc.wrapping_sub(1));

        self.reg
            .set_flag(&StatusFlag::AddSubtract, false); // N

        //if self.get_loc16_big_endian(&Reg(R16::BC)) == 0 {
        if self.get_loc16_big_endian(&Reg(R16::BC)).wrapping_sub(1) != 0 {
            self.reg.set_flag(&StatusFlag::ParityOverflow, true);
        }
    }
    // dst - src (si incluye_carry = true, tambien resta 1) -> dst hecho por mi
    pub fn resta_index(
        &mut self,
        dst: &Local8,
        src: &Local8,
        incluye_carry: bool,
        store_result: bool,
    ) {
        let v1 = self.get_loc8_lit_endian(dst);
        let mut v2 = self.get_loc8_lit_endian(src);

        if incluye_carry && self.reg.get_flag(&StatusFlag::Carry) {
            v2 = v2.wrapping_add(1);
        }

        let (sum, ov) = v1.overflowing_sub(v2);
        if store_result {
            self.set_loc8_lit_endian(dst, sum);
        }

        // Bandera de acarreo (borrow total)
        self.reg.set_flag(&StatusFlag::Carry, ov);

        // Bandera de suma/resta
        self.reg
            .set_flag(&StatusFlag::AddSubtract, true);

        // Bandera de desbordamiento (overflow en resta con signo)
        let v1_sign = (v1 & 0x80) != 0;
        let v2_sign = (v2 & 0x80) != 0;
        let sum_sign = (sum & 0x80) != 0;
        let overflow_signed = (v1_sign != v2_sign) && (v1_sign != sum_sign);
        self.reg
            .set_flag(&StatusFlag::ParityOverflow, overflow_signed);

        // Bandera de medio acarreo (borrow del bit 4)
        // self.registros
        //     .set_flag(&StatusFlag::HalfCarry, Self::is_borrow(v1, v2, 3));
        self.reg
            .set_flag(&StatusFlag::HalfCarry, (v1 & 0x0F) < (v2 & 0x0F));

        // Bandera de cero
        self.reg
            .set_flag(&StatusFlag::Zero, sum == 0);

        // Bandera de signo
        self.reg
            .set_flag(&StatusFlag::Sign, (sum & 0x80) != 0);
    }
    fn im(&mut self, modo: Option<u8>) {
        self.im = modo;
    }
    fn retn(&mut self) -> Option<u16> {
        Some(self.pop_val_little_endian())
    }
    fn negate(&mut self) {
        let reg_a = R8::A;
        let a = self.reg.get_reg8(reg_a);

        let complement = (1_u16 << 8) - u16::from(a);
        let [result, _] = complement.to_le_bytes();
        // let result = (!a) + 1
        self.reg.set_reg8(reg_a, result);

        self.reg
            .set_flag(&StatusFlag::Sign, (result & 0b1000_0000) != 0);
        self.reg
            .set_flag(&StatusFlag::Zero, result == 0);
        // self.registros
        //     .set_flag(&StatusFlag::HalfCarry, Self::is_borrow(0, a, 3)); // verifica prestamo en los bits 0-3
        // Segun DS la bandera H en NEG se activa si el nibble bajo de A no es cero,
        // no por una comparación decimal directa.
        self.reg
            .set_flag(&StatusFlag::HalfCarry,
                      (a & 0x0F) != 0,  // ← H=1 si el nibble bajo de A ≠ 0 (segun DS)
            );
        self.reg
            .set_flag(&StatusFlag::ParityOverflow, a == 0x80);
        self.reg
            .set_flag(&StatusFlag::AddSubtract, true);
        self.reg
            .set_flag(&StatusFlag::Carry, a != 0x00);
    }

    // fn is_borrow(min: u8, sub: u8, bit: u8) -> bool {
    //     let mask = (1 << (bit + 1)) - 1;
    //     (min & mask) < (sub & mask)
    // }
    fn di(&mut self) {
        self.iff1 = false;
        self.iff2 = false;
    }
    fn ei(&mut self) {
        self.iff1 = true;
        self.iff2 = true;
    }
    fn rotate_nibble_left(&mut self) {
        let acc = self.get_loc8_lit_endian(&Local8::Reg8(R8::A));
        let hl = self.get_loc8_lit_endian(&Local8::RegIndirecto8(R16::HL));

        let acc2 = (acc & 0xf0) | (hl >> 4);
        let hl2 = (hl << 4) | (acc & 0x0f);

        self.set_loc8_lit_endian(&Local8::Reg8(R8::A), acc2);
        self.set_loc8_lit_endian(&Local8::RegIndirecto8(R16::HL), hl2);

        self.reg.set_flag(&StatusFlag::HalfCarry, false);
        self.reg
            .set_flag(&StatusFlag::AddSubtract, false);
        self.paridad_flags(acc2);
    }

    fn rotate_nibble_right(&mut self) {
        let acc = self.get_loc8_lit_endian(&Local8::Reg8(R8::A));
        let hl = self.get_loc8_lit_endian(&Local8::RegIndirecto8(R16::HL));

        let acc2 = (acc & 0xf0) | (hl & 0x0f);
        let hl2 = (hl >> 4) | ((acc & 0x0f) << 4);

        self.set_loc8_lit_endian(&Local8::Reg8(R8::A), acc2);
        self.set_loc8_lit_endian(&Local8::RegIndirecto8(R16::HL), hl2);

        self.reg.set_flag(&StatusFlag::HalfCarry, false);
        self.reg
            .set_flag(&StatusFlag::AddSubtract, false);
        self.paridad_flags(acc2);
    }
    // Intercambia (SP) con HL
    fn ex_sp_hl(&mut self) {
        let sp = self.reg.sp as usize;
        // crea variables intermedias
        let h = self.reg.h;
        let l = self.reg.l;

        self.reg.l = self.mem.mem[sp];
        self.reg.h = self.mem.mem[sp.wrapping_add(1)];

        self.mem.mem[sp.wrapping_add(1)] = h;
        self.mem.mem[sp] = l;
    }
    // Intercambia (SP) con IX
    fn ex_sp_ix(&mut self) {

        // crea variables intermedias

        // Obtenemos las partes de IX como valores intermedios
        let ix_mem_bajo = (self.reg.ix & 0x00FF) as u8;
        let ix_mem_alto = ((self.reg.ix & 0xFF00) >> 8) as u8;

        // Obtenemos los bytes que apunta el SP
        let sp = self.reg.sp as usize;
        let sp_mem_bajo = self.mem.mem[sp];
        let sp_mem_alto = self.mem.mem[sp.wrapping_add(1)];

        self.mem.mem[sp.wrapping_add(1)] = ix_mem_alto;
        self.mem.mem[sp] = ix_mem_bajo;

        self.reg.ix = ((sp_mem_alto as u16) << 8) | sp_mem_bajo as u16;
    }
    // Intercambia (SP) con IX
    fn ex_sp_iy(&mut self) {

        // crea variables intermedias

        // Obtenemos las partes de IY como valores intermedios
        let iy_mem_bajo = (self.reg.iy & 0x00FF) as u8;
        let iy_mem_alto = ((self.reg.iy & 0xFF00) >> 8) as u8;

        // Obtenemos los bytes que apunta el SP
        let sp = self.reg.sp as usize;
        let sp_mem_bajo = self.mem.mem[sp];
        let sp_mem_alto = self.mem.mem[sp.wrapping_add(1)];

        self.mem.mem[sp.wrapping_add(1)] = iy_mem_alto;
        self.mem.mem[sp] = iy_mem_bajo;

        self.reg.iy = ((sp_mem_alto as u16) << 8) | sp_mem_bajo as u16;
    }
    // Intercambia DE con HL
    fn ex(&mut self) {
        // crea variables intermedias
        let d = self.reg.d;
        let e = self.reg.e;

        self.reg.d = self.reg.h;
        self.reg.e = self.reg.l;

        self.reg.h = d;
        self.reg.l = e;
    }
    // Intercambia BC DE HL con BC' DE' HL'
    fn exx(&mut self) {
        // crea variables intermedias
        let b = self.reg.b;
        let c = self.reg.c;
        let d = self.reg.d;
        let e = self.reg.e;
        let h = self.reg.h;
        let l = self.reg.l;

        self.reg.b = self.reg.bp;
        self.reg.c = self.reg.cp;
        self.reg.d = self.reg.dp;
        self.reg.e = self.reg.ep;
        self.reg.h = self.reg.hp;
        self.reg.l = self.reg.lp;

        self.reg.bp = b;
        self.reg.cp = c;
        self.reg.dp = d;
        self.reg.ep = e;
        self.reg.hp = h;
        self.reg.lp = l;
    }
    fn _daa(&mut self) {
        // 0x27 Ajusta el registro A después de una operación aritmética para obtener el resultado correcto en BCD.
        // - Si el nibble inferior > 9 o el flag Half-Carry está activo, se suma 0x06.
        // - Si el nibble superior > 9 o el flag Carry está activo, se suma 0x60.
        // Si A es 0 despues de la operacion ->Z=1
        // Si A despues de la operacion tiene paridad ->P/V=1
        // N no afectado
        // Si el bit 7 de A es 1 despues de la operacion ->S=1

        // Flags actualizados: Half-Carry (H), Carry (C)
        //  daa              27         4 z-0x decimal adjust akku (Creada por mi)

        let a: u8 = self.reg.a;
        let mut ajuste: u8 = 0; // Cantidad a ajustar (0x06 o 0x60)
        let mut nuevo_carry: bool = false; // Para determinar si se activa el flag Carry

        // Obtiene los flags relevantes y el valor actual de A
        let half_carry = self.reg.get_flag(&StatusFlag::HalfCarry);
        let negativo = self.reg.get_flag(&StatusFlag::AddSubtract);
        let carry = self.reg.get_flag(&StatusFlag::Carry);

        // --- Ajuste del nibble inferior ---
        // Si el nibble inferior > 9 o el flag Half-Carry está activo, sumar 0x06
        if half_carry || (!negativo && (a & 0xF) > 9) {
            ajuste = 0x06;
        }

        // --- Ajuste del nibble superior ---
        // Si el nibble superior > 9 o el flag Carry está activo, sumar 0x60
        if carry || (!negativo && a > 0x99) {
            ajuste |= 0x60;
            nuevo_carry = true;
        }

        // --- Aplicar el ajuste a A ---
        // Si la operación anterior fue una resta (N=1), se resta el ajuste; de lo contrario, se suma
        self.reg.a = if negativo {
            a.wrapping_sub(ajuste)
        } else {
            a.wrapping_add(ajuste)
        };

        // --- Actualización de flags ---
        // Zero (Z): activo si A == 0
        self.reg.set_flag(&StatusFlag::Zero, self.reg.a == 0);
        // Half-Carry (H): siempre desactivado tras DAA
        self.reg.set_flag(&StatusFlag::HalfCarry, false);
        // Carry (C): se actualiza si se generó durante el ajuste
        self.reg.set_flag(&StatusFlag::Carry, carry || nuevo_carry);

        // Calcula la paridad del valor: true si el número de bits cero es par.
        let parity = self.reg.a.count_zeros() % 2 == 0;
        self.reg.set_flag(&StatusFlag::ParityOverflow, parity);
    }
    fn daa2(&mut self) {
        let a = self.reg.a;
        let mut ajuste = 0; // Cantidad a ajustar (0x06 o 0x60)

        let mut carry = self.reg.get_flag(&StatusFlag::Carry);
        let es_resta = self.reg.get_flag(&StatusFlag::AddSubtract);

        let original_h = self.reg.get_flag(&StatusFlag::HalfCarry); // Guardar HF original

        let mut lower_ajustado = false; // Indica si se aplicó ajuste al nibble inferior (0x06)
        let mut upper_ajustado = false; // Indica si se aplicó ajuste al nibble superior (0x60)

        // 1. Determinar ajuste para nibble inferior
        let lower = a & 0x0F;
        if original_h || (!es_resta && lower > 9) || (es_resta && lower > 9) {
            ajuste += 0x06;
            lower_ajustado = true; // Marcar que se aplicó corrección al nibble inferior
        }

        // 2. Determinar ajuste para nibble superior
        let upper = (a >> 4) & 0x0F;
        if carry || upper > 9 || (upper >= 9 && lower > 9) {
            ajuste += 0x60;
            carry = true;
            upper_ajustado = true;
        }

        // 3. Calcular valor temporal de A después del ajuste
        let temp_a = if es_resta {
            a.wrapping_sub(ajuste)
        } else {
            a.wrapping_add(ajuste)
        };

        // 4. Calcular nuevo HF según modo (suma/resta)
        let new_h = if es_resta {
            // Para resta: HF' = 1 solo si HF original era 1 y el nibble inferior ajustado <= 5
            original_h && ((temp_a & 0x0F) <= 5)
        } else {
            // Para suma: HF' = 1 si se aplicó ajuste al nibble inferior
            lower_ajustado
        };

        // 5. Aplicar ajuste y actualizar flags
        self.reg.a = temp_a;
        self.reg.set_flag(&StatusFlag::Carry, carry);
        self.reg.set_flag(&StatusFlag::HalfCarry, new_h); // Usar new_h corregido
        self.reg.set_flag(&StatusFlag::Zero, self.reg.a == 0);
        self.reg.set_flag(&StatusFlag::Sign, (self.reg.a & 0x80) != 0);
        self.reg.set_flag(&StatusFlag::ParityOverflow, (self.reg.a.count_ones() % 2) == 0);
    }

    fn daa(&mut self) {
        let mut t = 0;
        let lsb = self.reg.a & 0x0F;
        let es_resta = self.reg.get_flag(&StatusFlag::AddSubtract);
        let carry_original = self.reg.get_flag(&StatusFlag::Carry);
        let h_original = self.reg.get_flag(&StatusFlag::HalfCarry);

        // Determinar ajustes necesarios
        if h_original || (lsb > 9) {
            t += 1;
        }

        if carry_original || (self.reg.a > 0x99) {
            t += 2;
            self.reg.set_flag(&StatusFlag::Carry, true);
        } else {
            self.reg.set_flag(&StatusFlag::Carry, false);
        }

        // Calcular nuevo Half-Carry según modo (suma/resta)
        let new_h = if es_resta {
            if !h_original {
                false
            } else {
                (self.reg.a & 0x0F) < 6
            }
        } else {
            lsb >= 0x0A
        };
        self.reg.set_flag(&StatusFlag::HalfCarry, new_h);

        // Aplicar ajustes según tabla
        let ajuste = match t {
            1 => if es_resta { 0xFA } else { 0x06 },  // -6 o +6
            2 => if es_resta { 0xA0 } else { 0x60 },  // -0x60 o +0x60
            3 => if es_resta { 0x9A } else { 0x66 },  // -0x66 o +0x66
            _ => 0x00,
        };

        self.reg.a = self.reg.a.wrapping_add(ajuste);

        // Actualizar flags restantes
        self.reg.set_flag(&StatusFlag::Zero, self.reg.a == 0);
        self.reg.set_flag(&StatusFlag::Sign, (self.reg.a & 0x80) != 0);
        self.reg.set_flag(&StatusFlag::ParityOverflow, (self.reg.a.count_ones() % 2) == 0);
    }

    fn paridad_flags(&mut self, val: u8) {
        // Calcula la paridad del valor: true si el número de bits cero es par.
        let parity = val.count_zeros() % 2 == 0;

        // Establece el flag ParityOverflow según la paridad calculada.
        self.reg
            .set_flag(&StatusFlag::ParityOverflow, parity);
        // Establece el flag Zero si el valor es cero.
        self.reg
            .set_flag(&StatusFlag::Zero, val == 0);
        // Establece el flag Sign si el bit más significativo (bit 7) del valor es 1.
        self.reg
            .set_flag(&StatusFlag::Sign, (val & 0b1000_0000) != 0);
    }

    fn get_loc8_lit_endian(&self, loc: &Local8) -> u8 {
        match loc {
            Local8::Inmediato8(v) => *v,
            Local8::Reg8(reg) => self.reg.get_reg8(*reg),
            Local8::RegIndirecto8(reg) => {
                let addr = self.reg.get_reg16_lit_endian(&reg);
                self.mem.mem[addr as usize]
            }
            Local8::InmediatoIndirecto8(addr) => self.mem.mem[*addr as usize],
            // Local8::Indexado8(reg) => {
            //     let addr = self.registros.get_reg16_lit_endian(&reg);
            //     self.memoria.memoria[addr as usize]
            // }
            Local8::Indexado8(reg, d) => {
                let base = self.reg.get_reg16_lit_endian(reg);

                let address = base.wrapping_add_signed(*d as i16);
                self.mem.mem[address as usize]
            }
        }
    }

    fn get_loc8_big_endian(&self, loc: &Local8) -> u8 {
        match loc {
            Local8::Inmediato8(v) => *v,
            Local8::Reg8(reg) => self.reg.get_reg8(*reg),
            Local8::RegIndirecto8(reg) => {
                let addr = self.reg.get_reg16_big_endian(&reg);
                self.mem.mem[addr as usize]
            }
            Local8::InmediatoIndirecto8(addr) => self.mem.mem[*addr as usize],
            // Local8::Indexado8(reg) => {
            //     let addr = self.registros.get_reg16_big_endian(&reg);
            //     self.memoria.memoria[addr as usize]
            // }
            Local8::Indexado8(reg, d) => {
                let base = self.reg.get_reg16_big_endian(reg);
                let address = base.wrapping_add_signed(*d as i16);
                self.mem.mem[address as usize]
            }
        }
    }

    fn set_loc8_lit_endian(&mut self, loc: &Local8, val: u8) {
        match loc {
            Local8::Inmediato8(_) => panic!("Attempting to set immediate value!"),
            Local8::Reg8(reg) => self.reg.set_reg8(*reg, val),
            Local8::InmediatoIndirecto8(addr) => {
                self.mem.mem[*addr as usize] = val;
            }
            Local8::RegIndirecto8(reg) => {
                let addr = self.reg.get_reg16_lit_endian(reg);
                self.mem.mem[addr as usize] = val;
            }
            // Local8::Indexado8(reg) => {
            //     let addr = self.registros.get_reg16_lit_endian(&reg);
            //     self.memoria.memoria[addr as usize] = val;
            // }
            Local8::Indexado8(reg, d) => {
                let base = self.reg.get_reg16_lit_endian(reg);
                let address = base.wrapping_add_signed(*d as i16);
                self.mem.mem[address as usize] = val;
            }
        }
    }
    fn set_loc8_big_endian(&mut self, loc: &Local8, val: u8) {
        match loc {
            Local8::Inmediato8(_) => panic!("Attempting to set immediate value!"),
            Local8::Reg8(reg) => self.reg.set_reg8(*reg, val),
            Local8::InmediatoIndirecto8(addr) => {
                self.mem.mem[*addr as usize] = val;
            }
            Local8::RegIndirecto8(reg) => {
                let addr = self.reg.get_reg16_big_endian(reg);
                self.mem.mem[addr as usize] = val;
            }
            // Local8::Indexado8(reg) => {
            //     let addr = self.registros.get_reg16_big_endian(&reg);
            //     self.memoria.memoria[addr as usize] = val;
            // }
            Local8::Indexado8(reg, d) => {
                let base = self.reg.get_reg16_big_endian(reg);
                let address = base.wrapping_add_signed(*d as i16);
                self.mem.mem[address as usize] = val;
            }
        }
    }
    fn get_loc16_lit_endian(&self, loc: &Local16) -> u16 {
        match loc {
            Reg(reg) => self.reg.get_reg16_lit_endian(reg),
            RegIndirecto(reg) => self.get_loc16_lit_endian(
                &InmediatoIndirecto(self.reg.get_reg16_lit_endian(reg)),
            ),
            Inmediato(n) => *n,
            InmediatoIndirecto(n) => u16::from_le_bytes([
                self.mem.mem[*n as usize],
                self.mem.mem[(*n + 1) as usize],
            ]),
            // Nuevo caso: Direccionamiento indexado (IX+d o IY+d)
            Indexado(reg, d) => {
                let base = self.reg.get_reg16_lit_endian(reg);
                // Calcula la dirección efectiva (base + desplazamiento con signo)
                let address = base.wrapping_add_signed(*d as i16);  // i8 -> i16 para extensión de signo
                self.get_loc16_lit_endian(&InmediatoIndirecto(address))
            }
        }
    }
    pub fn get_loc16_big_endian(&self, loc: &Local16) -> u16 {
        match loc {
            Reg(reg) => self.reg.get_reg16_big_endian(reg),
            RegIndirecto(reg) => self.get_loc16_big_endian(
                &InmediatoIndirecto(self.reg.get_reg16_big_endian(reg)),
            ),
            Inmediato(n) => *n,
            InmediatoIndirecto(n) => u16::from_be_bytes([
                self.mem.mem[(*n + 1) as usize],
                self.mem.mem[*n as usize],
            ]),
            // Nuevo caso: Direccionamiento indexado (IX+d o IY+d)
            Indexado(reg, d) => {
                let base = self.reg.get_reg16_big_endian(reg);
                // Calcula la dirección efectiva (base + desplazamiento con signo)
                let address = base.wrapping_add_signed(*d as i16);  // i8 -> i16 para extensión de signo
                self.get_loc16_big_endian(&Local16::InmediatoIndirecto(address))
            }
        }
    }
    pub fn set_loc16_lit_endian(&mut self, loc: &Local16, v: u16) {
        match loc {
            Inmediato(_) => panic!("Attempting to set immediate value!"),
            Reg(reg) => self.reg.set_reg16_lit_endian(reg, v),
            RegIndirecto(reg) => self.set_loc16_lit_endian(
                &InmediatoIndirecto(self.reg.get_reg16_lit_endian(reg)),
                v,
            ),
            InmediatoIndirecto(n) => {
                let [n1, n2] = v.to_le_bytes();
                self.mem.mem[*n as usize] = n1;
                self.mem.mem[(*n + 1) as usize] = n2;
            }
            Indexado(reg, d) => {
                let base = self.reg.get_reg16_big_endian(reg);
                let address = base.wrapping_add_signed(*d as i16);
                self.set_loc16_lit_endian(&Local16::InmediatoIndirecto(address), v);
            }
        }
    }
    pub fn set_loc16_big_endian(&mut self, loc: &Local16, v: u16) {
        match loc {
            Inmediato(_) => panic!("Attempting to set immediate value!"),
            Reg(reg) => {
                self.reg.set_reg16_big_endian(reg, v)
            }
            RegIndirecto(reg) => {
                self.set_loc16_big_endian(&InmediatoIndirecto(self.reg.get_reg16_big_endian(reg)), v)
            }
            InmediatoIndirecto(n) => {
                let [n1, n2] = v.to_be_bytes();

                self.mem.mem[*n as usize] = n2;
                self.mem.mem[(*n + 1) as usize] = n1;
            }
            // Nuevo caso: Direccionamiento indexado (IX+d o IY+d)
            Indexado(reg, d) => {
                let base = self.reg.get_reg16_big_endian(reg);
                let address = base.wrapping_add_signed(*d as i16);
                self.set_loc16_big_endian(&Local16::InmediatoIndirecto(address), v);
            }
        }
    }

    // // Origen y destino son iguales
    // fn inc16(&mut self, _: &Local16, src: &Local16) {
    //     match src {
    //         Local16::Reg(BC) => {
    //             let resultado = self.registros.get_reg16_big_endian(BC).wrapping_add(1);
    //             self.registros.set_reg16_big_endian(BC, resultado);
    //         }
    //         Local16::Reg(DE) => {
    //             let resultado = self.registros.get_reg16_big_endian(DE).wrapping_add(1);
    //             self.registros.set_reg16_big_endian(DE, resultado);
    //         }
    //         Local16::Reg(HL) => {
    //             let resultado = self.registros.get_reg16_big_endian(HL).wrapping_add(1);
    //             self.registros.set_reg16_big_endian(HL, resultado);
    //         }
    //         Local16::Reg(SP) => {
    //             let resultado = self.registros.get_reg16_big_endian(SP).wrapping_add(1);
    //             self.registros.set_reg16_big_endian(SP, resultado);
    //         }
    //         _ => panic!("Intentando incrementar en inc16() incorrectamente"),
    //     };
    // }

    // Origen y destino son iguales
    fn inc16(&mut self, _: &Local16, src: &Local16) {
        match src {
            // Reg(R16::BC) => { // Usar la variante BC de Reg16
            //     let resultado = self.registros.get_reg16_big_endian(&R16::BC).wrapping_add(1);
            //     self.registros.set_reg16_big_endian(&R16::BC, resultado);
            // }
            // Reg(R16::DE) => { // Usar la variante DE de Reg16
            //     let resultado = self.registros.get_reg16_big_endian(&R16::DE).wrapping_add(1);
            //     self.registros.set_reg16_big_endian(&R16::DE, resultado);
            // }
            // Reg(R16::HL) => { // Usar la variante HL de Reg16
            //     let resultado = self.registros.get_reg16_big_endian(&R16::HL).wrapping_add(1);
            //     self.registros.set_reg16_big_endian(&R16::HL, resultado);
            // }
            // Reg(R16::SP) => { // Usar la variante SP de Reg16
            //     let resultado = self.registros.get_reg16_big_endian(&R16::SP).wrapping_add(1);
            //     self.registros.set_reg16_big_endian(&R16::SP, resultado);
            // }

            Reg(r) => { // Usar la variante BC de Reg16
                let resultado = self.reg.get_reg16_big_endian(&r).wrapping_add(1);
                self.reg.set_reg16_big_endian(&r, resultado);
            }
            _ => panic!("..."),
        }
    }

    // // Origen y destino son iguales
    // fn dec16(&mut self, dst: &Local16, src: &Local16) {
    //     match src {
    //         Local16::Reg(BC) => {
    //             let resultado = self.registros.get_reg16_big_endian(BC).wrapping_sub(1);
    //             self.registros.set_reg16_big_endian(BC, resultado);
    //         }
    //         Local16::Reg(DE) => {
    //             let resultado = self.registros.get_reg16_big_endian(DE).wrapping_sub(1);
    //             self.registros.set_reg16_big_endian(DE, resultado);
    //         }
    //         Local16::Reg(HL) => {
    //             let resultado = self.registros.get_reg16_big_endian(HL).wrapping_sub(1);
    //             self.registros.set_reg16_big_endian(HL, resultado);
    //         }
    //         Local16::Reg(SP) => {
    //             let resultado = self.registros.get_reg16_big_endian(SP).wrapping_sub(1);
    //             self.registros.set_reg16_big_endian(SP, resultado);
    //         }
    //         _ => panic!("Intentando decrementar en dec16() incorrectamente"),
    //     };
    // }

    // Origen y destino son iguales
    fn dec16(&mut self, _: &Local16, src: &Local16) {
        match src {
            // Reg(R16::BC) => {
            //     let resultado = self.registros.get_reg16_big_endian(&R16::BC).wrapping_sub(1);
            //     self.registros.set_reg16_big_endian(&R16::BC, resultado);
            // }
            // Reg(R16::DE) => {
            //     let resultado = self.registros.get_reg16_big_endian(&R16::DE).wrapping_sub(1);
            //     self.registros.set_reg16_big_endian(&R16::DE, resultado);
            // }
            // Reg(R16::HL) => {
            //     let resultado = self.registros.get_reg16_big_endian(&R16::HL).wrapping_sub(1);
            //     self.registros.set_reg16_big_endian(&R16::HL, resultado);
            // }
            // Reg(R16::SP) => {
            //     let resultado = self.registros.get_reg16_big_endian(&R16::SP).wrapping_sub(1);
            //     self.registros.set_reg16_big_endian(&R16::SP, resultado);
            // }
            Reg(r) => {
                let resultado = self.reg.get_reg16_big_endian(&r).wrapping_sub(1);
                self.reg.set_reg16_big_endian(&r, resultado);
            }
            _ => panic!("Intentando decrementar en dec16() incorrectamente"),
        };
    }

    fn _add8(&mut self, dst: &Local8, src: &Local8, include_carry: bool) {
        let v1 = self.get_loc8_lit_endian(dst);
        let mut v2 = self.get_loc8_lit_endian(src);

        if include_carry && self.reg.get_flag(&StatusFlag::Carry) {
            // FIXME: what if _this_ overflows? Behaviour seems undefined
            v2 += 1
        }

        let (sum, ov) = v1.overflowing_add(v2);
        self.set_loc8_lit_endian(&dst, sum);
        // Seven bit carry
        self.reg
            .set_flag(&StatusFlag::Carry, (v1 & v2 & 0b0100_0000) != 0);
        // Adding
        self.reg
            .set_flag(&StatusFlag::AddSubtract, false);
        // Eight bit carry

        self.reg
            .set_flag(&StatusFlag::ParityOverflow, ov);
        // Third bit carry
        self.reg
            .set_flag(&StatusFlag::HalfCarry, (v1 & v2 & 0b00100) != 0);
        // Sum is zero
        self.reg.set_flag(&StatusFlag::Zero, sum == 0);
        // 8th bit is 1
        self.reg
            .set_flag(&StatusFlag::Sign, (sum & 0b1000_0000) != 0);
    }

    fn add8(&mut self, dst: &Local8, src: &Local8, include_carry: bool, carry_salida: bool) {
        let v1 = self.get_loc8_lit_endian(dst);
        let v2 = self.get_loc8_lit_endian(src);
        let carry_in = if include_carry && self.reg.get_flag(&StatusFlag::Carry) { 1 } else { 0 };

        // Suma sin signo para Carry (C)
        let sum_u16 = v1 as u16 + v2 as u16 + carry_in as u16;
        let sum_u8 = sum_u16 as u8;
        let carry = sum_u16 > 0xFF;

        // Suma con signo para Overflow (V)
        let v1_i8 = v1 as i8;
        let v2_i8 = v2 as i8;
        let sum_i8 = v1_i8.wrapping_add(v2_i8).wrapping_add(carry_in as i8);
        let overflow = (v1_i8 > 0 && v2_i8 > 0 && sum_i8 < 0) ||
            (v1_i8 < 0 && v2_i8 < 0 && sum_i8 > 0);

        // Half Carry (H): Acarreo del bit 3 al 4
        let half_carry = ((v1 & 0x0F) + (v2 & 0x0F) + carry_in) > 0x0F;

        self.set_loc8_lit_endian(dst, sum_u8);

        if carry_salida {
            self.reg.set_flag(&StatusFlag::Carry, carry);
        }

        self.reg.set_flag(&StatusFlag::AddSubtract, false);
        self.reg.set_flag(&StatusFlag::ParityOverflow, overflow);
        //Experimento:
        // dbg_hex!(v2);
        // if v2 == 0x7F {
        //     self.reg.set_flag(&StatusFlag::ParityOverflow, true);
        // } else {
        //     self.reg.set_flag(&StatusFlag::ParityOverflow, false);
        // }
        self.reg.set_flag(&StatusFlag::HalfCarry, half_carry);
        self.reg.set_flag(&StatusFlag::Zero, sum_u8 == 0);
        self.reg.set_flag(&StatusFlag::Sign, (sum_u8 & 0x80) != 0);
    }

    /*fn _add8_big(&mut self, dst: &Local8, src: &Local8, include_carry: bool) {
        let v1 = self.get_loc8_big_endian(dst);
        let mut v2 = self.get_loc8_big_endian(src);

        if include_carry && self.reg.get_flag(&StatusFlag::Carry) {
            // FIXME: what if _this_ overflows? Behaviour seems undefined
            v2 += 1
        }

        let (sum, ov) = v1.overflowing_add(v2);
        self.set_loc8_big_endian(&dst, sum);
        // Seven bit carry
        self.reg
            .set_flag(&StatusFlag::Carry, (v1 & v2 & 0b0100_0000) != 0);
        // Adding
        self.reg
            .set_flag(&StatusFlag::AddSubtract, false);
        // Eight bit carry
        self.reg
            .set_flag(&StatusFlag::ParityOverflow, ov);
        // Third bit carry
        self.reg
            .set_flag(&StatusFlag::HalfCarry, (v1 & v2 & 0b00100) != 0);
        // Sum is zero
        self.reg.set_flag(&StatusFlag::Zero, sum == 0);
        // 8th bit is 1
        self.reg
            .set_flag(&StatusFlag::Sign, (sum & 0b1000_0000) != 0);
    }*/

    fn add8_big(&mut self, dst: &Local8, src: &Local8, include_carry: bool) {
        let v1 = self.get_loc8_big_endian(dst);
        let mut v2 = self.get_loc8_big_endian(src);

        if include_carry && self.reg.get_flag(&StatusFlag::Carry) {
            v2 = v2.wrapping_add(1);
        }

        let (sum, ov) = v1.overflowing_add(v2);
        self.set_loc8_big_endian(&dst, sum);

        // Flag de Carry: Indica si hubo un acarreo en la suma de 8 bits
        self.reg.set_flag(&StatusFlag::Carry, (v1 & v2 & 0b0100_0000) != 0);

        // Flag de Add/Subtract: Se establece en falso para una operación de suma.
        self.reg.set_flag(&StatusFlag::AddSubtract, false);

        let overflow_signed = (v1 ^ v2) & 0x80 == 0 && (v1 ^ sum) & 0x80 != 0;
        self.reg.set_flag(&StatusFlag::ParityOverflow, overflow_signed);

        self.reg.set_flag(&StatusFlag::HalfCarry, (v1 & 0x0F).wrapping_add(v2 & 0x0F) > 0x0F);

        // Si suma es cero
        self.reg.set_flag(&StatusFlag::Zero, sum == 0);

        // Si bit octavo es 1
        self.reg.set_flag(&StatusFlag::Sign, (sum & 0b1000_0000) != 0);
    }

    // realiza una suma de 16 bits entre dst y src, con la opción de incluir el bit de acarreo (carry)
    // si include_carry es true.
    fn add16_le(&mut self, dst: &Local16, src: &Local16, include_carry: bool) {
        let v1 = match src {
            Reg(R16::SP) => {
                self.get_loc16_lit_endian(dst).swap_bytes()
            }

            _ => self.get_loc16_lit_endian(dst),
        };

        let mut v2 = self.get_loc16_big_endian(src);
        dbg_hex!(v1, v2);
        // Manejo del acarreo (carry) con desbordamiento
        if include_carry && self.reg.get_flag(&StatusFlag::Carry) {
            let (new_v2, carry_overflow) = v2.overflowing_add(1);
            v2 = new_v2;
            if carry_overflow {
                self.reg.set_flag(&StatusFlag::Carry, true);
            }
        }

        // hace la suma
        let (sum, ov) = v1.overflowing_add(v2);

        // pone el resultado en el destino usando big endian
        self.set_loc16_big_endian(&dst, sum);

        // **** Flags *******
        self.reg // Acarreo bit 15
            .set_flag(&StatusFlag::Carry, ov);
        self.reg // Flag N
            .set_flag(&StatusFlag::AddSubtract, false);
        // self.registros. // Flag H HalfCarry
        //     set_flag(&StatusFlag::HalfCarry, ((v1 ^ v2 ^ sum) & 0x1000) != 0);
        self.reg. // Flag H HalfCarry
            set_flag(&StatusFlag::HalfCarry, ((v1 ^ v2 ^ sum) & 0x1000) != 0);

        self.reg. // Suma es cero
            set_flag(&StatusFlag::Zero, sum == 0);
        self.reg // Flag S Signo bit 15 es 1
            .set_flag(&StatusFlag::Sign, (sum & 0x8000) != 0);
    }

    // realiza una suma de 16 bits entre dst y src, con la opción de incluir el bit de acarreo (carry)
    // si include_carry es true. BIG ENDIAN
    // flag_salida: ADD solo afecta a C,N,H    ADC afecta C,N,H P/OV, Z, S
    fn add16_be(&mut self, dst: &Local16, src: &Local16, include_carry: bool, flags_salida: bool) {
        let mut v2 = self.get_loc16_big_endian(dst);

        let v1 = self.get_loc16_big_endian(src);

        //let v1 = self.get_loc16_lit_endian(src).swap_bytes();  // lo cambia a big endian <- esta es la unica dif

        // Manejo del acarreo (carry) con desbordamiento
        if include_carry && self.reg.get_flag(&StatusFlag::Carry) {
            let (new_v2, carry_overflow) = v2.overflowing_add(1);

            v2 = new_v2;
            if carry_overflow {
                self.reg.set_flag(&StatusFlag::Carry, true);
            }
        }

        // hace la suma
        let (sum, ov) = v1.overflowing_add(v2);

        // pone el resultado en el destino usando big endian
        self.set_loc16_big_endian(&dst, sum);

        // **** Flags *******

        self.reg // Flag N
            .set_flag(&StatusFlag::AddSubtract, false);
        // H is set if carry from bit 11; otherwise, it is reset.
        self.reg. // Flag H HalfCarry
            set_flag(&StatusFlag::HalfCarry, ((v1 ^ v2 ^ sum) & 0x1000) != 0);
        self.reg // C Acarreo bit 15
            .set_flag(&StatusFlag::Carry, ov);

        if flags_salida {
            self.reg. // Suma es cero
                set_flag(&StatusFlag::Zero, sum == 0);
            self.reg // Flag S Signo bit 15 es 1
                .set_flag(&StatusFlag::Sign, (sum & 0x8000) != 0);

            // Flag de Overflow (P/V) en suma de 16 bits con signo
            let v1_sign = (v1 & 0x8000) != 0; // Bit 15 de v1
            let v2_sign = (v2 & 0x8000) != 0; // Bit 15 de v2
            let sum_sign = (sum & 0x8000) != 0; // Bit 15 del resultado

            let overflow_signed = (v1_sign == v2_sign) && (v1_sign != sum_sign);

            self.reg.set_flag(&StatusFlag::ParityOverflow, overflow_signed);
        }
    }

    // Acarreo del septimo bit min->minuendo destino   sub substraendo  bit el numero de bit
    pub fn is_borrow(min: u8, sub: u8, bit: u8) -> bool {
        // crear una máscara de unos hasta ese bit
        let mask = (1 << (bit + 1)) - 1;
        (min & mask) < (sub & mask)
    }

    // dst - src (si incluye_carry = true, tambien resta 1) -> dst
    // si es llamado por DEC no afecta al flag carry
    pub fn resta(
        &mut self,
        dst: &Local8,
        src: &Local8,
        incluye_carry: bool,
        store_result: bool,
        flags_salida: bool,
    ) {
        let v1 = self.get_loc8_lit_endian(dst);
        let mut v2 = self.get_loc8_lit_endian(src);

        if incluye_carry && self.reg.get_flag(&StatusFlag::Carry) {
            v2 = v2.wrapping_add(1);
        }

        let (sum, ov) = v1.overflowing_sub(v2);
        //dbg_hex!(sum, ov);
        if store_result {
            self.set_loc8_lit_endian(dst, sum);
        }

        // Bandera de acarreo (borrow total) Ojo en DEC n no afecta el carry
        if flags_salida {
            self.reg.set_flag(&StatusFlag::Carry, ov);
        }

        // Bandera de suma/resta
        self.reg
            .set_flag(&StatusFlag::AddSubtract, true);

        // Bandera de desbordamiento (overflow en resta con signo)
        let v1_sign = (v1 & 0x80) != 0;
        let v2_sign = (v2 & 0x80) != 0;
        let sum_sign = (sum & 0x80) != 0;
        let overflow_signed = (v1_sign != v2_sign) && (v1_sign != sum_sign);
        self.reg
            .set_flag(&StatusFlag::ParityOverflow, overflow_signed);

        // Bandera de medio acarreo (borrow del bit 4)
        // self.registros
        //     .set_flag(&StatusFlag::HalfCarry, Self::is_borrow(v1, v2, 3));
        self.reg
            .set_flag(&StatusFlag::HalfCarry, (v1 & 0x0F) < (v2 & 0x0F));

        // Bandera de cero
        self.reg
            .set_flag(&StatusFlag::Zero, sum == 0);
        //dbg_hex!(sum);
        // Bandera de signo
        self.reg
            .set_flag(&StatusFlag::Sign, (sum & 0x80) != 0);
    }

    // dst - src (si incluye_carry = true, tambien resta 1) -> dst
    // si es llamado por DEC no afecta al flag carry
    pub fn resta_big(
        &mut self,
        dst: &Local8,
        src: &Local8,
        incluye_carry: bool,
        store_result: bool,
        flags_salida: bool,
    ) {
        let v1 = self.get_loc8_big_endian(dst);
        let mut v2 = self.get_loc8_big_endian(src);

        if incluye_carry && self.reg.get_flag(&StatusFlag::Carry) {
            v2 = v2.wrapping_add(1);
        }

        let (sum, ov) = v1.overflowing_sub(v2);
        if store_result {
            self.set_loc8_big_endian(dst, sum);
        }

        // Bandera de acarreo (borrow total) Ojo en DEC n no afecta el carry
        if flags_salida {
            self.reg.set_flag(&StatusFlag::Carry, ov);
        }

        // Bandera de suma/resta
        self.reg
            .set_flag(&StatusFlag::AddSubtract, true);

        // Bandera de desbordamiento (overflow en resta con signo)
        let v1_sign = (v1 & 0x80) != 0;
        let v2_sign = (v2 & 0x80) != 0;
        let sum_sign = (sum & 0x80) != 0;
        let overflow_signed = (v1_sign != v2_sign) && (v1_sign != sum_sign);
        self.reg
            .set_flag(&StatusFlag::ParityOverflow, overflow_signed);

        /*// Solo para el caso de ED A9 CPD  ya que en esta instruccion P/V=1 si BC-1 != 0
        if !incluye_carry && !store_result && !flags_salida
            && self.reg.get_reg16_big_endian(&R16::BC).wrapping_sub(1) != 0 {
            self.reg
                .set_flag(&StatusFlag::ParityOverflow, true);
        }*/

        // Bandera de medio acarreo (borrow del bit 4)
        // self.registros
        //     .set_flag(&StatusFlag::HalfCarry, Self::is_borrow(v1, v2, 3));
        self.reg
            .set_flag(&StatusFlag::HalfCarry, (v1 & 0x0F) < (v2 & 0x0F));

        // Bandera de cero
        self.reg
            .set_flag(&StatusFlag::Zero, sum == 0);

        // Bandera de signo
        self.reg
            .set_flag(&StatusFlag::Sign, (sum & 0x80) != 0);
    }

    // dst - src (si incluye_carry = true, tambien resta 1) -> dst
    // Carry se define, N=1, PV detecta overflow,  H, Z, S se definen
    pub fn resta16_big(
        &mut self,
        dst: &Local16,
        src: &Local16,
        incluye_carry: bool,
        store_result: bool,
    ) {
        let v1 = self.get_loc16_big_endian(dst);
        let mut v2 = self.get_loc16_big_endian(src);

        if incluye_carry && self.reg.get_flag(&StatusFlag::Carry) {
            v2 = v2.wrapping_add(1);
        }

        let (sum, ov) = v1.overflowing_sub(v2);
        if store_result {
            self.set_loc16_big_endian(dst, sum);
        }

        // Bandera de acarreo (borrow total)
        self.reg
            .set_flag(&StatusFlag::Carry, ov);

        // Bandera de suma/resta (N)
        self.reg
            .set_flag(&StatusFlag::AddSubtract, true);

        // Bandera de desbordamiento (overflow en resta con signo)
        let v1_sign = (v1 & 0x8000) != 0;
        let v2_sign = (v2 & 0x8000) != 0;
        let sum_sign = (sum & 0x8000) != 0;
        let overflow_signed = (v1_sign != v2_sign) && (v1_sign != sum_sign);
        self.reg
            .set_flag(&StatusFlag::ParityOverflow, overflow_signed);

        // H es 1 si hay acarreo recibido en el bit 12, si no lo hay es 0
        self.reg
            .set_flag(&StatusFlag::HalfCarry, (v1 & 0x0FFF) < (v2 & 0x0FFF));

        // Bandera de cero
        self.reg
            .set_flag(&StatusFlag::Zero, sum == 0);

        // Bandera de signo
        self.reg
            .set_flag(&StatusFlag::Sign, (sum & 0x8000) != 0);
    }

    // Rota bits de registro de 8 bits a la izquierda       Bit7->C     Bit7->Bit0
    fn rota_izquierda(&mut self, loc: &Local8, set_parity: bool) {
        let val = self.get_loc8_lit_endian(loc);
        let result = val.rotate_left(1);
        self.set_loc8_lit_endian(loc, result);

        self.reg.
            set_flag(&StatusFlag::Carry, (val & 0b1000_0000) != 0);
        self.reg
            .set_flag(&StatusFlag::HalfCarry, false);
        self.reg
            .set_flag(&StatusFlag::AddSubtract, false);

        if set_parity {
            self.paridad_flags(result) // Actualiza Sign, Zero, Parity
        }
    }

    // Rota bits de registro de 8 bits a la izquierda       Bit7->C     Bit7->Bit0
    fn rota_izquierda_big(&mut self, loc: &Local8, set_parity: bool) {
        let val = self.get_loc8_big_endian(loc);
        let result = val.rotate_left(1);
        self.set_loc8_big_endian(loc, result);

        self.reg.
            set_flag(&StatusFlag::Carry, (val & 0b1000_0000) != 0);
        self.reg
            .set_flag(&StatusFlag::HalfCarry, false);
        self.reg
            .set_flag(&StatusFlag::AddSubtract, false);

        if set_parity {
            self.paridad_flags(result) // Actualiza Sign, Zero, Parity
        }
    }

    // Rota bits de registro de 8 bits a la derecha
    fn rota_derecha(&mut self, loc: &Local8, set_parity: bool) {
        let val = self.get_loc8_lit_endian(loc);
        let result = val.rotate_right(1);
        self.set_loc8_lit_endian(loc, result);

        self.reg.
            set_flag(&StatusFlag::Carry, (val & 0b0000_0001) != 0);
        self.reg
            .set_flag(&StatusFlag::HalfCarry, false);
        self.reg
            .set_flag(&StatusFlag::AddSubtract, false);

        if set_parity {
            self.paridad_flags(result) // Actualiza Sign, Zero, Parity
        }
    }
    // Rota bits de registro de 8 bits a la derecha
    fn rota_derecha_big(&mut self, loc: &Local8, set_parity: bool) {
        let val = self.get_loc8_big_endian(loc);
        let result = val.rotate_right(1);
        self.set_loc8_big_endian(loc, result);

        self.reg.
            set_flag(&StatusFlag::Carry, (val & 0b0000_0001) != 0);
        self.reg
            .set_flag(&StatusFlag::HalfCarry, false);
        self.reg
            .set_flag(&StatusFlag::AddSubtract, false);

        if set_parity {
            self.paridad_flags(result) // Actualiza Sign, Zero, Parity
        }
    }
    fn rota_derecha_thru_acc(&mut self, loc: &Local8, set_parity: bool) {
        let val = self.get_loc8_lit_endian(loc);
        let carry = self.reg
            .get_flag(&StatusFlag::Carry);
        let mut result = val >> 1;
        if carry {
            result |= 0x80;
        }
        self.set_loc8_lit_endian(loc, result);
        self.reg
            .set_flag(&StatusFlag::Carry, val & 0b1 != 0);
        self.reg
            .set_flag(&StatusFlag::HalfCarry, false);
        self.reg
            .set_flag(&StatusFlag::AddSubtract, false);

        if set_parity {
            self.paridad_flags(result);
        }
    }
    fn rota_derecha_thru_acc_big(&mut self, loc: &Local8, set_parity: bool) {
        let val = self.get_loc8_big_endian(loc);
        let carry = self.reg
            .get_flag(&StatusFlag::Carry);
        let mut result = val >> 1;
        if carry {
            result |= 0x80;
        }
        self.set_loc8_big_endian(loc, result);
        self.reg
            .set_flag(&StatusFlag::Carry, val & 0b1 != 0);
        self.reg
            .set_flag(&StatusFlag::HalfCarry, false);
        self.reg
            .set_flag(&StatusFlag::AddSubtract, false);

        if set_parity {
            self.paridad_flags(result);
        }
    }
    fn intercambia16(&mut self, a: &Local16, _: &Local16) {
        match a {
            Local16::Reg(AF) => {
                let af = self.reg.get_reg16_lit_endian(&AF);
                let afp = self.reg.get_reg16_lit_endian(&AFP);
                self.reg.set_reg16_lit_endian(&AF, afp);
                self.reg.set_reg16_lit_endian(&AFP, af);
            }
            _ => panic!("Attempting to intercambia16 reg!"),
        }
    }

    fn decrement_jump(&mut self, offset: i8) -> Option<u16> {
        // Obtiene el valor del registro B.
        let b = self.reg.get_reg8(R8::B);

        // Decrementa el valor del registro B usando wrapping subtraction.
        let b = b.wrapping_sub(1);

        // Actualiza el registro B con el nuevo valor decrementado.
        self.reg.set_reg8(R8::B, b);

        // Verifica si el registro B es diferente de cero.
        if b != 0 {
            // Si B no es cero, calcula la nueva dirección de salto usando el offset.
            // Devuelve la nueva dirección como Some(u16).
            Some(self.pc_offset(offset))
        } else {
            // Si B no es cero, calcula la nueva dirección de salto usando el offset.
            // Devuelve la nueva dirección como Some(u16).
            None
        }
    }

    fn pc_offset(&self, offset: i8) -> u16 {
        self.reg
            // Obtiene el valor actual del contador de programa (PC)
            .get_pc()
            // Convierte el offset (i8) a u16 y lo suma al PC usando wrapping addition.
            .wrapping_add(offset as u16)
            // Añade 2 al resultado usando wrapping addition.
            .wrapping_add(2)
    }

    // A rota a la izquierda un bit, bit7->carry  carry anterior->bit0
    fn rota_izquierda_thru_acc(&mut self, loc: &Local8, set_parity: bool) {
        let val = self.get_loc8_lit_endian(loc);
        let carry = self.reg
            .get_flag(&StatusFlag::Carry);
        let mut result = val << 1;
        if carry {
            result |= 0b1;
        }
        self.set_loc8_lit_endian(loc, result);

        self.reg
            .set_flag(&StatusFlag::Carry, val & 0b1000_0000 != 0);
        self.reg
            .set_flag(&StatusFlag::HalfCarry, false);
        self.reg
            .set_flag(&StatusFlag::AddSubtract, false);

        if set_parity {
            self.paridad_flags(result)
        }
    }

    // A rota a la izquierda un bit, bit7->carry  carry anterior->bit0
    fn rota_izquierda_thru_acc_big(&mut self, loc: &Local8, set_parity: bool) {
        let val = self.get_loc8_big_endian(loc);
        let carry = self.reg
            .get_flag(&StatusFlag::Carry);
        let mut result = val << 1;
        if carry {
            result |= 0b1;
        }
        self.set_loc8_big_endian(loc, result);

        self.reg
            .set_flag(&StatusFlag::Carry, val & 0b1000_0000 != 0);
        self.reg
            .set_flag(&StatusFlag::HalfCarry, false);
        self.reg
            .set_flag(&StatusFlag::AddSubtract, false);

        if set_parity {
            self.paridad_flags(result)
        }
    }
    fn jump_relative(&mut self, cond: SaltoCondicional, offset: i8) -> Option<u16> {
        if self.eval_cond(cond) {
            // Range is -126 to 129
            Some(self.pc_offset(offset))
        } else {
            None
        }
    }

    // Aqui evalua la condicion devuelve true si se cumple
    fn eval_cond(&self, cond: SaltoCondicional) -> bool {
        match cond {
            SaltoCondicional::Incondicional => true,
            SaltoCondicional::Carry => self.reg.get_flag(&StatusFlag::Carry),
            SaltoCondicional::NoCarry => !self.reg.get_flag(&StatusFlag::Carry),

            SaltoCondicional::Cero => self.reg.get_flag(&StatusFlag::Zero),
            SaltoCondicional::NoCero => !self.reg.get_flag(&StatusFlag::Zero),

            SaltoCondicional::ParidadPar => {
                self.reg.get_flag(&StatusFlag::ParityOverflow)
            }
            SaltoCondicional::ParidadImpar => {
                !self.reg.get_flag(&StatusFlag::ParityOverflow)
            }

            SaltoCondicional::SignoNegativo => self.reg.get_flag(&StatusFlag::Sign),
            SaltoCondicional::SignoPositivo => !self.reg.get_flag(&StatusFlag::Sign),
            // _ => true  //provisional
        }
    }

    // Recibe una funcion logica entre dos bits y pone el resultado en A
    fn bool_op<F>(&mut self, src: &Local8, f: F, flag_h: bool)
    where
        F: Fn(u8, u8) -> u8,
    {
        let v1 = self.get_loc8_lit_endian(&Local8::Reg8(R8::A));
        let v2 = self.get_loc8_lit_endian(src);

        let result = f(v1, v2);
        self.set_loc8_lit_endian(&Local8::Reg8(R8::A), result);

        // Septimo bit carry es cero
        self.reg.set_flag(&StatusFlag::Carry, false);
        // Pone a 0 el flag N
        self.reg
            .set_flag(&StatusFlag::AddSubtract, false);

        // Pone a 1 el flag H si es AND, a 0 si es XOR, a 0 si es OR, estaba en false todos
        if flag_h {
            self.reg.set_flag(&StatusFlag::HalfCarry, true);
        } else {
            self.reg.set_flag(&StatusFlag::HalfCarry, false);
        }

        self.paridad_flags(result);
    }

    // Recibe una funcion logica entre dos bits y pone el resultado en A
    fn bool_op_big<F>(&mut self, src: &Local8, f: F, flag_h: bool)
    where
        F: Fn(u8, u8) -> u8,
    {
        let v1 = self.get_loc8_big_endian(&Local8::Reg8(R8::A));
        let v2 = self.get_loc8_big_endian(src);

        let result = f(v1, v2);
        self.set_loc8_big_endian(&Local8::Reg8(R8::A), result);

        // Septimo bit carry es cero
        self.reg.set_flag(&StatusFlag::Carry, false);
        // Pone a 0 el flag N
        self.reg
            .set_flag(&StatusFlag::AddSubtract, false);

        // Pone a 1 el flag H si es AND, a 0 si es XOR, a 0 si es OR, estaba en false todos
        if flag_h {
            self.reg.set_flag(&StatusFlag::HalfCarry, true);
        } else {
            self.reg.set_flag(&StatusFlag::HalfCarry, false);
        }

        self.paridad_flags(result);
    }

    // Convierte lo que hay en A a su complemento a 1 (invierte los bits)
    fn complemento(&mut self) {
        let reg_a = R8::A;
        let a = self.reg.get_reg8(reg_a);
        self.reg.set_reg8(reg_a, !a);

        self.reg.set_flag(&StatusFlag::HalfCarry, true);
        self.reg.set_flag(&StatusFlag::AddSubtract, true);
    }

    // Pone el flag carry a true
    fn set_carry(&mut self) {
        self.reg
            .set_flag(&StatusFlag::Carry, true);
        self.reg
            .set_flag(&StatusFlag::AddSubtract, false);
        self.reg
            .set_flag(&StatusFlag::HalfCarry, false);
    }

    // Invierte carry
    fn toggle_carry(&mut self) {
        let carry = self.reg
            .get_flag(&StatusFlag::Carry);
        self.reg
            .set_flag(&StatusFlag::Carry, !carry);
        self.reg
            .set_flag(&StatusFlag::AddSubtract, false);
        // Previo carry es copiado en H (lo pongo yo)
        self.reg
            .set_flag(&StatusFlag::HalfCarry, carry);
    }

    // Suma dos valores de 8 bits, incluyendo el carry actual si se especifica.
    // Actualiza las banderas de estado según el resultado.
    fn add(&mut self, dst: &Local8, src: &Local8, incluye_carry: bool) {
        let v1 = self.get_loc8_lit_endian(dst);
        let v2 = self.get_loc8_lit_endian(src);

        // 1. Calcular carry de entrada (0 o 1)
        let carry_in = if incluye_carry && self.reg.get_flag(&StatusFlag::Carry) {
            1
        } else {
            0
        };

        // 2. Calcular suma con carry de manera manual
        let (intermediate_sum, carry1) = v1.overflowing_add(v2);
        let (sum, carry2) = intermediate_sum.overflowing_add(carry_in);
        let carry = carry1 || carry2;

        // 3. Detectar overflow para números con signo
        let v1_sign = (v1 & 0x80) != 0;
        let v2_sign = (v2 & 0x80) != 0;
        let sum_sign = (sum & 0x80) != 0;
        let overflow = (v1_sign == v2_sign) && (sum_sign != v1_sign);

        // 4. Detectar medio carry (transición del bit 3 al 4)
        let half_carry = ((v1 & 0x0F) + (v2 & 0x0F) + carry_in) > 0x0F;

        // 5. Actualizar registro destino
        self.set_loc8_lit_endian(dst, sum);

        // 6. Establecer banderas
        self.reg.set_flag(&StatusFlag::Carry, carry);
        self.reg.set_flag(&StatusFlag::ParityOverflow, overflow);
        self.reg.set_flag(&StatusFlag::HalfCarry, half_carry);
        self.reg.set_flag(&StatusFlag::Zero, sum == 0);
        self.reg.set_flag(&StatusFlag::Sign, sum_sign);
        self.reg.set_flag(&StatusFlag::AddSubtract, false); // Operación de suma
    }

    // Suma dos valores de 8 bits, incluyendo el carry actual si se especifica.
    // Actualiza las banderas de estado según el resultado.
    fn add_big(&mut self, dst: &Local8, src: &Local8, incluye_carry: bool) {
        let v1 = self.get_loc8_big_endian(dst);
        let v2 = self.get_loc8_big_endian(src);

        // 1. Calcular carry de entrada (0 o 1)
        let carry_in = if incluye_carry && self.reg.get_flag(&StatusFlag::Carry) {
            1
        } else {
            0
        };

        // 2. Calcular suma con carry de manera manual
        let (intermediate_sum, carry1) = v1.overflowing_add(v2);
        let (sum, carry2) = intermediate_sum.overflowing_add(carry_in);
        let carry = carry1 || carry2;

        // 3. Detectar overflow para números con signo
        let v1_sign = (v1 & 0x80) != 0;
        let v2_sign = (v2 & 0x80) != 0;
        let sum_sign = (sum & 0x80) != 0;
        let overflow = (v1_sign == v2_sign) && (sum_sign != v1_sign);

        // 4. Detectar medio carry (transición del bit 3 al 4)
        let half_carry = ((v1 & 0x0F) + (v2 & 0x0F) + carry_in) > 0x0F;

        // 5. Actualizar registro destino
        self.set_loc8_big_endian(dst, sum);

        // 6. Establecer banderas
        self.reg.set_flag(&StatusFlag::Carry, carry);
        self.reg.set_flag(&StatusFlag::ParityOverflow, overflow);
        self.reg.set_flag(&StatusFlag::HalfCarry, half_carry);
        self.reg.set_flag(&StatusFlag::Zero, sum == 0);
        self.reg.set_flag(&StatusFlag::Sign, sum_sign);
        self.reg.set_flag(&StatusFlag::AddSubtract, false); // Operación de suma
    }

    // Mete un valor en el stack y modifica el SP --- ojo parece que esta mal
    fn push_val_lit_endian(&mut self, val: u16) {
        //self.reg.set_reg16_lit_endian(&R16::SP, self.reg.get_reg16_lit_endian(&R16::SP) - 2);
        self.reg.set_reg16_lit_endian(&R16::SP, (self.reg.get_reg16_lit_endian(&R16::SP)).wrapping_sub(2));
        self.set_loc16_lit_endian(&Local16::RegIndirecto(R16::SP), val);
    }

    // Mete un valor en el stack y modifica el SP
    fn push_val_big_endian(&mut self, val: u16) {
        //self.reg.set_reg16_big_endian(&R16::SP, self.reg.get_reg16_big_endian(&R16::SP) - 2);
        self.reg.set_reg16_big_endian(&R16::SP, self.reg.get_reg16_big_endian(&R16::SP).wrapping_sub(2));
        self.set_loc16_big_endian(&Local16::RegIndirecto(R16::SP), val);
    }

    // fn push(&mut self, src: &Local16) {
    //     self.push_val_big_endian(self.get_loc16_big_endian(src));
    // }

    // creo yo este push
    fn push(&mut self, src: &Local16) {
        self.reg.sp = self.reg.sp - 1;

        let bytes_big_endian = self.get_loc16_big_endian(src).to_be_bytes();

        self.mem.mem[self.reg.sp as usize] = bytes_big_endian[0]; // pongo el alto

        self.reg.sp = self.reg.sp - 1;

        self.mem.mem[self.reg.sp as usize] = bytes_big_endian[1]; // pongo el bajo

        //self.push_val_big_endian(self.get_loc16_big_endian(src));
    }

    // Saca un valor del stack y modifica el SP
    fn pop_val_little_endian(&mut self) -> u16 {
        let n = self.get_loc16_lit_endian(&Local16::RegIndirecto(R16::SP));
        self.reg.set_reg16_lit_endian(
            &R16::SP,
            //self.reg.get_reg16_lit_endian(&R16::SP) + 2,
            self.reg.get_reg16_lit_endian(&R16::SP).wrapping_add(2),
        );
        n
    }

    // Saca un valor del stack y modifica el SP hecho por mi
    fn pop_val_big_endian(&mut self) -> u16 {
        let n = self.get_loc16_big_endian(&Local16::RegIndirecto(R16::SP));
        self.reg.set_reg16_big_endian(
            &R16::SP,
            //self.reg.get_reg16_big_endian(&R16::SP) + 2,
            self.reg.get_reg16_big_endian(&R16::SP).wrapping_add(2),
        );
        n
    }

    fn pop(&mut self, dst: &Local16) {
        let val = self.pop_val_big_endian();
        self.set_loc16_big_endian(dst, val);
    }

    // hace un push del pc y pone los datos nuevos en el pc si se cumple la condicion
    fn call(&mut self, cond: SaltoCondicional, loc: u16) -> Option<u16> {
        if self.eval_cond(cond) {
            self.push_val_lit_endian(self.reg.get_pc() + 3); // Todas las instrucciones CALL tienen 3 bytes
            Some(loc)
        } else {
            None
        }
    }

    // Ejecuta instruccion ret si se cumple la condicion, haciendo un pop. Si no se cumple devuelve None
    fn ret(&mut self, cond: SaltoCondicional) -> Option<u16> {
        if self.eval_cond(cond) {
            Some(self.pop_val_little_endian())
        } else {
            None
        }
    }

    // Ejecuta instruccion ret si se cumple la condicion, haciendo un pop. Si no se cumple devuelve None
    fn ret_big(&mut self, cond: SaltoCondicional) -> Option<u16> {
        if self.eval_cond(cond) {
            Some(self.pop_val_big_endian())
        } else {
            None
        }
    }

    // Ejecuta instruccion ret si se cumple la condicion, haciendo un pop. Si no se cumple devuelve None
    fn reti(&mut self) -> Option<u16> {
        Some(self.pop_val_little_endian())
    }

    // Salta a una direccion cualquiera si se cumple la condicion si no se cumple devuelve None
    fn jump_cond(&mut self, cond: SaltoCondicional, loc: &Local16) -> Option<u16> {
        if self.eval_cond(cond) {
            Some(self.get_loc16_lit_endian(loc))
        } else {
            None
        }
    }

    // Salta a la direccion que dice hl
    fn jumphl(&mut self, loc: &Local16) -> Option<u16> {
        Some(self.get_loc16_big_endian(loc))
    }

    // Lo creo yo, para uso de rst, salta a una direccion dada y hace un push
    fn jump_incondicional_con_push_lit(&mut self, loc: &Local16) -> Option<u16> {
        let pc = self.reg.get_pc() + 1;
        self.push_val_lit_endian(pc);
        Some(self.get_loc16_lit_endian(loc))
    }

    // Lo creo yo, para uso de rst, salta a una direccion dada y hace un push
    fn jump_incondicional_con_push_big(&mut self, loc: &Local16) -> Option<u16> {
        let pc = self.reg.get_pc() + 1;
        self.push_val_big_endian(pc);
        Some(self.get_loc16_big_endian(loc))
    }

    fn shift_right(&mut self, loc: &Local8, preserve_sign: bool) {
        let val = self.get_loc8_lit_endian(loc);
        let carry = (val & 0x1) != 0;
        let mut result = val >> 1;

        if preserve_sign {
            result |= val & 0x80;
        }

        self.set_loc8_lit_endian(loc, result);

        self.reg
            .set_flag(&StatusFlag::Carry, carry);
        self.reg
            .set_flag(&StatusFlag::HalfCarry, false);
        self.reg
            .set_flag(&ops::StatusFlag::AddSubtract, false);
        self.paridad_flags(result);
    }

    fn shift_right_big(&mut self, loc: &Local8, preserve_sign: bool) {
        let val = self.get_loc8_big_endian(loc);
        let carry = (val & 0x1) != 0;
        let mut result = val >> 1;

        if preserve_sign {
            result |= val & 0x80;
        }

        self.set_loc8_big_endian(loc, result);

        self.reg
            .set_flag(&StatusFlag::Carry, carry);
        self.reg
            .set_flag(&StatusFlag::HalfCarry, false);
        self.reg
            .set_flag(&ops::StatusFlag::AddSubtract, false);
        self.paridad_flags(result);
    }

    fn shift_left(&mut self, loc: &Local8) {
        let val = self.get_loc8_lit_endian(loc);
        let carry = (val & 0x80) != 0;
        let result = val << 1;

        self.set_loc8_lit_endian(loc, result);

        self.reg
            .set_flag(&StatusFlag::Carry, carry);
        self.reg
            .set_flag(&StatusFlag::HalfCarry, false);
        self.reg
            .set_flag(&StatusFlag::AddSubtract, false);
        self.paridad_flags(result);
    }

    fn shift_left_big(&mut self, loc: &Local8) {
        let val = self.get_loc8_big_endian(loc);
        let carry = (val & 0x80) != 0;
        let result = val << 1;

        self.set_loc8_big_endian(loc, result);

        self.reg
            .set_flag(&StatusFlag::Carry, carry);
        self.reg
            .set_flag(&StatusFlag::HalfCarry, false);
        self.reg
            .set_flag(&StatusFlag::AddSubtract, false);
        self.paridad_flags(result);
    }

    // Provisional para uso de SLL
    fn shift_left_sll_lit(&mut self, loc: &Local8) {
        let val = self.get_loc8_lit_endian(loc);
        let carry = (val & 0x80) != 0;
        let mut result = val << 1;
        result = result | 0b0000_0001;

        self.set_loc8_lit_endian(loc, result);

        self.reg
            .set_flag(&StatusFlag::Carry, carry);
        self.reg
            .set_flag(&StatusFlag::HalfCarry, false);
        self.reg
            .set_flag(&StatusFlag::AddSubtract, false);
        self.paridad_flags(result);
    }

    // Provisional para uso de SLL
    fn shift_left_sll_big(&mut self, loc: &Local8) {
        let val = self.get_loc8_big_endian(loc);
        let carry = (val & 0x80) != 0;
        let mut result = val << 1;
        result = result | 0b0000_0001;

        self.set_loc8_big_endian(loc, result);

        self.reg
            .set_flag(&StatusFlag::Carry, carry);
        self.reg
            .set_flag(&StatusFlag::HalfCarry, false);
        self.reg
            .set_flag(&StatusFlag::AddSubtract, false);
        self.paridad_flags(result);
    }

    fn get_bit(&mut self, bit: u8, loc: &Local8) {
        assert!(bit < 8);
        let val = self.get_loc8_lit_endian(loc);
        self.reg
            .set_flag(&StatusFlag::Zero, val & (1 << bit) == 0);
        self.reg
            .set_flag(&StatusFlag::HalfCarry, true);
        self.reg
            .set_flag(&StatusFlag::AddSubtract, false);
    }

    fn set_bit_lit(&mut self, bit: u8, loc: &Local8) {
        assert!(bit < 8);
        let val = self.get_loc8_lit_endian(loc);
        self.set_loc8_lit_endian(loc, val | (1 << bit))
    }

    fn reset_bit_lit(&mut self, bit: u8, loc: &Local8) {
        assert!(bit < 8);
        let val = self.get_loc8_lit_endian(loc);
        self.set_loc8_lit_endian(loc, val & !(1 << bit));
    }

    fn set_bit_big(&mut self, bit: u8, loc: &Local8) {
        assert!(bit < 8);
        let val = self.get_loc8_big_endian(loc);
        self.set_loc8_big_endian(loc, val | (1 << bit))
    }

    fn reset_bit_big(&mut self, bit: u8, loc: &Local8) {
        assert!(bit < 8);
        let val = self.get_loc8_big_endian(loc);
        self.set_loc8_big_endian(loc, val & !(1 << bit));
    }

    // Usado en in a(n) Lee de periferico un u8 y lo pone en loc, no afecta a flags
    fn read_in(&mut self, periferico: &Local8, loc: &Local8) {
        suena("sound2.wav");
        let peripheral = self.get_loc8_lit_endian(periferico);
        let result = match self.input_devices.get_mut(&peripheral) {
            None => panic!("no hay un periferico instalado en 0x{:02x}", peripheral),
            Some(d) => d.input()
        };
        self.set_loc8_lit_endian(loc, result);
    }

    // Usado en in b(c), in d(c), in h,(c) (instrucciones miscelaneas) Lee de periferico un u8 y lo pone en loc
    // Afecta flags: carry no afectado, N=0, PV detecta paridad, H=0, A,S se definen
    fn read_in_flags(&mut self, periferico: &Local8, loc: &Local8) {
        let peripheral = self.get_loc8_lit_endian(periferico);
        let result = match self.input_devices.get_mut(&peripheral) {
            None => panic!("no hay un periferico instalado en 0x{:02x}", peripheral),
            Some(d) => {
                let ent = d.input();
                self.reg
                    .set_flag(&StatusFlag::AddSubtract, false);
                self.reg
                    .set_flag(&StatusFlag::Sign, (ent & 0b1000_0000) != 0);
                self.reg
                    .set_flag(&StatusFlag::HalfCarry, false);
                self.reg
                    .set_flag(&StatusFlag::Zero, ent == 0);

                ent
            }
        };
        self.set_loc8_lit_endian(loc, result);
    }

    // Usado en OUT Lee de loc un u8 y lo pone en periferico
    fn write_out_le(&mut self, periferico: &Local8, loc: &Local8) {
        //dbg_hex!(periferico);
        suena("sound.wav");
        let peripheral = self.get_loc8_lit_endian(periferico);
        //dbg_hex!(peripheral);

        //dbg_hex!(loc);
        let val = [self.get_loc8_lit_endian(loc)];
        //dbg_hex!(val);
        match self.output_devices.get_mut(&peripheral) {
            None => panic!("no hay un periferico instalado en 0x{:02x}", peripheral),
            Some(d) => d.write(&val),
        };
    }

    // Usado en OUT Lee de loc un u8 y lo pone en periferico
    fn write_out_be(&mut self, periferico: &Local8, loc: &Local8) {
        //dbg_hex!(periferico);
        suena("sound.wav");
        let peripheral = self.get_loc8_big_endian(periferico);
        //dbg_hex!(peripheral);

        //dbg_hex!(loc);
        let val = vec![self.get_loc8_big_endian(loc)];
        //dbg_hex!(&val);
        match self.output_devices.get_mut(&peripheral) {
            None => panic!("no hay un periferico instalado en 0x{:02x}", peripheral),
            Some(d) => {
                self.color_borde = val.last().unwrap() & 0b0000_0111;
                d.write(&val);
            }
        };
    }
}

