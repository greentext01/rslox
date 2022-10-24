use num_traits::FromPrimitive;

use crate::{chunk::Chunk, opcode::OpCode, util};

pub trait Instructions {
    fn simple_instruction(&self, name: &str, offset: &usize) -> usize;

    fn invalid_instruction(&self, instruction: &u8, offset: &usize) -> usize;

    fn disassemble_instruction(&self, offset: &usize) -> usize;

    fn constant_instruction(&self, name: &str, offset: &usize) -> usize;
}

impl Instructions for Chunk {
    fn simple_instruction(&self, name: &str, offset: &usize) -> usize {
        println!("{name}");
        offset + 1
    }

    fn invalid_instruction(&self, instruction: &u8, offset: &usize) -> usize {
        panic!("Invalid instruction \"{instruction}\" at offset {offset}");
    }

    fn constant_instruction(&self, name: &str, offset: &usize) -> usize {
        let constant = self.instructions[offset + 1];
        println!(
            "{name} {constant}' {values}",
            values = self.constants[constant as usize]
        );
        offset + 2
    }

    fn disassemble_instruction(&self, offset: &usize) -> usize {
        print!("{offset:0>4} ");
        if *offset > 0 && self.lines[*offset as usize] == self.lines[offset - 1] {
            print!("   | ")
        } else {
            print!("{line:0>4} ", line=self.lines[*offset])
        }

        util::flush_stdout();
        let instruction = self.instructions[*offset];
        match FromPrimitive::from_u8(instruction) {
            Some(OpCode::OpReturn) => self.simple_instruction("return", offset),
            Some(OpCode::OpConstant) => self.constant_instruction("constant", offset),
            Some(OpCode::OpNegate) => self.simple_instruction("negate", offset),
            Some(OpCode::OpAdd) => self.simple_instruction("add", offset),
            Some(OpCode::OpSubstract) => self.simple_instruction("substract", offset),
            Some(OpCode::OpMultiply) => self.simple_instruction("multiply", offset),
            Some(OpCode::OpDivide) => self.simple_instruction("divide", offset),
            None => self.invalid_instruction(&instruction, offset),
        }
    }
}
