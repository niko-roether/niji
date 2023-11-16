use std::path::Path;

use mlua::{FromLuaMulti, Lua};

use crate::utils::xdg::XdgDirs;

use super::api::{LuaApi, LuaApiInit};

pub struct LuaRuntimeInit {
	pub xdg: XdgDirs
}

pub struct LuaRuntime {
	lua: Lua
}

impl LuaRuntime {
	pub fn new(init: LuaRuntimeInit) -> mlua::Result<Self> {
		let lua = Lua::new();

		lua.load_from_std_lib(mlua::StdLib::ALL_SAFE);

		lua.globals()
			.set("niji", LuaApi::new(LuaApiInit { xdg: init.xdg }))?;

		Ok(Self { lua })
	}

	pub fn run_module<'a, R>(&'a self, path: &Path) -> mlua::Result<R>
	where
		R: FromLuaMulti<'a>
	{
		self.lua.load(path).call(())
	}
}
