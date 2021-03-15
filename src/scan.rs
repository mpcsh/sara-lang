use std::iter::Peekable;
use std::str::{Chars, FromStr};

use itertools::Itertools;
use strum_macros::{Display, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Display, EnumString)]
pub enum Keyword {
	// Program sigils
	Ingredients,
	Instructions,

	// Instructions
	Combine,
	Mix,
	Cut,
	#[strum(serialize = "into")]
	Into,
	Refridgerate,
	Bake,
}

#[derive(Debug, PartialEq)]
pub enum Token {
	Whitespace,
	Colon,
	Comma,
	Identifier(String),
	Keyword(Keyword),
	Number(f64),
}

pub struct Scanner<'a> {
	stream: Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
	fn next(&mut self) -> Option<char> {
		self.stream.next()
	}

	fn peek(&mut self) -> Option<&char> {
		self.stream.peek()
	}

	fn next_number(&mut self) -> Result<f64, String> {
		let source = self.stream
			.peeking_take_while(|&c| c.is_numeric() || c == '.' || c == ',')
			.collect::<String>();

		source
			.parse::<f64>()
			.map_err(|_| format!("Failed to scan \"{:?}\" as a number", source))
	}

	fn next_word(&mut self) -> Option<String> {
		let word = self.stream
			.peeking_take_while(|&c| c.is_alphabetic() || c == '-')
			.collect::<String>();

		if word == "" {
			None
		} else {
			Some(word)
		}
	}

	pub fn scan(mut self) -> Result<Vec<Token>, String> {
		let mut tokens = Vec::new();

		loop {
			match self.peek() {
				None => { return Ok(tokens); }
				Some(&c) if c.is_whitespace() && c != ' ' => {
					let last_token = tokens.last();
					match last_token {
						Some(Token::Whitespace) => {}
						Some(_) | None => { tokens.push(Token::Whitespace); }
					}
					self.next();
				}
				Some(' ') => { self.next(); }
				Some(':') => {
					tokens.push(Token::Colon);
					self.next();
				}
				Some(',') => {
					tokens.push(Token::Comma);
					self.next();
				}
				Some(c) if c.is_numeric() => {
					tokens.push(Token::Number(self.next_number()?));
				}
				Some(c) if c.is_alphabetic() => {
					match self.next_word() {
						None => unreachable!(),
						Some(word) if let Ok(keyword) = Keyword::from_str(&word) => {
						tokens.push(Token::Keyword(keyword));
						}
						Some(word) if let Some(Token::Identifier(id)) = tokens.last_mut() => {
							id.push(' ');
							id.push_str(&word);
						}
						Some(word) => {
						tokens.push(Token::Identifier(word));
						}
					}
				}
				Some(c) => {
					return Err(format!("Unrecognized input \"{:?}\"", c));
				}
			};
		}
	}

	pub fn new(source_text: &'a String) -> Self {
		Scanner { stream: source_text.chars().peekable() }
	}
}

pub fn reassemble(tokens: Vec<Token>) -> String {
	tokens
		.iter()
		.map(|t| match t {
			Token::Whitespace => "\n".to_string(),
			Token::Colon => ":".to_string(),
			Token::Comma => ",".to_string(),
			Token::Identifier(identifier) => identifier.to_string(),
			Token::Keyword(keyword) => keyword.to_string(),
			Token::Number(number) => number.to_string(),
		})
		.collect::<Vec<String>>()
		.iter()
		.flat_map(|s| s.chars())
		.collect()
}
