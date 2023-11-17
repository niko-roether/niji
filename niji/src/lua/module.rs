use std::{io, path::Path};

use thiserror::Error;

use crate::config::{Config, Theme};

use super::runtime::LuaRuntime;

#[derive(Debug, Error)]
pub enum LoadError {
	#[error("Failed to read module files: {0}")]
	FileReadErr(#[from] io::Error),

	#[error("{0}")]
	LuaErr(#[from] mlua::Error)
}

#[derive(Debug, Error)]
pub enum ExecError {
	#[error("{0}")]
	LuaErr(#[from] mlua::Error)
}

pub struct Module<'lua> {
	runtime: &'lua LuaRuntime,
	name: String,
	apply: Option<mlua::Function<'lua>>,
	configure: Option<mlua::Function<'lua>>
}

impl<'lua> Module<'lua> {
	pub fn load(runtime: &'lua LuaRuntime, path: &Path) -> Result<Self, LoadError> {
		let name = path.file_name().unwrap().to_string_lossy().into_owned();
		let entry_point = path.join("module.lua");

		let module: mlua::Table = runtime.load_module(&entry_point)?;
		let apply: Option<mlua::Function> = module.get("apply")?;
		let configure: Option<mlua::Function> = module.get("configure")?;

		Ok(Self {
			runtime,
			name,
			apply,
			configure
		})
	}

	pub fn configure(&self, config: &Config) -> Result<(), ExecError> {
		if let Some(configure) = &self.configure {
			self.in_context(|| configure.call(config.clone()))?;
		}
		Ok(())
	}

	pub fn apply(&self, theme: &Theme) -> Result<(), ExecError> {
		if let Some(apply) = &self.apply {
			self.in_context(|| apply.call(theme.clone()))?;
		}
		Ok(())
	}

	fn in_context(&self, cb: impl FnOnce() -> Result<(), mlua::Error>) -> Result<(), ExecError> {
		self.runtime.set_module_context(Some(&self.name));
		cb()?;
		self.runtime.set_module_context(None);

		Ok(())
	}
}
