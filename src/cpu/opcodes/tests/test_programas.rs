use crate::cpu::opcodes::tests::test_aux::*;
use crate::ops::StatusFlag;
use crate::z80::Z80;
use zilog_z80::cpu::CPU;

// macro que compara tres valores
macro_rules! assert_eq3 {
    ($a:expr, $b:expr, $c:expr $(,)?) => {
        if !($a == $b && $b == $c) {
            panic!(
                "Assertion failed: {} == {} == {}\nGot: {:?}, {:?}, {:?}",
                stringify!($a), stringify!($b), stringify!($c),
                $a, $b, $c
            );
        }
    };
}

#[test]
fn adc_a_ixh_ixl() {
    // 0000   3E 00                  LD   A,0x00
    // 0002   DD 21 61 41            LD   IX,0x4161
    // 0006   8F                     ADC   A,A
    // 0007   DD 8C                  ADC   A,IXH
    // 0009   DD 8D                  ADC   A,IXL
    // 000B

    let fichero = "PROGRAMAS/adc_a_ixh_ixl.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    //let mut pc = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    // LD A,0x00
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    //assert_eq!(cpu.reg.a, 0x00);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x00);

    // LD IX,0x4161
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.ix,cpu.reg.get_ix(), 0x4161);

    // ADC A,A
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x00);
    prueba_flags(&z80, 0, 1, 0, 0, 0, 0);

    // ADC A,IXH
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x41);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // ADC A,IXL
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xA2);
    prueba_flags(&z80, 1, 0, 0, 1, 0, 0);

    // Verificar finalización correcta (PC esta despues de la ultima instruccion)
    assert_eq!(z80.reg.pc as usize, 0x000B, "Contador de programa incorrecto");
}
#[test]
fn adc_a_iyh_iyl() {
    // 0000   3E 00                  LD   A,0x00
    // 0002   FD 21 61 41            LD   IY,0x4161
    // 0006   8F                     ADC   A,A
    // 0007   FD 8C                  ADC   A,IYH
    // 0009   FD 8D                  ADC   A,IYL
    let fichero = "PROGRAMAS/adc_a_iyh_iyl.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    //let mut pc = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    // LD   A,0x00
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x00);

    // LD IY,0x4161
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.iy,cpu.reg.get_iy(), 0x4161);

    // ADC   A,A
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x00);
    prueba_flags(&z80, 0, 1, 0, 0, 0, 0);

    // ADC A,IYH
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x41);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // ADC A,IYL
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xA2);
    prueba_flags(&z80, 1, 0, 0, 1, 0, 0);

    // Verificar finalización correcta (PC esta despues de la ultima instruccion)
    assert_eq!(z80.reg.pc as usize, 0x000B, "Contador de programa incorrecto");
}
#[test]
fn adc_i_hl_ix_iy() {
    // 0000   21 00 10               LD   HL,0x1000
    // 0003   DD 21 00 10            LD   IX,0x1000
    // 0007   FD 21 03 10            LD   IY,0x1003
    // 000B   3E 00                  LD   A,0x00
    // 000D   86                     ADD   A,(HL)
    // 000E   DD 8E 01               ADC   A,(IX+1)
    // 0011   FD 8E FF               ADC   A,(IY-1)
    // 0014   DD 8E 03               ADC   A,(IX+3)
    let fichero = "PROGRAMAS/adc_i_hl_ix_iy.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    //let mut pc = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    graba_mem(&mut z80, &mut cpu, 0x1000, 0x41);
    graba_mem(&mut z80, &mut cpu, 0x1001, 0x61);
    graba_mem(&mut z80, &mut cpu, 0x1002, 0x81);
    graba_mem(&mut z80, &mut cpu, 0x1003, 0x02);

    // LD   HL,0x1000
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.h,cpu.reg.h, 0x10);
    assert_eq3!(z80.reg.l,cpu.reg.l, 0x00);

    // LD IX,0x1000
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.get_ixh(),cpu.reg.ixh, 0x10);
    assert_eq3!(z80.reg.get_ixl(),cpu.reg.ixl, 0x00);

    // LD IY,0x1003
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.get_iyh(),cpu.reg.iyh, 0x10);
    assert_eq3!(z80.reg.get_iyl(),cpu.reg.iyl, 0x03);

    // LD   A,0x00
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x00);

    // ADD A,(HL)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x41);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // ADC A,(IX+1)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xA2);
    prueba_flags(&z80, 1, 0, 0, 1, 0, 0);

    // ADC A,(IY-1)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x23);
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);

    // ADC A,(IX+3)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x26);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // Verificar finalización correcta (PC esta despues de la ultima instruccion)
    assert_eq!(z80.reg.pc as usize, 0x0017, "Contador de programa incorrecto");
}

