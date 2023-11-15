use mlua::{IntoLua, Lua};

use crate::config::Config;

use self::col::ColApi;

mod col;

pub struct LuaApi {
	config: Config
}

impl LuaApi {
	pub fn new(config: Config) -> Self {
		Self { config }
	}
}

impl<'lua> IntoLua<'lua> for LuaApi {
	fn into_lua(self, lua: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
		let module = lua.create_table()?;

		module.raw_set("col", ColApi)?;

		module.into_lua(lua)
	}
}
