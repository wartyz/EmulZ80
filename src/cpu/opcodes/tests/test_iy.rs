use crate::cpu::opcodes::tests::test_aux::*;
use crate::z80::Z80;
use zilog_z80::cpu::CPU;

#[test]
fn inc_b() {
    // FD 04 inc b Carry no afectado, N=0, PV detecta overflow, H,Z,S se definen
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // TODO
    z80.debug.debug_zilog = false;

    // pongo B = 0xFF
    z80.reg.b = 0xFF;
    c.reg.b = 0xFF;

    ejecutar(&mut z80, 0xFD, 0x04, 0, 0, &mut c);

    // compruebo que ahora b = 0x00
    assert_eq!(z80.reg.b, 0x00);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 2);
}
#[test]
fn dec_b() {
    // FD 05 dec b Carry no afectado, N=1, PV detecta overflow, H,Z,S se definen
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // pongo B = 0x00
    z80.reg.b = 0x00;
    c.reg.b = 0x00;

    ejecutar(&mut z80, 0x05, 0, 0, 0, &mut c);

    // compruebo que ahora b = 0xFF
    assert_eq!(z80.reg.b, 0xFF);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 1, 2);
}
#[test]
fn ld_b_n() {
    // FD 06 ld b,n no afecta alos flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // TODO
    z80.debug.debug_zilog = false;

    ejecutar(&mut z80, 0xFD, 0x06, 0x5A, 0, &mut c);

    // compruebo que ahora b = 0x5A
    assert_eq!(z80.reg.b, 0x5A);
}

#[test]
fn add_iy_bc() {
    // FD 09  IY + BC -> IY     Carry se define, N=0, PV no afectado,  H se define, Z, S no afectados,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // TODO
    z80.debug.debug_zilog = false;

    // pongo IY = 0x1234
    z80.reg.iy = 0x1234;
    c.reg.set_iy(0x1234);

    // pongo BC = 0xB5A4
    //z80.registros.b = 0xB5;
    //z80.registros.c = 0xA4;
    set_bc_test_big(&mut z80, &mut c, 0xB5A4);

    ejecutar(&mut z80, 0xFD, 0x09, 0, 0, &mut c);

    assert_eq!(z80.reg.iy, 0xC7D8);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 2, 2, 0, 2, 0, 0);
}
#[test]
fn ld_0nn0_iy() {
    // FD 22 NH NL       IY -> (NN)    no afecta flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.iy = 0xCD18;
    c.reg.set_iy(0xCD18);

    ejecutar(&mut z80, 0xFD, 0x22, 0x34, 0x12, &mut c);

    assert_eq!(z80.mem.mem[0x1234], 0x18);
    assert_eq!(z80.mem.mem[0x1235], 0xCD);
}

#[test]
fn ld_iy_0nn0() {
    // FD 2A NH NL       IY <- (NN)    no afecta flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    graba_mem(&mut z80, &mut c, 0x1234, 0x18);
    graba_mem(&mut z80, &mut c, 0x1235, 0xCD);

    ejecutar(&mut z80, 0xFD, 0x2A, 0x34, 0x12, &mut c);

    assert_eq!(z80.reg.iy, 0xCD18);
}
#[test]
fn dec0iymasd0() {
    // FD 35 D Carry no afectado, N=1 PV detecta overflow, H,Z,S se definen
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.mem.mem[0x456C] = 0x03;
    c.bus.write_byte(0x456C, 0x03);

    z80.reg.iy = 0x4567;
    c.reg.set_iy(0x4567);
    // iy incrementa 5
    ejecutar(&mut z80, 0xFD, 0x35, 0x05, 0, &mut c);

    // Comprobar que en la direccion 0x456C ahora es 0x02
    assert_eq!(z80.mem.mem[0x456C], 0x02);
}
#[test]
fn ld_0iy_mas_d0_n() {
    // FD 36 D N     ld(iy+d),n   No afecta flags

    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.iy = 0x4500;
    c.reg.set_iy(0x4500);

    ejecutar(&mut z80, 0xFD, 0x36, 0x6C, 0xE6, &mut c);
}
#[test]
fn ld_b_0iy_mas_d0() {
    // FD 46 D     ld b,(iy+d)   No afecta flags

    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.iy = 0x4500;
    c.reg.set_iy(0x4500);

    z80.mem.mem[0x456C] = 0x03;
    c.bus.write_byte(0x456C, 0x03);

    ejecutar(&mut z80, 0xFD, 0x46, 0x6C, 0x00, &mut c);
    assert_eq!(z80.reg.b, 0x03);
}
#[test]
fn ld_c_0iy_mas_d0() {
    // FD 4E D     ld c,(iy+d)   No afecta flags

    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.iy = 0x4500;
    c.reg.set_iy(0x4500);

    z80.mem.mem[0x456C] = 0x03;
    c.bus.write_byte(0x456C, 0x03);

    ejecutar(&mut z80, 0xFD, 0x4E, 0x6C, 0x00, &mut c);
    assert_eq!(z80.reg.c, 0x03);
}

