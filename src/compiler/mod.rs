mod lexer;
mod parser;
mod interpreter;
mod code_generator;

pub fn compile(source: String) {
    let tokens = lexer::scan(&source);

    println!("\nsource: {}\n", source);

    for t in tokens {
        println!("token: {:?}", t);
    }
}

// pub fn run() {
//     println!("Hello, I am the Interpreter!");
//     println!("I have also not been implemented");
// }

// To be implemented (maybe)
// pub fun repl() {
//
// }
