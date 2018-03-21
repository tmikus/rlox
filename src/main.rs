mod scanner;
mod token_type;
mod token;
mod runtime_status;
mod error;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::prelude::*;
use std::path::Path;
use scanner::Scanner;
use runtime_status::RuntimeStatus;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() > 2 {
    println!("Usage: rlox [script]");
  }
  else if args.len() == 2 {
    run_file(&args[1]);
  }
  else {
    run_prompt();
  }
}

fn run_file(path: &String) {
  let mut runtime_status = RuntimeStatus::new();
  let file_path = Path::new(path);
  let mut file = match File::open(&file_path) {
    Err(why) => panic!("Couldn't open {}: {}", file_path.display(), why.description()),
    Ok(file) => file,
  };
  let mut source = String::new();
  match file.read_to_string(&mut source) {
    Err(why) => panic!("Couldn't read {}: {}", file_path.display(), why.description()),
    Ok(_) => {},
  }
  run(source, &mut runtime_status);
  if runtime_status.had_error {
    std::process::exit(65);
  }
}

fn run_prompt() {
  let mut runtime_status = RuntimeStatus::new(); 
  let stdin = io::stdin();
  let mut stdout = io::stdout();
  let mut stdin_iterator = stdin.lock().lines();
  loop {
    print!("> ");
    stdout.flush().unwrap();
    run(stdin_iterator.next().unwrap().unwrap(), &mut runtime_status);
    runtime_status.had_error = false;
  }
}

fn run(source: String, runtime_status: &mut RuntimeStatus) {
  let mut scanner = Scanner::new(source);
  let tokens = scanner.scan_tokens(runtime_status);

  for token in tokens {
    println!("{}", token);
  }
}
