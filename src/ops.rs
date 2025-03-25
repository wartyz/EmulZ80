use std::fmt;

/// Op represents a single operation.
/// This representation (and backing implementation) is more expressive than
/// the processor itself.
/// For example `ADD8(Location8::Reg(Reg8::D), Location8::Immediate(10))` is a valid representation, but
/// the Z80 features no such instruction.
/// Usually executing an instruction like this will just work, but in some cases a panic will occur
/// (Such as attempting to store to an immediate, which doesn't make any sense).
/// It is probably best to stick to the "guide rails" of the Z80 operations.
#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    ADC8(Local8, Local8), // Suma incluyendo Carry

    ADC8BIG(Local8, Local8), // Suma incluyendo Carry

    ADD8(Local8, Local8), // ADD (8-bit)

    ADD8BIG(Local8, Local8), // ADD (8-bit)

    //ADC16(Local16, Local16), // puesto por mi

    ADC16BIG(Local16, Local16), // ADd including Carry

    ADD16(Local16, Local16), // ADD (16-bit) (puesto por mi)

    ADD16BIG(Local16, Local16), // ADD (16-bit) (puesto por mi)

    INC8(Local8), // INCrement

    INC8BIG(Local8), // INCrement

    INC16(Local16, Local16), // INCrement puesto por mi

    INC16BIG(Local16, Local16), // INCrement puesto por mi

    SBC(Local8, Local8), // Resta de 8 bits incluyendo carry anterior

    SBCBIG(Local8, Local8), // Resta de 8 bits incluyendo carry anterior

    SBC16BIG(Local16, Local16), // Resta de 16 bits incluyendo carry anterior

    SUB8(Local8, Local8), // SUBtraction (8-bit)

    SUB8BIG(Local8, Local8), // SUBtraction (8-bit)

    DEC8(Local8), // Decrementa un valor de 8 bits

    DEC8BIG(Local8), // Decrementa un valor de 8 bits

    DEC8INDEX(Local8, i8), // DECrement de registros IX e IY

    DEC16(Local16, Local16), // Decrementa (16 bits) (puesto por mi)

    DEC16BIG(Local16, Local16), // Decrementa (16 bits) (puesto por mi)

    AND(Local8), // bitwise AND

    ANDBIG(Local8), // bitwise AND

    OR(Local8), // bitwise OR

    ORBIG(Local8), // bitwise OR

    XOR(Local8), // bitwise XOR

    XORBIG(Local8), // bitwise XOR

    CP(Local8), // two's ComPliment

    CPBIG(Local8), // two's ComPliment

    CPL, // One's Compliment

    NEG, // Negacion (complemento a 2)

    CCF, // toggle carry flag

    SCF, // Set the Carry Flag unconditionally

    NOP, // No hacer nada (No-OPeration)

    HALT, // End execution (until woken)

    DAA, // BCD nonsense. Not implemented

    RLCA, // Rotate Accumulator Left, set Carry

    RLA, // Rotate Accumulator Left, through carry

    RRCA, // Rotate Accumulator Right, set Carry

    RRA, // Rotate Accumulator Left, through carry

    RLC(Local8), // Rota a la izquierda y pone en carry

    RLCBIG(Local8), // Rota a la izquierda y pone en carry

    RL(Local8), // Rotate Left, through carry

    RLBIG(Local8), // Rotate Left, through carry

    RRC(Local8), // Rota a la derecha y pone en carry

    RRCBIG(Local8), // Rota a la derecha y pone en carry

    RR(Local8), // Rotate Right, through carry

    RRBIG(Local8), // Rotate Right, through carry

    SLA(Local8), // Shift Left

    SLABIG(Local8), // Shift Left

    SRL(Local8), // Shift Right

    SRLBIG(Local8), // Shift Right

    SRA(Local8), // Shift Right, preserving 7th bit

    SRABIG(Local8), // Shift Right, preserving 7th bit

    RLD, // Rotate nibbles Left through accumulator

    RRD, // Rotate nibbles Right through accumulator

    BIT(u8, Local8), // set zero flag if BIT is on

    SETBIG(u8, Local8), // SET b bit in location

    RESBIG(u8, Local8), // RESet b bit in location

    IN(Local8, Local8), // INput from a peripheral

    INFLAGS(Local8, Local8), // INput de un periferico que afecta a los flga

    OUTBE(Local8, Local8), // OUTput to a peripheral

    JP(SaltoCondicional, Local16), // JumP a la posicion dada segun una condicion

    JPHL(Local16), // JumP especial 0xE9 jp (hl) a la posicion dada

    JR(SaltoCondicional, i8), // Jump to the given Relative position

    DJNZ(i8), // Decrement register b, then Jump if register b is Non Zero

    CALL(SaltoCondicional, u16), // CALL a method

    RET(SaltoCondicional), // RETurn from a method call

    RETBIG(SaltoCondicional), // RETurn from a method call

    POP(Local16), // Pop an address off of the stack

    PUSH(Local16), // Push an address onto a stack

    //LD8(Local8, Local8), // LoaD the given address (8-bit)

    LD8R(u8), // Solo para ED 5F   ld a,r y  ED 57   ld a,i     ya que afecta alos flags (8-bit)

    LD8BIG(Local8, Local8), // LoaD the given address (8-bit)

    //LD16(Local16, Local16), // LoaD the given address (16-bit)

    LD16BIG(Local16, Local16), // Carga una direccion pero el resultado es big endian (16-bit)

    // ***** Incompletas *****************
    CPD(Local8, Local8),
    CPDR,
    CPI,
    CPIR,
    DI,
    EI,

    EX16, // Intercambia registros de 16 bits
    EXX,
    IM(Option<u8>),
    IND,
    INDR,
    INI,
    INIR,
    LDD,
    LDDR,
    LDIR,
    LDI,
    OTDR,
    OTIR,
    OUTD,
    OUTI,
    RETI,
    RETN,

    RST(Local16), // Salto a una pagina de memoria

    RSTBIG(Local16), // Salto a una pagina de memoria

    // SLA,

    SLL(Local8),
    SLLBIG(Local8),

    SL1,
    // SRA,
    // SRL,
    //DEC16(Location16), // puesto por mi
    EXSPHL, // puesto por mi
    EXSPIX, // puesto por mi
    EXSPIY, // puesto por mi
    EX, // puesto por mi
    MLT(Local16), //puesto por mi para instrucciones mlt rr (0xED 0x4C por ejemplo)

}

