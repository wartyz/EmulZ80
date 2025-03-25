use crate::cpu::opcodes::tests::test_aux::*;
use crate::ops::StatusFlag;
use crate::z80;
use crate::z80::Z80;
//use crate::z80;
use zilog_z80::cpu::CPU;

#[test]
fn add_a_b() {
    // 0x80  A + B -> A     Carry se define, N=0, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Caso 1: Sin resultado con acarreo ------------------------------
    z80.reg.a = 0xCD;
    z80.reg.b = 0x02;
    c.reg.a = 0xCD;
    c.reg.b = 0x02;

    ejecutar(&mut z80, 0x80, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCF); // compruebo que A es 0xCF

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // Caso 2: Con resultado con acarreo ------------------------------
    z80.reg.set_pc(0); // reseteo pc
    c.reg.pc = 0;
    z80.reg.a = 0xFF;
    z80.reg.b = 0x01;
    c.reg.a = 0xFF;
    c.reg.b = 0x01;

    ejecutar(&mut z80, 0x80, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);
}

#[test]
fn add_a_c() {
    // 0x81  A + C -> A     Carry se define, N=0, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Caso 1: Sin resultado con acarreo ------------------------------
    z80.reg.a = 0xCD;
    z80.reg.c = 0x02;
    c.reg.a = 0xCD;
    c.reg.c = 0x02;

    ejecutar(&mut z80, 0x81, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCF); // compruebo que A es 0xCF

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // Caso 2: Con resultado con acarreo ------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.a = 0xFF;
    z80.reg.c = 0x01;
    c.reg.a = 0xFF;
    c.reg.c = 0x01;

    ejecutar(&mut z80, 0x81, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);
}

#[test]
fn add_a_d() {
    // 0x82  A + D -> A     Carry se define, N=0, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Caso 1: Sin resultado con acarreo ------------------------------
    z80.reg.a = 0xCD;
    z80.reg.d = 0x02;
    c.reg.a = 0xCD;
    c.reg.d = 0x02;

    ejecutar(&mut z80, 0x82, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCF); // compruebo que A es 0xCF

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // Caso 2: Con resultado con acarreo ------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;

    z80.reg.a = 0xFF;
    z80.reg.d = 0x01;
    c.reg.a = 0xFF;
    c.reg.d = 0x01;

    ejecutar(&mut z80, 0x82, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);
}

#[test]
fn add_a_e() {
    // 0x83  A + E -> A     Carry se define, N=0, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Caso 1: Sin resultado con acarreo ------------------------------
    z80.reg.a = 0xCD;
    z80.reg.e = 0x02;
    c.reg.a = 0xCD;
    c.reg.e = 0x02;

    ejecutar(&mut z80, 0x83, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCF); // compruebo que A es 0xCF

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // Caso 2: Con resultado con acarreo ------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.a = 0xFF;
    z80.reg.e = 0x01;
    c.reg.a = 0xFF;
    c.reg.e = 0x01;

    ejecutar(&mut z80, 0x83, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);
}

#[test]
fn add_a_h() {
    // 0x84  A + H -> A     Carry se define, N=0, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Caso 1: Sin resultado con acarreo ------------------------------
    z80.reg.a = 0xCD;
    z80.reg.h = 0x02;
    c.reg.a = 0xCD;
    c.reg.h = 0x02;
    ejecutar(&mut z80, 0x84, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCF); // compruebo que A es 0xCF

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // Caso 2: Con resultado con acarreo ------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.a = 0xFF;
    z80.reg.h = 0x01;
    c.reg.a = 0xFF;
    c.reg.h = 0x01;

    ejecutar(&mut z80, 0x84, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);
}
#[test]
fn add_a_l() {
    // 0x85  A + L -> A     Carry se define, N=0, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Caso 1: Sin resultado con acarreo ------------------------------
    z80.reg.a = 0xCD;
    z80.reg.l = 0x02;
    c.reg.a = 0xCD;
    c.reg.l = 0x02;

    ejecutar(&mut z80, 0x85, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCF); // compruebo que A es 0xCF

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // Caso 2: Con resultado con acarreo ------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.a = 0xFF;
    z80.reg.l = 0x01;
    c.reg.a = 0xFF;
    c.reg.l = 0x01;

    ejecutar(&mut z80, 0x85, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);
}

#[test]
fn add_a_0hl0() {
    // 0x86  A + (HL) -> A     Carry se define, N=0, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Caso 1: Sin resultado con acarreo ------------------------------
    z80.reg.a = 0xCD;
    c.reg.a = 0xCD;

    // pongo en la direccion 0xEF10 el valor 0x02
    z80.mem.mem[0xEF10] = 0x02;
    c.bus.write_byte(0xEF10, 0x02);

    // pongo HL = 0xEF10
    // z80.registros.h = 0xEF;
    // z80.registros.l = 0x10;
    // c.reg.set_hl(0xEF10);
    set_hl_test_big(&mut z80, &mut c, 0xEF10);

    ejecutar(&mut z80, 0x86, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCF); // compruebo que A es 0xCF

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // Caso 2: Con resultado con acarreo ------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.a = 0xFF;
    z80.mem.mem[0xEF10] = 0x01;
    c.reg.a = 0xFF;
    c.bus.write_byte(0xEF10, 0x01);

    ejecutar(&mut z80, 0x86, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);
}

#[test]
fn add_a_a() {
    // 0x87  A + A -> A     Carry se define, N=0, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Caso 1: Sin resultado con acarreo ------------------------------
    z80.reg.a = 0x07;
    c.reg.a = 0x07;

    ejecutar(&mut z80, 0x87, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x0E); // compruebo que A es 0xCF

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // Caso 2: Con resultado con acarreo ------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.a = 0x80;
    c.reg.a = 0x80;

    ejecutar(&mut z80, 0x87, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 0, 1, 0, 1);
}

#[test]
fn adc_a_b() {
    // 0x88  A + L +Carry -> A     Carry se define, N=0, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1a: Sin resultado con acarreo y acarreo anterior------------------------------
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;

    z80.reg.a = 0xCD;
    z80.reg.b = 0x02;
    c.reg.a = 0xCD;
    c.reg.b = 0x02;

    ejecutar(&mut z80, 0x88, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xD0); // compruebo que A es

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 0, 0);

    // Caso 1b: Sin resultado con acarreo y sin acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;

    z80.reg.a = 0xCD;
    z80.reg.b = 0x02;
    c.reg.a = 0xCD;
    c.reg.b = 0x02;

    ejecutar(&mut z80, 0x88, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCF); // compruebo que A es

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // Caso 2a: Con resultado con acarreo y acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;

    z80.reg.a = 0xFE;
    z80.reg.b = 0x01;
    c.reg.a = 0xFE;
    c.reg.b = 0x01;

    ejecutar(&mut z80, 0x88, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);

    // Caso 2b: Con resultado con acarreo y sin acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xFF;
    z80.reg.b = 0x01;
    c.reg.a = 0xFF;
    c.reg.b = 0x01;

    ejecutar(&mut z80, 0x88, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);
}

#[test]
fn adc_a_c() {
    // 0x89  A + C +Carry -> A     Carry se define, N=0, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1a: Sin resultado con acarreo y acarreo anterior------------------------------
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0xCD;
    z80.reg.c = 0x02;
    c.reg.a = 0xCD;
    c.reg.c = 0x02;

    ejecutar(&mut z80, 0x89, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xD0); // compruebo que A es

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 0, 0);

    // Caso 1b: Sin resultado con acarreo y sin acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xCD;
    z80.reg.c = 0x02;
    c.reg.a = 0xCD;
    c.reg.c = 0x02;

    ejecutar(&mut z80, 0x89, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCF); // compruebo que A es

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // Caso 2a: Con resultado con acarreo y acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0xFE;
    z80.reg.c = 0x01;
    c.reg.a = 0xFE;
    c.reg.c = 0x01;

    ejecutar(&mut z80, 0x89, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);

    // Caso 2b: Con resultado con acarreo y sin acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xFF;
    z80.reg.c = 0x01;
    c.reg.a = 0xFF;
    c.reg.c = 0x01;

    ejecutar(&mut z80, 0x89, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);
}

#[test]
fn adc_a_d() {
    // 0x8A  A + D +Carry -> A     Carry se define, N=0, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1a: Sin resultado con acarreo y acarreo anterior------------------------------
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0xCD;
    z80.reg.d = 0x02;
    c.reg.a = 0xCD;
    c.reg.d = 0x02;
    ejecutar(&mut z80, 0x8A, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xD0); // compruebo que A es

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 0, 0);

    // Caso 1b: Sin resultado con acarreo y sin acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xCD;
    z80.reg.d = 0x02;
    c.reg.a = 0xCD;
    c.reg.d = 0x02;
    ejecutar(&mut z80, 0x8A, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCF); // compruebo que A es

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // Caso 2a: Con resultado con acarreo y acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0xFE;
    z80.reg.d = 0x01;
    c.reg.a = 0xFE;
    c.reg.d = 0x01;

    ejecutar(&mut z80, 0x8A, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);

    // Caso 2b: Con resultado con acarreo y sin acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xFF;
    z80.reg.d = 0x01;
    c.reg.a = 0xFF;
    c.reg.d = 0x01;

    ejecutar(&mut z80, 0x8A, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);
}

