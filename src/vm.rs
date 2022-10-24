use num_traits::FromPrimitive;

use crate::{chunk::Chunk, instructions::Instructions, opcode::OpCode, values::Value};

macro_rules! binary_op {
    ($slf:ident, $operator:tt) => {
        {
            let operand2 = $slf.stack.pop().expect("Second operand not found");
            let operand1 = $slf.stack.pop().expect("First operand not found");
            $slf.stack.push(operand1 $operator operand2);
        }
    };
}

pub struct VM {
    chunk: Chunk,
    ip: u8,
    stack: Vec<Value>,
}

impl VM {
    pub fn new(chunk: Chunk) -> VM {
        VM {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    fn read_byte(&mut self) -> Option<u8> {
        let instruction = *self.chunk.instructions.get(self.ip as usize)?;
        self.ip += 1;
        return Some(instruction);
    }

    fn read_constant(&mut self) -> Option<&Value> {
        let constant_ref = self.read_byte()?;
        Some(
            self.chunk
                .constants
                .get(constant_ref as usize)
                .expect("Invalid constant index"),
        )
    }

    pub fn run(&mut self) {
        loop {
            if cfg!(debug_assertions) && self.chunk.instructions.len() > self.ip as usize {
                println!("{:?}", self.stack);
                self.chunk.disassemble_instruction(&(self.ip as usize));
            }

            let instruction = match self.read_byte() {
                Some(i) => i,
                None => break,
            };

            match FromPrimitive::from_u8(instruction) {
                Some(OpCode::OpReturn) => {
                    match self.stack.pop() {
                        Some(v) => println!("{}", v),
                        None => {}
                    }
                    return;
                }
                Some(OpCode::OpConstant) => {
                    let constant = *self.read_constant().expect("Can't read constant");
                    self.stack.push(constant);
                }
                Some(OpCode::OpNegate) => {
                    let negated = -self.stack.pop().expect("No number to negate");
                    self.stack.push(negated);
                }
                Some(OpCode::OpAdd) => binary_op!(self, +),
                Some(OpCode::OpSubstract) => binary_op!(self, -),
                Some(OpCode::OpMultiply) => binary_op!(self, *),
                Some(OpCode::OpDivide) => binary_op!(self, /),
                None => panic!("Invalid instruction {instruction}"),
            };
        }
    }
}
