use std::path::Path;

use mlua::{FromLuaMulti, Lua};

use crate::{config::Config, utils::xdg::XdgDirs};

use super::api::{LuaApi, LuaApiInit};

pub struct LuaRuntimeInit {
	pub config: Config,
	pub xdg: XdgDirs
}

pub struct LuaRuntime {
	lua: Lua
}

impl LuaRuntime {
	pub fn new(init: LuaRuntimeInit) -> mlua::Result<Self> {
		let lua = Lua::new();

		lua.globals().set(
			"niji",
			LuaApi::new(LuaApiInit {
				config: init.config,
				xdg: init.xdg
			})
		)?;

		Ok(Self { lua })
	}

	pub fn run_module<'a, R>(&'a self, path: &Path) -> mlua::Result<R>
	where
		R: FromLuaMulti<'a>
	{
		self.lua.load(path).call(())
	}
}
