use std::{collections::HashSet, ffi::OsString, fs, io, path::PathBuf, rc::Rc};

use thiserror::Error;

use crate::{
	config::{self, Theme},
	console,
	files::Files
};

#[derive(Debug, Error)]
pub enum Error {
	#[error("Failed to access theme state: {0}")]
	AccessThemeState(io::Error),

	#[error("Couldn't read theme {0}; {1}")]
	ThemeRead(String, config::Error),

	#[error("Theme \"{0}\" doesn't exist!")]
	UnknownTheme(String),

	#[error("Current theme is \"{0}\", but that theme doesn't exist!")]
	UnknownCurrentTheme(String)
}

pub struct ThemeManager {
	files: Rc<Files>
}

#[derive(Debug, Clone)]
pub struct NamedTheme {
	pub name: String,
	pub values: Theme
}

impl ThemeManager {
	pub fn new(files: Rc<Files>) -> Self {
		Self { files }
	}

	pub fn list_themes(&self) -> Vec<String> {
		let mut themes = HashSet::new();
		for path in self.files.iter_themes() {
			let Some(os_name) = path.file_stem() else {
				continue;
			};
			let name = os_name.to_string_lossy().into_owned();
			if themes.insert(name.clone()) {
				console::debug!("Found theme {name} at {}", path.display());
			}
		}
		themes.into_iter().collect()
	}

	pub fn current_theme(&self) -> Result<Option<NamedTheme>, Error> {
		if !self.files.current_theme_file().exists() {
			self.reset_theme()?;
		}

		let current_theme =
			fs::read_to_string(self.files.current_theme_file()).map_err(Error::AccessThemeState)?;

		if current_theme.is_empty() {
			return Ok(None);
		}

		let values: Option<Theme> = self.read_theme(&current_theme)?;
		let Some(values) = values else {
			return Err(Error::UnknownCurrentTheme(current_theme));
		};

		Ok(Some(NamedTheme {
			name: current_theme,
			values
		}))
	}

	pub fn set_theme(&self, name: String) -> Result<(), Error> {
		if self.find_theme_path(&name).is_none() {
			return Err(Error::UnknownTheme(name));
		}
		fs::write(self.files.current_theme_file(), name).map_err(Error::AccessThemeState)?;
		Ok(())
	}

	pub fn reset_theme(&self) -> Result<(), Error> {
		fs::write(self.files.current_theme_file(), "").map_err(Error::AccessThemeState)?;
		Ok(())
	}

	fn find_theme_path(&self, name: &str) -> Option<PathBuf> {
		let Some(path) = self
			.files
			.iter_themes()
			.find(|f| f.file_stem() == Some(&OsString::from(name)))
		else {
			return None;
		};

		Some(path)
	}

	fn read_theme(&self, name: &str) -> Result<Option<Theme>, Error> {
		let Some(path) = self.find_theme_path(name) else {
			return Ok(None);
		};

		console::debug!("Reading theme \"{name}\" from {}", path.display());

		Ok(Some(
			config::read(path).map_err(|e| Error::ThemeRead(name.to_string(), e))?
		))
	}
}