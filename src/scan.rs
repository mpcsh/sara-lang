use std::error::Error;
use std::fmt;
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

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
	Newline,
	Colon,
	Comma,
	Identifier(String),
	Keyword(Keyword),
	Number(f64),
}

#[derive(Debug)]
pub struct ScanError {
	source_text: String,
	description: String,
	line: usize,
	column: usize,
}

impl fmt::Display for ScanError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let leading_whitespace = self
			.source_text
			.chars()
			.take_while(|c| c.is_whitespace())
			.collect::<String>();
		writeln!(f, "Scan error: {}", self.description)?;
		writeln!(f, "--> {}:{}", self.line, self.column + leading_whitespace.len())?;
		writeln!(f, "{}", self.source_text)
	}
}

impl Error for ScanError {}

pub type ScanResult<T> = Result<T, ScanError>;

pub struct Scanner<'a> {
	source_text: &'a String,
	stream: Peekable<Chars<'a>>,
	tokens: Vec<Token>,
	line: usize,
	column: usize,
}

impl<'a> Scanner<'a> {
	fn next(&mut self) -> Option<char> {
		let next = self.stream.next();
		if next == Some('\n') {
			self.line += 1;
			self.column = 0;
		} else {
			self.column += 1;
		}
		next
	}

	fn peek(&mut self) -> Option<&char> {
		self.stream.peek()
	}

	fn push(&mut self, token: Token) {
		self.tokens.push(token)
	}

	fn error(&self, description: String) -> ScanError {
		ScanError {
			source_text: self.source_text.split("\n").nth(self.line - 1).unwrap().to_string(),
			description: description,
			line: self.line,
			column: self.column,
		}
	}

	fn next_number(&mut self) -> ScanResult<f64> {
		let source = self
			.stream
			.peeking_take_while(|&c| c.is_numeric() || c == '.' || c == ',')
			.collect::<String>();

		self.column += source.len();

		source
			.parse::<f64>()
			.map_err(|_| self.error(format!("Failed to scan \"{:?}\" as a number", source)))
	}

	fn next_word(&mut self) -> ScanResult<String> {
		let word = self
			.stream
			.peeking_take_while(|&c| c.is_alphabetic() || c == '-')
			.collect::<String>();

		self.column += word.len();

		if word == "" {
			Err(self.error("Expected word, but got none".to_string()))
		} else {
			Ok(word)
		}
	}

	pub fn scan(mut self) -> ScanResult<Vec<Token>> {
		loop {
			match self.peek() {
				None => {
					return Ok(self.tokens);
				}
				Some('\n') => {
					self.push(Token::Newline);
					self.next();
				}
				Some(&c) if c.is_whitespace() => {
					self.next();
				}
				Some(':') => {
					self.push(Token::Colon);
					self.next();
				}
				Some(',') => {
					self.push(Token::Comma);
					self.next();
				}
				Some(c) if c.is_numeric() => {
					let number = self.next_number()?;
					self.push(Token::Number(number));
				}
				Some(c) if c.is_alphabetic() => {
					let word = self.next_word()?;
					// if it's a keyword, push a keyword
					if let Ok(keyword) = Keyword::from_str(&word) {
						self.push(Token::Keyword(keyword));
					}
					// else if the last token was an identifier, concatenate
					else if let Some(Token::Identifier(id)) = self.tokens.last_mut() {
						id.push(' ');
						id.push_str(&word);
					}
					// else, push a new identifier
					else {
						self.push(Token::Identifier(word));
					}
				}
				Some(c) => {
					let description = format!("Unrecognized input \"{:?}\"", c);
					return Err(self.error(description));
				}
			};
		}
	}

	pub fn new(source_text: &'a String) -> Self {
		Scanner {
			source_text,
			stream: source_text.chars().peekable(),
			tokens: Default::default(),
			line: 1,
			column: 1,
		}
	}
}
