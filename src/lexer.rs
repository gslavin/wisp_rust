use std::io::prelude::*;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenParen,
    Number(f32),
    Identifier(String),
    CloseParen,
}

/*
impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        match 
    }
}
*/

pub fn parse(buff: &str) -> Vec<Token>{
    let mut tokens = Vec::new();
    let mut token = String::new();

    lazy_static! {
        static ref IDENT: Regex = Regex::new(r"([A-Za-z_]|[/*\+-])([0-9A-Za-z_]|[/*\+-])*").unwrap();
        static ref NUMBER: Regex = Regex::new(r"\d+").unwrap();
        static ref WHITESPACE: Regex = Regex::new(r"[:space:]").unwrap();
    }
    for c in buff.chars() {
        // White space and parentheses trigger the completion of the previous token
        if WHITESPACE.is_match(c.to_string().as_str()) || c == '(' || c == ')' {
            if NUMBER.is_match(token.as_str()) {
                let num = token.parse::<f32>().expect("Invalid number!");
                tokens.push(Token::Number(num));
            }
            else if IDENT.is_match(token.as_str()) {
                tokens.push(Token::Identifier(token.clone()));
            }
            else if !token.is_empty() {
                panic!("Invalid token: {}", token);
            }
            token.clear();
        }
        if WHITESPACE.is_match(c.to_string().as_str()) {
            // WHITESPACE is ignored
        }
        else if c == '(' {
            tokens.push(Token::OpenParen);
        }
        else if c == ')' {
            tokens.push(Token::CloseParen);
        }
        else {
            token.push(c);
        }
    }

    return tokens;
}

#[cfg(test)]
mod test {
    use lexer;

    #[test]
    fn simple_parse() {
        let tokens = lexer::parse("(+ 3 4)");
        let expected_tokens = vec![lexer::Token::OpenParen, lexer::Token::Identifier(String::from("+")),
            lexer::Token::Number(3.0), lexer::Token::Number(4.0), lexer::Token::CloseParen];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    #[should_panic]
    fn bad_parse() {
        lexer::parse("(+ ??? 4");
    
    }
}
