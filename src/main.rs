use std::process;
use std::env;

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

fn run_file(_path: &String) {

}

fn run_prompt() {

}