// Cualquier valor de 8 bits puede venir desde un lugar o ir a un lugar
#[derive(Debug, PartialEq, Clone)]
pub enum Local8 {
    // Viene o va a un registro (8 bits)
    Reg8(R8),
    // Viene o va a una zona de memoria apuntada por un registro de 16 bits
    RegIndirecto8(R16),
    // Una localizacion en memoria apuntada por un numero literal
    InmediatoIndirecto8(u16),
    // un numero literal
    Inmediato8(u8),
    // Direccionamiento usado en IX e IY, un dato de 8 bits que esta en
    //una direccion de memoria apuntada por la suma
    // del registro indice (IX o IY) y un desplazamiento con signo de 8 bits
    // i8: Desplazamiento con signo de 8 bits (rango: -128 a +127).
    Indexado8(R16, i8),
}
// Cualquier valor de 16 bits puede venir desde un lugar o ir a un lugar
#[derive(Debug, PartialEq, Clone)]
pub enum Local16 {
    // Una pareja de registros que hacen 16 bits
    Reg(R16),
    // Viene o va a una zona de memoria apuntada por un registro de 16 bits
    RegIndirecto(R16),
    // A location in memory, pointed to by a literal number
    InmediatoIndirecto(u16),
    // un numero literal
    Inmediato(u16),
    // Direccionamiento usado en IX e IY, un dato de 16 bits que esta en
    // una direccion de memoria apuntada por la suma
    // del registro indice (IX o IY) y un desplazamiento con signo de 8 bits
    // i8: Desplazamiento con signo de 8 bits (rango: -128 a +127).
    Indexado(R16, i8),
}