#[test]
fn adc_r() {
    // 0000   3E 00                  LD   A,0x00
    // 0002   06 41                  LD   B,0x41
    // 0004   0E 61                  LD   C,0x61
    // 0006   16 81                  LD   D,0x81
    // 0008   1E 41                  LD   E,0x41
    // 000A   26 61                  LD   H,0x61
    // 000C   2E 81                  LD   L,0x81
    // 000E   8F                     ADC   A,A
    // 000F   88                     ADC   A,B
    // 0010   89                     ADC   A,C
    // 0011   8A                     ADC   A,D
    // 0012   8B                     ADC   A,E
    // 0013   8C                     ADC   A,H
    // 0014   8D                     ADC   A,L
    // 0015   CE 01                  ADC   A,0x01
    // 0017   3E 0F                  LD   A,0x0F
    // 0019   06 01                  LD   B,0x01
    // 001B   88                     ADC   A,B

    let fichero = "PROGRAMAS/adc_r.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    //let mut pc = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    // LD   A,0x00
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x00);

    // LD   B,0x41
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.b,cpu.reg.b, 0x41);

    // LD   C,0x61
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.c,cpu.reg.c, 0x61);

    // LD   D,0x81
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.d,cpu.reg.d, 0x81);

    // LD   E,0x41
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.e,cpu.reg.e, 0x41);

    // LD   H,0x61
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.h,cpu.reg.h, 0x61);

    // LD   L,0x81
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.l,cpu.reg.l, 0x81);

    // ADC A,A
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x00);
    prueba_flags(&z80, 0, 1, 0, 0, 0, 0);

    // ADC A,B
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x41);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // ADC A,C
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a, cpu.reg.a, 0xA2);
    prueba_flags(&z80, 1, 0, 0, 1, 0, 0);

    // ADC A,D
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x23);
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);

    // ADC A,E
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x65);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // ADC A,H
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xC6);
    prueba_flags(&z80, 1, 0, 0, 1, 0, 0);

    // ADC A,L
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x47);
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);

    // ADC A,0x01
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x49);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // LD A,0x0F
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x0F);

    // LD B,0x01
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.b,cpu.reg.b, 0x01);

    // ADC A,B
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x10);
    prueba_flags(&z80, 0, 0, 1, 0, 0, 0);

    // Verificar finalización correcta (PC esta despues de la ultima instruccion)
    assert_eq!(z80.reg.pc as usize, 0x001C, "Contador de programa incorrecto");
}
#[test]
fn add_a_ixh_ixl() {
    // 0000   3E 0F                  LD   A,0x0F
    // 0002   87                     ADD   A,A
    // 0003   DD 21 80 E0            LD   IX,0xE080
    // 0007   DD 84                  ADD   A,IXH
    // 0009   3E 81                  LD   A,0x81
    // 000B   DD 85                  ADD   A,IXL

    let fichero = "PROGRAMAS/add_a_ixh_ixl.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    //let mut pc = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    // LD A,0x0F
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x0F);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // ADD A,A
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x1E);
    prueba_flags(&z80, 0, 0, 1, 0, 0, 0);

    // LD IX,0xE080
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.get_ixh(),cpu.reg.ixh, 0xE0);
    assert_eq3!(z80.reg.get_ixl(),cpu.reg.ixl, 0x80);

    // ADD A,IXH
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xFE);
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // LD A,0x81
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x81);

    // ADD,IXL
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x01);
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);

    // Verificar finalización correcta (PC esta despues de la ultima instruccion)
    assert_eq!(z80.reg.pc as usize, 0x000D, "Contador de programa incorrecto");
}
#[test]
fn add_a_iyh_iyl() {
    // 0000   3E 0F                  LD   A,0x0F
    // 0002   87                     ADD   A,A
    // 0003   FD 21 80 E0            LD   IY,0xE080
    // 0007   FD 84                  ADD   A,IYH
    // 0009   3E 81                  LD   A,0x81
    // 000B   FD 85                  ADD   A,IYL

    let fichero = "PROGRAMAS/add_a_iyh_iyl.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    //let mut pc = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    // LD A,0x0F
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x0F);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // ADD A,A
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x1E);
    prueba_flags(&z80, 0, 0, 1, 0, 0, 0);

    // LD IY,0xE080
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.iy,cpu.reg.get_iy(), 0xE080);

    // ADD A,IYH
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xFE);
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // LD A,0x81
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x81);

    // ADD A,IYL
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x01);
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);

    // Verificar finalización correcta (PC esta despues de la ultima instruccion)
    assert_eq!(z80.reg.pc as usize, 0x000D, "Contador de programa incorrecto");
}
#[test]
fn add_adc_sbc_16() {
    // 0000   21 FC 00               LD   HL,0x00FC
    // 0003   01 08 00               LD   BC,0x0008
    // 0006   11 FF FF               LD   DE,0xFFFF
    // 0009   09                     ADD   HL,BC
    // 000A   19                     ADD   HL,DE
    // 000B   ED 4A                  ADC   HL,BC
    // 000D   29                     ADD   HL,HL
    // 000E   19                     ADD   HL,DE
    // 000F   ED 42                  SBC   HL,BC
    // 0011   DD 21 FC 00            LD   IX,0x00FC
    // 0015   31 00 10               LD   SP,0x1000
    // 0018   DD 09                  ADD   IX,BC
    // 001A   DD 19                  ADD   IX,DE
    // 001C   DD 29                  ADD   IX,IX
    // 001E   DD 39                  ADD   IX,SP
    // 0020   FD 21 FF FF            LD   IY,0xFFFF
    // 0024   FD 09                  ADD   IY,BC
    // 0026   FD 19                  ADD   IY,DE
    // 0028   FD 29                  ADD   IY,IY
    // 002A   FD 39                  ADD   IY,SP
    // 002C   21 FF 7F               LD   HL,0x7FFF
    // 002F   01 01 00               LD   BC,0x0001
    // 0032   ED 4A                  ADC   HL,BC
    // 0034   ED 42                  SBC   HL,BC

    let fichero = "PROGRAMAS/add_adc_sbc_16.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    //let mut pc = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    // LD HL,0x00FC
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    //assert_eq3!(z80.reg.h,cpu.reg.h, 0x00);
    //assert_eq3!(z80.reg.l,cpu.reg.l, 0xFC);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x00FC);

    // LD BC,0x0008
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    //assert_eq3!(z80.reg.b,cpu.reg.b, 0x00);
    //assert_eq3!(z80.reg.c,cpu.reg.c, 0x08);
    assert_eq!(get_bc_test_big(&mut z80, &mut cpu), 0x0008);

    // Ejecutar LD DE,0xFFFF
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    //assert_eq3!(z80.reg.d,cpu.reg.d, 0xFF);
    //assert_eq3!(z80.reg.e,cpu.reg.e, 0xFF);
    assert_eq!(get_de_test_big(&mut z80, &mut cpu), 0xFFFF);

    // ADD HL,BC
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    //assert_eq3!(z80.reg.h,cpu.reg.h, 0x01);
    //assert_eq3!(z80.reg.l,cpu.reg.l, 0x04);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x0104);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // ADD HL,DE
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    //assert_eq3!(z80.reg.h,cpu.reg.h, 0x01);
    //assert_eq3!(z80.reg.l,cpu.reg.l, 0x03);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x0103);
    prueba_flags(&z80, 0, 0, 1, 0, 0, 1);

    // ADC   HL,BC
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    //assert_eq3!(z80.reg.h,cpu.reg.h, 0x01);
    //assert_eq3!(z80.reg.l,cpu.reg.l, 0x0C);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x010C);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    //ADD   HL,HL
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    //assert_eq3!(z80.reg.h,cpu.reg.h, 0x02);
    //assert_eq3!(z80.reg.l,cpu.reg.l, 0x18);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x0218);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // ADD   HL,DE
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    //assert_eq3!(z80.reg.h,cpu.reg.h, 0x02);
    //assert_eq3!(z80.reg.l,cpu.reg.l, 0x17);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x0217);
    prueba_flags(&z80, 0, 0, 1, 0, 0, 1);

    // SBC   HL,BC
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    // assert_eq3!(z80.reg.h,cpu.reg.h, 0x02);
    //assert_eq3!(z80.reg.l,cpu.reg.l, 0x0E);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x020E);
    prueba_flags(&z80, 0, 0, 0, 0, 1, 0);

    // LD   IX,0x00FC
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.ix,cpu.reg.get_ix(), 0x00FC);

    // LD   SP,0x1000
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.sp,cpu.reg.sp, 0x1000);

    // ADD   IX,BC
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.ix,cpu.reg.get_ix(), 0x0104);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // ADD   IX,DE
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.ix,cpu.reg.get_ix(), 0x0103);
    prueba_flags(&z80, 0, 0, 1, 0, 0, 1);

    // ADD   IX,IX
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.ix,cpu.reg.get_ix(), 0x0206);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // ADD   IX,SP
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.ix,cpu.reg.get_ix(), 0x1206);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    //  LD   IY,0xFFFF
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.iy,cpu.reg.get_iy(), 0xFFFF);

    // ADD   IY,BC
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.iy,cpu.reg.get_iy(), 0x0007);
    prueba_flags(&z80, 0, 0, 1, 0, 0, 1);

    // ADD   IY,DE
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.iy,cpu.reg.get_iy(), 0x0006);
    prueba_flags(&z80, 0, 0, 1, 0, 0, 1);

    // ADD   IY,IY
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.iy,cpu.reg.get_iy(), 0x000C);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // ADD   IY,SP
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.iy,cpu.reg.get_iy(), 0x100C);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // LD   HL,0x7FFF
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    //assert_eq3!(z80.reg.h,cpu.reg.h, 0x7F);
    //assert_eq3!(z80.reg.l,cpu.reg.l, 0xFF);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x7FFF);

    // LD   BC,0x0001
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    //assert_eq3!(z80.reg.b,cpu.reg.b, 0x00);
    //assert_eq3!(z80.reg.c,cpu.reg.c, 0x01);
    assert_eq!(get_bc_test_big(&mut z80, &mut cpu), 0x0001);

    //  ADC   HL,BC
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    //assert_eq3!(z80.reg.h,cpu.reg.h, 0x80);
    //assert_eq3!(z80.reg.l,cpu.reg.l, 0x00);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x8000);
    prueba_flags(&z80, 1, 0, 1, 1, 0, 0);

    // SBC   HL,BC
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    //assert_eq3!(z80.reg.h,cpu.reg.h, 0x7F);
    //assert_eq3!(z80.reg.l,cpu.reg.l, 0xFF);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x7FFF);

    prueba_flags(&z80, 0, 0, 1, 1, 1, 0);

    // Verificar finalización correcta (PC esta despues de la ultima instruccion)
    assert_eq!(z80.reg.pc as usize, 0x0036, "Contador de programa incorrecto");
}
#[test]
fn add_i_hl_ix_iy() {
    // 0000   21 00 10               LD   HL,0x1000
    // 0003   DD 21 00 10            LD   IX,0x1000
    // 0007   FD 21 03 10            LD   IY,0x1003
    // 000B   3E FF                  LD   A,0xFF
    // 000D   A6                     AND   (HL)
    // 000E   DD A6 01               AND   (IX+1)
    // 0011   FD A6 FF               AND   (IY-1)

    let fichero = "PROGRAMAS/add_i_hl_ix_iy.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    //let mut pc = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    graba_mem(&mut z80, &mut cpu, 0x1000, 0x41);
    graba_mem(&mut z80, &mut cpu, 0x1001, 0x61);
    graba_mem(&mut z80, &mut cpu, 0x1002, 0x81);

    // LD   HL,0x7FFF
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    //assert_eq3!(z80.reg.h,cpu.reg.h, 0x10);
    //assert_eq3!(z80.reg.l,cpu.reg.l, 0x00);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x1000);

    // LD   IX,0x1000
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.ix,cpu.reg.get_ix(), 0x1000);

    // LD   IY,0x1003
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.iy,cpu.reg.get_iy(), 0x1003);

    // LD A,0x00
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x00);

    // ADD A,(HL)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x41);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // ADD A,(IX+1)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xA2);
    prueba_flags(&z80, 1, 0, 0, 1, 0, 0);

    // ADD A,(IY-1)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x23);
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);

    // Verificar finalización correcta (PC esta despues de la ultima instruccion)
    assert_eq!(z80.reg.pc as usize, 0x0014, "Contador de programa incorrecto");
}
#[test]
fn add_r() {
    // 0000   3E 0F                  LD   A,0x0F
    // 0002   87                     ADD   A,A
    // 0003   06 E0                  LD   B,0xE0
    // 0005   80                     ADD   A,B
    // 0006   3E 81                  LD   A,0x81
    // 0008   0E 80                  LD   C,0x80
    // 000A   81                     ADD   A,C
    // 000B   16 FF                  LD   D,0xFF
    // 000D   82                     ADD   A,D
    // 000E   1E 40                  LD   E,0x40
    // 0010   83                     ADD   A,E
    // 0011   26 80                  LD   H,0x80
    // 0013   84                     ADD   A,H
    // 0014   2E 33                  LD   L,0x33
    // 0016   85                     ADD   A,L
    // 0017   C6 44                  ADD   A,0x44

    let fichero = "PROGRAMAS/add_r.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    //let mut pc = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    // LD A,0x0F
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x0F);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // ADD A,A
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x1E);
    prueba_flags(&z80, 0, 0, 1, 0, 0, 0);

    // LD B,0xE0
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.b,cpu.reg.b, 0xE0);

    // ADD A,B
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xFE);
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // LD A,0x81
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x81);

    // LD C,0x80
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.c,cpu.reg.c, 0x80);

    // ADD A,C
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x01);
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);

    // LD D,0xFF
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.d,cpu.reg.d, 0xFF);

    // ADD A,D
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x00);
    prueba_flags(&z80, 0, 1, 1, 0, 0, 1);

    // LD E,0x40
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.e,cpu.reg.e, 0x40);

    // ADD A,E
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x40);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // LD H,0x80
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.h,cpu.reg.h, 0x80);

    // ADD A,H
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xC0);
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // LD L,0x33
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.l,cpu.reg.l, 0x33);

    // ADD A,L
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xF3);
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // ADD A,0x44
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x37);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);

    // Verificar finalización correcta (PC esta despues de la ultima instruccion)
    assert_eq!(z80.reg.pc as usize, 0x0019, "Contador de programa incorrecto");
}

