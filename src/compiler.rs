use num_traits::ToPrimitive;

use crate::{
    chunk::Chunk,
    scanner::{Scanner, Token, TokenType},
    util, vm, opcode::OpCode, values::Value, precedence,
};

pub fn interpret(source: &str) -> Result<(), &'static str> {
    let mut compiler = Compiler::new(source);
    compiler.compile();
    
    vm::VM::new(compiler.chunk);
    Ok(())
}

enum Precedence {
    None,
    Assignment,  // =
    Or,          // or
    And,         // and
    Equality,    // == !=
    Comparison,  // < > <= >=
    Term,        // + -
    Factor,      // * /
    Unary,       // ! -
    Call,        // . ()
    Primary
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

    fn compiling_chunk(&self) -> Chunk {
        self.chunk
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
        self.parse_precedence(Precedence::Assignment);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        
    }

    fn number(&mut self) {
        let start = self.previous.start;
        let len = self.previous.length as usize;
        let value_str = &self.source[start..start + len];
        let value: f64 = value_str.parse().unwrap();
        self.emit_constant(&value);
    }

    fn emit_constant(&mut self, value: &Value) -> usize {
        self.compiling_chunk().add_constant(value)
    }
    
    fn make_constant(&mut self, value: &Value) -> Result<(), &'static str> {
        let constant = self.emit_constant(value);
        if constant > u8::MAX as usize {
            self.error("Too many constants in one chunk.");
            return Err("Too many constants in one chunk");
        }

        Ok(())
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();
        loop {
            self.current = self.scanner.scan_token();
            if self.current.token_type != TokenType::Error {
                break;
            }

            self.error_at_current(self.current.start.to_string().as_str());
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume_msg(TokenType::RightParen, "Expected '(' after expression.");
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

    fn unary(&mut self) {
        let op_type = self.current.token_type;
        self.expression();
        match op_type {
            TokenType::Minus => self.emit_byte(OpCode::OpNegate as u8),
            _ => {
                // Unreachable. 
            }
        }
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.current.clone(), message);
    }

    fn error(&mut self, message: &str) {
        self.error_at(self.previous.clone(), message);
    }

    fn consume(&mut self, tt: TokenType) {
        let default = format!("Expected {tt:?}, found, {:?}", self.current.token_type);
        self.consume_msg(tt, &default);
    }

    fn consume_msg(&mut self, tt: TokenType, message: &str) {
        if self.current.token_type == tt {
            self.advance()
        } else {
            self.error_at_current(message);
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
