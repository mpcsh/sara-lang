use std::collections::HashMap;

use crate::ast;
use crate::scan;

pub fn parse(tokens: Vec<scan::Token>) -> Result<ast::Recipe, String> {
	let recipe = ast::Recipe {
			ingredients: HashMap::new(),
			instructions: Vec::new(),
	};

	return Ok(recipe);
}
