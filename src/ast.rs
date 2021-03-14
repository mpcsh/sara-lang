use std::collections::HashMap;

#[derive(Debug)]
pub enum Weight {
	Grams,
	Kilograms,
	Ounces,
	Pounds,
}

#[derive(Debug)]
pub enum Volume {
	Cups,
	Teaspoons,
	Tablespoons,
}

#[derive(Debug)]
pub enum Temperature {
	Fahrenheit,
	Celsius,
}

#[derive(Debug)]
pub enum Time {
	Seconds,
	Minutes,
	Hours,
}

#[derive(Debug)]
pub enum Unit {
	Weight,
	Volume,
	Temperature,
	Time,
}

#[derive(Debug)]
pub struct Amount {
	pub quantity: f64,
	pub unit: Unit,
}

#[derive(Debug)]
pub struct Ingredient {
	pub name: String,
	pub amount: Amount,
}

#[derive(Debug)]
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
	pub ingredients: HashMap<String, Amount>,
	pub instructions: Vec<Instruction>,
}
