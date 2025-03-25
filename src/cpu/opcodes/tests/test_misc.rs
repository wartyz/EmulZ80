use crate::cpu::opcodes::tests::test_aux::*;
//use crate::ops::Local16::Reg;
use crate::ops::StatusFlag;
use crate::z80;
use crate::z80::Z80;
use zilog_z80::cpu::CPU;

#[test]
fn in_b0c0() {
    // 0xED 40  in b (c)   Carry no afectado, N=0, PV detecta paridad, H=0, A,S se definen
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.debug.debug_zilog = false;

    //z80.registros.b = 0x10; // indica que destino es el registro B
    z80.reg.c = 0xFE; // indica el dispositivo de entrada

    // El dispositivo de entrada tiene estos datos
    let inp = z80::io::BufInput::new(vec!(0xC3, 0xA2, 0xB8));

    // Instalo un device de entrada en 0xFE
    z80.install_input(0xFE, Box::new(inp.clone()));

    ejecutar(&mut z80, 0xED, 0x40, 0, 0, &mut c);

    assert_eq!((inp.input.borrow_mut()[0]), 0xC3);
    assert_eq!((inp.input.borrow_mut()[1]), 0xA2);

    // Comprobamos A que se ha sacado del BufInput
    assert_eq!(z80.reg.b, 0xB8);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 2);
}
#[test]
fn out0c0_b() {
    // 0xED 0x41   out(c),b No afecta a los flags. El valor de B es escrito al puerto cuya
    // direccion esta formada por B en los bits altos y C en los bits bajos

    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    //z80.registros.d = 0xE6;
    //z80.registros.b = 0x60;  // 96 base 10
    //z80.registros.c = 0xFE; // indica el dispositivo de salida

    set_bc_test_big(&mut z80, &mut c, 0x60FE);
    let out = z80::io::BufOutput::default();

    // Instalo un device de salida en 0xFE
    z80.install_output(0xFE, Box::new(out.clone()));
    // Ahora se podra usar `OUT (0xFE), <registro>`.

    ejecutar(&mut z80, 0xED, 0x41, 0, 0, &mut c);

    for (clave, valor) in z80.output_devices {
        println!("clave->{:2X}  valor->{:?}", clave, valor);
    }
}
#[test]
fn sbc_hl_bc() {
    // 0xED 42  sbc hl,bc,  hl - bc - carry -> hl   Carry se define, N=1, PV detecta overflow, H,Z,S se definen
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Caso 1: Resta sin acarreo ni overflow resultante y acarreo anterior = 0
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;

    //z80.registros.set_reg8(H, 0xCD);
    //z80.registros.set_reg8(L, 0x04);
    set_hl_test_big(&mut z80, &mut c, 0xCD04);
    //z80.registros.set_reg8(B, 0x12);
    //z80.registros.set_reg8(C, 0x01);
    set_bc_test_big(&mut z80, &mut c, 0x1201);

    ejecutar(&mut z80, 0xED, 0x42, 0, 0, &mut c);

    assert_eq!(z80.reg.h, 0xBB, "Valor en H incorrecto");
    assert_eq!(z80.reg.l, 0x03, "Valor en L incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 1, 0);

    // Caso 2: Resta con acarreo anterior = 1
    z80.reg.pc = 0;  // Resetea contador de programa
    c.reg.pc = 0;
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;

    //z80.registros.set_reg8(H, 0x1D);
    //z80.registros.set_reg8(L, 0xA4);
    set_hl_test_big(&mut z80, &mut c, 0x1DA4);
    //z80.registros.set_reg8(B, 0x1D);
    //z80.registros.set_reg8(C, 0xF2);
    set_bc_test_big(&mut z80, &mut c, 0x1DF2);

    ejecutar(&mut z80, 0xED, 0x42, 0, 0, &mut c);

    assert_eq!(z80.reg.h, 0xFF, "Valor en H incorrecto");
    assert_eq!(z80.reg.l, 0xB1, "Valor en L incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);
}
// #[test]
// fn ld_0nn0hl() {
//     // 0x22
//     let mut z80 = Z80::default(); let mut c=CPU::new(0xFFFF);
//
//     //z80.registros.set_reg16_lit_endian(&HL, 0xABCD);
//     z80.registros.h = 0xCD;
//     z80.registros.l = 0xAB;
//     ejecutar(&mut z80, 0x22, 0x34, 0x12, 0, &mut c);
//
//     // compruebo que en la dirección 0x1234 se ha guardado 0xAB
//     assert_eq!(z80.memoria.memoria[0x1234], 0xAB);
//     assert_eq!(z80.memoria.memoria[0x1235], 0xCD);
// }
#[test]
fn ld_0nn0_bc() {
    // ED 43 ld (nn),bc No afecta flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    //z80.registros.b = 0xCD;
    //z80.registros.c = 0xAB;
    set_bc_test_big(&mut z80, &mut c, 0xCDAB);

    ejecutar(&mut z80, 0xED, 0x43, 0x34, 0x12, &mut c);

    // compruebo que en la dirección 0x1234 se ha guardado 0xABCD
    assert_eq!(z80.mem.mem[0x1234], 0xAB);
    assert_eq!(z80.mem.mem[0x1235], 0xCD);

    assert_eq!(c.bus.read_byte(0x1234), 0xAB);
    assert_eq!(c.bus.read_byte(0x1235), 0xCD);
}
#[test]
fn neg() {
    // ED 44  A es negado y sumado 1(complemento a 2),es lo mismo que si hicieramos 0-A
    // c es = 1 si A no tenia 0 antes de la operacion,
    // n=1 pv detecta overflow h del bit 4, z, s se definen
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0b1001_1000; // 0x98 152d
    c.reg.a = 0b1001_1000;

    ejecutar(&mut z80, 0xED, 0x44, 0, 0, &mut c);

    assert_eq!(z80.reg.a, 0b0110_1000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 0, 1, 1);
}
#[test]
fn ret_n() {
    // ED 45 retn   retorna de una interrupcion no enmascarable
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F simulando una interrupcion no enmascarable
    z80.reg.pc = 0xA24F;
    c.reg.pc = 0xA24F;
    // haciendo un call a 0x0066
    hace_un_call_incondicional_a_direcc_dada(&mut z80, 0x00, 0x66, &mut c);

    // Ejecuto ret_n en 0x00AA
    ejecutar_en_direccion(&mut z80, 0x0066, 0xED, 0x45, 0, 0, &mut c);

    // Comprobar que el PC ha retornado a la siguiente direccion
    assert_eq!(z80.reg.pc, 0xA252);
}

#[test]
fn im_0() {
    // ED 46 Pone el modo de interrupcion 0
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    ejecutar(&mut z80, 0xED, 0x46, 0, 0, &mut c);

    // Comprobamos que modo de interrupcion  es 0
    assert_eq!(z80.int, Some(0));
}

#[test]
fn ld_i_a() {
    // ED 47 ld i,a   almacena A en registro I
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.a = 0x5A;
    c.reg.a = 0x5A;

    ejecutar(&mut z80, 0xED, 0x47, 0, 0, &mut c);

    // compruebo que I es 0x5A
    assert_eq!(z80.reg.i, 0x5A);
}

#[test]
fn in_c0c0() {
    // 0xED 48  in c(c)   Carry no afectado, N=0, PV detecta paridad, H=0, Z,S se definen
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.debug.debug_zilog = false;

    //z80.registros.b = 0x10; // indica que destino es el registro B
    z80.reg.c = 0xFE; // indica el dispositivo de entrada

    // El dispositivo de entrada tiene estos datos
    let inp = z80::io::BufInput::new(vec!(0xC3, 0xA2, 0xB8));

    // Instalo un device de entrada en 0xFE
    z80.install_input(0xFE, Box::new(inp.clone()));

    ejecutar(&mut z80, 0xED, 0x48, 0, 0, &mut c);

    assert_eq!((inp.input.borrow_mut()[0]), 0xC3);
    assert_eq!((inp.input.borrow_mut()[1]), 0xA2);

    // Comprobamos A que se ha sacado del BufInput
    assert_eq!(z80.reg.c, 0xB8);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 2, 0, 0, 0, 0, 2);
}
#[test]
fn out0c0_c() {
    // 0xED 0x49   out(c),c No afecta a los flags. El valor de B es escrito al puerto cuya
    // direccion esta formada por B en los bits altos y C en los bits bajos

    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.debug.debug_zilog = false;
    //z80.registros.d = 0xE6;

    //z80.reg.b = 0x60;  // 96 base 10
    //z80.reg.c = 0xFE; // indica el dispositivo de salida (254 decimal)

    set_bc_test_big(&mut z80, &mut c, 0x60FE);
    let out = z80::io::BufOutput::default();

    // Instalo un device de salida en 0xFE
    z80.install_output(0xFE, Box::new(out.clone()));
    // Ahora se podra usar `OUT (0xFE), <registro>`.

    ejecutar(&mut z80, 0xED, 0x49, 0, 0, &mut c);

    for (clave, valor) in z80.output_devices {
        println!("clave->{:2X}  valor->{:?}", clave, valor);
    }
}
#[test]
fn adc_hl_bc() {
    // 0xED 0x4A Carry se define, N=0, PV detecta overflow, H, A, S se definen
    // HL + BC + Carry -> HL
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    //pongo Carry=1
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;

    // pongo HL = 0x3412
    //z80.reg.h = 0x34;
    //z80.reg.l = 0x12;

    set_hl_test_big(&mut z80, &mut c, 0x3412);

    // pongo BC = 0xB5A4
    //z80.reg.b = 0xB5;
    //z80.reg.c = 0xA4;

    set_bc_test_big(&mut z80, &mut c, 0xB5A4);

    ejecutar(&mut z80, 0xED, 0x4A, 0, 0, &mut c);

    // compruebo que ahora hl = 0xE9B7
    assert_eq!(z80.reg.h, 0xE9);
    assert_eq!(z80.reg.l, 0xB7);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 2, 2, 0, 2, 0, 0);
}

