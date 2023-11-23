use std::process::{Command, Stdio};

use mlua::{IntoLua, Lua};

use super::Module;

pub struct OsApi;

impl OsApi {
	fn exec_detached(_: &Lua, command: String) -> mlua::Result<()> {
		let mut parts = command.split(' ').filter(|s| !s.is_empty());
		let Some(program) = parts.next() else {
			return Ok(());
		};
		let args = parts;

		Command::new(program)
			.args(args)
			.stdin(Stdio::null())
			.stdout(Stdio::null())
			.stderr(Stdio::null())
			.spawn()
			.map_err(mlua::Error::runtime)?;

		Ok(())
	}
}

impl Module for OsApi {
	const NAMESPACE: &'static str = "os";

	fn build(lua: &Lua) -> mlua::Result<mlua::Value> {
		let table = lua.create_table()?;

		table.raw_set("exec_detached", lua.create_function(Self::exec_detached)?)?;

		table.into_lua(lua)
	}
}
