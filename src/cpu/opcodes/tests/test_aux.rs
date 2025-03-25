use crate::assert_hex;
use crate::ops::StatusFlag;
use crate::z80::Z80;
use zilog_z80::cpu::CPU;

pub fn ejecutar(z80: &mut Z80, m0: u8, m1: u8, m2: u8, m3: u8, c: &mut CPU) {
    z80.es_halted = false;

    z80.mem.mem[0] = m0;
    z80.mem.mem[1] = m1;
    z80.mem.mem[2] = m2;
    z80.mem.mem[3] = m3;

    c.bus.write_byte(0, m0);
    c.bus.write_byte(1, m1);
    c.bus.write_byte(2, m2);
    c.bus.write_byte(3, m3);

    // Ejecuto la instruccion
    z80.step(c);
}

// Devuelve bytes consumidos
pub fn ejecutar_en_direccion(z80: &mut Z80, direcc: usize, m0: u8, m1: u8, m2: u8, m3: u8, c: &mut CPU) -> usize {
    z80.es_halted = false;

    z80.mem.mem[direcc] = m0;
    z80.mem.mem[direcc + 1] = m1;
    z80.mem.mem[direcc + 2] = m2;
    z80.mem.mem[direcc + 3] = m3;

    c.bus.write_byte(direcc as u16, m0);
    c.bus.write_byte((direcc + 1) as u16, m1);
    c.bus.write_byte((direcc + 2) as u16, m2);
    c.bus.write_byte((direcc + 3) as u16, m3);

    // Ejecuto la instruccion
    z80.step(c)
}

// 0 => false, 1 => true, 2 => indiferente
// s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
pub fn prueba_flags(z80: &Z80, s: u8, z: u8, h: u8, pv: u8, n: u8, c: u8) {
    let flags = [
        (s, StatusFlag::Sign, "Sign"),
        (z, StatusFlag::Zero, "Zero"),
        (h, StatusFlag::HalfCarry, "HalfCarry"),
        (pv, StatusFlag::ParityOverflow, "ParityOverflow"),
        (n, StatusFlag::AddSubtract, "AddSubtract"),
        (c, StatusFlag::Carry, "Carry"),
    ];

    for (val, flag, name) in flags {
        match val {
            0 => assert!(!z80.reg.get_flag(&flag), "Flag {} es false", name),
            1 => assert!(z80.reg.get_flag(&flag), "Flag {} es true", name),
            _ => (),
        }
    }
}

pub fn hace_un_call_incondicional_a_direcc_dada(z80: &mut Z80, direcc_h: u8, direcc_l: u8, c: &mut CPU) {
    // 0xCC n1 n2  no afecta flags

    // Establecemos SP como 0xFFFF
    z80.reg.sp = 0xFFFF;
    c.reg.sp = 0xFFFF;

    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    z80.reg.pc = 0xA24F;
    c.reg.pc = 0xA24F;

    // primero se almacena el mas significativo en el stack
    //ejecutar_en_una_direccion(z80, 0xA24F, 0xCD, 0x15, 0xE0, 0);
    ejecutar_en_direccion(z80, 0xA24F, 0xCD, direcc_l, direcc_h, 0, c);

    // comprobar que SP ha cambiado
    assert_eq!(z80.reg.sp, 0xFFFD);

    // comprobar los bytes del stack
    assert_eq!(z80.mem.mem[0xFFFE], 0xA2);
    // 0xFFFD = 0x52  (ha sumado 3 a 4F) para que cuando haya un ret vuelva
    // a la direccion posterior al call que siempre es de 3 bytes
    assert_eq!(z80.mem.mem[0xFFFD], 0x52);

    // Comprueba el nuevo valor del pc
    let r = ((direcc_h as u16) << 8) | (direcc_l as u16);
    //assert_eq!(z80.registros.pc, 0xE015);
    assert_eq!(z80.reg.pc, r);
}