#[test]
fn ld_bc_0nn0() {
    // 0xED 0x4B No afecta flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.mem.mem[0x1234] = 0xAB;
    z80.mem.mem[0x1235] = 0xCD;
    c.bus.write_byte(0x1234, 0xAB);
    c.bus.write_byte(0x1235, 0xCD);

    ejecutar(&mut z80, 0xED, 0x4B, 0x34, 0x12, &mut c);

    // compruebo que en la dirección 0x1234 se ha guardado 0xABCD
    assert_eq!(z80.reg.c, 0xAB);
    assert_eq!(z80.reg.b, 0xCD);
}

#[test]
fn mlt_bc() {
    /*// ED 4C   Intruccion no documentada   coge bc multiplica entre si b * c
    // y pone el resultado en BC. No afecta flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // caso 1 sin overflow
    //z80.reg.b = 0x04;
    //z80.reg.c = 0x10;
    set_bc_test(&mut z80, &mut c, 0x0410);
    ejecutar(&mut z80, 0xED, 0x4C, 0, 0, &mut c);

    assert_eq!(z80.reg.b, 0x00);
    assert_eq!(z80.reg.c, 0x40);

    // caso 2 con overflow
    z80.reg.pc = 0;
    //z80.reg.b = 0xE8;
    //z80.reg.c = 0xF5;
    set_bc_test(&mut z80, &mut c, 0xE8F5);

    ejecutar(&mut z80, 0xED, 0x4C, 0, 0, &mut c);
    //dbg_hex!(z80.reg.b);
    //dbg_hex!(z80.reg.c);

    assert_eq!(z80.reg.b, 0xDE);
    assert_eq!(z80.reg.c, 0x08);*/
}
#[test]
fn reti() {
    // ED 4D  Usada al final de un servicio de interrupcion enmascarable. No afecta flags
    // informa al dispositivo que ha provocado la interrupcion que esta ha acabado
    //   retorna haciendo un pop en el PC
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Establecemos PC como 0xA24F (Hacemos el call desde 0xA24F
    hace_un_call_incondicional_a_direcc_dada(&mut z80, 0xE0, 0x15, &mut c);

    // Ejecuto reti
    ejecutar_en_direccion(&mut z80, 0xE015, 0xED, 0x4D, 0, 0, &mut c);

    // Comprobar que el PC ha retornado a la siguiente direccion
    assert_eq!(z80.reg.pc, 0xA252);

    // TODO no se como comprobar lo de la interrupcion
}
#[test]
fn ld_r_a() {
    // ED 4F   A->R y no afecta a los flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // pongo A = 0xCD
    z80.reg.a = 0xCD;
    c.reg.a = 0xCD;
    ejecutar(&mut z80, 0xED, 0x4F, 0, 0, &mut c);

    // compruebo que R es 0xCD
    assert_eq!(z80.reg.r, 0xCD);
}
#[test]
fn in_d_0c0() {
    // 0xED 0x50  in d (c)   Carry no afectado, N=0, PV detecta paridad, H=0, A,S se definen
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.debug.debug_zilog = false;

    //z80.registros.b = 0x10; // indica que destino es el registro D
    z80.reg.c = 0xFE; // indica el dispositivo de entrada

    // El dispositivo de entrada tiene estos datos
    let inp = z80::io::BufInput::new(vec!(0xC3, 0xA2, 0xB8));

    // Instalo un device de entrada en 0xFE
    z80.install_input(0xFE, Box::new(inp.clone()));

    ejecutar(&mut z80, 0xED, 0x50, 0, 0, &mut c);

    assert_eq!((inp.input.borrow_mut()[0]), 0xC3);
    assert_eq!((inp.input.borrow_mut()[1]), 0xA2);

    // Comprobamos A que se ha sacado del BufInput
    assert_eq!(z80.reg.d, 0xB8);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 2);
}
#[test]
fn out_0c0_d() {
    // 0xED 0x51   out(c),d No afecta a los flags. El valor de D es escrito al puerto cuya
    // direccion esta formada por B en los bits altos y C en los bits bajos

    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.debug.debug_zilog = false;

    z80.reg.d = 0x60;  // 96 base 10
    z80.reg.c = 0xFE; // indica el dispositivo de entrada
    let out = z80::io::BufOutput::default();

    // Instalo un device de salida en 0xFE
    z80.install_output(0xFE, Box::new(out.clone()));
    // Ahora se podra usar `OUT (0xFE), <registro>`.

    ejecutar(&mut z80, 0xED, 0x51, 0, 0, &mut c);

    for (clave, valor) in z80.output_devices {
        println!("clave->{:2X}  valor->{:?}", clave, valor);
    }
}
#[test]
fn sbc_hl_de() {
    // 0xED 52  sbc hl,de,  hl - de - carry -> hl   Carry se define, N=1, PV detecta overflow, H,Z,S se definen
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Caso 1: Resta sin acarreo ni overflow resultante y acarreo anterior = 0
    z80.reg.set_flag(&StatusFlag::Carry, false);
    c.reg.flags.c = false;

    //z80.reg.set_reg8(H, 0xCD);
    //z80.reg.set_reg8(L, 0x04);
    set_hl_test_big(&mut z80, &mut c, 0xCD04);
    //z80.reg.set_reg8(D, 0x12);
    //z80.reg.set_reg8(E, 0x01);
    set_de_test_big(&mut z80, &mut c, 0x1201);

    ejecutar(&mut z80, 0xED, 0x52, 0, 0, &mut c);

    assert_eq!(z80.reg.h, 0xBB, "Valor en H incorrecto");
    assert_eq!(z80.reg.l, 0x03, "Valor en L incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 1, 0);

    // Caso 2: Resta con acarreo anterior = 1
    z80.reg.pc = 0;  // Resetea contador de programa
    c.reg.pc = 0;

    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;

    //z80.reg.set_reg8(H, 0x1D);
    //z80.reg.set_reg8(L, 0xA4);
    set_hl_test_big(&mut z80, &mut c, 0x1DA4);
    //z80.reg.set_reg8(D, 0x1D);
    //z80.reg.set_reg8(E, 0xF2);
    set_de_test_big(&mut z80, &mut c, 0x1DF2);

    ejecutar(&mut z80, 0xED, 0x52, 0, 0, &mut c);

    assert_eq!(z80.reg.h, 0xFF, "Valor en H incorrecto");
    assert_eq!(z80.reg.l, 0xB1, "Valor en L incorrecto");

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);
}

