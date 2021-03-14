use std::collections::HashSet;
use std::iter::{Iterator, Peekable};
use std::str::FromStr;

use crate::ast::{Amount, Ingredient, IngredientUnit, Recipe};
use crate::scan::{Keyword, Token};

type TokenStream<'a> = Peekable<std::slice::Iter<'a, Token>>;

fn expect(stream: &mut TokenStream, tokens: Vec<Token>) -> Result<(), String> {
	for expected in tokens {
		match stream.next() {
			None => {
				return Err(format!("Expected token {:?}, but got nothing", expected));
			}
			Some(received) if received != &expected => {
				return Err(format!(
					"Expected token {:?}, but got token {:?}",
					expected, received
				));
			}
			_ => {}
		}
	}

	Ok(())
}

fn expect_identifier(stream: &mut TokenStream) -> Result<String, String> {
	match stream.next() {
		Some(Token::Identifier(id)) => Ok(id.to_string()),
		Some(token) => Err(format!("Expected identifier, but got token {:?}", token)),
		None => Err(format!("Expected identifier, but got nothing")),
	}
}

fn expect_number(stream: &mut TokenStream) -> Result<f64, String> {
	match stream.next() {
		Some(Token::Number(num)) => Ok(*num),
		Some(token) => Err(format!("Expected number, but got token {:?}", token)),
		None => Err(format!("Expected number, but got nothing")),
	}
}

fn parse_ingredient(stream: &mut TokenStream) -> Result<Ingredient, String> {
	let name = expect_identifier(stream)?;
	expect(stream, vec![Token::Colon])?;
	let quantity = expect_number(stream)?;
	let unit = match stream.peek() {
		Some(Token::Whitespace) => IngredientUnit::Units,
		_ => {
			let unit_id = expect_identifier(stream)?;
			IngredientUnit::from_str(&unit_id)
				.map_err(|_| format!("Couldn't parse {:?} as IngredientUnit", unit_id))?
		}
	};

	Ok(Ingredient {
		name,
		amount: Amount { quantity, unit },
	})
}

fn parse_ingredients(stream: &mut TokenStream) -> Result<HashSet<Ingredient>, String> {
	let mut ingredients = HashSet::new();

	while stream.peek() != Some(&&Token::Keyword(Keyword::Instructions)) {
		ingredients.insert(parse_ingredient(stream)?);
		expect(stream, vec![Token::Whitespace])?;
	}

	Ok(ingredients)
}

fn parse_recipe(stream: &mut TokenStream) -> Result<Recipe, String> {
	expect(
		stream,
		vec![
			Token::Keyword(Keyword::Ingredients),
			Token::Colon,
			Token::Whitespace,
		],
	)?;

	let ingredients = parse_ingredients(stream)?;
	println!("Ingredients: {:?}", ingredients);

	unimplemented!()
}

pub fn parse(tokens: Vec<Token>) -> Result<Recipe, String> {
	return parse_recipe(&mut tokens.iter().peekable());
}
