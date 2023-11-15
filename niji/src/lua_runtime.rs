use std::path::Path;

use mlua::{FromLuaMulti, Lua};

use crate::{api::LuaApi, config::Config};

pub struct LuaRuntimeInit {
	pub config: Config
}

pub struct LuaRuntime {
	lua: Lua
}

impl LuaRuntime {
	pub fn new(init: LuaRuntimeInit) -> mlua::Result<Self> {
		let lua = Lua::new();

		lua.globals().set("niji", LuaApi::new(init.config))?;

		Ok(Self { lua })
	}

	pub fn run_module<'a, R>(&'a self, path: &Path) -> mlua::Result<R>
	where
		R: FromLuaMulti<'a>
	{
		self.lua.load(path).call(())
	}
}
