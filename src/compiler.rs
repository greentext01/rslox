use crate::{
    chunk::Chunk,
    scanner::{Scanner, TokenType, Token},
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
    previous: Option<Token>,
    current: Option<Token>,
}

impl<'a> Compiler<'a> {
    fn new(source: &'a str) -> Compiler<'a> {
        let mut scanner = Scanner::new(source);
        Compiler {
            source,
            scanner,
            previous: None,
            current: None,
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
            self.current = Some(self.scanner.scan_token());
            if matches!(self.current., Some(TokenType::Error(_))) {

            }
        }
    }
}
