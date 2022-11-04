pub mod tokenizer {
    use regex::{Match, Regex, RegexSet};
    use crate::parsing::parser::tokenizer::Literals::{FloatLiteral, IntegerLiteral, StringLiteral, TextLiteral};
    use crate::parsing::parser::tokenizer::Token::{Comparator, Keyword, Literal, Operator, Symbol, Type};

    #[derive(Debug)]
    pub enum Token {
        Keyword(String),
        Operator(String),
        Comparator(String),
        Type(String),
        Symbol(String),
        Empty(String),
        Literal(Literals),
    }

    #[derive(Debug)]
    pub enum Literals {
        IntegerLiteral(u32),
        FloatLiteral(f32),
        StringLiteral(String),
        TextLiteral(String),
    }

    pub fn tokenize(script: String) -> Result<Vec<Token>, String> {
        let mut work_script = script.clone();

        let mut tokens: Vec<Token> = Vec::new();

        let keyword_regex: Regex = Regex::new("^(setze|speicherplatz|ausgeben|wenn|sonst|solange)").unwrap();
        let operators_regex: Regex = Regex::new("^(=|\\+|-|\\*|/|%|nicht)").unwrap();
        let comparators_regex: Regex = Regex::new("^(==|>=|<=|!=|<|>|und|oder)").unwrap();
        let types_regex: Regex = Regex::new("^(ganzzahl|gleitkommazahl|text)").unwrap();
        let symboles_regex: Regex = Regex::new("^(\\(|\\)|\\{|\\}|\\[|\\]|;)").unwrap();
        let empty_regex: Regex = Regex::new("^(\\s)").unwrap();

        let integer_regex: Regex = Regex::new("^[0-9]+").unwrap();
        let float_regex: Regex = Regex::new("^[0-9]*\\.[0-9]+").unwrap();
        let string_regex: Regex = Regex::new("^\".*\"").unwrap();
        let text_regex: Regex = Regex::new("^[a-zA-Z]+").unwrap();

        let mut find_data: Match;
        while work_script.len() > 0 {
            if empty_regex.is_match(&*work_script) {
                find_data = empty_regex.find(&*work_script).unwrap();

            } else if keyword_regex.is_match(&*work_script) {
                find_data = keyword_regex.find(&*work_script).unwrap();
                tokens.push(Keyword(String::from(find_data.as_str())));

            } else if comparators_regex.is_match(&*work_script) {
                find_data = comparators_regex.find(&*work_script).unwrap();
                tokens.push(Comparator(String::from(find_data.as_str())));

            } else if types_regex.is_match(&*work_script) {
                find_data = types_regex.find(&*work_script).unwrap();
                tokens.push(Type(String::from(find_data.as_str())));

            } else if symboles_regex.is_match(&*work_script) {
                find_data = symboles_regex.find(&*work_script).unwrap();
                tokens.push(Symbol(String::from(find_data.as_str())));

            } else if float_regex.is_match(&*work_script) {
                find_data = float_regex.find(&*work_script).unwrap();
                tokens.push(Literal(FloatLiteral(find_data.as_str().parse::<f32>().unwrap())));

            } else if integer_regex.is_match(&*work_script) {
                find_data = integer_regex.find(&*work_script).unwrap();
                tokens.push(Literal(IntegerLiteral(find_data.as_str().parse::<u32>().unwrap())));

            } else if string_regex.is_match(&*work_script) {
                find_data = string_regex.find(&*work_script).unwrap();
                tokens.push(Literal(StringLiteral(String::from(find_data.as_str()))));

            } else if text_regex.is_match(&*work_script) {
                find_data = text_regex.find(&*work_script).unwrap();
                tokens.push(Literal(TextLiteral(String::from(find_data.as_str()))));

            } else if operators_regex.is_match(&*work_script) {
                find_data = operators_regex.find(&*work_script).unwrap();
                tokens.push(Operator(String::from(find_data.as_str())));

            } else {
                return Err(String::from("INVALID TOKEN!"));

            }
            work_script.replace_range(find_data.start()..find_data.end(), "");
        }

        return Ok(tokens);
    }
}