#[test]
fn adc_a_e() {
    // 0x8B  A + E +Carry -> A     Carry se define, N=0, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1a: Sin resultado con acarreo y acarreo anterior------------------------------
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0xCD;
    z80.reg.e = 0x02;
    c.reg.a = 0xCD;
    c.reg.e = 0x02;
    ejecutar(&mut z80, 0x8B, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xD0); // compruebo que A es

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 0, 0);

    // Caso 1b: Sin resultado con acarreo y sin acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xCD;
    z80.reg.e = 0x02;
    c.reg.a = 0xCD;
    c.reg.e = 0x02;
    ejecutar(&mut z80, 0x8B, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCF); // compruebo que A es

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // Caso 2a: Con resultado con acarreo y acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0xFE;
    z80.reg.e = 0x01;
    c.reg.a = 0xFE;
    c.reg.e = 0x01;

    ejecutar(&mut z80, 0x8B, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);

    // Caso 2b: Con resultado con acarreo y sin acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xFF;
    z80.reg.e = 0x01;
    c.reg.a = 0xFF;
    c.reg.e = 0x01;

    ejecutar(&mut z80, 0x8B, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);
}
#[test]
fn adc_a_h() {
    // 0x8C  A + H +Carry -> A     Carry se define, N=0, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1a: Sin resultado con acarreo y acarreo anterior------------------------------
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0xCD;
    z80.reg.h = 0x02;
    c.reg.a = 0xCD;
    c.reg.h = 0x02;
    ejecutar(&mut z80, 0x8C, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xD0); // compruebo que A es

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 0, 0);

    // Caso 1b: Sin resultado con acarreo y sin acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xCD;
    z80.reg.h = 0x02;
    c.reg.a = 0xCD;
    c.reg.h = 0x02;
    ejecutar(&mut z80, 0x8C, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCF); // compruebo que A es

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // Caso 2a: Con resultado con acarreo y acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0xFE;
    z80.reg.h = 0x01;
    c.reg.a = 0xFE;
    c.reg.h = 0x01;

    ejecutar(&mut z80, 0x8C, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);

    // Caso 2b: Con resultado con acarreo y sin acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xFF;
    z80.reg.h = 0x01;
    c.reg.a = 0xFF;
    c.reg.h = 0x01;

    ejecutar(&mut z80, 0x8C, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);
}
#[test]
fn adc_a_l() {
    // 0x8D  A + L +Carry -> A     Carry se define, N=0, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1a: Sin resultado con acarreo y acarreo anterior------------------------------
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0xCD;
    z80.reg.l = 0x02;
    c.reg.a = 0xCD;
    c.reg.l = 0x02;
    ejecutar(&mut z80, 0x8D, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xD0); // compruebo que A es

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 0, 0);

    // Caso 1b: Sin resultado con acarreo y sin acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xCD;
    z80.reg.l = 0x02;
    c.reg.a = 0xCD;
    c.reg.l = 0x02;
    ejecutar(&mut z80, 0x8D, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCF); // compruebo que A es

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // Caso 2a: Con resultado con acarreo y acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0xFE;
    z80.reg.l = 0x01;
    c.reg.a = 0xFE;
    c.reg.l = 0x01;

    ejecutar(&mut z80, 0x8D, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);

    // Caso 2b: Con resultado con acarreo y sin acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xFF;
    z80.reg.l = 0x01;
    c.reg.a = 0xFF;
    c.reg.l = 0x01;

    ejecutar(&mut z80, 0x8D, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);
}
#[test]
fn adc_a_0hl0() {
    // 0x8E  A + (HL)) +Carry -> A     Carry se define, N=0, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1a: Sin resultado con acarreo y acarreo anterior------------------------------
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0xCD;
    c.reg.a = 0xCD;
    // pongo en la direccion 0xEF10 el valor 0x02
    z80.mem.mem[0xEF10] = 0x02;
    c.bus.write_byte(0xEF10, 0x02);

    // pongo HL = 0xEF10
    // z80.registros.h = 0xEF;
    // z80.registros.l = 0x10;
    // c.reg.h = 0xEF;
    // c.reg.l = 0x10;
    set_hl_test_big(&mut z80, &mut c, 0xEF10);
    ejecutar(&mut z80, 0x8E, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xD0); // compruebo que A es

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 0, 0);

    // Caso 1b: Sin resultado con acarreo y sin acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xCD;
    c.reg.a = 0xCD;

    ejecutar(&mut z80, 0x8E, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCF); // compruebo que A es

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // Caso 2a: Con resultado con acarreo y acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0xFE;
    c.reg.a = 0xFE;

    // pongo en la direccion 0x10EF el valor 0xCD
    z80.mem.mem[0xEF10] = 0x01;
    c.bus.write_byte(0xEF10, 0x01);

    ejecutar(&mut z80, 0x8E, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);

    // Caso 2b: Con resultado con acarreo y sin acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xFF;
    c.reg.a = 0xFF;
    // pongo en la direccion 0x10EF el valor 0xCD
    z80.mem.mem[0xEF10] = 0x01;
    c.bus.write_byte(0xEF10, 0x01);

    ejecutar(&mut z80, 0x8E, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);
}
#[test]
fn adc_a_a() {
    // 0x8F  A + A +Carry -> A     Carry se define, N=0, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1a: Sin resultado con acarreo y acarreo anterior------------------------------
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0x07;
    c.reg.a = 0x07;

    ejecutar(&mut z80, 0x8F, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x0F); // compruebo que A es

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // Caso 1b: Sin resultado con acarreo y sin acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0x80;
    c.reg.a = 0x80;

    ejecutar(&mut z80, 0x8F, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 0, 1, 0, 1);

    // Caso 2a: Con resultado con acarreo y acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0x80;
    c.reg.a = 0x80;

    ejecutar(&mut z80, 0x8F, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x01); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);

    // Caso 2b: Con resultado con acarreo y sin acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xFF;
    z80.reg.l = 0x01;
    c.reg.a = 0xFF;
    c.reg.l = 0x01;

    ejecutar(&mut z80, 0x8D, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);
}

#[test]
fn sub_b() {
    // 0x90  A - B -> A     Carry se define, N=1, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1: Resta sin acarreo ni overflow
    z80.reg.a = 0xCD;
    z80.reg.b = 0x02;
    c.reg.a = 0xCD;
    c.reg.b = 0x02;

    ejecutar(&mut z80, 0x90, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCB, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 1, 0);

    // Caso 2: Resta con acarreo (borrow) en nibble bajo y total
    z80.reg.pc = 0;  // Resetea contador de programa
    c.reg.pc = 0;
    z80.reg.a = 0x01;
    z80.reg.b = 0x02;
    c.reg.a = 0x01;
    c.reg.b = 0x02;
    ejecutar(&mut z80, 0x90, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xFF, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);

    // Caso 3: Overflow en operación con signo
    z80.reg.pc = 0;
    c.reg.pc = 0;
    z80.reg.a = 0x80;  // -128 (complemento a 2)
    z80.reg.b = 0x01;
    c.reg.a = 0x80;
    c.reg.b = 0x01;

    ejecutar(&mut z80, 0x90, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x7F, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 1, 1, 0);
}

#[test]
fn sub_c() {
    // 0x91  A - C -> A     Carry se define, N=1, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1: Resta sin acarreo ni overflow
    z80.reg.a = 0xCD;
    z80.reg.c = 0x02;
    c.reg.a = 0xCD;
    c.reg.c = 0x02;

    ejecutar(&mut z80, 0x91, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCB, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 1, 0);

    // Caso 2: Resta con acarreo (borrow) en nibble bajo y total
    z80.reg.pc = 0;  // Resetea contador de programa
    c.reg.pc = 0;
    z80.reg.a = 0x01;
    z80.reg.c = 0x02;
    c.reg.a = 0x01;
    c.reg.c = 0x02;
    ejecutar(&mut z80, 0x91, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xFF, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);

    // Caso 3: Overflow en operación con signo
    z80.reg.pc = 0;
    c.reg.pc = 0;
    z80.reg.a = 0x80;  // -128 (complemento a 2)
    z80.reg.c = 0x01;
    c.reg.a = 0x80;
    c.reg.c = 0x01;
    ejecutar(&mut z80, 0x91, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x7F, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 1, 1, 0);
}
#[test]
fn sub_d() {
    // 0x92  A - D -> A     Carry se define, N=1, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1: Resta sin acarreo ni overflow
    z80.reg.a = 0xCD;
    z80.reg.d = 0x02;
    c.reg.a = 0xCD;
    c.reg.d = 0x02;

    ejecutar(&mut z80, 0x92, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCB, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 1, 0);

    // Caso 2: Resta con acarreo (borrow) en nibble bajo y total
    z80.reg.pc = 0;  // Resetea contador de programa
    c.reg.pc = 0;
    z80.reg.a = 0x01;
    z80.reg.d = 0x02;
    c.reg.a = 0x01;
    c.reg.d = 0x02;
    ejecutar(&mut z80, 0x92, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xFF, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);

    // Caso 3: Overflow en operación con signo
    z80.reg.pc = 0;
    c.reg.pc = 0;
    z80.reg.a = 0x80;  // -128 (complemento a 2)
    z80.reg.d = 0x01;
    c.reg.a = 0x80;
    c.reg.d = 0x01;
    ejecutar(&mut z80, 0x92, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x7F, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 1, 1, 0);
}
#[test]
fn sub_e() {
    // 0x93  A - E -> A     Carry se define, N=1, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1: Resta sin acarreo ni overflow
    z80.reg.a = 0xCD;
    z80.reg.e = 0x02;
    c.reg.a = 0xCD;
    c.reg.e = 0x02;

    ejecutar(&mut z80, 0x93, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCB, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 1, 0);

    // Caso 2: Resta con acarreo (borrow) en nibble bajo y total
    z80.reg.pc = 0;  // Resetea contador de programa
    c.reg.pc = 0;
    z80.reg.a = 0x01;
    z80.reg.e = 0x02;
    c.reg.a = 0x01;
    c.reg.e = 0x02;
    ejecutar(&mut z80, 0x93, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xFF, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);

    // Caso 3: Overflow en operación con signo
    z80.reg.pc = 0;
    c.reg.pc = 0;
    z80.reg.a = 0x80;  // -128 (complemento a 2)
    z80.reg.e = 0x01;
    c.reg.a = 0x80;
    c.reg.e = 0x01;
    ejecutar(&mut z80, 0x93, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x7F, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 1, 1, 0);
}

#[test]
fn sub_h() {
    // 0x94  A - H -> A     Carry se define, N=1, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1: Resta sin acarreo ni overflow
    z80.reg.a = 0xCD;
    z80.reg.h = 0x02;
    c.reg.a = 0xCD;
    c.reg.h = 0x02;

    ejecutar(&mut z80, 0x94, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCB, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 1, 0);

    // Caso 2: Resta con acarreo (borrow) en nibble bajo y total
    z80.reg.pc = 0;  // Resetea contador de programa
    c.reg.pc = 0;
    z80.reg.a = 0x01;
    z80.reg.h = 0x02;
    c.reg.a = 0x01;
    c.reg.h = 0x02;
    ejecutar(&mut z80, 0x94, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xFF, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);

    // Caso 3: Overflow en operación con signo
    z80.reg.pc = 0;
    c.reg.pc = 0;
    z80.reg.a = 0x80;  // -128 (complemento a 2)
    z80.reg.h = 0x01;
    c.reg.a = 0x80;
    c.reg.h = 0x01;

    ejecutar(&mut z80, 0x94, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x7F, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 1, 1, 0);
}
#[test]
fn sub_l() {
    // 0x95  A - L -> A     Carry se define, N=1, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1: Resta sin acarreo ni overflow
    z80.reg.a = 0xCD;
    z80.reg.l = 0x02;
    c.reg.a = 0xCD;
    c.reg.l = 0x02;

    ejecutar(&mut z80, 0x95, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCB, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 1, 0);

    // Caso 2: Resta con acarreo (borrow) en nibble bajo y total
    z80.reg.pc = 0;  // Resetea contador de programa
    c.reg.pc = 0;
    z80.reg.a = 0x01;
    z80.reg.l = 0x02;
    c.reg.a = 0x01;
    c.reg.l = 0x02;
    ejecutar(&mut z80, 0x95, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xFF, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);

    // Caso 3: Overflow en operación con signo
    z80.reg.pc = 0;
    c.reg.pc = 0;
    z80.reg.a = 0x80;  // -128 (complemento a 2)
    z80.reg.l = 0x01;
    c.reg.a = 0x80;
    c.reg.l = 0x01;
    ejecutar(&mut z80, 0x95, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x7F, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 1, 1, 0);
}

#[test]
fn sub_0hl0() {
    // 0x96  A - (HL) -> A     Carry se define, N=1, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1: Resta sin acarreo ni overflow
    z80.reg.a = 0xCD;
    c.reg.a = 0xCD;

    // pongo en la direccion 0xEF10 el valor 0x02
    z80.mem.mem[0xEF10] = 0x02;
    c.bus.write_byte(0x0EF10, 0x02);

    // pongo HL = 0xEF10
    // z80.registros.h = 0xEF;
    // z80.registros.l = 0x10;
    // c.reg.set_hl(0xEF10);
    set_hl_test_big(&mut z80, &mut c, 0xEF10);

    ejecutar(&mut z80, 0x96, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCB, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 1, 0);

    // Caso 2: Resta con acarreo (borrow) en nibble bajo y total
    z80.reg.pc = 0;  // Resetea contador de programa
    c.reg.pc = 0;

    z80.reg.a = 0x01;
    c.reg.a = 0x01;

    // pongo en la direccion 0x10EF el valor 0x02
    z80.mem.mem[0xEF10] = 0x02;
    c.bus.write_byte(0x0EF10, 0x02);

    // pongo HL = 0xEF10
    // z80.registros.h = 0xEF;
    // z80.registros.l = 0x10;
    //c.reg.set_hl(0xEF10);

    set_hl_test_big(&mut z80, &mut c, 0xEF10);

    ejecutar(&mut z80, 0x96, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xFF, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);

    // Caso 3: Overflow en operación con signo
    z80.reg.pc = 0;
    c.reg.pc = 0;

    z80.reg.a = 0x80;  // -128 (complemento a 2)
    c.reg.a = 0x80;

    // pongo en la direccion 0xEF10 el valor 0x02
    z80.mem.mem[0xEF10] = 0x01;
    c.bus.write_byte(0xEF10, 0x01);

    // pongo HL = 0xEF10
    //z80.registros.h = 0xEF;
    //z80.registros.l = 0x10;
    //c.reg.set_hl(0xEF10);
    set_hl_test_big(&mut z80, &mut c, 0xEF10);

    ejecutar(&mut z80, 0x96, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x7F, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 1, 1, 0);
}

