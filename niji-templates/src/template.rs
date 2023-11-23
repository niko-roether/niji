use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Name {
	Full(Vec<String>),
	Inherent
}

impl fmt::Display for Name {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Inherent => write!(f, "."),
			Self::Full(segments) => write!(f, "{}", segments.join("."))
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Section {
	pub name: Name,
	pub inverted: bool,
	pub content: Vec<Token>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Insert {
	pub name: Name,
	pub format: Option<String>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Token {
	String(String),
	Insert(Insert),
	Section(Section)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template {
	tokens: Vec<Token>
}

impl Template {
	pub(crate) fn new(tokens: Vec<Token>) -> Self {
		Self { tokens }
	}
}
