use std::str::FromStr;

use thiserror::Error;

struct Section {
	name: String,
	content: Vec<Token>
}

enum Token {
	String(String),
	Value(String),
	Section(Section)
}

#[derive(Debug, Error)]
pub enum ParseError {}

pub struct Template {
	tokens: Vec<Token>
}

impl FromStr for Template {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut tokens = Vec::<Token>::new();

		Ok(Self { tokens })
	}
}