#[test]
fn ld_d_0iy_mas_d0() {
    // FD 56 D     ld c,(iy+d)   No afecta flags

    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.iy = 0x4500;
    c.reg.set_iy(0x4500);

    z80.mem.mem[0x456C] = 0x03;
    c.bus.write_byte(0x456C, 0x03);

    ejecutar(&mut z80, 0xFD, 0x56, 0x6C, 0x00, &mut c);
    assert_eq!(z80.reg.d, 0x03);
}

#[test]
fn ld_e_0iy_mas_d0() {
    // FD 5E D     ld c,(iy+d)   No afecta flags

    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.iy = 0x4500;
    c.reg.set_iy(0x4500);

    z80.mem.mem[0x456C] = 0x03;
    c.bus.write_byte(0x456C, 0x03);

    ejecutar(&mut z80, 0xFD, 0x5E, 0x6C, 0x00, &mut c);
    assert_eq!(z80.reg.e, 0x03);
}

#[test]
fn ld_h_0iy_mas_d0() {
    // FD 66 D     ld c,(iy+d)   No afecta flags

    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.iy = 0x4500;
    c.reg.set_iy(0x4500);

    z80.mem.mem[0x456C] = 0x03;
    c.bus.write_byte(0x456C, 0x03);

    ejecutar(&mut z80, 0xFD, 0x66, 0x6C, 0x00, &mut c);
    assert_eq!(z80.reg.h, 0x03);
}
#[test]
fn ld_l_0iy_mas_d0() {
    // FD 6E D     ld c,(iy+d)   No afecta flags

    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.iy = 0x4500;
    c.reg.set_iy(0x4500);

    z80.mem.mem[0x456C] = 0x03;
    c.bus.write_byte(0x456C, 0x03);

    ejecutar(&mut z80, 0xFD, 0x6E, 0x6C, 0x00, &mut c);
    assert_eq!(z80.reg.l, 0x03);
}

#[test]
fn ld_a_0iy_mas_d0() {
    // FD 7E D     ld c,(iy+d)   No afecta flags

    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.iy = 0x4500;
    c.reg.set_iy(0x4500);

    z80.mem.mem[0x456C] = 0x03;
    c.bus.write_byte(0x456C, 0x03);

    ejecutar(&mut z80, 0xFD, 0x7E, 0x6C, 0x00, &mut c);
    assert_eq!(z80.reg.a, 0x03);
}
#[test]
fn ld_sp_iy() {
    // FD F9 NL NH       No afecta flags

    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.mem.mem[0x45AB] = 0x03;
    c.bus.write_byte(0x45AB, 0x03);

    z80.reg.iy = 0x45AB;
    c.reg.set_iy(0x45AB);

    ejecutar(&mut z80, 0xFD, 0xF9, 0x00, 0x00, &mut c);

    assert_eq!(z80.reg.sp, 0x45AB);
}