use crate::scanner::Scanner;

pub fn compile(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan();
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
