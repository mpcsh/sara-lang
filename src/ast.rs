use std::collections::HashMap;

use strum_macros::EnumString;
use unicase::UniCase;

#[derive(Debug, PartialEq, EnumString)]
pub enum AmountUnit {
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
	pub unit: AmountUnit,
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

pub type Ingredient = UniCase<String>;

#[derive(Debug)]
pub enum Instruction {
	Combine {
		ingredients: Vec<Ingredient>,
	},
	CutInto {
		source: Ingredient,
		destination: Ingredient,
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
	pub ingredients: HashMap<Ingredient, Amount>,
	pub instructions: Vec<Instruction>,
}
