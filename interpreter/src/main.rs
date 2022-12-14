mod error;
mod scanner;
mod token;

use std::env;
use std::error::Error;
use std::fs;
use std::io::{prelude::*, stdin, stdout};
use std::process;

use scanner::Scanner;

pub fn run_prompt() -> Result<(), Box<dyn Error>> {
    println!("Running xox prompt...");

    loop {
        print!("> ");
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;

        if input.trim().is_empty() {
            println!("Exiting xox! Xoxo!");
            break;
        }

        // TODO: Remove this
        println!("You wrote: {}", input);
    }

    Ok(())
}

pub fn run_file(path: &str) -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string(path)?;
    run(source);
    Ok(())
}

pub fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for token in tokens {
        println!("Token: {:?}", token);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_] => run_prompt()?,
        [_, file] => run_file(file)?,
        _ => {
            println!("Usage: xox [script]");
            process::exit(64);
        }
    }

    Ok(())
}
