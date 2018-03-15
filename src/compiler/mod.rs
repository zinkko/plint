mod lexer;
mod parser;
mod interpreter;
mod code_generator;

pub fn compile(source: String) {
    let tokens = lexer::scan(&source);
    let ast = parser::parse(tokens);

    show(ast);
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
