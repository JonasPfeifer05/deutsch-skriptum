use std::fs;
use crate::parsing::parser::tokenizer::tokenize;

mod parsing;


fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("examples/skript.txt")
        .expect("Should have been able to read the file");

    let result = tokenize(contents);
    println!("{:?}", result);
}
