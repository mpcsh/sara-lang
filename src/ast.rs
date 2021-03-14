use std::collections::HashMap;

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

#[derive(Debug, PartialEq)]
pub struct Ingredient {
	pub name: String,
	pub amount: Amount,
}

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

#[derive(Debug, PartialEq)]
pub struct Recipe {
	pub ingredients: HashMap<String, Amount>,
	pub instructions: Vec<Instruction>,
}
