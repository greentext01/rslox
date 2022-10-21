use num_traits::{FromPrimitive, ToPrimitive};

use crate::{chunk::Chunk, opcode::OpCode, values::Value};

enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

struct VM {
    chunk: Chunk,
    ip: u8,
}

impl VM {
    fn new(chunk: Chunk) -> VM {
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

    fn read_constant(&mut self) -> Result<u8, &'static str> {
        let constant_ref = self.read_byte()?;
        match self.chunk.instructions.get(constant_ref as usize) {
            None => Err("Invalid constant"),
            Some(c) => Ok(*c),
        }
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = match self.read_byte() {
                Err(_) => return InterpretResult::InterpretRuntimeError,
                Ok(instruction) => instruction,
            };
            match FromPrimitive::from_u8(instruction) {
                Some(OpCode::OpReturn) => return InterpretResult::InterpretOk,
                Some(OpCode::OpConstant) => {
                    let constant = match self.read_constant() {
                        Ok(constant) => constant,
                        Err(_) => return InterpretResult::InterpretRuntimeError,
                    };
                    println!("{}", constant);
                    return InterpretResult::InterpretOk;
                }
                None => {}
            }
        }

        InterpretResult::InterpretOk
    }
}
