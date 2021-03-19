use std::collections::HashMap;
use std::default::Default;
use std::iter::{Iterator, Peekable};
use std::str::FromStr;

use unicase::UniCase;

use crate::ast::{Amount, AmountUnit, Instruction, Recipe, Reference, Temperature, TemperatureUnit, Time, TimeUnit};
use crate::scan::{Keyword, Token};

pub struct Parser<'a> {
	stream: Peekable<std::slice::Iter<'a, Token>>,
	mise_en_place: HashMap<Reference, Amount>,
}

impl<'a> Parser<'a> {
	fn next(&mut self) -> Option<&Token> {
		self.stream.next()
	}

	fn peek(&mut self) -> Option<&&Token> {
		self.stream.peek()
	}

	fn expect(&mut self, tokens: Vec<Token>) -> Result<(), String> {
		for expected in tokens {
			match self.next() {
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

	fn expect_identifier(&mut self) -> Result<String, String> {
		match self.next() {
			Some(Token::Identifier(id)) => Ok(id.to_string()),
			Some(token) => Err(format!("Expected identifier, but got token {:?}", token)),
			None => Err(format!("Expected identifier, but got nothing")),
		}
	}

	fn expect_keyword(&mut self) -> Result<Keyword, String> {
		match self.next() {
			Some(Token::Keyword(keyword)) => Ok(*keyword),
			Some(token) => Err(format!("Expected keyword, but got token {:?}", token)),
			None => Err(format!("Expected keyword, but got nothing")),
		}
	}

	fn expect_number(&mut self) -> Result<f64, String> {
		match self.next() {
			Some(Token::Number(num)) => Ok(*num),
			Some(token) => Err(format!("Expected number, but got token {:?}", token)),
			None => Err(format!("Expected number, but got nothing")),
		}
	}

	fn parse_ingredient(&mut self) -> Result<(Reference, Amount), String> {
		let name = UniCase::from(self.expect_identifier()?);
		self.expect(vec![Token::Colon])?;
		let quantity = self.expect_number()?;
		let unit = match self.peek() {
			Some(Token::Newline) => AmountUnit::Units,
			_ => {
				let unit_id = self.expect_identifier()?;
				AmountUnit::from_str(&unit_id).map_err(|_| format!("Couldn't parse {:?} as AmountUnit", unit_id))?
			}
		};

		Ok((name, Amount { quantity, unit }))
	}

	fn parse_ingredients(&mut self) -> Result<(), String> {
		self.expect(vec![Token::Keyword(Keyword::Ingredients), Token::Colon, Token::Newline])?;

		while self.peek() != Some(&&Token::Keyword(Keyword::Instructions)) {
			let (name, amount) = self.parse_ingredient()?;
			self.mise_en_place.insert(name, amount);
			self.expect(vec![Token::Newline])?;
		}

		Ok(())
	}

	fn expect_reference(&mut self) -> Result<Reference, String> {
		let reference = UniCase::from(self.expect_identifier()?);

		if reference == UniCase::new("result") {
			return Ok(reference);
		}

		if self.mise_en_place.contains_key(&reference) {
			return Ok(reference);
		}

		return Err(format!("Could not find ingredient {:?}", reference));
	}

	fn expect_reference_list(&mut self) -> Result<Vec<Reference>, String> {
		let mut references = Vec::new();
		loop {
			let reference = self.expect_reference()?;
			references.push(reference);

			match self.peek() {
				Some(Token::Newline) => {
					return Ok(references);
				}
				Some(Token::Comma) => {
					self.next();
				}
				Some(token) => {
					return Err(format!("Expected comma or whitespace but got token {:?}", token));
				}
				None => {
					return Err("Expected comma or whitespace but got nothing".to_string());
				}
			}
		}
	}

	fn parse_instruction(&mut self) -> Result<Instruction, String> {
		match self.expect_keyword()? {
			sigil @ (Keyword::Ingredients | Keyword::Instructions) => {
				Err(format!("Expected instruction but got sigil {:?}", sigil))
			}

			Keyword::Combine | Keyword::Mix => Ok(Instruction::Combine {
				ingredients: self.expect_reference_list()?,
			}),

			Keyword::Cut => {
				let source = self.expect_reference()?;
				self.expect(vec![Token::Keyword(Keyword::Into)])?;
				let destination = self.expect_reference()?;
				Ok(Instruction::CutInto { source, destination })
			}
			Keyword::Into => Err("Cursor should never reach bare Into keyword".to_string()),

			Keyword::Refridgerate => {
				let ingredient = self.expect_reference()?;
				let duration = self.expect_number()?;
				let unit_id = self.expect_identifier()?;
				let unit = TimeUnit::from_str(&unit_id).map_err(|_| format!("Couldn't parse {:?} as TimeUnit", unit_id))?;

				Ok(Instruction::Refridgerate {
					ingredient,
					time: Time { duration, unit },
				})
			}

			Keyword::Bake => {
				let ingredient = self.expect_reference()?;

				let degrees = self.expect_number()?;
				let temperature_unit_id = self.expect_identifier()?;
				let temperature_unit = TemperatureUnit::from_str(&temperature_unit_id)
					.map_err(|_| format!("Couldn't parse {:?} as TemperatureUnit", temperature_unit_id))?;

				let duration = self.expect_number()?;
				let time_unit_id = self.expect_identifier()?;
				let time_unit =
					TimeUnit::from_str(&time_unit_id).map_err(|_| format!("Couldn't parse {:?} as TimeUnit", time_unit_id))?;

				Ok(Instruction::Bake {
					ingredient,
					temperature: Temperature {
						degrees,
						unit: temperature_unit,
					},
					time: Time {
						duration,
						unit: time_unit,
					},
				})
			}
		}
	}

	fn parse_instructions(&mut self) -> Result<Vec<Instruction>, String> {
		self.expect(vec![Token::Keyword(Keyword::Instructions), Token::Colon])?;

		let mut instructions = Vec::new();

		while !self.peek().is_none() {
			self.expect(vec![Token::Newline])?;
			instructions.push(self.parse_instruction()?);
		}

		return Ok(instructions);
	}

	pub fn parse(mut self) -> Result<Recipe, String> {
		self.parse_ingredients()?;
		let instructions = self.parse_instructions()?;

		Ok(Recipe {
			ingredients: self.mise_en_place,
			instructions,
		})
	}

	pub fn new(tokens: &'a Vec<Token>) -> Self {
		Parser {
			stream: tokens.iter().peekable(),
			mise_en_place: Default::default(),
		}
	}
}
