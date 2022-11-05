use std::fs;
use crate::parsing::parser::parse;
use crate::parsing::tokenizer::tokenize;

mod parsing;


fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("examples/skript.txt")
        .expect("Should have been able to read the file");

    let tokens = tokenize(contents).unwrap();
    let parse_tree = parse(tokens);

    println!("{:?}", parse_tree);
}
