#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::prelude::*;
use std::fs::File;
use std::io::Error;

pub mod lexer;
pub mod parser;
pub mod eval;

fn read_file(filename: &str) -> Result<String, Error> {
	let mut f = try!(File::open(filename));
	let mut s = String::new();
	try!(f.read_to_string(&mut s));

	return Ok(s);
}

fn main() {

	let result = read_file("input.wsp");
	let s = result.expect("Unable to read file");
	println!("{}", s);
	let tokens = lexer::parse(s.as_str());
	for t in tokens {
		println!("{:?}", t);
	}
}