#[test]
fn ld_0nn0_de() {
    // ED 53 ld (nn),de No afecta flags
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // z80.reg.d = 0xCD;
    //z80.reg.e = 0xAB;

    set_de_test_big(&mut z80, &mut c, 0xCDAB);

    ejecutar(&mut z80, 0xED, 0x53, 0x34, 0x12, &mut c);

    // compruebo que en la dirección 0x1234 se ha guardado 0xABCD
    assert_eq!(z80.mem.mem[0x1234], 0xAB);
    assert_eq!(z80.mem.mem[0x1235], 0xCD);
}
#[test]
fn im_1() {
    // ED 56 Pone el modo de interrupcion 1
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    ejecutar(&mut z80, 0xED, 0x56, 0, 0, &mut c);

    // Comprobamos que modo de interrupcion  es 0
    assert_eq!(z80.int, Some(1));
}
#[test]
fn ld_a_i() {
    // ED 57 ld a,i   almacena en A el valor del registro I
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.i = 0x5A;
    c.reg.i = 0x5A;

    ejecutar(&mut z80, 0xED, 0x57, 0, 0, &mut c);

    // compruebo que A es 0x5A
    assert_eq!(z80.reg.a, 0x5A);
}

// Ed 58
#[test]
fn out_0c0_e() {
    // 0xED 0x59   out(c),e No afecta a los flags. El valor de B es escrito al puerto cuya
    // direccion esta formada por B en los bits altos y C en los bits bajos

    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.debug.debug_zilog = false;

    z80.reg.e = 0xE6;  // 230 d
    //z80.registros.b = 0x60;  // 96 d
    z80.reg.c = 0xFE;  // indica el dispositivo de salida (254 decimal)
    let out = z80::io::BufOutput::default();

    // Instalo un device de salida en 0xFE
    z80.install_output(0xFE, Box::new(out.clone()));
    // Ahora se podra usar `OUT (0xFE), <registro>`.

    ejecutar(&mut z80, 0xED, 0x59, 0, 0, &mut c);

    for (clave, valor) in z80.output_devices {
        println!("clave->{:2X}  valor->{:?}", clave, valor);
    }
}

