use crate::cpu::opcodes::tests::test_aux::*;
use crate::ops::StatusFlag;
use crate::z80::Z80;
use zilog_z80::cpu::CPU;

#[test]
fn rlc_b() {
    // 0xCB 0x00 rlc b    rota bits de registro de 8 bits a la izquierda       Bit7->C     Bit7->Bit0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.b = 0b1000_0000;
    c.reg.b = 0b1000_0000;

    ejecutar(&mut z80, 0xCB, 0x00, 0, 0, &mut c);
    assert_eq!(z80.reg.b, 0b0000_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn rlc_c() {
    // 0xCB 0x01 rlc c    rota bits de registro de 8 bits a la izquierda       Bit7->C     Bit7->Bit0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.c = 0b1000_0000;
    c.reg.c = 0b1000_0000;

    ejecutar(&mut z80, 0xCB, 0x01, 0, 0, &mut c);
    assert_eq!(z80.reg.c, 0b0000_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn rlc_d() {
    // 0xCB 0x02 rlc d    rota bits de registro de 8 bits a la izquierda       Bit7->C     Bit7->Bit0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.d = 0b1000_0000;
    c.reg.d = 0b1000_0000;

    ejecutar(&mut z80, 0xCB, 0x02, 0, 0, &mut c);
    assert_eq!(z80.reg.d, 0b0000_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn rlc_e() {
    // 0xCB 0x03 rlc e    rota bits de registro de 8 bits a la izquierda       Bit7->C     Bit7->Bit0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.e = 0b1000_0000;
    c.reg.e = 0b1000_0000;

    ejecutar(&mut z80, 0xCB, 0x03, 0, 0, &mut c);
    assert_eq!(z80.reg.e, 0b0000_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn rlc_h() {
    // 0xCB 0x04 rlc h    rota bits de registro de 8 bits a la izquierda       Bit7->C     Bit7->Bit0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.h = 0b1000_0000;
    c.reg.h = 0b1000_0000;

    ejecutar(&mut z80, 0xCB, 0x04, 0, 0, &mut c);
    assert_eq!(z80.reg.h, 0b0000_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn rlc_l() {
    // 0xCB 0x05 rlc l    rota bits de registro de 8 bits a la izquierda       Bit7->C     Bit7->Bit0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.l = 0b1000_0000;
    c.reg.l = 0b1000_0000;

    ejecutar(&mut z80, 0xCB, 0x05, 0, 0, &mut c);
    assert_eq!(z80.reg.l, 0b0000_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn rlc_0hl0() {
    // 0xCB 0x06 rlc (hl))    rota bits de registro de 8 bits a la izquierda       Bit7->C     Bit7->Bit0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // pongo HL = 0x10EF
    //z80.registros.h = 0x10;
    //z80.registros.l = 0xEF;
    set_hl_test_big(&mut z80, &mut c, 0x10EF);

    z80.mem.mem[0x10EF] = 0b1000_0000;
    c.bus.write_byte(0x10EF, 0b1000_0000);

    ejecutar(&mut z80, 0xCB, 0x06, 0, 0, &mut c);
    assert_eq!(z80.mem.mem[0x10EF], 0b0000_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn rlc_a() {
    // 0xCB 0x07 rlc a    rota bits de registro de 8 bits a la izquierda       Bit7->C     Bit7->Bit0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0b1000_0000;
    c.reg.a = 0b1000_0000;

    ejecutar(&mut z80, 0xCB, 0x07, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0b0000_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn rrc_b() {
    // 0xCB 0x08 rrc b    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Bit0->Bit7
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.b = 0b0000_0001;
    c.reg.b = 0b0000_0001;

    ejecutar(&mut z80, 0xCB, 0x08, 0, 0, &mut c);
    assert_eq!(z80.reg.b, 0b1000_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 1);
}
#[test]
fn rrc_c() {
    // 0xCB 0x09 rrc c    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Bit0->Bit7
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.c = 0b0000_0001;
    c.reg.c = 0b0000_0001;

    ejecutar(&mut z80, 0xCB, 0x09, 0, 0, &mut c);
    assert_eq!(z80.reg.c, 0b1000_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 1);
}
#[test]
fn rrc_d() {
    // 0xCB 0x0A rrc d    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Bit0->Bit7
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.d = 0b0000_0001;
    c.reg.d = 0b0000_0001;

    ejecutar(&mut z80, 0xCB, 0x0A, 0, 0, &mut c);
    assert_eq!(z80.reg.d, 0b1000_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 1);
}
#[test]
fn rrc_e() {
    // 0xCB 0x0B rrc e    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Bit0->Bit7
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.e = 0b0000_0001;
    c.reg.e = 0b0000_0001;

    ejecutar(&mut z80, 0xCB, 0x0B, 0, 0, &mut c);
    assert_eq!(z80.reg.e, 0b1000_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 1);
}
#[test]
fn rrc_h() {
    // 0xCB 0x0C rrc h    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Bit0->Bit7
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.h = 0b0000_0001;
    c.reg.h = 0b0000_0001;

    ejecutar(&mut z80, 0xCB, 0x0C, 0, 0, &mut c);
    assert_eq!(z80.reg.h, 0b1000_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 1);
}
#[test]
fn rrc_l() {
    // 0xCB 0x0D rrc l    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Bit0->Bit7
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.l = 0b0000_0001;
    c.reg.l = 0b0000_0001;

    ejecutar(&mut z80, 0xCB, 0x0D, 0, 0, &mut c);
    assert_eq!(z80.reg.l, 0b1000_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 1);
}
#[test]
fn rrc_0hl0() {
    // 0xCB 0x0E rrc (hl))    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Bit0->Bit7
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // pongo HL = 0x10EF
    //z80.registros.h = 0xEF;
    //z80.registros.l = 0x10;
    set_hl_test_big(&mut z80, &mut c, 0x10EF);

    z80.mem.mem[0x10EF] = 0b0000_0001;
    c.bus.write_byte(0x10EF, 0b0000_0001);

    ejecutar(&mut z80, 0xCB, 0x0E, 0, 0, &mut c);
    assert_eq!(z80.mem.mem[0x10EF], 0b1000_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 1);
}
#[test]
fn rrc_a() {
    // 0xCB 0x0F rrc a    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Bit0->Bit7
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0b0000_0001;
    c.reg.a = 0b0000_0001;

    ejecutar(&mut z80, 0xCB, 0x0F, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0b1000_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 1);
}

#[test]
fn rl_b() {
    // 0xCB 0x10 rl b    rota bits de registro de 8 bits a la izquierda       Bit7->Carry     previo carry->Bit0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;

    z80.reg.b = 0b1000_1000;
    c.reg.b = 0b1000_1000;

    ejecutar(&mut z80, 0xCB, 0x10, 0, 0, &mut c);
    assert_eq!(z80.reg.b, 0b0001_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);
}
#[test]
fn rl_c() {
    // 0xCB 0x11 rl c    rota bits de registro de 8 bits a la izquierda       Bit7->Carry     Bit7->Bit0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.c = 0b1000_1000;
    c.reg.c = 0b1000_1000;

    ejecutar(&mut z80, 0xCB, 0x11, 0, 0, &mut c);
    assert_eq!(z80.reg.c, 0b0001_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);
}
#[test]
fn rl_d() {
    // 0xCB 0x12 rl d    rota bits de registro de 8 bits a la izquierda       Bit7->Carry     Bit7->Bit0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.d = 0b1000_1000;
    c.reg.d = 0b1000_1000;

    ejecutar(&mut z80, 0xCB, 0x12, 0, 0, &mut c);
    assert_eq!(z80.reg.d, 0b0001_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);
}
#[test]
fn rl_e() {
    // 0xCB 0x13 rl e    rota bits de registro de 8 bits a la izquierda       Bit7->Carry     Bit7->Bit0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.e = 0b1000_1000;
    c.reg.e = 0b1000_1000;

    ejecutar(&mut z80, 0xCB, 0x13, 0, 0, &mut c);
    assert_eq!(z80.reg.e, 0b0001_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);
}
#[test]
fn rl_h() {
    // 0xCB 0x14 rl h    rota bits de registro de 8 bits a la izquierda       Bit7->Carry     Bit7->Bit0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.h = 0b1000_1000;
    c.reg.h = 0b1000_1000;

    ejecutar(&mut z80, 0xCB, 0x14, 0, 0, &mut c);
    assert_eq!(z80.reg.h, 0b0001_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);
}
#[test]
fn rl_l() {
    // 0xCB 0x15 rl l    rota bits de registro de 8 bits a la izquierda       Bit7->Carry     Bit7->Bit0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.l = 0b1000_1000;
    c.reg.l = 0b1000_1000;

    ejecutar(&mut z80, 0xCB, 0x15, 0, 0, &mut c);
    assert_eq!(z80.reg.l, 0b0001_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);
}
#[test]
fn rl_0hl0() {
    // 0xCB 0x16 rl (hl))    rota bits de registro de 8 bits a la izquierda       Bit7->Carry     Bit7->Bit0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    // pongo HL = 0x10EF
    //z80.registros.h = 0xEF;
    //z80.registros.l = 0x10;
    set_hl_test_big(&mut z80, &mut c, 0xEF10);
    z80.mem.mem[0xEF10] = 0b1000_1000;
    c.bus.write_byte(0xEF10, 0b1000_1000);

    ejecutar(&mut z80, 0xCB, 0x16, 0, 0, &mut c);
    assert_eq!(z80.mem.mem[0xEF10], 0b0001_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);
}
#[test]
fn rl_a() {
    // 0xCB 0x17 rl a    rota bits de registro de 8 bits a la izquierda       Bit7->C     Bit7->Bit0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0b1000_1000;
    c.reg.a = 0b1000_1000;

    ejecutar(&mut z80, 0xCB, 0x17, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0b0001_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);
}
#[test]
fn rr_b() {
    // 0xCB 0x18 rr b    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Carry viejo->Bit7
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.b = 0b0000_0001;
    c.reg.b = 0b0000_0001;

    ejecutar(&mut z80, 0xCB, 0x18, 0, 0, &mut c);
    assert_eq!(z80.reg.b, 0b0000_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 0, 1, 0, 1);
}
#[test]
fn rr_c() {
    // 0xCB 0x19 rr c    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Bit0->Bit7
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.c = 0b0000_0001;
    c.reg.c = 0b0000_0001;

    ejecutar(&mut z80, 0xCB, 0x19, 0, 0, &mut c);
    assert_eq!(z80.reg.c, 0b0000_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 0, 1, 0, 1);
}
#[test]
fn rr_d() {
    // 0xCB 0x1A rr d    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Bit0->Bit7
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.d = 0b0000_0001;
    c.reg.d = 0b0000_0001;

    ejecutar(&mut z80, 0xCB, 0x1A, 0, 0, &mut c);
    assert_eq!(z80.reg.d, 0b0000_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 0, 1, 0, 1);
}
#[test]
fn rr_e() {
    // 0xCB 0x1B rr e    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Bit0->Bit7
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.e = 0b0000_0001;
    c.reg.e = 0b0000_0001;

    ejecutar(&mut z80, 0xCB, 0x1B, 0, 0, &mut c);
    assert_eq!(z80.reg.e, 0b0000_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 0, 1, 0, 1);
}
#[test]
fn rr_h() {
    // 0xCB 0x1C rr h    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Bit0->Bit7
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.h = 0b0000_0001;
    c.reg.h = 0b0000_0001;

    ejecutar(&mut z80, 0xCB, 0x1C, 0, 0, &mut c);
    assert_eq!(z80.reg.h, 0b0000_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 0, 1, 0, 1);
}
#[test]
fn rr_l() {
    // 0xCB 0x1D rr l    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Bit0->Bit7
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.l = 0b0000_0001;
    c.reg.l = 0b0000_0001;

    ejecutar(&mut z80, 0xCB, 0x1D, 0, 0, &mut c);
    assert_eq!(z80.reg.l, 0b0000_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 0, 1, 0, 1);
}
#[test]
fn rr_0hl0() {
    // 0xCB 0x1E rr (hl))    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Bit0->Bit7
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    // pongo HL = 0xEF10
    //z80.registros.h = 0xEF;
    //z80.registros.l = 0x10;
    set_hl_test_big(&mut z80, &mut c, 0xEF10);
    z80.mem.mem[0xEF10] = 0b0000_0001;
    c.bus.write_byte(0xEF10, 0b0000_0001);

    ejecutar(&mut z80, 0xCB, 0x1E, 0, 0, &mut c);
    assert_eq!(z80.mem.mem[0xEF10], 0b0000_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 0, 1, 0, 1);
}
#[test]
fn rr_a() {
    // 0xCB 0x1F rr a    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Bit0->Bit7
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0b0000_0001;
    c.reg.a = 0b0000_0001;

    ejecutar(&mut z80, 0xCB, 0x1F, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0b0000_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 0, 1, 0, 1);
}

#[test]
fn sla_b() {
    // 0xCB 0x20 sla b    rota bits de registro de 8 bits a la izquierda       Bit7->Carry     Bit0=0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.b = 0b1000_1000;
    c.reg.b = 0b1000_1000;

    ejecutar(&mut z80, 0xCB, 0x20, 0, 0, &mut c);
    assert_eq!(z80.reg.b, 0b0001_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn sla_c() {
    // 0xCB 0x21 sla c    rota bits de registro de 8 bits a la izquierda       Bit7->Carry     Bit0=0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.c = 0b1000_1000;
    c.reg.c = 0b1000_1000;

    ejecutar(&mut z80, 0xCB, 0x21, 0, 0, &mut c);
    assert_eq!(z80.reg.c, 0b0001_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn sla_d() {
    // 0xCB 0x22 sla d    rota bits de registro de 8 bits a la izquierda       Bit7->Carry     Bit0=0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.d = 0b1000_1000;
    c.reg.d = 0b1000_1000;

    ejecutar(&mut z80, 0xCB, 0x22, 0, 0, &mut c);
    assert_eq!(z80.reg.d, 0b0001_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn sla_e() {
    // 0xCB 0x23 sla e    rota bits de registro de 8 bits a la izquierda       Bit7->Carry     Bit0=0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.e = 0b1000_1000;
    c.reg.e = 0b1000_1000;

    ejecutar(&mut z80, 0xCB, 0x23, 0, 0, &mut c);
    assert_eq!(z80.reg.e, 0b0001_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn sla_h() {
    // 0xCB 0x24 sla h    rota bits de registro de 8 bits a la izquierda       Bit7->Carry     Bit0=0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.h = 0b1000_1000;
    c.reg.h = 0b1000_1000;

    ejecutar(&mut z80, 0xCB, 0x24, 0, 0, &mut c);
    assert_eq!(z80.reg.h, 0b0001_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn sla_l() {
    // 0xCB 0x25 sla l    rota bits de registro de 8 bits a la izquierda       Bit7->Carry     Bit0=0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.l = 0b1000_1000;
    c.reg.l = 0b1000_1000;

    ejecutar(&mut z80, 0xCB, 0x25, 0, 0, &mut c);
    assert_eq!(z80.reg.l, 0b0001_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn sla_0hl0() {
    // 0xCB 0x26 sla (hl))    rota bits de registro de 8 bits a la izquierda       Bit7->Carry     Bit0=0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // pongo HL = 0xEF10
    // z80.registros.h = 0xEF;
    // z80.registros.l = 0x10;
    set_hl_test_big(&mut z80, &mut c, 0xEF10);
    z80.mem.mem[0xEF10] = 0b1000_1000;
    c.bus.write_byte(0xEF10, 0b1000_1000);

    ejecutar(&mut z80, 0xCB, 0x26, 0, 0, &mut c);
    assert_eq!(z80.mem.mem[0xEF10], 0b0001_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn sla_a() {
    // 0xCB 0x27 sla a    rota bits de registro de 8 bits a la izquierda       Bit7->C     Bit0=0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;
    z80.reg.a = 0b1000_1000;
    c.reg.a = 0b1000_1000;

    ejecutar(&mut z80, 0xCB, 0x27, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0b0001_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn sra_b() {
    // 0xCB 0x28 sra b    mueve bits de registro de 8 bits a la derecha       Bit0->Carry    Bit7se mantiene
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.b = 0b1000_0001;
    c.reg.b = 0b1000_0001;

    ejecutar(&mut z80, 0xCB, 0x28, 0, 0, &mut c);
    assert_eq!(z80.reg.b, 0b1100_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 1, 0, 1);
}
#[test]
fn sra_c() {
    // 0xCB 0x29 sra c    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Bit7se mantiene
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.c = 0b1000_0001;
    c.reg.c = 0b1000_0001;

    ejecutar(&mut z80, 0xCB, 0x29, 0, 0, &mut c);
    assert_eq!(z80.reg.c, 0b1100_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 1, 0, 1);
}
#[test]
fn sra_d() {
    // 0xCB 0x2A sra d    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Bit7se mantiene
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.d = 0b1000_0001;
    c.reg.d = 0b1000_0001;

    ejecutar(&mut z80, 0xCB, 0x2A, 0, 0, &mut c);
    assert_eq!(z80.reg.d, 0b1100_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 1, 0, 1);
}
#[test]
fn sra_e() {
    // 0xCB 0x2B sra e    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Bit7se mantiene
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.e = 0b1000_0001;
    c.reg.e = 0b1000_0001;

    ejecutar(&mut z80, 0xCB, 0x2B, 0, 0, &mut c);
    assert_eq!(z80.reg.e, 0b1100_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 1, 0, 1);
}
#[test]
fn sra_h() {
    // 0xCB 0x2C sra h    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Bit7se mantiene
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.h = 0b1000_0001;
    c.reg.h = 0b1000_0001;

    ejecutar(&mut z80, 0xCB, 0x2C, 0, 0, &mut c);
    assert_eq!(z80.reg.h, 0b1100_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 1, 0, 1);
}
#[test]
fn sra_l() {
    // 0xCB 0x2D sra l    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Bit7se mantiene
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.l = 0b1000_0001;
    c.reg.l = 0b1000_0001;

    ejecutar(&mut z80, 0xCB, 0x2D, 0, 0, &mut c);
    assert_eq!(z80.reg.l, 0b1100_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 1, 0, 1);
}
#[test]
fn sra_0hl0() {
    // 0xCB 0x2E sra (hl))    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Bit7se mantiene
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // pongo HL = 0xEF10
    // z80.registros.h = 0xEF;
    //z80.registros.l = 0x10;
    set_hl_test_big(&mut z80, &mut c, 0xEF10);
    z80.mem.mem[0xEF10] = 0b1000_0001;
    c.bus.write_byte(0xEF10, 0b1000_0001);

    ejecutar(&mut z80, 0xCB, 0x2E, 0, 0, &mut c);
    assert_eq!(z80.mem.mem[0xEF10], 0b1100_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 1, 0, 1);
}
#[test]
fn sra_a() {
    // 0xCB 0x2F sra a    rota bits de registro de 8 bits a la derecha       Bit0->Carry     Bit7se mantiene
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;
    z80.reg.a = 0b1000_0001;
    c.reg.a = 0b1000_0001;

    ejecutar(&mut z80, 0xCB, 0x2F, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0b1100_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 1, 0, 1);
}

#[test]
fn sll_b() {
    // 0xCB 0x30 sll b    rota bits de registro de 8 bits a la izquierda       Bit7->Carry     Bit0=1
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.b = 0b1000_1000;
    c.reg.b = 0b1000_1000;

    ejecutar(&mut z80, 0xCB, 0x30, 0, 0, &mut c);
    assert_eq!(z80.reg.b, 0b0001_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);
}
#[test]
fn sll_c() {
    // 0xCB 0x31 sll c    rota bits de registro de 8 bits a la izquierda       Bit7->Carry     Bit0=1
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.c = 0b1000_1000;
    c.reg.c = 0b1000_1000;

    ejecutar(&mut z80, 0xCB, 0x31, 0, 0, &mut c);
    assert_eq!(z80.reg.c, 0b0001_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);
}
#[test]
fn sll_d() {
    // 0xCB 0x32 sll d    rota bits de registro de 8 bits a la izquierda       Bit7->Carry     Bit0=1
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.d = 0b1000_1000;
    c.reg.d = 0b1000_1000;

    ejecutar(&mut z80, 0xCB, 0x32, 0, 0, &mut c);
    assert_eq!(z80.reg.d, 0b0001_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);
}
#[test]
fn sll_e() {
    // 0xCB 0x33 sll e    rota bits de registro de 8 bits a la izquierda       Bit7->Carry     Bit0=1
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.e = 0b1000_1000;
    c.reg.e = 0b1000_1000;

    ejecutar(&mut z80, 0xCB, 0x33, 0, 0, &mut c);
    assert_eq!(z80.reg.e, 0b0001_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);
}
#[test]
fn sll_h() {
    // 0xCB 0x34 sll h    rota bits de registro de 8 bits a la izquierda       Bit7->Carry     Bit0=1
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.h = 0b1000_1000;
    c.reg.h = 0b1000_1000;

    ejecutar(&mut z80, 0xCB, 0x34, 0, 0, &mut c);
    assert_eq!(z80.reg.h, 0b0001_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);
}
#[test]
fn sll_l() {
    // 0xCB 0x35 sll l    rota bits de registro de 8 bits a la izquierda       Bit7->Carry     Bit0=1
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.l = 0b1000_1000;
    c.reg.l = 0b1000_1000;

    ejecutar(&mut z80, 0xCB, 0x35, 0, 0, &mut c);
    assert_eq!(z80.reg.l, 0b0001_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);
}
#[test]
fn sll_0hl0() {
    // 0xCB 0x36 sll (hl))    rota bits de registro de 8 bits a la izquierda       Bit7->Carry     Bit0=1
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // pongo HL = 0xEF10
    //z80.registros.h = 0xEF;
    //z80.registros.l = 0x10;
    set_hl_test_big(&mut z80, &mut c, 0xEF10);
    z80.mem.mem[0xEF10] = 0b1000_1000;
    c.bus.write_byte(0xEF10, 0b1000_1000);

    ejecutar(&mut z80, 0xCB, 0x36, 0, 0, &mut c);
    assert_eq!(z80.mem.mem[0xEF10], 0b0001_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);
}
#[test]
fn sll_a() {
    // 0xCB 0x37 sll a    rota bits de registro de 8 bits a la izquierda       Bit7->C     Bit0=1
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0b1000_1000;
    c.reg.a = 0b1000_1000;

    ejecutar(&mut z80, 0xCB, 0x37, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0b0001_0001);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);
}
#[test]
fn srl_b() {
    // 0xCB 0x38 srl b    mueve bits de registro de 8 bits a la derecha       Bit0->Carry    Bit7=0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.b = 0b1000_0001;
    c.reg.b = 0b1000_0001;

    ejecutar(&mut z80, 0xCB, 0x38, 0, 0, &mut c);
    assert_eq!(z80.reg.b, 0b0100_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn srl_c() {
    // 0xCB 0x39 srl c    rota bits de registro de 8 bits a la derecha       Bit0->Carry      Bit7=0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.c = 0b1000_0001;
    c.reg.c = 0b1000_0001;

    ejecutar(&mut z80, 0xCB, 0x39, 0, 0, &mut c);
    assert_eq!(z80.reg.c, 0b0100_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn srl_d() {
    // 0xCB 0x3A srl d    rota bits de registro de 8 bits a la derecha       Bit0->Carry      Bit7=0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.d = 0b1000_0001;
    c.reg.d = 0b1000_0001;

    ejecutar(&mut z80, 0xCB, 0x3A, 0, 0, &mut c);
    assert_eq!(z80.reg.d, 0b0100_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn srl_e() {
    // 0xCB 0x3B srl e    rota bits de registro de 8 bits a la derecha       Bit0->Carry      Bit7=0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.e = 0b1000_0001;
    c.reg.e = 0b1000_0001;

    ejecutar(&mut z80, 0xCB, 0x3B, 0, 0, &mut c);
    assert_eq!(z80.reg.e, 0b0100_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn srl_h() {
    // 0xCB 0x3C srl h    rota bits de registro de 8 bits a la derecha       Bit0->Carry      Bit7=0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.h = 0b1000_0001;
    c.reg.h = 0b1000_0001;

    ejecutar(&mut z80, 0xCB, 0x3C, 0, 0, &mut c);
    assert_eq!(z80.reg.h, 0b0100_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn srl_l() {
    // 0xCB 0x3D srl l    rota bits de registro de 8 bits a la derecha       Bit0->Carry      Bit7=0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.l = 0b1000_0001;
    c.reg.l = 0b1000_0001;

    ejecutar(&mut z80, 0xCB, 0x3D, 0, 0, &mut c);
    assert_eq!(z80.reg.l, 0b0100_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn srl_0hl0() {
    // 0xCB 0x3E srl (hl))    rota bits de registro de 8 bits a la derecha       Bit0->Carry      Bit7=0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // pongo HL = 0x10EF
    //z80.registros.h = 0xEF;
    //z80.registros.l = 0x10;
    set_hl_test_big(&mut z80, &mut c, 0xEF10);
    z80.mem.mem[0xEF10] = 0b1000_0001;
    c.bus.write_byte(0xEF10, 0b1000_0001);

    ejecutar(&mut z80, 0xCB, 0x3E, 0, 0, &mut c);
    assert_eq!(z80.mem.mem[0xEF10], 0b0100_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn srl_a() {
    // 0xCB 0x3F srl a    rota bits de registro de 8 bits a la derecha       Bit0->Carry      Bit7=0
    // Carry se define  N reset, P/V detecta paridad, H=0, Z,S se define
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0b1000_0001;
    c.reg.a = 0b1000_0001;

    ejecutar(&mut z80, 0xCB, 0x3F, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0b0100_0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}

// Prueba todas las instrucciones bit cd la tabla CB
#[test]
fn testea_bit() {
    // Prueba bit de registro. Carry no afecta, N=0,P/V indefinido H=1 Z definido  S indefinido
    let mut z80 = Z80::default();
    let mut cpu_z = CPU::new(0xFFFF);

    // B=000 C=001 D=010 E=011 H=100 L=101 A=111
    // BIT para B (0b000)
    bit_pba(&mut z80, 0, 0b000, 0x40, &mut cpu_z); // BIT 0,B (CB 40)
    bit_pba(&mut z80, 1, 0b000, 0x48, &mut cpu_z); // BIT 1,B (CB 48)
    bit_pba(&mut z80, 2, 0b000, 0x50, &mut cpu_z); // BIT 2,B (CB 50)
    bit_pba(&mut z80, 3, 0b000, 0x58, &mut cpu_z); // BIT 3,B (CB 58)
    bit_pba(&mut z80, 4, 0b000, 0x60, &mut cpu_z); // BIT 4,B (CB 60)
    bit_pba(&mut z80, 5, 0b000, 0x68, &mut cpu_z); // BIT 5,B (CB 68)
    bit_pba(&mut z80, 6, 0b000, 0x70, &mut cpu_z); // BIT 6,B (CB 70)
    bit_pba(&mut z80, 7, 0b000, 0x78, &mut cpu_z); // BIT 7,B (CB 78)

    // BIT para C (0b001)
    bit_pba(&mut z80, 0, 0b001, 0x41, &mut cpu_z); // BIT 0,C (CB 41)
    bit_pba(&mut z80, 1, 0b001, 0x49, &mut cpu_z); // BIT 1,C (CB 49)
    bit_pba(&mut z80, 2, 0b001, 0x51, &mut cpu_z); // BIT 2,C (CB 51)
    bit_pba(&mut z80, 3, 0b001, 0x59, &mut cpu_z); // BIT 3,C (CB 59)
    bit_pba(&mut z80, 4, 0b001, 0x61, &mut cpu_z); // BIT 4,C (CB 61)
    bit_pba(&mut z80, 5, 0b001, 0x69, &mut cpu_z); // BIT 5,C (CB 69)
    bit_pba(&mut z80, 6, 0b001, 0x71, &mut cpu_z); // BIT 6,C (CB 71)
    bit_pba(&mut z80, 7, 0b001, 0x79, &mut cpu_z); // BIT 7,C (CB 79)

    // BIT para D (0b010)
    bit_pba(&mut z80, 0, 0b010, 0x42, &mut cpu_z); // BIT 0,D (CB 42)
    bit_pba(&mut z80, 1, 0b010, 0x4A, &mut cpu_z); // BIT 1,D (CB 4A)
    bit_pba(&mut z80, 2, 0b010, 0x52, &mut cpu_z); // BIT 2,D (CB 52)
    bit_pba(&mut z80, 3, 0b010, 0x5A, &mut cpu_z); // BIT 3,D (CB 5A)
    bit_pba(&mut z80, 4, 0b010, 0x62, &mut cpu_z); // BIT 4,D (CB 62)
    bit_pba(&mut z80, 5, 0b010, 0x6A, &mut cpu_z); // BIT 5,D (CB 6A)
    bit_pba(&mut z80, 6, 0b010, 0x72, &mut cpu_z); // BIT 6,D (CB 72)
    bit_pba(&mut z80, 7, 0b010, 0x7A, &mut cpu_z); // BIT 7,D (CB 7A)

    // BIT para E (0b011)
    bit_pba(&mut z80, 0, 0b011, 0x43, &mut cpu_z); // BIT 0,E (CB 43)
    bit_pba(&mut z80, 1, 0b011, 0x4B, &mut cpu_z); // BIT 1,E (CB 4B)
    bit_pba(&mut z80, 2, 0b011, 0x53, &mut cpu_z); // BIT 2,E (CB 53)
    bit_pba(&mut z80, 3, 0b011, 0x5B, &mut cpu_z); // BIT 3,E (CB 5B)
    bit_pba(&mut z80, 4, 0b011, 0x63, &mut cpu_z); // BIT 4,E (CB 63)
    bit_pba(&mut z80, 5, 0b011, 0x6B, &mut cpu_z); // BIT 5,E (CB 6B)
    bit_pba(&mut z80, 6, 0b011, 0x73, &mut cpu_z); // BIT 6,E (CB 73)
    bit_pba(&mut z80, 7, 0b011, 0x7B, &mut cpu_z); // BIT 7,E (CB 7B)

    // BIT para H (0b100)
    bit_pba(&mut z80, 0, 0b100, 0x44, &mut cpu_z); // BIT 0,H (CB 44)
    bit_pba(&mut z80, 1, 0b100, 0x4C, &mut cpu_z); // BIT 1,H (CB 4C)
    bit_pba(&mut z80, 2, 0b100, 0x54, &mut cpu_z); // BIT 2,H (CB 54)
    bit_pba(&mut z80, 3, 0b100, 0x5C, &mut cpu_z); // BIT 3,H (CB 5C)
    bit_pba(&mut z80, 4, 0b100, 0x64, &mut cpu_z); // BIT 4,H (CB 64)
    bit_pba(&mut z80, 5, 0b100, 0x6C, &mut cpu_z); // BIT 5,H (CB 6C)
    bit_pba(&mut z80, 6, 0b100, 0x74, &mut cpu_z); // BIT 6,H (CB 74)
    bit_pba(&mut z80, 7, 0b100, 0x7C, &mut cpu_z); // BIT 7,H (CB 7C)

    // BIT para L (0b101)
    bit_pba(&mut z80, 0, 0b101, 0x45, &mut cpu_z); // BIT 0,L (CB 45)
    bit_pba(&mut z80, 1, 0b101, 0x4D, &mut cpu_z); // BIT 1,L (CB 4D)
    bit_pba(&mut z80, 2, 0b101, 0x55, &mut cpu_z); // BIT 2,L (CB 55)
    bit_pba(&mut z80, 3, 0b101, 0x5D, &mut cpu_z); // BIT 3,L (CB 5D)
    bit_pba(&mut z80, 4, 0b101, 0x65, &mut cpu_z); // BIT 4,L (CB 65)
    bit_pba(&mut z80, 5, 0b101, 0x6D, &mut cpu_z); // BIT 5,L (CB 6D)
    bit_pba(&mut z80, 6, 0b101, 0x75, &mut cpu_z); // BIT 6,L (CB 75)
    bit_pba(&mut z80, 7, 0b101, 0x7D, &mut cpu_z); // BIT 7,L (CB 7D)

    // BIT para A (0b111)
    bit_pba(&mut z80, 0, 0b111, 0x47, &mut cpu_z); // BIT 0,A (CB 47)
    bit_pba(&mut z80, 1, 0b111, 0x4F, &mut cpu_z); // BIT 1,A (CB 4F)
    bit_pba(&mut z80, 2, 0b111, 0x57, &mut cpu_z); // BIT 2,A (CB 57)
    bit_pba(&mut z80, 3, 0b111, 0x5F, &mut cpu_z); // BIT 3,A (CB 5F)
    bit_pba(&mut z80, 4, 0b111, 0x67, &mut cpu_z); // BIT 4,A (CB 67)
    bit_pba(&mut z80, 5, 0b111, 0x6F, &mut cpu_z); // BIT 5,A (CB 6F)
    bit_pba(&mut z80, 6, 0b111, 0x77, &mut cpu_z); // BIT 6,A (CB 77)
    bit_pba(&mut z80, 7, 0b111, 0x7F, &mut cpu_z); // BIT 7,A (CB 7F)
}

#[test]
fn reset_bit0_b() {
    // 0x80 no afecta flags res 0,b
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.b = 0b0100_0001;
    c.reg.b = 0b0100_0001;

    ejecutar(&mut z80, 0xCB, 0x80, 0, 0, &mut c);
    // comprobar que bit 0 de b ahora es 0:
    assert_eq!(z80.reg.b, 0b0100_0000);
}

#[test]
fn reset_bit0_c() {
    // 0x81 no afecta flags res 0,b
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.c = 0b0100_0001;
    c.reg.c = 0b0100_0001;

    ejecutar(&mut z80, 0xCB, 0x81, 0, 0, &mut c);
    // comprobar que bit 0 de b ahora es 0:
    assert_eq!(z80.reg.c, 0b0100_0000);
}
// B=000 C=001 D=010 E=011 H=100 L=101 A=111
#[test]
fn reset_bit() {
    let mut z80 = Z80::default();
    let mut cpu_z = CPU::new(0xFFFF);
    // B=000 C=001 D=010 E=011 H=100 L=101 A=111
    // RES para B (0b000)
    bit_reset(&mut z80, 0, 0b000, 0x80, &mut cpu_z);  // RES 0,B (CB 80)
    bit_reset(&mut z80, 1, 0b000, 0x88, &mut cpu_z);  // RES 1,B (CB 88)
    bit_reset(&mut z80, 2, 0b000, 0x90, &mut cpu_z);  // RES 2,B (CB 90)
    bit_reset(&mut z80, 3, 0b000, 0x98, &mut cpu_z);  // RES 3,B (CB 98)
    bit_reset(&mut z80, 4, 0b000, 0xA0, &mut cpu_z);  // RES 4,B (CB A0)
    bit_reset(&mut z80, 5, 0b000, 0xA8, &mut cpu_z);  // RES 5,B (CB A8)
    bit_reset(&mut z80, 6, 0b000, 0xB0, &mut cpu_z);  // RES 6,B (CB B0)
    bit_reset(&mut z80, 7, 0b000, 0xB8, &mut cpu_z);  // RES 7,B (CB B8)

    // RES para C (0b001)
    bit_reset(&mut z80, 0, 0b001, 0x81, &mut cpu_z);  // RES 0,C (CB 81)
    bit_reset(&mut z80, 1, 0b001, 0x89, &mut cpu_z);  // RES 1,C (CB 89)
    bit_reset(&mut z80, 2, 0b001, 0x91, &mut cpu_z);  // RES 2,C (CB 91)
    bit_reset(&mut z80, 3, 0b001, 0x99, &mut cpu_z);  // RES 3,C (CB 99)
    bit_reset(&mut z80, 4, 0b001, 0xA1, &mut cpu_z);  // RES 4,C (CB A1)
    bit_reset(&mut z80, 5, 0b001, 0xA9, &mut cpu_z);  // RES 5,C (CB A9)
    bit_reset(&mut z80, 6, 0b001, 0xB1, &mut cpu_z);  // RES 6,C (CB B1)
    bit_reset(&mut z80, 7, 0b001, 0xB9, &mut cpu_z);  // RES 7,C (CB B9)

    // RES para D (0b010)
    bit_reset(&mut z80, 0, 0b010, 0x82, &mut cpu_z);  // RES 0,D (CB 82)
    bit_reset(&mut z80, 1, 0b010, 0x8A, &mut cpu_z);  // RES 1,D (CB 8A)
    bit_reset(&mut z80, 2, 0b010, 0x92, &mut cpu_z);  // RES 2,D (CB 92)
    bit_reset(&mut z80, 3, 0b010, 0x9A, &mut cpu_z);  // RES 3,D (CB 9A)
    bit_reset(&mut z80, 4, 0b010, 0xA2, &mut cpu_z);  // RES 4,D (CB A2)
    bit_reset(&mut z80, 5, 0b010, 0xAA, &mut cpu_z);  // RES 5,D (CB AA)
    bit_reset(&mut z80, 6, 0b010, 0xB2, &mut cpu_z);  // RES 6,D (CB B2)
    bit_reset(&mut z80, 7, 0b010, 0xBA, &mut cpu_z);  // RES 7,D (CB BA)

    // RES para E (0b011)
    bit_reset(&mut z80, 0, 0b011, 0x83, &mut cpu_z);  // RES 0,E (CB 83)
    bit_reset(&mut z80, 1, 0b011, 0x8B, &mut cpu_z);  // RES 1,E (CB 8B)
    bit_reset(&mut z80, 2, 0b011, 0x93, &mut cpu_z);  // RES 2,E (CB 93)
    bit_reset(&mut z80, 3, 0b011, 0x9B, &mut cpu_z);  // RES 3,E (CB 9B)
    bit_reset(&mut z80, 4, 0b011, 0xA3, &mut cpu_z);  // RES 4,E (CB A3)
    bit_reset(&mut z80, 5, 0b011, 0xAB, &mut cpu_z);  // RES 5,E (CB AB)
    bit_reset(&mut z80, 6, 0b011, 0xB3, &mut cpu_z);  // RES 6,E (CB B3)
    bit_reset(&mut z80, 7, 0b011, 0xBB, &mut cpu_z);  // RES 7,E (CB BB)

    // RES para H (0b100)
    bit_reset(&mut z80, 0, 0b100, 0x84, &mut cpu_z);  // RES 0,H (CB 84)
    bit_reset(&mut z80, 1, 0b100, 0x8C, &mut cpu_z);  // RES 1,H (CB 8C)
    bit_reset(&mut z80, 2, 0b100, 0x94, &mut cpu_z);  // RES 2,H (CB 94)
    bit_reset(&mut z80, 3, 0b100, 0x9C, &mut cpu_z);  // RES 3,H (CB 9C)
    bit_reset(&mut z80, 4, 0b100, 0xA4, &mut cpu_z);  // RES 4,H (CB A4)
    bit_reset(&mut z80, 5, 0b100, 0xAC, &mut cpu_z);  // RES 5,H (CB AC)
    bit_reset(&mut z80, 6, 0b100, 0xB4, &mut cpu_z);  // RES 6,H (CB B4)
    bit_reset(&mut z80, 7, 0b100, 0xBC, &mut cpu_z);  // RES 7,H (CB BC)

    // RES para L (0b101)
    bit_reset(&mut z80, 0, 0b101, 0x85, &mut cpu_z);  // RES 0,L (CB 85)
    bit_reset(&mut z80, 1, 0b101, 0x8D, &mut cpu_z);  // RES 1,L (CB 8D)
    bit_reset(&mut z80, 2, 0b101, 0x95, &mut cpu_z);  // RES 2,L (CB 95)
    bit_reset(&mut z80, 3, 0b101, 0x9D, &mut cpu_z);  // RES 3,L (CB 9D)
    bit_reset(&mut z80, 4, 0b101, 0xA5, &mut cpu_z);  // RES 4,L (CB A5)
    bit_reset(&mut z80, 5, 0b101, 0xAD, &mut cpu_z);  // RES 5,L (CB AD)
    bit_reset(&mut z80, 6, 0b101, 0xB5, &mut cpu_z);  // RES 6,L (CB B5)
    bit_reset(&mut z80, 7, 0b101, 0xBD, &mut cpu_z);  // RES 7,L (CB BD)

    // RES para A (0b111)
    bit_reset(&mut z80, 0, 0b111, 0x87, &mut cpu_z);  // RES 0,A (CB 87)
    bit_reset(&mut z80, 1, 0b111, 0x8F, &mut cpu_z);  // RES 1,A (CB 8F)
    bit_reset(&mut z80, 2, 0b111, 0x97, &mut cpu_z);  // RES 2,A (CB 97)
    bit_reset(&mut z80, 3, 0b111, 0x9F, &mut cpu_z);  // RES 3,A (CB 9F)
    bit_reset(&mut z80, 4, 0b111, 0xA7, &mut cpu_z);  // RES 4,A (CB A7)
    bit_reset(&mut z80, 5, 0b111, 0xAF, &mut cpu_z);  // RES 5,A (CB AF)
    bit_reset(&mut z80, 6, 0b111, 0xB7, &mut cpu_z);  // RES 6,A (CB B7)
    bit_reset(&mut z80, 7, 0b111, 0xBF, &mut cpu_z);  // RES 7,A (CB BF)
}

#[test]
fn set_bit() {
    let mut z80 = Z80::default();
    let mut cpu_z = CPU::new(0xFFFF);
    // B=000 C=001 D=010 E=011 H=100 L=101 A=111
    // SET para B (0b000)
    bit_set(&mut z80, 0, 0b000, 0xC0, &mut cpu_z);  // SET 0,B (CB C0)
    bit_set(&mut z80, 1, 0b000, 0xC8, &mut cpu_z);  // SET 1,B (CB C8)
    bit_set(&mut z80, 2, 0b000, 0xD0, &mut cpu_z);  // SET 2,B (CB D0)
    bit_set(&mut z80, 3, 0b000, 0xD8, &mut cpu_z);  // SET 3,B (CB D8)
    bit_set(&mut z80, 4, 0b000, 0xE0, &mut cpu_z);  // SET 4,B (CB E0)
    bit_set(&mut z80, 5, 0b000, 0xE8, &mut cpu_z);  // SET 5,B (CB E8)
    bit_set(&mut z80, 6, 0b000, 0xF0, &mut cpu_z);  // SET 6,B (CB F0)
    bit_set(&mut z80, 7, 0b000, 0xF8, &mut cpu_z);  // SET 7,B (CB F8)

    // SET para C (0b001)
    bit_set(&mut z80, 0, 0b001, 0xC1, &mut cpu_z);  // SET 0,C (CB C1)
    bit_set(&mut z80, 1, 0b001, 0xC9, &mut cpu_z);  // SET 1,C (CB C9)
    bit_set(&mut z80, 2, 0b001, 0xD1, &mut cpu_z);  // SET 2,C (CB D1)
    bit_set(&mut z80, 3, 0b001, 0xD9, &mut cpu_z);  // SET 3,C (CB D9)
    bit_set(&mut z80, 4, 0b001, 0xE1, &mut cpu_z);  // SET 4,C (CB E1)
    bit_set(&mut z80, 5, 0b001, 0xE9, &mut cpu_z);  // SET 5,C (CB E9)
    bit_set(&mut z80, 6, 0b001, 0xF1, &mut cpu_z);  // SET 6,C (CB F1)
    bit_set(&mut z80, 7, 0b001, 0xF9, &mut cpu_z);  // SET 7,C (CB F9)

    // SET para D (0b010)
    bit_set(&mut z80, 0, 0b010, 0xC2, &mut cpu_z);  // SET 0,D (CB C2)
    bit_set(&mut z80, 1, 0b010, 0xCA, &mut cpu_z);  // SET 1,D (CB CA)
    bit_set(&mut z80, 2, 0b010, 0xD2, &mut cpu_z);  // SET 2,D (CB D2)
    bit_set(&mut z80, 3, 0b010, 0xDA, &mut cpu_z);  // SET 3,D (CB DA)
    bit_set(&mut z80, 4, 0b010, 0xE2, &mut cpu_z);  // SET 4,D (CB E2)
    bit_set(&mut z80, 5, 0b010, 0xEA, &mut cpu_z);  // SET 5,D (CB EA)
    bit_set(&mut z80, 6, 0b010, 0xF2, &mut cpu_z);  // SET 6,D (CB F2)
    bit_set(&mut z80, 7, 0b010, 0xFA, &mut cpu_z);  // SET 7,D (CB FA)

    // SET para E (0b011)
    bit_set(&mut z80, 0, 0b011, 0xC3, &mut cpu_z);  // SET 0,E (CB C3)
    bit_set(&mut z80, 1, 0b011, 0xCB, &mut cpu_z);  // SET 1,E (CB CB)
    bit_set(&mut z80, 2, 0b011, 0xD3, &mut cpu_z);  // SET 2,E (CB D3)
    bit_set(&mut z80, 3, 0b011, 0xDB, &mut cpu_z);  // SET 3,E (CB DB)
    bit_set(&mut z80, 4, 0b011, 0xE3, &mut cpu_z);  // SET 4,E (CB E3)
    bit_set(&mut z80, 5, 0b011, 0xEB, &mut cpu_z);  // SET 5,E (CB EB)
    bit_set(&mut z80, 6, 0b011, 0xF3, &mut cpu_z);  // SET 6,E (CB F3)
    bit_set(&mut z80, 7, 0b011, 0xFB, &mut cpu_z);  // SET 7,E (CB FB)

    // SET para H (0b100)
    bit_set(&mut z80, 0, 0b100, 0xC4, &mut cpu_z);  // SET 0,H (CB C4)
    bit_set(&mut z80, 1, 0b100, 0xCC, &mut cpu_z);  // SET 1,H (CB CC)
    bit_set(&mut z80, 2, 0b100, 0xD4, &mut cpu_z);  // SET 2,H (CB D4)
    bit_set(&mut z80, 3, 0b100, 0xDC, &mut cpu_z);  // SET 3,H (CB DC)
    bit_set(&mut z80, 4, 0b100, 0xE4, &mut cpu_z);  // SET 4,H (CB E4)
    bit_set(&mut z80, 5, 0b100, 0xEC, &mut cpu_z);  // SET 5,H (CB EC)
    bit_set(&mut z80, 6, 0b100, 0xF4, &mut cpu_z);  // SET 6,H (CB F4)
    bit_set(&mut z80, 7, 0b100, 0xFC, &mut cpu_z);  // SET 7,H (CB FC)

    // SET para L (0b101)
    bit_set(&mut z80, 0, 0b101, 0xC5, &mut cpu_z);  // SET 0,L (CB C5)
    bit_set(&mut z80, 1, 0b101, 0xCD, &mut cpu_z);  // SET 1,L (CB CD)
    bit_set(&mut z80, 2, 0b101, 0xD5, &mut cpu_z);  // SET 2,L (CB D5)
    bit_set(&mut z80, 3, 0b101, 0xDD, &mut cpu_z);  // SET 3,L (CB DD)
    bit_set(&mut z80, 4, 0b101, 0xE5, &mut cpu_z);  // SET 4,L (CB E5)
    bit_set(&mut z80, 5, 0b101, 0xED, &mut cpu_z);  // SET 5,L (CB ED)
    bit_set(&mut z80, 6, 0b101, 0xF5, &mut cpu_z);  // SET 6,L (CB F5)
    bit_set(&mut z80, 7, 0b101, 0xFD, &mut cpu_z);  // SET 7,L (CB FD)

    // SET para A (0b111)
    bit_set(&mut z80, 0, 0b111, 0xC7, &mut cpu_z);  // SET 0,A (CB C7)
    bit_set(&mut z80, 1, 0b111, 0xCF, &mut cpu_z);  // SET 1,A (CB CF)
    bit_set(&mut z80, 2, 0b111, 0xD7, &mut cpu_z);  // SET 2,A (CB D7)
    bit_set(&mut z80, 3, 0b111, 0xDF, &mut cpu_z);  // SET 3,A (CB DF)
    bit_set(&mut z80, 4, 0b111, 0xE7, &mut cpu_z);  // SET 4,A (CB E7)
    bit_set(&mut z80, 5, 0b111, 0xEF, &mut cpu_z);  // SET 5,A (CB EF)
    bit_set(&mut z80, 6, 0b111, 0xF7, &mut cpu_z);  // SET 6,A (CB F7)
    bit_set(&mut z80, 7, 0b111, 0xFF, &mut cpu_z);  // SET 7,A (CB FF)
}