#[test]
fn and_i_hl_ix_iy() {
    // 0000   21 00 10               LD   HL,0x1000
    // 0003   DD 21 00 10            LD   IX,0x1000
    // 0007   FD 21 03 10            LD   IY,0x1003
    // 000B   3E FF                  LD   A,0xFF
    // 000D   A6                     AND   (HL)
    // 000E   DD A6 01               AND   (IX+1)
    // 0011   FD A6 FF               AND   (IY-1)

    let fichero = "PROGRAMAS/and_i_hl_ix_iy.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    let mut pc = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    graba_mem(&mut z80, &mut cpu, 0x1000, 0xFE);
    graba_mem(&mut z80, &mut cpu, 0x1001, 0xAA);
    graba_mem(&mut z80, &mut cpu, 0x1002, 0x99);

    // LD  HL,0x1000    LD  IX,0x1000     LD  IY,0x1003     LD  A,0xFF
    for _ in 0..4 {
        pc = z80.reg.pc as usize;
        ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    }

    // AND (HL)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xFE);
    prueba_flags(&z80, 1, 0, 1, 0, 0, 0);

    // AND (IX+1)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xAA);
    prueba_flags(&z80, 1, 0, 1, 1, 0, 0);

    // AND (IY-1)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x88);
    prueba_flags(&z80, 1, 0, 1, 1, 0, 0);

    // Verificar finalización correcta (PC esta despues de la ultima instruccion)
    assert_eq!(z80.reg.pc as usize, 0x0014, "Contador de programa incorrecto");
}
#[test]
fn and_r() {
    // 0000   3E FF                  LD   A,0xFF
    // 0002   06 01                  LD   B,0x01
    // 0004   0E 03                  LD   C,0x03
    // 0006   16 04                  LD   D,0x04
    // 0008   1E 08                  LD   E,0x08
    // 000A   26 10                  LD   H,0x10
    // 000C   2E 20                  LD   L,0x20
    // 000E   A0                     AND   B
    // 000F   F6 FF                  OR   0xFF
    // 0011   A1                     AND   C
    // 0012   F6 FF                  OR   0xFF
    // 0014   A2                     AND   D
    // 0015   F6 FF                  OR   0xFF
    // 0017   A3                     AND   E
    // 0018   F6 FF                  OR   0xFF
    // 001A   A4                     AND   H
    // 001B   F6 FF                  OR   0xFF
    // 001D   A5                     AND   L
    // 001E   F6 FF                  OR   0xFF
    // 0020   E6 40                  AND   0x40
    // 0022   F6 FF                  OR   0xFF
    // 0024   E6 AA                  AND   0xAA
    let fichero = "PROGRAMAS/and_r.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    let mut pc = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    // LD  A,0xFF   LD  B,0x01   LD  C,0x03   LD  D,0x04   LD  E,0x08   LD  H,0x10   LD  L,0x20
    for _ in 0..7 {
        pc = z80.reg.pc as usize;
        ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    }

    // AND B
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x01);
    prueba_flags(&z80, 0, 0, 1, 0, 0, 0);

    // OR 0xFF
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xFF);
    prueba_flags(&z80, 1, 0, 0, 1, 0, 0);

    // AND C
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x03);
    prueba_flags(&z80, 0, 0, 1, 1, 0, 0);

    // OR 0xFF
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xFF);
    prueba_flags(&z80, 1, 0, 0, 1, 0, 0);

    // AND D
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x04);
    prueba_flags(&z80, 0, 0, 1, 0, 0, 0);

    // OR 0xFF
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xFF);
    prueba_flags(&z80, 1, 0, 0, 1, 0, 0);

    // AND E
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x08);
    prueba_flags(&z80, 0, 0, 1, 0, 0, 0);

    // OR 0xFF
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xFF);
    prueba_flags(&z80, 1, 0, 0, 1, 0, 0);

    // AND H
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x10);
    prueba_flags(&z80, 0, 0, 1, 0, 0, 0);

    // OR 0xFF
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xFF);
    prueba_flags(&z80, 1, 0, 0, 1, 0, 0);

    // AND L
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x20);
    prueba_flags(&z80, 0, 0, 1, 0, 0, 0);

    // OR 0xFF
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xFF);
    prueba_flags(&z80, 1, 0, 0, 1, 0, 0);

    // AND 0x40
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x40);
    prueba_flags(&z80, 0, 0, 1, 0, 0, 0);

    // OR 0xFF
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xFF);
    prueba_flags(&z80, 1, 0, 0, 1, 0, 0);

    // AND 0xAA
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xAA);
    prueba_flags(&z80, 1, 0, 1, 1, 0, 0);

    // Verificar finalización correcta (PC esta despues de la ultima instruccion)
    assert_eq!(z80.reg.pc as usize, 0x0026, "Contador de programa incorrecto");
}