#[test]
fn sub_a() {
    // 0x97  A - A -> A     Carry se define, N=1, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1: Resta sin acarreo ni overflow
    z80.reg.a = 0xCD;
    c.reg.a = 0xCD;

    ejecutar(&mut z80, 0x97, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 0, 0, 1, 0);

    // Caso 2: Resta con acarreo (borrow) en nibble bajo y total
    z80.reg.pc = 0;  // Resetea contador de programa
    c.reg.pc = 0;
    z80.reg.a = 0x01;
    c.reg.a = 0x01;

    ejecutar(&mut z80, 0x97, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 0, 0, 1, 0);

    // Caso 3: Overflow en operación con signo
    z80.reg.pc = 0;
    c.reg.pc = 0;
    z80.reg.a = 0x80;  // -128 (complemento a 2)
    c.reg.a = 0x80;

    ejecutar(&mut z80, 0x97, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 0, 0, 1, 0);
}

#[test]
fn sbc_a_b() {
    // 0x98  A - B - Carry -> A     Carry se define, N=1, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1: Resta sin acarreo ni overflow resultante y acarreo anterior = 0
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xCD;
    z80.reg.b = 0x02;
    c.reg.a = 0xCD;
    c.reg.b = 0x02;

    ejecutar(&mut z80, 0x98, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCB, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 1, 0);

    // Caso 2: Resta con acarreo (borrow) en nibble bajo y total y acarreo anterior = 1
    z80.reg.pc = 0;  // Resetea contador de programa
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0x01;
    c.reg.a = 0x01;
    z80.reg.b = 0x02;
    c.reg.b = 0x02;
    ejecutar(&mut z80, 0x98, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xFE, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);

    // Caso 3: Overflow en operación con signo y acarreo anterior = 1
    z80.reg.pc = 0;
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0x80;  // -128 (complemento a 2)
    c.reg.a = 0x80;
    z80.reg.b = 0x01;
    c.reg.b = 0x01;
    ejecutar(&mut z80, 0x98, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x7E, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 1, 1, 0);
}
#[test]
fn sbc_a_c() {
    // 0x99  A - C - Carry -> A     Carry se define, N=1, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1: Resta sin acarreo ni overflow resultante y acarreo anterior = 0
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xCD;
    c.reg.a = 0xCD;
    z80.reg.c = 0x02;
    c.reg.c = 0x02;

    ejecutar(&mut z80, 0x99, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCB, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 1, 0);

    // Caso 2: Resta con acarreo (borrow) en nibble bajo y total y acarreo anterior = 1
    z80.reg.pc = 0;  // Resetea contador de programa
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0x01;
    c.reg.a = 0x01;
    z80.reg.c = 0x02;
    c.reg.c = 0x02;
    ejecutar(&mut z80, 0x99, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xFE, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);

    // Caso 3: Overflow en operación con signo y acarreo anterior = 1
    z80.reg.pc = 0;
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0x80;  // -128 (complemento a 2)
    c.reg.a = 0x80;
    z80.reg.c = 0x01;
    c.reg.c = 0x01;
    ejecutar(&mut z80, 0x99, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x7E, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 1, 1, 0);
}

#[test]
fn sbc_a_d() {
    // 0x9A  A - D - Carry -> A     Carry se define, N=1, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1: Resta sin acarreo ni overflow resultante y acarreo anterior = 0
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xCD;
    c.reg.a = 0xCD;
    z80.reg.d = 0x02;
    c.reg.d = 0x02;

    ejecutar(&mut z80, 0x9A, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCB, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 1, 0);

    // Caso 2: Resta con acarreo (borrow) en nibble bajo y total y acarreo anterior = 1
    z80.reg.pc = 0;  // Resetea contador de programa
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0x01;
    c.reg.a = 0x01;
    z80.reg.d = 0x02;
    c.reg.d = 0x02;
    ejecutar(&mut z80, 0x9A, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xFE, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);

    // Caso 3: Overflow en operación con signo y acarreo anterior = 1
    z80.reg.pc = 0;
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0x80;  // -128 (complemento a 2)
    c.reg.a = 0x80;
    z80.reg.d = 0x01;
    c.reg.d = 0x01;
    ejecutar(&mut z80, 0x9A, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x7E, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 1, 1, 0);
}

#[test]
fn sbc_a_e() {
    // 0x9B  A - E - Carry -> A     Carry se define, N=1, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1: Resta sin acarreo ni overflow resultante y acarreo anterior = 0
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xCD;
    c.reg.a = 0xCD;
    z80.reg.e = 0x02;
    c.reg.e = 0x02;

    ejecutar(&mut z80, 0x9B, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCB, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 1, 0);

    // Caso 2: Resta con acarreo (borrow) en nibble bajo y total y acarreo anterior = 1
    z80.reg.pc = 0;  // Resetea contador de programa
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0x01;
    c.reg.a = 0x01;
    z80.reg.e = 0x02;
    c.reg.e = 0x02;
    ejecutar(&mut z80, 0x9B, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xFE, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);

    // Caso 3: Overflow en operación con signo y acarreo anterior = 1
    z80.reg.pc = 0;
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0x80;  // -128 (complemento a 2)
    c.reg.a = 0x80;
    z80.reg.e = 0x01;
    c.reg.e = 0x01;
    ejecutar(&mut z80, 0x9B, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x7E, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 1, 1, 0);
}
#[test]
fn sbc_a_h() {
    // 0x9C  A - H - Carry -> A     Carry se define, N=1, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1: Resta sin acarreo ni overflow resultante y acarreo anterior = 0
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xCD;
    c.reg.a = 0xCD;
    z80.reg.h = 0x02;
    c.reg.h = 0x02;

    ejecutar(&mut z80, 0x9C, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCB, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 1, 0);

    // Caso 2: Resta con acarreo (borrow) en nibble bajo y total y acarreo anterior = 1
    z80.reg.pc = 0;  // Resetea contador de programa
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0x01;
    c.reg.a = 0x01;
    z80.reg.h = 0x02;
    c.reg.h = 0x02;
    ejecutar(&mut z80, 0x9C, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xFE, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);

    // Caso 3: Overflow en operación con signo y acarreo anterior = 1
    z80.reg.pc = 0;
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0x80;  // -128 (complemento a 2)
    c.reg.a = 0x80;
    z80.reg.h = 0x01;
    c.reg.h = 0x01;
    ejecutar(&mut z80, 0x9C, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x7E, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 1, 1, 0);
}
#[test]
fn sbc_a_l() {
    // 0x9D  A - L - Carry -> A     Carry se define, N=1, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1: Resta sin acarreo ni overflow resultante y acarreo anterior = 0
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xCD;
    c.reg.a = 0xCD;
    z80.reg.l = 0x02;
    c.reg.l = 0x02;

    ejecutar(&mut z80, 0x9D, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCB, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 1, 0);

    // Caso 2: Resta con acarreo (borrow) en nibble bajo y total y acarreo anterior = 1
    z80.reg.pc = 0;  // Resetea contador de programa
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0x01;
    c.reg.a = 0x01;
    z80.reg.l = 0x02;
    c.reg.l = 0x02;
    ejecutar(&mut z80, 0x9D, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xFE, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);

    // Caso 3: Overflow en operación con signo y acarreo anterior = 1
    z80.reg.pc = 0;
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0x80;  // -128 (complemento a 2)
    c.reg.a = 0x80;
    z80.reg.l = 0x01;
    c.reg.l = 0x01;
    ejecutar(&mut z80, 0x9D, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x7E, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 1, 1, 0);
}

#[test]
fn sbc_a_0hl0() {
    // 0x9E  A - (HL) - Carry -> A     Carry se define, N=1, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1: Resta sin acarreo ni overflow resultante y acarreo anterior = 0
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xCD;
    c.reg.a = 0xCD;
    // pongo en la direccion 0xEF10 el valor 0x02
    z80.mem.mem[0xEF10] = 0x02;
    c.bus.write_byte(0xEF10, 0x02);

    // pongo HL = 0xEF10
    //z80.registros.h = 0xEF;
    //z80.registros.l = 0x10;
    set_hl_test_big(&mut z80, &mut c, 0xEF10);

    ejecutar(&mut z80, 0x9E, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCB, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 1, 0);

    // Caso 2: Resta con acarreo (borrow) en nibble bajo y total y acarreo anterior = 1
    z80.reg.pc = 0;  // Resetea contador de programa
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0x01;
    c.reg.a = 0x01;
    // pongo en la direccion 0xEF10 el valor 0x02
    z80.mem.mem[0xEF10] = 0x02;
    c.bus.write_byte(0xEF10, 0x02);

    // pongo HL = 0xEF10
    //z80.registros.h = 0xEF;
    //z80.registros.l = 0x10;
    set_hl_test_big(&mut z80, &mut c, 0xEF10);
    ejecutar(&mut z80, 0x9E, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xFE, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);

    // Caso 3: Overflow en operación con signo y acarreo anterior = 1
    z80.reg.pc = 0;
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0x80;  // -128 (complemento a 2)
    c.reg.a = 0x80;
    // pongo en la direccion 0x10EF el valor 0x02
    z80.mem.mem[0xEF10] = 0x01;
    c.bus.write_byte(0xEF10, 0x01);

    // pongo HL = 0x10EF
    //z80.registros.h = 0xEF;
    //z80.registros.l = 0x10;
    set_hl_test_big(&mut z80, &mut c, 0xEF10);
    ejecutar(&mut z80, 0x9E, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x7E, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 1, 1, 0);
}

#[test]
fn sbc_a_a() {
    // 0x9F  A - A - Carry -> A     Carry se define, N=1, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // Caso 1: Resta sin acarreo ni overflow resultante y acarreo anterior = 0
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xCD;
    c.reg.a = 0xCD;

    ejecutar(&mut z80, 0x9F, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 0, 0, 1, 0);

    // Caso 2: Resta con acarreo (borrow) en nibble bajo y total y acarreo anterior = 1
    z80.reg.pc = 0;  // Resetea contador de programa
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0x01;
    c.reg.a = 0x01;

    ejecutar(&mut z80, 0x9F, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xFF, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);

    // Caso 3: Overflow en operación con signo y acarreo anterior = 1
    z80.reg.pc = 0;
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0x80;  // -128 (complemento a 2)
    c.reg.a = 0x80;

    ejecutar(&mut z80, 0x9F, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xFF, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);
}

#[test]
fn and_b() {
    // 0xA0  A & B -> A     Carry = 0, N=0, PV detecta paridad,  H=1, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.b = 0x09;
    c.reg.b = 0x09;

    ejecutar(&mut z80, 0xA0, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x08, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 0, 0, 0);
}
#[test]
fn and_c() {
    // 0xA1  A & C -> A     Carry = 0, N=0, PV detecta paridad,  H=1, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.c = 0x09;
    c.reg.c = 0x09;

    ejecutar(&mut z80, 0xA1, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x08, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 0, 0, 0);
}
#[test]
fn and_d() {
    // 0xA2  A & D -> A     Carry = 0, N=0, PV detecta paridad,  H=1, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.d = 0x09;
    c.reg.d = 0x09;

    ejecutar(&mut z80, 0xA2, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x08, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 0, 0, 0);
}
#[test]
fn and_e() {
    // 0xA3  A & E -> A     Carry = 0, N=0, PV detecta paridad,  H=1, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.e = 0x09;
    c.reg.e = 0x09;

    ejecutar(&mut z80, 0xA3, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x08, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 0, 0, 0);
}
#[test]
fn and_h() {
    // 0xA4  A & H -> A     Carry = 0, N=0, PV detecta paridad,  H=1, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.h = 0x09;
    c.reg.h = 0x09;

    ejecutar(&mut z80, 0xA4, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x08, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 0, 0, 0);
}
#[test]
fn and_l() {
    // 0xA5  A & L -> A     Carry = 0, N=0, PV detecta paridad,  H=1, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.l = 0x09;
    c.reg.l = 0x09;

    ejecutar(&mut z80, 0xA5, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x08, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 0, 0, 0);
}

