use std::{collections::HashMap, fmt, fs, io, path::Path};

use niji_macros::IntoLua;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{files::Files, types::color::Color};

#[derive(Debug, Clone, IntoLua, Serialize, Deserialize)]
#[lua_with("ToString::to_string")]
#[serde(rename_all = "lowercase")]
pub enum ColorScheme {
	Light,
	Dark
}

impl fmt::Display for ColorScheme {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Light => write!(f, "light"),
			Self::Dark => write!(f, "dark")
		}
	}
}

#[derive(Debug, Clone, IntoLua, Serialize, Deserialize)]
pub struct UiTheme {
	pub color_scheme: ColorScheme,
	pub background: Color,
	pub surface: Color,
	pub primary: Color,
	pub border: Color,
	pub text_background: Color,
	pub text_surface: Color,
	pub text_primary: Color,
	pub info: Color,
	pub warning: Color,
	pub error: Color
}

#[derive(Debug, Clone, IntoLua, Serialize, Deserialize)]
pub struct Palette {
	pub black: Color,
	pub dark_red: Color,
	pub dark_green: Color,
	pub orange: Color,
	pub dark_blue: Color,
	pub purple: Color,
	pub turquoise: Color,
	pub light_gray: Color,
	pub dark_gray: Color,
	pub bright_red: Color,
	pub bright_green: Color,
	pub yellow: Color,
	pub bright_blue: Color,
	pub magenta: Color,
	pub cyan: Color,
	pub white: Color
}

#[derive(Debug, Clone, IntoLua, Serialize, Deserialize)]
pub struct Theme {
	pub ui: UiTheme,
	pub palette: Palette
}

#[derive(Debug, Clone, IntoLua, Serialize, Deserialize)]
pub struct GeneralConfig {
	pub icons: Option<String>,
	pub cursor: Option<String>,
	pub cursor_size: Option<u32>,
	pub font_family: Option<String>,
	pub font_size: Option<u32>
}

#[derive(Debug, Default, Clone, IntoLua, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModuleConfig {
	#[default]
	Nil,
	String(String),
	Int(i64),
	Float(f64),
	Bool(bool),
	Vec(Vec<ModuleConfig>),
	Map(HashMap<String, ModuleConfig>)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	pub modules: Vec<String>,
	pub general: GeneralConfig,

	#[serde(flatten)]
	pub module_config: HashMap<String, ModuleConfig>
}

#[derive(Debug, Error)]
pub enum Error {
	#[error("Failed to read {0}: {1}")]
	Read(String, io::Error),

	#[error("Failed to write to {0}: {1}")]
	Write(String, io::Error),

	#[error("Invalid syntax in {0}: {1}")]
	Parse(String, toml::de::Error),

	#[error("Error while serializing: {0}")]
	Serialize(#[from] toml::ser::Error)
}

pub fn read<C, P>(path: P) -> Result<C, Error>
where
	C: for<'de> Deserialize<'de>,
	P: AsRef<Path>
{
	let config_str = fs::read_to_string(&path)
		.map_err(|e| Error::Read(path.as_ref().display().to_string(), e))?;
	let config = toml::from_str(&config_str)
		.map_err(|e| Error::Parse(path.as_ref().display().to_string(), e))?;
	Ok(config)
}