// prueba_flags():     0 => false, 1 => true, 2 => indiferente
// s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
pub fn bit_pba(z80: &mut Z80, nbit: u8, registro: u8, parametro: u8, cpu_z: &mut CPU) {
    let binario: u8 = 1 << nbit;
    match registro {
        0b000 => {  // B
            // Reseteamos PC
            z80.reg.pc = 0;
            cpu_z.reg.pc = 0;
            z80.reg.b = !binario;
            cpu_z.reg.b = !binario;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            prueba_flags(&z80, 2, 1, 1, 2, 0, 2);

            z80.reg.pc = 0;
            cpu_z.reg.pc = 0;
            z80.reg.b = binario;
            cpu_z.reg.b = binario;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            prueba_flags(&z80, 2, 0, 1, 2, 0, 2);
        }
        0b001 => { // C
            z80.reg.pc = 0;
            cpu_z.reg.pc = 0;
            z80.reg.c = !binario;
            cpu_z.reg.c = !binario;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            prueba_flags(&z80, 2, 1, 1, 2, 0, 2);

            z80.reg.pc = 0;
            cpu_z.reg.pc = 0;
            z80.reg.c = binario;
            cpu_z.reg.c = binario;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            prueba_flags(&z80, 2, 0, 1, 2, 0, 2);
        }
        0b010 => { // D
            z80.reg.pc = 0;
            cpu_z.reg.pc = 0;
            z80.reg.d = !binario;
            cpu_z.reg.d = !binario;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            prueba_flags(&z80, 2, 1, 1, 2, 0, 2);

            z80.reg.pc = 0;
            cpu_z.reg.pc = 0;
            z80.reg.d = binario;
            cpu_z.reg.d = binario;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            prueba_flags(&z80, 2, 0, 1, 2, 0, 2);
        }
        // Registro E (0b011)
        0b011 => {
            z80.reg.pc = 0;
            cpu_z.reg.pc = 0;
            z80.reg.e = !binario;
            cpu_z.reg.e = !binario;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            prueba_flags(&z80, 2, 1, 1, 2, 0, 2);

            z80.reg.pc = 0;
            cpu_z.reg.pc = 0;
            z80.reg.e = binario;
            cpu_z.reg.e = binario;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            prueba_flags(&z80, 2, 0, 1, 2, 0, 2);
        }

        // Registro H (0b100)
        0b100 => {
            z80.reg.pc = 0;
            cpu_z.reg.pc = 0;
            z80.reg.h = !binario;
            cpu_z.reg.h = !binario;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            prueba_flags(&z80, 2, 1, 1, 2, 0, 2);

            z80.reg.pc = 0;
            cpu_z.reg.pc = 0;
            z80.reg.h = binario;
            cpu_z.reg.h = binario;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            prueba_flags(&z80, 2, 0, 1, 2, 0, 2);
        }

        // Registro L (0b101)
        0b101 => {
            z80.reg.pc = 0;
            cpu_z.reg.pc = 0;
            z80.reg.l = !binario;
            cpu_z.reg.l = !binario;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            prueba_flags(&z80, 2, 1, 1, 2, 0, 2);

            z80.reg.pc = 0;
            cpu_z.reg.pc = 0;
            z80.reg.l = binario;
            cpu_z.reg.l = binario;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            prueba_flags(&z80, 2, 0, 1, 2, 0, 2);
        }

        // Registro A (0b111)
        0b111 => {
            z80.reg.pc = 0;
            cpu_z.reg.pc = 0;
            z80.reg.a = !binario;
            cpu_z.reg.a = !binario;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            prueba_flags(&z80, 2, 1, 1, 2, 0, 2);

            z80.reg.pc = 0;
            cpu_z.reg.pc = 0;
            z80.reg.a = binario;
            cpu_z.reg.a = binario;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            prueba_flags(&z80, 2, 0, 1, 2, 0, 2);
        }
        _ => panic!("Registro no válido: {:08b}", registro)
    }
}

