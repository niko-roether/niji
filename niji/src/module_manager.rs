use std::{path::PathBuf, rc::Rc};

use thiserror::Error;

use crate::{
	config::{Config, GeneralConfig, ModuleConfig, Theme},
	console,
	file_manager::FileManager,
	files::Files,
	lua::runtime::{LuaRuntime, LuaRuntimeInit},
	module::{self, Module},
	utils::xdg::XdgDirs
};

#[derive(Debug, Error)]
pub enum Error {
	#[error("Module \"{0}\" doesn't exist!")]
	UnknownModule(String),

	#[error("Failed to initialize lua runtime: {0}")]
	RuntimeInit(mlua::Error),

	#[error("Failed to load module {0}: {1}")]
	ModuleLoad(String, module::LoadError),

	#[error("Failed to execute module {0}: {1}")]
	ModuleExec(String, module::ExecError)
}

pub struct ModuleManagerInit {
	pub xdg: Rc<XdgDirs>,
	pub files: Rc<Files>,
	pub config: Rc<Config>,
	pub file_manager: Rc<FileManager>
}

struct ActiveModule {
	name: String,
	path: PathBuf,
	config: ModuleConfig
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

			let module_config = config
				.module_config
				.get(mod_name)
				.cloned()
				.unwrap_or_default();

			console::debug!(
				"Activating module \"{mod_name}\" at path {} with config {module_config:?}",
				module_dir.display()
			);

			active_modules.push(ActiveModule {
				name: mod_name.to_string(),
				path: module_dir,
				config: module_config
			});
		}

		let lua_runtime = LuaRuntime::new(LuaRuntimeInit {
			xdg: Rc::clone(&xdg),
			file_manager: Rc::clone(&file_manager)
		})
		.map_err(Error::RuntimeInit)?;

		Ok(Self {
			active_modules,
			lua_runtime
		})
	}

	pub fn configure(&self, config: &GeneralConfig) -> Result<(), Error> {
		for (name, module) in self.iter_loaded_modules() {
			console::debug!("Configuring module {name}");
			module
				.configure(config)
				.map_err(|e| Error::ModuleExec(name.to_string(), e))?;
		}
		Ok(())
	}

	pub fn apply(&self, theme: &Theme) -> Result<(), Error> {
		for (name, module) in self.iter_loaded_modules() {
			console::debug!("Applying theme to module {name}");
			module
				.apply(theme)
				.map_err(|e| Error::ModuleExec(name.to_string(), e))?;
		}
		Ok(())
	}

	fn iter_loaded_modules(&self) -> impl Iterator<Item = (&str, Module)> {
		self.active_modules
			.iter()
			.filter_map(|ActiveModule { path, config, name }| {
				match Module::load(&self.lua_runtime, path, config.clone()) {
					Ok(m) => Some((name.as_str(), m)),
					Err(err) => {
						console::error!("{err}");
						None
					}
				}
			})
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
