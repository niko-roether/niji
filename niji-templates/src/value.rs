use std::collections::HashMap;

#[derive(Debug)]
pub enum Value {
	String(String),
	Bool(bool),
	Vec(Vec<Value>),
	Map(HashMap<String, Value>)
}

impl From<String> for Value {
	fn from(value: String) -> Self {
		Self::String(value)
	}
}

impl From<bool> for Value {
	fn from(value: bool) -> Self {
		Self::Bool(value)
	}
}

impl<I> FromIterator<I> for Value
where
	I: Into<Value>
{
	fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
		Self::Vec(iter.into_iter().map(Into::into).collect())
	}
}

impl<V> FromIterator<(String, V)> for Value
where
	V: Into<Value>
{
	fn from_iter<T: IntoIterator<Item = (String, V)>>(iter: T) -> Self {
		Self::Map(iter.into_iter().map(|(k, v)| (k, v.into())).collect())
	}
}

impl<I> From<Vec<I>> for Value
where
	I: Into<Value>
{
	fn from(value: Vec<I>) -> Self {
		value.into_iter().collect()
	}
}

impl<V> From<HashMap<String, V>> for Value
where
	V: Into<Value>
{
	fn from(value: HashMap<String, V>) -> Self {
		value.into_iter().collect()
	}
}
