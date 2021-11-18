use crate::register::Register;

pub enum Op {
    Lw,
    Sw,
    Beq,
    Bne,
    Addi,
    Slti,
    Lui,
}

impl Op {
    pub const fn to_code(&self) -> u8 {
        match self {
            Op::Lw => 0b100011,
            Op::Sw => 0b101011,
            Op::Beq => 0b000100,
            Op::Bne => 0b000101,
            Op::Addi => 0b001000,
            Op::Slti => 0b001010,
            Op::Lui => 0b001111,
        }
    }
}

#[derive(Debug)]
pub struct ICode(u32);

impl ICode {
    pub const fn new(op: Op, rs: Register, rt: Register, address: i16) -> Self {
        let mut output: u32 = 0;
        output += op.to_code() as u32;
        output <<= 5;
        output += rs.value() as u32;
        output <<= 5;
        output += rt.value() as u32;
        output <<= 16;
        output += address as u32;
        Self(output)
    }

    pub const fn code(&self) -> u32 {
        self.0
    }
}