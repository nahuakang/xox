mod error;
mod scanner;
mod token;

use anyhow::Result;
use std::env;
use std::fs;
use std::io::{prelude::*, stdin, stdout};
use std::process;

use scanner::Scanner;

pub fn run_prompt() -> Result<()> {
    println!("Running xox prompt...");

    loop {
        print!("🦞 ");
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;

        if input.trim().is_empty() {
            println!("Exiting xox! Xoxo!");
            break;
        }

        // Execute the line of input
        run(input)?;
    }

    Ok(())
}

pub fn run_file(path: &str) -> Result<()> {
    let source = fs::read_to_string(path)?;
    run(source)?;
    Ok(())
}

pub fn run(source: String) -> Result<()> {
    let mut scanner = Scanner::new();
    scanner.scan_tokens(&mut source.chars().peekable())?;
    let tokens = scanner.get_tokens();
    for token in tokens {
        println!("Token: {:?}", token);
    }
    Ok(())
}

fn main() -> Result<()> {
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
