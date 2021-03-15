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

#[derive(Debug, PartialEq)]
pub struct Amount {
	pub quantity: f64,
	pub unit: IngredientUnit,
}

#[derive(Debug, PartialEq, EnumString)]
pub enum TemperatureUnit {
	#[strum(serialize = "F")]
	Fahrenheit,

	#[strum(serialize = "C")]
	Celsius,
}

#[derive(Debug, PartialEq)]
pub struct Temperature {
	pub degrees: f64,
	pub unit: TemperatureUnit,
}

#[derive(Debug, PartialEq, EnumString)]
pub enum TimeUnit {
	#[strum(serialize = "s")]
	Seconds,

	#[strum(serialize = "min", serialize = "mins")]
	Minutes,

	#[strum(serialize = "hr", serialize = "hrs")]
	Hours,
}

#[derive(Debug, PartialEq)]
pub struct Time {
	pub duration: f64,
	pub unit: TimeUnit,
}

#[derive(Debug)]
pub struct Ingredient {
	pub name: String,
	pub amount: Amount,
}

impl Hash for Ingredient {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.name.to_lowercase().hash(state);
	}
}

impl PartialEq for Ingredient {
	fn eq(&self, other: &Self) -> bool {
		self.name.to_lowercase() == other.name.to_lowercase()
	}
}

impl Eq for Ingredient {}

#[derive(Debug)]
pub enum Reference<'a> {
	Result,
	Ingredient(&'a Ingredient),
}

#[derive(Debug)]
pub enum Instruction<'a> {
	Combine {
		ingredients: Vec<Reference<'a>>,
	},
	CutInto {
		source: Reference<'a>,
		destination: Reference<'a>,
	},
	Refridgerate {
		ingredient: Reference<'a>,
		time: Time,
	},
	Bake {
		ingredient: Reference<'a>,
		temperature: Temperature,
		time: Time,
	},
}

#[derive(Debug)]
pub struct Recipe<'a> {
	pub ingredients: HashSet<Ingredient>,
	pub instructions: Vec<Instruction<'a>>,
}
