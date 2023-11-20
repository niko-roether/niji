use std::{collections::HashMap, fmt, fs, io, path::Path};

use niji_macros::IntoLua;
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
	pub error: Color,
	pub text_info: Color,
	pub text_warning: Color,
	pub text_error: Color
}

fn color_display(text: &str, bg_col: Color, fg_col: Color) -> String {
	format!(
		"\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m {text} \x1b[0m",
		bg_col.r, bg_col.g, bg_col.b, fg_col.r, fg_col.g, fg_col.b
	)
}

impl fmt::Display for UiTheme {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "Color scheme: {}", self.color_scheme)?;
		writeln!(
			f,
			"{}",
			color_display("Background", self.background, self.text_background)
		)?;
		writeln!(
			f,
			"{}",
			color_display("Surface", self.surface, self.text_surface)
		)?;
		writeln!(
			f,
			"{}",
			color_display("Primary", self.primary, self.text_primary)
		)?;

		writeln!(f)?;

		writeln!(f, "{}", color_display("Info", self.info, self.text_info))?;
		writeln!(
			f,
			"{}",
			color_display("Warning", self.warning, self.text_warning)
		)?;
		writeln!(f, "{}", color_display("Error", self.error, self.text_error))?;

		Ok(())
	}
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

fn colored_square(color: Color) -> String {
	format!("\x1b[48;2;{};{};{}m   \x1b[0m", color.r, color.g, color.b)
}

impl fmt::Display for Palette {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(&colored_square(self.black))?;
		f.write_str(&colored_square(self.dark_red))?;
		f.write_str(&colored_square(self.dark_green))?;
		f.write_str(&colored_square(self.orange))?;
		f.write_str(&colored_square(self.dark_blue))?;
		f.write_str(&colored_square(self.purple))?;
		f.write_str(&colored_square(self.turquoise))?;
		f.write_str(&colored_square(self.light_gray))?;

		writeln!(f)?;

		f.write_str(&colored_square(self.dark_gray))?;
		f.write_str(&colored_square(self.bright_red))?;
		f.write_str(&colored_square(self.bright_green))?;
		f.write_str(&colored_square(self.yellow))?;
		f.write_str(&colored_square(self.bright_blue))?;
		f.write_str(&colored_square(self.magenta))?;
		f.write_str(&colored_square(self.cyan))?;
		f.write_str(&colored_square(self.white))?;

		Ok(())
	}
}

#[derive(Debug, Clone, IntoLua, Serialize, Deserialize)]
pub struct Theme {
	pub ui: UiTheme,
	pub palette: Palette
}

impl fmt::Display for Theme {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "{}\n{}", self.ui, self.palette)
	}
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

	#[error("Invalid syntax in {0}: {1}")]
	Parse(String, Box<toml::de::Error>)
}

pub fn read<C, P>(path: P) -> Result<C, Error>
where
	C: for<'de> Deserialize<'de>,
	P: AsRef<Path>
{
	let config_str = fs::read_to_string(&path)
		.map_err(|e| Error::Read(path.as_ref().display().to_string(), e))?;
	let config = toml::from_str(&config_str)
		.map_err(|e| Error::Parse(path.as_ref().display().to_string(), Box::new(e)))?;
	Ok(config)
}
