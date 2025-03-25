use super::util::reg_bits;
use crate::ops::Op;
// CB
pub fn analiza_bits(op: u8) -> (Op, usize) {
    let loc = reg_bits(op);
    let opr = match op >> 6 {
        0b00 => {
            let opr = match op >> 3 {
                0b000 => Op::RLCBIG,
                0b001 => Op::RRCBIG,
                0b010 => Op::RLBIG,
                0b011 => Op::RRBIG,
                0b100 => Op::SLABIG,
                0b101 => Op::SRABIG,
                // http://z80-heaven.wikidot.com/instructions-set:sll
                //0b110 => panic!("Uso de una instruccion no documentada SLL"),
                0b110 => Op::SLLBIG,
                0b111 => Op::SRLBIG,
                _ => unreachable!(),
            };
            return (opr(loc), 2);
        }
        0b01 => Op::BIT,
        0b10 => Op::RESBIG,
        0b11 => Op::SETBIG,
        _ => unreachable!(),
    };
    let reg = (op >> 3) & 0b111;
    (opr(reg, loc), 2)
}