#[test]
fn adc_hl_de() {
    // ED 5A Carry se define, N=0, PV detecta overflow, H, A, S se definen
    // HL + DE + Carry -> HL
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    //pongo Carry=1
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;

    // pongo HL = 0x3412
    //z80.reg.h = 0x34;
    //z80.reg.l = 0x12;
    set_hl_test_big(&mut z80, &mut c, 0x3412);
    // pongo BC = 0xB5A4
    //z80.reg.d = 0xB5;
    //z80.reg.e = 0xA4;
    set_de_test_big(&mut z80, &mut c, 0xB5A4);

    ejecutar(&mut z80, 0xED, 0x5A, 0, 0, &mut c);

    // compruebo que ahora hl = 0xE9B7
    assert_eq!(z80.reg.h, 0xE9);
    assert_eq!(z80.reg.l, 0xB7);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 2, 2, 0, 2, 0, 0);
}

#[test]
fn ld_de_0nn0() {
    // ED 5B FLAGS NO AFECTADOS
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // pongo en memoria datos
    z80.mem.mem[0x1234] = 0xD4;
    z80.mem.mem[0x1235] = 0xE1;
    c.bus.write_byte(0x1234, 0xD4);
    c.bus.write_byte(0x1235, 0xE1);

    ejecutar(&mut z80, 0xED, 0x5B, 0x34, 0x12, &mut c);

    // compruebo que ahora de = 0xE1D4
    assert_eq!(z80.reg.d, 0xE1);
    assert_eq!(z80.reg.e, 0xD4);
}

#[test]
fn im_2() {
    // ED 5E Pone el modo de interrupcion 2
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    ejecutar(&mut z80, 0xED, 0x5E, 0, 0, &mut c);

    // Comprobamos que modo de interrupcion  es 0
    assert_eq!(z80.int, Some(2));
}
#[test]
fn ld_a_r() {
    // ED 5F   A ← R
    // Carry no afectado, S=1 si R es negativo  Z is set si R=0  H=0  PV contiene el contenido de IFF2  N=0
    // Si ocurre una interrupcion durante la ejecucion de esta instruccion el flag parity=0
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // pongo R = 0xCD
    z80.reg.r = 0xCD;
    c.reg.r = 0xCD;

    ejecutar(&mut z80, 0xED, 0x5F, 0, 0, &mut c);

    // compruebo que L es 0xCD
    assert_eq!(z80.reg.a, 0xCD);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 1, 0, 0, 0, 0, 2);
}

