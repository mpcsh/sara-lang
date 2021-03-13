use std::collections::VecDeque;
use std::str::Chars;

use std::str::FromStr;
use strum_macros::EnumString;

type Scan<T> = Result<T, String>;

#[derive(Debug, EnumString)]
pub enum Keyword {
  // Program sigils
  Ingredients,
  Instructions,
  Result,
  
  // Instructions
  Combine,
  Cut, Into,
  Mix,
  Refridgerate,
  Bake,
}

#[derive(Debug)]
pub enum Token {
  Colon,
  Comma,
  Identifier(String),
  Keyword(Keyword),
  Number(f64),
}

fn keyword(first: char, chars: &mut Chars) -> Result<Keyword, String> {
  let mut rest = chars.take_while(|c|
    ('a'..='z').contains(&c) ||
    ('A'..='Z').contains(&c)
  ).collect::<VecDeque<char>>();
  
  rest.push_front(first);
  let id = rest.into_iter().collect::<String>();
  Keyword::from_str(&id).map_err(|_| id)
}

// fn identifier(word) -> Scan<String> {
//   let rest = chars.take_while(|c|
//     ('a'..='z').contains(&c) ||
//     ('A'..='Z').contains(&c) ||
//     ['-', ' '].contains(&c)
//   ).collect::<String>();

//   return Ok(format!("{}{}", first, rest));
// }

pub fn scan(source_text: String) -> Scan<Vec<Token>> {
  let mut tokens = Vec::new();
  
  let words = &mut source_text.split(char::is_whitespace);
  loop {
    let word = match words.next() {
      Some(w) => w,
      None => break,
    };

    tokens.push(
      match word {
        ":" => Token::Colon,
        "," => Token::Comma,
        _ if let Ok(keyword) = Keyword::from_str(word) => Token::Keyword(keyword),
        _ => Token::Identifier(word.to_string())
      }
    );
  }

  return Ok(tokens);
}