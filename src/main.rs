use mips_assembler::instruction::Instruction;
use mips_assembler::register::consts::*;

fn main() {
    let initial = 0x2903bc84;
    let j_address = ((initial >> 2) + 2) & 0x03FF_FFFF;

    let instructions: Vec<Instruction> = vec![
        Instruction::add(R20, R0, R0),
        Instruction::add(R21, R0, R0),
        Instruction::slti(R20, R20, 51),
        Instruction::bne(R22, R0, 5),
        Instruction::add(R22, R0, R7),
        Instruction::lw(R22, R22, 0),
        Instruction::add(R21, R21, R22),
        Instruction::addi(R20, R20, 1),
        Instruction::j(j_address),
    ];

    for instruction in instructions {
        println!("{}", instruction);
    }
}
