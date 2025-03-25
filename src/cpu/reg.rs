use crate::ops::{StatusFlag, R16, R8};

// Representacion de los registros del Z80
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Registros {
    pub a: u8,
    pub f: u8,

    pub b: u8,
    pub c: u8,

    pub d: u8,
    pub e: u8,

    pub h: u8,
    pub l: u8,

    pub r: u8,
    pub i: u8, // interrupciones

    pub ap: u8, // _p estos don los prima
    pub fp: u8,

    pub bp: u8,
    pub cp: u8,

    pub dp: u8,
    pub ep: u8,

    pub hp: u8,
    pub lp: u8,

    // pub ixl: u8, // bit bajo de IX
    // pub ixh: u8, // bit alto de IX
    //
    // pub iyl: u8, // bit bajo de IY
    // pub iyh: u8, // bit alto de IY

    pub ix: u16,
    pub iy: u16,

    pub sp: u16,
    pub pc: u16,

    pub iff1: u8,
    pub iff2: u8,
    pub im: u8,

}

impl Registros {
    pub fn get_f(&mut self) -> u8 {
        let f = 0;
        //dbg!(&StatusFlag::Carry);
        f | self.flag_mask(&StatusFlag::Carry)
            | self.flag_mask(&StatusFlag::AddSubtract)
            | self.flag_mask(&StatusFlag::ParityOverflow)
            | self.flag_mask(&StatusFlag::HalfCarry)
            | self.flag_mask(&StatusFlag::Zero)
            | self.flag_mask(&StatusFlag::Sign)
    }
    fn flag_mask(&self, f: &StatusFlag) -> u8 {
        match f {
            StatusFlag::Carry => 1,
            StatusFlag::AddSubtract => 1 << 1,
            StatusFlag::ParityOverflow => 1 << 2,
            // bit 3 no usado
            StatusFlag::HalfCarry => 1 << 4,
            // bit 5 no usado
            StatusFlag::Zero => 1 << 6,
            StatusFlag::Sign => 1 << 7,
        }
    }

    // Devuelve el estado de un flag haciendo mascara con registro F.
    pub fn get_flag(&self, f: &StatusFlag) -> bool {
        (self.f & self.flag_mask(f)) != 0
    }

    // Pone el estado de un flag haciendo mascara con registro F.
    pub fn set_flag(&mut self, f: &StatusFlag, set: bool) {
        if set {
            self.f |= self.flag_mask(f)
        } else {
            self.f &= !self.flag_mask(f)
        }
    }
    // Devuelve el valor del registro que le pedimos
    pub fn get_reg8(&self, r: R8) -> u8 {
        match r {
            R8::A => self.a,
            R8::B => self.b,
            R8::C => self.c,
            R8::D => self.d,
            R8::E => self.e,
            R8::F => self.f,
            R8::H => self.h,
            R8::L => self.l,
            R8::I => self.i,
            R8::R => self.r,

            R8::AP => self.ap,
            R8::BP => self.bp,
            R8::CP => self.cp,
            R8::DP => self.dp,
            R8::EP => self.ep,
            R8::FP => self.fp,
            R8::HP => self.hp,
            R8::LP => self.lp,

            R8::IFF1 => self.iff1,
            R8::IFF2 => self.iff2,
            R8::IM => self.im,

            R8::IXL => self.ix as u8, // bit bajo de IX
            R8::IXH => (self.ix >> 8) as u8, // bit alto de IX

            R8::IYL => self.iy as u8, // bit bajo de IY
            R8::IYH => (self.iy >> 8) as u8, // bit alto de IY
        }
    }

    // Set un registro de 8-bits con el valor v
    pub fn set_reg8(&mut self, r: R8, v: u8) {
        match r {
            R8::A => self.a = v,
            R8::B => self.b = v,
            R8::C => self.c = v,
            R8::D => self.d = v,
            R8::E => self.e = v,
            R8::F => self.f = v,
            R8::H => self.h = v,
            R8::L => self.l = v,
            R8::I => self.i = v,
            R8::R => self.r = v,

            R8::AP => self.ap = v,
            R8::BP => self.bp = v,
            R8::CP => self.cp = v,
            R8::DP => self.dp = v,
            R8::EP => self.ep = v,
            R8::FP => self.fp = v,
            R8::HP => self.hp = v,
            R8::LP => self.lp = v,

            R8::IFF1 => self.iff1 = v,
            R8::IFF2 => self.iff2 = v,
            R8::IM => self.im = v,

            R8::IXL => self.ix = (self.ix & 0xFF00) | v as u16, // bit bajo de IX
            R8::IXH => self.ix = (self.ix & 0x00FF) | ((v as u16) << 8), // bit alto de IX

            R8::IYL => self.iy = (self.iy & 0xFF00) | v as u16, // bit bajo de IY
            R8::IYH => self.iy = (self.iy & 0x00FF) | ((v as u16) << 8), // bit alto de IY
        }
    }

