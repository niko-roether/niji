use mlua::{IntoLua, Lua};

pub struct FsApi;

impl<'lua> IntoLua<'lua> for FsApi {
	fn into_lua(self, lua: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
		lua.load(include_str!("./fs.lua")).call(())
	}
}
