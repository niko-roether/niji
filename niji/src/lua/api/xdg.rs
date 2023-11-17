use mlua::IntoLua;

use crate::utils::xdg::XdgDirs;

use super::Module;

pub struct XdgApi;

impl Module for XdgApi {
	const NAMESPACE: &'static str = "xdg";

	fn build(lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
		let xdg = lua.app_data_ref::<XdgDirs>().unwrap();
		xdg.clone().into_lua(lua)
	}
}
