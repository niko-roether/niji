use std::{
	env::{self, split_paths},
	fs::{self, read_dir, DirEntry, ReadDir},
	io,
	path::{Path, PathBuf}
};

use thiserror::Error;

use crate::utils::fs::{find_dirs, find_files};

#[derive(Debug, Error)]
pub enum InitError {
	#[error("The HOME environment variable is not set!")]
	NoHome,

	#[error("Failed to create {0}: {1}")]
	CreationFailed(String, io::Error)
}

#[derive(Debug)]
pub struct Files {
	config_file: PathBuf,
	current_theme_file: PathBuf,
	themes_dirs: Vec<PathBuf>,
	modules_dirs: Vec<PathBuf>
}

impl Files {
	const PREFIX: &'static str = "niji";
	const CONFIG_FILE: &'static str = "config.toml";
	const CURRENT_THEME_FILE: &'static str = "current_theme";
	const THEMES_DIR: &'static str = "themes";
	const MODULES_DIR: &'static str = "modules";

	pub fn init() -> Result<Self, InitError> {
		let Some(home) = env::var_os("HOME").map(PathBuf::from) else {
			return Err(InitError::NoHome);
		};

		let mut config_dir = env::var_os("XDG_CONFIG_HOME")
			.map(PathBuf::from)
			.unwrap_or_else(|| home.join(".config"));

		config_dir.push(Self::PREFIX);

		init_dir(&config_dir)?;

		let config_file = config_dir.join(Self::CONFIG_FILE);
		let current_theme_file = config_dir.join(Self::CURRENT_THEME_FILE);
		let custom_themes_dir = config_dir.join(Self::THEMES_DIR);
		let custom_modules_dir = config_dir.join(Self::MODULES_DIR);

		init_dir(&custom_themes_dir)?;
		init_dir(&custom_modules_dir)?;

		let mut themes_dirs = vec![custom_themes_dir];
		let mut modules_dirs = vec![custom_modules_dir];

		let mut data_dirs = env::var_os("XDG_DATA_DIRS")
			.map(|a| split_paths(&a).map(PathBuf::from).collect::<Vec<_>>())
			.unwrap_or_else(|| vec!["/usr/local/share".into(), "/usr/share".into()]);

		data_dirs.iter_mut().for_each(|dir| dir.push(Self::PREFIX));

		let builtin_themes_dirs = data_dirs.iter().map(|dir| dir.join(Self::THEMES_DIR));
		themes_dirs.extend(builtin_themes_dirs);

		let builtin_modules_dirs = data_dirs.iter().map(|dir| dir.join(Self::MODULES_DIR));
		modules_dirs.extend(builtin_modules_dirs);

		Ok(Self {
			config_file,
			current_theme_file,
			themes_dirs,
			modules_dirs
		})
	}

	#[inline]
	pub fn config_file(&self) -> &Path {
		&self.config_file
	}

	#[inline]
	pub fn current_theme_file(&self) -> &Path {
		&self.current_theme_file
	}

	pub fn iter_themes(&self) -> impl Iterator<Item = PathBuf> + '_ {
		find_files(&self.themes_dirs)
	}

	pub fn iter_modules(&self) -> impl Iterator<Item = PathBuf> + '_ {
		find_dirs(&self.modules_dirs)
	}
}

fn init_dir(dir: &Path) -> Result<(), InitError> {
	fs::create_dir_all(dir).map_err(|err| InitError::CreationFailed(dir.display().to_string(), err))
}

fn iter_valid_entries(dir: &Path) -> impl Iterator<Item = PathBuf> {
	read_dir(dir)
		.into_iter()
		.flatten()
		.flatten()
		.map(|entry| entry.path())
}