#[test]
fn and_0hl0() {
    // 0xA6  A & (HL) -> A     Carry = 0, N=0, PV detecta paridad,  H=1, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;

    // pongo en la direccion 0xEF10 el valor 0x02
    z80.mem.mem[0xEF10] = 0x09;
    c.bus.write_byte(0xEF10, 0x09);

    // pongo HL = 0xEF10
    //z80.registros.h = 0xEF;
    //z80.registros.l = 0x10;
    set_hl_test_big(&mut z80, &mut c, 0xEF10);

    ejecutar(&mut z80, 0xA6, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x08, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 0, 0, 0);
}
#[test]
fn and_a() {
    // 0xA7  A & A -> A     Carry = 0, N=0, PV detecta paridad,  H=1, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;

    ejecutar(&mut z80, 0xA7, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x0C, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 1, 0, 0);
}

#[test]
fn xor_b() {
    // 0xA8  A ^ B -> A     Carry = 0, N=0, PV detecta paridad,  H=0, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.b = 0x09;
    c.reg.b = 0x09;

    ejecutar(&mut z80, 0xA8, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x05, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 0);
}
#[test]
fn xor_c() {
    // 0xA9  A ^ C -> A     Carry = 0, N=0, PV detecta paridad,  H=0, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.c = 0x09;
    c.reg.c = 0x09;

    ejecutar(&mut z80, 0xA9, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x05, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 0);
}
#[test]
fn xor_d() {
    // 0xAA  A ^ D -> A     Carry = 0, N=0, PV detecta paridad,  H=0, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.d = 0x09;
    c.reg.d = 0x09;

    ejecutar(&mut z80, 0xAA, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x05, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 0);
}
#[test]
fn xor_e() {
    // 0xAB  A ^ E -> A     Carry = 0, N=0, PV detecta paridad,  H=0, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.e = 0x09;
    c.reg.e = 0x09;

    ejecutar(&mut z80, 0xAB, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x05, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 0);
}
#[test]
fn xor_h() {
    // 0xAC  A ^ H -> A     Carry = 0, N=0, PV detecta paridad,  H=0, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.h = 0x09;
    c.reg.h = 0x09;

    ejecutar(&mut z80, 0xAC, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x05, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 0);
}
#[test]
fn xor_l() {
    // 0xAD  A ^ L -> A     Carry = 0, N=0, PV detecta paridad,  H=0, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.l = 0x09;
    c.reg.l = 0x09;

    ejecutar(&mut z80, 0xAD, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x05, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 0);
}

#[test]
fn xor_0hl0() {
    // 0xAE  A ^ (HL) -> A     Carry = 0, N=0, PV detecta paridad,  H=0, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    // pongo en la direccion 0xEF10 el valor 0x02
    z80.mem.mem[0xEF10] = 0x09;
    c.bus.write_byte(0xEF10, 0x09);

    // pongo HL = 0xEF10
    //z80.registros.h = 0xEF;
    //z80.registros.l = 0x10;
    set_hl_test_big(&mut z80, &mut c, 0xEF10);

    ejecutar(&mut z80, 0xAE, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x05, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 0);
}
#[test]
fn xor_a() {
    // 0xAF  A ^ A -> A     Carry = 0, N=0, PV detecta paridad,  H=0, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;

    ejecutar(&mut z80, 0xAF, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 0, 1, 0, 0);
}

#[test]
fn or_b() {
    // 0xB0  A | B -> A     Carry = 0, N=0, PV detecta paridad,  H=0, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.b = 0x09;
    c.reg.b = 0x09;

    ejecutar(&mut z80, 0xB0, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x0D, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);
}
#[test]
fn or_c() {
    // 0xB1  A | C -> A     Carry = 0, N=0, PV detecta paridad,  H=0, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.c = 0x09;
    c.reg.c = 0x09;

    ejecutar(&mut z80, 0xB1, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x0D, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);
}
#[test]
fn or_d() {
    // 0xB2  A | D -> A     Carry = 0, N=0, PV detecta paridad,  H=0, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.d = 0x09;
    c.reg.d = 0x09;

    ejecutar(&mut z80, 0xB2, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x0D, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);
}
#[test]
fn or_e() {
    // 0xB3  A | E -> A     Carry = 0, N=0, PV detecta paridad,  H=0, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.e = 0x09;
    c.reg.e = 0x09;

    ejecutar(&mut z80, 0xB3, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x0D, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);
}
#[test]
fn or_h() {
    // 0xB4  A | H -> A     Carry = 0, N=0, PV detecta paridad,  H=0, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.h = 0x09;
    c.reg.h = 0x09;

    ejecutar(&mut z80, 0xB4, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x0D, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);
}
#[test]
fn or_l() {
    // 0xB5  A | L -> A     Carry = 0, N=0, PV detecta paridad,  H=0, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.l = 0x09;
    c.reg.l = 0x09;

    ejecutar(&mut z80, 0xB5, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x0D, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);
}

#[test]
fn or_0hl0() {
    // 0xB6  A | (HL) -> A     Carry = 0, N=0, PV detecta paridad,  H=0, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    // pongo en la direccion 0xEF10 el valor 0x02
    z80.mem.mem[0xEF10] = 0x09;
    c.bus.write_byte(0xEF10, 0x09);

    // pongo HL = 0xEF10
    //z80.registros.h = 0xEF;
    //z80.registros.l = 0x10;
    set_hl_test_big(&mut z80, &mut c, 0xEF10);

    ejecutar(&mut z80, 0xB6, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x0D, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);
}
#[test]
fn or_a() {
    // 0xB7  A | A -> A     Carry = 0, N=0, PV detecta paridad,  H=0, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;

    ejecutar(&mut z80, 0xB7, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x0C, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 0);
}

#[test]
fn cp_b() {
    // 0xB8  A cp B  no modifica A solo banderas
    //     Carry = se define, N=1, PV detecta paridad,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.b = 0x09;
    c.reg.b = 0x09;

    ejecutar(&mut z80, 0xB8, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x0C, "Valor en A incorrecto"); // A no modificado

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 1, 0);
}
#[test]
fn cp_c() {
    // 0xB9  A cp C no modifica A solo banderas
    //     Carry = se define, N=1, PV detecta paridad,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.c = 0x09;
    c.reg.c = 0x09;

    ejecutar(&mut z80, 0xB9, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x0C, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 1, 0);
}
#[test]
fn cp_d() {
    // 0xBA  A cp D no modifica A solo banderas
    // Carry = se define, N=1, PV detecta paridad,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.d = 0x09;
    c.reg.d = 0x09;

    ejecutar(&mut z80, 0xBA, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x0C, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 1, 0);
}
#[test]
fn cp_e() {
    // 0xBB  A cp E no modifica A solo banderas
    // Carry = se define, N=1, PV detecta paridad,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.e = 0x09;
    c.reg.e = 0x09;

    ejecutar(&mut z80, 0xBB, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x0C, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 1, 0);
}
#[test]
fn cp_h() {
    // 0xBC  A cp H no modifica A solo banderas
    // Carry = se define, N=1, PV detecta paridad,  H se define, Z se define, S se define,,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    z80.reg.h = 0x0C;
    c.reg.a = 0x0C;
    c.reg.h = 0x0C;

    ejecutar(&mut z80, 0xBC, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x0C, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 0, 0, 1, 0);
}
#[test]
fn cp_l() {
    // 0xBD  A cp L no modifica A solo banderas
    // Carry = se define, N=1, PV detecta paridad,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    z80.reg.l = 0x09;
    c.reg.l = 0x09;

    ejecutar(&mut z80, 0xBD, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x0C, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 1, 0);
}

#[test]
fn cp_0hl0() {
    // 0xBE  A cp (HL) no modifica A solo banderas
    // Carry = se define, N=1, PV detecta paridad,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;
    // pongo en la direccion 0xEF10 el valor 0x02
    z80.mem.mem[0xEF10] = 0x09;
    c.bus.write_byte(0xEF10, 0x09);

    // pongo HL = 0xEF10
    //z80.registros.h = 0xEF;
    //z80.registros.l = 0x10;
    set_hl_test_big(&mut z80, &mut c, 0xEF10);

    ejecutar(&mut z80, 0xBE, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x0C, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 1, 0);
}
#[test]
fn cp_a() {
    // 0xBF  A cp A no modifica A solo banderas
    // Carry = se define, N=1, PV detecta paridad,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;

    ejecutar(&mut z80, 0xBF, 0, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x0C, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 0, 0, 1, 0);
}

#[test]
fn ret_nz() {
    // 0xC0 No afecta flags. Si el flag Z anterior = 0   retorna haciendo un pop en el PC
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // a) se cumple la condicion z=0 *****
    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    hace_un_call_incondicional_a_direcc_dada(&mut z80, 0xE0, 0x15, &mut c);

    // Pongo flag Z a 0
    z80.reg.set_flag(&StatusFlag::Zero, false);

    // Ejecuto ret_nz
    ejecutar_en_direccion(&mut z80, 0xE015, 0xC0, 0, 0, 0, &mut c);

    // Comprobar que el PC ha retornado a la siguiente direccion
    assert_eq!(z80.reg.pc, 0xA252);

    // b) no se cumple la condicion z=1 *****
    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    hace_un_call_incondicional_a_direcc_dada(&mut z80, 0xE0, 0x15, &mut c);

    // Pongo flag Z a 1
    z80.reg.set_flag(&StatusFlag::Zero, true);
    c.reg.flags.z = true;

    // Ejecuto ret_nz
    ejecutar_en_direccion(&mut z80, 0xE015, 0xC0, 0, 0, 0, &mut c);

    // Comprobar que el PC no retorna
    assert_eq!(z80.reg.pc, 0xE016);
}
#[test]
fn pop_bc() {
    // 0xC1
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // pongo sp en 0xFFFF
    z80.reg.sp = 0xFF18;
    c.reg.sp = 0xFF18;

    // primero debemos hacer un push
    // alto = 0xE1    bajo = 0x2A
    hacer_push_test_nn(&mut z80, &mut c, 0xE12A);

    // ponemos PC a 0
    //z80.registros.set_pc(0);

    // Ejecutamos POP en BC
    ejecutar(&mut z80, 0xC1, 0, 0, 0, &mut c);

    //assert_eq!(z80.reg.b, 0xE1);
    //assert_eq!(z80.reg.c, 0x2A);
    assert_eq!(get_bc_test_big(&mut z80, &mut c), 0xE12A);
}

#[test]
fn jp_nz_nn() {
    // 0xC2 No afecta a los flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // ponemos flag z = 1
    z80.reg.set_flag(&StatusFlag::Zero, true);
    c.reg.flags.z = true;
    ejecutar(&mut z80, 0xC2, 0x34, 0xA6, 0, &mut c);

    assert_eq!(z80.reg.pc, 0x0003);

    // ponemos flag z = 0

    z80.reg.set_flag(&StatusFlag::Zero, false);
    c.reg.flags.z = false;

    // reseteamos pc
    z80.reg.pc = 0;
    c.reg.pc = 0;
    ejecutar(&mut z80, 0xC2, 0x34, 0xA6, 0, &mut c);

    assert_eq!(z80.reg.pc, 0xA634);
}
#[test]
fn jp_nn() {
    // 0xC3 No afecta a los flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    ejecutar(&mut z80, 0xC3, 0x34, 0xA6, 0, &mut c);

    assert_eq!(z80.reg.pc, 0xA634);
}
#[test]
fn call_nz_nn() {
    // 0xC4 n1 n2  no afecta flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Establecemos SP como 0xFF18
    z80.reg.sp = 0xFF18;
    c.reg.sp = 0xFF18;

    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    z80.reg.pc = 0xA24F;
    c.reg.pc = 0xA24F;

    // ponemos z=0
    z80.reg.set_flag(&StatusFlag::Zero, false);
    c.reg.flags.z = false;

    // primero se almacena el mas significativo en el stack
    ejecutar_en_direccion(&mut z80, 0xA24F, 0xC4, 0x15, 0xE0, 0, &mut c);

    // comprobar que SP ha cambiado
    assert_eq!(z80.reg.sp, 0xFF16);

    // comprobar los bytes del stack
    // 0xFFFD = 0x52  (ha sumado 3 a 4F) para que cuando haya un ret vuelva
    // a la direccion posterior al call que siempre es de 3 bytes
    assert_eq!(z80.mem.mem[0xFF16], 0x52);
    assert_eq!(z80.mem.mem[0xFF17], 0xA2);

    // Comprueba el nuevo valor del pc
    assert_eq!(z80.reg.pc, 0xE015);
}

