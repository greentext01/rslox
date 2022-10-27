use std::usize;

use crate::instructions::Instructions;
use crate::values::Value;

#[derive(Debug)]
pub struct Chunk {
    pub(super) instructions: Vec<u8>,
    pub(super) constants: Vec<Value>,
    pub(super) lines: Vec<i32>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            instructions: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, bytes: &u8, line: i32) {
        self.instructions.push(*bytes);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, constant: &Value) -> usize {
        self.constants.push(*constant);
        self.constants.len() - 1
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        let mut offset = 0;
        while offset < self.instructions.len() {
            offset = self.disassemble_instruction(&offset);
        }
        println!("========");
    }
}
