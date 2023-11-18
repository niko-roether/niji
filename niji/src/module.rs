use std::{io, path::Path};

use thiserror::Error;

use crate::{
	config::{GeneralConfig, ModuleConfig, Theme},
	lua::{self, runtime::LuaRuntime}
};

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

pub struct Module<'lua>(lua::runtime::Module<'lua>);

impl<'lua> Module<'lua> {
	pub fn load(
		runtime: &'lua LuaRuntime,
		path: &Path,
		config: ModuleConfig
	) -> Result<Self, LoadError> {
		let module = runtime.load_module(path, config)?;
		Ok(Self(module))
	}

	pub fn configure(&self, config: &GeneralConfig) -> Result<(), ExecError> {
		Ok(self.0.call("configure", config.clone())?)
	}

	pub fn apply(&self, theme: &Theme) -> Result<(), ExecError> {
		Ok(self.0.call("apply", theme.clone())?)
	}
}
