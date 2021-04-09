use std::collections::{HashMap, HashSet};

use unicase::UniCase;

use crate::ast::{Ingredient, Instruction, Recipe};

pub struct Baker<'a> {
	recipe: &'a Recipe,
	cookbook: HashMap<HashSet<Ingredient>, Ingredient>,
}

impl<'a> Baker<'a> {


	fn step(&self, instruction: Instruction) -> String {
		match instruction {
			Instruction::Combine { ingredients } => {
				let lookup = ingredients.iter().collect::<HashSet<Ingredient>>();
				match self.cookbook.get(&lookup) {
					Some(&product) => product,
					None => ingredients.iter()
						.intersperse("with").intersperse(" ")
						.flatten().collect()
				}
			}
			_ => unimplemented!()
		}

		unimplemented!()
	}

	pub fn bake(&self) -> (String, HashSet<UniCase<String>>) {
		for instruction in &self.recipe.instructions {
		}

		unimplemented!()
	}

	pub fn new(recipe: &'a Recipe) -> Self {
		Baker {
			recipe,
		}
	}
}