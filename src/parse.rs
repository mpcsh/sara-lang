use std::collections::HashSet;
use std::iter::{Iterator, Peekable};
use std::str::FromStr;

use crate::ast::{Amount, Ingredient, IngredientUnit, Instruction, Recipe, Reference, Temperature, TemperatureUnit, Time, TimeUnit};
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

fn expect_keyword(stream: &mut TokenStream) -> Result<Keyword, String> {
	match stream.next() {
		Some(Token::Keyword(keyword)) => Ok(*keyword),
		Some(token) => Err(format!("Expected keyword, but got token {:?}", token)),
		None => Err(format!("Expected keyword, but got nothing")),
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
	expect(
		stream,
		vec![
			Token::Keyword(Keyword::Ingredients),
			Token::Colon,
			Token::Whitespace,
		],
	)?;

	let mut ingredients = HashSet::new();

	while stream.peek() != Some(&&Token::Keyword(Keyword::Instructions)) {
		ingredients.insert(parse_ingredient(stream)?);
		expect(stream, vec![Token::Whitespace])?;
	}

	Ok(ingredients)
}

fn expect_reference<'a>(
	stream: &mut TokenStream,
	mise_en_place: &'a HashSet<Ingredient>,
) -> Result<Reference<'a>, String> {
	let reference = expect_identifier(stream)?;

	if &reference == "result" {
		Ok(Reference::Result)
	} else {
		mise_en_place
			.get(&Ingredient {
				name: reference.to_string(),
				amount: Amount {
					quantity: 0.0,
					unit: IngredientUnit::Units,
				},
			})
			.map(|ingredient| Reference::Ingredient(ingredient))
			.ok_or(format!("Could not find {:?}", reference))
	}
}

fn parse_reference_list<'a>(
	stream: &mut TokenStream,
	mise_en_place: &'a HashSet<Ingredient>,
) -> Result<Vec<Reference<'a>>, String> {
	let mut references = Vec::new();
	loop {
		let reference = expect_reference(stream, mise_en_place)?;
		references.push(reference);

		match stream.peek() {
			Some(Token::Whitespace) => {
				return Ok(references);
			}
			Some(Token::Comma) => {
				stream.next();
			}
			Some(token) => {
				return Err(format!(
					"Expected comma or whitespace but got token {:?}",
					token
				));
			}
			None => {
				return Err("Expected comma or whitespace but got nothing".to_string());
			}
		}
	}
}

fn parse_instruction<'a>(
	stream: &mut TokenStream,
	mise_en_place: &'a HashSet<Ingredient>,
) -> Result<Instruction<'a>, String> {
	match expect_keyword(stream)? {
		sigil @ (Keyword::Ingredients | Keyword::Instructions) => Err(format!("Expected instruction but got sigil {:?}", sigil)),

		Keyword::Combine | Keyword::Mix => {
			Ok(Instruction::Combine {
				ingredients: parse_reference_list(stream, mise_en_place)?
			})
		}

		Keyword::Cut => {
			let source = expect_reference(stream, mise_en_place)?;
			expect(stream, vec![Token::Keyword(Keyword::Into)])?;
			let destination = expect_reference(stream, mise_en_place)?;
			Ok(Instruction::CutInto { source, destination })
		}
		Keyword::Into => Err("Cursor should never reach bare Into keyword".to_string()),

		Keyword::Refridgerate => {
			let ingredient = expect_reference(stream, mise_en_place)?;
			let duration = expect_number(stream)?;
			let unit_id = expect_identifier(stream)?;
			let unit = TimeUnit::from_str(&unit_id)
				.map_err(|_| format!("Couldn't parse {:?} as TimeUnit", unit_id))?;

			Ok(Instruction::Refridgerate { ingredient, time: Time { duration, unit }})
		}

		Keyword::Bake => {
			let ingredient = expect_reference(stream, mise_en_place)?;

			let degrees = expect_number(stream)?;
			let temperature_unit_id = expect_identifier(stream)?;
			let temperature_unit = TemperatureUnit::from_str(&temperature_unit_id)
				.map_err(|_| format!("Couldn't parse {:?} as TemperatureUnit", temperature_unit_id))?;

			let duration = expect_number(stream)?;
			let time_unit_id = expect_identifier(stream)?;
			let time_unit = TimeUnit::from_str(&time_unit_id)
				.map_err(|_| format!("Couldn't parse {:?} as TimeUnit", time_unit_id))?;

			Ok(Instruction::Bake {
				ingredient,
				temperature: Temperature { degrees, unit: temperature_unit },
				time: Time { duration, unit: time_unit },
			})
		}
	}
}

fn parse_instructions<'a>(
	stream: &mut TokenStream,
	mise_en_place: &'a HashSet<Ingredient>,
) -> Result<Vec<Instruction<'a>>, String> {
	expect(
		stream,
		vec![
			Token::Keyword(Keyword::Instructions),
			Token::Colon,
		],
	)?;

	let mut instructions = Vec::new();

	while !stream.peek().is_none() {
		expect(stream, vec![Token::Whitespace])?;
		instructions.push(parse_instruction(stream, mise_en_place)?);
	}

	return Ok(instructions);
}

pub fn parse<'a>(tokens: Vec<Token>) -> Result<Recipe<'a>, String> {
	let stream = &mut tokens.iter().peekable();

	let ingredients = parse_ingredients(stream)?;
	println!("Ingredients: {:?}", ingredients);
	println!();

	let instructions = parse_instructions(stream, &ingredients)?;
	println!("Instructions: {:?}", instructions);
	println!();

	// Ok(Recipe { ingredients, instructions })
	unimplemented!()
}
