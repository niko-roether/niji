use std::{io, path::Path};

use thiserror::Error;

use crate::{
	config::{ModuleConfig, Theme},
	lua::runtime::{LuaModule, LuaRuntime}
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

pub struct Module<'lua>(LuaModule<'lua>);

impl<'lua> Module<'lua> {
	pub fn load(runtime: &'lua LuaRuntime, path: &Path) -> Result<Self, LoadError> {
		let module = runtime.load_lua_module(path)?;
		Ok(Self(module))
	}

	pub fn apply(&self, config: ModuleConfig, theme: Theme) -> Result<(), ExecError> {
		Ok(self.0.call("apply", (config, theme))?)
	}
}
