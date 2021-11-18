use core::fmt;

use crate::register::Register;

pub mod icode;
pub mod jcode;
pub mod rcode;

#[derive(Debug)]
pub enum Instruction {
    RCode(rcode::RCode),
    ICode(icode::ICode),
    JCode(jcode::JCode),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:08X}",
            match self {
                Instruction::RCode(code) => code.code(),
                Instruction::ICode(code) => code.code(),
                Instruction::JCode(code) => code.code(),
            }
        )
    }
}

impl Instruction {
    pub const fn add(rd: Register, rs: Register, rt: Register) -> Self {
        Self::RCode(rcode::RCode::new(
            rs,
            rt,
            rd,
            rcode::Shamt::zero(),
            rcode::Funct::Add,
        ))
    }

    pub const fn sub(rd: Register, rs: Register, rt: Register) -> Self {
        Self::RCode(rcode::RCode::new(
            rs,
            rt,
            rd,
            rcode::Shamt::zero(),
            rcode::Funct::Sub,
        ))
    }

    pub const fn and(rd: Register, rs: Register, rt: Register) -> Self {
        Self::RCode(rcode::RCode::new(
            rs,
            rt,
            rd,
            rcode::Shamt::zero(),
            rcode::Funct::And,
        ))
    }

    pub const fn or(rd: Register, rs: Register, rt: Register) -> Self {
        Self::RCode(rcode::RCode::new(
            rs,
            rt,
            rd,
            rcode::Shamt::zero(),
            rcode::Funct::Or,
        ))
    }

    pub const fn slt(rd: Register, rs: Register, rt: Register) -> Self {
        Self::RCode(rcode::RCode::new(
            rs,
            rt,
            rd,
            rcode::Shamt::zero(),
            rcode::Funct::Slt,
        ))
    }

    pub const fn jr(rd: Register, rs: Register, rt: Register) -> Self {
        Self::RCode(rcode::RCode::new(
            rs,
            rt,
            rd,
            rcode::Shamt::zero(),
            rcode::Funct::Jr,
        ))
    }

    pub const fn lw(rt: Register, rs: Register, address: i16) -> Self {
        Self::ICode(icode::ICode::new(icode::Op::Lw, rs, rt, address))
    }

    pub const fn sw(rt: Register, rs: Register, address: i16) -> Self {
        Self::ICode(icode::ICode::new(icode::Op::Sw, rs, rt, address))
    }

    pub const fn beq(rt: Register, rs: Register, address: i16) -> Self {
        Self::ICode(icode::ICode::new(icode::Op::Beq, rs, rt, address))
    }

    pub const fn bne(rt: Register, rs: Register, address: i16) -> Self {
        Self::ICode(icode::ICode::new(icode::Op::Bne, rs, rt, address))
    }

    pub const fn addi(rt: Register, rs: Register, address: i16) -> Self {
        Self::ICode(icode::ICode::new(icode::Op::Addi, rs, rt, address))
    }

    pub const fn slti(rt: Register, rs: Register, address: i16) -> Self {
        Self::ICode(icode::ICode::new(icode::Op::Slti, rs, rt, address))
    }

    pub const fn lui(rt: Register, rs: Register, address: i16) -> Self {
        Self::ICode(icode::ICode::new(icode::Op::Lui, rs, rt, address))
    }

    pub fn j(address: u32) -> Self {
        Self::JCode(jcode::JCode::new(
            jcode::Op::J,
            jcode::Address::new(address).unwrap(),
        ))
    }

    pub fn jal(address: u32) -> Self {
        Self::JCode(jcode::JCode::new(
            jcode::Op::Jal,
            jcode::Address::new(address).unwrap(),
        ))
    }
}

impl From<rcode::RCode> for Instruction {
    fn from(code: rcode::RCode) -> Self {
        Self::RCode(code)
    }
}

impl From<icode::ICode> for Instruction {
    fn from(code: icode::ICode) -> Self {
        Self::ICode(code)
    }
}

impl From<jcode::JCode> for Instruction {
    fn from(code: jcode::JCode) -> Self {
        Self::JCode(code)
    }
}

#[cfg(test)]
mod test {
    use super::super::register::consts::*;
    use super::Instruction;

    #[test]
    fn test_instructions() {
        let inst = Instruction::add(R1, R2, R0);
        assert_eq!(format!("{}", inst), "00400820");

        let inst = Instruction::add(R1, R0, R0);
        assert_eq!(format!("{}", inst), "00000820");

        let inst = Instruction::lw(R8, R19, 100);
        assert_eq!(format!("{}", inst), "8E680064");

        let inst = Instruction::sw(R5, R11, 255);
        assert_eq!(format!("{}", inst), "AD6500FF");
    }
}
