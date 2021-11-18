use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::{map_opt, map_res},
    multi::many0,
    sequence::tuple,
    IResult,
};

use crate::{instruction::Instruction, register::Register};

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn register(input: &str) -> IResult<&str, Register> {
    let (input, _) = tag("$")(input)?;
    map_opt(
        map_res(take_while_m_n(1, 2, is_digit), |s| {
            u8::from_str_radix(s, 10)
        }),
        Register::new,
    )(input)
}

fn consume_white_and_camma(input: &str) -> IResult<&str, ()> {
    let (input, _) = tuple((many0(tag(" ")), tag(","), many0(tag(" "))))(input)?;
    Ok((input, ()))
}

fn get3_reg_args(input: &str) -> IResult<&str, (Register, Register, Register)> {
    let (input, (_, r1, _, r2, _, r3)) = tuple((
        many0(tag(" ")),
        register,
        consume_white_and_camma,
        register,
        consume_white_and_camma,
        register,
    ))(input)?;
    Ok((input, (r1, r2, r3)))
}

pub fn add(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("add ")(input)?;
    let (input, (rd, rs, rt)) = get3_reg_args(input)?;
    Ok((input, Instruction::add(rd, rs, rt)))
}

pub fn sub(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("sub ")(input)?;
    let (input, (rd, rs, rt)) = get3_reg_args(input)?;
    Ok((input, Instruction::sub(rd, rs, rt)))
}
