use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct Register(u8);

impl Register {
    pub const fn new(reg: u8) -> Option<Self> {
        if reg < 32 {
            Some(Register(reg))
        } else {
            None
        }
    }

    pub const fn value(&self) -> u8 {
        self.0
    }
}
#[derive(Debug, Error)]
pub enum RegisterError {
    #[error("The register specified was out of range. registers have to be in range 0..32. {0} was givem")]
    OutOfRange(u8),
}

impl TryFrom<u8> for Register {
    type Error = RegisterError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Register::new(value).ok_or(RegisterError::OutOfRange(value))
    }
}

pub mod consts {
    use super::Register;
    macro_rules! define_register {
        ($name:ident,$value:expr) => {
            pub const $name: Register = Register($value);
        };
    }

    define_register!(R0, 0);
    define_register!(R1, 1);
    define_register!(R2, 2);
    define_register!(R3, 3);
    define_register!(R4, 4);
    define_register!(R5, 5);
    define_register!(R6, 6);
    define_register!(R7, 7);
    define_register!(R8, 8);
    define_register!(R9, 9);
    define_register!(R10, 10);
    define_register!(R11, 11);
    define_register!(R12, 12);
    define_register!(R13, 13);
    define_register!(R14, 14);
    define_register!(R15, 15);
    define_register!(R16, 16);
    define_register!(R17, 17);
    define_register!(R18, 18);
    define_register!(R19, 19);
    define_register!(R20, 20);
    define_register!(R21, 21);
    define_register!(R22, 22);
    define_register!(R23, 23);
    define_register!(R24, 24);
    define_register!(R25, 25);
    define_register!(R26, 26);
    define_register!(R27, 27);
    define_register!(R28, 28);
    define_register!(R29, 29);
    define_register!(R30, 30);
    define_register!(R31, 31);
}
