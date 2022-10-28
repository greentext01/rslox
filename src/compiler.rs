use crate::{
    chunk::Chunk,
    scanner::{Scanner, Token, TokenType},
    util, vm,
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
    had_error: bool,
    previous: Option<Token>,
    current: Option<Token>,
}

impl<'a> Compiler<'a> {
    fn new(source: &'a str) -> Compiler<'a> {
        let mut scanner = Scanner::new(source);
        Compiler {
            source,
            scanner,
            had_error: false,
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
            match self.current.unwrap().token_type {
                TokenType::Error(e) => self.error_at_current(""),
                _ => break,
            }
        }
    }

    fn error_at_current(&self, message: &str) {
        if let Some(t) = self.current {
            self.error_at(t, message);
        }
    }

    fn error_at(&self, token: Token, message: &str) {
        print!("[line {}] Error", token.line);

        match token.token_type {
            TokenType::Error(m) => print!(": {m}"),
            TokenType::EOF => print!(" at end"),
            _ => print!(" at {}", token.start)
        }

        if !matches!(token.token_type, TokenType::Error(_)) {
            print!(": {message}")
        }

        util::flush_stdout();
        self.had_error = true;
    }
}
