use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString)]
pub enum IngredientUnit {
	#[strum(serialize = "g")]
	Grams,

	#[strum(serialize = "oz")]
	Ounces,

	#[strum(serialize = "cup", serialize = "cups")]
	Cups,

	#[strum(serialize = "tsp")]
	Teaspoons,

	#[strum(serialize = "tbsp")]
	Tablespoons,

	#[strum(disabled)]
	Units,
}

#[derive(Debug, PartialEq, EnumString)]
pub enum Temperature {
	#[strum(serialize = "F")]
	Fahrenheit,

	#[strum(serialize = "C")]
	Celsius,
}

#[derive(Debug, PartialEq, EnumString)]
pub enum Time {
	#[strum(serialize = "s")]
	Seconds,

	#[strum(serialize = "min", serialize = "mins")]
	Minutes,

	#[strum(serialize = "hr", serialize = "hrs")]
	Hours,
}

#[derive(Debug, PartialEq)]
pub struct Amount {
	pub quantity: f64,
	pub unit: IngredientUnit,
}

#[derive(Debug)]
pub struct Ingredient {
	pub name: String,
	pub amount: Amount,
}

impl Hash for Ingredient {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.name.hash(state);
	}
}

impl PartialEq for Ingredient {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}

impl Eq for Ingredient {}

#[derive(Debug, PartialEq)]
pub enum Instruction {
	Combine {
		ingredients: Vec<Ingredient>,
	},
	CutInto {
		source: Ingredient,
		destination: Ingredient,
	},
	Mix {
		ingredients: Vec<Ingredient>,
	},
	Refridgerate {
		ingredient: Ingredient,
		time: Time,
	},
	Bake {
		ingredient: Ingredient,
		temperature: Temperature,
		time: Time,
	},
}

#[derive(Debug)]
pub struct Recipe {
	pub ingredients: HashSet<Ingredient>,
	pub instructions: Vec<Instruction>,
}
