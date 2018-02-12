mod compiler;

fn main() {
    println!("Hello, world!");
    
    compiler::compile();

    compiler::run();
}
