use num_traits::{FromPrimitive};

use crate::{chunk::Chunk, opcode::OpCode, instructions::Instructions};

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

pub struct VM {
    chunk: Chunk,
    ip: u8,
}

impl VM {
    pub fn new(chunk: Chunk) -> VM {
        VM { chunk, ip: 0 }
    }

    fn read_byte(&mut self) -> Result<u8, &'static str> {
        let instruction = match self.chunk.instructions.get(self.ip as usize) {
            None => Err(""),
            Some(instruction) => Ok(*instruction),
        };
        self.ip += 1;
        return instruction;
    }

    fn read_constant(&mut self) -> Result<f64, &'static str> {
        let constant_ref = self.read_byte()?;
        match self.chunk.constants.get(constant_ref as usize) {
            None => Err("Invalid constant"),
            Some(c) => Ok(*c),
        }
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            if cfg!(debug_assertions) {
                self.chunk.disassemble_instruction(&(self.ip as usize));
            }

            let instruction = match self.read_byte() {
                Err(_) => return InterpretResult::InterpretCompileError,
                Ok(instruction) => instruction,
            };

            match FromPrimitive::from_u8(instruction) {
                Some(OpCode::OpReturn) => break,
                Some(OpCode::OpConstant) => {
                    let constant = match self.read_constant() {
                        Ok(constant) => constant,
                        Err(_) => return InterpretResult::InterpretCompileError,
                    };
                    println!("Constant: {}", constant);
                }
                None => break
            }
        }

        InterpretResult::InterpretOk
    }
}