#[test]
fn push_bc() {
    // 0xC5 No afecta a los flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // pongo sp en 0xFFFF
    //z80.registros.sp = 0xFF18;
    //c.reg.sp = 0xFF18;
    set_sp_test_big(&mut z80, &mut c, 0xFF18);

    //z80.registros.b = 0xE1;
    //z80.registros.c = 0x2A;
    set_bc_test_big(&mut z80, &mut c, 0xE12A);

    ejecutar(&mut z80, 0xC5, 0, 0, 0, &mut c);

    // Comprobar SP
    assert_eq!(z80.reg.sp, 0xFF16);
    // Comprobar los datos en la memoria que apunta el SP
    assert_eq!(z80.mem.mem[0xFF16], 0x2A);
    assert_eq!(z80.mem.mem[0xFF17], 0xE1);
}
#[test]
fn add_a_n() {
    // 0xC6 A + N -> A     Carry se define, N=0, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Caso 1: Sin resultado con acarreo ------------------------------
    z80.reg.a = 0xCD;
    c.reg.a = 0xCD;

    // n = 0x02
    ejecutar(&mut z80, 0xC6, 0x02, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCF); // compruebo que A es 0xCF

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // Caso 2: Con resultado con acarreo ------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.a = 0xFF;
    c.reg.a = 0xFF;

    // n = 0x01
    ejecutar(&mut z80, 0xC6, 0x01, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);
}
#[test]
fn rst_00_h() {
    // 0xC7 No afecta flags, se hace push del PC, PC = 0x00
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Establecemos SP como 0xFFFF
    //z80.registros.sp = 0xFF18;
    set_sp_test_big(&mut z80, &mut c, 0xFF18);

    // Establecemos PC como 0xA24F (Hacemos el rst 00h desde 0xA24F
    z80.reg.pc = 0xA24F;
    c.reg.pc = 0xA24F;

    // primero se almacena el mas significativo en el stack
    ejecutar_en_direccion(&mut z80, 0xA24F, 0xC7, 0, 0, 0, &mut c);

    // Comprobar que el PC=0
    assert_eq!(z80.reg.pc, 0);

    // Comprobar SP
    assert_eq!(z80.reg.sp, 0xFF16);
    // Comprobar los datos en la memoria que apunta el SP
    assert_eq!(z80.mem.mem[0xFF16], 0x50);
    assert_eq!(z80.mem.mem[0xFF17], 0xA2);
}

#[test]
fn ret_z() {
    // 0xC8 No afecta flags. Si el flag Z anterior = 1   retorna haciendo un pop en el PC
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // a) se cumple la condicion z=1 *****
    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    hace_un_call_incondicional_a_direcc_dada(&mut z80, 0xE0, 0x15, &mut c);

    // Pongo flag Z a 1
    z80.reg.set_flag(&StatusFlag::Zero, true);
    c.reg.flags.z = true;

    // Ejecuto ret_z
    ejecutar_en_direccion(&mut z80, 0xE015, 0xC8, 0, 0, 0, &mut c);

    // Comprobar que el PC ha retornado a la siguiente direccion
    assert_eq!(z80.reg.pc, 0xA252);

    // b) no se cumple la condicion z=0 *****
    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    hace_un_call_incondicional_a_direcc_dada(&mut z80, 0xE0, 0x15, &mut c);

    // Pongo flag Z a 0
    z80.reg.set_flag(&StatusFlag::Zero, false);
    c.reg.flags.z = false;
    // Ejecuto ret_nz
    ejecutar_en_direccion(&mut z80, 0xE015, 0xC8, 0, 0, 0, &mut c);

    // Comprobar que el PC no retorna
    assert_eq!(z80.reg.pc, 0xE016);
}

#[test]
fn ret() {
    // 0xC9 No afecta flags. Si el flag Z anterior = 1   retorna haciendo un pop en el PC
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    hace_un_call_incondicional_a_direcc_dada(&mut z80, 0xE0, 0x15, &mut c);

    // Ejecuto ret_z
    ejecutar_en_direccion(&mut z80, 0xE015, 0xC9, 0, 0, 0, &mut c);

    // Comprobar que el PC ha retornado a la siguiente direccion
    assert_eq!(z80.reg.pc, 0xA252);
}
#[test]
fn jp_z_nn() {
    // 0xCA No afecta a los flags PENDIENTE *********************
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // ponemos flag z = 0
    z80.reg.set_flag(&StatusFlag::Zero, false);
    c.reg.flags.z = false;
    ejecutar(&mut z80, 0xCA, 0x34, 0xA6, 0, &mut c);

    assert_eq!(z80.reg.pc, 0x0003);

    // ponemos flag z = 1

    z80.reg.set_flag(&StatusFlag::Zero, true);
    c.reg.flags.z = true;

    // reseteamos pc
    z80.reg.set_pc(0);
    c.reg.pc = 0;
    ejecutar(&mut z80, 0xCA, 0x34, 0xA6, 0, &mut c);

    assert_eq!(z80.reg.pc, 0xA634);
}
// Probamos 0xCB (bits) en fichero test_cb.rs -------------------------------------------------------

#[test]
fn call_z_nn() {
    // 0xCC n1 n2  no afecta flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Establecemos SP como 0xFFFF
    //z80.registros.sp = 0xFF18;
    set_sp_test_big(&mut z80, &mut c, 0xFF18);

    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    z80.reg.pc = 0xA24F;
    c.reg.pc = 0xA24F;

    // ponemos z=1
    z80.reg.set_flag(&StatusFlag::Zero, true);
    c.reg.flags.z = true;

    // primero se almacena el mas significativo en el stack
    ejecutar_en_direccion(&mut z80, 0xA24F, 0xCC, 0x15, 0xE0, 0, &mut c);

    // comprobar que SP ha cambiado
    assert_eq!(z80.reg.sp, 0xFF16);

    // comprobar los bytes del stack

    // 0xFFFD = 0x52  (ha sumado 3 a 4F) para que cuando haya un ret vuelva
    // a la direccion posterior al call que siempre es de 3 bytes
    assert_eq!(z80.mem.mem[0xFF16], 0x52);
    assert_eq!(z80.mem.mem[0xFF17], 0xA2);

    // Comprueba el nuevo valor del pc
    assert_eq!(z80.reg.pc, 0xE015);
}
#[test]
fn call_nn() {
    // 0xCD n1 n2  no afecta flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Establecemos SP como 0xFFFF
    //z80.registros.sp = 0xFF18;
    set_sp_test_big(&mut z80, &mut c, 0xFF18);

    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    z80.reg.pc = 0xA24F;
    c.reg.pc = 0xA24F;

    // primero se almacena el mas significativo en el stack
    ejecutar_en_direccion(&mut z80, 0xA24F, 0xCD, 0x15, 0xE0, 0, &mut c);

    // comprobar que SP ha cambiado
    assert_eq!(z80.reg.sp, 0xFF16);

    // comprobar los bytes del stack

    // 0xFF16 = 0x52  (ha sumado 3 a 4F) para que cuando haya un ret vuelva
    // a la direccion posterior al call que siempre es de 3 bytes
    assert_eq!(z80.mem.mem[0xFF16], 0x52);
    assert_eq!(z80.mem.mem[0xFF17], 0xA2);

    // Comprueba el nuevo valor del pc
    assert_eq!(z80.reg.pc, 0xE015);
}

#[test]
fn adc_a_n() {
    // 0xCE  A + N +Carry -> A     Carry se define, N=0, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Caso 1a: Sin resultado con acarreo y acarreo anterior------------------------------
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0xCD;
    c.reg.a = 0xCD;
    ejecutar(&mut z80, 0xCE, 0x02, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xD0); // compruebo que A es

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 0, 0);

    // Caso 1b: Sin resultado con acarreo y sin acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xCD;
    c.reg.a = 0xCD;

    ejecutar(&mut z80, 0xCE, 0x02, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCF); // compruebo que A es

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // Caso 2a: Con resultado con acarreo y acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0xFE;
    c.reg.a = 0xFE;

    ejecutar(&mut z80, 0xCE, 0x01, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);

    // Caso 2b: Con resultado con acarreo y sin acarreo anterior------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xFF;
    c.reg.a = 0xFF;

    ejecutar(&mut z80, 0xCE, 0x01, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x00); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);
}

#[test]
fn rst_08_h() {
    // 0xCF No afecta flags, se hace push del PC, PC = 0x08
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Establecemos SP como 0xFFFF
    // z80.registros.sp = 0xFFFF;
    set_sp_test_big(&mut z80, &mut c, 0xFFFF);

    // Establecemos PC como 0xA24F (Hacemos el rst 00h desde 0xA24F
    z80.reg.pc = 0xA24F;
    c.reg.pc = 0xA24F;

    // primero se almacena el mas significativo en el stack
    ejecutar_en_direccion(&mut z80, 0xA24F, 0xCF, 0, 0, 0, &mut c);

    // Comprobar que el PC=8
    assert_eq!(z80.reg.pc, 0x0008);

    // Comprobar SP

    assert_eq!(z80.reg.sp, 0xFFFD);
    // Comprobar los datos en la memoria que apunta el SP
    assert_eq!(z80.mem.mem[0xFFFD], 0x50);
    assert_eq!(z80.mem.mem[0xFFFE], 0xA2);
}

#[test]
fn ret_nc() {
    // 0xD0 No afecta flags. Si el flag C anterior = 0   retorna haciendo un pop en el PC
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // a) se cumple la condicion c=0 *****
    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    hace_un_call_incondicional_a_direcc_dada(&mut z80, 0xE0, 0x15, &mut c);

    // Pongo flag C a 0
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;

    // Ejecuto ret_nc
    ejecutar_en_direccion(&mut z80, 0xE015, 0xD0, 0, 0, 0, &mut c);

    // Comprobar que el PC ha retornado a la siguiente direccion
    assert_eq!(z80.reg.pc, 0xA252);

    // b) no se cumple la condicion c=1 *****
    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    hace_un_call_incondicional_a_direcc_dada(&mut z80, 0xE0, 0x15, &mut c);

    // Pongo flag C a 1
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    // Ejecuto ret_nz
    ejecutar_en_direccion(&mut z80, 0xE015, 0xD0, 0, 0, 0, &mut c);

    // Comprobar que el PC no retorna
    assert_eq!(z80.reg.pc, 0xE016);
}
#[test]
fn pop_de() {
    // 0xD1
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // pongo sp en 0xFFFF
    //z80.registros.sp = 0xFF18;
    set_sp_test_big(&mut z80, &mut c, 0xFF18);

    // primero debemos hacer un push
    // alto = 0xE1    bajo = 0x2A
    hacer_push_test_nn(&mut z80, &mut c, 0xE12A);

    // ponemos PC a 0
    z80.reg.set_pc(0);
    c.reg.pc = 0;

    // Ejecutamos POP en BC
    ejecutar(&mut z80, 0xD1, 0, 0, 0, &mut c);

    //assert_eq!(z80.reg.d, 0xE1);
    //assert_eq!(z80.reg.e, 0x2A);
    assert_eq!(get_de_test_big(&mut z80, &mut c), 0xE12A);
}

