#![allow(dead_code)]
#![allow(incomplete_features)]
#![feature(if_let_guard)]
#![feature(or_patterns)]

use std::env::args;
use std::error::Error;
use std::fs::read_to_string;

mod ast;
mod bake;
mod parse;
mod scan;

use bake::Baker;
use parse::Parser;
use scan::Scanner;

fn interpret<'a>(source_text: String) -> Result<String, Box<dyn Error>> {
	let scanner = Scanner::new(&source_text);
	let tokens = scanner.scan()?;

	let parser = Parser::new(&tokens);
	let recipe = parser.parse()?;

  let baker = Baker::new(&recipe);
	let (baked_good, ingredients) = baker.bake();

	return Ok(baked_good);
}

fn main() {
	println!();

	let filename = args().last().unwrap();
	let source_text = read_to_string(filename).unwrap();

	let result = interpret(source_text);
	match result {
		Ok(baked_good) => {
			println!("{}", baked_good);
		}
		Err(error) => {
			println!("{}", error);
		}
	}
}
