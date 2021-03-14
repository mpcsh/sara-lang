use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq)]
pub enum Weight {
	Grams,
	Kilograms,
	Ounces,
	Pounds,
}

#[derive(Debug, PartialEq)]
pub enum Volume {
	Cups,
	Teaspoons,
	Tablespoons,
}

#[derive(Debug, PartialEq)]
pub enum Temperature {
	Fahrenheit,
	Celsius,
}

#[derive(Debug, PartialEq)]
pub enum Time {
	Seconds,
	Minutes,
	Hours,
}

#[derive(Debug, PartialEq)]
pub enum Unit {
	Weight,
	Volume,
	Temperature,
	Time,
}

#[derive(Debug, PartialEq)]
pub struct Amount {
	pub quantity: f64,
	pub unit: String,
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
