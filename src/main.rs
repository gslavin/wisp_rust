#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::prelude::*;
use std::fs::File;
use std::io::Error;
use regex::Regex;

#[derive(Debug)]
enum Token {
    OpenParen,
    NUM(f32),
    IDENT(String),
    CloseParen,
}

fn read_file(filename: &str) -> Result<String, Error> {
	let mut f = try!(File::open(filename));
	let mut s = String::new();
	try!(f.read_to_string(&mut s));

	return Ok(s);
}

fn parse(buff: &str) -> Vec<Token>{
	let mut tokens = Vec::new();
	let mut token = String::new();

	lazy_static! {
        static ref IDENT: Regex = Regex::new(r"[:^digit:].*").unwrap();
        static ref NUMBER: Regex = Regex::new(r"\d+").unwrap();
        static ref WHITESPACE: Regex = Regex::new(r"[:space:]").unwrap();
	}
	for c in buff.chars() {
		// White space and parentheses trigger the completion of the previous token
		if WHITESPACE.is_match(c.to_string().as_str()) || c == '(' || c == ')' {
			if IDENT.is_match(token.as_str()) {
				tokens.push(Token::IDENT(token.clone()));
			}
			else if NUMBER.is_match(token.as_str()) {
				let num = token.parse::<f32>().expect("Invalid number!");
				tokens.push(Token::NUM(num));
			}
			else if !token.is_empty() {
				println!("Invalid token: {}", token);
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

fn main() {

	let result = read_file("input.wsp");
	let s = result.expect("Unable to read file");
	println!("{}", s);
	let tokens = parse(s.as_str());
	for t in tokens {
		println!("{:?}", t);
	}
}
