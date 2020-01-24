#[macro_use]
extern crate lazy_static;

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::process;

mod scanner;
mod tokens;
mod keywords;

#[cfg(test)]
mod tests;

/// main
/// no arguments: run interactively
/// 1 argument: run the script file specified
fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(args.get(0).unwrap()),
        _ => {
            println!("Usage: lox: [script]");
            process::exit(64);
        }
    }
}

/// run a script given in a file having the path specified
fn run_file(path: &String) {
    // open file
    match File::open(path) {
        Ok(mut file) => {
            // read contents into string
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            // run the script
            match run(content) {
                // exit on runtime error
                Err(_) => { process::exit(65); }
                _ => {}
            }
        }

        // report a compilation error
        Err(error) => {
            eprintln!("Error compiling: {}", error);
        }
    }
}

/// run commands interactively
fn run_prompt() {
    // setup stdin to be able to read from it
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // run continuously
    loop {
        // show prompt
        print!(">");
        io::stdout().flush().unwrap();

        // read string from stdin
        let mut content = String::new();
        handle.read_line(&mut content).unwrap();
        let source = String::from(content.trim());

        // run input
        match run(source) {
            Err(message) => {
                eprintln!("{}", message)
            }
            _ => {}
        }
    }
}

/// start interpreting and running the script
fn run(source: String) -> Result<&'static str, &'static str> {
    return match scanner::scan_tokens(source.as_str()) {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
            Ok("Ok")
        }
        Err(code) => {
            Err(code)
        }
    };
}