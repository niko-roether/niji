use std::rc::Rc;

use thiserror::Error;

use crate::{
	config::{self, Config, ModuleConfig},
	file_manager::{self, FileManager},
	files::{self, Files},
	module_manager::{self, ModuleManager, ModuleManagerInit},
	theme_manager::{self, ThemeManager},
	utils::xdg::{self, XdgDirs}
};

#[derive(Debug, Error)]
#[error("{0}")]
pub enum InitError {
	Xdg(#[from] xdg::Error),
	Files(#[from] files::InitError),
	Config(#[from] config::Error),
	FileManager(#[from] file_manager::Error),
	ModuleManager(#[from] module_manager::Error)
}

pub struct NijiApp {
	xdg: Rc<XdgDirs>,
	files: Rc<Files>,
	config: Rc<Config>,
	file_manager: Rc<FileManager>,
	theme_manager: Rc<ThemeManager>,
	module_manager: Rc<ModuleManager>
}

macro_rules! err {
	($expr:expr) => {
		match $expr {
			Ok(value) => value,
			Err(error) => {
				crate::console::error!("{error}");
				return Err(());
			}
		}
	};
}

impl NijiApp {
	pub fn init() -> Result<Self, InitError> {
		let xdg = Rc::new(XdgDirs::new()?);
		let files = Rc::new(Files::new(&xdg)?);
		let config = Rc::<Config>::new(config::read(files.config_file())?);
		let file_manager = Rc::new(FileManager::new(Rc::clone(&files))?);
		let theme_manager = Rc::new(ThemeManager::new(Rc::clone(&files)));
		let module_manager = Rc::new(ModuleManager::new(ModuleManagerInit {
			xdg: Rc::clone(&xdg),
			files: Rc::clone(&files),
			config: Rc::clone(&config),
			file_manager: Rc::clone(&file_manager)
		})?);

		Ok(Self {
			xdg,
			files,
			config,
			file_manager,
			theme_manager,
			module_manager
		})
	}
}
