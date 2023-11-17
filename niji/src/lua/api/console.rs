use mlua::{IntoLua, Lua};

use crate::console;

use super::{Module, ModuleContext};

pub struct ConsoleApi;

macro_rules! define_log_function {
	($name:ident) => {
		fn $name(lua: &Lua, message: String) -> mlua::Result<()> {
			let source = Self::get_source(lua)?;
			console::$name!(source = &source, "{message}");
			Ok(())
		}
	};
}

impl ConsoleApi {
	fn get_source(lua: &Lua) -> mlua::Result<String> {
		let module_ctx = lua.app_data_ref::<ModuleContext>().unwrap();
		if let Some(module_name) = &module_ctx.name {
			Ok(format!("module:{module_name}"))
		} else {
			Err(mlua::Error::runtime(
				"Console invocation failed; not in module context"
			))
		}
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

impl Module for ConsoleApi {
	const NAMESPACE: &'static str = "console";

	fn build(lua: &Lua) -> mlua::Result<mlua::Value> {
		let module = lua.create_table()?;

		module.raw_set("info", lua.create_function(Self::info)?)?;
		module.raw_set("warn", lua.create_function(Self::warn)?)?;
		module.raw_set("error", lua.create_function(Self::error)?)?;
		module.raw_set("prompt", lua.create_function(Self::prompt)?)?;

		module.into_lua(lua)
	}
}