#[test]
fn in_h_0c0() {
    // 0xED 60  in h (c)   Carry no afectado, N=0, PV detecta paridad, H=0, A,S se definen
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.debug.debug_zilog = false;

    //z80.registros.b = 1; // indica que destino es el registro D
    z80.reg.c = 0xFE; // indica el dispositivo de entrada

    // El dispositivo de entrada tiene estos datos
    let inp = z80::io::BufInput::new(vec!(0xC3, 0xA2, 0xB8));

    // Instalo un device de entrada en 0xFE
    z80.install_input(0xFE, Box::new(inp.clone()));

    ejecutar(&mut z80, 0xED, 0x60, 0, 0, &mut c);

    assert_eq!((inp.input.borrow_mut()[0]), 0xC3);
    assert_eq!((inp.input.borrow_mut()[1]), 0xA2);

    // Comprobamos A que se ha sacado del BufInput
    assert_eq!(z80.reg.h, 0xB8);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 0, 0, 2);
}
#[test]
fn out_0c0_h() {
    // 0xED 0x61   out(c),d No afecta a los flags. El valor de D es escrito al puerto cuya
    // direccion esta formada por B en los bits altos y C en los bits bajos

    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.debug.debug_zilog = false;

    z80.reg.h = 0x60;  // 96 base 10
    z80.reg.c = 0xFE; // indica el dispositivo de entrada
    let out = z80::io::BufOutput::default();

    // Instalo un device de salida en 0xFE
    z80.install_output(0xFE, Box::new(out.clone()));
    // Ahora se podra usar `OUT (0xFE), <registro>`.

    ejecutar(&mut z80, 0xED, 0x61, 0, 0, &mut c);

    for (clave, valor) in z80.output_devices {
        println!("clave->{:2X}  valor->{:?}", clave, valor);
    }
}
// ED 62
#[test]
fn ld_0nn0_hl() {
    // ED 63 FLAGS NO AFECTADOS
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    //z80.reg.h = 0x60;
    //z80.reg.l = 0xFE;
    set_hl_test_big(&mut z80, &mut c, 0x60FE);

    ejecutar(&mut z80, 0xED, 0x63, 0x34, 0x12, &mut c);

    // compruebo que ahora la memoria en 0x1234
    // pongo en memoria datos
    assert_eq!(z80.mem.mem[0x1234], 0xFE);
    assert_eq!(z80.mem.mem[0x1235], 0x60);
}
// ED 64
// ED 67
// ED 68
#[test]
fn out0c0_l() {
    // 0xED 0x69   out(c),e No afecta a los flags. El valor de B es escrito al puerto cuya
    // direccion esta formada por B en los bits altos y C en los bits bajos

    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    z80.debug.debug_zilog = false;

    z80.reg.l = 0xE6;  // 230 d
    //z80.registros.b = 0x60;  // 96 d
    z80.reg.c = 0xFE;  // indica el dispositivo de salida (254 decimal)
    let out = z80::io::BufOutput::default();

    // Instalo un device de salida en 0xFE
    z80.install_output(0xFE, Box::new(out.clone()));
    // Ahora se podra usar `OUT (0xFE), <registro>`.

    ejecutar(&mut z80, 0xED, 0x69, 0, 0, &mut c);

    for (clave, valor) in z80.output_devices {
        println!("clave->{:2X}  valor->{:?}", clave, valor);
    }
}
#[test]
fn adc_hl_hl() {
    // ED 6A Carry se define, N=0, PV detecta overflow, H, A, S se definen
    // HL + DE + Carry -> HL
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    //pongo Carry=1
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;

    // pongo HL = 0x3412
    //z80.reg.h = 0x34;
    //z80.reg.l = 0x12;
    set_hl_test_big(&mut z80, &mut c, 0x3412);

    ejecutar(&mut z80, 0xED, 0x6A, 0, 0, &mut c);

    // compruebo que ahora hl = 0x6824
    assert_eq!(z80.reg.h, 0x68);
    assert_eq!(z80.reg.l, 0x25);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 2, 2, 0, 2, 0, 0);
}
#[test]
fn ld_hl_0nn0() {
    // ED 6B FLAGS NO AFECTADOS
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // pongo en memoria datos
    z80.mem.mem[0x1234] = 0xD4;
    z80.mem.mem[0x1235] = 0xE1;
    c.bus.write_byte(0x1234, 0xD4);
    c.bus.write_byte(0x1235, 0xE1);

    ejecutar(&mut z80, 0xED, 0x6B, 0x34, 0x12, &mut c);

    // compruebo que ahora hl = 0xE1D4
    assert_eq!(z80.reg.h, 0xE1);
    assert_eq!(z80.reg.l, 0xD4);
}
// ED 6F
// ED 72
#[test]
fn ld0nn0_sp() {
    // ED 73 FLAGS NO AFECTADOS
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.reg.sp = 0x60FE;
    c.reg.sp = 0x60FE;

    ejecutar(&mut z80, 0xED, 0x73, 0x34, 0x12, &mut c);

    // compruebo que ahora la memoria en 0x1234
    // pongo en memoria datos
    assert_eq!(z80.mem.mem[0x1234], 0xFE);
    assert_eq!(z80.mem.mem[0x1235], 0x60);
}
// ED 78
#[test]
fn out0c0_a() {
    // 0xED 0x79   out(c),a No afecta a los flags. El valor de B es escrito al puerto cuya
    // direccion esta formada por B en los bits altos y C en los bits bajos

    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    z80.debug.debug_zilog = false;

    z80.reg.a = 0xE6;  // 230 d
    //z80.registros.b = 0x60;  // 96 d
    z80.reg.c = 0xFE;  // indica el dispositivo de salida (254 decimal)
    let out = z80::io::BufOutput::default();

    // Instalo un device de salida en 0xFE
    z80.install_output(0xFE, Box::new(out.clone()));
    // Ahora se podra usar `OUT (0xFE), <registro>`.

    ejecutar(&mut z80, 0xED, 0x79, 0, 0, &mut c);

    for (clave, valor) in z80.output_devices {
        println!("clave->{:2X}  valor->{:?}", clave, valor);
    }
}

