use trie_rs::TrieBuilder;

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Not really code.
    Error(String),
    EOF,
}

pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub length: i32,
    pub line: i32,
}

pub struct Scanner<'s> {
    source: &'s str,
    start: usize,
    current: usize,
    line: i32,
}

impl<'s> Scanner<'s> {
    pub fn new(source: &'s str) -> Scanner {
        let mut keywords = TrieBuilder::new();
        keywords.push("");
        Scanner {
            source,
            current: 0,
            line: 0,
            start: 0,
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            let token = self.scan_token();
            if token.token_type == TokenType::EOF {
                break;
            }
            tokens.push(token);
        }

        tokens
    }

    fn error_token(&self, message: &str) -> Token {
        Token {
            token_type: TokenType::Error(String::from(message)),
            length: (self.current - self.start) as i32,
            line: self.line,
            start: self.start,
        }
    }

    fn make_token(&self, tt: TokenType) -> Token {
        Token {
            token_type: tt,
            length: (self.current - self.start) as i32,
            line: self.line,
            start: self.start,
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                Some(' ') | Some('\r') | Some('\t') => {
                    self.advance();
                }
                Some('\n') => {
                    self.line += 1;
                    self.advance();
                }
                Some('/') => {
                    if self.peek_next() == Some('/') {
                        while self.peek() != Some('\n') && self.peek() != None {
                            self.advance();
                        }
                    }
                }
                _ => return,
            }
        }
    }

    fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;
        let c = self.advance();

        println!("{:?}", c);

        match c {
            Some('"') => self.string(),
            Some('(') => self.make_token(TokenType::LeftParen),
            Some(')') => self.make_token(TokenType::RightParen),
            Some('{') => self.make_token(TokenType::LeftBrace),
            Some('}') => self.make_token(TokenType::RightBrace),
            Some(';') => self.make_token(TokenType::Semicolon),
            Some(',') => self.make_token(TokenType::Comma),
            Some('.') => self.make_token(TokenType::Dot),
            Some('-') => self.make_token(TokenType::Minus),
            Some('+') => self.make_token(TokenType::Plus),
            Some('/') => self.make_token(TokenType::Slash),
            Some('*') => self.make_token(TokenType::Star),
            Some('!') => self.ternary_match('=', TokenType::BangEqual, TokenType::Bang),
            Some('=') => self.ternary_match('=', TokenType::EqualEqual, TokenType::Equal),
            Some('<') => self.ternary_match('=', TokenType::GreaterEqual, TokenType::Greater),
            Some('>') => self.ternary_match('=', TokenType::LessEqual, TokenType::Less),
            Some(c) if c.is_numeric() => self.number(),
            Some(_) => self.error_token("Unexpected character."),
            None => self.make_token(TokenType::EOF),
        }
    }

    fn number(&mut self) -> Token {
        while let Some(c) = self.advance() {
            if !c.is_numeric() {
                break;
            }
        }

        let unparsed = String::from(&self.source[self.start..self.current - 1]);

        self.make_token(TokenType::Number(unparsed.parse::<f64>().unwrap()))
    }

    fn string(&mut self) -> Token {
        loop {
            let current = self.advance();
            match current {
                Some('"') => break,
                Some('\n') => self.line += 1,
                Some(_) => {
                    self.advance();
                }
                None => return self.error_token("Unterminated string."),
            }
        }

        self.make_token(TokenType::String(String::from(
            &self.source[self.start..self.current],
        )))
    }

    fn ternary_match(
        &mut self,
        expected: char,
        if_match: TokenType,
        if_no_match: TokenType,
    ) -> Token {
        let matched_type = if self.match_token(expected) {
            if_match
        } else {
            if_no_match
        };

        self.make_token(matched_type)
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }

    fn match_token(&mut self, expected: char) -> bool {
        match self.source.chars().nth(self.current) {
            Some(c) if c == expected => {
                self.advance();
                true
            }
            Some(_) => false,
            None => false,
        }
    }
}