#[test]
fn jp_nc_nn() {
    // 0xD2 No afecta a los flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // ponemos flag c = 1
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;

    ejecutar(&mut z80, 0xD2, 0x34, 0xA6, 0, &mut c);

    assert_eq!(z80.reg.pc, 0x0003);

    // ponemos flag c = 0

    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;

    // reseteamos pc
    z80.reg.set_pc(0);
    ejecutar(&mut z80, 0xD2, 0x34, 0xA6, 0, &mut c);

    assert_eq!(z80.reg.pc, 0xA634);
}
#[test]
fn out_0n0_a() {
    // 0xD3 out(n), a No afecta a los flags. El valor de A es escrito al puerto cuya
    // direccion esta formada por A en los bits altos y n en los bits bajos
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    let out = z80::io::BufOutput::default();

    // Instalo un device de salida en 0xFE
    z80.install_output(0xFE, Box::new(out.clone()));
    // Ahora se podra usar `OUT (0xFE), <registro>`.

    // Ponemos en A 0xA1
    z80.reg.a = 0xA1;
    c.reg.a = 0xA1;

    //ejecutar(&mut z80, 0xD3, 0xFE, 0, 0, &mut c);
    ejecutar(&mut z80, 0xD3, 0xFE, 0, 0, &mut c);
    for (clave, valor) in z80.output_devices {
        println!("clave->{:?}  valor->{:?}", clave, valor);
    }
}
#[test]
fn call_nc_nn() {
    // 0xD4 n1 n2  no afecta flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Establecemos SP como 0xFF18
    //z80.registros.sp = 0xFF18;
    set_sp_test_big(&mut z80, &mut c, 0xFF18);

    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    z80.reg.pc = 0xA24F;
    c.reg.pc = 0xA24F;

    // ponemos c=0
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;

    // primero se almacena el mas significativo en el stack
    ejecutar_en_direccion(&mut z80, 0xA24F, 0xD4, 0x15, 0xE0, 0, &mut c);

    // comprobar que SP ha cambiado
    assert_eq!(z80.reg.sp, 0xFF16);

    // comprobar los bytes del stack

    // 0xFF16 = 0x52  (ha sumado 3 a 4F) para que cuando haya un ret vuelva
    // a la direccion posterior al call que siempre es de 3 bytes
    assert_eq!(z80.mem.mem[0xFF16], 0x52);
    assert_eq!(z80.mem.mem[0xFF17], 0xA2);

    // Comprueba el nuevo valor del pc
    assert_eq!(z80.reg.pc, 0xE015);
}

#[test]
fn push_de() {
    // 0xD5 No afecta a los flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // pongo sp en 0xFF18
    //z80.registros.sp = 0xFF18;
    set_sp_test_big(&mut z80, &mut c, 0xFF18);

    // z80.registros.d = 0xE1;
    // z80.registros.e = 0x2A;
    set_de_test_big(&mut z80, &mut c, 0xE12A);

    ejecutar(&mut z80, 0xD5, 0, 0, 0, &mut c);

    // Comprobar SP
    assert_eq!(z80.reg.sp, 0xFF16);
    // Comprobar los datos en la memoria que apunta el SP

    assert_eq!(z80.mem.mem[0xFF16], 0x2A);
    assert_eq!(z80.mem.mem[0xFF17], 0xE1);
}
#[test]
fn sub_n() {
    // 0xD6 A - N -> A     Carry se define, N=1, PV detecta overflow,  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Caso 1: Sin resultado con acarreo ------------------------------
    z80.reg.a = 0xCD;
    c.reg.a = 0xCD;

    // n = 0x02
    ejecutar(&mut z80, 0xD6, 0x02, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCB); // compruebo que A es 0xCF

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 1, 0);

    // Caso 2: Con resultado con acarreo ------------------------------
    z80.reg.pc = 0; // reseteo pc
    c.reg.pc = 0;
    z80.reg.a = 0x00;
    c.reg.a = 0x00;

    // n = 0x01
    ejecutar(&mut z80, 0xD6, 0x01, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xFF); // compruebo que A es 0x00

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);
}
#[test]
fn rst_10_h() {
    // 0xD7 No afecta flags, se hace push del PC, PC = 0x00
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Establecemos SP como 0xFF18
    //z80.registros.sp = 0xFF18;
    set_sp_test_big(&mut z80, &mut c, 0xFF18);

    // Establecemos PC como 0xA24F (Hacemos el rst 00h desde 0xA24F
    z80.reg.pc = 0xA24F;
    c.reg.pc = 0xA24F;

    // primero se almacena el mas significativo en el stack
    ejecutar_en_direccion(&mut z80, 0xA24F, 0xD7, 0, 0, 0, &mut c);

    // Comprobar que el PC=0x10
    assert_eq!(z80.reg.pc, 0x10);

    // Comprobar SP

    assert_eq!(z80.reg.sp, 0xFF16);
    // Comprobar los datos en la memoria que apunta el SP
    assert_eq!(z80.mem.mem[0xFF16], 0x50);
    assert_eq!(z80.mem.mem[0xFF17], 0xA2);
}

#[test]
fn ret_c() {
    // 0xD8 No afecta flags. Si el flag Z anterior = 1   retorna haciendo un pop en el PC
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // a) se cumple la condicion c=1 *****
    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    hace_un_call_incondicional_a_direcc_dada(&mut z80, 0xE0, 0x15, &mut c);

    // Pongo flag C a 1
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;

    // Ejecuto ret_z
    ejecutar_en_direccion(&mut z80, 0xE015, 0xD8, 0, 0, 0, &mut c);

    // Comprobar que el PC ha retornado a la siguiente direccion
    assert_eq!(z80.reg.pc, 0xA252);

    // b) no se cumple la condicion c=0 *****
    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    hace_un_call_incondicional_a_direcc_dada(&mut z80, 0xE0, 0x15, &mut c);

    // Pongo flag C a 0
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;

    // Ejecuto ret_nz
    ejecutar_en_direccion(&mut z80, 0xE015, 0xD8, 0, 0, 0, &mut c);

    // Comprobar que el PC no retorna
    assert_eq!(z80.reg.pc, 0xE016);
}

#[test]
fn exx() {
    // 0xD9 No afecta flags. Intercambia BC DE HL con BC' DE' HL'
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    z80.debug.debug_zilog = false;

    //z80.reg.b = 0x47;
    //z80.reg.c = 0x1A;
    set_bc_test_big(&mut z80, &mut c, 0x471A);
    //z80.reg.d = 0x2B;
    //z80.reg.e = 0x3C;
    set_de_test_big(&mut z80, &mut c, 0x2B3C);
    //z80.reg.h = 0x4D;
    //z80.reg.l = 0x5E;
    set_hl_test_big(&mut z80, &mut c, 0x4D5E);

    z80.reg.bp = 0x6F;
    z80.reg.cp = 0x70;
    z80.reg.dp = 0x81;
    z80.reg.ep = 0x92;
    z80.reg.hp = 0xA3;
    z80.reg.lp = 0xB4;

    ejecutar(&mut z80, 0xD9, 0, 0, 0, &mut c);
    // Comprobar registros normales
    //assert_eq!(z80.reg.b, 0x6F);
    //assert_eq!(z80.reg.c, 0x70);
    assert_eq!(get_bc_test_big(&mut z80, &mut c), 0x6F70);
    //assert_eq!(z80.reg.d, 0x81);
    //assert_eq!(z80.reg.e, 0x92);
    assert_eq!(get_de_test_big(&mut z80, &mut c), 0x8192);
    //assert_eq!(z80.reg.h, 0xA3);
    //assert_eq!(z80.reg.l, 0xB4);
    assert_eq!(get_hl_test_big(&mut z80, &mut c), 0xA3B4);

    // Comprobar registros prima
    assert_eq!(z80.reg.bp, 0x47);
    assert_eq!(z80.reg.cp, 0x1A);
    assert_eq!(z80.reg.dp, 0x2B);
    assert_eq!(z80.reg.ep, 0x3C);
    assert_eq!(z80.reg.hp, 0x4D);
    assert_eq!(z80.reg.lp, 0x5E);
}
#[test]
fn jp_c_nn() {
    // 0xDA No afecta a los flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // ponemos flag c = 0
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;

    ejecutar(&mut z80, 0xDA, 0x34, 0xA6, 0, &mut c);

    assert_eq!(z80.reg.pc, 0x0003);

    // ponemos flag c = 1

    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;

    // reseteamos pc
    z80.reg.set_pc(0);
    ejecutar(&mut z80, 0xDA, 0x34, 0xA6, 0, &mut c);

    assert_eq!(z80.reg.pc, 0xA634);
}
//
#[test]
fn in_a_0n0() {
    // 0xDB n No afecta a los flags. El valor del puerto cuya direccion esta formada
    // por A en los bits altos y n en los bits bajos es escrito en A
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    z80.debug.debug_zilog = false;

    z80.reg.a = 0xA1;

    let inp = z80::io::BufInput::new(vec!(0xC3, 0xA2, 0xB8));

    // Instalo un device de entrada en 0xFE
    z80.install_input(0xFE, Box::new(inp.clone()));

    ejecutar(&mut z80, 0xDB, 0xFE, 0, 0, &mut c);

    assert_eq!((inp.input.borrow_mut()[0]), 0xC3);
    assert_eq!((inp.input.borrow_mut()[1]), 0xA2);

    // Comprobamos A que se ha sacado del BufInput
    assert_eq!(z80.reg.a, 0xB8);
}

#[test]
fn call_c_nn() {
    // 0xDC n1 n2  no afecta flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Establecemos SP como 0xFFFF
    //z80.registros.sp = 0xFFFF;
    set_sp_test_big(&mut z80, &mut c, 0xFFFF);

    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    z80.reg.pc = 0xA24F;
    c.reg.pc = 0xA2AF;

    // ponemos c=1
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;

    // primero se almacena el mas significativo en el stack
    ejecutar_en_direccion(&mut z80, 0xA24F, 0xDC, 0x15, 0xE0, 0, &mut c);

    // comprobar que SP ha cambiado
    assert_eq!(z80.reg.sp, 0xFFFD);

    // comprobar los bytes del stack
    assert_eq!(z80.mem.mem[0xFFFE], 0xA2);
    // 0xFFFD = 0x52  (ha sumado 3 a 4F) para que cuando haya un ret vuelva
    // a la direccion posterior al call que siempre es de 3 bytes
    assert_eq!(z80.mem.mem[0xFFFD], 0x52);

    // Comprueba el nuevo valor del pc
    assert_eq!(z80.reg.pc, 0xE015);
}
// 0xDD Instrucciones IX

#[test]
fn sbc_a_n() {
    // 0xDE  A - N - Carry -> A     Carry se define, N=1, PV detecta overflow,
    //  H se define, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Caso 1: Resta sin acarreo ni overflow resultante y acarreo anterior = 0
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0xCD;
    c.reg.a = 0xCD;

    ejecutar(&mut z80, 0xDE, 0x02, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xCB, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 1, 0);

    // Caso 2: Resta con acarreo (borrow) en nibble bajo y total y acarreo anterior = 1
    z80.reg.pc = 0;  // Resetea contador de programa
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0x01;
    c.reg.a = 0x01;

    ejecutar(&mut z80, 0xDE, 0x02, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xFE, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);

    // Caso 3: Overflow en operación con signo y acarreo anterior = 1
    z80.reg.pc = 0;
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0x80;  // -128 (complemento a 2)
    c.reg.a = 0x80;

    ejecutar(&mut z80, 0xDE, 0x01, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0x7E, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 1, 1, 0);
}

#[test]
fn rst_18_h() {
    // 0xDF No afecta flags, se hace push del PC, PC = 0x08
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Establecemos SP como 0xFFFF
    //z80.registros.sp = 0xFFFF;
    set_sp_test_big(&mut z80, &mut c, 0xFFFF);

    // Establecemos PC como 0xA24F (Hacemos el rst 00h desde 0xA24F
    z80.reg.pc = 0xA24F;
    c.reg.pc = 0xA2AF;

    // primero se almacena el mas significativo en el stack
    ejecutar_en_direccion(&mut z80, 0xA24F, 0xDF, 0, 0, 0, &mut c);

    // Comprobar que el PC=0x18
    assert_eq!(z80.reg.pc, 0x0018);

    // Comprobar SP

    assert_eq!(z80.reg.sp, 0xFFFD);
    // Comprobar los datos en la memoria que apunta el SP
    assert_eq!(z80.mem.mem[0xFFFD], 0x50);
    assert_eq!(z80.mem.mem[0xFFFE], 0xA2);
}