#[test]
fn adc_hl_sp() {
    // ED 7A Carry se define, N=0, PV detecta overflow, H, A, S se definen
    // HL + SP + Carry -> HL
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    //pongo Carry=1
    z80.reg.set_flag(&StatusFlag::Carry, true);
    c.reg.flags.c = true;

    //z80.reg.h = 0x34;
    //z80.reg.l = 0x12;
    set_hl_test_big(&mut z80, &mut c, 0x3412);

    // pongo SP = 0xB5A4
    z80.reg.sp = 0xB5A4;
    c.reg.sp = 0xB5A4;

    ejecutar(&mut z80, 0xED, 0x7A, 0, 0, &mut c);

    // compruebo que ahora hl = 0xE9B7
    assert_eq!(z80.reg.h, 0xE9);
    assert_eq!(z80.reg.l, 0xB7);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 2, 2, 0, 2, 0, 0);
}

#[test]
fn ld_sp_0nn0() {
    // ED 7B FLAGS NO AFECTADOS
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // pongo en memoria datos
    z80.mem.mem[0x1234] = 0xD4;
    z80.mem.mem[0x1235] = 0xE1;
    c.bus.write_byte(0x1234, 0xD4);
    c.bus.write_byte(0x1235, 0xE1);

    ejecutar(&mut z80, 0xED, 0x7B, 0x34, 0x12, &mut c);

    // compruebo que ahora sp = 0xE1D4
    assert_eq!(z80.reg.sp, 0xE1D4);
}
#[test]
fn ldi() {
    // ED A0 (DE) <-(HL)   DE <- DE+1  HÑ <- HL+1   BC < BC-1
    // Carry no afectado N=0    H=0 Z,S no afectados
    // P/V=1 si BC-1 != 0

    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Ponemos el 0x1B valor en la direccion 0xE345
    z80.mem.mem[0xE345] = 0x1B;
    c.bus.write_byte(0xE345, 0x1B);

    // Hacemos que HL apunte a esa direccion de memoria
    //z80.reg.h = 0xE3;
    //z80.reg.l = 0x45;
    set_hl_test_big(&mut z80, &mut c, 0xE345);

    // Hacemos que DE apunte a otra direccion de memoria
    //z80.reg.d = 0x56;
    //z80.reg.e = 0x7B;
    set_de_test_big(&mut z80, &mut c, 0x567B);

    // Ponemos a BC un valor (se usa normalmente como contador que se decrementa)
    //z80.reg.b = 0x12;
    //z80.reg.c = 0xD1;
    set_bc_test_big(&mut z80, &mut c, 0x12D1);

    ejecutar(&mut z80, 0xED, 0xA0, 0, 0, &mut c);

    // Comprobamos que ahora (DE)  <- (HL)
    assert_eq!(z80.mem.mem[0x567B], 0x1B);

    // Comprobamos que HL y DE se han incrementado y BC se ha decrementado
    assert_eq!(z80.reg.h, 0xE3);
    assert_eq!(z80.reg.l, 0x46);
    assert_eq!(z80.reg.d, 0x56);
    assert_eq!(z80.reg.e, 0x7C);
    assert_eq!(z80.reg.b, 0x12);
    assert_eq!(z80.reg.c, 0xD0);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 2, 2, 0, 1, 0, 2);
}
// ED 7B
#[test]
fn cpi() {
    // ED A1 A-(HL)    HL<-HL+1    BC<-BC-1
    // Carry no afectado    N=1     PV=1 si BC-1!=0     H=1 si hay borrow desde el bit 4
    // Z = 1 si A = (HL) ,S afetado por el signo

    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    z80.reg.a = 0x0C;
    c.reg.a = 0x0C;

    // pongo en la direccion 0xEF10 el valor 0x02
    z80.mem.mem[0xEF10] = 0x09;
    c.bus.write_byte(0xEF10, 0x09);

    // pongo HL = 0xEF10  y BC = 0x1234
    // z80.reg.h = 0xEF;
    // z80.reg.l = 0x10;
    set_hl_test_big(&mut z80, &mut c, 0xEF10);
    // z80.reg.b = 0x12;
    // z80.reg.c = 0x34;
    set_bc_test_big(&mut z80, &mut c, 0x1234);

    ejecutar(&mut z80, 0xED, 0xA1, 0, 0, &mut c);
    assert_eq!(z80.reg.a, 0x0C); // No debe cambiar
    assert_eq!(z80.reg.h, 0xEF);
    assert_eq!(z80.reg.l, 0x11);
    assert_eq!(z80.reg.b, 0x12);
    assert_eq!(z80.reg.c, 0x33);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 0, 1, 1, 2);
}

