use crate::{
    chunk::Chunk,
    scanner::{Scanner, TokenType},
    vm,
};

pub fn interpret(source: &str) -> Result<(), &'static str> {
    let mut compiler = Compiler::new(source);
    let chunk = compiler.compile()?;

    vm::VM::new(chunk);
    Ok(())
}

struct Compiler<'a> {
    source: &'a str,
    scanner: Scanner<'a>,
    previous: usize,
    current: usize,
}

impl<'a> Compiler<'a> {
    fn new(source: &'a str) -> Compiler<'a> {
        let mut scanner = Scanner::new(source);
        Compiler {
            source,
            scanner,
            previous: 0,
            current: 0,
        }
    }

    fn compile(&mut self) -> Result<Chunk, &'static str> {
        self.advance();
        self.expression();
        self.consume(TokenType::EOF, "Expected end of expression.")
    }

    fn advance(&mut self) {
        self.previous = self.current;
        loop {
            self.current = self.scanner.scan_token();
        }
    }
}
