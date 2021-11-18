use crate::instruction::Instruction;

#[derive(Debug)]
pub enum Asm<'a> {
    Instruction(Instruction),
    Label { name: &'a str, offset: usize },
    Comment(&'a str),
}