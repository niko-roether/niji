use std::{fs, path::PathBuf, rc::Rc};

use mlua::{IntoLua, Lua};

use crate::{console, file_manager::FileManager, files::Files, utils::xdg::XdgDirs};

use super::{Module, ModuleContext};

pub struct FilesystemApi;

impl FilesystemApi {
	fn write(lua: &Lua, (path, content): (String, String)) -> mlua::Result<()> {
		let file_mgr = lua.app_data_ref::<Rc<FileManager>>().unwrap();
		let path = PathBuf::from(path);

		file_mgr
			.write_managed(&path, &content)
			.map_err(mlua::Error::runtime)?;

		fs::write(path, content).map_err(mlua::Error::runtime)?;
		Ok(())
	}

	fn write_config(lua: &Lua, (path, content): (String, String)) -> mlua::Result<()> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		Self::write(
			lua,
			(
				xdg.config_home.join(path).to_string_lossy().into_owned(),
				content
			)
		)
	}

	fn write_state(lua: &Lua, (path, content): (String, String)) -> mlua::Result<()> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		Self::write(
			lua,
			(
				xdg.state_home.join(path).to_string_lossy().into_owned(),
				content
			)
		)
	}

	fn write_data(lua: &Lua, (path, content): (String, String)) -> mlua::Result<()> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		Self::write(
			lua,
			(
				xdg.data_home.join(path).to_string_lossy().into_owned(),
				content
			)
		)
	}

	fn read_config(lua: &Lua, path: String) -> mlua::Result<mlua::Value> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		fs::read_to_string(xdg.config_home.join(path))
			.map_err(mlua::Error::runtime)?
			.into_lua(lua)
	}

	fn read_state(lua: &Lua, path: String) -> mlua::Result<mlua::Value> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		fs::read_to_string(xdg.state_home.join(path))
			.map_err(mlua::Error::runtime)?
			.into_lua(lua)
	}

	fn read_data(lua: &Lua, path: String) -> mlua::Result<mlua::Value> {
		let xdg = lua.app_data_ref::<Rc<XdgDirs>>().unwrap();
		fs::read_to_string(xdg.data_home.join(path))
			.map_err(mlua::Error::runtime)?
			.into_lua(lua)
	}

	fn write_output(lua: &Lua, (path, content): (String, String)) -> mlua::Result<()> {
		let mod_ctx = lua.app_data_ref::<ModuleContext>().unwrap();
		let files = lua.app_data_ref::<Rc<Files>>().unwrap();
		let path = files.output_dir().join(&mod_ctx.name).join(path);

		console::info!("Outputting to {}", path.display());
		fs::create_dir_all(path.parent().unwrap()).map_err(mlua::Error::runtime)?;
		fs::write(path, content).map_err(mlua::Error::runtime)?;
		Ok(())
	}

	fn read_config_asset(lua: &Lua, path: String) -> mlua::Result<mlua::Value> {
		let mod_ctx = lua.app_data_ref::<ModuleContext>().unwrap();
		let files = lua.app_data_ref::<Rc<Files>>().unwrap();
		let path = files
			.config_file()
			.parent()
			.unwrap()
			.join(&mod_ctx.name)
			.join(path);

		fs::read_to_string(path)
			.map_err(mlua::Error::runtime)?
			.into_lua(lua)
	}
}

impl Module for FilesystemApi {
	const NAMESPACE: &'static str = "fs";

	fn build(lua: &Lua) -> mlua::Result<mlua::Value> {
		let module = lua.create_table()?;

		module.raw_set("write", lua.create_function(Self::write)?)?;
		module.raw_set("write_config", lua.create_function(Self::write_config)?)?;
		module.raw_set("write_state", lua.create_function(Self::write_state)?)?;
		module.raw_set("write_data", lua.create_function(Self::write_data)?)?;
		module.raw_set("write_output", lua.create_function(Self::write_output)?)?;
		module.raw_set("read_config", lua.create_function(Self::read_config)?)?;
		module.raw_set("read_state", lua.create_function(Self::read_state)?)?;
		module.raw_set("read_data", lua.create_function(Self::read_data)?)?;
		module.raw_set(
			"read_config_asset",
			lua.create_function(Self::read_config_asset)?
		)?;

		module.into_lua(lua)
	}
}
