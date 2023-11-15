use mlua::{IntoLua, Lua};

use crate::config::{Config, Theme};

use self::col::ColApi;

mod col;

pub struct LuaApi {
	theme: Theme,
	config: Config
}

impl LuaApi {
	pub fn new(theme: Theme, config: Config) -> Self {
		Self { theme, config }
	}
}

impl<'lua> IntoLua<'lua> for LuaApi {
	fn into_lua(self, lua: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
		let module = lua.create_table()?;

		module.raw_set("col", ColApi)?;
		module.raw_set("theme", self.theme)?;
		module.raw_set("cfg", self.config)?;

		module.into_lua(lua)
	}
}
