use std::{path::PathBuf, rc::Rc};

use log::{debug, error, info, warn};
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
		reload: bool,
		filter: Option<&[&str]>
	) -> Result<(), Error> {
		for ActiveModule { name, path } in &self.active_modules {
			heading!("{name}");

			let module = match Module::load(&self.lua_runtime, path) {
				Ok(module) => module,
				Err(error) => {
					error!("{error}");
					println!();
					continue;
				}
			};

			if let Some(filter) = filter {
				if !filter.contains(&name.as_str()) {
					continue;
				}
			}

			let mut module_config = config.global.clone();
			if let Some(specific) = config.module_config.get(name) {
				module_config.extend(specific.clone().into_iter());
			}

			if let Err(err) = module.apply(module_config, theme.clone()) {
				error!("{err}");
				error!("Aborting module execution");
				println!();
				continue;
			}
			if reload {
				if config.disable_reloads.is_disabled(name) {
					info!(
						"Reloading is disabled for module {name}. You will only see the changes \
						 after a restart"
					)
				} else if module.can_reload() {
					info!("Reloading...");
					if let Err(err) = module.reload() {
						error!("{err}");
						error!("Reloading of {name} failed");
						println!();
					}
				} else {
					warn!(
						"Module {name} does not support reloading. You will only see the changes \
						 on a restart."
					)
				}
			}
			info!("Done!");
			println!();
		}
		Ok(())
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