#[test]
fn ret_po() {
    // 0xE0 No afecta flags. Si el flag PV anterior = 0   retorna haciendo un pop en el PC
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // a) se cumple la condicion pv=0 *****
    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    hace_un_call_incondicional_a_direcc_dada(&mut z80, 0xE0, 0x15, &mut c);

    // Pongo flag PV a 0
    z80.reg.set_flag(&StatusFlag::ParityOverflow, false);
    c.reg.flags.p = false;

    // Ejecuto ret_nc
    ejecutar_en_direccion(&mut z80, 0xE015, 0xE0, 0, 0, 0, &mut c);

    // Comprobar que el PC ha retornado a la siguiente direccion
    assert_eq!(z80.reg.pc, 0xA252);

    // b) no se cumple la condicion pv=1 *****
    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    hace_un_call_incondicional_a_direcc_dada(&mut z80, 0xE0, 0x15, &mut c);

    // Pongo flag PV a 1
    z80.reg.set_flag(&StatusFlag::ParityOverflow, true);
    c.reg.flags.p = true;
    // Ejecuto ret_nz
    ejecutar_en_direccion(&mut z80, 0xE015, 0xE0, 0, 0, 0, &mut c);

    // Comprobar que el PC no retorna
    assert_eq!(z80.reg.pc, 0xE016);
}
#[test]
fn pop_hl() {
    // 0xE1
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // pongo sp en 0xFF18
    //z80.registros.sp = 0xFF18;
    set_sp_test_big(&mut z80, &mut c, 0xFF18);

    // primero debemos hacer un push
    // alto = 0xE1    bajo = 0x2A
    hacer_push_test_nn(&mut z80, &mut c, 0xE12A);

    // ponemos PC a 0
    z80.reg.pc = 0;
    c.reg.pc = 0;

    // Ejecutamos POP en HL
    ejecutar(&mut z80, 0xE1, 0, 0, 0, &mut c);

    //assert_eq!(z80.reg.h, 0xE1);
    //assert_eq!(z80.reg.l, 0x2A);
    assert_eq!(get_hl_test_big(&mut z80, &mut c), 0xE12A);
}

#[test]
fn jp_po_nn() {
    // 0xE2 No afecta a los flags  Si el flag PV anterior = 0   salta a nn
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // ponemos flag pv = 1
    z80.reg.set_flag(&StatusFlag::ParityOverflow, true);
    c.reg.flags.p = true;
    ejecutar(&mut z80, 0xE2, 0x34, 0xA6, 0, &mut c);

    assert_eq!(z80.reg.pc, 0x0003);

    // ponemos flag pv = 0

    z80.reg.set_flag(&StatusFlag::ParityOverflow, false);
    c.reg.flags.p = false;

    // reseteamos pc
    z80.reg.pc = 0;
    c.reg.pc = 0;

    ejecutar(&mut z80, 0xE2, 0x34, 0xA6, 0, &mut c);

    assert_eq!(z80.reg.pc, 0xA634);
}
#[test]
fn ex_0sp0_hl() {
    // 0xE3 No afecta a los flags, Intercambia (SP) con HL
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    // primero debemos hacer un push
    z80.reg.sp = 0xFF18;
    set_sp_test_big(&mut z80, &mut c, 0xFF18);

    // alto = 0xE1    bajo = 0x2A
    hacer_push_test_nn(&mut z80, &mut c, 0xE12A);

    // reseteamos PC
    z80.reg.pc = 0;
    c.reg.pc = 0;
    // ponemos un valor a hl
    //z80.registros.h = 0x3F;
    //z80.registros.l = 0x52;
    set_hl_test_big(&mut z80, &mut c, 0x3F52);
    ejecutar(&mut z80, 0xE3, 0, 0, 0, &mut c);

    // Comprobar valores en stack
    let sp = z80.reg.sp as usize;

    assert_eq!(z80.mem.mem[sp], 0x52);
    assert_eq!(z80.mem.mem[sp + 1], 0x3F);

    // Comprobar valores en hl
    //assert_eq!(z80.reg.h, 0xE1);
    //assert_eq!(z80.reg.l, 0x2A);
    assert_eq!(get_hl_test_big(&mut z80, &mut c), 0xE12A);
}
#[test]
fn call_po_nn() {
    // 0xE4 n1 n2  no afecta flags  Si el flag PV anterior = 0   hace call a nn
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Establecemos SP como 0xFF18
    //z80.registros.sp = 0xFF18;
    set_sp_test_big(&mut z80, &mut c, 0xFF18);

    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    z80.reg.pc = 0xA24F;
    c.reg.pc = 0;

    // ponemos c=0
    z80.reg.set_flag(&StatusFlag::ParityOverflow, false);
    c.reg.flags.c = false;

    // primero se almacena el mas significativo en el stack
    ejecutar_en_direccion(&mut z80, 0xA24F, 0xE4, 0x15, 0xE0, 0, &mut c);

    // comprobar que SP ha cambiado
    assert_eq!(z80.reg.sp, 0xFF16);

    // comprobar los bytes del stack

    // 0xFF16 = 0x52  (ha sumado 3 a 4F) para que cuando haya un ret vuelva
    // a la direccion posterior al call que siempre es de 3 bytes
    assert_eq!(z80.mem.mem[0xFF16], 0x52);
    assert_eq!(z80.mem.mem[0xFF17], 0xA2);

    // Comprueba el nuevo valor del pc
    assert_eq!(z80.reg.pc, 0xE015);
}

#[test]
fn push_hl() {
    // 0xE5 No afecta a los flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // pongo sp en 0xFF18
    //z80.registros.sp = 0xFF18;
    set_sp_test_big(&mut z80, &mut c, 0xFF18);

    hacer_push_test_nn(&mut z80, &mut c, 0xE12A);
}

#[test]
fn and_n() {
    // 0xE6 A & N -> A     Carry =0, N=0, PV detecta parity,  H=1, Z se define, S se define,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;

    ejecutar(&mut z80, 0xE6, 0x09, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x08, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 0, 0, 0);
}
#[test]
fn rst_20_h() {
    // 0xE7 No afecta flags, se hace push del PC, PC = 0x00
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Establecemos SP como 0xFF18
    //z80.registros.sp = 0xFF18;
    set_sp_test_big(&mut z80, &mut c, 0xFF18);

    // Establecemos PC como 0xA24F (Hacemos el rst 00h desde 0xA24F
    z80.reg.pc = 0xA24F;
    c.reg.pc = 0xA24F;

    // primero se almacena el mas significativo en el stack
    ejecutar_en_direccion(&mut z80, 0xA24F, 0xE7, 0, 0, 0, &mut c);

    // Comprobar que el PC=0x10
    assert_eq!(z80.reg.pc, 0x20);

    // Comprobar SP

    assert_eq!(z80.reg.sp, 0xFF16);
    // Comprobar los datos en la memoria que apunta el SP
    assert_eq!(z80.mem.mem[0xFF16], 0x50);
    assert_eq!(z80.mem.mem[0xFF17], 0xA2);
}

#[test]
fn ret_pe() {
    // 0xE8 No afecta flags. Si el flag PV anterior = 1   retorna haciendo un pop en el PC
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // a) se cumple la condicion pv=1 *****
    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    hace_un_call_incondicional_a_direcc_dada(&mut z80, 0xE0, 0x15, &mut c);

    // Pongo flag PV a 1
    z80.reg.set_flag(&StatusFlag::ParityOverflow, true);
    c.reg.flags.p = true;

    // Ejecuto ret_z
    ejecutar_en_direccion(&mut z80, 0xE015, 0xE8, 0, 0, 0, &mut c);

    // Comprobar que el PC ha retornado a la siguiente direccion
    assert_eq!(z80.reg.pc, 0xA252);

    // b) no se cumple la condicion PV=0 *****
    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    hace_un_call_incondicional_a_direcc_dada(&mut z80, 0xE0, 0x15, &mut c);

    // Pongo flag PV a 0
    z80.reg.set_flag(&StatusFlag::ParityOverflow, false);
    c.reg.flags.p = false;

    // Ejecuto ret_nz
    ejecutar_en_direccion(&mut z80, 0xE015, 0xE8, 0, 0, 0, &mut c);

    // Comprobar que el PC no retorna
    assert_eq!(z80.reg.pc, 0xE016);
}

#[test]
fn jp_0hl0() {
    // 0xE9 No afecta a los flags salta a la direccion  que diga hl
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // ponemos un valor a hl
    //z80.registros.h = 0x3F;
    //z80.registros.l = 0x52;
    set_hl_test_big(&mut z80, &mut c, 0x3F52);

    ejecutar(&mut z80, 0xE9, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.pc, 0x3F52);
}
#[test]
fn jp_pe_nn() {
    // 0xEA No afecta a los flags Si pv=1 salta a nn
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // ponemos flag pv = 0
    z80.reg.set_flag(&StatusFlag::ParityOverflow, false);
    c.reg.flags.p = false;

    ejecutar(&mut z80, 0xEA, 0x34, 0xA6, 0, &mut c);

    assert_eq!(z80.reg.pc, 0x0003);

    // ponemos flag pv = 1

    z80.reg.set_flag(&StatusFlag::ParityOverflow, true);
    c.reg.flags.p = true;

    // reseteamos pc
    z80.reg.pc = 0;
    c.reg.pc = 0;

    ejecutar(&mut z80, 0xEA, 0x34, 0xA6, 0, &mut c);

    assert_eq!(z80.reg.pc, 0xA634);
}

#[test]
fn ex_de_hl() {
    // 0xEB no afecta a los flags intercambia registros de y hl
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // ponemos un valor a hl
    //z80.registros.h = 0x3F;
    //z80.registros.l = 0x52;
    set_hl_test_big(&mut z80, &mut c, 0x3F52);

    // ponemos un valor a de
    // z80.registros.d = 0x4A;
    // z80.registros.e = 0x5B;
    set_de_test_big(&mut z80, &mut c, 0x4A5B);
    ejecutar(&mut z80, 0xEB, 0, 0, 0, &mut c);

    // Comprobar valores en hl
    //assert_eq!(z80.reg.h, 0x4A);
    //assert_eq!(z80.reg.l, 0x5B);
    assert_eq!(get_hl_test_big(&mut z80, &mut c), 0x4A5B);
    // Comprobar valores en hl
    //assert_eq!(z80.reg.d, 0x3F);
    //assert_eq!(z80.reg.e, 0x52);
    assert_eq!(get_de_test_big(&mut z80, &mut c), 0x3F52);
}

#[test]
fn call_pe_nn() {
    // 0xEC n1 n2  no afecta flags, si pv=1 hace call
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Establecemos SP como 0xFFFF
    //z80.registros.sp = 0xFFFF;
    set_sp_test_big(&mut z80, &mut c, 0xFFFF);

    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    z80.reg.pc = 0xA24F;
    c.reg.pc = 0xA24F;

    // ponemos p=1
    z80.reg.set_flag(&StatusFlag::ParityOverflow, true);
    c.reg.flags.p = true;

    // primero se almacena el mas significativo en el stack
    ejecutar_en_direccion(&mut z80, 0xA24F, 0xEC, 0x15, 0xE0, 0, &mut c);

    // comprobar que SP ha cambiado
    assert_eq!(z80.reg.sp, 0xFFFD);

    // comprobar los bytes del stack
    assert_eq!(z80.mem.mem[0xFFFE], 0xA2);
    // 0xFFFD = 0x52  (ha sumado 3 a 4F) para que cuando haya un ret vuelva
    // a la direccion posterior al call que siempre es de 3 bytes
    assert_eq!(z80.mem.mem[0xFFFD], 0x52);

    // Comprueba el nuevo valor del pc
    assert_eq!(z80.reg.pc, 0xE015);
}
// 0xED Instrucciones Miscelaneas

#[test]
fn xor_n() {
    // 0xEE Carry=0, N=0, PV detecta paridad, h=0, Z, S se definen
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;

    ejecutar(&mut z80, 0xEE, 0x09, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x05, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 0);
}

