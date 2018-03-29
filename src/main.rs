use std::env;
use std::fs::File;
use std::io::prelude::*;

mod compiler;

/// CLI of the interpreter. Usage: plint filename
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./plint filename");
        return;
    }

    let filename = &args[1];

    let mut f = File::open(filename).expect("file not found");

    let mut source = String::new();
    f.read_to_string(&mut source)
        .expect("unable to read file");

    compiler::run(source);
}
