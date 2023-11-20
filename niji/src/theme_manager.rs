use std::{ffi::OsString, fs, io, path::PathBuf, rc::Rc};

use thiserror::Error;

use crate::{
	config::{self, Theme},
	files::Files
};

#[derive(Debug, Error)]
pub enum Error {
	#[error("Failed to access theme state: {0}")]
	AccessThemeState(io::Error),

	#[error("{0}")]
	ThemeAccess(#[from] config::Error),

	#[error("Theme \"{0}\" doesn't exist!")]
	UnknownTheme(String)
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
		self.files
			.iter_themes()
			.filter_map(|path| path.file_stem().map(|os| os.to_string_lossy().into_owned()))
			.collect()
	}

	pub fn current_theme(&self) -> Result<Option<NamedTheme>, Error> {
		if !self.files.current_theme_file().exists() {
			self.reset_theme();
		}

		let current_theme =
			fs::read_to_string(self.files.current_theme_file()).map_err(Error::AccessThemeState)?;

		if current_theme.is_empty() {
			return Ok(None);
		}

		let values: Theme = config::read(&current_theme)?;

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
		self.files
			.iter_themes()
			.find(|f| f.file_stem() == Some(&OsString::from(name)))
	}

	fn read_theme(&self, name: &str) -> Result<Option<Theme>, Error> {
		let Some(path) = self.find_theme_path(name) else {
			return Ok(None);
		};

		Ok(Some(config::read(path)?))
	}
}
