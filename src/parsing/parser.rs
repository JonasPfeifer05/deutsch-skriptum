pub mod tokenizer {
    use regex::{Match, Regex, RegexSet};
    use crate::parsing::parser::tokenizer::Literals::{Float_literal, Integer_literal, String_literal, Text_literal};
    use crate::parsing::parser::tokenizer::Token::{Comparator, Keyword, Literal, Operator, Symbol, Type};

    #[derive(Debug)]
    enum Token {
        Keyword(String),
        Operator(String),
        Comparator(String),
        Type(String),
        Symbol(String),
        Empty(String),
        Literal(Literals),
    }

    #[derive(Debug)]
    enum Literals {
        Integer_literal(u32),
        Float_literal(f32),
        String_literal(String),
        Text_literal(String),
    }

    pub fn tokenize(script: String) {
        let mut workon_script = script.clone();

        let mut tokens: Vec<Token> = Vec::new();

        let keyword_regex: Regex = Regex::new("^(speicherplatz|ausgeben|wenn|sonst|solange)").unwrap();
        let operators_regex: Regex = Regex::new("^(=|\\+|-|\\*|/|%|nicht)").unwrap();
        let comparators_regex: Regex = Regex::new("^(==|>=|<=|!=|<|>|und|oder)").unwrap();
        let types_regex: Regex = Regex::new("^(ganzzahl|gleitkommazahl|text)").unwrap();
        let symboles_regex: Regex = Regex::new("^(\\(|\\)|\\{|\\}|\\[|\\]|;|\")").unwrap();
        let empty_regex: Regex = Regex::new("^(\\s)").unwrap();

        let general_literals_regex: Regex = Regex::new("^([0-9]+|[0-9]*.[0-9]+|\"[a-zA-Z]+\"|[a-zA-Z]+)").unwrap();
        let integer_regex: Regex = Regex::new("^[0-9]+").unwrap();
        let float_regex: Regex = Regex::new("^[0-9]*.[0-9]+").unwrap();
        let string_regex: Regex = Regex::new("^\"[a-zA-Z_!?.,ßäöü]+\"").unwrap();
        let text_regex: Regex = Regex::new("^[a-zA-Z_]+").unwrap();

        let mut find_data: Option<Match> = None;
        while workon_script.len() > 0 {
            if empty_regex.is_match(&*workon_script) {
                let tmp = empty_regex.find(&*workon_script).unwrap();
                find_data = Some(tmp);
            } else if keyword_regex.is_match(&*workon_script) {
                let tmp = keyword_regex.find(&*workon_script).unwrap();
                find_data = Some(tmp);
                tokens.push(Keyword(String::from(tmp.as_str())));
            } else if operators_regex.is_match(&*workon_script) {
                let tmp = operators_regex.find(&*workon_script).unwrap();
                find_data = Some(tmp);
                tokens.push(Operator(String::from(tmp.as_str())));
            } else if comparators_regex.is_match(&*workon_script) {
                let tmp = comparators_regex.find(&*workon_script).unwrap();
                find_data = Some(tmp);
                tokens.push(Comparator(String::from(tmp.as_str())));
            } else if types_regex.is_match(&*workon_script) {
                let tmp = types_regex.find(&*workon_script).unwrap();
                find_data = Some(tmp);
                tokens.push(Type(String::from(tmp.as_str())));
            } else if symboles_regex.is_match(&*workon_script) {
                let tmp = symboles_regex.find(&*workon_script).unwrap();
                find_data = Some(tmp);
                tokens.push(Symbol(String::from(tmp.as_str())));
            } else if float_regex.is_match(&*workon_script) {
                let tmp = float_regex.find(&*workon_script).unwrap();
                find_data = Some(tmp);
                println!("'{}'", tmp.as_str());
                let literal = Literal(Float_literal(tmp.as_str().parse::<f32>().unwrap()));
                tokens.push(literal);
            } else if integer_regex.is_match(&*workon_script) {
                let tmp = integer_regex.find(&*workon_script).unwrap();
                find_data = Some(tmp);
                let literal = Literal(Integer_literal(tmp.as_str().parse::<u32>().unwrap()));
                tokens.push(literal);
            } else if string_regex.is_match(&*workon_script) {
                let tmp = string_regex.find(&*workon_script).unwrap();
                find_data = Some(tmp);
                let literal = Literal(String_literal(String::from(tmp.as_str())));
                tokens.push(literal);
            } else if text_regex.is_match(&*workon_script) {
                let tmp = text_regex.find(&*workon_script).unwrap();
                find_data = Some(tmp);
                let literal = Literal(Text_literal(String::from(tmp.as_str())));
                tokens.push(literal);
            } else {
                println!("ERROR: INVALID TOKEN!");
                println!("'{}'", workon_script);
                break;
            }
            if find_data.is_some() {
                workon_script.replace_range(find_data.unwrap().start()..find_data.unwrap().end(), "");
            }
        }
        println!("{:?}", tokens);
    }
}
