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
    Identifier,
    String,
    Number,

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
    Error,
    EOF,
}

pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub length: i32,
    pub line: i32,
}

pub struct Scanner<'a> {
    source: &'a str,
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
            token_type: TokenType::Error,
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

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;
        let c = self.advance();

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
            Some(c) if c.is_alphabetic() => self.keyword(),
            Some(c) if c.is_numeric() => self.number(),
            Some(_) => self.error_token("Unexpected character."),
            None => self.make_token(TokenType::EOF),
        }
    }

    fn keyword_type(&mut self) -> TokenType {
        macro_rules! check_keywords {
            [$($next:literal, $expected: literal => $tt: expr),+] => {
                if self.current - self.start > 1 {
                    let current = self.source.chars().nth(self.start + 1);
                    match current {
                        $(
                            Some($next) => self.check_keyword($expected, $tt, 2),
                        )*
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            };
        }

        let c = self.source.chars().nth(self.start);

        match c {
            Some('a') => self.check_keyword("nd", TokenType::And, 1),
            Some('c') => self.check_keyword("lass", TokenType::Class, 1),
            Some('e') => self.check_keyword("lse", TokenType::Else, 1),
            Some('i') => self.check_keyword("f", TokenType::If, 1),
            Some('n') => self.check_keyword("il", TokenType::Nil, 1),
            Some('o') => self.check_keyword("r", TokenType::Or, 1),
            Some('p') => self.check_keyword("rint", TokenType::Print, 1),
            Some('r') => self.check_keyword("eturn", TokenType::Return, 1),
            Some('s') => self.check_keyword("uper", TokenType::Super, 1),
            Some('v') => self.check_keyword("ar", TokenType::Var, 1),
            Some('w') => self.check_keyword("hile", TokenType::While, 1),
            Some('f') => check_keywords![
                'a', "lse" => TokenType::False,
                'o', "r" => TokenType::For,
                'u', "n" => TokenType::Fun
            ],
            Some('t') => check_keywords![
                'a', "lse" => TokenType::False,
                'o', "r" => TokenType::For,
                'u', "n" => TokenType::Fun
            ],
            _ => TokenType::Identifier,
        }
    }

    fn keyword(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() {
                self.advance();
            } else {
                break;
            }
        }

        let tt = self.keyword_type();
        self.make_token(tt)
    }

    fn check_keyword(
        &mut self,
        rest: &str,
        tt: TokenType,
        already_checked_len: usize,
    ) -> TokenType {
        let scrutinee = &self.source[self.start + already_checked_len..self.current];
        if scrutinee == rest {
            tt
        } else {
            TokenType::Identifier
        }
    }

    fn number(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if !c.is_numeric() {
                break;
            }
            self.advance();
        }

        self.make_token(TokenType::Number)
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

        self.make_token(TokenType::String)
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

    pub fn print_tokens(&self, tokens: &Vec<Token>) {
        let mut prev_line = -1;
        for token in tokens {
            if token.line != prev_line {
                print!("{:0>4} ", token.line);
                prev_line = token.line;
            } else { 
                print!("   | ");
            }
            println!("{:?} '{}'", token.token_type, token.start);
        }
    }
}