#[test]
fn rst_28_h() {
    // 0xEF No afecta flags, se hace push del PC, PC = 0x08
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Establecemos SP como 0xFFFF
    //z80.registros.sp = 0xFFFF;
    set_sp_test_big(&mut z80, &mut c, 0xFFFF);

    // Establecemos PC como 0xA24F (Hacemos el rst 00h desde 0xA24F
    z80.reg.pc = 0xA24F;
    c.reg.pc = 0xA24F;

    // primero se almacena el mas significativo en el stack
    ejecutar_en_direccion(&mut z80, 0xA24F, 0xEF, 0, 0, 0, &mut c);

    // Comprobar que el PC=0x28
    assert_eq!(z80.reg.pc, 0x0028);

    // Comprobar SP

    assert_eq!(z80.reg.sp, 0xFFFD);
    // Comprobar los datos en la memoria que apunta el SP
    assert_eq!(z80.mem.mem[0xFFFD], 0x50);
    assert_eq!(z80.mem.mem[0xFFFE], 0xA2);
}
#[test]
fn ret_p() {
    // 0xF0 No afecta flags. Si el flag S anterior = 0   retorna haciendo un pop en el PC
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // a) se cumple la condicion pv=0 *****
    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    hace_un_call_incondicional_a_direcc_dada(&mut z80, 0xE0, 0x15, &mut c);

    // Pongo flag S a 0
    z80.reg.set_flag(&StatusFlag::Sign, false);
    c.reg.flags.s = false;

    // Ejecuto ret_nc
    ejecutar_en_direccion(&mut z80, 0xE015, 0xF0, 0, 0, 0, &mut c);

    // Comprobar que el PC ha retornado a la siguiente direccion
    assert_eq!(z80.reg.pc, 0xA252);

    // b) no se cumple la condicion pv=1 *****
    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    hace_un_call_incondicional_a_direcc_dada(&mut z80, 0xE0, 0x15, &mut c);

    // Pongo flag PV a 1
    z80.reg.set_flag(&StatusFlag::Sign, true);
    c.reg.flags.s = true;

    // Ejecuto ret_nz
    ejecutar_en_direccion(&mut z80, 0xE015, 0xF0, 0, 0, 0, &mut c);

    // Comprobar que el PC no retorna
    assert_eq!(z80.reg.pc, 0xE016);
}
#[test]
fn pop_af() {
    // 0xF1
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // pongo sp en 0xFF18
    //z80.registros.sp = 0xFF18;
    set_sp_test_big(&mut z80, &mut c, 0xFF18);

    // primero debemos hacer un push
    // alto = 0xE1    bajo = 0x2A
    hacer_push_test_nn(&mut z80, &mut c, 0xE12A);

    // ponemos PC a 0
    z80.reg.pc = 0;
    c.reg.pc = 0;

    // Ejecutamos POP en AF
    ejecutar(&mut z80, 0xF1, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0xE1);
    assert_eq!(z80.reg.f, 0x2A);
}

#[test]
fn jp_p_nn() {
    // 0xF2 No afecta a los flags  Si el flag PV anterior = 0   salta a nn
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // ponemos flag s = 1
    z80.reg.set_flag(&StatusFlag::Sign, true);
    c.reg.flags.s = true;

    ejecutar(&mut z80, 0xF2, 0x34, 0xA6, 0, &mut c);

    assert_eq!(z80.reg.pc, 0x0003);

    // ponemos flag pv = 0
    z80.reg.set_flag(&StatusFlag::Sign, false);
    c.reg.flags.s = false;

    // reseteamos pc
    z80.reg.pc = 0;
    c.reg.pc = 0;

    ejecutar(&mut z80, 0xF2, 0x34, 0xA6, 0, &mut c);

    assert_eq!(z80.reg.pc, 0xA634);
}
#[test]
fn di() {
    // 0xF3 No afecta a los flags,
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    z80.debug.debug_zilog = false;

    z80.iff1 = true;
    z80.iff2 = true;

    ejecutar(&mut z80, 0xF3, 0, 0, 0, &mut c);
    assert_eq!(z80.iff1, false);
    assert_eq!(z80.iff2, false);
}
#[test]
fn call_p_nn() {
    // 0xF4 n1 n2  no afecta flags  Si el flag S anterior = 0   hace call a nn
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Establecemos SP como 0xFF18
    //z80.registros.sp = 0xFF18;
    set_sp_test_big(&mut z80, &mut c, 0xFF18);

    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    z80.reg.pc = 0xA24F;
    c.reg.pc = 0xA24F;

    // ponemos s=0
    z80.reg.set_flag(&StatusFlag::Sign, false);
    c.reg.flags.s = false;

    // primero se almacena el mas significativo en el stack
    ejecutar_en_direccion(&mut z80, 0xA24F, 0xF4, 0x15, 0xE0, 0, &mut c);

    // comprobar que SP ha cambiado
    assert_eq!(z80.reg.sp, 0xFF16);

    // comprobar los bytes del stack

    // 0xFF16 = 0x52  (ha sumado 3 a 4F) para que cuando haya un ret vuelva
    // a la direccion posterior al call que siempre es de 3 bytes
    assert_eq!(z80.mem.mem[0xFF16], 0x52);
    assert_eq!(z80.mem.mem[0xFF17], 0xA2);

    // Comprueba el nuevo valor del pc
    assert_eq!(z80.reg.pc, 0xE015);
}

#[test]
fn push_af() {
    // 0xF5 No afecta a los flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // pongo sp en 0xFF18
    //z80.registros.sp = 0xFF18;
    set_sp_test_big(&mut z80, &mut c, 0xFF18);

    //hacer_push(&mut z80);
    // pongo HL = 0xE12A
    z80.reg.a = 0xE1;
    c.reg.a = 0xE1;
    z80.reg.f = 0x2A;
    c.reg.flags.set_from_byte(0x2A);

    // ejecutar push hl
    ejecutar(&mut z80, 0xF5, 0, 0, 0, &mut c);

    // Comprobar SP
    assert_eq!(z80.reg.sp, 0xFF16);

    // Comprobar los datos en la memoria que apunta el SP
    assert_eq!(z80.mem.mem[0xFF16], 0x2A);
    assert_eq!(z80.mem.mem[0xFF17], 0xE1);
}

#[test]
fn or_n() {
    // 0xF6 A | N -> A     Carry =0, N=0, PV detecta parity,  H=0, Z, S se definen
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;

    ejecutar(&mut z80, 0xF6, 0x09, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x0D, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);
}
#[test]
fn rst_30_h() {
    // 0xF7 No afecta flags, se hace push del PC, PC = 0x00
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Establecemos SP como 0xFF18
    //z80.registros.sp = 0xFF18;
    set_sp_test_big(&mut z80, &mut c, 0xFF18);

    // Establecemos PC como 0xA24F (Hacemos el rst 00h desde 0xA24F
    z80.reg.pc = 0xA24F;
    c.reg.pc = 0xA24F;

    // primero se almacena el mas significativo en el stack
    ejecutar_en_direccion(&mut z80, 0xA24F, 0xF7, 0, 0, 0, &mut c);

    // Comprobar que el PC=0x10
    assert_eq!(z80.reg.pc, 0x30);

    // Comprobar SP
    assert_eq!(z80.reg.sp, 0xFF16);

    // Comprobar los datos en la memoria que apunta el SP
    assert_eq!(z80.mem.mem[0xFF16], 0x50);
    assert_eq!(z80.mem.mem[0xFF17], 0xA2);
}

#[test]
fn ret_m() {
    // 0xF8 No afecta flags. Si el flag de signo anterior = 1   retorna haciendo un pop en el PC
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // a) se cumple la condicion S=1 *****
    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    hace_un_call_incondicional_a_direcc_dada(&mut z80, 0xE0, 0x15, &mut c);

    // Pongo flag S a 1
    z80.reg.set_flag(&StatusFlag::Sign, true);
    c.reg.flags.s = true;

    // Ejecuto ret_m
    ejecutar_en_direccion(&mut z80, 0xE015, 0xF8, 0, 0, 0, &mut c);

    // Comprobar que el PC ha retornado a la siguiente direccion
    assert_eq!(z80.reg.pc, 0xA252);
    c.reg.pc = 0xA252;

    // b) no se cumple la condicion S=0 *****
    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    hace_un_call_incondicional_a_direcc_dada(&mut z80, 0xE0, 0x15, &mut c);

    // Pongo flag S a 0
    z80.reg.set_flag(&StatusFlag::Sign, false);
    c.reg.flags.s = false;

    // Ejecuto ret_nz
    ejecutar_en_direccion(&mut z80, 0xE015, 0xF8, 0, 0, 0, &mut c);

    // Comprobar que el PC no retorna
    assert_eq!(z80.reg.pc, 0xE016);
}

#[test]
fn ld_sp_hl() {
    // 0xF9 No afecta a los flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // ponemos un valor a hl
    //z80.registros.h = 0x3F;
    //z80.registros.l = 0x52;
    set_hl_test_big(&mut z80, &mut c, 0x3F52);

    ejecutar(&mut z80, 0xF9, 0, 0, 0, &mut c);

    assert_eq!(z80.reg.sp, 0x3F52);
}
#[test]
fn jp_m_nn() {
    // 0xFA No afecta a los flags Si s=1 salta a nn
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // ponemos flag s = 0
    z80.reg.set_flag(&StatusFlag::Sign, false);
    c.reg.flags.s = false;

    ejecutar(&mut z80, 0xFA, 0x34, 0xA6, 0, &mut c);

    assert_eq!(z80.reg.pc, 0x0003);

    // ponemos flag s = 1
    z80.reg.set_flag(&StatusFlag::Sign, true);
    c.reg.flags.s = true;

    // reseteamos pc
    z80.reg.set_pc(0);
    c.reg.pc = 0;

    ejecutar(&mut z80, 0xFA, 0x34, 0xA6, 0, &mut c);

    assert_eq!(z80.reg.pc, 0xA634);
}

#[test]
fn ei() {
    // 0xFB no afecta a los flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    z80.debug.debug_zilog = false;

    z80.iff1 = false;
    z80.iff2 = false;

    ejecutar(&mut z80, 0xFB, 0, 0, 0, &mut c);

    assert_eq!(z80.iff1, true);
    assert_eq!(z80.iff2, true);
}

#[test]
fn call_m_nn() {
    // 0xFC n1 n2  no afecta flags, si s=1 hace call
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Establecemos SP como 0xFFFF
    //z80.registros.sp = 0xFFFF;
    set_sp_test_big(&mut z80, &mut c, 0xFFFF);

    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    z80.reg.pc = 0xA24F;
    c.reg.pc = 0xA24F;

    // ponemos s=1
    z80.reg.set_flag(&StatusFlag::Sign, true);
    c.reg.flags.s = true;

    // primero se almacena el mas significativo en el stack
    ejecutar_en_direccion(&mut z80, 0xA24F, 0xFC, 0x15, 0xE0, 0, &mut c);

    // comprobar que SP ha cambiado
    assert_eq!(z80.reg.sp, 0xFFFD);

    // comprobar los bytes del stack
    assert_eq!(z80.mem.mem[0xFFFE], 0xA2);
    // 0xFFFD = 0x52  (ha sumado 3 a 4F) para que cuando haya un ret vuelva
    // a la direccion posterior al call que siempre es de 3 bytes
    assert_eq!(z80.mem.mem[0xFFFD], 0x52);

    // Comprueba el nuevo valor del pc
    assert_eq!(z80.reg.pc, 0xE015);
}
// 0xFD Instrucciones IY

#[test]
fn cp_n() {
    // 0xFE Carry se define, N=1, PV detecta overflow, H, Z, S se definen
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;

    ejecutar(&mut z80, 0xFE, 0x09, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x0C, "Valor en A incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 1, 0);
}

#[test]
fn rst_38_h() {
    // 0xFF No afecta flags, se hace push del PC, PC = 0x38
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Establecemos SP como 0xFFFF
    //z80.registros.sp = 0xFFFF;
    set_sp_test_big(&mut z80, &mut c, 0xFFFF);

    // Establecemos PC como 0xA24F (Hacemos el rst 00h desde 0xA24F
    z80.reg.pc = 0xA24F;
    c.reg.pc = 0xA24F;

    // primero se almacena el mas significativo en el stack
    ejecutar_en_direccion(&mut z80, 0xA24F, 0xFF, 0, 0, 0, &mut c);

    // Comprobar que el PC=0x28
    assert_eq!(z80.reg.pc, 0x0038);

    // Comprobar SP
    assert_eq!(z80.reg.sp, 0xFFFD);

    // Comprobar los datos en la memoria que apunta el SP
    assert_eq!(z80.mem.mem[0xFFFD], 0x50);
    assert_eq!(z80.mem.mem[0xFFFE], 0xA2);
}





