use mlua::{IntoLua, Lua};

use super::ApiModule;

pub struct UtilApi;

impl UtilApi {
	fn by_theme<'lua>(
		_: &'lua Lua,
		(theme, value): (mlua::Table<'lua>, mlua::Value<'lua>)
	) -> mlua::Result<mlua::Value<'lua>> {
		let table = match value {
			mlua::Value::Table(table) => table,
			_ => return Ok(value)
		};

		let default: mlua::Value = table.get("default")?;

		let Some(name) = theme.get::<_, Option<String>>("name")? else {
			return Ok(default);
		};

		Ok(table
			.get::<_, Option<mlua::Value>>(name)?
			.unwrap_or(default))
	}
}

impl ApiModule for UtilApi {
	const NAMESPACE: &'static str = "util";

	fn build(lua: &Lua) -> mlua::Result<mlua::Value> {
		let module = lua.create_table()?;

		module.raw_set("by_theme", lua.create_function(Self::by_theme)?)?;

		module.into_lua(lua)
	}
}
