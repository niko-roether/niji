use std::{path::PathBuf, rc::Rc};

use mlua::Lua;

use crate::{file_manager::FileManager, files::Files, utils::xdg::XdgDirs};

use self::{
	color::ColorApi, console::ConsoleApi, filesystem::FilesystemApi, module_meta::ModuleMetaApi,
	os::OsApi, template::TemplateApi, xdg::XdgApi
};

mod color;
mod console;
mod filesystem;
mod module_meta;
mod os;
mod template;
mod xdg;

pub struct ModuleContext {
	pub name: String,
	pub path: PathBuf
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
	pub xdg: Rc<XdgDirs>,
	pub files: Rc<Files>,
	pub file_manager: Rc<FileManager>
}

pub fn init(lua: &Lua, init: Init) -> mlua::Result<()> {
	reset_module_context(lua);
	lua.set_app_data(init.xdg);
	lua.set_app_data(init.file_manager);
	lua.set_app_data(init.files);

	let api = lua.create_table()?;

	insert_module::<ColorApi>(lua, &api)?;
	insert_module::<FilesystemApi>(lua, &api)?;
	insert_module::<ModuleMetaApi>(lua, &api)?;
	insert_module::<ConsoleApi>(lua, &api)?;
	insert_module::<XdgApi>(lua, &api)?;
	insert_module::<TemplateApi>(lua, &api)?;
	insert_module::<OsApi>(lua, &api)?;

	lua.globals().set(API_GLOBAL, api)?;

	Ok(())
}

pub fn set_module_context(lua: &Lua, ctx: ModuleContext) {
	lua.set_app_data(ctx);
}

pub fn reset_module_context(lua: &Lua) {
	lua.remove_app_data::<ModuleContext>();
}