// prueba_flags():     0 => false, 1 => true, 2 => indiferente
// s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
pub fn bit_reset(z80: &mut Z80, nbit: u8, registro: u8, parametro: u8, cpu_z: &mut CPU) {

    // Reseteamos PC
    z80.reg.pc = 0;
    match registro {
        0b000 => {  // B
            z80.reg.b = 1 << nbit;
            cpu_z.reg.b = 1 << nbit;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            assert_eq!(z80.reg.b, 0);
        }
        0b001 => { // C
            z80.reg.c = 1 << nbit;
            cpu_z.reg.c = 1 << nbit;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            assert_eq!(z80.reg.c, 0, "RES {} C falló", nbit);
        }
        0b010 => { // D
            z80.reg.d = 1 << nbit;
            cpu_z.reg.d = 1 << nbit;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            assert_eq!(z80.reg.d, 0, "RES {} D falló", nbit);
        }
        0b011 => { // E
            z80.reg.e = 1 << nbit;
            cpu_z.reg.e = 1 << nbit;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            assert_eq!(z80.reg.e, 0, "RES {} E falló", nbit);
        }
        0b100 => { // H
            z80.reg.h = 1 << nbit;
            cpu_z.reg.h = 1 << nbit;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            assert_eq!(z80.reg.h, 0, "RES {} H falló", nbit);
        }
        0b101 => { // L
            z80.reg.l = 1 << nbit;
            cpu_z.reg.l = 1 << nbit;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            assert_eq!(z80.reg.l, 0, "RES {} L falló", nbit);
        }
        0b111 => { // A
            z80.reg.a = 1 << nbit;
            cpu_z.reg.a = 1 << nbit;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            assert_eq!(z80.reg.a, 0, "RES {} A falló", nbit);
        }
        _ => panic!("Registro no válido: {:08b}", registro)
    }
}
// prueba_flags():     0 => false, 1 => true, 2 => indiferente
// s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
pub fn bit_set(z80: &mut Z80, nbit: u8, registro: u8, parametro: u8, cpu_z: &mut CPU) {

    // Reseteamos PC
    z80.reg.set_pc(0);
    let expected = 1 << nbit;

    match registro {
        0b000 => {  // B
            z80.reg.b = 0;
            cpu_z.reg.b = 0;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            assert_eq!(z80.reg.b, expected, "SET {} B falló", nbit);
        }
        0b001 => { // C
            z80.reg.c = 0;
            cpu_z.reg.c = 0;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            assert_eq!(z80.reg.c, expected, "SET {} C falló", nbit);
        }
        0b010 => { // D
            z80.reg.d = 0;
            cpu_z.reg.d = 0;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            assert_eq!(z80.reg.d, expected, "SET {} D falló", nbit);
        }
        0b011 => { // E
            z80.reg.e = 0;
            cpu_z.reg.e = 0;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            assert_eq!(z80.reg.e, expected, "SET {} E falló", nbit);
        }
        0b100 => { // H
            z80.reg.h = 0;
            cpu_z.reg.h = 0;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            assert_eq!(z80.reg.h, expected, "SET {} H falló", nbit);
        }
        0b101 => { // L
            z80.reg.l = 0;
            cpu_z.reg.l = 0;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            assert_eq!(z80.reg.l, expected, "SET {} L falló", nbit);
        }
        0b111 => { // A
            z80.reg.a = 0;
            cpu_z.reg.a = 0;
            ejecutar(z80, 0xCB, parametro, 0, 0, cpu_z);
            assert_eq!(z80.reg.a, expected, "SET {} A falló", nbit);
        }
        _ => panic!("Registro no válido: {:08b}", registro)
    }
}

pub fn hacer_push_test_nn(z80: &mut Z80, c: &mut CPU, nn: u16) {
    // pongo HL = 0xE12A
    //z80.registros.h = 0xE1;
    //z80.registros.l = 0x2A;
    let sp_inicial = z80.reg.sp;

    push_nn_test(z80, c, nn);

    // ejecutar push hl
    //ejecutar(z80, 0xE5, 0, 0, 0, c);

    // Comprobar SP
    assert_eq!(z80.reg.sp, sp_inicial.wrapping_sub(2));

    // Comprobar los datos en la memoria que apunta el SP
    assert_eq!(z80.mem.mem[0xFF16], 0x2A);
    assert_eq!(z80.mem.mem[0xFF17], 0xE1);
}

// Setters **************************************************
pub fn set_af_test_big(z80: &mut Z80, cpuz: &mut CPU, v: u16) {
    let a = ((v & 0b1111_1111_0000_0000) >> 8) as u8;
    let f = (v & 0b0000_0000_1111_1111) as u8;
    z80.reg.f = f;
    z80.reg.a = a;
    cpuz.reg.set_af(v);
}
pub fn set_hl_test_big(z80: &mut Z80, cpuz: &mut CPU, v: u16) {
    let h = ((v & 0b1111_1111_0000_0000) >> 8) as u8;
    let l = (v & 0b0000_0000_1111_1111) as u8;
    z80.reg.h = h;
    cpuz.reg.h = h;
    z80.reg.l = l;
    cpuz.reg.l = l;
}
pub fn set_bc_test_big(z80: &mut Z80, cpuz: &mut CPU, v: u16) {
    let b = ((v & 0b1111_1111_0000_0000) >> 8) as u8;
    let c = (v & 0b0000_0000_1111_1111) as u8;
    z80.reg.b = b;
    cpuz.reg.b = b;
    z80.reg.c = c;
    cpuz.reg.c = c;
}
pub fn set_de_test_big(z80: &mut Z80, cpuz: &mut CPU, v: u16) {
    let d = ((v & 0b1111_1111_0000_0000) >> 8) as u8;
    let e = (v & 0b0000_0000_1111_1111) as u8;
    z80.reg.d = d;
    cpuz.reg.d = d;
    z80.reg.e = e;
    cpuz.reg.e = e;
}
pub fn set_sp_test_big(z80: &mut Z80, cpuz: &mut CPU, v: u16) {
    z80.reg.sp = v;
    cpuz.reg.sp = v;
}

