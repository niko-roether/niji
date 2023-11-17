use handlebars::Handlebars;
use mlua::Lua;

use crate::{file_manager::FileManager, utils::xdg::XdgDirs};

use self::{
	color::ColorApi, console::ConsoleApi, filesystem::FilesystemApi, module::ModuleApi, xdg::XdgApi
};

mod color;
mod console;
mod filesystem;
mod module;
mod xdg;

struct ModuleContext {
	name: Option<String>
}

trait Module: Sized {
	const NAMESPACE: &'static str;

	fn build(lua: &Lua) -> mlua::Result<mlua::Value>;
}

fn insert_module<M: Module>(lua: &Lua, api: &mlua::Table) -> mlua::Result<()> {
	api.raw_set(M::NAMESPACE, M::build(lua)?)
}

const API_GLOBAL: &str = "niji";

pub struct Init {
	pub xdg: XdgDirs,
	pub file_manager: FileManager
}

pub fn init(lua: &Lua, init: Init) -> mlua::Result<()> {
	set_module_context(lua, None);
	lua.set_app_data(init.xdg);
	lua.set_app_data(init.file_manager);

	let api = lua.create_table()?;

	insert_module::<ColorApi>(lua, &api)?;
	insert_module::<FilesystemApi>(lua, &api)?;
	insert_module::<ModuleApi>(lua, &api)?;
	insert_module::<ConsoleApi>(lua, &api)?;
	insert_module::<XdgApi>(lua, &api)?;

	lua.globals().set(API_GLOBAL, api)?;

	Ok(())
}

pub fn set_module_context(lua: &Lua, module_name: Option<&str>) {
	lua.set_app_data(ModuleContext {
		name: module_name.map(String::from)
	});
}
