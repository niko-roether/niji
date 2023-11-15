use mlua::{FromLuaMulti, Lua};

use crate::{
	api::LuaApi,
	config::{Config, Theme}
};

pub struct LuaRuntimeInit {
	theme: Theme,
	config: Config
}

pub struct LuaRuntime {
	lua: Lua
}

impl LuaRuntime {
	pub fn new(init: LuaRuntimeInit) -> mlua::Result<Self> {
		let lua = Lua::new();

		lua.globals()
			.set("niji", LuaApi::new(init.theme, init.config))?;

		Ok(Self { lua })
	}

	pub fn run_module<'a, R>(&'a mut self, name: &str, code: String) -> mlua::Result<R>
	where
		R: FromLuaMulti<'a>
	{
		let chunk = self.lua.load(code);

		chunk.set_name(name).call(())
	}
}
