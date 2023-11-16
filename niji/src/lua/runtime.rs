use std::path::Path;

use mlua::{FromLuaMulti, Lua};

use crate::utils::xdg::XdgDirs;

use super::api::{mod_::ModApi, LuaApi, LuaApiInit};

pub struct LuaRuntimeInit {
	pub xdg: XdgDirs
}

pub struct LuaRuntime {
	lua: Lua
}

impl LuaRuntime {
	pub const API_GLOBAL: &str = "niji";

	pub fn new(init: LuaRuntimeInit) -> mlua::Result<Self> {
		let lua = Lua::new();

		lua.load_from_std_lib(mlua::StdLib::ALL_SAFE)?;

		lua.globals()
			.set(Self::API_GLOBAL, LuaApi::new(LuaApiInit { xdg: init.xdg }))?;

		Ok(Self { lua })
	}

	pub fn load_module<'a, R>(&'a self, path: &Path) -> mlua::Result<R>
	where
		R: FromLuaMulti<'a>
	{
		self.lua.load(path).call(())
	}

	pub fn set_module_context(&self, name: Option<&str>) -> mlua::Result<()> {
		let mod_api = name.map(|name| ModApi {
			name: name.to_owned()
		});

		let api: mlua::Table = self.lua.globals().get(Self::API_GLOBAL)?;
		api.raw_set("mod", mod_api)?;

		Ok(())
	}
}
