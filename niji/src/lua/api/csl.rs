use mlua::{IntoLua, Lua};

use crate::{console, lua::runtime::LuaRuntime};

pub struct CslApi;

macro_rules! define_log_function {
	($name:ident) => {
		fn $name(lua: &Lua, message: String) -> mlua::Result<()> {
			let source = Self::get_source(lua)?;
			console::$name!(source = &source, "{message}");
			Ok(())
		}
	};
}

impl CslApi {
	fn get_source(lua: &Lua) -> mlua::Result<String> {
		let api: mlua::Table = lua.globals().get(LuaRuntime::API_GLOBAL)?;
		let mod_data: Option<mlua::Table> = api.get("mod")?;
		let mod_name: String = match mod_data {
			Some(data) => data.get("name")?,
			None => String::from("?")
		};
		Ok(format!("module:{}", mod_name))
	}

	define_log_function!(info);
	define_log_function!(warn);
	define_log_function!(error);

	fn prompt(lua: &Lua, (message, default): (String, Option<bool>)) -> mlua::Result<bool> {
		let source = Self::get_source(lua)?;
		let result = if let Some(default) = default {
			console::prompt!(source = &source, default = default, "{message}")
		} else {
			console::prompt!(source = &source, "{message}")
		};

		Ok(result)
	}
}

impl<'lua> IntoLua<'lua> for CslApi {
	fn into_lua(self, lua: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
		let module = lua.create_table()?;

		module.raw_set("info", lua.create_function(Self::info)?)?;
		module.raw_set("warn", lua.create_function(Self::warn)?)?;
		module.raw_set("error", lua.create_function(Self::error)?)?;
		module.raw_set("prompt", lua.create_function(Self::prompt)?)?;

		module.into_lua(lua)
	}
}
