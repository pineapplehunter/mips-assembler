use crate::register::Register;

pub enum Funct {
    Add,
    Sub,
    And,
    Or,
    Slt,
    Jr,
}

impl Funct {
    pub const fn to_code(&self) -> u8 {
        match self {
            Funct::Add => 0b100000,
            Funct::Sub => 0b100010,
            Funct::And => 0b100100,
            Funct::Or => 0b100101,
            Funct::Slt => 0b101010,
            Funct::Jr => 0b001000,
        }
    }
}
pub struct Shamt(u8);

impl Shamt {
    pub const fn zero() -> Self {
        Self(0)
    }

    pub const fn value(&self) -> u8 {
        self.0
    }
}

#[derive(Debug)]
pub struct RCode(u32);

impl RCode {
    pub const fn new(rs: Register, rt: Register, rd: Register, shamt: Shamt, funct: Funct) -> Self {
        let mut output: u32 = 0;
        output += rs.value() as u32;
        output <<= 5;
        output += rt.value() as u32;
        output <<= 5;
        output += rd.value() as u32;
        output <<= 5;
        output += shamt.value() as u32;
        output <<= 6;
        output += funct.to_code() as u32;
        Self(output)
    }

    pub const fn code(&self) -> u32 {
        self.0
    }
}
