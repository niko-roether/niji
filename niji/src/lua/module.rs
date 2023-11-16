use std::{io, path::Path};

use thiserror::Error;

use crate::config::Theme;

use super::runtime::LuaRuntime;

#[derive(Debug, Error)]
pub enum LoadError {
	#[error("Failed to read module files: {0}")]
	FileReadErr(#[from] io::Error),

	#[error("{0}")]
	ExecErr(#[from] mlua::Error)
}

#[derive(Debug, Error)]
pub enum ApplyError {
	#[error("{0}")]
	ExecErr(#[from] mlua::Error)
}

pub struct Module<'lua> {
	name: String,
	apply: mlua::Function<'lua>
}

impl<'lua> Module<'lua> {
	pub fn load(lua: &'lua LuaRuntime, path: &Path) -> Result<Self, LoadError> {
		let name = path.file_name().unwrap().to_string_lossy().into_owned();
		let entry_point = path.join("module.lua");

		let module: mlua::Table = lua.run_module(&entry_point)?;
		let apply: mlua::Function = module.get("apply")?;

		Ok(Self { name, apply })
	}

	pub fn apply(&self, theme: &Theme) -> Result<(), ApplyError> {
		self.apply.call(theme.clone())?;
		Ok(())
	}
}