// Enum que identifica a los registros de 8 bits
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum R8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    // Interrupciones
    I,
    // Registro R ???????????
    R,
    // A'
    AP,
    // F'
    FP,
    // B'
    BP,
    // C'
    CP,
    // D'
    DP,
    // E'
    EP,
    // H'
    HP,
    // L'
    LP,
    IFF1,
    IFF2,
    IM,

    // Inventados para probar en IX e IY
    IXL,
    IXH,
    IYL,
    IYH,
}

// Implementar manualmente el trait `Display` para `Reg8`
impl fmt::Display for R8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            R8::A => write!(f, "A"),
            R8::F => write!(f, "F"),
            R8::B => write!(f, "B"),
            R8::C => write!(f, "C"),
            R8::D => write!(f, "D"),
            R8::E => write!(f, "E"),
            R8::H => write!(f, "H"),
            R8::L => write!(f, "L"),
            R8::I => write!(f, "I"),
            R8::R => write!(f, "R"),
            R8::AP => write!(f, "A'"),
            R8::FP => write!(f, "F'"),
            R8::BP => write!(f, "B'"),
            R8::CP => write!(f, "C'"),
            R8::DP => write!(f, "D'"),
            R8::EP => write!(f, "E'"),
            R8::HP => write!(f, "H'"),
            R8::LP => write!(f, "L'"),

            R8::IFF1 => write!(f, "FF1"),
            R8::IFF2 => write!(f, "FF2"),
            R8::IM => write!(f, "IM"),

            R8::IXH => write!(f, "IXH"),
            R8::IXL => write!(f, "IXL"),
            R8::IYH => write!(f, "IXH"),
            R8::IYL => write!(f, "IXL"),
        }
    }
}

// Registros de 16-bits
#[derive(Debug, PartialEq, Clone)]
pub enum R16 {
    AF,
    BC,
    DE,
    HL,
    // AF'
    AFP,
    // BC'
    BCP,
    // DE'
    DEP,
    // HL'
    HLP,

    IX,
    IY,
    // Stack Pointer
    SP,
}
// Implementar manualmente el trait `Display` para `Reg16`
impl fmt::Display for R16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            R16::AF => write!(f, "AF"),
            R16::BC => write!(f, "BC"),
            R16::DE => write!(f, "DE"),
            R16::HL => write!(f, "HL"),
            R16::AFP => write!(f, "AF'"),
            R16::BCP => write!(f, "BC'"),
            R16::DEP => write!(f, "DE'"),
            R16::HLP => write!(f, "HL'"),
            R16::IX => write!(f, "IX"),
            R16::IY => write!(f, "IY"),
            R16::SP => write!(f, "SP"),
        }
    }
}

// Status Flags. Implementado como campo de bits en el registro F
#[derive(Debug, PartialEq, Clone)]
pub enum StatusFlag {
    // C -> Bit 0. Indica acarreo o toma prestado del bit 7
    Carry,
    // N -> Bit 1. Generalmente 0 después de la suma, 1 después de la resta
    AddSubtract,
    // P/V - > Bit 2. Indica desbordamiento después de la aritmética, o paridad después de operaciones bit a bit
    // La paridad se establece si la cantidad de 1 en el número es par, de lo contrario se restablece
    ParityOverflow,
    // Bit 3 sin usar
    // H -> Bit 4. Indica acarreo o toma prestado del bit 3
    HalfCarry,
    // Z -> Bit 6. Se establece si el resultado de una operación fue cero
    Zero,
    // S -> Bit 7. Se establece si el séptimo bit es 1 después de una operación aritmética, es decir, el número es negativo si se considera con signo
    Sign,

}

// Jumps y Returns pueden ser condicionales cuando ciertos flags estan puestos
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SaltoCondicional {
    // Jump y Return tienen versiones incondicionales.
    // Rather than a seperate Op, these will have this flag.
    // Siempre se evalua como true
    Incondicional,
    // True si el flag Zero es reset
    NoCero,
    // True si el flag Zero es set
    Cero,
    // True si el flag Carry es reset
    NoCarry,
    // True si el flag Carry es set
    Carry,
    // True si el bit ParityOverflow es reset
    ParidadImpar,
    // True si el bit ParityOverflow es set
    ParidadPar,
    // True si el bit Sign es reset
    SignoPositivo,
    // True si el bit Sign is set
    SignoNegativo,
}