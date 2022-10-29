use crate::{
    chunk::Chunk,
    scanner::{Scanner, Token, TokenType},
    util, vm,
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
    previous: &'a Option<Token>,
    current: &'a Option<Token>,
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
            previous: &None,
            current: &None,
        }
    }

    fn compile(&mut self) -> bool {
        self.advance();
        //self.expression();
        //self.consume(TokenType::EOF, "Expected end of expression.");
        !self.had_error
    }

    fn advance(&mut self) {
        self.previous = self.current;
        loop {
            let token = self.scanner.scan_token();
            self.current = token;
            match self.current.unwrap().token_type {
                TokenType::Error => self.error_at_current(""),
                _ => break,
            }
        }
    }

    fn error_at_current(&mut self, message: &str) {
        if let Some(t) = self.current {
            self.error_at(t, message);
        }
    }

    fn consume(&mut self, tt: TokenType) {
        match self.current.unwrap() {
            tt => self.advance(),
            _ => {
                let message = format!(
                    "Expected {tt:?}, found, {:?}",
                    self.current.unwrap().token_type
                );

                self.error_at_current(message.as_str());
            }
        }
    }

    fn error_at(&mut self, token: &Token, message: &str) {
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
