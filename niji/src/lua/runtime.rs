use std::path::Path;

use mlua::{FromLuaMulti, Lua};

use crate::{file_manager::FileManager, utils::xdg::XdgDirs};

use super::api;

pub struct LuaRuntimeInit {
	pub xdg: XdgDirs,
	pub file_manager: FileManager
}

pub struct LuaRuntime {
	lua: Lua
}

impl LuaRuntime {
	pub fn new(init: LuaRuntimeInit) -> mlua::Result<Self> {
		let lua = Lua::new();

		lua.load_from_std_lib(mlua::StdLib::ALL_SAFE)?;
		api::init(
			&lua,
			api::Init {
				xdg: init.xdg,
				file_manager: init.file_manager
			}
		)?;

		Ok(Self { lua })
	}

	pub fn load_module<'a, R>(&'a self, path: &Path) -> mlua::Result<R>
	where
		R: FromLuaMulti<'a>
	{
		self.lua.load(path).call(())
	}

	pub fn set_module_context(&self, name: Option<&str>) {
		api::set_module_context(&self.lua, name);
	}
}
