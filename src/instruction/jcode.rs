pub enum Op {
    J,
    Jal,
}

impl Op {
    pub const fn to_code(&self) -> u8 {
        match self {
            Op::J => 0b000010,
            Op::Jal => 0b000011,
        }
    }
}

#[derive(Debug)]
pub struct JCode(u32);

pub struct Address(u32);

impl Address {
    pub const fn new(address: u32) -> Option<Self> {
        if address < 0x0400_0000 {
            Some(Self(address as u32))
        } else {
            None
        }
    }

    pub const fn value(&self) -> u32 {
        self.0
    }
}

impl JCode {
    pub const fn new(op: Op, address: Address) -> Self {
        let mut output: u32 = 0;
        output += op.to_code() as u32;
        output <<= 26;
        output += address.value();
        Self(output)
    }

    pub const fn code(&self) -> u32 {
        self.0
    }
}