// ED A2
#[test]
pub fn outi() {
    // ED A3 (C) ← (HL), B ← B – 1, HL ← HL + 1
    // Z=1 si B – 1 = 0  N=1 Carry afectado, (no lo dice la documentacion)   H, PV desconocido

    // En C se almacena temporalmente lo que hay en (HL)
    // Luego, después de que B se decrementa, C se coloca en la mitad inferior (A0 a A7) del bus de direcciones
    // para seleccionar el dispositivo de E/S en uno de los 256 puertos posibles.
    // B se puede utilizar como un contador de bytes y su valor decrementado se
    // coloca en la mitad superior (A8 a A15) del bus de direcciones. El byte que se va a generar se coloca
    // en el bus de datos y se escribe en un dispositivo periférico seleccionado.
    // Finalmente, el par de registros HL se incrementa.

    // El valor de (HL) es escrito al puerto cuya
    // direccion esta formada por B-1 en los bits altos y C en los bits bajos

    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);
    z80.debug.debug_zilog = false;

    let out = z80::io::BufOutput::default();

    // Instalo un device de salida en 0xFE
    z80.install_output(0xFE, Box::new(out.clone()));
    // Ahora se podra usar `OUT (0xFE), <registro>`.

    // puerto de salida
    z80.reg.c = 0xFE;

    //z80.reg.h = 0x12;
    //z80.reg.l = 0x45;
    set_hl_test_big(&mut z80, &mut c, 0x1245);

    // Ponemos en (HL) 0xA1
    z80.mem.mem[0x1245] = 0xA1;

    ejecutar(&mut z80, 0xED, 0xA3, 0, 0, &mut c);

    assert_eq!(z80.reg.h, 0x12);
    assert_eq!(z80.reg.l, 0x46);

    for (clave, valor) in z80.output_devices {
        println!("clave->{:?}  valor->{:?}", clave, valor);
    }
}
// ED A8

#[test]
fn cpd() {
    // ED A9  Compara A con (HL)    HL <- HL-1, BC <- BC-1
    // S afectado    Z=1 si A = (HL)     H=1 si borrow del bit 4   P/V=1 si BC-1 != 0  N=1   C no afectado
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // Primer caso a = (HL)
    z80.mem.mem[0x2345] = 0x4B;
    c.bus.write_byte(0x2345, 0x4B);
    z80.reg.a = 0x4B;
    c.reg.a = 0x4B;
    set_hl_test_big(&mut z80, &mut c, 0x2345);
    set_bc_test_big(&mut z80, &mut c, 0xA043);

    ejecutar(&mut z80, 0xED, 0xA9, 0, 0, &mut c);

    assert_eq!(z80.reg.h, 0x23);
    assert_eq!(z80.reg.l, 0x44);
    assert_eq!(z80.reg.b, 0xA0);
    assert_eq!(z80.reg.c, 0x42);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 0, 1, 1, 2);

    // Segundo caso A!=(HL)
    z80.mem.mem[0x235B] = 0x2F;
    c.bus.write_byte(0x235B, 0x2F);
    z80.reg.a = 0x4B;
    c.reg.a = 0x4B;
    set_hl_test_big(&mut z80, &mut c, 0x235B);
    set_bc_test_big(&mut z80, &mut c, 0x45B1);

    z80.reg.pc = 0;
    c.reg.pc = 0;

    ejecutar(&mut z80, 0xED, 0xA9, 0, 0, &mut c);

    assert_eq!(z80.reg.h, 0x23);
    assert_eq!(z80.reg.l, 0x5A);
    assert_eq!(z80.reg.b, 0x45);
    assert_eq!(z80.reg.c, 0xB0);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 0, 1, 1, 1, 2);
}
// ED AA
// ED AB
#[test]
fn ldir() {
    // ED B0 repeat {(DE) ← (HL), DE ← DE + 1, HL ← HL + 1, BC ← BC – 1} while (BC ≠ 0)
    // C, S, Z no afectados    H=0  PV=0  N=0
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // pongo en la direccion 0xEF10 el valor 0x02  0xEF09 el valor 0xB1  0xEF08 el valor 0xA3
    z80.mem.mem[0xEF10] = 0x02;
    z80.mem.mem[0xEF09] = 0xB1;
    z80.mem.mem[0xEF08] = 0xA3;
    c.bus.write_byte(0xEF10, 0x02);
    c.bus.write_byte(0xEF09, 0xB1);
    c.bus.write_byte(0xEF08, 0xA3);

    // pongo HL = 0xEF10  y BC = 0x0003 y DE = 0x1234
    // z80.reg.h = 0xEF;
    // z80.reg.l = 0x10;
    set_hl_test_big(&mut z80, &mut c, 0xEF10);
    // z80.reg.d = 0x12;
    // z80.reg.e = 0x34;
    set_de_test_big(&mut z80, &mut c, 0x1234);
    // z80.reg.b = 0x00;
    // z80.reg.c = 0x03;
    set_bc_test_big(&mut z80, &mut c, 0x0003);

    ejecutar(&mut z80, 0xED, 0xB0, 0, 0, &mut c);

    //dbg_hex!(z80.reg.l);

    assert_eq!(z80.reg.h, 0xEF);
    assert_eq!(z80.reg.l, 0x13);

    assert_eq!(z80.reg.d, 0x12);
    assert_eq!(z80.reg.e, 0x37);

    assert_eq!(z80.reg.b, 0x00);
    assert_eq!(z80.reg.c, 0x00);
}