#[test]
fn call_cc_ret_cc() {

    // 0204   97                     SUB   A
    // 0205   C4 29 02               CALL   NZ,l0
    // 0208   CC 29 02               CALL   Z,l0
    // 020B   C6 01                  ADD   A,0x01
    // 020D   CC 2B 02               CALL   Z,l1
    // 0210   C4 2B 02               CALL   NZ,l1
    // 0213   07                     RLCA
    // 0214   EC 2D 02               CALL   PE,l2
    // 0217   E4 2D 02               CALL   PO,l2
    // 021A   D6 03                  SUB   0x03
    // 021C   F4 2F 02               CALL   P,l3
    // 021F   FC 2F 02               CALL   M,l3
    // 0222   D4 31 02               CALL   NC,l4
    // 0225   DC 31 02               CALL   C,l4
    // 0228   C9                     RET
    // 0229   C0           L0:       RET   NZ
    // 022A   C8                     RET   Z
    // 022B   C8           L1:       RET   Z
    // 022C   C0                     RET   NZ
    // 022D   E8           L2:       RET   PE
    // 022E   E0                     RET   PO
    // 022F   F0           L3:       RET   P
    // 0230   F8                     RET   M
    // 0231   D0           L4:       RET   NC
    // 0232   D8                     RET   C
    //
    //
    // L0:                 0229 DEFINED AT LINE 18
    //                     > USED AT LINE 4
    //                     > USED AT LINE 5
    // L1:                 022B DEFINED AT LINE 20
    //                     > USED AT LINE 7
    //                     > USED AT LINE 8
    // L2:                 022D DEFINED AT LINE 22
    //                     > USED AT LINE 10
    //                     > USED AT LINE 11
    // L3:                 022F DEFINED AT LINE 24
    //                     > USED AT LINE 13
    //                     > USED AT LINE 14
    // L4:                 0231 DEFINED AT LINE 26
    //                     > USED AT LINE 15
    //                     > USED AT LINE 16
    let fichero = "PROGRAMAS/call_cc_ret_cc.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    let mut pc: usize = 0x0204;
    let sp = 0x0100;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    z80.reg.pc = pc as u16;
    cpu.reg.pc = pc as u16;
    z80.reg.sp = sp;
    cpu.reg.sp = sp;

    // SUB   A

    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x00);

    //  CALL NZ,l0    no hace call
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0208);

    // CALL    Z,l0 si hace call
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0229);

    // RET NZ no retorna
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x022A);

    // RET Z retorna
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x020B);

    // ADD A,0x01
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x01);

    // CALL   Z,l1 no hace call
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0210);

    // CALL NZ,l1 si hace call
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x022B);

    // RET Z no retorna
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x022C);

    // RET NZ si retorna
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0213);

    //  RLCA
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x02);

    // CALL   PE,l2 no salta
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0217);

    // CALL   PO,l2  si salta
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x022D);

    // RET PE No retorna
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x022E);

    // RET PO si retorna
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x021A);

    // SUB   0x03
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xFF);

    // CALL   P,l3 no salta
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x021F);

    // CALL   M,l3 si salta
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x022F);

    // RET P  no retorna
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0230);

    // RET    si retorna
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0222);

    // CALL   NC,l4 no salta
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0225);

    // CALL   C,l4   si salta
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0231);

    // RET NC   no retorna
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0232);

    // RET C    retorna
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0228);

    // Verificar finalización correcta (PC esta despues de la ultima instruccion)

    assert_eq!(z80.reg.pc, 0x0228, "Contador de programa incorrecto");
}
#[test]
fn call_ret() {
    // 0204   CD 0A 02               CALL   l0
    // 0207   CD 0A 02               CALL   l0
    // 020A   C9           L0:       RET
    //
    //
    // L0:                 020A DEFINED AT LINE 5
    //                     > USED AT LINE 3
    //                     > USED AT LINE 4
    let fichero = "PROGRAMAS/call_ret.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    let mut pc: usize = 0x0204;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    z80.reg.pc = pc as u16;
    cpu.reg.pc = pc as u16;

    //     CALL l0
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x020A);
    assert_eq3!(z80.reg.sp,cpu.reg.sp, 0xFFFE);
    assert_eq3!(z80.mem.mem[0xFFFE],cpu.bus.read_byte(0xFFFE),0x07);
    assert_eq3!(z80.mem.mem[0xFFFF],cpu.bus.read_byte(0xFFFF),0x02);

    // RET
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0207);
    assert_eq3!(z80.reg.sp,cpu.reg.sp, 0x0000);

    //     CALL l0
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x020A);
    assert_eq3!(z80.reg.sp,cpu.reg.sp, 0xFFFE);
    assert_eq3!(z80.mem.mem[0xFFFE],cpu.bus.read_byte(0xFFFE),0x0A);
    assert_eq3!(z80.mem.mem[0xFFFF],cpu.bus.read_byte(0xFFFF),0x02);

    // RET
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x020A);
    assert_eq3!(z80.reg.sp,cpu.reg.sp, 0x0000);
}
#[test]
fn ccf_scf() {

    // 0000   97                     SUB   A
    // 0001   37                     SCF
    // 0002   3F                     CCF
    // 0003   D6 CC                  SUB   0xCC
    // 0005   3F                     CCF
    // 0006   37                     SCF

    let fichero = "PROGRAMAS/ccf_scf.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    //let mut pc: usize = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    //z80.reg.pc = pc as u16;
    //cpu.reg.pc = pc as u16;

    // SUB A
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x00);
    prueba_flags(&z80, 0, 1, 0, 0, 1, 0);

    // SCF
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x00);
    prueba_flags(&z80, 0, 1, 0, 0, 0, 1);

    // CCF
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x00);
    prueba_flags(&z80, 0, 1, 1, 0, 0, 0);

    // SUB 0xCC
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x34);
    prueba_flags(&z80, 0, 0, 1, 0, 1, 1);

    // CCF
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x34);
    prueba_flags(&z80, 0, 0, 1, 0, 0, 0);

    // SCF
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x34);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);
}
#[test]
fn cp_i_hl_ix_iy() {
    // 0000   21 00 10               LD   HL,0x1000
    // 0003   DD 21 00 10            LD   IX,0x1000
    // 0007   FD 21 03 10            LD   IY,0x1003
    // 000B   3E 41                  LD   A,0x41
    // 000D   BE                     CP   (HL)
    // 000E   DD BE 01               CP   (IX+1)
    // 0011   FD BE FF               CP   (IY-1)

    let fichero = "PROGRAMAS/cp_i_hl_ix_iy.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    //let mut pc: usize = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    graba_mem(&mut z80, &mut cpu, 0x1000, 0x41);
    graba_mem(&mut z80, &mut cpu, 0x1001, 0x61);
    graba_mem(&mut z80, &mut cpu, 0x1002, 0x22);

    // LD   HL,0x1000
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    //assert_eq3!(z80.reg.h,cpu.reg.h, 0x10);
    //assert_eq3!(z80.reg.l,cpu.reg.l, 0x00);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x1000);

    // LD   IX,0x1000
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.ix,cpu.reg.get_ix(), 0x1000);

    // LD   IY,0x1003
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.iy,cpu.reg.get_iy(), 0x1003);

    // LD   A,0x41
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x41);

    // CP (HL)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x41);
    prueba_flags(&z80, 0, 1, 0, 0, 1, 0);

    // CP (IX+1)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x41);
    prueba_flags(&z80, 1, 0, 0, 0, 1, 1);

    // CP (IY-1)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x41);
    prueba_flags(&z80, 0, 0, 1, 0, 1, 0);
}
#[test]
fn cp_r() {

    // 0000   3E 04                  LD   A,0x04
    // 0002   06 05                  LD   B,0x05
    // 0004   0E 03                  LD   C,0x03
    // 0006   16 FF                  LD   D,0xff
    // 0008   1E AA                  LD   E,0xaa
    // 000A   26 80                  LD   H,0x80
    // 000C   2E 7F                  LD   L,0x7f
    // 000E   BF                     CP   A
    // 000F   B8                     CP   B
    // 0010   B9                     CP   C
    // 0011   BA                     CP   D
    // 0012   BB                     CP   E
    // 0013   BC                     CP   H
    // 0014   BD                     CP   L
    // 0015   FE 04                  CP   0x04

    let fichero = "PROGRAMAS/cp_r.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    // let mut pc: usize = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    // LD   A,0x04
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x04);

    // LD B,0x05
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.b,cpu.reg.b, 0x05);

    // LD C,0x03
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.c,cpu.reg.c, 0x03);

    // LD D,0xFF
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.d,cpu.reg.d, 0xFF);

    // LD E,0xAA
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.e,cpu.reg.e, 0xAA);

    // LD H,0x80
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.h,cpu.reg.h, 0x80);

    // LD L,0x7F
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.l,cpu.reg.l, 0x7F);

    // CP A
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x04);
    prueba_flags(&z80, 0, 1, 0, 0, 1, 0);

    // CP B
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x04);
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);

    // CP C
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x04);
    prueba_flags(&z80, 0, 0, 0, 0, 1, 0);

    // CP D
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x04);
    prueba_flags(&z80, 0, 0, 1, 0, 1, 1);

    // CP E
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x04);
    prueba_flags(&z80, 0, 0, 1, 0, 1, 1);

    // CP H
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x04);
    prueba_flags(&z80, 1, 0, 0, 1, 1, 1);

    // CP L
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x04);
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);

    // CP 0x04
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x04);
    prueba_flags(&z80, 0, 1, 0, 0, 1, 0);
}
#[test]
fn cpd() {
    // 0000   21 03 10               LD   HL,0x1003
    // 0003   01 04 00               LD   BC,0x0004
    // 0006   3E 03                  LD   A,0x03
    // 0008   ED A9                  CPD
    // 000A   ED A9                  CPD
    // 000C   ED A9                  CPD
    // 000E   ED A9                  CPD

    let fichero = "PROGRAMAS/cpd.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    let mut pc: usize = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    graba_mem(&mut z80, &mut cpu, 0x1000, 0x01);
    graba_mem(&mut z80, &mut cpu, 0x1001, 0x02);
    graba_mem(&mut z80, &mut cpu, 0x1002, 0x03);
    graba_mem(&mut z80, &mut cpu, 0x1003, 0x04);

    for _ in 0..3 {
        pc = z80.reg.pc as usize;
        ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    }

    // CPD  (primero)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    // assert_eq3!(z80.reg.h,cpu.reg.h, 0x10);
    // assert_eq3!(z80.reg.l,cpu.reg.l, 0x02);
    // assert_eq3!(z80.reg.b,cpu.reg.b, 0x00);
    // assert_eq3!(z80.reg.c,cpu.reg.c, 0x03);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x1002);
    assert_eq!(get_bc_test_big(&mut z80, &mut cpu), 0x0003);

    prueba_flags(&z80, 1, 0, 1, 1, 1, 0);

    // CPD  (segundo)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    // assert_eq3!(z80.reg.h,cpu.reg.h, 0x10);
    // assert_eq3!(z80.reg.l,cpu.reg.l, 0x01);
    // assert_eq3!(z80.reg.b,cpu.reg.b, 0x00);
    // assert_eq3!(z80.reg.c,cpu.reg.c, 0x02);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x1001);
    assert_eq!(get_bc_test_big(&mut z80, &mut cpu), 0x0002);

    prueba_flags(&z80, 0, 1, 0, 1, 1, 0);

    // CPD  (tercero)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    // assert_eq3!(z80.reg.h,cpu.reg.h, 0x10);
    // assert_eq3!(z80.reg.l,cpu.reg.l, 0x00);
    // assert_eq3!(z80.reg.b,cpu.reg.b, 0x00);
    // assert_eq3!(z80.reg.c,cpu.reg.c, 0x01);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x1000);
    assert_eq!(get_bc_test_big(&mut z80, &mut cpu), 0x0001);

    prueba_flags(&z80, 0, 0, 0, 1, 1, 0);

    // CPD  (cuarto)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    // assert_eq3!(z80.reg.h,cpu.reg.h, 0x0F);
    // assert_eq3!(z80.reg.l,cpu.reg.l, 0xFF);
    // assert_eq3!(z80.reg.b,cpu.reg.b, 0x00);
    // assert_eq3!(z80.reg.c,cpu.reg.c, 0x00);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x0FFF);
    assert_eq!(get_bc_test_big(&mut z80, &mut cpu), 0x0000);

    prueba_flags(&z80, 0, 0, 0, 0, 1, 0);
}
#[test]
fn cpi() {
    // 0000   21 00 10               LD   HL,0X1000
    // 0003   01 04 00               LD   BC,0X0004
    // 0006   3E 03                  LD   A,0X03
    // 0008   ED A1                  CPI
    // 000A   ED A1                  CPI
    // 000C   ED A1                  CPI
    // 000E   ED A1                  CPI

    let fichero = "PROGRAMAS/cpi.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    let mut pc: usize = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    graba_mem(&mut z80, &mut cpu, 0x1000, 0x01);
    graba_mem(&mut z80, &mut cpu, 0x1001, 0x02);
    graba_mem(&mut z80, &mut cpu, 0x1002, 0x03);
    graba_mem(&mut z80, &mut cpu, 0x1003, 0x04);

    //  LD   HL,0X1000     LD   BC,0X0004      LD   A,0X03
    for _ in 0..3 {
        pc = z80.reg.pc as usize;
        ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    }

    // CPI (primero)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    // assert_eq3!(z80.reg.h,cpu.reg.h, 0x10);
    // assert_eq3!(z80.reg.l,cpu.reg.l, 0x01);
    // assert_eq3!(z80.reg.b,cpu.reg.b, 0x00);
    // assert_eq3!(z80.reg.c,cpu.reg.c, 0x03);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x1001);
    assert_eq!(get_bc_test_big(&mut z80, &mut cpu), 0x0003);

    prueba_flags(&z80, 0, 0, 0, 1, 1, 0);

    z80.reg.set_flag(&StatusFlag::Carry, true);
    cpu.reg.flags.c = true;

    // CPI (segundo)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    // assert_eq3!(z80.reg.h,cpu.reg.h, 0x10);
    // assert_eq3!(z80.reg.l,cpu.reg.l, 0x02);
    // assert_eq3!(z80.reg.b,cpu.reg.b, 0x00);
    // assert_eq3!(z80.reg.c,cpu.reg.c, 0x02);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x1002);
    assert_eq!(get_bc_test_big(&mut z80, &mut cpu), 0x0002);

    prueba_flags(&z80, 0, 0, 0, 1, 1, 1);

    // CPI (tercero)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    // assert_eq3!(z80.reg.h,cpu.reg.h, 0x10);
    // assert_eq3!(z80.reg.l,cpu.reg.l, 0x03);
    // assert_eq3!(z80.reg.b,cpu.reg.b, 0x00);
    // assert_eq3!(z80.reg.c,cpu.reg.c, 0x01);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x1003);
    assert_eq!(get_bc_test_big(&mut z80, &mut cpu), 0x0001);

    prueba_flags(&z80, 0, 1, 0, 1, 1, 1);

    // CPI (cuarto)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    // assert_eq3!(z80.reg.h,cpu.reg.h, 0x10);
    // assert_eq3!(z80.reg.l,cpu.reg.l, 0x04);
    // assert_eq3!(z80.reg.b,cpu.reg.b, 0x00);
    // assert_eq3!(z80.reg.c,cpu.reg.c, 0x00);

    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x1004);
    assert_eq!(get_bc_test_big(&mut z80, &mut cpu), 0x0000);

    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);
}
#[test]
fn cpir() {
    // ED B1   A – (HL), HL ← HL+1, BC ← BC – 1
    // S is set if result is negative; otherwise, it is reset.
    // Z is set if A equals (HL); otherwise, it is reset.
    // H is set if borrow from bit 4; otherwise, it is reset.
    // P/V is set if BC – 1 does not equal 0; otherwise, it is reset.
    // N is set.
    // C is not affected

    // 0000   21 00 10               LD   HL,0X1000
    // 0003   01 04 00               LD   BC,0X0004
    // 0006   3E 03                  LD   A,0X03
    // 0008   ED B1                  CPIR
    // 000A   ED B1                  CPIR

    let fichero = "PROGRAMAS/cpir.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    let mut pc: usize = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    graba_mem(&mut z80, &mut cpu, 0x1000, 0x01);
    graba_mem(&mut z80, &mut cpu, 0x1001, 0x02);
    graba_mem(&mut z80, &mut cpu, 0x1002, 0x03);
    graba_mem(&mut z80, &mut cpu, 0x1003, 0x04);

    for _ in 0..3 {
        pc = z80.reg.pc as usize;
        ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    }

    // CPIR (primero)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    //assert_eq3!(z80.reg.h,cpu.reg.h, 0x10);
    //assert_eq3!(z80.reg.l,cpu.reg.l, 0x03);
    //assert_eq3!(z80.reg.b,cpu.reg.b, 0x00);
    //assert_eq3!(z80.reg.c,cpu.reg.c, 0x01);

    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x1003);
    assert_eq!(get_bc_test_big(&mut z80, &mut cpu), 0x0001);

    prueba_flags(&z80, 0, 1, 0, 1, 1, 0);

    // CPIR (segundo)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.h,cpu.reg.h, 0x10);
    assert_eq3!(z80.reg.l,cpu.reg.l, 0x04);
    assert_eq3!(z80.reg.b,cpu.reg.b, 0x00);
    assert_eq3!(z80.reg.c,cpu.reg.c, 0x00);

    prueba_flags(&z80, 1, 0, 1, 0, 1, 0);
}
#[test]
fn cpl() {
    // 0000   97                     SUB   A
    // 0001   2F                     CPL
    // 0002   2F                     CPL
    // 0003   C6 AA                  ADD   A,0xAA
    // 0005   2F                     CPL
    // 0006   2F                     CPL

    let fichero = "PROGRAMAS/cpl.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    //let mut pc: usize = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    // SUB A
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x00);
    prueba_flags(&z80, 0, 1, 0, 0, 1, 0);

    // CPL (primero)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xFF);
    prueba_flags(&z80, 0, 1, 1, 0, 1, 0);

    // CPL (segundo)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x00);
    prueba_flags(&z80, 0, 1, 1, 0, 1, 0);

    // ADD A,0xAA
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xAA);
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // CPL (tercero)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x55);
    prueba_flags(&z80, 1, 0, 1, 0, 1, 0);

    // CPL (cuarto)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xAA);
    prueba_flags(&z80, 1, 0, 1, 0, 1, 0);
}
#[test]
fn daa() {
    // 0000   3E 15                  LD   A,0x15
    // 0002   06 27                  LD   B,0x27
    // 0004   80                     ADD   A,B
    // 0005   27                     DAA
    // 0006   90                     SUB   B
    // 0007   27                     DAA
    // 0008   3E 90                  LD   A,0x90
    // 000A   06 15                  LD   B,0x15
    // 000C   80                     ADD   A,B
    // 000D   27                     DAA
    // 000E   90                     SUB   B
    // 000F   27                     DAA

    let fichero = "PROGRAMAS/daa.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    //let mut pc: usize = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    // LD A,0x15
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x15);

    // LD B,0x27
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.b,cpu.reg.b, 0x27);

    // ADD A,B
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x3C);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // DAA
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x42);
    prueba_flags(&z80, 0, 0, 1, 1, 0, 0);

    // SUB B
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x1B);
    prueba_flags(&z80, 0, 0, 1, 0, 1, 0);

    // DAA
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x15);
    prueba_flags(&z80, 0, 0, 0, 0, 1, 0);

    // LD A,0x90
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x90);
    prueba_flags(&z80, 0, 0, 0, 0, 1, 0);

    // LD B,0x15
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.b,cpu.reg.b, 0x15);
    prueba_flags(&z80, 0, 0, 0, 0, 1, 0);

    // ADD A,B
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xA5);
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // DAA
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x05);
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);

    // SUB B
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xF0);
    prueba_flags(&z80, 1, 0, 0, 0, 1, 1);

    // DAA
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x90);
    prueba_flags(&z80, 1, 0, 0, 1, 1, 1);
}
#[test]
fn djnz() {
    // 0204   06 03                  LD   B,0x03
    // 0206   97                     SUB   A
    // 0207   3C           LOOP:     INC   A
    // 0208   10 FD                  DJNZ   loop
    // 020A   00                     NOP
    //
    //
    // LOOP:               0207 DEFINED AT LINE 4
    //                     > USED AT LINE 5

    let fichero = "PROGRAMAS/djnz.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    let mut pc: usize = 0x0204;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    z80.reg.pc = pc as u16;
    cpu.reg.pc = pc as u16;

    // LD   B,0x03
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.b,cpu.reg.b, 0x03);

    // SUB   A
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x00);

    // LOOP:     INC   A
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x01);

    // DJNZ   loop
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.b,cpu.reg.b, 0x02);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0207);

    // INC A
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.b,cpu.reg.b, 0x02);

    // DJNZ   loop
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.b,cpu.reg.b, 0x01);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0207);

    // INC A
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x03);

    // DJNZ   loop
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.b,cpu.reg.b, 0x00);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x020A);
}
#[test]
fn ex() {
    // 0000   21 34 12               LD   HL,0x1234
    // 0003   11 78 56               LD   DE,0x5678
    // 0006   EB                     EX   DE,HL
    // 0007   3E 11                  LD   A,0x11
    // 0009   08                     EX   AF,AF'
    // 000A   3E 22                  LD   A,0x22
    // 000C   08                     EX   AF,AF'
    // 000D   01 BC 9A               LD   BC,0x9ABC
    // 0010   D9                     EXX
    // 0011   21 11 11               LD   HL,0x1111
    // 0014   11 22 22               LD   DE,0x2222
    // 0017   01 33 33               LD   BC,0x3333
    // 001A   D9                     EXX
    // 001B   31 00 01               LD   SP,0x0100
    // 001E   D5                     PUSH   DE
    // 001F   E3                     EX   (SP),HL
    // 0020   DD 21 99 88            LD   IX,0x8899
    // 0024   DD E3                  EX   (SP),IX
    // 0026   FD 21 77 66            LD   IY,0x6677
    // 002A   FD E3                  EX   (SP),IY

    let fichero = "PROGRAMAS/ex.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    //let mut pc: usize = 0x0204;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    // LD HL 0x1234
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x1234);

    // LD   DE,0x5678
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_de_test_big(&mut z80, &mut cpu), 0x5678);

    // EX DE,HL
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_de_test_big(&mut z80, &mut cpu), 0x1234);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x5678);

    // LD A,0x11
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_af_test_big(&mut z80, &mut cpu), 0x1100);
    assert_eq!(get_afp_test_big(&mut z80, &mut cpu), 0x0000);

    // EX AF,AF'
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_af_test_big(&mut z80, &mut cpu), 0x0000);
    assert_eq!(get_afp_test_big(&mut z80, &mut cpu), 0x1100);

    // LD A,0x22
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_af_test_big(&mut z80, &mut cpu), 0x2200);
    assert_eq!(get_afp_test_big(&mut z80, &mut cpu), 0x1100);

    //  EX AF,AF'
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_af_test_big(&mut z80, &mut cpu), 0x1100);
    assert_eq!(get_afp_test_big(&mut z80, &mut cpu), 0x2200);

    //  LD BC,0x9ABC
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_bc_test_big(&mut z80, &mut cpu), 0x9ABC);

    // EXX
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x0000);
    assert_eq!(get_hlp_test_big(&mut z80, &mut cpu), 0x5678);
    assert_eq!(get_de_test_big(&mut z80, &mut cpu), 0x0000);
    assert_eq!(get_dep_test_big(&mut z80, &mut cpu), 0x1234);
    assert_eq!(get_bc_test_big(&mut z80, &mut cpu), 0x0000);
    assert_eq!(get_bcp_test_big(&mut z80, &mut cpu), 0x9ABC);

    // LD HL,0x1111
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x1111);

    // LD DE,0x2222
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_de_test_big(&mut z80, &mut cpu), 0x2222);

    // LD BC,0x3333
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_bc_test_big(&mut z80, &mut cpu), 0x3333);

    // EXX
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x5678);
    assert_eq!(get_hlp_test_big(&mut z80, &mut cpu), 0x1111);
    assert_eq!(get_de_test_big(&mut z80, &mut cpu), 0x1234);
    assert_eq!(get_dep_test_big(&mut z80, &mut cpu), 0x2222);
    assert_eq!(get_bc_test_big(&mut z80, &mut cpu), 0x9ABC);
    assert_eq!(get_bcp_test_big(&mut z80, &mut cpu), 0x3333);

    // LD SP,0x0100
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.sp,cpu.reg.sp, 0x0100);

    // PUSH DE
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_mem_16_big(&mut z80, &mut cpu, 0x00FE), 0x1234);

    // EX (SP),HL
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x1234);
    assert_eq!(get_mem_16_big(&mut z80, &mut cpu, 0x00FE), 0x5678);

    //  LD IX,0x8899
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.ix,cpu.reg.get_ix(), 0x8899);

    //  EX (SP),IX
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.ix,cpu.reg.get_ix(), 0x5678);
    assert_eq!(get_mem_16_big(&mut z80, &mut cpu, 0x00FE), 0x8899);

    //  LD IY,0x6677
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.iy,cpu.reg.get_iy(), 0x6677);

    // EX (SP),IY
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.iy,cpu.reg.get_iy(), 0x8899);
    assert_eq!(get_mem_16_big(&mut z80, &mut cpu, 0x00FE), 0x6677);
}
#[test]
fn halt() {
    // 0000   76                     HALT
    /*let fichero = "PROGRAMAS/halt.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    let mut pc: usize = 0x0204;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();
    // HALT (primero)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0000);
    // HALT (segundo)

    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0000);
    // HALT (tercero)

    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0000);*/
}
#[test]
fn in_a() {
    // .target "z80"
    // .format "bin"
    // .org 0
    //
    //             LD  SP,0xFF00
    //             LD  B,0xDE
    // @loop       IN  A,(0x07)
    //             CP  B
    //             JP  NZ,@loop
    //             RET

    // TODO No se como probarlo

}
#[test]
fn inc_dec_i_hl_ix_iy() {
    // 0000   21 00 10               LD   HL,0x1000
    // 0003   DD 21 00 10            LD   IX,0x1000
    // 0007   FD 21 03 10            LD   IY,0x1003
    // 000B   35                     DEC   (HL)
    // 000C   34                     INC   (HL)
    // 000D   DD 34 01               INC   (IX+1)
    // 0010   DD 35 01               DEC   (IX+1)
    // 0013   FD 34 FF               INC   (IY-1)
    // 0016   FD 35 FF               DEC   (IY-1)
    let fichero = "PROGRAMAS/inc_dec_i_hl_ix_iy.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    let mut pc: usize = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    graba_mem(&mut z80, &mut cpu, 0x1000, 0x00);
    graba_mem(&mut z80, &mut cpu, 0x1001, 0x3F);
    graba_mem(&mut z80, &mut cpu, 0x1002, 0x7F);

    //     for _ in 0..3 {
    //         c.execute();
    //     }
    // LD      HL,0x1000    LD      IX,0x1000     LD      IY,0x1003
    for _ in 0..3 {
        pc = z80.reg.pc as usize;
        ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    }

    // DEC (HL)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(0xFF, get_mem_u8(&mut z80, &mut cpu, 0x1000));
    prueba_flags(&z80, 1, 0, 1, 0, 1, 0);

    // INC (HL)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(0x00, get_mem_u8(&mut z80, &mut cpu, 0x1000));
    prueba_flags(&z80, 0, 1, 1, 0, 0, 0);

    // INC (IX+1)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(0x40, get_mem_u8(&mut z80, &mut cpu, 0x1001));
    prueba_flags(&z80, 0, 0, 1, 0, 0, 0);

    // DEC (IX+1)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(0x3F, get_mem_u8(&mut z80, &mut cpu, 0x1001));
    prueba_flags(&z80, 0, 0, 1, 0, 1, 0);

    // INC (IY-1)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(0x80, get_mem_u8(&mut z80, &mut cpu, 0x1002));
    prueba_flags(&z80, 1, 0, 1, 1, 0, 0);

    // DEC (IY-1)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(0x7F, get_mem_u8(&mut z80, &mut cpu, 0x1002));
    prueba_flags(&z80, 0, 0, 1, 1, 1, 0);
}
#[test]
fn inc_dec_r() {
    // 0000   3E 00                  LD   A,0x00
    // 0002   06 FF                  LD   B,0xFF
    // 0004   0E 0F                  LD   C,0x0F
    // 0006   16 0E                  LD   D,0x0E
    // 0008   1E 7F                  LD   E,0x7F
    // 000A   26 3E                  LD   H,0x3E
    // 000C   2E 23                  LD   L,0x23
    // 000E   3C                     INC   A
    // 000F   3D                     DEC   A
    // 0010   04                     INC   B
    // 0011   05                     DEC   B
    // 0012   0C                     INC   C
    // 0013   0D                     DEC   C
    // 0014   14                     INC   D
    // 0015   15                     DEC   D
    // 0016   FE 01                  CP   0x01   set carry flag (should be preserved)
    // 0018   1C                     INC   E
    // 0019   1D                     DEC   E
    // 001A   24                     INC   H
    // 001B   25                     DEC   H
    // 001C   2C                     INC   L
    // 001D   2D                     DEC   L

    let fichero = "PROGRAMAS/inc_dec_r.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    let mut pc: usize = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    for _ in 0..7 {
        pc = z80.reg.pc as usize;
        ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    }

    // INC A
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a,0x01);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // DEC A
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a,0x00);
    prueba_flags(&z80, 0, 1, 0, 0, 1, 0);

    // INC B
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.b,cpu.reg.b,0x00);
    prueba_flags(&z80, 0, 1, 1, 0, 0, 0);

    // DEC B
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.b,cpu.reg.b,0xFF);
    prueba_flags(&z80, 1, 0, 1, 0, 1, 0);

    // INC C
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.c,cpu.reg.c,0x10);
    prueba_flags(&z80, 0, 0, 1, 0, 0, 0);

    // DEC C
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.c,cpu.reg.c,0x0F);
    prueba_flags(&z80, 0, 0, 1, 0, 1, 0);

    // INC D
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.d,cpu.reg.d,0x0F);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // DEC D
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.d,cpu.reg.d,0x0E);
    prueba_flags(&z80, 0, 0, 0, 0, 1, 0);

    // CP 0x01   set carry flag (should be preserved)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a,0x00);
    prueba_flags(&z80, 1, 0, 1, 0, 1, 1);

    // INC E
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.e,cpu.reg.e,0x80);
    prueba_flags(&z80, 1, 0, 1, 1, 0, 1);

    // DEC E
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.e,cpu.reg.e,0x7F);
    prueba_flags(&z80, 0, 0, 1, 1, 1, 1);

    // INC H
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.h,cpu.reg.h,0x3F);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);

    // DEC H
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.h,cpu.reg.h,0x3E);
    prueba_flags(&z80, 0, 0, 0, 0, 1, 1);

    // INC L
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.l,cpu.reg.l,0x24);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 1);

    // DEC L
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.l,cpu.reg.l,0x23);
    prueba_flags(&z80, 0, 0, 0, 0, 1, 1);
}

