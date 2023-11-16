use std::{
	fs::{self, read_dir},
	io,
	path::{Path, PathBuf}
};

use thiserror::Error;

use crate::utils::{
	fs::{find_dirs, find_files},
	xdg::XdgDirs
};

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
	managed_files_file: PathBuf,
	themes_dirs: Vec<PathBuf>,
	modules_dirs: Vec<PathBuf>
}

impl Files {
	const PREFIX: &'static str = "niji";
	const CONFIG_FILE: &'static str = "config.toml";
	const CURRENT_THEME_FILE: &'static str = "current_theme.txt";
	const MANAGED_FILES_FILE: &'static str = "managed_files.csv";
	const THEMES_DIR: &'static str = "themes";
	const MODULES_DIR: &'static str = "modules";

	pub fn init(xdg: &XdgDirs) -> Result<Self, InitError> {
		let config_dir = xdg.config_home.join(Self::PREFIX);
		let state_dir = xdg.state_home.join(Self::PREFIX);

		init_dir(&config_dir)?;
		init_dir(&state_dir)?;

		let config_file = config_dir.join(Self::CONFIG_FILE);
		let current_theme_file = state_dir.join(Self::CURRENT_THEME_FILE);
		let managed_files_file = state_dir.join(Self::MANAGED_FILES_FILE);
		let custom_themes_dir = config_dir.join(Self::THEMES_DIR);
		let custom_modules_dir = config_dir.join(Self::MODULES_DIR);

		init_dir(&custom_themes_dir)?;
		init_dir(&custom_modules_dir)?;

		let mut themes_dirs = vec![custom_themes_dir];
		let mut modules_dirs = vec![custom_modules_dir];

		let data_dirs = xdg
			.data_dirs
			.iter()
			.map(|d| d.join(Self::PREFIX))
			.collect::<Vec<_>>();

		let builtin_themes_dirs = data_dirs.iter().map(|dir| dir.join(Self::THEMES_DIR));
		themes_dirs.extend(builtin_themes_dirs);

		let builtin_modules_dirs = data_dirs.iter().map(|dir| dir.join(Self::MODULES_DIR));
		modules_dirs.extend(builtin_modules_dirs);

		Ok(Self {
			config_file,
			current_theme_file,
			managed_files_file,
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

	#[inline]
	pub fn managed_files_file(&self) -> &Path {
		&self.managed_files_file
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