// Getters *********************************************************
pub fn get_mem_u8(z80: &mut Z80, cpuz: &mut CPU, direcc: u16) -> u8 {
    let by80 = z80.mem.mem[direcc as usize];
    let byzi = cpuz.bus.read_byte(direcc);
    if z80.debug.debug_zilog {
        assert_eq!(by80, byzi);
    }
    by80
}
pub fn get_hl_test_big(z80: &mut Z80, cpuz: &mut CPU) -> u16 {
    let rz80 = ((z80.reg.h as u16) << 8) | z80.reg.l as u16;
    let rcpuz = cpuz.reg.get_hl();
    if z80.debug.debug_zilog {
        assert_eq!(rz80, rcpuz);
    }

    rz80
}
pub fn get_hlp_test_big(z80: &mut Z80, cpuz: &mut CPU) -> u16 {
    let rz80 = ((z80.reg.hp as u16) << 8) | z80.reg.lp as u16;
    let rcpuz = cpuz.alt.get_hl();
    if z80.debug.debug_zilog {
        assert_eq!(rz80, rcpuz);
    }

    rz80
}
pub fn get_bc_test_big(z80: &mut Z80, cpuz: &mut CPU) -> u16 {
    let rz80 = ((z80.reg.b as u16) << 8) | z80.reg.c as u16;
    let rcpuz = cpuz.reg.get_bc();
    if z80.debug.debug_zilog {
        assert_eq!(rz80, rcpuz);
    }

    rz80
}
pub fn get_bcp_test_big(z80: &mut Z80, cpuz: &mut CPU) -> u16 {
    let rz80 = ((z80.reg.bp as u16) << 8) | z80.reg.cp as u16;
    let rcpuz = cpuz.alt.get_bc();
    if z80.debug.debug_zilog {
        assert_eq!(rz80, rcpuz);
    }

    rz80
}
pub fn get_de_test_big(z80: &mut Z80, cpuz: &mut CPU) -> u16 {
    let rz80 = ((z80.reg.d as u16) << 8) | z80.reg.e as u16;
    let rcpuz = cpuz.reg.get_de();
    if z80.debug.debug_zilog {
        assert_eq!(rz80, rcpuz);
    }

    rz80
}
pub fn get_dep_test_big(z80: &mut Z80, cpuz: &mut CPU) -> u16 {
    let rz80 = ((z80.reg.dp as u16) << 8) | z80.reg.ep as u16;
    let rcpuz = cpuz.alt.get_de();
    if z80.debug.debug_zilog {
        assert_eq!(rz80, rcpuz);
    }

    rz80
}

pub fn get_af_test_big(z80: &mut Z80, cpuz: &mut CPU) -> u16 {
    let rz80 = ((z80.reg.a as u16) << 8) | z80.reg.f as u16;
    let rcpuz = cpuz.reg.get_af();
    if z80.debug.debug_zilog {
        assert_hex!(rz80, rcpuz);
    }

    rz80
}

pub fn get_afp_test_big(z80: &mut Z80, cpuz: &mut CPU) -> u16 {
    let rz80 = ((z80.reg.ap as u16) << 8) | z80.reg.fp as u16;
    let rcpuz = cpuz.alt.get_af();
    if z80.debug.debug_zilog {
        assert_eq!(rz80, rcpuz);
    }

    rz80
}
pub fn get_mem_16_big(z80: &mut Z80, cpuz: &mut CPU, dir: u16) -> u16 {
    let bajo = z80.mem.mem[dir as usize] as u16;
    let alto = (z80.mem.mem[dir as usize + 1] as u16) << 8;

    let memz = cpuz.bus.read_word(dir);

    if z80.debug.debug_zilog {
        assert_eq!(alto | bajo, memz);
    }
    alto | bajo
}
fn push_nn_test(z80: &mut Z80, cpu: &mut CPU, nn: u16) {
    z80.reg.sp = z80.reg.sp - 1;
    cpu.reg.sp = cpu.reg.sp - 1;
    //
    //         let bytes_big_endian = self.get_loc16_big_endian(src).to_be_bytes();
    //
    let alto = ((nn & 0b1111_1111_0000_0000) >> 8) as u8;
    z80.mem.mem[z80.reg.sp as usize] = alto; // pongo el alto
    cpu.bus.write_byte(cpu.reg.sp, alto);
    //
    z80.reg.sp = z80.reg.sp - 1;
    cpu.reg.sp = cpu.reg.sp - 1;

    let bajo = (nn & 0b0000_0000_1111_1111) as u8;
    z80.mem.mem[z80.reg.sp as usize] = bajo; // pongo el bajo
    cpu.bus.write_byte(cpu.reg.sp, bajo);
    //
    //         //self.push_val_big_endian(self.get_loc16_big_endian(src));

}

pub fn graba_mem(z80: &mut Z80, cpu: &mut CPU, direccion: u16, dato: u8) {
    z80.mem.mem[direccion as usize] = dato;
    cpu.bus.write_byte(direccion, dato);
}


