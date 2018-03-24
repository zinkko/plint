mod lexer;
mod parser;
mod interpreter;
mod code_generator;

pub fn compile(source: String) {
    let tokens = match lexer::scan(&source) {
        Ok(tokens) => tokens,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    for t in tokens {
        println!("{:?}", t);
    }

    // match parser::parse(tokens) {
    //     Ok(ast) => show(ast),
    //     Err(msg) => println!("Parsing failed: {}", msg),
    // };
}

fn show(ast: parser::ast::Ast) {
    println!("Statements:");

    for s in ast.statements {
        println!("{:?}", s);
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
