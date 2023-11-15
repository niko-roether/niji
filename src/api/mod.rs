use mlua::{IntoLua, Lua};

use self::col::ColApi;

mod col;

pub struct LuaApi;

impl<'lua> IntoLua<'lua> for LuaApi {
	fn into_lua(self, lua: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
		let module = lua.create_table()?;

		module.raw_set("col", ColApi)?;

		module.into_lua(lua)
	}
}
