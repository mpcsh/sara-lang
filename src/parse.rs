use std::collections::HashMap;
use std::iter::{Iterator, Peekable};

use crate::ast::{Recipe, Ingredient, Amount};
use crate::scan::{Token, Keyword};

type TokenStream<'a> = Peekable<std::slice::Iter<'a, Token>>;

fn expect(stream: &mut TokenStream, tokens: Vec<Token>) -> Result<(), String> {
	for expected in tokens {
		match stream.next() {
			None => {
				return Err(format!("Expected token {:?}, but got nothing", expected));
			}
			Some(received) if received != &expected => {
				return Err(format!("Expected token {:?}, but got token {:?}", expected, received));
			}
			_ => {}
		}
	}

	Ok(())
}

fn expect_identifier(stream: &mut TokenStream) -> Result<String, String> {
	match stream.next() {
		Some(Token::Identifier(id)) => {
			Ok(id.to_string())
		}
		Some(token) => {
			Err(format!("Expected identifier, but got token {:?}", token))
		}
		None => {
			Err(format!("Expected identifier, but got nothing"))
		}
	}
}

fn expect_number(stream: &mut TokenStream) -> Result<f64, String> {
	match stream.next() {
		Some(Token::Number(num)) => {
			Ok(*num)
		}
		Some(token) => {
			Err(format!("Expected number, but got token {:?}", token))
		}
		None => {
			Err(format!("Expected number, but got nothing"))
		}
	}
}

fn parse_ingredient(stream: &mut TokenStream) -> Result<Ingredient, String> {
	let name = expect_identifier(stream)?;
	expect(stream, vec![Token::Colon])?;
	let quantity = expect_number(stream)?;
	let unit = expect_identifier(stream)?;

	Ok(Ingredient { name, amount: Amount { quantity, unit }})
}

fn parse_ingredients(stream: &mut TokenStream) -> Result<HashMap<String, Amount>, String> {
	let ingredient = parse_ingredient(stream);
	println!("{:?}", ingredient);

	unimplemented!()
}

fn parse_recipe(stream: &mut TokenStream) -> Result<Recipe, String> {
	expect(stream, vec![Token::Keyword(Keyword::Ingredients), Token::Colon, Token::Whitespace])?;

	let ingredients = parse_ingredients(stream)?;

	unimplemented!()
}

pub fn parse(tokens: Vec<Token>) -> Result<Recipe, String> {
  return parse_recipe(&mut tokens.iter().peekable());
}
