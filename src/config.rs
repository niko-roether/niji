use serde::{Deserialize, Serialize};

use crate::types::color::Color;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ColorScheme {
	Light,
	Dark
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalTheme {
	pub black: Color,
	pub red: Color,
	pub green: Color,
	pub yellow: Color,
	pub blue: Color,
	pub magenta: Color,
	pub cyan: Color,
	pub white: Color,
	pub bright_black: Color,
	pub bright_red: Color,
	pub bright_green: Color,
	pub bright_yellow: Color,
	pub bright_blue: Color,
	pub bright_magenta: Color,
	pub bright_cyan: Color,
	pub bright_white: Color
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
	pub ui: UiTheme,
	pub terminal: TerminalTheme
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	pub icons: String,
	pub cursor: String,
	pub cursor_size: u32,
	pub font_family: String,
	pub font_size: u32
}
