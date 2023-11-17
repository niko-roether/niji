use std::path::PathBuf;

use mlua::{IntoLua, Lua};

use crate::{file_manager::FileManager, utils::xdg::XdgDirs};

use super::Module;

pub struct FilesystemApi;

impl FilesystemApi {
	fn open_managed(lua: &Lua, path: String) -> mlua::Result<mlua::Value> {
		let mut file_mgr = lua.app_data_mut::<FileManager>().unwrap();
		let path = PathBuf::from(path);

		file_mgr.manage(&path).map_err(mlua::Error::runtime)?;

		Self::io_open(lua, path, "w".to_string())
	}

	fn io_open(lua: &Lua, path: PathBuf, mode: String) -> mlua::Result<mlua::Value> {
		lua.globals()
			.get::<_, mlua::Table>("io")?
			.get::<_, mlua::Function>("open")?
			.call((path.to_string_lossy(), mode))
	}
}

impl Module for FilesystemApi {
	const NAMESPACE: &'static str = "fs";

	fn build(lua: &Lua) -> mlua::Result<mlua::Value> {
		let module = lua.create_table()?;

		module.raw_set("open_managed", lua.create_function(Self::open_managed)?)?;

		module.into_lua(lua)
	}
}
