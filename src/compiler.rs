use num_traits::ToPrimitive;

use crate::{
    chunk::Chunk,
    scanner::{Scanner, Token, TokenType},
    util, vm, opcode::OpCode,
};

pub fn interpret(source: &str) -> Result<(), &'static str> {
    let mut compiler = Compiler::new(source);
    compiler.compile();
    
    vm::VM::new(compiler.chunk);
    Ok(())
}

struct Compiler<'a> {
    source: &'a str,
    scanner: Scanner<'a>,
    had_error: bool,
    panic: bool,
    previous: Token,
    current: Token,
    pub chunk: Chunk,
}

impl<'a> Compiler<'a> {
    fn new(source: &'a str) -> Compiler<'a> {
        let scanner = Scanner::new(source);
        Compiler {
            source,
            scanner,
            chunk: Chunk::new(),
            panic: false,
            had_error: false,
            previous: Token::placeholder(),
            current: Token::placeholder(),
        }
    }

    fn compile(&mut self) -> bool {
        self.advance();
        self.expression();
        self.consume(TokenType::EOF);
        !self.had_error
    }

    fn emit_byte(&mut self, byte: u8) {
        let line = self.previous.line;
        self.compiling_chunk().add_instruction(&byte, line);
    }

    fn end_compiler(&mut self) {
        self.emit_return();
    }
    

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn as u8);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn expression(&mut self) {
        
    }

    fn number(&mut self) {
        let start = self.previous.start;
        let len = self.previous.length as usize;
        let value_str = &self.source[start..start + len];
        let value: f64 = value_str.parse().unwrap();
        
    }

    fn compiling_chunk(&mut self) -> &mut Chunk {
        &mut self.chunk
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();
        loop {
            let token = self.scanner.scan_token();
            self.current = token;
            match self.current.token_type {
                TokenType::Error => self.error_at_current(""),
                _ => break,
            }
        }
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.current.clone(), message);
    }

    fn consume(&mut self, tt: TokenType) {
        if self.current.token_type == tt {
            self.advance()
        } else {
            let message = format!("Expected {tt:?}, found, {:?}", self.current.token_type);

            self.error_at_current(message.as_str());
        }
    }

    fn error_at(&mut self, token: Token, message: &str) {
        if self.panic {
            return;
        }

        self.had_error = true;
        self.panic = true;

        print!("[line {}] Error", token.line);

        match token.token_type {
            TokenType::Error => print!(": "),
            TokenType::EOF => print!(" at end"),
            _ => print!(" at {}", token.start),
        }

        print!(": {message}");

        util::flush_stdout();
        self.had_error = true;
    }
}
