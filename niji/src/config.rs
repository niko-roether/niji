use std::{collections::HashMap, fmt};

use niji_macros::IntoLua;
use serde::{Deserialize, Serialize};

use crate::types::color::Color;

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
	pub icons: String,
	pub cursor: String,
	pub cursor_size: u32,
	pub font_family: String,
	pub font_size: u32
}

#[derive(Debug, Clone, IntoLua, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModuleConfig {
	String(String),
	Int(i64),
	Float(f64),
	Bool(bool),
	Vec(Vec<ModuleConfig>),
	Map(HashMap<String, ModuleConfig>)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	#[serde(flatten)]
	pub general: GeneralConfig,

	#[serde(flatten)]
	pub modules: HashMap<String, ModuleConfig>
}
