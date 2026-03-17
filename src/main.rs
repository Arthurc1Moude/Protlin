mod lexer;
mod parser;
mod ast;
mod interpreter;
mod types;
mod environment;
mod builtins;
mod error;
mod graphics;

use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        run_file(&args[1]);
    } else {
        run_repl();
    }
}

fn run_file(path: &str) {
    let source = fs::read_to_string(path)
        .expect("Failed to read file");
    
    execute(&source);
}

fn run_repl() {
    println!("Protlin REPL v0.1.0 - Born from an Egg, Maximum Myriad");
    println!("Type 'exit' to quit\n");
    
    let mut env = environment::Environment::new();
    
    loop {
        print!("protlin> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let input = input.trim();
        if input == "exit" {
            break;
        }
        
        if input.is_empty() {
            continue;
        }
        
        match execute_with_env(&input, &mut env) {
            Ok(result) => {
                if !matches!(result, ast::Value::Void) {
                    println!("{}", result);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn execute(source: &str) {
    let mut env = environment::Environment::new();
    match execute_with_env(source, &mut env) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn execute_with_env(source: &str, env: &mut environment::Environment) -> Result<ast::Value, error::ProtlinError> {
    let mut lexer = lexer::Lexer::new(source);
    let tokens = lexer.tokenize()?;
    
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse()?;
    
    let mut interpreter = interpreter::Interpreter::new(env);
    interpreter.execute(&ast)
}