#[test]
fn inc_dec_ss() {
    // 0000   01 00 00               LD   BC,0x0000
    // 0003   11 FF FF               LD   DE,0xffff
    // 0006   21 FF 00               LD   HL,0x00ff
    // 0009   31 11 11               LD   SP,0x1111
    // 000C   0B                     DEC   BC
    // 000D   03                     INC   BC
    // 000E   13                     INC   DE
    // 000F   1B                     DEC   DE
    // 0010   23                     INC   HL
    // 0011   2B                     DEC   HL
    // 0012   33                     INC   SP
    // 0013   3B                     DEC   SP

    let fichero = "PROGRAMAS/inc_dec_ss.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    let mut pc: usize = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    for _ in 0..4 {
        pc = z80.reg.pc as usize;
        ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    }

    // DEC   BC
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_bc_test_big(&mut z80, &mut cpu), 0xFFFF);

    // INC   BC
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_bc_test_big(&mut z80, &mut cpu), 0x0000);

    // INC   DE
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_de_test_big(&mut z80, &mut cpu), 0x0000);

    // DEC   DE
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_de_test_big(&mut z80, &mut cpu), 0xFFFF);

    // INC   HL
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x0100);

    // DEC   HL
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x00FF);

    // INC   SP
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.sp,cpu.reg.sp, 0x1112);

    // DEC   SP
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.sp,cpu.reg.sp, 0x1111);
}
#[test]
fn inc_dec_ss_ix_iy() {
    // 0000   01 00 00               LD   BC,0x0000
    // 0003   11 FF FF               LD   DE,0xffff
    // 0006   21 FF 00               LD   HL,0x00ff
    // 0009   31 11 11               LD   SP,0x1111
    // 000C   DD 21 FF 0F            LD   IX,0x0fff
    // 0010   FD 21 34 12            LD   IY,0x1234
    // 0014   0B                     DEC   BC
    // 0015   03                     INC   BC
    // 0016   13                     INC   DE
    // 0017   1B                     DEC   DE
    // 0018   23                     INC   HL
    // 0019   2B                     DEC   HL
    // 001A   33                     INC   SP
    // 001B   3B                     DEC   SP
    // 001C   DD 23                  INC   IX
    // 001E   DD 2B                  DEC   IX
    // 0020   FD 23                  INC   IY
    // 0022   FD 2B                  DEC   IY

    let fichero = "PROGRAMAS/inc_dec_ss_ix_iy.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    let mut pc: usize = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    for _ in 0..6 {
        pc = z80.reg.pc as usize;
        ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    }

    // DEC  BC
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_bc_test_big(&mut z80, &mut cpu), 0xFFFF);

    // INC BC
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_bc_test_big(&mut z80, &mut cpu), 0x0000);

    // INC DE
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_de_test_big(&mut z80, &mut cpu), 0x0000);

    // DEC DE
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_de_test_big(&mut z80, &mut cpu), 0xFFFF);

    // INC HL
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x0100);

    // DEC HL
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x00FF);

    // INC SP
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.sp,cpu.reg.sp, 0x1112);

    // DEC   SP
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.sp,cpu.reg.sp, 0x1111);

    // INC IX
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.ix,cpu.reg.get_ix(), 0x1000);

    // DEC IX
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.ix,cpu.reg.get_ix(), 0x0FFF);

    // INC IY
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.iy,cpu.reg.get_iy(), 0x1235);

    // DEC IY
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.iy,cpu.reg.get_iy(), 0x1234);
}
#[test]
fn int() {
    // if this test loops forever, interrupts are not working
    // 0000   31 00 FF               LD   SP,0xFF00
    // 0003   3E 0F                  LD   A,0x0F
    // 0005   C3 0A 00               JP   start
    // 0008                INT:      .ORG   0x0008
    // 0008   47                     LD   B,A
    // 0009   C9                     RET
    // 0010                START:    .ORG   0x0010
    // 0010   FB                     EI
    // 0011   B8           LOOP:     CP   B
    // 0012   C2 11 00               JP   NZ,loop
    // 0015   C9                     RET
    // 0016                          .END
    //
    //
    // INT:                0008 DEFINED AT LINE 6
    // START:              000A DEFINED AT LINE 10
    //                     > USED AT LINE 4
    // LOOP:               0011 DEFINED AT LINE 12
    //                     > USED AT LINE 13
    let fichero = "PROGRAMAS/int.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    let mut pc: usize = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    //  c.bus.load_bin("bin/int.bin", 0).unwrap();
    //     for _ in 0..7 {
    //         c.execute();
    //     }

    for _ in 0..7 {
        pc = z80.reg.pc as usize;
        ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    }

    z80.int_request(0xCF);
    cpu.int_request(0xCF);
    loop {
        //dbg_hex!(cpu.reg.pc);
        pc = z80.reg.pc as usize;
        ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
        //dbg_hex!(cpu.reg.pc);
        if cpu.reg.pc == 0x0000 {
            break;
        }
    }
}

