use std::{
	fs::{self, File},
	io::{self, Read},
	path::PathBuf
};

use thiserror::Error;

use crate::config::Theme;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Failed to list themes: {0}")]
	List(io::Error),

	#[error("Failed to access theme state: {0}")]
	AccessThemeState(io::Error),

	#[error("Failed to read theme at {0}: {1}")]
	ReadTheme(String, io::Error),

	#[error("Failed to read current theme; The file was deleted")]
	ReadCurrentTheme,

	#[error("Invalid TOML in theme at {0}: {1}")]
	ThemeSyntax(String, toml::de::Error),

	#[error("Theme \"{0}\" doesn't exist!")]
	UnknownTheme(String)
}

pub struct ThemeManager {
	current_theme_file: PathBuf,
	themes_dir: PathBuf
}

impl ThemeManager {
	pub fn list_themes(&self) -> Result<Vec<String>, Error> {
		Ok(self
			.themes_dir
			.read_dir()
			.map_err(Error::List)?
			.filter_map(|e| {
				let entry = e.ok()?;
				if entry.file_type().ok()?.is_file() {
					Some(entry.path().file_stem()?.to_string_lossy().into_owned())
				} else {
					None
				}
			})
			.collect())
	}

	pub fn current_theme(&self) -> Result<Theme, Error> {
		let current_theme =
			fs::read_to_string(&self.current_theme_file).map_err(Error::AccessThemeState)?;

		self.read_theme(&current_theme)
	}

	pub fn set_theme(&self, name: String) -> Result<(), Error> {
		let path = self.path_for_theme(&name);
		if !path.exists() {
			return Err(Error::UnknownTheme(name));
		}
		fs::write(&self.current_theme_file, name).map_err(Error::AccessThemeState)?;
		Ok(())
	}

	fn path_for_theme(&self, name: &str) -> PathBuf {
		self.themes_dir.join(format!("{name}.toml"))
	}

	fn read_theme(&self, name: &str) -> Result<Theme, Error> {
		let path = self.path_for_theme(name);

		let theme_str = fs::read_to_string(&path)
			.map_err(|e| Error::ReadTheme(path.display().to_string(), e))?;

		let theme: Theme = toml::from_str(&theme_str)
			.map_err(|e| Error::ThemeSyntax(path.display().to_string(), e))?;

		Ok(theme)
	}
}
