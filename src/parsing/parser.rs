use std::any::Any;
use std::ops::Add;
use std::vec::IntoIter;
use crate::parsing::parser::Component::Declaration;
use crate::parsing::tokenizer::{Literals, Token};
use crate::parsing::tokenizer::Literals::TextLiteral;
use crate::parsing::tokenizer::Token::{Comparator, Keyword, Literal, Operator, Symbol, Type};

#[derive(Debug)]
pub enum Component {
    Declaration(),
    Assignment(),
    If(),
    While(),
    Print(),
}

#[derive(Debug)]
pub struct ParseTree {
    commands: Vec<Component>
}

impl ParseTree {
    pub fn new() -> Self {
        return ParseTree{commands: Vec::new()};
    }
}

fn get_next<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> &'a Token {
    let token: &Token = tokens.get(*index).unwrap();
    (*index) += 1;

    return token;
}

fn expect_next<'a>(tokens: &'a Vec<Token>, index: &mut usize, expected: Token) -> Result<&'a Token, String> {
    let token = get_next(tokens, index);
    if token.type_id().eq(&expected.type_id()) {
        return Ok(token);
    }
    return Err(String::from("Tokens did not match!"));
}

fn expect_next_value<'a>(tokens: &'a Vec<Token>, index: &mut usize, expected: Token) -> Result<&'a Token, String> {
    let token = get_next(tokens, index);
    if *token == expected {
        return Ok(token);
    }
    return Err(String::from("Tokens did not match!"));
}

fn create_expression(tokens: &'a Vec<Token>, index: &mut usize) {

}

fn create_declaration(tokens: &Vec<Token>, index: &mut usize, typee: &String) -> Result<Component, String> {
    let keyword = expect_next_value(tokens, index, Keyword(String::from("speicherplatz"))).expect("Expected Speicherplatz token");
    let variable_name = expect_next(tokens, index, Literal(TextLiteral(String::new()))).expect("Expected variable name!");
    expect_next_value(tokens, index, Operator(String::from("="))).expect("Expected assigment operator!");

    println!("{:?}", variable_name);

    return Ok(Declaration())
}

fn create_assignment(tokens: &Vec<Token>, index: &mut usize) {

}

fn create_if(tokens: &Vec<Token>, index: &mut usize) {

}

fn create_while(tokens: &Vec<Token>, index: &mut usize) {

}

fn create_print(tokens: &Vec<Token>, index: &mut usize) {

}


pub fn parse(tokens: Vec<Token>) -> Result<ParseTree, String> {
    for mut i in 0..tokens.len() {
        match get_next(&tokens, &mut i) {
            Keyword(value) => {
                match value.as_str() {
                    "setze" => {create_assignment(&tokens, &mut i)}
                    "ausgeben" => {create_print(&tokens, &mut i)}
                    "wenn" => {create_if(&tokens, &mut i)}
                    "solange" => {create_while(&tokens, &mut i)}
                    _ => {
                        return Err(String::from("Invalid starting token!"));
                    }
                }
            }
            Type(value) => {
                println!("{:?}", create_declaration(&tokens, &mut i, value));
            }
            Literal(literal_type) => {
                match literal_type {
                    Literals::TextLiteral(value) => {

                    }
                    _ => {
                        return Err(String::from("Invalid starting token!"));
                    }
                }
            }
            _ => {
                return Err(String::from("Invalid starting token!"));
            }
        }
    }
    return Ok(ParseTree::new());
}