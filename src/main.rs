#![allow(dead_code)]
#![allow(incomplete_features)]
#![feature(if_let_guard)]
#![feature(or_patterns)]

use std::env::args;
use std::error::Error;
use std::fs::read_to_string;

mod ast;
mod parse;
mod scan;

use ast::Recipe;
use parse::Parser;
use scan::Scanner;

fn interpret<'a>(source_text: String) -> Result<Recipe, Box<dyn Error>> {
	let scanner = Scanner::new(&source_text);
	let tokens = scanner.scan()?;

	let parser = Parser::new(&tokens);
	let recipe = parser.parse()?;

	return Ok(recipe);
}

fn main() {
	println!();

	let filename = args().last().unwrap();
	let source_text = read_to_string(filename).unwrap();

	let result = interpret(source_text);
	match result {
		Ok(recipe) => {
			println!("{:?}", recipe);
		}
		Err(error) => {
			println!("{}", error);
		}
	}
}