#[test]
fn int_im1() {

    // if this test loops forever, mode 1 interrupts are not working
    // 0000                          .ORG   0
    // 0000   31 00 FF               LD   SP,0xFF00
    // 0003   3E 0F                  LD   A,0x0F
    // 0005   C3 3A 00               JP   start
    // 0018                          .ORG   0x0018
    // 0018   4F                     LD   C,A
    // 0019   C9                     RET
    // 0038                          .ORG   0x0038
    // 0038   47                     LD   B,A
    // 0039   C9                     RET
    // 0050                START:    .ORG   0x0050
    // 0050   ED 56                  IM   1
    // 0052   FB                     EI
    // 0053   B8           LOOP:     CP   B
    // 0054   C2 53 00               JP   NZ,loop
    // 0057   C9                     RET
    // 0058                          .END
    //
    //
    // START:              003A DEFINED AT LINE 14
    //                     > USED AT LINE 4
    // LOOP:               0053 DEFINED AT LINE 17
    //                     > USED AT LINE 18

    let fichero = "PROGRAMAS/im1.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    let mut pc: usize = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    //     for _ in 0..8 {
    //         c.execute();
    //     }
    for _ in 0..8 {
        pc = z80.reg.pc as usize;
        ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    }
    z80.int_request(0xDF);
    cpu.int_request(0xDF);
    loop {
        //cpu.execute();
        pc = z80.reg.pc as usize;
        ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);

        if cpu.reg.pc == 0x0000 {
            break;
        }
    }
}
#[test]
fn int_im2() {
    // if this test loops forever, mode 2 interrupts are not working
    // 0000   31 00 FF               LD   SP,0xFF00
    // 0003   3E 01                  LD   A,0x01
    // 0005   ED 47                  LD   I,A
    // 0007   3E 0F                  LD   A,0x0F
    // 0009   C3 3A 00               JP   start
    // 0038                          .ORG   0x0038
    // 0038   57                     LD   D,A
    // 0039   C9                     RET
    // 0050                START:    .ORG   0x0050
    // 0050   ED 5E                  IM   2
    // 0052   FB                     EI
    // 0053   B8           LOOP:     CP   B
    // 0054   C2 53 00               JP   NZ,loop
    // 0057   C9                     RET
    // 0102                          .ORG   0x0102
    // 0102   06 01                  DW   0x0106
    // 0106                          .ORG   0x0106
    // 0106   47                     LD   B,A
    // 0107   C9                     RET
    // 0108                          .END
    //
    //
    // START:              003A DEFINED AT LINE 12
    //                     > USED AT LINE 6
    // LOOP:               0053 DEFINED AT LINE 15
    //                     > USED AT LINE 16

    let fichero = "PROGRAMAS/im2.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    let mut pc: usize = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();
    // for _ in 0..9 {
    //         c.execute();
    //     }
    for _ in 0..9 {
        pc = z80.reg.pc as usize;
        ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    }
    z80.int_request(0x02);
    cpu.int_request(0x02);
    loop {
        //cpu.execute();
        pc = z80.reg.pc as usize;
        ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);

        if cpu.reg.pc == 0x0000 {
            break;
        }
    }
}
#[test]
fn jp_cc_nn() {
    // 0204   97                     SUB   A
    // 0205   C2 0C 02               JP   NZ,label0
    // 0208   CA 0C 02               JP   Z,label0
    // 020B   00                     NOP
    // 020C   C6 01        LABEL0:   ADD   A,0x01
    // 020E   CA 15 02               JP   Z,label1
    // 0211   C2 15 02               JP   NZ,label1
    // 0214   00                     NOP
    // 0215   07           LABEL1:   RLCA
    // 0216   EA 1D 02               JP   PE,label2
    // 0219   E2 1D 02               JP   PO,label2
    // 021C   00                     NOP
    // 021D   C6 FD        LABEL2:   ADD   A,0xFD
    // 021F   F2 26 02               JP   P,label3
    // 0222   FA 26 02               JP   M,label3
    // 0225   00                     NOP
    // 0226   D2 2D 02     LABEL3:   JP   NC,label4
    // 0229   DA 2D 02               JP   C,label4
    // 022C   00                     NOP
    // 022D   00           LABEL4:   NOP
    //
    //
    // LABEL0:             020C DEFINED AT LINE 6
    //                     > USED AT LINE 3
    //                     > USED AT LINE 4
    // LABEL1:             0215 DEFINED AT LINE 10
    //                     > USED AT LINE 7
    //                     > USED AT LINE 8
    // LABEL2:             021D DEFINED AT LINE 14
    //                     > USED AT LINE 11
    //                     > USED AT LINE 12
    // LABEL3:             0226 DEFINED AT LINE 18
    //                     > USED AT LINE 15
    //                     > USED AT LINE 16
    // LABEL4:             022D DEFINED AT LINE 21
    //                     > USED AT LINE 18
    //                     > USED AT LINE 19

    let fichero = "PROGRAMAS/jp_cc_nn.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);
    let mut pc: usize = 0x0000;

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    z80.reg.pc = 0x0204;
    cpu.reg.pc = 0x0204;

    // SUB A
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x00);
    prueba_flags(&z80, 0, 1, 0, 0, 1, 0);

    // JP NZ,label0   no salta
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0208);

    // JP Z,label0   si salta
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x020C);

    // label0: ADD   A,0x01
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x01);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // JP Z,label1  no salta
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0211);

    // JP NZ,label1
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0215);

    // LABEL1:   RLCA
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x02);
    prueba_flags(&z80, 0, 0, 0, 0, 0, 0);

    // JP PE,label2 no salta
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0219);

    // JP PO,label2  si salta
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x021D);

    // LABEL2:   ADD   A,0xFD
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0xFF);
    prueba_flags(&z80, 1, 0, 0, 0, 0, 0);

    // JP P,label3 no salta
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0222);

    // JP M,label3  si salta
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x0226);

    // LABEL3:   JP   NC,label4  si salta
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc,cpu.reg.pc, 0x022D);
}
#[test]
fn jp_jr() {
    // 0204   21 16 02               LD   HL,l3
    // 0207   DD 21 19 02            LD   IX,l4
    // 020B   FD 21 21 02            LD   IY,l5
    // 020F   C3 14 02               JP   l0
    // 0212   18 04        L1:       JR   l2
    // 0214   18 FC        L0:       JR   l1
    // 0216   DD E9        L3:       JP   (IX)
    // 0218   E9           L2:       JP   (HL)
    // 0219   FD E9        L4:       JP   (IY)
    // 021B   18 06        L6:       JR   l7
    // 021D   00                     NOP
    // 021E   00                     NOP
    // 021F   00                     NOP
    // 0220   00                     NOP
    // 0221   18 F8        L5:       JR   l6
    // 0223   00           L7:       NOP
    //
    //
    // L1:                 0212 DEFINED AT LINE 8
    //                     > USED AT LINE 9
    // L0:                 0214 DEFINED AT LINE 9
    //                     > USED AT LINE 7
    // L3:                 0216 DEFINED AT LINE 10
    //                     > USED AT LINE 4
    // L2:                 0218 DEFINED AT LINE 11
    //                     > USED AT LINE 8
    // L4:                 0219 DEFINED AT LINE 12
    //                     > USED AT LINE 5
    // L6:                 021B DEFINED AT LINE 13
    //                     > USED AT LINE 18
    // L5:                 0221 DEFINED AT LINE 18
    //                     > USED AT LINE 6
    // L7:                 0223 DEFINED AT LINE 19
    //                     > USED AT LINE 13

    let fichero = "PROGRAMAS/jp_jr.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    z80.reg.pc = 0x0204;
    cpu.reg.pc = 0x0204;

    let pc = z80.reg.pc as usize;

    // LD   HL,l3
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_hl_test_big(&mut z80, &mut cpu), 0x0216);

    // LD   IX,l4
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.ix, cpu.reg.get_ix(), 0x0219);

    // LD   IY,l5
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.iy, cpu.reg.get_iy(), 0x0221);

    // JP   l0
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc, cpu.reg.pc, 0x0214);

    // JR   l1
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc, cpu.reg.pc, 0x0212);

    // JR   l2
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc, cpu.reg.pc, 0x0218);

    // JP   (HL)
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc, cpu.reg.pc, 0x0216);

    //  JP   (IX)
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc, cpu.reg.pc, 0x0219);

    // JP   (IY)
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc, cpu.reg.pc, 0x0221);

    // JR   l7
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc, cpu.reg.pc, 0x021B);

    // NOP
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc, cpu.reg.pc, 0x0223);
}