#[test]
fn cpir() {
    // ED B1   A – (HL), HL ← HL+1, BC ← BC – 1
    // Se compara (HL) con A. Durante la comparacion sone a 1 un bit de condicion.
    // HL es incrementado y BC es decrementado. Si despues de decrementar BC = 0 || A = (HL),
    // se termina la instruccion. Si BC != 0  && and A != (HL), se decrementa el PC en 2 y se repite
    // la instruccion.
    // Si BC = 0 antes de la ejecucion de la instruccion, la introccion continua
    // a traves de los 64 KB si no se encuentra nada
    //
    // S afectado por el signo      P/V=1 si BC-1!= 0   Z=1 si A = (HL)  H=1 si hay borrow en bit 4
    // Carry no afectado    N=1

    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    z80.mem.mem[0xEF10] = 0x09;
    c.bus.write_byte(0xEF10, 0x09);
    z80.mem.mem[0xEF11] = 0x09;
    c.bus.write_byte(0xEF11, 0x09);
    z80.mem.mem[0xEF12] = 0x09;
    c.bus.write_byte(0xEF12, 0x09);

    set_hl_test_big(&mut z80, &mut c, 0xEF10);

    set_bc_test_big(&mut z80, &mut c, 0x0004);

    ejecutar(&mut z80, 0xED, 0xB1, 0, 0, &mut c);
    //dbg_hex!(get_hl_test_big(&mut z80, &mut c));
    //dbg_hex!(get_bc_test_big(&mut z80, &mut c));

    assert_eq!(get_hl_test_big(&mut z80, &mut c), 0xEF14);
    assert_eq!(get_bc_test_big(&mut z80, &mut c), 0x0000);

    // 0 => false, 1 => true, 2 => indiferente
    // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 0, 1, 0, 0, 1, 2);
}
// ED B2
// Ed B3

#[test]
fn lddr() {
    // 0xED 0xB8 (DE) ← (HL), DE ← DE – 1, HL ← HL – 1, BC ← BC – 1
    // C, S, Z no afectados    H=0  PV=0  N=0
    let mut z80 = Z80::default();
    let mut c = CPU::new(0xFFFF);

    // pongo en la direccion 0xEF10 el valor 0x02  0xEF09 el valor 0xB1  0xEF08 el valor 0xA3
    z80.mem.mem[0xEF10] = 0x02;
    z80.mem.mem[0xEF09] = 0xB1;
    z80.mem.mem[0xEF08] = 0xA3;
    c.bus.write_byte(0xEF10, 0x02);
    c.bus.write_byte(0xEF09, 0xB1);
    c.bus.write_byte(0xEF08, 0xA3);

    // pongo HL = 0xEF10  y BC = 0x0003 y DE = 0x1234
    // z80.reg.h = 0xEF;
    // z80.reg.l = 0x10;
    //
    // z80.reg.d = 0x12;
    // z80.reg.e = 0x34;
    //
    // z80.reg.b = 0x00;
    // z80.reg.c = 0x03;
    set_hl_test_big(&mut z80, &mut c, 0xEF10);
    set_de_test_big(&mut z80, &mut c, 0x1234);
    set_bc_test_big(&mut z80, &mut c, 0x0003);

    //let bc = z80.get_loc16_big_endian(&Reg(R16::BC));
    //while z80.get_loc16_big_endian(&Reg(R16::BC)) != 0 {
    ejecutar(&mut z80, 0xED, 0xB8, 0, 0, &mut c);
    //}

    //dbg_hex!(z80.reg.h);
    //dbg_hex!(z80.reg.l);

    assert_eq!(z80.reg.h, 0xEF);
    assert_eq!(z80.reg.l, 0x0D);

    assert_eq!(z80.reg.d, 0x12);
    assert_eq!(z80.reg.e, 0x31);

    assert_eq!(z80.reg.b, 0x00);
    assert_eq!(z80.reg.c, 0x00);

    //     // 0 => false, 1 => true, 2 => indiferente
    //     // s(Sign)  z(Zero)  h(Halfcarry)  pv(Parityoverflow)  n(AddSubstract)  c(Carry)
    prueba_flags(&z80, 2, 2, 0, 0, 0, 2);
}
// ED B9
// ED BA
// ED BB

