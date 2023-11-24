use std::{path::PathBuf, rc::Rc};

use log::{debug, error, info};
use niji_console::heading;
use thiserror::Error;

use crate::{
	config::{Config, Theme},
	file_manager::FileManager,
	files::Files,
	lua::runtime::{LuaRuntime, LuaRuntimeInit},
	module::Module,
	utils::xdg::XdgDirs
};

#[derive(Debug, Error)]
pub enum Error {
	#[error("Module \"{0}\" doesn't exist!")]
	UnknownModule(String),

	#[error("Failed to initialize lua runtime: {0}")]
	RuntimeInit(mlua::Error)
}

pub struct ModuleManagerInit {
	pub xdg: Rc<XdgDirs>,
	pub files: Rc<Files>,
	pub config: Rc<Config>,
	pub file_manager: Rc<FileManager>
}

struct ActiveModule {
	name: String,
	path: PathBuf
}

pub struct ModuleManager {
	active_modules: Vec<ActiveModule>,
	lua_runtime: LuaRuntime
}

impl ModuleManager {
	pub fn new(
		ModuleManagerInit {
			xdg,
			files,
			config,
			file_manager
		}: ModuleManagerInit
	) -> Result<Self, Error> {
		let mut active_modules = Vec::<ActiveModule>::with_capacity(config.modules.len());
		for mod_name in &config.modules {
			let module_dir = Self::find_module_dir(&files, mod_name)
				.ok_or_else(|| Error::UnknownModule(mod_name.clone()))?;

			debug!(
				"Activating module \"{mod_name}\" at path {}",
				module_dir.display()
			);

			active_modules.push(ActiveModule {
				name: mod_name.to_string(),
				path: module_dir
			});
		}

		let lua_runtime = LuaRuntime::new(LuaRuntimeInit {
			xdg: Rc::clone(&xdg),
			files: Rc::clone(&files),
			file_manager: Rc::clone(&file_manager)
		})
		.map_err(Error::RuntimeInit)?;

		Ok(Self {
			active_modules,
			lua_runtime
		})
	}

	pub fn apply(
		&self,
		config: &Config,
		theme: &Theme,
		filter: Option<&[&str]>
	) -> Result<(), Error> {
		for (name, module) in self.iter_loaded_modules() {
			if let Some(filter) = filter {
				if !filter.contains(&name) {
					continue;
				}
			}

			let mut module_config = config.global.clone();
			if let Some(specific) = config.module_config.get(name) {
				module_config.extend(specific.clone().into_iter());
			}

			heading!("{name}");
			if let Err(err) = module.apply(module_config, theme.clone()) {
				error!("{err}");
				error!("Aborting module execution");
				println!();
				continue;
			}
			info!("Done!");
			println!();
		}
		Ok(())
	}

	fn iter_loaded_modules(&self) -> impl Iterator<Item = (&str, Module)> {
		self.active_modules
			.iter()
			.filter_map(
				|ActiveModule { path, name }| match Module::load(&self.lua_runtime, path) {
					Ok(m) => Some((name.as_str(), m)),
					Err(err) => {
						error!("{err}");
						None
					}
				}
			)
	}

	fn find_module_dir(files: &Files, name: &str) -> Option<PathBuf> {
		for module_dir in files.iter_modules() {
			let Some(dirname) = module_dir.file_name() else {
				continue;
			};
			if dirname.to_str() == Some(name) {
				return Some(module_dir);
			}
		}

		None
	}
}
