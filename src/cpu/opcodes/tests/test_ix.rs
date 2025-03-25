use crate::cpu::opcodes::tests::test_aux::*;
use crate::z80::Z80;
use zilog_z80::cpu::CPU;
// DD
#[test]
fn ld_b_n() {
    // DD 06 N   ld b,n   no afecta alos flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // TODO
    z80.debug.debug_zilog = false;

    ejecutar(&mut z80, 0xDD, 0x06, 0x5A, 0, &mut c);

    // compruebo que ahora b = 0x5A
    assert_eq!(z80.reg.b, 0x5A);
}
#[test]
fn ld_c_n() {
    // DD 0E N   ld b,n no afecta alos flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // TODO
    z80.debug.debug_zilog = false;

    ejecutar(&mut z80, 0xDD, 0x0E, 0x5A, 0, &mut c);

    assert_eq!(z80.reg.c, 0x5A);
}

#[test]
fn ld_d_n() {
    // DD 16 N    ld d,n no afecta alos flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // TODO
    z80.debug.debug_zilog = false;

    ejecutar(&mut z80, 0xDD, 0x16, 0x5A, 0, &mut c);

    assert_eq!(z80.reg.d, 0x5A);
}

#[test]
fn ld_e_n() {
    // DD 1E N    ld e,n no afecta alos flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // TODO
    z80.debug.debug_zilog = false;

    ejecutar(&mut z80, 0xDD, 0x1E, 0x5A, 0, &mut c);

    assert_eq!(z80.reg.e, 0x5A);
}

#[test]
fn ld_ixh_n() {
    // DD 26 N  ld ixh,n no afecta alos flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // TODO
    z80.debug.debug_zilog = false;

    ejecutar(&mut z80, 0xDD, 0x1E, 0x5A, 0, &mut c);

    assert_eq!(z80.reg.e, 0x5A);
}
#[test]
fn inc_ixh() {
    // 0xDD 0x24 INC IXH segun deepseek (No estándar) hay que probarlo
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.ix = 0x1234;
    c.reg.set_ix(0x1234);
    ejecutar(&mut z80, 0xDD, 0x24, 0, 0, &mut c);
}