#[test]
fn jr_cc() {
    // 0204   97                     SUB   A
    // 0205   20 03                  JR   NZ,l0
    // 0207   28 01                  JR   Z,l0
    // 0209   00                     NOP
    // 020A   C6 01        L0:       ADD   A,0x01
    // 020C   28 03                  JR   Z,l1
    // 020E   20 01                  JR   NZ,l1
    // 0210   00                     NOP
    // 0211   D6 03        L1:       SUB   0x03
    // 0213   30 03                  JR   NC,l2
    // 0215   38 01                  JR   C,l2
    // 0217   00                     NOP
    // 0218   00           L2:       NOP
    //
    //
    // L0:                 020A DEFINED AT LINE 6
    //                     > USED AT LINE 3
    //                     > USED AT LINE 4
    // L1:                 0211 DEFINED AT LINE 10
    //                     > USED AT LINE 7
    //                     > USED AT LINE 8
    // L2:                 0218 DEFINED AT LINE 14
    //                     > USED AT LINE 11
    //                     > USED AT LINE 12
    let fichero = "PROGRAMAS/jr_cc.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    z80.reg.pc = 0x0204;
    cpu.reg.pc = 0x0204;

    let pc = z80.reg.pc as usize;

    // SUB A
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a, cpu.reg.a, 0x00);

    // JR NZ,l0
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc, cpu.reg.pc, 0x207);

    // JR Z,l0
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc, cpu.reg.pc, 0x20A);

    // ADD A,0x01
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a, cpu.reg.a, 0x01);

    // JR Z,l1
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc, cpu.reg.pc, 0x020E);

    // JR NZ,l1
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc, cpu.reg.pc, 0x0211);

    // SUB 0x03
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a, cpu.reg.a, 0xFE);

    // JR NC,l2
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc, cpu.reg.pc, 0x0215);

    // JR C,l2
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.pc, cpu.reg.pc, 0x0218);
}
#[test]
fn ld_a_i_bc_de_nn() {
    // 0000   01 00 10               LD   BC,0x1000
    // 0003   11 01 10               LD   DE,0x1001
    // 0006   0A                     LD   A,(BC)
    // 0007   1A                     LD   A,(DE)
    // 0008   3A 02 10               LD   A,(0x1002)

    let fichero = "PROGRAMAS/ld_a_i_bc_de_nn.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    let pc = 0x0000;

    graba_mem(&mut z80, &mut cpu, 0x1000, 0x11);
    graba_mem(&mut z80, &mut cpu, 0x1001, 0x22);
    graba_mem(&mut z80, &mut cpu, 0x1002, 0x33);

    // LD BC,0x1000
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_bc_test_big(&mut z80, &mut cpu), 0x1000);

    //LD DE,0x1001
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_de_test_big(&mut z80, &mut cpu), 0x1001);

    // LD A,(BC)
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a, cpu.reg.a, 0x11);

    // LD A,(DE)
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a, cpu.reg.a, 0x22);

    // LD A,(0x1002)
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a, cpu.reg.a, 0x33);
}

