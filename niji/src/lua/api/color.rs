use mlua::{IntoLua, Lua};

use crate::types::color::Color;

use super::Module;

pub struct ColorApi;

impl ColorApi {
	#[inline]
	#[allow(clippy::new_ret_no_self)]
	pub fn new(_: &Lua, col: Color) -> mlua::Result<Color> {
		Ok(col)
	}

	pub fn blend(_: &Lua, (col1, col2, t): (Color, Color, f32)) -> mlua::Result<Color> {
		Ok(Color::blend(col1, col2, t))
	}

	pub fn mix(_: &Lua, (col1, col2): (Color, Color)) -> mlua::Result<Color> {
		Ok(Color::mix(col1, col2))
	}
}

impl Module for ColorApi {
	const NAMESPACE: &'static str = "col";

	fn build(lua: &Lua) -> mlua::Result<mlua::Value> {
		let module = lua.create_table()?;

		module.raw_set("new", lua.create_function(ColorApi::new)?)?;
		module.raw_set("blend", lua.create_function(ColorApi::blend)?)?;
		module.raw_set("mix", lua.create_function(ColorApi::mix)?)?;

		module.into_lua(lua)
	}
}
