use std::{fs, path::PathBuf, rc::Rc};

use mlua::{IntoLua, Lua};

use crate::{console, file_manager::FileManager, files::Files, utils::xdg::XdgDirs};

use super::{Module, ModuleContext};

pub struct FilesystemApi;

impl FilesystemApi {
	fn open_managed(lua: &Lua, path: String) -> mlua::Result<mlua::Value> {
		let file_mgr = lua.app_data_ref::<Rc<FileManager>>().unwrap();
		let path = PathBuf::from(path);

		file_mgr.manage(&path).map_err(mlua::Error::runtime)?;

		Self::io_open(lua, path, "w".to_string())
	}

	fn manage_config(lua: &Lua, path: String) -> mlua::Result<mlua::Value> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		Self::open_managed(
			lua,
			xdg.config_home.join(path).to_string_lossy().into_owned()
		)
	}

	fn manage_state(lua: &Lua, path: String) -> mlua::Result<mlua::Value> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		Self::open_managed(
			lua,
			xdg.state_home.join(path).to_string_lossy().into_owned()
		)
	}

	fn manage_data(lua: &Lua, path: String) -> mlua::Result<mlua::Value> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		Self::open_managed(lua, xdg.data_home.join(path).to_string_lossy().into_owned())
	}

	fn open_config(lua: &Lua, path: String) -> mlua::Result<mlua::Value> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		Self::io_open(lua, xdg.config_home.join(path), "r".to_string())
	}

	fn open_state(lua: &Lua, path: String) -> mlua::Result<mlua::Value> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		Self::io_open(lua, xdg.config_home.join(path), "r".to_string())
	}

	fn open_data(lua: &Lua, path: String) -> mlua::Result<mlua::Value> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		Self::io_open(lua, xdg.config_home.join(path), "r".to_string())
	}

	fn open_output(lua: &Lua, path: String) -> mlua::Result<mlua::Value> {
		let mod_ctx = lua.app_data_ref::<ModuleContext>().unwrap();
		let files = lua.app_data_ref::<Rc<Files>>().unwrap();
		let path = files.output_dir().join(&mod_ctx.name).join(path);

		console::info!("Outputting to {}", path.display());
		fs::create_dir_all(path.parent().unwrap()).map_err(mlua::Error::runtime)?;
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
		module.raw_set("manage_config", lua.create_function(Self::manage_config)?)?;
		module.raw_set("manage_state", lua.create_function(Self::manage_state)?)?;
		module.raw_set("manage_data", lua.create_function(Self::manage_data)?)?;
		module.raw_set("open_config", lua.create_function(Self::open_config)?)?;
		module.raw_set("open_state", lua.create_function(Self::open_state)?)?;
		module.raw_set("open_data", lua.create_function(Self::open_data)?)?;
		module.raw_set("open_output", lua.create_function(Self::open_output)?)?;

		module.into_lua(lua)
	}
}
