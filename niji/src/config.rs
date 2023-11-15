use std::fmt;

use niji_macros::IntoLua;
use serde::{Deserialize, Serialize};

use crate::types::color::Color;

#[derive(Debug, Clone, IntoLua, Serialize, Deserialize)]
#[lua(as_string)]
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
	pub terminal: Palette
}

#[derive(Debug, Clone, IntoLua, Serialize, Deserialize)]
pub struct Config {
	pub icons: String,
	pub cursor: String,
	pub cursor_size: u32,
	pub font_family: String,
	pub font_size: u32
}
