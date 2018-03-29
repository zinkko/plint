mod lexer;
mod parser;
mod interpreter;

pub fn run(source: String) {
    let tokens = match lexer::scan(&source) {
        Ok(tokens) => tokens,
        Err(e) => { println!("Scanning failed: {}", e); return; },
    };

    let ast = match parser::parse(tokens) {
        Ok(ast) => ast,
        Err(msg) => { println!("Parsing failed: {}", msg); return; },
    };

    let result = interpreter::evaluate(ast);
    match result {
        Ok(_) => (),
        Err(e) => println!("Runtime error: {}", e),
    }
}
