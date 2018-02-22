mod compiler;

fn main() {
    println!("Hello, world!");

    compiler::compile(String::from("var foo : string := \"test\";"));

    // compiler::run();
}
