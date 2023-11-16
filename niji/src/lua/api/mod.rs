use mlua::{IntoLua, Lua};

use crate::utils::xdg::XdgDirs;

use self::{col::ColApi, csl::CslApi, fs::FsApi};

mod col;
mod csl;
mod fs;
pub mod mod_;

#[derive(Debug, Clone)]
pub struct LuaApiInit {
	pub xdg: XdgDirs
}

#[derive(Debug, Clone)]
pub struct LuaApi {
	xdg: XdgDirs
}

impl LuaApi {
	pub fn new(init: LuaApiInit) -> Self {
		Self { xdg: init.xdg }
	}
}

impl<'lua> IntoLua<'lua> for LuaApi {
	fn into_lua(self, lua: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
		let module = lua.create_table()?;

		module.raw_set("col", ColApi)?;
		module.raw_set("xdg", self.xdg)?;
		module.raw_set("fs", FsApi)?;
		module.raw_set("csl", CslApi)?;

		module.into_lua(lua)
	}
}
