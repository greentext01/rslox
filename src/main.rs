use std::{
    env::args,
    fs,
    io::{self},
    process::exit,
};

mod chunk;
mod compiler;
mod instructions;
mod opcode;
mod scanner;
mod util;
mod values;
mod vm;

fn repl() {
    let mut line = String::new();
    loop {
        print!(">>> ");
        util::flush_stdout();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Err(_) => panic!("Couldn't read from input"),
            _ => {}
        };

        interpret(&line);
        line.clear();
    }
}

fn run_file(path: &str) {
    let source = fs::read_to_string(path).expect("Couldn't read from file");
    interpret(&source);
}

fn interpret(source: &str) {
    compiler::compile(source);
}

fn main() {
    match args().len() {
        1 => repl(),
        2 => run_file(&args().nth(1).unwrap()),
        _ => {
            println!("Usage clox [path]");
            exit(64)
        }
    }
}
