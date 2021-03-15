#![allow(dead_code)]
#![allow(incomplete_features)]
#![feature(if_let_guard)]
#![feature(or_patterns)]

use std::env::args;
use std::fs::read_to_string;

mod ast;
mod parse;
mod scan;

use parse::Parser;

fn interpret<'a>(source_text: String) -> Result<ast::Recipe, String> {
	let tokens = scan::scan(source_text)?;
	println!("Tokens: {:?}", tokens);
	println!();

	let parser = Parser::new(&tokens);
	let recipe = parser.parse()?;
	println!("AST: {:?}", recipe);
	println!();

	return Ok(recipe);
}

fn main() {
	println!();

	let filename = args().last().unwrap();
	let source_text = read_to_string(filename).unwrap();
	println!("Source text: {:?}", source_text);
	println!();

	let result = interpret(source_text);
	println!("Result: {:?}", result);
}
