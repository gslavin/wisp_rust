/* lexer.rs
 *
 * Takes str as an input and returns a Vec of tokens.
 * Panics if an invalid token is encountered
 */
use std::io::prelude::*;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenParen,
    Number(f64),
    Identifier(String),
    CloseParen,
}

pub fn parse(buff: &str) -> Vec<Token>{
    let mut tokens = Vec::new();
    let mut token = String::new();

    lazy_static! {
        static ref IDENT: Regex = Regex::new(r"([A-Za-z_]|[/*\+-])([0-9A-Za-z_]|[/*\+-])*").unwrap();
        static ref NUMBER: Regex = Regex::new(r"\d+(\.\d+)?").unwrap();
        static ref WHITESPACE: Regex = Regex::new(r"[:space:]").unwrap();
    }
    for c in buff.chars() {
        // White space and parentheses trigger the completion of the previous token
        if WHITESPACE.is_match(c.to_string().as_str()) || c == '(' || c == ')' {
            if NUMBER.is_match(token.as_str()) {
                let num = token.parse::<f64>().expect("Invalid number!");
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
    use lexer::parse;
    use lexer::Token;

    #[test]
    fn simple_parse() {
        let tokens = parse("(+ 3 4)");
        let expected_tokens = vec![Token::OpenParen, Token::Identifier(String::from("+")),
            Token::Number(3.0), Token::Number(4.0), Token::CloseParen];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn nested_parse() {
        let tokens = parse("(+ (* 3 5) 4)");
        let expected_tokens = vec![Token::OpenParen, Token::Identifier(String::from("+")),
            Token::OpenParen, Token::Identifier(String::from("*")), Token::Number(3.0), Token::Number(5.0),
            Token::CloseParen,  Token::Number(4.0), Token::CloseParen];
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn floating_point() {
        parse("(+ 3 4.0)");
    }

    #[test]
    fn valid_ops() {
        parse("(+ 3 4.0)");
        parse("(- 3 4.0)");
        parse("(* 3 4.0)");
        parse("(/ 3 4.0)");
    }

    #[test]
    #[should_panic]
    fn invalid_identifier() {
        parse("(+ ??? 4)");
    }

    #[test]
    #[should_panic]
    fn leading_digit() {
        parse("(+ 4add 4)");
    }

}