#[test]
fn ld_a_ir() {
    // 0000   FB                     EI
    // 0001   ED 57                  LD   A,I
    // 0003   97                     SUB   A
    // 0004   ED 5F                  LD   A,R

    let fichero = "PROGRAMAS/ld_a_ir.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    let mut pc = 0x0000;

    z80.reg.r = 0x34;
    cpu.reg.r = 0x34;
    z80.reg.i = 0x01;
    cpu.reg.i = 0x01;

    z80.reg.set_flag(&StatusFlag::Carry, true);
    cpu.reg.flags.c = true;

    // EI
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);

    // LD A,I
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x01);
    prueba_flags(&z80, 0, 0, 0, 1, 0, 1);

    // SUB   A
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x00);
    prueba_flags(&z80, 0, 1, 0, 0, 1, 0);

    // LD   A,R
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a,cpu.reg.a, 0x34);
    prueba_flags(&z80, 0, 0, 0, 1, 0, 0);
}

#[test]
fn ld_hl() {

    // 0100   77                     LD   (HL),A
    // 0101   46                     LD   B,(HL)
    // 0102   4E                     LD   C,(HL)
    // 0103   56                     LD   D,(HL)
    // 0104   5E                     LD   E,(HL)
    // 0105   66                     LD   H,(HL)
    let fichero = "PROGRAMAS/ld_hl.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    z80.reg.pc = 0x0100;
    cpu.reg.pc = 0x0100;

    z80.reg.a = 0x33;
    cpu.reg.a = 0x33;

    // hl=0x1000
    set_hl_test_big(&mut z80, &mut cpu, 0x1000);

    // LD   (HL),A
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(get_mem_u8(&mut z80, &mut cpu, 0x1000), 0x33);

    // LD   B,(HL)
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.b, cpu.reg.b, 0x33);

    // LD   C,(HL)
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.c, cpu.reg.c, 0x33);

    //  LD   D,(HL)
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.d, cpu.reg.d, 0x33);

    //  LD   E,(HL)
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.e, cpu.reg.e, 0x33);

    //  LD   H,(HL)
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.h, cpu.reg.h, 0x33);
}
#[test]
fn ld_hl_dd_ix_iy_inn() {
    // 0000   2A 00 10               LD   HL,(0x1000)
    // 0003   ED 4B 01 10            LD   BC,(0x1001)
    // 0007   ED 5B 02 10            LD   DE,(0x1002)
    // 000B   2A 03 10               LD   HL,(0x1003)
    // 000E   ED 7B 04 10            LD   SP,(0x1004)
    // 0012   DD 2A 05 10            LD   IX,(0x1005)
    // 0016   FD 2A 06 10            LD   IY,(0x1006)

    let fichero = "PROGRAMAS/ld_hl_dd_ix_iy_inn.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    graba_mem(&mut z80, &mut cpu, 0x1000, 0x01);
    graba_mem(&mut z80, &mut cpu, 0x1001, 0x02);
    graba_mem(&mut z80, &mut cpu, 0x1002, 0x03);
    graba_mem(&mut z80, &mut cpu, 0x1003, 0x04);
    graba_mem(&mut z80, &mut cpu, 0x1004, 0x05);
    graba_mem(&mut z80, &mut cpu, 0x1005, 0x06);
    graba_mem(&mut z80, &mut cpu, 0x1006, 0x07);
    graba_mem(&mut z80, &mut cpu, 0x1007, 0x08);

    // LD HL,(0x1000)
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    let hl = get_hl_test_big(&mut z80, &mut cpu);
    assert_eq!(0x0201, hl);

    // LD BC,(0x1001)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    let bc = get_bc_test_big(&mut z80, &mut cpu);
    assert_eq!(0x0302, bc);

    // LD DE,(0x1002)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    let de = get_de_test_big(&mut z80, &mut cpu);
    assert_eq!(0x0403, de);

    // LD HL,(0x1003)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    let hl = get_hl_test_big(&mut z80, &mut cpu);
    assert_eq!(0x0504, hl);

    // LD SP,(0x1004)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.sp, cpu.reg.sp,0x0605);

    // LD IX,(0x1004)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.ix, cpu.reg.get_ix(),0x0706);

    // LD IY,(0x1005)
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.iy, cpu.reg.get_iy(),0x0807);
}

#[test]
fn ld_hl_n() {
    // 0000   21 00 20               LD   HL,0x2000
    // 0003   36 33                  LD   (HL),0x33
    // 0005   21 00 10               LD   HL,0x1000
    // 0008   36 65                  LD   (HL),0x65
    let fichero = "PROGRAMAS/ld_hl_n.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    // LD   HL,0x2000
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(get_hl_test_big(&mut z80,&mut cpu), cpu.reg.get_hl(),0x2000);

    // LD   (HL),0x33
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(0x33, get_mem_16_big(&mut z80, &mut cpu, 0x2000));

    // LD   HL,0x1000
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(get_hl_test_big(&mut z80,&mut cpu), cpu.reg.get_hl(),0x1000);

    // LD   (HL),0x65
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(0x65, get_mem_16_big(&mut z80, &mut cpu, 0x1000));
}

#[test]
fn ld_i_bc_de_nn_a() {
    // 0000   01 00 10               LD   BC,0x1000
    // 0003   11 01 10               LD   DE,0x1001
    // 0006   3E 77                  LD   A,0x77
    // 0008   02                     LD   (BC),A
    // 0009   12                     LD   (DE),A
    // 000A   32 02 10               LD   (0x1002),A
    let fichero = "PROGRAMAS/ld_i_bc_de_nn_a.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();

    //LD   BC,0x1000
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(get_bc_test_big(&mut z80,&mut cpu), cpu.reg.get_bc(),0x1000);

    // LD   DE,0x1001
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(get_de_test_big(&mut z80,&mut cpu), cpu.reg.get_de(),0x1001);

    // LD A,0x77
    let mut pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq3!(z80.reg.a, cpu.reg.a, 0x77);

    // LD (BC),A
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(0x77, get_mem_16_big(&mut z80, &mut cpu, 0x1000));

    // LD (DE),A
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(0x77, get_mem_16_big(&mut z80, &mut cpu, 0x1001));

    // LD (0x1002),A
    pc = z80.reg.pc as usize;
    ejecutar_en_direccion(&mut z80, pc, pg[pc], pg[pc + 1], pg[pc + 2], pg[pc + 3], &mut cpu);
    assert_eq!(0x77, get_mem_16_big(&mut z80, &mut cpu, 0x1002));
}

#[test]
fn ld_i_hl_r() {
    // 0000   21 00 10               LD   HL,0x1000
    // 0003   3E 12                  LD   A,0x12
    // 0005   77                     LD   (HL),A
    // 0006   06 13                  LD   B,0x13
    // 0008   70                     LD   (HL),B
    // 0009   0E 14                  LD   C,0x14
    // 000B   71                     LD   (HL),C
    // 000C   16 15                  LD   D,0x15
    // 000E   72                     LD   (HL),D
    // 000F   1E 16                  LD   E,0x16
    // 0011   73                     LD   (HL),E
    // 0012   74                     LD   (HL),H
    // 0013   75                     LD   (HL),L
    let fichero = "PROGRAMAS/ld_i_hl_r.z80.bin";

    let mut z80 = Z80::default();
    let mut cpu = CPU::new(0xFFFF);

    let pg: Vec<u8> = std::fs::read(fichero).expect("No se pudo cargar el programa");

    // Copiar el programa a la memoria
    z80.load(pg.as_slice());
    cpu.bus.load_bin(fichero, 0).unwrap();
    //     assert_eq!(c.execute(), 10);
    //     assert_eq!(0x1000, c.reg.get_hl());

    //     assert_eq!(c.execute(), 7);
    //     assert_eq!(0x12, c.reg.a);

    //     assert_eq!(c.execute(), 7);
    //     assert_eq!(0x12, c.bus.read_byte(0x1000));

    //     assert_eq!(c.execute(), 7);
    //     assert_eq!(0x13, c.reg.b);

    //     assert_eq!(c.execute(), 7);
    //     assert_eq!(0x13, c.bus.read_byte(0x1000));

    //     assert_eq!(c.execute(), 7);
    //     assert_eq!(0x14, c.reg.c);

    //     assert_eq!(c.execute(), 7);
    //     assert_eq!(0x14, c.bus.read_byte(0x1000));

    //     assert_eq!(c.execute(), 7);
    //     assert_eq!(0x15, c.reg.d);

    //     assert_eq!(c.execute(), 7);
    //     assert_eq!(0x15, c.bus.read_byte(0x1000));

    //     assert_eq!(c.execute(), 7);
    //     assert_eq!(0x16, c.reg.e);

    //     assert_eq!(c.execute(), 7);
    //     assert_eq!(0x16, c.bus.read_byte(0x1000));

    //     assert_eq!(c.execute(), 7);
    //     assert_eq!(0x10, c.bus.read_byte(0x1000));

    //     assert_eq!(c.execute(), 7);
    //     assert_eq!(0x00, c.bus.read_byte(0x1000));
}

#[test]
fn ld_inn_hl_dd_ix_iy() {}

#[test]
fn ld_ir_a() {}

#[test]
fn ld_ix_iy_n() {}

#[test]
fn ld_ix_iy_nn() {}

#[test]
fn ld_ix_iy_r() {}

#[test]
fn ld_r_ix_iy() {}

#[test]
fn ld_r_r() {}

#[test]
fn ld_sp_hl_ix_iy() {}

#[test]
fn ldd() {}

#[test]
fn lddr() {}

#[test]
fn ldi() {}

#[test]
fn ldir() {}

#[test]
fn neg() {}

#[test]
fn nmi() {}

#[test]
fn or_r() {}

#[test]
fn or_xor_i_hl_ix_iy() {}

#[test]
fn out_a() {}

#[test]
fn push_pop() {}

#[test]
fn rlc_rl_rrc_rr_r() {}

#[test]
fn rlca_rla_rrca_rra() {}

#[test]
fn rld_rrd() {}

#[test]
fn rrc_rlc_rr_rl_i_hl_ix_iy() {}

#[test]
fn sbc_i_hl_ix_iy() {}

// hay mas.........................