    // Obtiene el valor de una pareja de registros en un u16. Es una combinacion little-endian de 2 registros de 8 bits.
    // Ejemplo (H = 0x12, L = 0x34) r = HL => devuelve 0x3412
    pub fn get_reg16_lit_endian(&self, r: &R16) -> u16 {
        let (r0, r1) = match r {
            R16::AF => (self.a, self.f),
            R16::BC => (self.b, self.c),
            R16::DE => (self.d, self.e),
            R16::HL => (self.h, self.l),
            R16::AFP => (self.ap, self.fp),
            R16::BCP => (self.bp, self.cp),
            R16::DEP => (self.dp, self.ep),
            R16::HLP => (self.hp, self.lp),
            R16::IX => return self.ix,
            R16::IY => return self.iy,
            R16::SP => return self.sp,
        };
        (u16::from(r1) << 8) + u16::from(r0)
    }
    // Pone un valor de 16 bits en una pareja de registros (16 bits little-endian).
    // Recibe un numero en little endian
    // Ejemplo: v = 0x1234, r = HL => h = 0x34   l = 0x12
    pub fn set_reg16_lit_endian(&mut self, r: &R16, v: u16) {
        let [r0, r1] = v.to_le_bytes();
        match r {
            R16::AF => {
                self.a = r0;
                self.f = r1;
            }
            R16::BC => {
                self.b = r0;
                self.c = r1;
            }
            R16::DE => {
                self.d = r0;
                self.e = r1;
            }
            R16::HL => {
                self.h = r0;
                self.l = r1;
            }
            R16::AFP => {
                self.ap = r0;
                self.fp = r1;
            }
            R16::BCP => {
                self.bp = r0;
                self.cp = r1;
            }
            R16::DEP => {
                self.dp = r0;
                self.ep = r1;
            }
            R16::HLP => {
                self.hp = r0;
                self.lp = r1;
            }
            R16::IX => self.ix = v,
            R16::IY => self.iy = v,
            R16::SP => self.sp = v,
        }
    }

    // Obtiene el valor de una pareja de registros en un u16. Es una combinacion big-endian de 2 registros de 8 bits.
    // Ejemplo (H = 0x12, L = 0x34) r = HL => devuelve 0x1234
    pub fn get_reg16_big_endian(&self, r: &R16) -> u16 {
        let (r1, r0) = match r {
            R16::AF => (self.a, self.f),
            R16::BC => (self.b, self.c),
            R16::DE => (self.d, self.e),
            R16::HL => (self.h, self.l),
            R16::AFP => (self.ap, self.fp),
            R16::BCP => (self.bp, self.cp),
            R16::DEP => (self.dp, self.ep),
            R16::HLP => (self.hp, self.lp),
            R16::IX => return self.ix,
            R16::IY => return self.iy,
            R16::SP => return self.sp,
        };
        (u16::from(r1) << 8) + u16::from(r0)
    }
    // Pone un valor de 16 bits en una pareja de registros (16 bits big-endian).
    // Ejemplo: v = 0x1234, r = HL => h = 0x12   l = 0x34
    pub fn set_reg16_big_endian(&mut self, r: &R16, v: u16) {
        //let [r0, r1] = v.to_le_bytes();
        let [r0, r1] = v.to_be_bytes();

        match r {
            R16::AF => {
                self.a = r0;
                self.f = r1;
            }
            R16::BC => {
                self.b = r0;
                self.c = r1;
            }
            R16::DE => {
                self.d = r0;
                self.e = r1;
            }
            R16::HL => {
                self.h = r0;
                self.l = r1;
            }
            R16::AFP => {
                self.ap = r0;
                self.fp = r1;
            }
            R16::BCP => {
                self.bp = r0;
                self.cp = r1;
            }
            R16::DEP => {
                self.dp = r0;
                self.ep = r1;
            }
            R16::HLP => {
                self.hp = r0;
                self.lp = r1;
            }
            R16::IX => self.ix = v,
            R16::IY => self.iy = v,
            R16::SP => self.sp = v,
        }
    }

    // Obtiene el contador de programa actual
    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    //Cambia eñ contador de programa
    pub fn set_pc(&mut self, pc: u16) {
        self.pc = pc
    }

    pub fn get_ixh(&mut self) -> u8 {
        ((self.ix & 0xFF00) >> 8) as u8
    }
    pub fn get_ixl(&mut self) -> u8 {
        (self.ix & 0x00FF) as u8
    }

    pub fn get_iyh(&mut self) -> u8 {
        ((self.iy & 0xFF00) >> 8) as u8
    }
    pub fn get_iyl(&mut self) -> u8 {
        (self.iy & 0x00FF) as u8
    }
}